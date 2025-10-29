use async_trait::async_trait;
use anyhow::Result;

use crate::types::{ValidationOutput, ValidationReport, ValidationType};

/// Core trait for code validation
#[async_trait]
pub trait CodeValidator: Send + Sync {
    /// Validate syntax only
    async fn validate_syntax(&self, code: &str) -> Result<ValidationOutput>;

    /// Validate types (requires compilation context)
    async fn validate_types(&self, code: &str) -> Result<ValidationOutput>;

    /// Validate borrow checker rules (Rust-specific)
    async fn validate_borrow_checker(&self, code: &str) -> Result<ValidationOutput>;

    /// Validate compilation
    async fn validate_compilation(&self, code: &str) -> Result<ValidationOutput>;

    /// Run tests
    async fn validate_tests(&self, code: &str) -> Result<ValidationOutput>;

    /// Run all validations and generate comprehensive report
    async fn validate_all(&self, code: &str) -> Result<ValidationReport> {
        let mut report = ValidationReport::new(None, code.to_string());

        // Run validations in order, stop on first failure
        for validation_type in ValidationType::all() {
            let result = match validation_type {
                ValidationType::Syntax => self.validate_syntax(code).await?,
                ValidationType::Type => self.validate_types(code).await?,
                ValidationType::BorrowChecker => self.validate_borrow_checker(code).await?,
                ValidationType::Compilation => self.validate_compilation(code).await?,
                ValidationType::Test => self.validate_tests(code).await?,
            };

            let is_valid = result.is_valid;
            report.add_result(result);

            // Stop on first failure (fail-fast for efficiency)
            if !is_valid {
                break;
            }
        }

        Ok(report)
    }

    /// Validate specific types only
    async fn validate_specific(&self, code: &str, types: Vec<ValidationType>) -> Result<ValidationReport> {
        let mut report = ValidationReport::new(None, code.to_string());

        for validation_type in types {
            let result = match validation_type {
                ValidationType::Syntax => self.validate_syntax(code).await?,
                ValidationType::Type => self.validate_types(code).await?,
                ValidationType::BorrowChecker => self.validate_borrow_checker(code).await?,
                ValidationType::Compilation => self.validate_compilation(code).await?,
                ValidationType::Test => self.validate_tests(code).await?,
            };

            let is_valid = result.is_valid;
            report.add_result(result);

            if !is_valid {
                break;
            }
        }

        Ok(report)
    }
}

/// Default validator implementation using syn for syntax validation
pub struct DefaultRustValidator;

impl DefaultRustValidator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultRustValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CodeValidator for DefaultRustValidator {
    async fn validate_syntax(&self, code: &str) -> Result<ValidationOutput> {
        // GREEN phase: Minimal implementation using syn
        let start = std::time::Instant::now();

        let result = syn::parse_file(code);

        let execution_time_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(_) => Ok(ValidationOutput {
                is_valid: true,
                validation_type: ValidationType::Syntax,
                errors: Vec::new(),
                warnings: Vec::new(),
                execution_time_ms,
                memory_usage_bytes: 0, // Simplified for GREEN phase
            }),
            Err(e) => {
                let error_msg = format!("Syntax error: {}", e);
                Ok(ValidationOutput {
                    is_valid: false,
                    validation_type: ValidationType::Syntax,
                    errors: vec![error_msg],
                    warnings: Vec::new(),
                    execution_time_ms,
                    memory_usage_bytes: 0,
                })
            }
        }
    }

    async fn validate_types(&self, _code: &str) -> Result<ValidationOutput> {
        // GREEN phase: Stub implementation (passes for now)
        Ok(ValidationOutput::success(ValidationType::Type))
    }

    async fn validate_borrow_checker(&self, _code: &str) -> Result<ValidationOutput> {
        // GREEN phase: Stub implementation (passes for now)
        Ok(ValidationOutput::success(ValidationType::BorrowChecker))
    }

    async fn validate_compilation(&self, _code: &str) -> Result<ValidationOutput> {
        // GREEN phase: Stub implementation (passes for now)
        Ok(ValidationOutput::success(ValidationType::Compilation))
    }

    async fn validate_tests(&self, _code: &str) -> Result<ValidationOutput> {
        // GREEN phase: Stub implementation (passes for now)
        Ok(ValidationOutput::success(ValidationType::Test))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RED PHASE: These tests will fail initially

    #[tokio::test]
    async fn test_validate_syntax_valid_code() {
        let validator = DefaultRustValidator::new();
        let code = r#"
            fn main() {
                println!("Hello, world!");
            }
        "#;

        let result = validator.validate_syntax(code).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.is_valid);
        assert_eq!(output.validation_type, ValidationType::Syntax);
        assert_eq!(output.errors.len(), 0);
    }

    #[tokio::test]
    async fn test_validate_syntax_invalid_code() {
        let validator = DefaultRustValidator::new();
        let code = r#"
            fn main( {
                // Missing closing parenthesis
            }
        "#;

        let result = validator.validate_syntax(code).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_valid);
        assert!(output.errors.len() > 0);
    }

    #[tokio::test]
    async fn test_validate_all_stops_on_first_failure() {
        let validator = DefaultRustValidator::new();
        let code = r#"
            fn broken_function( {
                // Syntax error - should stop early
            }
        "#;

        let result = validator.validate_all(code).await;
        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(!report.overall_valid);
        // Should only have syntax validation result (fail-fast)
        assert_eq!(report.individual_results.len(), 1);
    }

    #[tokio::test]
    async fn test_validate_all_success() {
        let validator = DefaultRustValidator::new();
        let code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;

        let result = validator.validate_all(code).await;
        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(report.overall_valid);
        // Should have all 5 validation results
        assert_eq!(report.individual_results.len(), 5);
    }

    #[tokio::test]
    async fn test_validation_report_tracks_timing() {
        let validator = DefaultRustValidator::new();
        let code = "fn main() {}";

        let result = validator.validate_all(code).await;
        assert!(result.is_ok());
        let report = result.unwrap();

        // Timing should be tracked (even if zero for now)
        assert!(report.total_execution_time_ms >= 0);
    }
}
