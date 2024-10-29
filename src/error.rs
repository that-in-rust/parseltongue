// Level 4: Centralized Error Handling
// - Defines common Result and Error types
// - Aggregates error sources for consistent handling

use thiserror::Error;
use zip::result::ZipError;
use rocksdb::Error as RocksDbError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

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

    #[error("Other error: {0}")]
    Generic(String),
} 