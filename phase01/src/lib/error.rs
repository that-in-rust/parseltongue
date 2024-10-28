// Using error.rs from core/error.rs with adjustments for new structure
use std::{io, path::PathBuf};
use thiserror::Error;
use tokio::time::error::Elapsed;
use std::sync::Arc;
use metrics::{Counter, Gauge};
use std::time::Duration;
use tokio::sync::Mutex;

/// Core error types for the ZIP processing system
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Database error: {0}")]
    Database(#[from] sled::Error),

    #[error("Encoding error: {0}")]
    Encoding(#[from] std::string::FromUtf8Error),

    #[error("Operation timed out after {0:?}")]
    Timeout(Duration, #[source] Box<Elapsed>),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    #[error("Task cancelled")]
    Cancelled,

    #[error("Shutdown in progress")]
    Shutdown,
}

/// Result type alias for our error type
pub type Result<T> = std::result::Result<T, Error>;

// Rest of the error handling infrastructure...

