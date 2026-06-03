use crate::error::Result;
use rusqlite::Connection;

pub struct DocumentRecord {
    pub id: String,
    pub job_id: String,
    pub file_path: String,
    pub markdown_content: String,
    pub version_id: usize,
    pub created_at: String,
}

pub struct DocumentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> DocumentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, record: &DocumentRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO documents (id, job_id, file_path, markdown_content, version_id, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &record.id,
                &record.job_id,
                &record.file_path,
                &record.markdown_content,
                record.version_id as i64,
                &record.created_at,
            ),
        )?;
        Ok(())
    }

    pub fn get_by_job(&self, job_id: &str) -> Result<Option<DocumentRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, job_id, file_path, markdown_content, version_id, created_at FROM documents WHERE job_id = ?1 ORDER BY version_id DESC LIMIT 1"
        )?;
        let mut rows = stmt.query([job_id])?;

        if let Some(row) = rows.next()? {
            let record = DocumentRecord {
                id: row.get(0)?,
                job_id: row.get(1)?,
                file_path: row.get(2)?,
                markdown_content: row.get(3)?,
                version_id: row.get::<_, i64>(4)? as usize,
                created_at: row.get(5)?,
            };
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }
}
