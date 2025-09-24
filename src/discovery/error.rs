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
            Self::PerformanceViolation { .. } |
            Self::MemoryLimitExceeded { .. } |
            Self::ConcurrencyError { .. } |
            Self::SerializationError { .. }
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

/// Performance contract violation detector
/// 
/// Monitors operation timing and detects violations of performance contracts.
/// Used to ensure discovery operations meet their performance guarantees.
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    /// Discovery operation time limit (100ms)
    discovery_time_limit: Duration,
    /// Existing query time limit (50μs)
    existing_query_limit: Duration,
    /// Memory usage limit increase (20%)
    memory_limit_increase_percent: f64,
}

impl PerformanceMonitor {
    /// Create a new performance monitor with default limits
    pub fn new() -> Self {
        Self {
            discovery_time_limit: Duration::from_millis(100),
            existing_query_limit: Duration::from_micros(50),
            memory_limit_increase_percent: 20.0,
        }
    }
    
    /// Create a performance monitor with custom limits
    pub fn with_limits(
        discovery_time_limit: Duration,
        existing_query_limit: Duration,
        memory_limit_increase_percent: f64,
    ) -> Self {
        Self {
            discovery_time_limit,
            existing_query_limit,
            memory_limit_increase_percent,
        }
    }
    
    /// Check if a discovery operation meets its performance contract
    pub fn check_discovery_performance(
        &self,
        operation: &str,
        elapsed: Duration,
    ) -> Result<(), DiscoveryError> {
        if elapsed > self.discovery_time_limit {
            return Err(DiscoveryError::performance_violation(
                operation,
                elapsed,
                self.discovery_time_limit,
            ));
        }
        Ok(())
    }
    
    /// Check if an existing query meets its performance contract
    pub fn check_existing_query_performance(
        &self,
        operation: &str,
        elapsed: Duration,
    ) -> Result<(), DiscoveryError> {
        if elapsed > self.existing_query_limit {
            return Err(DiscoveryError::performance_violation(
                operation,
                elapsed,
                self.existing_query_limit,
            ));
        }
        Ok(())
    }
    
    /// Check if memory usage is within acceptable limits
    pub fn check_memory_usage(
        &self,
        current_mb: usize,
        baseline_mb: usize,
    ) -> Result<(), DiscoveryError> {
        let increase_percent = if baseline_mb > 0 {
            ((current_mb as f64 - baseline_mb as f64) / baseline_mb as f64) * 100.0
        } else {
            0.0
        };
        
        if increase_percent > self.memory_limit_increase_percent {
            let limit_mb = baseline_mb + ((baseline_mb as f64 * self.memory_limit_increase_percent / 100.0) as usize);
            return Err(DiscoveryError::memory_limit_exceeded(current_mb, limit_mb));
        }
        Ok(())
    }
    
    /// Get performance contract summary for reporting
    pub fn contract_summary(&self) -> String {
        format!(
            "Performance Contracts:\n\
             - Discovery operations: <{:?}\n\
             - Existing queries: <{:?}\n\
             - Memory increase: <{:.1}%",
            self.discovery_time_limit,
            self.existing_query_limit,
            self.memory_limit_increase_percent
        )
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

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

/// Enhanced error context for CLI user experience
/// 
/// Provides actionable error messages with suggestions for resolution.
pub struct ErrorContext {
    operation: String,
    suggestions: Vec<String>,
    related_commands: Vec<String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            suggestions: Vec::new(),
            related_commands: Vec::new(),
        }
    }
    
    /// Add a suggestion for resolving the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
    
    /// Add a related command that might help
    pub fn with_related_command(mut self, command: impl Into<String>) -> Self {
        self.related_commands.push(command.into());
        self
    }
    
    /// Format the error with context for CLI display
    pub fn format_error(&self, error: &DiscoveryError) -> String {
        let mut message = format!("❌ {} failed: {}\n", self.operation, error);
        
        // Add category-specific suggestions
        match error.category() {
            ErrorCategory::NotFound => {
                message.push_str("\n💡 Suggestions:\n");
                message.push_str("  • Check entity name spelling\n");
                message.push_str("  • Use 'parseltongue list-entities' to see available entities\n");
                message.push_str("  • Verify the file path exists\n");
            }
            ErrorCategory::InvalidInput => {
                message.push_str("\n💡 Suggestions:\n");
                message.push_str("  • Check command syntax and arguments\n");
                message.push_str("  • Use 'parseltongue --help' for usage information\n");
            }
            ErrorCategory::Performance => {
                message.push_str("\n⚠️  Performance Issue Detected:\n");
                message.push_str("  • This operation exceeded performance contracts\n");
                message.push_str("  • Consider using filters to reduce scope\n");
                message.push_str("  • Check system resources and load\n");
            }
            ErrorCategory::Resource => {
                message.push_str("\n🔧 Resource Issue:\n");
                message.push_str("  • System may be low on memory\n");
                message.push_str("  • Try reducing query scope or batch size\n");
                message.push_str("  • Consider restarting the application\n");
            }
            ErrorCategory::DataIntegrity => {
                message.push_str("\n🚨 Data Integrity Issue:\n");
                message.push_str("  • Internal data structures may be corrupted\n");
                message.push_str("  • Try re-ingesting the codebase\n");
                message.push_str("  • Report this issue if it persists\n");
            }
            ErrorCategory::Concurrency => {
                message.push_str("\n🔄 Concurrency Issue:\n");
                message.push_str("  • Multiple operations may be conflicting\n");
                message.push_str("  • Try the operation again\n");
                message.push_str("  • Consider sequential execution\n");
            }
            ErrorCategory::Serialization => {
                message.push_str("\n📄 Serialization Issue:\n");
                message.push_str("  • Data format may be incompatible\n");
                message.push_str("  • Check output format requirements\n");
                message.push_str("  • Try different output formats\n");
            }
            ErrorCategory::Internal => {
                message.push_str("\n🐛 Internal Error:\n");
                message.push_str("  • This is likely a bug in Parseltongue\n");
                message.push_str("  • Please report this issue with reproduction steps\n");
            }
        }
        
        // Add custom suggestions
        if !self.suggestions.is_empty() {
            message.push_str("\n💡 Additional Suggestions:\n");
            for suggestion in &self.suggestions {
                message.push_str(&format!("  • {}\n", suggestion));
            }
        }
        
        // Add related commands
        if !self.related_commands.is_empty() {
            message.push_str("\n🔗 Related Commands:\n");
            for command in &self.related_commands {
                message.push_str(&format!("  • {}\n", command));
            }
        }
        
        message
    }
}

/// Create context-rich error for CLI operations
pub fn create_cli_error(
    error: DiscoveryError,
    operation: &str,
) -> anyhow::Error {
    let context = match &error {
        DiscoveryError::EntityNotFound { name } => {
            ErrorContext::new(operation)
                .with_suggestion(format!("Try 'parseltongue list-entities --type functions' to see available functions"))
                .with_suggestion(format!("Search for similar names: 'parseltongue list-entities | grep {}'", name))
                .with_related_command("parseltongue list-entities")
                .with_related_command("parseltongue entities-in-file <file>")
        }
        DiscoveryError::FileNotFound { path } => {
            ErrorContext::new(operation)
                .with_suggestion("Check if the file path is correct")
                .with_suggestion("Use relative paths from the project root")
                .with_related_command("parseltongue list-entities --json | jq '.[] | .file_path' | sort | uniq")
        }
        DiscoveryError::InvalidQuery { .. } => {
            ErrorContext::new(operation)
                .with_suggestion("Check command syntax with --help")
                .with_suggestion("Verify all required arguments are provided")
                .with_related_command("parseltongue --help")
        }
        DiscoveryError::QueryTimeout { .. } => {
            ErrorContext::new(operation)
                .with_suggestion("Try reducing the scope of your query")
                .with_suggestion("Use filters to limit results")
                .with_suggestion("Check system performance and load")
        }
        DiscoveryError::PerformanceViolation { .. } => {
            ErrorContext::new(operation)
                .with_suggestion("This operation exceeded performance contracts")
                .with_suggestion("Consider using more specific filters")
                .with_suggestion("Check if the system is under heavy load")
        }
        _ => ErrorContext::new(operation),
    };
    
    anyhow::anyhow!("{}", context.format_error(&error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    // Basic error creation tests
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
    fn test_all_error_variants_creation() {
        // Test all error creation methods
        let _ = DiscoveryError::entity_not_found("test");
        let _ = DiscoveryError::file_not_found("test.rs");
        let _ = DiscoveryError::invalid_query("invalid");
        let _ = DiscoveryError::query_timeout("query", Duration::from_secs(1));
        let _ = DiscoveryError::index_corruption("index");
        let _ = DiscoveryError::performance_violation("op", Duration::from_millis(200), Duration::from_millis(100));
        let _ = DiscoveryError::memory_limit_exceeded(200, 100);
        let _ = DiscoveryError::concurrency_error("operation");
        let _ = DiscoveryError::serialization_error("message");
        let _ = DiscoveryError::internal("internal error");
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
        
        let performance_error = DiscoveryError::performance_violation(
            "test", Duration::from_millis(200), Duration::from_millis(100)
        );
        assert!(performance_error.is_performance_issue());
        assert!(!performance_error.is_data_integrity_issue());
        assert!(performance_error.is_recoverable());
        
        let memory_error = DiscoveryError::memory_limit_exceeded(200, 100);
        assert!(memory_error.is_performance_issue());
        assert!(!memory_error.is_data_integrity_issue());
        assert!(memory_error.is_recoverable());
        
        let internal_error = DiscoveryError::internal("test");
        assert!(!internal_error.is_performance_issue());
        assert!(internal_error.is_data_integrity_issue());
        assert!(!internal_error.is_recoverable());
    }
    
    #[test]
    fn test_error_categories() {
        assert_eq!(DiscoveryError::entity_not_found("test").category(), ErrorCategory::NotFound);
        assert_eq!(DiscoveryError::file_not_found("test").category(), ErrorCategory::NotFound);
        assert_eq!(DiscoveryError::invalid_query("test").category(), ErrorCategory::InvalidInput);
        assert_eq!(DiscoveryError::query_timeout("test", Duration::from_secs(1)).category(), ErrorCategory::Performance);
        assert_eq!(DiscoveryError::performance_violation("test", Duration::from_millis(200), Duration::from_millis(100)).category(), ErrorCategory::Performance);
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
        
        let error = DiscoveryError::memory_limit_exceeded(150, 100);
        assert!(error.to_string().contains("Memory limit exceeded"));
        assert!(error.to_string().contains("150MB"));
        assert!(error.to_string().contains("100MB"));
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
    
    // Performance monitor tests
    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert_eq!(monitor.discovery_time_limit, Duration::from_millis(100));
        assert_eq!(monitor.existing_query_limit, Duration::from_micros(50));
        assert_eq!(monitor.memory_limit_increase_percent, 20.0);
        
        let custom_monitor = PerformanceMonitor::with_limits(
            Duration::from_millis(200),
            Duration::from_micros(100),
            30.0,
        );
        assert_eq!(custom_monitor.discovery_time_limit, Duration::from_millis(200));
        assert_eq!(custom_monitor.existing_query_limit, Duration::from_micros(100));
        assert_eq!(custom_monitor.memory_limit_increase_percent, 30.0);
    }
    
    #[test]
    fn test_performance_monitor_discovery_performance_check() {
        let monitor = PerformanceMonitor::new();
        
        // Should pass for fast operations
        let result = monitor.check_discovery_performance("list_entities", Duration::from_millis(50));
        assert!(result.is_ok());
        
        // Should fail for slow operations
        let result = monitor.check_discovery_performance("list_entities", Duration::from_millis(150));
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert!(matches!(error, DiscoveryError::PerformanceViolation { .. }));
            assert!(error.to_string().contains("list_entities"));
            assert!(error.to_string().contains("150ms"));
            assert!(error.to_string().contains("100ms"));
        }
    }
    
    #[test]
    fn test_performance_monitor_existing_query_performance_check() {
        let monitor = PerformanceMonitor::new();
        
        // Should pass for fast queries
        let result = monitor.check_existing_query_performance("blast_radius", Duration::from_micros(25));
        assert!(result.is_ok());
        
        // Should fail for slow queries
        let result = monitor.check_existing_query_performance("blast_radius", Duration::from_micros(100));
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert!(matches!(error, DiscoveryError::PerformanceViolation { .. }));
            assert!(error.to_string().contains("blast_radius"));
        }
    }
    
    #[test]
    fn test_performance_monitor_memory_usage_check() {
        let monitor = PerformanceMonitor::new();
        
        // Should pass for acceptable memory usage
        let result = monitor.check_memory_usage(110, 100); // 10% increase
        assert!(result.is_ok());
        
        // Should pass at exactly the limit
        let result = monitor.check_memory_usage(120, 100); // 20% increase
        assert!(result.is_ok());
        
        // Should fail for excessive memory usage
        let result = monitor.check_memory_usage(130, 100); // 30% increase
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert!(matches!(error, DiscoveryError::MemoryLimitExceeded { .. }));
            assert!(error.to_string().contains("130MB"));
            assert!(error.to_string().contains("120MB"));
        }
        
        // Should handle zero baseline gracefully
        let result = monitor.check_memory_usage(100, 0);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_performance_monitor_contract_summary() {
        let monitor = PerformanceMonitor::new();
        let summary = monitor.contract_summary();
        
        assert!(summary.contains("Performance Contracts:"));
        assert!(summary.contains("Discovery operations: <100ms"));
        assert!(summary.contains("Existing queries: <50µs"));
        assert!(summary.contains("Memory increase: <20.0%"));
    }
    
    // Error context tests
    #[test]
    fn test_error_context_creation() {
        let context = ErrorContext::new("list entities")
            .with_suggestion("Try using filters")
            .with_related_command("parseltongue list-entities --type functions");
        
        assert_eq!(context.operation, "list entities");
        assert_eq!(context.suggestions.len(), 1);
        assert_eq!(context.related_commands.len(), 1);
    }
    
    #[test]
    fn test_error_context_formatting_not_found() {
        let context = ErrorContext::new("find entity");
        let error = DiscoveryError::entity_not_found("missing_function");
        let formatted = context.format_error(&error);
        
        assert!(formatted.contains("❌ find entity failed"));
        assert!(formatted.contains("Entity not found: missing_function"));
        assert!(formatted.contains("💡 Suggestions:"));
        assert!(formatted.contains("Check entity name spelling"));
        assert!(formatted.contains("parseltongue list-entities"));
    }
    
    #[test]
    fn test_error_context_formatting_performance() {
        let context = ErrorContext::new("slow operation");
        let error = DiscoveryError::performance_violation(
            "list_all", 
            Duration::from_millis(200), 
            Duration::from_millis(100)
        );
        let formatted = context.format_error(&error);
        
        assert!(formatted.contains("❌ slow operation failed"));
        assert!(formatted.contains("⚠️  Performance Issue Detected"));
        assert!(formatted.contains("exceeded performance contracts"));
        assert!(formatted.contains("Consider using filters"));
    }
    
    #[test]
    fn test_error_context_formatting_data_integrity() {
        let context = ErrorContext::new("corrupted operation");
        let error = DiscoveryError::index_corruption("entity_index");
        let formatted = context.format_error(&error);
        
        assert!(formatted.contains("❌ corrupted operation failed"));
        assert!(formatted.contains("🚨 Data Integrity Issue"));
        assert!(formatted.contains("Internal data structures may be corrupted"));
        assert!(formatted.contains("Try re-ingesting the codebase"));
    }
    
    #[test]
    fn test_error_context_formatting_with_custom_suggestions() {
        let context = ErrorContext::new("custom operation")
            .with_suggestion("Custom suggestion 1")
            .with_suggestion("Custom suggestion 2")
            .with_related_command("custom-command");
        
        let error = DiscoveryError::internal("test error");
        let formatted = context.format_error(&error);
        
        assert!(formatted.contains("💡 Additional Suggestions:"));
        assert!(formatted.contains("Custom suggestion 1"));
        assert!(formatted.contains("Custom suggestion 2"));
        assert!(formatted.contains("🔗 Related Commands:"));
        assert!(formatted.contains("custom-command"));
    }
    
    #[test]
    fn test_create_cli_error_entity_not_found() {
        let error = DiscoveryError::entity_not_found("missing_func");
        let cli_error = create_cli_error(error, "find entity");
        
        let error_message = cli_error.to_string();
        assert!(error_message.contains("❌ find entity failed"));
        assert!(error_message.contains("Entity not found: missing_func"));
        assert!(error_message.contains("Try 'parseltongue list-entities --type functions'"));
        assert!(error_message.contains("Search for similar names"));
    }
    
    #[test]
    fn test_create_cli_error_file_not_found() {
        let error = DiscoveryError::file_not_found("missing/file.rs");
        let cli_error = create_cli_error(error, "list file entities");
        
        let error_message = cli_error.to_string();
        assert!(error_message.contains("❌ list file entities failed"));
        assert!(error_message.contains("File not found: missing/file.rs"));
        assert!(error_message.contains("Check if the file path is correct"));
        assert!(error_message.contains("Use relative paths"));
    }
    
    #[test]
    fn test_create_cli_error_performance_violation() {
        let error = DiscoveryError::performance_violation(
            "slow_query", 
            Duration::from_millis(200), 
            Duration::from_millis(100)
        );
        let cli_error = create_cli_error(error, "execute query");
        
        let error_message = cli_error.to_string();
        assert!(error_message.contains("❌ execute query failed"));
        assert!(error_message.contains("Performance contract violation"));
        assert!(error_message.contains("exceeded performance contracts"));
        assert!(error_message.contains("Consider using more specific filters"));
    }
    
    // Comprehensive error condition tests
    #[test]
    fn test_all_error_categories_have_context_formatting() {
        let test_cases = vec![
            (DiscoveryError::entity_not_found("test"), ErrorCategory::NotFound),
            (DiscoveryError::invalid_query("test"), ErrorCategory::InvalidInput),
            (DiscoveryError::query_timeout("test", Duration::from_secs(1)), ErrorCategory::Performance),
            (DiscoveryError::memory_limit_exceeded(200, 100), ErrorCategory::Resource),
            (DiscoveryError::index_corruption("test"), ErrorCategory::DataIntegrity),
            (DiscoveryError::concurrency_error("test"), ErrorCategory::Concurrency),
            (DiscoveryError::serialization_error("test"), ErrorCategory::Serialization),
            (DiscoveryError::internal("test"), ErrorCategory::Internal),
        ];
        
        let context = ErrorContext::new("test operation");
        
        for (error, expected_category) in test_cases {
            assert_eq!(error.category(), expected_category);
            
            let formatted = context.format_error(&error);
            assert!(formatted.contains("❌ test operation failed"));
            assert!(
                formatted.contains("💡 Suggestions:") || 
                formatted.contains("⚠️") || 
                formatted.contains("🚨") ||
                formatted.contains("🔧") ||
                formatted.contains("🔄") ||
                formatted.contains("📄") ||
                formatted.contains("🐛")
            );
        }
    }
    
    #[test]
    fn test_error_recoverability_classification() {
        // Recoverable errors
        let recoverable_errors = vec![
            DiscoveryError::entity_not_found("test"),
            DiscoveryError::file_not_found("test"),
            DiscoveryError::invalid_query("test"),
            DiscoveryError::query_timeout("test", Duration::from_secs(1)),
            DiscoveryError::memory_limit_exceeded(200, 100),
            DiscoveryError::concurrency_error("test"),
        ];
        
        for error in recoverable_errors {
            assert!(error.is_recoverable(), "Error should be recoverable: {:?}", error);
        }
        
        // Non-recoverable errors
        let non_recoverable_errors = vec![
            DiscoveryError::index_corruption("test"),
            DiscoveryError::internal("test"),
        ];
        
        for error in non_recoverable_errors {
            assert!(!error.is_recoverable(), "Error should not be recoverable: {:?}", error);
        }
    }
    
    #[test]
    fn test_performance_issue_classification() {
        let performance_errors = vec![
            DiscoveryError::query_timeout("test", Duration::from_secs(1)),
            DiscoveryError::performance_violation("test", Duration::from_millis(200), Duration::from_millis(100)),
            DiscoveryError::memory_limit_exceeded(200, 100),
        ];
        
        for error in performance_errors {
            assert!(error.is_performance_issue(), "Error should be a performance issue: {:?}", error);
        }
        
        let non_performance_errors = vec![
            DiscoveryError::entity_not_found("test"),
            DiscoveryError::file_not_found("test"),
            DiscoveryError::invalid_query("test"),
            DiscoveryError::index_corruption("test"),
            DiscoveryError::concurrency_error("test"),
            DiscoveryError::serialization_error("test"),
            DiscoveryError::internal("test"),
        ];
        
        for error in non_performance_errors {
            assert!(!error.is_performance_issue(), "Error should not be a performance issue: {:?}", error);
        }
    }
    
    #[test]
    fn test_data_integrity_issue_classification() {
        let integrity_errors = vec![
            DiscoveryError::index_corruption("test"),
            DiscoveryError::internal("test"),
        ];
        
        for error in integrity_errors {
            assert!(error.is_data_integrity_issue(), "Error should be a data integrity issue: {:?}", error);
        }
        
        let non_integrity_errors = vec![
            DiscoveryError::entity_not_found("test"),
            DiscoveryError::file_not_found("test"),
            DiscoveryError::invalid_query("test"),
            DiscoveryError::query_timeout("test", Duration::from_secs(1)),
            DiscoveryError::performance_violation("test", Duration::from_millis(200), Duration::from_millis(100)),
            DiscoveryError::memory_limit_exceeded(200, 100),
            DiscoveryError::concurrency_error("test"),
            DiscoveryError::serialization_error("test"),
        ];
        
        for error in non_integrity_errors {
            assert!(!error.is_data_integrity_issue(), "Error should not be a data integrity issue: {:?}", error);
        }
    }
}