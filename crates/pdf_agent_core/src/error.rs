use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("PDF processing error: {0}")]
    Pdf(String),

    #[error("Inference/OCR error: {0}")]
    Inference(String),

    #[error("LLM Service error: {0}")]
    Llm(String),

    #[error("Pipeline error at stage {stage}: {message}")]
    Pipeline {
        stage: String,
        message: String,
    },

    #[error("Job cancelled")]
    Cancelled,

    #[error("General error: {0}")]
    General(String),
}

pub type Result<T> = std::result::Result<T, Error>;
