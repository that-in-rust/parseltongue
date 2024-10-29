// Level 4: Error Handling
// - Defines custom error types
// - Implements conversions from external errors
// - Provides Result type alias

use thiserror::Error;
use tokio::sync::AcquireError;
use tokio::task::JoinError;
use tokio::sync::mpsc::error::SendError;
use zip::result::ZipError;
use rocksdb::Error as RocksDbError;
use std::io;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] ZipError),

    #[error("Database error: {0}")]
    Db(#[from] RocksDbError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Task join error: {0}")]
    Join(#[from] JoinError),

    #[error("Parse error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Channel send error: {0}")]
    MpscSendError(String),

    #[error("Semaphore acquire error: {0}")]
    SemaphoreError(String),

    #[error("Generic error: {0}")]
    Generic(String),
}

// Implement From for SendError
impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Error::MpscSendError(err.to_string())
    }
}

// Implement From for AcquireError
impl From<AcquireError> for Error {
    fn from(err: AcquireError) -> Self {
        Error::SemaphoreError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;