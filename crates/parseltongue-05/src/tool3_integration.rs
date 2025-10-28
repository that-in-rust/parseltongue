//! Integration with Tool 3 validation results

use crate::error::FileWriterResult;
use crate::file_writer::FileWriteInput;
use crate::report::FileOperation;
use crate::safety::WriteSafetyLevel;
use parseltongue_04::ValidationReport;
use serde::{Deserialize, Serialize};

/// Result of validation integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub passed: bool,
    /// Overall validation score (0.0 - 1.0)
    pub score: f32,
    /// Validation messages and warnings
    pub messages: Vec<String>,
    /// Recommended safety level based on validation
    pub recommended_safety_level: WriteSafetyLevel,
    /// Specific validation results
    pub validation_details: ValidationDetails,
    /// Whether the file should be written based on validation
    pub should_write: bool,
}

/// Detailed validation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetails {
    /// Syntax validation result
    pub syntax_valid: bool,
    /// Type validation result
    pub types_valid: bool,
    /// Borrow checker validation result
    pub borrow_checker_valid: bool,
    /// Compilation validation result
    pub compilation_valid: bool,
    /// Performance regression check
    pub performance_regression: bool,
    /// Number of validation errors
    pub error_count: usize,
    /// Number of validation warnings
    pub warning_count: usize,
}

/// Trait for converting Tool 3 validation results to file writer validation
pub trait ValidationToFileWriterConverter {
    /// Convert Tool 3 ValidationReport to ValidationResult
    fn convert_validation_report(
        &self,
        report: &ValidationReport,
    ) -> FileWriterResult<ValidationResult>;

    /// Determine if file writing should proceed based on validation
    fn should_proceed_with_write(&self, validation: &ValidationResult) -> bool;

    /// Adjust safety level based on validation results
    fn adjust_safety_level(
        &self,
        base_level: WriteSafetyLevel,
        validation: &ValidationResult,
    ) -> WriteSafetyLevel;
}

/// Default converter implementation
pub struct DefaultValidationConverter;

impl DefaultValidationConverter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultValidationConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationToFileWriterConverter for DefaultValidationConverter {
    fn convert_validation_report(
        &self,
        report: &ValidationReport,
    ) -> FileWriterResult<ValidationResult> {
        // Calculate overall score based on validation results
        let score = self.calculate_validation_score(report);

        // Determine if validation passed
        let passed = report.overall_valid && score >= 0.7; // 70% threshold

        // Extract messages from validation report
        let messages = self.extract_validation_messages(report);

        // Determine recommended safety level
        let recommended_safety_level = self.determine_safety_level(report, score);

        // Extract validation details
        let validation_details = self.extract_validation_details(report);

        // Determine if writing should proceed
        let should_write = self.should_proceed_with_write_internal(report, score);

        Ok(ValidationResult {
            passed,
            score,
            messages,
            recommended_safety_level,
            validation_details,
            should_write,
        })
    }

    fn should_proceed_with_write(&self, validation: &ValidationResult) -> bool {
        validation.should_write
    }

    fn adjust_safety_level(
        &self,
        base_level: WriteSafetyLevel,
        validation: &ValidationResult,
    ) -> WriteSafetyLevel {
        // If validation failed, increase safety level
        if !validation.passed {
            return WriteSafetyLevel::Strict;
        }

        // If validation score is low, increase safety level
        if validation.score < 0.8 {
            match base_level {
                WriteSafetyLevel::None => WriteSafetyLevel::Basic,
                WriteSafetyLevel::Basic => WriteSafetyLevel::Standard,
                WriteSafetyLevel::Standard => WriteSafetyLevel::Strict,
                WriteSafetyLevel::Strict => WriteSafetyLevel::Strict,
            }
        } else {
            base_level
        }
    }
}

impl DefaultValidationConverter {
    /// Calculate overall validation score from report
    fn calculate_validation_score(&self, report: &ValidationReport) -> f32 {
        // Simple scoring algorithm - can be made more sophisticated
        let mut score: f32 = 1.0;

        // Deduct points for validation failures
        if !report.overall_valid {
            score -= 0.5;
        }

        // Consider validation output results
        for output in &report.individual_results {
            if !output.is_valid {
                score -= 0.1;
            }
        }

        // For now, ignore performance regressions as they're not in the current ValidationReport structure
        // In GREEN phase, we'll add proper performance regression checking

        score.max(0.0).min(1.0)
    }

    /// Extract validation messages from report
    fn extract_validation_messages(&self, report: &ValidationReport) -> Vec<String> {
        let mut messages = Vec::new();

        // Add success/failure message
        if report.overall_valid {
            messages.push("Validation passed successfully".to_string());
        } else {
            messages.push("Validation failed - review issues before proceeding".to_string());
        }

        // Add validation output messages
        for output in &report.individual_results {
            if !output.is_valid {
                messages.push(format!(
                    "{} validation failed: {:?}",
                    output.validation_type, output.errors
                ));
            }
        }

        // Add performance messages (placeholder for GREEN phase)
        if report.total_execution_time_ms > 5000 {
            messages.push("Validation took longer than expected".to_string());
        }

        messages
    }

    /// Determine recommended safety level based on validation
    fn determine_safety_level(&self, report: &ValidationReport, score: f32) -> WriteSafetyLevel {
        if !report.overall_valid {
            WriteSafetyLevel::Strict
        } else if score < 0.8 {
            WriteSafetyLevel::Standard
        } else if score < 0.9 {
            WriteSafetyLevel::Basic
        } else {
            WriteSafetyLevel::None // High confidence, lower safety needed
        }
    }

    /// Extract detailed validation information
    fn extract_validation_details(&self, report: &ValidationReport) -> ValidationDetails {
        let mut syntax_valid = true;
        let mut types_valid = true;
        let mut borrow_checker_valid = true;
        let mut compilation_valid = true;
        let mut error_count = 0;
        let mut warning_count = 0;

        for output in &report.individual_results {
            match output.validation_type {
                parseltongue_04::ValidationType::Syntax => {
                    syntax_valid = output.is_valid;
                    if !output.is_valid {
                        error_count += output.errors.len();
                    }
                }
                parseltongue_04::ValidationType::Type => {
                    types_valid = output.is_valid;
                    if !output.is_valid {
                        error_count += output.errors.len();
                    }
                }
                parseltongue_04::ValidationType::BorrowChecker => {
                    borrow_checker_valid = output.is_valid;
                    if !output.is_valid {
                        error_count += output.errors.len();
                    }
                }
                parseltongue_04::ValidationType::Compilation => {
                    compilation_valid = output.is_valid;
                    if !output.is_valid {
                        error_count += output.errors.len();
                    }
                }
                // Skip other validation types for now
                _ => {}
            }

            warning_count += output.warnings.len();
        }

        // For now, assume no performance regression (GREEN phase will add proper checking)
        let performance_regression = false;

        ValidationDetails {
            syntax_valid,
            types_valid,
            borrow_checker_valid,
            compilation_valid,
            performance_regression,
            error_count,
            warning_count,
        }
    }

    /// Internal logic for determining if writing should proceed
    fn should_proceed_with_write_internal(&self, report: &ValidationReport, score: f32) -> bool {
        // Don't proceed if validation failed
        if !report.overall_valid {
            return false;
        }

        // Don't proceed if score is too low
        if score < 0.6 {
            return false;
        }

        // Don't proceed if there are compilation errors
        for output in &report.individual_results {
            if output.validation_type == parseltongue_04::ValidationType::Compilation
                && !output.is_valid
            {
                return false;
            }
        }

        true
    }
}

/// Pipeline for integrating validation with file writing
pub struct ValidationWritePipeline {
    converter: Box<dyn ValidationToFileWriterConverter + Send + Sync>,
}

impl ValidationWritePipeline {
    /// Create a new validation write pipeline
    pub fn new() -> Self {
        Self {
            converter: Box::new(DefaultValidationConverter::new()),
        }
    }

    /// Create a pipeline with a custom converter
    pub fn with_converter(
        converter: Box<dyn ValidationToFileWriterConverter + Send + Sync>,
    ) -> Self {
        Self { converter }
    }

    /// Process validation results and create file write input
    pub async fn process_validation_for_write(
        &self,
        validation_report: &ValidationReport,
        file_path: String,
        content: Vec<u8>,
        operation_type: FileOperation,
    ) -> FileWriterResult<FileWriteInput> {
        let validation_result = self
            .converter
            .convert_validation_report(validation_report)?;

        Ok(FileWriteInput {
            path: file_path,
            content,
            operation_type,
            validation_results: Some(validation_result),
        })
    }

    /// Check if writing should proceed based on validation
    pub async fn should_proceed_with_write(
        &self,
        validation_report: &ValidationReport,
    ) -> FileWriterResult<bool> {
        let validation_result = self
            .converter
            .convert_validation_report(validation_report)?;
        Ok(self.converter.should_proceed_with_write(&validation_result))
    }

    /// Get recommended safety level based on validation
    pub async fn get_recommended_safety_level(
        &self,
        validation_report: &ValidationReport,
        base_level: WriteSafetyLevel,
    ) -> FileWriterResult<WriteSafetyLevel> {
        let validation_result = self
            .converter
            .convert_validation_report(validation_report)?;
        Ok(self
            .converter
            .adjust_safety_level(base_level, &validation_result))
    }
}

impl Default for ValidationWritePipeline {
    fn default() -> Self {
        Self::new()
    }
}
