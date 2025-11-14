//! Error types for pt02-llm-cozodb-to-context-writer
//!
//! Following S01 principle #6: Structured Error Handling
//! - thiserror for library errors (minimal, mostly re-exports)
//! - anyhow for application errors (used in main.rs)

use thiserror::Error;

/// PT02 error types (minimal - most errors come from parseltongue-core)
#[derive(Error, Debug)]
pub enum ContextWriterError {
    /// Database connection or query error
    #[error("Database error: {reason}")]
    DatabaseError { reason: String },

    /// File I/O error during export
    #[error("File I/O error: {reason}")]
    IoError { reason: String },

    /// JSON serialization error
    #[error("Serialization error: {reason}")]
    SerializationError { reason: String },
}

/// Re-export parseltongue-core errors for convenience
pub use parseltongue_core::error::{ParseltongError, Result};
