//! Error types for Tool 1: folder-to-cozoDB-streamer
//! Following structured error patterns with proper context

use std::path::PathBuf;

/// Tool-specific error types with detailed context
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("File discovery failed: {0}")]
    FileDiscovery(String),

    #[error("File filtering error: {0}")]
    FileFilter(String),

    #[error("Tree-sitter parsing failed: {0}")]
    ParseError(String),

    #[error("Chunking strategy failed: {0}")]
    ChunkingError(String),

    #[error("CozoDB connection error: {0}")]
    CozoConnection(String),

    #[error("CozoDB ingestion error: {0}")]
    CozoIngestion(String),

    #[error("Stream processing error: {0}")]
    StreamError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UUID generation error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Performance contract violation: {0}")]
    PerformanceViolation(String),

    #[error("Resource not found: {path:?}")]
    ResourceNotFound { path: PathBuf },

    #[error("Permission denied: {path:?}")]
    PermissionDenied { path: PathBuf },

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for Tool operations
pub type ToolResult<T> = Result<T, ToolError>;

impl From<cozo::Error> for ToolError {
    fn from(error: cozo::Error) -> Self {
        Self::CozoConnection(error.to_string())
    }
}

impl ToolError {
    /// Create a file discovery error with context
    pub fn file_discovery(msg: impl Into<String>) -> Self {
        Self::FileDiscovery(msg.into())
    }

    /// Create a parse error with context
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::ParseError(msg.into())
    }

    /// Create a chunking error with context
    pub fn chunking_error(msg: impl Into<String>) -> Self {
        Self::ChunkingError(msg.into())
    }

    /// Create a CozoDB connection error with context
    pub fn cozo_connection(msg: impl Into<String>) -> Self {
        Self::CozoConnection(msg.into())
    }

    /// Create a CozoDB ingestion error with context
    pub fn cozo_ingestion(msg: impl Into<String>) -> Self {
        Self::CozoIngestion(msg.into())
    }

    /// Create a stream error with context
    pub fn stream_error(msg: impl Into<String>) -> Self {
        Self::StreamError(msg.into())
    }

    /// Create an invalid configuration error with context
    pub fn invalid_config(msg: impl Into<String>) -> Self {
        Self::InvalidConfig(msg.into())
    }

    /// Create a performance violation error with context
    pub fn performance_violation(msg: impl Into<String>) -> Self {
        Self::PerformanceViolation(msg.into())
    }

    /// Create a resource not found error
    pub fn resource_not_found(path: impl Into<PathBuf>) -> Self {
        Self::ResourceNotFound { path: path.into() }
    }

    /// Create a permission denied error
    pub fn permission_denied(path: impl Into<PathBuf>) -> Self {
        Self::PermissionDenied { path: path.into() }
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
