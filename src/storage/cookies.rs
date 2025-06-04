// this may be the most wrong way to handle cookies but idc
// sue me if it works it works
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<DateTime<Utc>>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct CookieManager {
    db_path: std::path::PathBuf,
    auto_clear_days: u32,
}

impl CookieManager {
    pub fn new(data_dir: &Path, auto_clear_days: u32) -> Result<Self> {
        let db_path = data_dir.join("cookies.db");
        let manager = Self {
            db_path,
            auto_clear_days,
        };
        manager.init_database()?;
        Ok(manager)
    }

    fn init_database(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS cookies (
                name TEXT NOT NULL,
                value TEXT NOT NULL,
                domain TEXT NOT NULL,
                path TEXT NOT NULL,
                expires DATETIME,
                secure BOOLEAN DEFAULT 0,
                http_only BOOLEAN DEFAULT 0,
                same_site TEXT,
                created_at DATETIME NOT NULL,
                PRIMARY KEY(name, domain, path)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_cookies_domain ON cookies(domain)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_cookies_expires ON cookies(expires)",
            [],
        )?;

        Ok(())
    }

    pub fn set_cookie(&self, cookie: Cookie) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;

        let expires_str = cookie
            .expires
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());

        conn.execute(
            "INSERT OR REPLACE INTO cookies
             (name, value, domain, path, expires, secure, http_only, same_site, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                cookie.name,
                cookie.value,
                cookie.domain,
                cookie.path,
                expires_str,
                cookie.secure,
                cookie.http_only,
                cookie.same_site,
                cookie.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
            ],
        )?;

        Ok(())
    }

    pub fn get_cookies_for_domain(&self, domain: &str) -> Result<Vec<Cookie>> {
        let conn = Connection::open(&self.db_path)?;

        let mut stmt = conn.prepare(
            "SELECT name, value, domain, path, expires, secure, http_only, same_site, created_at
             FROM cookies
             WHERE domain = ?1 OR domain = ?2
             ORDER BY path DESC, name",
        )?;

        let domain_with_dot = format!(".{}", domain);
        let rows = stmt.query_map(params![domain, domain_with_dot], |row| {
            Ok(self.row_to_cookie(row)?)
        })?;

        let mut cookies = Vec::new();
        for row in rows {
            let cookie = row?;
            // Check if cookie is still valid
            if let Some(expires) = cookie.expires {
                if expires < Utc::now() {
                    continue; // Skip expired cookies
                }
            }
            cookies.push(cookie);
        }

        Ok(cookies)
    }

    pub fn delete_cookie(&self, name: &str, domain: &str, path: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "DELETE FROM cookies WHERE name = ?1 AND domain = ?2 AND path = ?3",
            params![name, domain, path],
        )?;
        Ok(())
    }

    pub fn delete_cookies_for_domain(&self, domain: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        let domain_with_dot = format!(".{}", domain);
        conn.execute(
            "DELETE FROM cookies WHERE domain = ?1 OR domain = ?2",
            params![domain, domain_with_dot],
        )?;
        Ok(())
    }

    pub fn clear_all_cookies(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM cookies", [])?;
        Ok(())
    }

    pub fn clear_expired_cookies(&self) -> Result<u32> {
        let conn = Connection::open(&self.db_path)?;
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let count = conn.execute(
            "DELETE FROM cookies WHERE expires IS NOT NULL AND expires < ?1",
            params![now],
        )?;

        Ok(count as u32)
    }

    pub fn clear_old_cookies(&self) -> Result<u32> {
        let conn = Connection::open(&self.db_path)?;
        let cutoff_date = Utc::now() - chrono::Duration::days(self.auto_clear_days as i64);

        let count = conn.execute(
            "DELETE FROM cookies WHERE created_at < ?1",
            params![cutoff_date.format("%Y-%m-%d %H:%M:%S").to_string()],
        )?;

        Ok(count as u32)
    }

    pub fn get_cookie_count(&self) -> Result<u32> {
        let conn = Connection::open(&self.db_path)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM cookies", [], |row| row.get(0))?;
        Ok(count as u32)
    }

    pub fn get_domains_with_cookies(&self) -> Result<Vec<String>> {
        let conn = Connection::open(&self.db_path)?;

        let mut stmt = conn.prepare("SELECT DISTINCT domain FROM cookies ORDER BY domain")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

        let mut domains = Vec::new();
        for row in rows {
            domains.push(row?);
        }

        Ok(domains)
    }

    pub fn cleanup_task(&self) -> Result<(u32, u32)> {
        let expired_count = self.clear_expired_cookies()?;
        let old_count = self.clear_old_cookies()?;
        Ok((expired_count, old_count))
    }

    fn row_to_cookie(&self, row: &Row) -> Result<Cookie, rusqlite::Error> {
        let expires_str: Option<String> = row.get(4)?;
        let created_at_str: String = row.get(8)?;

        let expires = if let Some(expires_str) = expires_str {
            Some(
                DateTime::parse_from_str(&expires_str, "%Y-%m-%d %H:%M:%S")
                    .map_err(|_| {
                        rusqlite::Error::InvalidColumnType(
                            4,
                            "expires".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?
                    .with_timezone(&Utc),
            )
        } else {
            None
        };

        let created_at = DateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    8,
                    "created_at".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?
            .with_timezone(&Utc);

        Ok(Cookie {
            name: row.get(0)?,
            value: row.get(1)?,
            domain: row.get(2)?,
            path: row.get(3)?,
            expires,
            secure: row.get(5)?,
            http_only: row.get(6)?,
            same_site: row.get(7)?,
            created_at,
        })
    }
}
