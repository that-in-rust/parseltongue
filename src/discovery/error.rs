//! Error types for the discovery system
//! 
//! Provides comprehensive error handling for discovery operations with
//! structured error types that enable proper error handling and debugging.

use std::time::Duration;
use thiserror::Error;

/// Comprehensive error handling for discovery operations
/// 
/// Follows the design principle of exhaustive error enumeration to ensure
/// all possible failure conditions are handled explicitly.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryError {
    #[error("Entity not found: {name}")]
    EntityNotFound { name: String },
    
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid query: {reason}")]
    InvalidQuery { reason: String },
    
    #[error("Query timeout: {query} took longer than {limit:?}")]
    QueryTimeout { query: String, limit: Duration },
    
    #[error("Index corruption detected: {index_type}")]
    IndexCorruption { index_type: String },
    
    #[error("Performance contract violation: {operation} took {actual:?}, expected <{limit:?}")]
    PerformanceViolation {
        operation: String,
        actual: Duration,
        limit: Duration,
    },
    
    #[error("Memory limit exceeded: {current_mb}MB > {limit_mb}MB")]
    MemoryLimitExceeded { current_mb: usize, limit_mb: usize },
    
    #[error("Concurrent access error: {operation}")]
    ConcurrencyError { operation: String },
    
    #[error("Serialization error: {message}")]
    SerializationError { message: String },
    
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl DiscoveryError {
    /// Create an EntityNotFound error
    pub fn entity_not_found(name: impl Into<String>) -> Self {
        Self::EntityNotFound { name: name.into() }
    }
    
    /// Create a FileNotFound error
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound { path: path.into() }
    }
    
    /// Create an InvalidQuery error
    pub fn invalid_query(reason: impl Into<String>) -> Self {
        Self::InvalidQuery { reason: reason.into() }
    }
    
    /// Create a QueryTimeout error
    pub fn query_timeout(query: impl Into<String>, limit: Duration) -> Self {
        Self::QueryTimeout { 
            query: query.into(), 
            limit 
        }
    }
    
    /// Create an IndexCorruption error
    pub fn index_corruption(index_type: impl Into<String>) -> Self {
        Self::IndexCorruption { 
            index_type: index_type.into() 
        }
    }
    
    /// Create a PerformanceViolation error
    pub fn performance_violation(
        operation: impl Into<String>, 
        actual: Duration, 
        limit: Duration
    ) -> Self {
        Self::PerformanceViolation {
            operation: operation.into(),
            actual,
            limit,
        }
    }
    
    /// Create a MemoryLimitExceeded error
    pub fn memory_limit_exceeded(current_mb: usize, limit_mb: usize) -> Self {
        Self::MemoryLimitExceeded { current_mb, limit_mb }
    }
    
    /// Create a ConcurrencyError
    pub fn concurrency_error(operation: impl Into<String>) -> Self {
        Self::ConcurrencyError { 
            operation: operation.into() 
        }
    }
    
    /// Create a SerializationError
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::SerializationError { 
            message: message.into() 
        }
    }
    
    /// Create an Internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal { 
            message: message.into() 
        }
    }
    
    /// Check if this error indicates a performance issue
    pub fn is_performance_issue(&self) -> bool {
        matches!(self, 
            Self::QueryTimeout { .. } | 
            Self::PerformanceViolation { .. } |
            Self::MemoryLimitExceeded { .. }
        )
    }
    
    /// Check if this error indicates a data integrity issue
    pub fn is_data_integrity_issue(&self) -> bool {
        matches!(self, 
            Self::IndexCorruption { .. } |
            Self::Internal { .. }
        )
    }
    
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(self, 
            Self::EntityNotFound { .. } |
            Self::FileNotFound { .. } |
            Self::InvalidQuery { .. } |
            Self::QueryTimeout { .. } |
            Self::ConcurrencyError { .. }
        )
    }
    
    /// Get error category for logging and metrics
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::EntityNotFound { .. } | Self::FileNotFound { .. } => ErrorCategory::NotFound,
            Self::InvalidQuery { .. } => ErrorCategory::InvalidInput,
            Self::QueryTimeout { .. } | Self::PerformanceViolation { .. } => ErrorCategory::Performance,
            Self::MemoryLimitExceeded { .. } => ErrorCategory::Resource,
            Self::IndexCorruption { .. } => ErrorCategory::DataIntegrity,
            Self::ConcurrencyError { .. } => ErrorCategory::Concurrency,
            Self::SerializationError { .. } => ErrorCategory::Serialization,
            Self::Internal { .. } => ErrorCategory::Internal,
        }
    }
}

/// Error categories for metrics and logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    NotFound,
    InvalidInput,
    Performance,
    Resource,
    DataIntegrity,
    Concurrency,
    Serialization,
    Internal,
}

impl ErrorCategory {
    /// Get human-readable name for the category
    pub fn name(&self) -> &'static str {
        match self {
            Self::NotFound => "not_found",
            Self::InvalidInput => "invalid_input",
            Self::Performance => "performance",
            Self::Resource => "resource",
            Self::DataIntegrity => "data_integrity",
            Self::Concurrency => "concurrency",
            Self::Serialization => "serialization",
            Self::Internal => "internal",
        }
    }
}

/// Result type alias for discovery operations
pub type DiscoveryResult<T> = Result<T, DiscoveryError>;

/// Context-rich error handling for CLI operations
/// 
/// Provides additional context for errors that occur during CLI operations,
/// making them more actionable for users.
pub fn add_discovery_context<T>(
    result: DiscoveryResult<T>,
    context: &str,
) -> anyhow::Result<T> {
    result.map_err(|e| anyhow::anyhow!("Discovery operation failed: {}: {}", context, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_error_creation() {
        let error = DiscoveryError::entity_not_found("test_function");
        assert!(matches!(error, DiscoveryError::EntityNotFound { name } if name == "test_function"));
        
        let error = DiscoveryError::file_not_found("src/main.rs");
        assert!(matches!(error, DiscoveryError::FileNotFound { path } if path == "src/main.rs"));
        
        let error = DiscoveryError::invalid_query("empty query");
        assert!(matches!(error, DiscoveryError::InvalidQuery { reason } if reason == "empty query"));
        
        let error = DiscoveryError::query_timeout("list_all", Duration::from_secs(5));
        assert!(matches!(error, DiscoveryError::QueryTimeout { query, limit } 
            if query == "list_all" && limit == Duration::from_secs(5)));
    }
    
    #[test]
    fn test_error_classification() {
        let timeout_error = DiscoveryError::query_timeout("test", Duration::from_secs(1));
        assert!(timeout_error.is_performance_issue());
        assert!(!timeout_error.is_data_integrity_issue());
        assert!(timeout_error.is_recoverable());
        
        let corruption_error = DiscoveryError::index_corruption("entity_index");
        assert!(!corruption_error.is_performance_issue());
        assert!(corruption_error.is_data_integrity_issue());
        assert!(!corruption_error.is_recoverable());
        
        let not_found_error = DiscoveryError::entity_not_found("missing");
        assert!(!not_found_error.is_performance_issue());
        assert!(!not_found_error.is_data_integrity_issue());
        assert!(not_found_error.is_recoverable());
    }
    
    #[test]
    fn test_error_categories() {
        assert_eq!(DiscoveryError::entity_not_found("test").category(), ErrorCategory::NotFound);
        assert_eq!(DiscoveryError::invalid_query("test").category(), ErrorCategory::InvalidInput);
        assert_eq!(DiscoveryError::query_timeout("test", Duration::from_secs(1)).category(), ErrorCategory::Performance);
        assert_eq!(DiscoveryError::memory_limit_exceeded(100, 50).category(), ErrorCategory::Resource);
        assert_eq!(DiscoveryError::index_corruption("test").category(), ErrorCategory::DataIntegrity);
        assert_eq!(DiscoveryError::concurrency_error("test").category(), ErrorCategory::Concurrency);
        assert_eq!(DiscoveryError::serialization_error("test").category(), ErrorCategory::Serialization);
        assert_eq!(DiscoveryError::internal("test").category(), ErrorCategory::Internal);
    }
    
    #[test]
    fn test_error_category_names() {
        assert_eq!(ErrorCategory::NotFound.name(), "not_found");
        assert_eq!(ErrorCategory::InvalidInput.name(), "invalid_input");
        assert_eq!(ErrorCategory::Performance.name(), "performance");
        assert_eq!(ErrorCategory::Resource.name(), "resource");
        assert_eq!(ErrorCategory::DataIntegrity.name(), "data_integrity");
        assert_eq!(ErrorCategory::Concurrency.name(), "concurrency");
        assert_eq!(ErrorCategory::Serialization.name(), "serialization");
        assert_eq!(ErrorCategory::Internal.name(), "internal");
    }
    
    #[test]
    fn test_error_display() {
        let error = DiscoveryError::entity_not_found("test_function");
        assert_eq!(error.to_string(), "Entity not found: test_function");
        
        let error = DiscoveryError::performance_violation(
            "list_entities", 
            Duration::from_millis(150), 
            Duration::from_millis(100)
        );
        assert!(error.to_string().contains("Performance contract violation"));
        assert!(error.to_string().contains("list_entities"));
        assert!(error.to_string().contains("150ms"));
        assert!(error.to_string().contains("100ms"));
    }
    
    #[test]
    fn test_add_discovery_context() {
        let error = DiscoveryError::entity_not_found("test");
        let result: DiscoveryResult<()> = Err(error);
        
        let context_result = add_discovery_context(result, "testing context");
        assert!(context_result.is_err());
        
        let error_message = context_result.unwrap_err().to_string();
        assert!(error_message.contains("Discovery operation failed"));
        assert!(error_message.contains("testing context"));
        assert!(error_message.contains("Entity not found: test"));
    }
}