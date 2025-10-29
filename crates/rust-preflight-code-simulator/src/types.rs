use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::errors::ValidationError;

/// Type of validation to perform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationType {
    /// Basic syntax validation using parser
    Syntax,
    /// Type checking and type inference validation
    Type,
    /// Borrow checker validation (Rust-specific)
    BorrowChecker,
    /// Full compilation validation
    Compilation,
    /// Test suite execution
    Test,
}

impl ValidationType {
    /// Get all validation types in order
    pub fn all() -> Vec<Self> {
        vec![
            Self::Syntax,
            Self::Type,
            Self::BorrowChecker,
            Self::Compilation,
            Self::Test,
        ]
    }

    /// Get validation types up to and including this one
    pub fn up_to(self) -> Vec<Self> {
        match self {
            Self::Syntax => vec![Self::Syntax],
            Self::Type => vec![Self::Syntax, Self::Type],
            Self::BorrowChecker => vec![Self::Syntax, Self::Type, Self::BorrowChecker],
            Self::Compilation => vec![Self::Syntax, Self::Type, Self::BorrowChecker, Self::Compilation],
            Self::Test => Self::all(),
        }
    }
}

/// Result of a single validation check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationOutput {
    /// Whether the validation passed
    pub is_valid: bool,
    /// Type of validation performed
    pub validation_type: ValidationType,
    /// Errors encountered during validation
    pub errors: Vec<String>,
    /// Warnings (non-blocking issues)
    pub warnings: Vec<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
}

impl ValidationOutput {
    /// Create a successful validation output
    pub fn success(validation_type: ValidationType) -> Self {
        Self {
            is_valid: true,
            validation_type,
            errors: Vec::new(),
            warnings: Vec::new(),
            execution_time_ms: 0,
            memory_usage_bytes: 0,
        }
    }

    /// Create a failed validation output
    pub fn failure(validation_type: ValidationType, error: ValidationError) -> Self {
        Self {
            is_valid: false,
            validation_type,
            errors: vec![error.to_string()],
            warnings: Vec::new(),
            execution_time_ms: 0,
            memory_usage_bytes: 0,
        }
    }

    /// Add timing information
    pub fn with_timing(mut self, execution_time_ms: u64, memory_usage_bytes: usize) -> Self {
        self.execution_time_ms = execution_time_ms;
        self.memory_usage_bytes = memory_usage_bytes;
        self
    }
}

/// Complete validation report for a code snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Path to file being validated (if applicable)
    pub file_path: Option<PathBuf>,
    /// Code snippet that was validated
    pub code_snippet: String,
    /// Individual validation results
    pub individual_results: Vec<ValidationOutput>,
    /// Overall validation status (all checks must pass)
    pub overall_valid: bool,
    /// Total execution time across all validations
    pub total_execution_time_ms: u64,
    /// Total memory usage across all validations
    pub total_memory_usage_bytes: usize,
    /// When this report was generated
    pub generated_at: DateTime<Utc>,
}

impl ValidationReport {
    /// Create a new validation report
    pub fn new(file_path: Option<PathBuf>, code_snippet: String) -> Self {
        Self {
            file_path,
            code_snippet,
            individual_results: Vec::new(),
            overall_valid: true,
            total_execution_time_ms: 0,
            total_memory_usage_bytes: 0,
            generated_at: Utc::now(),
        }
    }

    /// Add a validation result to this report
    pub fn add_result(&mut self, result: ValidationOutput) {
        self.overall_valid = self.overall_valid && result.is_valid;
        self.total_execution_time_ms += result.execution_time_ms;
        self.total_memory_usage_bytes += result.memory_usage_bytes;
        self.individual_results.push(result);
    }

    /// Get all errors from all validation results
    pub fn all_errors(&self) -> Vec<String> {
        self.individual_results
            .iter()
            .flat_map(|r| r.errors.iter().cloned())
            .collect()
    }

    /// Get all warnings from all validation results
    pub fn all_warnings(&self) -> Vec<String> {
        self.individual_results
            .iter()
            .flat_map(|r| r.warnings.iter().cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_type_all() {
        let types = ValidationType::all();
        assert_eq!(types.len(), 5);
        assert_eq!(types[0], ValidationType::Syntax);
        assert_eq!(types[4], ValidationType::Test);
    }

    #[test]
    fn test_validation_type_up_to() {
        let up_to_compilation = ValidationType::Compilation.up_to();
        assert_eq!(up_to_compilation.len(), 4);
        assert!(!up_to_compilation.contains(&ValidationType::Test));
    }

    #[test]
    fn test_validation_output_success() {
        let output = ValidationOutput::success(ValidationType::Syntax);
        assert!(output.is_valid);
        assert_eq!(output.errors.len(), 0);
    }

    #[test]
    fn test_validation_report_overall_status() {
        let mut report = ValidationReport::new(None, "fn main() {}".to_string());

        // Add successful result
        report.add_result(ValidationOutput::success(ValidationType::Syntax));
        assert!(report.overall_valid);

        // Add failed result
        let error = crate::errors::ValidationError::Parse("test error".to_string());
        report.add_result(ValidationOutput::failure(ValidationType::Compilation, error));
        assert!(!report.overall_valid);
    }

    #[test]
    fn test_validation_report_aggregates_errors() {
        let mut report = ValidationReport::new(None, "fn main() {}".to_string());

        let error1 = crate::errors::ValidationError::Parse("error 1".to_string());
        let error2 = crate::errors::ValidationError::Parse("error 2".to_string());

        report.add_result(ValidationOutput::failure(ValidationType::Syntax, error1));
        report.add_result(ValidationOutput::failure(ValidationType::Compilation, error2));

        let all_errors = report.all_errors();
        assert_eq!(all_errors.len(), 2);
    }
}
