//! Error Module - Pyramidal Structure
//! Layer 1: Error Types
//! Layer 2: Error Context
//! Layer 3: Error Conversion
//! Layer 4: Error Handling
//! Layer 5: Error Reporting

use std::path::PathBuf;
use thiserror::Error;
use std::backtrace::Backtrace;
use tracing::{error, warn};

// Layer 1: Core Error Types
#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("IO error: {source} at {path:?}")]
    Io {
        #[from]
        source: std::io::Error,
        path: Option<PathBuf>,
        backtrace: Backtrace,
    },

    #[error("ZIP error: {0}")]
    Zip(String, #[backtrace] Backtrace),

    #[error("Storage error: {0}")]
    Storage(String, #[backtrace] Backtrace),

    #[error("Runtime error: {0}")]
    Runtime(String, #[backtrace] Backtrace),
}

// Layer 2: Error Context
#[derive(Debug)]
pub struct ErrorContext {
    pub file: Option<PathBuf>,
    pub operation: String,
    pub timestamp: std::time::SystemTime,
}

// Layer 3: Error Extensions
pub trait ErrorExt {
    fn with_path(self, path: impl Into<PathBuf>) -> Self;
    fn with_context(self, context: impl Into<String>) -> Self;
}

impl ErrorExt for ProcessorError {
    fn with_path(self, path: impl Into<PathBuf>) -> Self {
        match self {
            Self::Io { source, .. } => Self::Io {
                source,
                path: Some(path.into()),
            },
            other => other,
        }
    }

    fn with_context(self, context: impl Into<String>) -> Self {
        match self {
            Self::Zip(_) => Self::Zip(context.into()),
            Self::Storage(_) => Self::Storage(context.into()),
            Self::Runtime(_) => Self::Runtime(context.into()),
            other => other,
        }
    }
}

// Layer 4: Result Type
pub type Result<T> = std::result::Result<T, ProcessorError>;

// Layer 5: Error Reporting
#[derive(Debug)]
pub struct ErrorReport {
    error: ProcessorError,
    context: ErrorContext,
    backtrace: Option<std::backtrace::Backtrace>,
}

impl ErrorReport {
    pub fn new(error: ProcessorError) -> Self {
        Self {
            error,
            context: ErrorContext {
                file: None,
                operation: String::new(),
                timestamp: std::time::SystemTime::now(),
            },
            backtrace: std::backtrace::Backtrace::capture().into(),
        }
    }

    pub fn print_report(&self) -> String {
        use std::fmt::Write;
        let mut output = String::new();

        writeln!(&mut output, "Error: {}", self.error).unwrap();
        if let Some(path) = &self.context.file {
            writeln!(&mut output, "File: {}", path.display()).unwrap();
        }
        writeln!(&mut output, "Operation: {}", self.context.operation).unwrap();
        if let Some(bt) = &self.backtrace {
            writeln!(&mut output, "Backtrace:\n{}", bt).unwrap();
        }

        output
    }
}
