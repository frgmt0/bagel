use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub folder_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favicon: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkFolder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub position: i32,
}

pub struct BookmarkManager {
    db_path: std::path::PathBuf,
}

impl BookmarkManager {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let db_path = data_dir.join("bookmarks.db");
        let manager = Self { db_path };
        manager.init_database()?;
        Ok(manager)
    }

    fn init_database(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmark_folders (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id TEXT,
                created_at DATETIME NOT NULL,
                position INTEGER DEFAULT 0,
                FOREIGN KEY(parent_id) REFERENCES bookmark_folders(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                folder_id TEXT,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL,
                favicon TEXT,
                description TEXT,
                FOREIGN KEY(folder_id) REFERENCES bookmark_folders(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmark_tags (
                bookmark_id TEXT NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY(bookmark_id, tag),
                FOREIGN KEY(bookmark_id) REFERENCES bookmarks(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Create default folders
        self.create_default_folders()?;

        Ok(())
    }

    fn create_default_folders(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Check if root folder exists
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM bookmark_folders WHERE id = 'root'",
            [],
            |row| row.get(0),
        )?;

        if count == 0 {
            conn.execute(
                "INSERT INTO bookmark_folders (id, name, parent_id, created_at, position) 
                 VALUES ('root', 'Bookmarks', NULL, ?1, 0)",
                params![now],
            )?;

            conn.execute(
                "INSERT INTO bookmark_folders (id, name, parent_id, created_at, position) 
                 VALUES ('toolbar', 'Bookmarks Toolbar', 'root', ?1, 0)",
                params![now],
            )?;

            conn.execute(
                "INSERT INTO bookmark_folders (id, name, parent_id, created_at, position) 
                 VALUES ('other', 'Other Bookmarks', 'root', ?1, 1)",
                params![now],
            )?;
        }

        Ok(())
    }

    pub fn add_bookmark(&self, title: &str, url: &str, folder_id: Option<String>) -> Result<String> {
        let conn = Connection::open(&self.db_path)?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let folder = folder_id.unwrap_or_else(|| "toolbar".to_string());

        conn.execute(
            "INSERT INTO bookmarks (id, title, url, folder_id, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![id, title, url, folder, now],
        )?;

        Ok(id)
    }

    pub fn update_bookmark(&self, id: &str, title: Option<&str>, url: Option<&str>, folder_id: Option<&str>) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        if let Some(title) = title {
            conn.execute(
                "UPDATE bookmarks SET title = ?1, updated_at = ?2 WHERE id = ?3",
                params![title, now, id],
            )?;
        }

        if let Some(url) = url {
            conn.execute(
                "UPDATE bookmarks SET url = ?1, updated_at = ?2 WHERE id = ?3",
                params![url, now, id],
            )?;
        }

        if let Some(folder_id) = folder_id {
            conn.execute(
                "UPDATE bookmarks SET folder_id = ?1, updated_at = ?2 WHERE id = ?3",
                params![folder_id, now, id],
            )?;
        }

        Ok(())
    }

    pub fn delete_bookmark(&self, id: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_bookmarks_in_folder(&self, folder_id: &str) -> Result<Vec<Bookmark>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, title, url, folder_id, created_at, updated_at, favicon, description 
             FROM bookmarks 
             WHERE folder_id = ?1 
             ORDER BY title"
        )?;

        let rows = stmt.query_map(params![folder_id], |row| {
            Ok(self.row_to_bookmark(row)?)
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            let mut bookmark = row?;
            bookmark.tags = self.get_bookmark_tags(&bookmark.id)?;
            bookmarks.push(bookmark);
        }

        Ok(bookmarks)
    }

    pub fn search_bookmarks(&self, query: &str) -> Result<Vec<Bookmark>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, title, url, folder_id, created_at, updated_at, favicon, description 
             FROM bookmarks 
             WHERE title LIKE ?1 OR url LIKE ?1 OR description LIKE ?1
             ORDER BY title"
        )?;

        let search_pattern = format!("%{}%", query);
        let rows = stmt.query_map(params![search_pattern], |row| {
            Ok(self.row_to_bookmark(row)?)
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            let mut bookmark = row?;
            bookmark.tags = self.get_bookmark_tags(&bookmark.id)?;
            bookmarks.push(bookmark);
        }

        Ok(bookmarks)
    }

    pub fn create_folder(&self, name: &str, parent_id: Option<String>) -> Result<String> {
        let conn = Connection::open(&self.db_path)?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let parent = parent_id.unwrap_or_else(|| "other".to_string());

        conn.execute(
            "INSERT INTO bookmark_folders (id, name, parent_id, created_at) 
             VALUES (?1, ?2, ?3, ?4)",
            params![id, name, parent, now],
        )?;

        Ok(id)
    }

    pub fn get_folders(&self) -> Result<Vec<BookmarkFolder>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, created_at, position 
             FROM bookmark_folders 
             ORDER BY position, name"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(self.row_to_folder(row)?)
        })?;

        let mut folders = Vec::new();
        for row in rows {
            folders.push(row?);
        }

        Ok(folders)
    }

    pub fn add_bookmark_tag(&self, bookmark_id: &str, tag: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT OR IGNORE INTO bookmark_tags (bookmark_id, tag) VALUES (?1, ?2)",
            params![bookmark_id, tag],
        )?;
        Ok(())
    }

    pub fn remove_bookmark_tag(&self, bookmark_id: &str, tag: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "DELETE FROM bookmark_tags WHERE bookmark_id = ?1 AND tag = ?2",
            params![bookmark_id, tag],
        )?;
        Ok(())
    }

    fn get_bookmark_tags(&self, bookmark_id: &str) -> Result<Vec<String>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT tag FROM bookmark_tags WHERE bookmark_id = ?1")?;
        
        let rows = stmt.query_map(params![bookmark_id], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }

        Ok(tags)
    }

    fn row_to_bookmark(&self, row: &Row) -> Result<Bookmark, rusqlite::Error> {
        let created_at_str: String = row.get(4)?;
        let updated_at_str: String = row.get(5)?;
        
        let created_at = DateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| rusqlite::Error::InvalidColumnType(4, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);
            
        let updated_at = DateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| rusqlite::Error::InvalidColumnType(5, "updated_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);

        Ok(Bookmark {
            id: row.get(0)?,
            title: row.get(1)?,
            url: row.get(2)?,
            folder_id: row.get(3)?,
            created_at,
            updated_at,
            favicon: row.get(6)?,
            description: row.get(7)?,
            tags: Vec::new(), // Will be populated separately
        })
    }

    fn row_to_folder(&self, row: &Row) -> Result<BookmarkFolder, rusqlite::Error> {
        let created_at_str: String = row.get(3)?;
        
        let created_at = DateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| rusqlite::Error::InvalidColumnType(3, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);

        Ok(BookmarkFolder {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            created_at,
            position: row.get(4)?,
        })
    }
}