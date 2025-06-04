use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub visit_count: i32,
    pub last_visit: DateTime<Utc>,
    pub first_visit: DateTime<Utc>,
}

pub struct HistoryManager {
    db_path: std::path::PathBuf,
}

impl HistoryManager {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let db_path = data_dir.join("history.db");
        let manager = Self { db_path };
        manager.init_database()?;
        Ok(manager)
    }

    fn init_database(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                visit_count INTEGER DEFAULT 1,
                last_visit DATETIME NOT NULL,
                first_visit DATETIME NOT NULL,
                UNIQUE(url)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_url ON history(url)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_last_visit ON history(last_visit)",
            [],
        )?;

        Ok(())
    }

    pub fn add_entry(&self, url: &str, title: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        let now = Utc::now();

        conn.execute(
            "INSERT INTO history (url, title, visit_count, last_visit, first_visit)
             VALUES (?1, ?2, 1, ?3, ?3)
             ON CONFLICT(url) DO UPDATE SET
                title = ?2,
                visit_count = visit_count + 1,
                last_visit = ?3",
            params![url, title, now.format("%Y-%m-%d %H:%M:%S").to_string()],
        )?;

        Ok(())
    }

    pub fn search_history(&self, query: &str, limit: usize) -> Result<Vec<HistoryEntry>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_count, last_visit, first_visit 
             FROM history 
             WHERE url LIKE ?1 OR title LIKE ?1 
             ORDER BY visit_count DESC, last_visit DESC 
             LIMIT ?2"
        )?;

        let search_pattern = format!("%{}%", query);
        let rows = stmt.query_map(params![search_pattern, limit], |row| {
            Ok(self.row_to_history_entry(row)?)
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }

        Ok(entries)
    }

    pub fn get_recent_history(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_count, last_visit, first_visit 
             FROM history 
             ORDER BY last_visit DESC 
             LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok(self.row_to_history_entry(row)?)
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }

        Ok(entries)
    }

    pub fn get_top_sites(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_count, last_visit, first_visit 
             FROM history 
             ORDER BY visit_count DESC, last_visit DESC 
             LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok(self.row_to_history_entry(row)?)
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }

        Ok(entries)
    }

    pub fn delete_entry(&self, id: i64) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM history WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_by_url(&self, url: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM history WHERE url = ?1", params![url])?;
        Ok(())
    }

    pub fn clear_all_history(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub fn clear_old_history(&self, days: u32) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        let cutoff_date = Utc::now() - chrono::Duration::days(days as i64);
        
        conn.execute(
            "DELETE FROM history WHERE last_visit < ?1",
            params![cutoff_date.format("%Y-%m-%d %H:%M:%S").to_string()],
        )?;
        
        Ok(())
    }

    fn row_to_history_entry(&self, row: &Row) -> Result<HistoryEntry, rusqlite::Error> {
        let last_visit_str: String = row.get(4)?;
        let first_visit_str: String = row.get(5)?;
        
        let last_visit = DateTime::parse_from_str(&last_visit_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| rusqlite::Error::InvalidColumnType(4, "last_visit".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);
            
        let first_visit = DateTime::parse_from_str(&first_visit_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| rusqlite::Error::InvalidColumnType(5, "first_visit".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);

        Ok(HistoryEntry {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            visit_count: row.get(3)?,
            last_visit,
            first_visit,
        })
    }
}