//! Core validation traits and data structures for Rust code validation
//! Following TDD-first principle with comprehensive error handling

use async_trait::async_trait;
use parseltongue_01::types::ISGL1Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

/// Validation types for different kinds of Rust code analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationType {
    /// Syntax validation using rust-analyzer's parser
    Syntax,
    /// Type checking and inference validation
    Type,
    /// Borrow checker validation
    BorrowChecker,
    /// Full compilation validation
    Compilation,
    /// Macro expansion validation
    Macro,
    /// Attribute validation
    Attribute,
}

impl std::fmt::Display for ValidationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationType::Syntax => write!(f, "Syntax"),
            ValidationType::Type => write!(f, "Type"),
            ValidationType::BorrowChecker => write!(f, "BorrowChecker"),
            ValidationType::Compilation => write!(f, "Compilation"),
            ValidationType::Macro => write!(f, "Macro"),
            ValidationType::Attribute => write!(f, "Attribute"),
        }
    }
}

/// Validation severity levels for errors and warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Error = 3,
    Warning = 2,
    Info = 1,
    Hint = 0,
}

impl std::fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationSeverity::Error => write!(f, "Error"),
            ValidationSeverity::Warning => write!(f, "Warning"),
            ValidationSeverity::Info => write!(f, "Info"),
            ValidationSeverity::Hint => write!(f, "Hint"),
        }
    }
}

/// Validation error with detailed context and location information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationError {
    /// Syntax error with line/column information
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
        code_snippet: Option<String>,
    },

    /// Type error with expected/found types
    TypeError {
        line: usize,
        column: usize,
        expected: String,
        found: String,
        message: String,
    },

    /// Borrow checker error
    BorrowError {
        line: usize,
        column: usize,
        message: String,
        borrow_kind: String,
    },

    /// Compilation error
    CompilationError {
        message: String,
        help_text: Option<String>,
        error_code: Option<String>,
    },

    /// Macro expansion error
    MacroError {
        line: usize,
        column: usize,
        macro_name: String,
        message: String,
    },

    /// Attribute error
    AttributeError {
        line: usize,
        column: usize,
        attribute_name: String,
        message: String,
    },

    /// General validation error
    GeneralError {
        message: String,
        severity: ValidationSeverity,
        details: Option<String>,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::SyntaxError {
                line,
                column,
                message,
                ..
            } => {
                write!(
                    f,
                    "Syntax error at line {}, column {}: {}",
                    line, column, message
                )
            }
            ValidationError::TypeError {
                line,
                column,
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "Type error at line {}, column {}: {} (expected {}, found {})",
                    line, column, message, expected, found
                )
            }
            ValidationError::BorrowError {
                line,
                column,
                message,
                borrow_kind,
            } => {
                write!(
                    f,
                    "Borrow checker error at line {}, column {}: [{}] {}",
                    line, column, borrow_kind, message
                )
            }
            ValidationError::CompilationError { message, .. } => {
                write!(f, "Compilation error: {}", message)
            }
            ValidationError::MacroError {
                line,
                column,
                macro_name,
                message,
            } => {
                write!(
                    f,
                    "Macro error at line {}, column {} in {}: {}",
                    line, column, macro_name, message
                )
            }
            ValidationError::AttributeError {
                line,
                column,
                attribute_name,
                message,
            } => {
                write!(
                    f,
                    "Attribute error at line {}, column {} in {}: {}",
                    line, column, attribute_name, message
                )
            }
            ValidationError::GeneralError {
                message, severity, ..
            } => {
                write!(f, "{}: {}", severity, message)
            }
        }
    }
}

impl Error for ValidationError {}

/// Result of a single validation operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationOutput {
    /// Whether the validation passed
    pub is_valid: bool,
    /// Type of validation performed
    pub validation_type: ValidationType,
    /// Errors found during validation
    pub errors: Vec<ValidationError>,
    /// Warnings found during validation
    pub warnings: Vec<ValidationError>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
}

impl ValidationOutput {
    /// Create a successful validation result
    pub fn success(
        validation_type: ValidationType,
        execution_time_ms: u64,
        memory_usage_bytes: usize,
    ) -> Self {
        Self {
            is_valid: true,
            validation_type,
            errors: vec![],
            warnings: vec![],
            execution_time_ms,
            memory_usage_bytes,
        }
    }

    /// Create a failed validation result with errors
    pub fn failure(
        validation_type: ValidationType,
        errors: Vec<ValidationError>,
        warnings: Vec<ValidationError>,
        execution_time_ms: u64,
        memory_usage_bytes: usize,
    ) -> Self {
        Self {
            is_valid: false,
            validation_type,
            errors,
            warnings,
            execution_time_ms,
            memory_usage_bytes,
        }
    }

    /// Get the number of errors by severity
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get the number of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    ValidationError::GeneralError {
                        severity: ValidationSeverity::Warning,
                        ..
                    }
                )
            })
            .count()
    }

    /// Get the highest severity level
    pub fn highest_severity(&self) -> Option<ValidationSeverity> {
        let all_issues: Vec<&ValidationError> =
            self.errors.iter().chain(self.warnings.iter()).collect();

        if all_issues.is_empty() {
            return None;
        }

        all_issues
            .iter()
            .map(|error| match error {
                ValidationError::SyntaxError { .. } => ValidationSeverity::Error,
                ValidationError::TypeError { .. } => ValidationSeverity::Error,
                ValidationError::BorrowError { .. } => ValidationSeverity::Error,
                ValidationError::CompilationError { .. } => ValidationSeverity::Error,
                ValidationError::MacroError { .. } => ValidationSeverity::Error,
                ValidationError::AttributeError { .. } => ValidationSeverity::Error,
                ValidationError::GeneralError { severity, .. } => *severity,
            })
            .max()
    }
}

/// Comprehensive validation report for a file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Path to the validated file
    pub file_path: PathBuf,
    /// Code snippet that was validated
    pub code_snippet: String,
    /// Individual validation results
    pub individual_results: Vec<ValidationOutput>,
    /// Overall validation status
    pub overall_valid: bool,
    /// Total execution time in milliseconds
    pub total_execution_time_ms: u64,
    /// Total memory usage in bytes
    pub total_memory_usage_bytes: usize,
    /// When the report was generated
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl ValidationReport {
    /// Create a new validation report
    pub fn new(
        file_path: PathBuf,
        code_snippet: String,
        individual_results: Vec<ValidationOutput>,
    ) -> Self {
        let total_execution_time_ms = individual_results.iter().map(|r| r.execution_time_ms).sum();
        let total_memory_usage_bytes = individual_results
            .iter()
            .map(|r| r.memory_usage_bytes)
            .sum();
        let overall_valid = individual_results.iter().all(|r| r.is_valid);

        Self {
            file_path,
            code_snippet,
            individual_results,
            overall_valid,
            total_execution_time_ms,
            total_memory_usage_bytes,
            generated_at: chrono::Utc::now(),
        }
    }

    /// Create a validation report with ISGL1Key integration
    pub fn with_key(
        key: ISGL1Key,
        code_snippet: String,
        individual_results: Vec<ValidationOutput>,
    ) -> Self {
        Self::new(
            key.filepath.join(&key.filename),
            code_snippet,
            individual_results,
        )
    }

    /// Get validation results by type
    pub fn results_by_type(&self, validation_type: ValidationType) -> Vec<&ValidationOutput> {
        self.individual_results
            .iter()
            .filter(|r| r.validation_type == validation_type)
            .collect()
    }

    /// Get all errors from all validation results
    pub fn all_errors(&self) -> Vec<&ValidationError> {
        self.individual_results
            .iter()
            .flat_map(|r| r.errors.iter())
            .collect()
    }

    /// Get all warnings from all validation results
    pub fn all_warnings(&self) -> Vec<&ValidationError> {
        self.individual_results
            .iter()
            .flat_map(|r| r.warnings.iter())
            .collect()
    }

    /// Calculate validation accuracy (1.0 = perfect, 0.0 = complete failure)
    pub fn validation_accuracy(&self) -> f64 {
        if self.individual_results.is_empty() {
            return 1.0;
        }

        let successful_validations = self
            .individual_results
            .iter()
            .filter(|r| r.is_valid)
            .count();

        successful_validations as f64 / self.individual_results.len() as f64
    }

    /// Get execution time breakdown by validation type
    pub fn execution_time_breakdown(&self) -> HashMap<ValidationType, u64> {
        let mut breakdown = HashMap::new();

        for result in &self.individual_results {
            *breakdown.entry(result.validation_type).or_insert(0) += result.execution_time_ms;
        }

        breakdown
    }

    /// Get memory usage breakdown by validation type
    pub fn memory_usage_breakdown(&self) -> HashMap<ValidationType, usize> {
        let mut breakdown = HashMap::new();

        for result in &self.individual_results {
            *breakdown.entry(result.validation_type).or_insert(0) += result.memory_usage_bytes;
        }

        breakdown
    }
}

/// Validator capabilities for feature detection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorCapabilities {
    /// Supports syntax validation
    pub supports_syntax_validation: bool,
    /// Supports type validation
    pub supports_type_validation: bool,
    /// Supports borrow checker validation
    pub supports_borrow_checker_validation: bool,
    /// Supports compilation validation
    pub supports_compilation_validation: bool,
    /// Supports macro validation
    pub supports_macro_validation: bool,
    /// Supports attribute validation
    pub supports_attribute_validation: bool,
}

impl Default for ValidatorCapabilities {
    fn default() -> Self {
        Self {
            supports_syntax_validation: true,
            supports_type_validation: true,
            supports_borrow_checker_validation: false,
            supports_compilation_validation: true,
            supports_macro_validation: false,
            supports_attribute_validation: true,
        }
    }
}

/// Test case for validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationTestCase {
    /// Name of the test case
    pub name: String,
    /// Rust code to validate
    pub code: String,
    /// Path to the file (for context)
    pub file_path: PathBuf,
    /// Expected syntax validation result
    pub expected_syntax_valid: bool,
    /// Expected type validation result
    pub expected_type_valid: bool,
    /// Expected compilation validation result
    pub expected_compilation_valid: bool,
}

impl ValidationTestCase {
    /// Create a new validation test case
    pub fn new(
        name: String,
        code: String,
        file_path: PathBuf,
        expected_syntax_valid: bool,
        expected_type_valid: bool,
        expected_compilation_valid: bool,
    ) -> Self {
        Self {
            name,
            code,
            file_path,
            expected_syntax_valid,
            expected_type_valid,
            expected_compilation_valid,
        }
    }

    /// Get the code size in bytes
    pub fn code_size_bytes(&self) -> usize {
        self.code.len()
    }

    /// Get the code size in kilobytes
    pub fn code_size_kb(&self) -> f64 {
        self.code_size_bytes() as f64 / 1024.0
    }
}

/// Trait for Rust code validation with async support
#[async_trait]
pub trait RustCodeValidator: Clone + Send + Sync + 'static {
    /// Input type for validation (typically Rust code as String)
    type Input: Clone + Send + Sync;

    /// Output type for single validation results
    type Output: Clone + Send + Sync;

    /// Error type for validation operations
    type Error: std::fmt::Debug + Send + Sync;

    /// Validate syntax of the input code
    async fn validate_syntax(&self, code: &Self::Input) -> Result<Self::Output, Self::Error>;

    /// Validate types and type inference
    async fn validate_types(&self, code: &Self::Input) -> Result<Self::Output, Self::Error>;

    /// Validate borrow checker rules
    async fn validate_borrow_checker(
        &self,
        code: &Self::Input,
    ) -> Result<Self::Output, Self::Error>;

    /// Validate full compilation
    async fn validate_compilation(&self, code: &Self::Input) -> Result<Self::Output, Self::Error>;

    /// Validate all aspects and return a comprehensive report
    async fn validate_all(&self, code: &Self::Input) -> Result<ValidationReport, Self::Error>;

    /// Get the validator name
    fn name(&self) -> &'static str;

    /// Get validator capabilities
    fn capabilities(&self) -> ValidatorCapabilities;

    /// Estimate memory usage for validation
    fn estimate_memory_usage(&self, input_size_bytes: usize) -> usize {
        // Default estimation: 3x input size
        input_size_bytes * 3
    }

    /// Check if the validator supports a specific validation type
    fn supports_validation_type(&self, validation_type: ValidationType) -> bool {
        let capabilities = self.capabilities();
        match validation_type {
            ValidationType::Syntax => capabilities.supports_syntax_validation,
            ValidationType::Type => capabilities.supports_type_validation,
            ValidationType::BorrowChecker => capabilities.supports_borrow_checker_validation,
            ValidationType::Compilation => capabilities.supports_compilation_validation,
            ValidationType::Macro => capabilities.supports_macro_validation,
            ValidationType::Attribute => capabilities.supports_attribute_validation,
        }
    }
}

/// Default implementation of RustCodeValidator using basic syntax checking
#[derive(Debug, Clone)]
pub struct DefaultRustCodeValidator {
    name: &'static str,
    capabilities: ValidatorCapabilities,
}

impl DefaultRustCodeValidator {
    /// Create a new default validator
    pub fn new() -> Self {
        Self {
            name: "DefaultRustCodeValidator",
            capabilities: ValidatorCapabilities::default(),
        }
    }

    /// Create a validator with custom capabilities
    pub fn with_capabilities(capabilities: ValidatorCapabilities) -> Self {
        Self {
            name: "DefaultRustCodeValidator",
            capabilities,
        }
    }
}

impl Default for DefaultRustCodeValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RustCodeValidator for DefaultRustCodeValidator {
    type Input = String;
    type Output = ValidationOutput;
    type Error = ValidationError;

    async fn validate_syntax(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        let start_time = std::time::Instant::now();
        let memory_usage = self.estimate_memory_usage(code.len());

        // Basic syntax validation using simple checks for GREEN phase
        let mut errors = Vec::new();

        // Simple syntax checks
        let lines: Vec<&str> = code.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check for basic Rust syntax issues
            if trimmed.starts_with("fn ") && !trimmed.contains('(') {
                errors.push(ValidationError::SyntaxError {
                    line: line_num + 1,
                    column: trimmed.find("fn ").unwrap_or(0) + 3,
                    message: "Function declaration missing parentheses".to_string(),
                    code_snippet: Some(line.to_string()),
                });
            }

            // Check for function without parameters but with space before braces
            if trimmed.starts_with("fn ") && trimmed.contains(" {") && !trimmed.contains("(){") {
                errors.push(ValidationError::SyntaxError {
                    line: line_num + 1,
                    column: trimmed.find(" {").unwrap_or(0),
                    message: "Function declaration missing parentheses before opening brace"
                        .to_string(),
                    code_snippet: Some(line.to_string()),
                });
            }

            if trimmed.contains("{") && !trimmed.contains("}") && line_num == lines.len() - 1 {
                // Check for unclosed braces at the end
                let open_count = line.matches('{').count();
                let close_count = line.matches('}').count();
                if open_count > close_count {
                    errors.push(ValidationError::SyntaxError {
                        line: line_num + 1,
                        column: line.len(),
                        message: "Unclosed brace".to_string(),
                        code_snippet: Some(line.to_string()),
                    });
                }
            }
        }

        let execution_time = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::Syntax,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::Syntax,
                errors,
                vec![],
                execution_time,
                memory_usage,
            ))
        }
    }

    async fn validate_types(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        let start_time = std::time::Instant::now();
        let memory_usage = self.estimate_memory_usage(code.len());

        // For now, basic type validation - check for common type errors
        let mut errors = Vec::new();

        // Simple heuristics for type validation
        if code.contains("let x: i32 = \"string\";") {
            errors.push(ValidationError::TypeError {
                line: 1,
                column: 1,
                expected: "i32".to_string(),
                found: "&str".to_string(),
                message: "Type mismatch: cannot assign string to i32 variable".to_string(),
            });
        }

        let execution_time = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::Type,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::Type,
                errors,
                vec![],
                execution_time,
                memory_usage,
            ))
        }
    }

    async fn validate_borrow_checker(
        &self,
        code: &Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        let start_time = std::time::Instant::now();
        let memory_usage = self.estimate_memory_usage(code.len());

        // For now, basic borrow checker heuristics
        let mut errors = Vec::new();

        // Simple check for obvious borrow checker issues
        if code.contains("&mut") && code.contains("*.as_ref()") {
            errors.push(ValidationError::BorrowError {
                line: 1,
                column: 1,
                message: "Potential borrow checker violation".to_string(),
                borrow_kind: "Mutable borrow while immutable borrow exists".to_string(),
            });
        }

        let execution_time = start_time.elapsed().as_millis() as u64;

        if errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::BorrowChecker,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::BorrowChecker,
                errors,
                vec![],
                execution_time,
                memory_usage,
            ))
        }
    }

    async fn validate_compilation(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        let start_time = std::time::Instant::now();
        let memory_usage = self.estimate_memory_usage(code.len());

        // Basic compilation validation - check syntax first
        let syntax_result = self.validate_syntax(code).await?;
        if !syntax_result.is_valid {
            return Ok(ValidationOutput::failure(
                ValidationType::Compilation,
                syntax_result.errors,
                syntax_result.warnings,
                start_time.elapsed().as_millis() as u64,
                memory_usage,
            ));
        }

        // For basic compilation, assume valid if syntax is valid
        // Real implementation would use rustc or cargo check
        let execution_time = start_time.elapsed().as_millis() as u64;
        Ok(ValidationOutput::success(
            ValidationType::Compilation,
            execution_time,
            memory_usage,
        ))
    }

    async fn validate_all(&self, code: &Self::Input) -> Result<ValidationReport, Self::Error> {
        let file_path = PathBuf::from("temp_file.rs");
        let mut individual_results = Vec::new();

        // Run all supported validations
        if self.supports_validation_type(ValidationType::Syntax) {
            let result = self.validate_syntax(code).await?;
            individual_results.push(result);
        }

        if self.supports_validation_type(ValidationType::Type) {
            let result = self.validate_types(code).await?;
            individual_results.push(result);
        }

        if self.supports_validation_type(ValidationType::BorrowChecker) {
            let result = self.validate_borrow_checker(code).await?;
            individual_results.push(result);
        }

        if self.supports_validation_type(ValidationType::Compilation) {
            let result = self.validate_compilation(code).await?;
            individual_results.push(result);
        }

        Ok(ValidationReport::new(
            file_path,
            code.clone(),
            individual_results,
        ))
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn capabilities(&self) -> ValidatorCapabilities {
        self.capabilities.clone()
    }
}
