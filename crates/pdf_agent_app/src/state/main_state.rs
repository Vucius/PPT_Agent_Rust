use pdf_agent_core::pipeline::job_event::JobProgress;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MainState {
    Empty,
    PdfLoaded {
        file_path: PathBuf,
        page_count: usize,
    },
    Converting {
        file_path: PathBuf,
        progress: JobProgress,
    },
    Converted {
        file_path: PathBuf,
        markdown: String,
        document: pdf_agent_core::schema::document::Document,
    },
    Failed {
        file_path: Option<PathBuf>,
        error: String,
    },
}
