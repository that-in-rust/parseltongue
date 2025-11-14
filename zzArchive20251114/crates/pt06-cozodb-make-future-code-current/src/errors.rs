use thiserror::Error;

/// Errors that can occur during state reset operations
#[derive(Error, Debug)]
pub enum StateResetError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Re-indexing failed: {0}")]
    ReindexingFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = StateResetError::Database("connection failed".to_string());
        assert!(err.to_string().contains("Database error"));
    }
}
