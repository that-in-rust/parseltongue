//! Error Handling - Pyramidal Structure
//! Layer 1: Error Types & Enums
//! Layer 2: Error Implementations
//! Layer 3: Result Type Aliases
//! Layer 4: Error Conversion
//! Layer 5: Helper Functions

use std::path::PathBuf;
use thiserror::Error;

// Layer 1: Core Error Types
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
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
pub type Result<T> = std::result::Result<T, Error>;

// Layer 3: Error Context Extensions
pub trait ErrorExt<T> {
    fn with_context<C>(self, ctx: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}

impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn with_context<C>(self, ctx: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let err: Error = e.into();
            Error::Runtime(format!("{}: {}", ctx, err))
        })
    }
}

// Layer 4: Helper Functions
pub(crate) fn path_error(path: impl Into<PathBuf>) -> Error {
    Error::Path(path.into())
}

// Layer 5: Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let err: Result<()> = Err(Error::MissingConfig("test"));
        let ctx_err = err.with_context("operation failed");
        assert!(ctx_err.is_err());
    }
}
