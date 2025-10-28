//! Error types for the file writer

use thiserror::Error;

/// File writer error types
#[derive(Error, Debug)]
pub enum FileWriterError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Backup creation failed: {reason}")]
    BackupFailed { reason: String },

    #[error("Rollback failed: {reason}")]
    RollbackFailed { reason: String },

    #[error("Safety check failed: {reason}")]
    SafetyCheckFailed { reason: String },

    #[error("Validation required but not provided")]
    ValidationRequired,

    #[error("Operation timed out after {duration_ms}ms")]
    Timeout { duration_ms: u64 },

    #[error("File size too large: {size} bytes (max: {max_size} bytes)")]
    FileTooLarge { size: u64, max_size: u64 },

    #[error("Concurrent modification detected: {path}")]
    ConcurrentModification { path: String },

    #[error("Invalid configuration: {reason}")]
    InvalidConfiguration { reason: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Temporary file error: {0}")]
    TempFile(#[from] tempfile::PersistError),
}

/// Result type for file writer operations
pub type FileWriterResult<T> = Result<T, FileWriterError>;
