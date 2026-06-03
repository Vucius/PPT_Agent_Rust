use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Lopdf error: {0}")]
    Lopdf(#[from] lopdf::Error),

    #[error("Page {0} out of bounds")]
    PageOutOfBounds(usize),

    #[error("PDF processing error: {0}")]
    Processing(String),
}

pub type Result<T> = std::result::Result<T, Error>;
