use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during file writing operations
#[derive(Error, Debug)]
pub enum FileWriterError {
    #[error("File already exists: {path}")]
    FileAlreadyExists { path: PathBuf },

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Invalid ISGL1 key format: {key}")]
    InvalidIsgl1Key { key: String },

    #[error("Future code missing for {action} operation")]
    MissingFutureCode { action: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),
}

impl FileWriterError {
    /// Create a file already exists error
    pub fn file_already_exists(path: PathBuf) -> Self {
        Self::FileAlreadyExists { path }
    }

    /// Create a file not found error
    pub fn file_not_found(path: PathBuf) -> Self {
        Self::FileNotFound { path }
    }

    /// Create an invalid ISGL1 key error
    pub fn invalid_isgl1_key(key: String) -> Self {
        Self::InvalidIsgl1Key { key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FileWriterError::file_already_exists(PathBuf::from("test.rs"));
        assert!(err.to_string().contains("test.rs"));
    }

    #[test]
    fn test_invalid_isgl1_key_error() {
        let err = FileWriterError::invalid_isgl1_key("invalid-key".to_string());
        assert!(err.to_string().contains("invalid-key"));
    }
}
