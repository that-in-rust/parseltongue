//! Error types for LLM-to-cozoDB-writer.

use thiserror::Error;
use parseltongue_core::error::ParseltongError;

/// LLM writer tool specific errors
#[derive(Debug, Error)]
pub enum LlmWriterError {
    /// LLM API communication errors
    #[error("LLM API error: {status} - {message}")]
    LlmApiError {
        status: u16,
        message: String,
    },

    /// Database query errors
    #[error("Database query failed: {query} - {reason}")]
    DatabaseQueryError {
        query: String,
        reason: String,
    },

    /// LLM response parsing errors
    #[error("Failed to parse LLM response: {reason}")]
    ResponseParseError {
        reason: String,
    },

    /// Temporal change validation errors
    #[error("Temporal change validation failed: {field} - {reason}")]
    ValidationError {
        field: String,
        reason: String,
    },

    /// Configuration errors
    #[error("Configuration error: {field} - {reason}")]
    ConfigurationError {
        field: String,
        reason: String,
    },

    /// Rate limiting errors
    #[error("Rate limit exceeded: retry after {seconds}s")]
    RateLimitError {
        seconds: u64,
    },

    /// Authentication errors
    #[error("Authentication failed: {reason}")]
    AuthenticationError {
        reason: String,
    },

    /// Request timeout errors
    #[error("Request timeout after {seconds}s")]
    TimeoutError {
        seconds: u64,
    },
}

impl From<LlmWriterError> for ParseltongError {
    fn from(err: LlmWriterError) -> Self {
        match err {
            LlmWriterError::DatabaseQueryError { query, reason } => {
                ParseltongError::DatabaseError {
                    details: format!("Query: {} - {}", query, reason),
                }
            }
            LlmWriterError::ValidationError { field, reason } => {
                ParseltongError::ValidationError {
                    field,
                    expected: "valid temporal change".to_string(),
                    actual: reason,
                }
            }
            LlmWriterError::ConfigurationError { field, reason } => {
                ParseltongError::ConfigurationError {
                    details: format!("{}: {}", field, reason),
                }
            }
            LlmWriterError::ResponseParseError { reason } => {
                ParseltongError::ParseError {
                    reason,
                    location: "LLM response".to_string(),
                }
            }
            _ => ParseltongError::LlmError {
                reason: err.to_string(),
            },
        }
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, LlmWriterError>;