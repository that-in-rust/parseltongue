// Level 4: Error Type Hierarchy
// - Defines core error types used throughout application
// - Implements error conversion traits
// - Provides error context and backtracing
// - Enables async-aware error propagation

use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum Error {
    // Level 3: Domain-Specific Errors
    #[error("ZIP processing error: {0}")]
    Zip(#[from] zip::result::ZipError),
    
    #[error("Storage error: {0}")]
    Storage(#[from] rocksdb::Error),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // Level 2: Application Errors
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    
    #[error("Invalid configuration: {msg}")]
    Config { msg: String },
    
    #[error("Processing error: {msg}")]
    Processing { msg: String },

    // Level 1: Runtime Errors
    #[error("Task error: {0}")]
    Task(#[from] tokio::task::JoinError),
    
    #[error("Channel error: {0}")]
    Channel(String),
}

// Convenience type alias used throughout the application
pub type Result<T> = std::result::Result<T, Error>; 