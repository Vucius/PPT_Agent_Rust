use crate::pipeline::stage::PipelineStage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobProgress {
    pub job_id: String,
    pub stage: PipelineStage,
    pub current_page: usize,
    pub total_pages: usize,
    pub elapsed_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobEvent {
    Started { job_id: String },
    Progress(JobProgress),
    Finished {
        job_id: String,
        markdown: String,
        document: crate::schema::document::Document,
    },
    Failed { job_id: String, error: String },
}
