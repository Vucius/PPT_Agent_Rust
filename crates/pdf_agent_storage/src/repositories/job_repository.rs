use crate::error::Result;
use rusqlite::Connection;

pub struct JobRecord {
    pub id: String,
    pub file_path: String,
    pub status: String,
    pub progress_stage: String,
    pub current_page: usize,
    pub total_pages: usize,
    pub created_at: String,
    pub updated_at: String,
    pub error_message: Option<String>,
}

pub struct JobRepository<'a> {
    conn: &'a Connection,
}

impl<'a> JobRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, record: &JobRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO jobs (id, file_path, status, progress_stage, current_page, total_pages, created_at, updated_at, error_message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &record.id,
                &record.file_path,
                &record.status,
                &record.progress_stage,
                record.current_page as i64,
                record.total_pages as i64,
                &record.created_at,
                &record.updated_at,
                &record.error_message,
            ),
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<JobRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, file_path, status, progress_stage, current_page, total_pages, created_at, updated_at, error_message FROM jobs WHERE id = ?1"
        )?;
        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            let record = JobRecord {
                id: row.get(0)?,
                file_path: row.get(1)?,
                status: row.get(2)?,
                progress_stage: row.get(3)?,
                current_page: row.get::<_, i64>(4)? as usize,
                total_pages: row.get::<_, i64>(5)? as usize,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                error_message: row.get(8)?,
            };
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    pub fn update_progress(&self, id: &str, stage: &str, current_page: usize, total_pages: usize) -> Result<()> {
        self.conn.execute(
            "UPDATE jobs SET progress_stage = ?1, current_page = ?2, total_pages = ?3, updated_at = ?4 WHERE id = ?5",
            (
                stage,
                current_page as i64,
                total_pages as i64,
                "now",
                id,
            ),
        )?;
        Ok(())
    }
}
