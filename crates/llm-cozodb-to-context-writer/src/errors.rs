//! Error handling for parseltongue-03 context writer.

use thiserror::Error;
use std::collections::HashMap;

/// Result type alias for parseltongue-03
pub type Result<T> = std::result::Result<T, ContextWriterError>;

/// Context writer error types
#[derive(Debug, Error)]
pub enum ContextWriterError {
    /// LLM API communication errors
    #[error("LLM API error: {status} - {message}")]
    LlmApiError { status: u16, message: String },

    /// Database query errors
    #[error("Database query failed: {query} - {reason}")]
    DatabaseQueryError { query: String, reason: String },

    /// Context generation errors
    #[error("Context generation failed: {entity} - {reason}")]
    ContextGenerationError { entity: String, reason: String },

    /// File I/O errors
    #[error("File operation failed: {path} - {reason}")]
    FileError { path: String, reason: String },

    /// Serialization errors
    #[error("JSON serialization failed: {data} - {reason}")]
    SerializationError { data: String, reason: String },

    /// Configuration errors
    #[error("Configuration error: {field} - {reason}")]
    ConfigurationError { field: String, reason: String },

    /// Rate limiting errors
    #[error("Rate limit exceeded: retry after {seconds}s")]
    RateLimitError { seconds: u64 },

    /// Token limit errors
    #[error("Token limit exceeded: {tokens} > {limit}")]
    TokenLimitError { tokens: usize, limit: usize },

    /// Relevance threshold errors
    #[error("Relevance threshold not met: {score} < {threshold}")]
    RelevanceError { score: f32, threshold: f32 },

    /// Graph analysis errors
    #[error("Graph analysis failed: {operation} - {reason}")]
    GraphAnalysisError { operation: String, reason: String },

    /// Network connectivity errors
    #[error("Network connectivity error: {endpoint} - {reason}")]
    NetworkError { endpoint: String, reason: String },

    /// Authentication errors
    #[error("Authentication failed: {service}")]
    AuthenticationError { service: String },

    /// Timeout errors
    #[error("Operation timed out: {operation} after {seconds}s")]
    TimeoutError { operation: String, seconds: u64 },

    /// Context size exceeds token limit (PRD requirement)
    #[error("Context too large: {actual} tokens > {limit} token limit")]
    ContextTooLarge { actual: usize, limit: usize },

    /// Database connection or operation errors
    #[error("Database error: {reason}")]
    DatabaseError { reason: String },
}

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum ErrorRecoveryStrategy {
    /// Retry with exponential backoff
    RetryWithBackoff { max_retries: u32, base_delay_ms: u64 },
    /// Use fallback entity
    UseFallback { fallback_entity: String },
    /// Skip and continue
    SkipAndContinue,
    /// Abort operation
    Abort,
}

/// Error context for debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub entity_id: Option<String>,
    pub attempt_count: u32,
    pub timestamp: std::time::SystemTime,
    pub additional_info: HashMap<String, String>,
}

impl Default for ErrorRecoveryStrategy {
    fn default() -> Self {
        ErrorRecoveryStrategy::RetryWithBackoff {
            max_retries: 3,
            base_delay_ms: 1000,
        }
    }
}