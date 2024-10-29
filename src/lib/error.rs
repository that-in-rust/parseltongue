//! Error Handling - Pyramidal Structure
//! Layer 1: Core Error Types
//! Layer 2: Error Variants
//! Layer 3: Error Context
//! Layer 4: Error Conversion
//! Layer 5: Helper Functions

use std::path::PathBuf;
use thiserror::Error;

// Pyramid Structure: Error Definitions -> Implementations
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP processing error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Database error: {0}")]
    Database(#[from] sled::Error),

    #[error("Missing configuration: {0}")]
    MissingConfig(&'static str),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(&'static str),

    #[error("Path error: {0}")]
    Path(PathBuf),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Shutdown error: {0}")]
    Shutdown(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// Layer 2: Result Type
pub type Result<T> = std::result::Result<T, AppError>;

// Layer 3: Error Context Extensions
pub trait ErrorExt<T> {
    fn with_context<C>(self, ctx: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}

impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: Into<AppError>,
{
    fn with_context<C>(self, ctx: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let err: AppError = e.into();
            AppError::Runtime(format!("{}: {}", ctx, err))
        })
    }
}

// Layer 4: Error Conversion
impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Runtime(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Runtime(s.to_string())
    }
}

// Layer 5: Helper Functions
pub(crate) fn path_error(path: impl Into<PathBuf>) -> AppError {
    AppError::Path(path.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let err: Result<()> = Err(AppError::MissingConfig("test"));
        let ctx_err = err.with_context("operation failed");
        assert!(ctx_err.is_err());
        assert!(matches!(ctx_err.unwrap_err(), AppError::Runtime(_)));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err: AppError = io_err.into();
        assert!(matches!(err, AppError::Io(_)));
    }
}
