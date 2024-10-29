// Level 4: Error Management
// - Custom error types
// - Error conversion traits
// - Error context

use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("ZIP error: {0}")]
    Zip(String),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, Error>; 