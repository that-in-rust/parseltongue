// Level 4: Error Management
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("ZIP processing error: {0}")]
    Processing(String),
    
    #[error("Encoding error: {0}")]
    Encoding(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>; 