use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Storage error: {0}")]
    General(String),
}

pub type Result<T> = std::result::Result<T, Error>;
