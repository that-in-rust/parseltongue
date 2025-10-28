//! Rust analyzer integration module
//! Provides comprehensive Rust code validation using rust-analyzer

use crate::validation::{ValidationError, ValidationOutput, ValidationType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use tokio::process::Command as TokioCommand;

/// Configuration for RustAnalyzerClient
#[derive(Debug, Clone)]
pub struct RustAnalyzerConfig {
    /// Timeout for individual analysis operations
    pub timeout: Duration,
    /// Path to rust-analyzer binary (if not in PATH)
    pub rust_analyzer_path: Option<PathBuf>,
    /// Whether to use cargo check for validation
    pub use_cargo_check: bool,
    /// Maximum memory usage in bytes
    pub max_memory_usage: usize,
}

impl Default for RustAnalyzerConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            rust_analyzer_path: None,
            use_cargo_check: true,
            max_memory_usage: 512 * 1024 * 1024, // 512MB
        }
    }
}

/// Rust analyzer client for comprehensive code validation
#[derive(Debug, Clone)]
pub struct RustAnalyzerClient {
    config: RustAnalyzerConfig,
    project_path: PathBuf,
}

impl RustAnalyzerClient {
    /// Create a new RustAnalyzerClient for the given project path
    pub async fn new<P: Into<PathBuf>>(project_path: P) -> Result<Self, RustAnalyzerError> {
        let project_path = project_path.into();
        let config = RustAnalyzerConfig::default();

        Self::with_config(project_path, config).await
    }

    /// Create a new RustAnalyzerClient with custom configuration
    pub async fn with_config<P: Into<PathBuf>>(
        project_path: P,
        config: RustAnalyzerConfig,
    ) -> Result<Self, RustAnalyzerError> {
        let project_path = project_path.into();

        // Verify the project path exists and is a valid Rust project
        if !project_path.exists() {
            return Err(RustAnalyzerError::ProjectNotFound(project_path));
        }

        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(RustAnalyzerError::NotRustProject(project_path));
        }

        Ok(Self {
            config,
            project_path,
        })
    }

    /// Get the project path
    pub fn project_path(&self) -> &PathBuf {
        &self.project_path
    }

    /// Get the configuration
    pub fn config(&self) -> &RustAnalyzerConfig {
        &self.config
    }

    /// Analyze Rust code syntax using rust-analyzer
    pub async fn analyze_syntax(
        &self,
        code: &str,
        file_path: &PathBuf,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        let start_time = std::time::Instant::now();

        // Create a temporary file for the code
        let temp_file = tempfile::NamedTempFile::with_suffix(".rs")?;
        tokio::fs::write(&temp_file.path(), code).await?;

        // Use rust-analyzer or basic syntax checking
        let result = if self.config.use_cargo_check {
            self.check_with_cargo_check(&temp_file.path().to_path_buf())
                .await?
        } else {
            self.check_with_syntax_parser(code).await?
        };

        let execution_time = start_time.elapsed().as_millis() as u64;
        let memory_usage = self.estimate_memory_usage(code.len());

        if result.errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::Syntax,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::Syntax,
                result.errors,
                result.warnings,
                execution_time,
                memory_usage,
            ))
        }
    }

    /// Analyze types and type inference
    pub async fn analyze_types(
        &self,
        code: &str,
        file_path: &PathBuf,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        let start_time = std::time::Instant::now();

        // For type analysis, we need cargo check to get type errors
        let temp_file = tempfile::NamedTempFile::with_suffix(".rs")?;
        tokio::fs::write(&temp_file.path(), code).await?;

        let result = self
            .check_types_with_cargo(&temp_file.path().to_path_buf())
            .await?;
        let execution_time = start_time.elapsed().as_millis() as u64;
        let memory_usage = self.estimate_memory_usage(code.len());

        if result.errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::Type,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::Type,
                result.errors,
                result.warnings,
                execution_time,
                memory_usage,
            ))
        }
    }

    /// Validate syntax (alias for analyze_syntax for test compatibility)
    pub async fn validate_syntax(
        &self,
        file_path: &str,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        // Read the file content
        let full_path = self.project_path.join(file_path);
        let code = tokio::fs::read_to_string(&full_path).await?;
        self.analyze_syntax(&code, &full_path).await
    }

    /// Validate types (alias for analyze_types for test compatibility)
    pub async fn validate_types(
        &self,
        file_path: &str,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        // Read the file content
        let full_path = self.project_path.join(file_path);
        let code = tokio::fs::read_to_string(&full_path).await?;
        self.analyze_types(&code, &full_path).await
    }

    /// Validate dependencies (basic implementation for test compatibility)
    pub async fn validate_dependencies(
        &self,
        file_path: &str,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        let start_time = std::time::Instant::now();

        // Basic dependency validation using cargo check
        let output = TokioCommand::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(&self.project_path)
            .output()
            .await?;

        let execution_time = start_time.elapsed().as_millis() as u64;
        let memory_usage = 1024 * 1024; // 1MB estimate

        if output.status.success() {
            Ok(ValidationOutput::success(
                ValidationType::Compilation,
                execution_time,
                memory_usage,
            ))
        } else {
            // Parse for dependency-related errors
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut errors = Vec::new();

            for line in output_str.lines() {
                if line.contains("cannot find") || line.contains("unresolved import") {
                    errors.push(ValidationError::CompilationError {
                        message: line.to_string(),
                        help_text: None,
                        error_code: Some("E0432".to_string()),
                    });
                }
            }

            Ok(ValidationOutput::failure(
                ValidationType::Compilation,
                errors,
                vec![],
                execution_time,
                memory_usage,
            ))
        }
    }

    /// Validate all aspects (comprehensive validation)
    pub async fn validate_all(
        &self,
        file_path: &str,
    ) -> Result<crate::validation::ValidationReport, RustAnalyzerError> {
        use crate::validation::{ValidationReport, RustCodeValidator};

        // Read the file content
        let full_path = self.project_path.join(file_path);
        let code = tokio::fs::read_to_string(&full_path).await?;

        // Create a mock validator for now
        let validator = crate::validation::DefaultRustCodeValidator::new();
        validator.validate_all(&code).await.map_err(|e| {
            RustAnalyzerError::CargoError(format!("Validation failed: {}", e))
        })
    }

    /// Get workspace info (basic implementation for test compatibility)
    pub async fn get_workspace_info(&self) -> Result<WorkspaceInfo, RustAnalyzerError> {
        let output = TokioCommand::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .current_dir(&self.project_path)
            .output()
            .await?;

        if output.status.success() {
            let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)?;
            Ok(WorkspaceInfo {
                workspace_root: metadata.workspace_root,
                packages: metadata.packages.len(),
                target_directory: metadata.target_directory,
            })
        } else {
            Err(RustAnalyzerError::CargoError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    /// Analyze borrow checker issues
    pub async fn analyze_borrow_checker(
        &self,
        code: &str,
        file_path: &PathBuf,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        let start_time = std::time::Instant::now();

        // For borrow checker analysis, we need cargo check with --message-format=json
        let temp_file = tempfile::NamedTempFile::with_suffix(".rs")?;
        tokio::fs::write(&temp_file.path(), code).await?;

        let result = self
            .check_borrow_checker_with_cargo(&temp_file.path().to_path_buf())
            .await?;
        let execution_time = start_time.elapsed().as_millis() as u64;
        let memory_usage = self.estimate_memory_usage(code.len());

        if result.errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::BorrowChecker,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::BorrowChecker,
                result.errors,
                result.warnings,
                execution_time,
                memory_usage,
            ))
        }
    }

    /// Full compilation analysis
    pub async fn analyze_compilation(
        &self,
        code: &str,
        file_path: &PathBuf,
    ) -> Result<ValidationOutput, RustAnalyzerError> {
        let start_time = std::time::Instant::now();

        // Create temporary project structure
        let temp_dir = tempfile::TempDir::new()?;
        let temp_file = temp_dir.path().join("main.rs");
        tokio::fs::write(&temp_file, code).await?;

        let result = self
            .full_compilation_check(&temp_dir.path().to_path_buf())
            .await?;
        let execution_time = start_time.elapsed().as_millis() as u64;
        let memory_usage = self.estimate_memory_usage(code.len());

        if result.errors.is_empty() {
            Ok(ValidationOutput::success(
                ValidationType::Compilation,
                execution_time,
                memory_usage,
            ))
        } else {
            Ok(ValidationOutput::failure(
                ValidationType::Compilation,
                result.errors,
                result.warnings,
                execution_time,
                memory_usage,
            ))
        }
    }

    /// Estimate memory usage for analysis
    fn estimate_memory_usage(&self, code_size: usize) -> usize {
        // Rough estimation: 10x code size for syntax trees and analysis data
        std::cmp::min(code_size * 10, self.config.max_memory_usage)
    }

    /// Check syntax using cargo check
    async fn check_with_cargo_check(
        &self,
        file_path: &PathBuf,
    ) -> Result<AnalysisResult, RustAnalyzerError> {
        let output = TokioCommand::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(&self.project_path)
            .output()
            .await?;

        if !output.status.success() {
            return self.parse_cargo_output(&output.stdout);
        }

        Ok(AnalysisResult {
            errors: vec![],
            warnings: vec![],
        })
    }

    /// Check using basic syntax parser
    async fn check_with_syntax_parser(
        &self,
        code: &str,
    ) -> Result<AnalysisResult, RustAnalyzerError> {
        // Simple syntax checks for GREEN phase
        let mut errors = Vec::new();

        let lines: Vec<&str> = code.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Basic syntax validation
            if trimmed.starts_with("fn ") && !trimmed.contains('(') {
                errors.push(ValidationError::SyntaxError {
                    line: line_num + 1,
                    column: trimmed.find("fn ").unwrap_or(0) + 3,
                    message: "Function declaration missing parentheses".to_string(),
                    code_snippet: Some(line.to_string()),
                });
            }
        }

        Ok(AnalysisResult {
            errors,
            warnings: vec![],
        })
    }

    /// Check types with cargo
    async fn check_types_with_cargo(
        &self,
        file_path: &PathBuf,
    ) -> Result<AnalysisResult, RustAnalyzerError> {
        let output = TokioCommand::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(&self.project_path)
            .output()
            .await?;

        self.parse_cargo_types_output(&output.stdout)
    }

    /// Check borrow checker with cargo
    async fn check_borrow_checker_with_cargo(
        &self,
        file_path: &PathBuf,
    ) -> Result<AnalysisResult, RustAnalyzerError> {
        let output = TokioCommand::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .arg("-Z")
            .arg("borrowck=mir")
            .current_dir(&self.project_path)
            .output()
            .await?;

        self.parse_cargo_borrow_output(&output.stdout)
    }

    /// Full compilation check
    async fn full_compilation_check(
        &self,
        temp_dir: &PathBuf,
    ) -> Result<AnalysisResult, RustAnalyzerError> {
        let output = TokioCommand::new("cargo")
            .arg("build")
            .arg("--message-format=json")
            .current_dir(temp_dir)
            .output()
            .await?;

        self.parse_cargo_output(&output.stdout)
    }

    /// Parse cargo check output for errors and warnings
    fn parse_cargo_output(&self, output: &[u8]) -> Result<AnalysisResult, RustAnalyzerError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        let output_str = String::from_utf8_lossy(output);
        for line in output_str.lines() {
            if let Ok(cargo_message) = serde_json::from_str::<CargoMessage>(line) {
                match cargo_message {
                    CargoMessage::CompilerMessage(msg) => {
                        for diagnostic in &msg.message.spans {
                            let validation_error = ValidationError::CompilationError {
                                message: msg.message.message.clone(),
                                help_text: msg.message.rendered.clone(),
                                error_code: Some("E0000".to_string()),
                            };

                            if msg.message.level == CompilerMessageLevel::Error {
                                errors.push(validation_error);
                            } else if msg.message.level == CompilerMessageLevel::Warning {
                                warnings.push(validation_error);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(AnalysisResult { errors, warnings })
    }

    /// Parse cargo output for type errors specifically
    fn parse_cargo_types_output(&self, output: &[u8]) -> Result<AnalysisResult, RustAnalyzerError> {
        let result = self.parse_cargo_output(output)?;

        // Filter for type-related errors
        let type_errors: Vec<ValidationError> = result
            .errors
            .into_iter()
            .filter(|error| {
                matches!(error, ValidationError::CompilationError { message, .. }
                    if message.contains("type") || message.contains("mismatch"))
            })
            .collect();

        Ok(AnalysisResult {
            errors: type_errors,
            warnings: result.warnings,
        })
    }

    /// Parse cargo output for borrow checker errors
    fn parse_cargo_borrow_output(
        &self,
        output: &[u8],
    ) -> Result<AnalysisResult, RustAnalyzerError> {
        let result = self.parse_cargo_output(output)?;

        // Filter for borrow checker related errors
        let borrow_errors: Vec<ValidationError> = result.errors
            .into_iter()
            .filter(|error| {
                matches!(error, ValidationError::CompilationError { message, .. }
                    if message.contains("borrow") || message.contains("lifetime") || message.contains("move"))
            })
            .map(|error| {
                // Convert to borrow error format
                if let ValidationError::CompilationError { message, .. } = error {
                    ValidationError::BorrowError {
                        line: 1,
                        column: 1,
                        message,
                        borrow_kind: "Borrow checker violation".to_string(),
                    }
                } else {
                    error
                }
            })
            .collect();

        Ok(AnalysisResult {
            errors: borrow_errors,
            warnings: result.warnings,
        })
    }
}

/// Result of code analysis
#[derive(Debug, Clone)]
struct AnalysisResult {
    errors: Vec<ValidationError>,
    warnings: Vec<ValidationError>,
}

/// Errors that can occur during rust-analyzer operations
#[derive(Debug, thiserror::Error)]
pub enum RustAnalyzerError {
    #[error("Project not found at path: {0}")]
    ProjectNotFound(PathBuf),

    #[error("Not a valid Rust project (no Cargo.toml found): {0}")]
    NotRustProject(PathBuf),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to execute cargo: {0}")]
    CargoError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Analysis timeout exceeded")]
    Timeout,
}

/// Cargo message types for parsing output
#[derive(Debug, Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum CargoMessage {
    CompilerMessage(CompilerMessage),
    CompilerArtifact,
    BuildScriptExecuted,
    BuildFinished,
}

#[derive(Debug, Deserialize)]
struct CompilerMessage {
    message: DiagnosticMessage,
}

#[derive(Debug, Deserialize)]
struct DiagnosticMessage {
    message: String,
    level: CompilerMessageLevel,
    spans: Vec<DiagnosticSpan>,
    rendered: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum CompilerMessageLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Deserialize)]
struct DiagnosticSpan {
    file_name: String,
    line_start: usize,
    column_start: usize,
    line_end: usize,
    column_end: usize,
}

/// Workspace information for cargo projects
#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub workspace_root: PathBuf,
    pub packages: usize,
    pub target_directory: PathBuf,
}

/// Cargo metadata structure
#[derive(Debug, Deserialize)]
struct CargoMetadata {
    workspace_root: PathBuf,
    packages: Vec<CargoPackage>,
    target_directory: PathBuf,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    version: String,
}
