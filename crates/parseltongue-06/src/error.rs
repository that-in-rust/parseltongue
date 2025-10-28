//! Error types for Tool 5: CozoDB State Reset
//! Following structured error patterns with proper context

use std::path::PathBuf;

/// Tool 5 specific error types with detailed context
#[derive(Debug, thiserror::Error)]
pub enum Tool5Error {
    #[error("CozoDB connection error: {0}")]
    CozoConnection(String),

    #[error("State reset error: {0}")]
    StateReset(String),

    #[error("Metadata backup error: {0}")]
    MetadataBackup(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Path not found: {path:?}")]
    PathNotFound { path: PathBuf },

    #[error("Permission denied: {path:?}")]
    PermissionDenied { path: PathBuf },

    #[error("Project path does not exist: {path:?}")]
    ProjectPathNotFound { path: PathBuf },

    #[error("Parseltongue project not found in: {path:?}")]
    ParseltongueProjectNotFound { path: PathBuf },

    #[error("Git operation failed: {0}")]
    GitOperation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

/// Result type for Tool 5 operations
pub type Tool5Result<T> = Result<T, Tool5Error>;

impl Tool5Error {
    /// Create a CozoDB connection error with context
    pub fn cozo_connection(msg: impl Into<String>) -> Self {
        Self::CozoConnection(msg.into())
    }

    /// Create a state reset error with context
    pub fn state_reset(msg: impl Into<String>) -> Self {
        Self::StateReset(msg.into())
    }

    /// Create a metadata backup error with context
    pub fn metadata_backup(msg: impl Into<String>) -> Self {
        Self::MetadataBackup(msg.into())
    }

    /// Create a file system error with context
    pub fn file_system(msg: impl Into<String>) -> Self {
        Self::FileSystem(msg.into())
    }

    /// Create a path not found error
    pub fn path_not_found(path: impl Into<PathBuf>) -> Self {
        Self::PathNotFound { path: path.into() }
    }

    /// Create a permission denied error
    pub fn permission_denied(path: impl Into<PathBuf>) -> Self {
        Self::PermissionDenied { path: path.into() }
    }

    /// Create a project path not found error
    pub fn project_path_not_found(path: impl Into<PathBuf>) -> Self {
        Self::ProjectPathNotFound { path: path.into() }
    }

    /// Create a Parseltongue project not found error
    pub fn parseltongue_project_not_found(path: impl Into<PathBuf>) -> Self {
        Self::ParseltongueProjectNotFound { path: path.into() }
    }

    /// Create a git operation error with context
    pub fn git_operation(msg: impl Into<String>) -> Self {
        Self::GitOperation(msg.into())
    }

    /// Create an internal error with context
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Create a configuration error with context
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::Configuration(msg.into())
    }

    /// Create a validation error with context
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
}

// Implement From for common error types
impl From<parseltongue_02::error::ToolError> for Tool5Error {
    fn from(error: parseltongue_02::error::ToolError) -> Self {
        match error {
            parseltongue_02::error::ToolError::CozoConnection(msg) => Self::cozo_connection(msg),
            parseltongue_02::error::ToolError::FileDiscovery(msg) => Self::file_system(msg),
            parseltongue_02::error::ToolError::Io(err) => Self::Io(err),
            parseltongue_02::error::ToolError::Serialization(err) => Self::Serialization(err),
            parseltongue_02::error::ToolError::Uuid(err) => Self::Internal(err.to_string()),
            _ => Self::internal(format!("Tool2 error: {}", error)),
        }
    }
}