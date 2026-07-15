use pdf_agent_core::pipeline::job_event::JobProgress;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Main,
    Settings,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
    TabChanged(Route),
    OpenFileClicked,
    PdfLoaded { path: PathBuf, page_count: usize },
    PdfLoadFailed(String),
    ConvertClicked,
    ConvertJobStarted(String),
    JobProgressUpdate(JobProgress),
    JobFinished {
        job_id: String,
        markdown: String,
        document: pdf_agent_core::schema::document::Document,
    },
    JobFailed { job_id: String, error: String },
    CancelClicked,
    FeedbackInputChanged(String),
    SubmitFeedbackClicked,
    PageChanged(usize),
    PageImageLoaded { page_index: usize, image: pdf_agent_core::providers::traits::PageImage },
    PageImageLoadFailed(String),
    BlockSelected(Option<String>),
    UndoClicked,
    RedoClicked,
    AcceptPatchClicked,
    RejectPatchClicked,
    LlmFeedbackResult(std::result::Result<(String, pdf_agent_core::schema::document::Document), String>),
    OcrModeChanged(String),
    OutputFormatChanged(String),
    LlmProviderChanged(String),
    LlmModelChanged(String),
    LlmBaseUrlChanged(String),
    LlmKeyChanged(String),
    LlmLimitChanged(String),
    SaveSettingsClicked,
    ExportMarkdownClicked,
    ExportJsonClicked,
    ExportCompleted(Result<String, String>),
}
