//! Error Types and Handling
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Error API
//! - Public error types
//! - Result type alias
//! - Error conversion
//! 
//! Level 3: Error Categories
//! - IO errors
//! - Runtime errors
//! - Domain errors
//! 
//! Level 2: Error Implementation
//! - Error conversion
//! - Error context
//! - Error formatting
//! 
//! Level 1 (Base): Error Infrastructure
//! - Basic error types
//! - Error traits
//! - Common functionality

use std::{io, path::PathBuf};
use thiserror::Error;

// Design Choice: Using thiserror for error definitions
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Database error: {0}")]
    Database(#[from] sled::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Operation timed out")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error("Task cancelled")]
    Cancelled,

    #[error("Shutdown in progress")]
    Shutdown,

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),
}

// Design Choice: Using type alias for Result
pub type Result<T> = std::result::Result<T, Error>;
