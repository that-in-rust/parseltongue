// Level 4: Centralized Error Handling
// - Defines custom error types
// - Implements conversions from external errors
// - Provides a Result alias for convenience

use thiserror::Error;
use zip::result::ZipError;
use rocksdb::Error as RocksDbError;
use std::io;
use tokio::sync::mpsc::error::SendError;

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

    #[error("Async task error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Channel send error: {0}")]
    MpscSendError(String),

    #[error("Other error: {0}")]
    Generic(String),
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Error::MpscSendError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>; 