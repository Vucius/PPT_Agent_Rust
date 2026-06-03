use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStage {
    Idle,
    LoadingPdf,
    LayoutAnalysis,
    Ocr,
    RunningProcessors,
    Rendering,
    Completed,
    Failed,
}
