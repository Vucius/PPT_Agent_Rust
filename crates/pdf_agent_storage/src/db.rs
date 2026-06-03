use crate::error::Result;
use rusqlite::Connection;
use std::path::Path;

pub struct DbConnection {
    conn: Connection,
}

impl DbConnection {
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn get_conn(&self) -> &Connection {
        &self.conn
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS jobs (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                status TEXT NOT NULL,
                progress_stage TEXT NOT NULL,
                current_page INTEGER NOT NULL,
                total_pages INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                error_message TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                job_id TEXT NOT NULL,
                file_path TEXT NOT NULL,
                markdown_content TEXT NOT NULL,
                version_id INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY(job_id) REFERENCES jobs(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS quotas (
                date TEXT PRIMARY KEY,
                tokens_used INTEGER NOT NULL,
                limit_threshold INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn get_tokens_used(&self, date: &str) -> Result<i64> {
        let mut stmt = self.conn.prepare("SELECT tokens_used FROM quotas WHERE date = ?")?;
        let mut rows = stmt.query([date])?;
        if let Some(row) = rows.next()? {
            Ok(row.get(0)?)
        } else {
            Ok(0)
        }
    }

    pub fn increment_tokens_used(&self, date: &str, amount: i64, limit_threshold: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO quotas (date, tokens_used, limit_threshold)
             VALUES (?, ?, ?)
             ON CONFLICT(date) DO UPDATE SET tokens_used = tokens_used + excluded.tokens_used",
            rusqlite::params![date, amount, limit_threshold],
        )?;
        Ok(())
    }
}
