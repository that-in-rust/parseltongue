//! Tool 2 integration module for parsing simulation outputs and converting to validation tests
//! Handles the data flow between Tool 2 (simulation) and Tool 3 (validation)

use crate::validation::{ValidationReport, ValidationTestCase, ValidationType};
use parseltongue_01::types::ISGL1Key;
// use parseltongue_03::{ChangeRequest, SimulationPlan, CozoCodeSimulationSorcerer}; // Not used in GREEN phase
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Parser for Tool 2 simulation outputs
#[derive(Debug, Clone)]
pub struct Tool2SimulationParser {
    /// Parser configuration
    config: ParserConfig,
}

/// Configuration for parsing Tool 2 outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Whether to extract code snippets from simulation steps
    pub extract_code_snippets: bool,
    /// Whether to validate simulation outputs before parsing
    pub validate_simulation_output: bool,
    /// Maximum size of code snippets to extract
    pub max_snippet_size_bytes: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            extract_code_snippets: true,
            validate_simulation_output: true,
            max_snippet_size_bytes: 100 * 1024, // 100KB
        }
    }
}

impl Tool2SimulationParser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create a parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse Tool 2 simulation output into structured data
    pub fn parse_simulation_output(
        &self,
        simulation_output: &str,
    ) -> Result<ParsedSimulationOutput, Tool2IntegrationError> {
        if self.config.validate_simulation_output && simulation_output.trim().is_empty() {
            return Err(Tool2IntegrationError::EmptySimulationOutput);
        }

        // Parse the simulation output (simplified for GREEN phase)
        // In real implementation, this would parse the actual JSON/structured format
        let mut parsed = ParsedSimulationOutput::new();

        // Extract code snippets if enabled
        if self.config.extract_code_snippets {
            let snippets = self.extract_code_snippets(simulation_output)?;
            parsed.code_snippets = snippets;
        }

        // Extract change requests
        let change_requests = self.extract_change_requests(simulation_output)?;
        parsed.change_requests = change_requests;

        // Extract simulation steps
        let simulation_steps = self.extract_simulation_steps(simulation_output)?;
        parsed.simulation_steps = simulation_steps;

        Ok(parsed)
    }

    /// Extract code snippets from simulation output
    fn extract_code_snippets(
        &self,
        simulation_output: &str,
    ) -> Result<Vec<CodeSnippet>, Tool2IntegrationError> {
        let mut snippets = Vec::new();

        // Simple regex-based extraction for GREEN phase
        // Real implementation would use proper parsing
        for (line_num, line) in simulation_output.lines().enumerate() {
            if line.trim().starts_with("```rust") {
                // Found a Rust code block
                let snippet = CodeSnippet {
                    id: format!("snippet_{}", snippets.len()),
                    language: "rust".to_string(),
                    content: line.trim_start_matches("```rust").trim().to_string(),
                    line_number: line_num,
                    context: Some(line.to_string()),
                };
                snippets.push(snippet);
            } else if line.contains("fn ") || line.contains("struct ") || line.contains("impl ") {
                // Found potential Rust code
                if line.len() <= self.config.max_snippet_size_bytes {
                    let snippet = CodeSnippet {
                        id: format!("snippet_{}", snippets.len()),
                        language: "rust".to_string(),
                        content: line.to_string(),
                        line_number: line_num,
                        context: Some(line.to_string()),
                    };
                    snippets.push(snippet);
                }
            }
        }

        Ok(snippets)
    }

    /// Extract change requests from simulation output
    fn extract_change_requests(
        &self,
        simulation_output: &str,
    ) -> Result<Vec<ExtractedChangeRequest>, Tool2IntegrationError> {
        let mut change_requests = Vec::new();

        // Simple extraction for GREEN phase
        // Look for patterns that suggest change requests
        for (line_num, line) in simulation_output.lines().enumerate() {
            if line.contains("change:") || line.contains("modify:") || line.contains("update:") {
                let change_request = ExtractedChangeRequest {
                    id: format!("change_{}", change_requests.len()),
                    description: line.to_string(),
                    target_file: None,
                    change_type: self.infer_change_type(line),
                    line_number: Some(line_num),
                    priority: ChangePriority::Medium,
                };
                change_requests.push(change_request);
            }
        }

        Ok(change_requests)
    }

    /// Extract simulation steps from simulation output
    fn extract_simulation_steps(
        &self,
        simulation_output: &str,
    ) -> Result<Vec<SimulationStep>, Tool2IntegrationError> {
        let mut steps = Vec::new();

        // Simple step extraction for GREEN phase
        for (line_num, line) in simulation_output.lines().enumerate() {
            if line.contains("Step") || line.contains("Phase") {
                let step = SimulationStep {
                    id: format!("step_{}", steps.len()),
                    phase: self.infer_phase(line),
                    description: line.to_string(),
                    line_number: line_num,
                    status: StepStatus::Completed,
                };
                steps.push(step);
            }
        }

        Ok(steps)
    }

    /// Infer change type from description
    fn infer_change_type(&self, description: &str) -> ChangeType {
        if description.contains("function") || description.contains("fn ") {
            ChangeType::FunctionModification
        } else if description.contains("struct") || description.contains("enum") {
            ChangeType::TypeDefinition
        } else if description.contains("impl") {
            ChangeType::Implementation
        } else if description.contains("import") || description.contains("use") {
            ChangeType::ImportChange
        } else {
            ChangeType::Unknown
        }
    }

    /// Infer phase from step description
    fn infer_phase(&self, description: &str) -> SimulationPhase {
        if description.contains("A01") || description.contains("A02") {
            SimulationPhase::Analysis
        } else if description.contains("B01") || description.contains("B02") {
            SimulationPhase::Planning
        } else if description.contains("C") {
            SimulationPhase::Implementation
        } else if description.contains("D") {
            SimulationPhase::Validation
        } else {
            SimulationPhase::Unknown
        }
    }
}

/// Parsed simulation output from Tool 2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSimulationOutput {
    /// Extracted code snippets
    pub code_snippets: Vec<CodeSnippet>,
    /// Extracted change requests
    pub change_requests: Vec<ExtractedChangeRequest>,
    /// Simulation steps
    pub simulation_steps: Vec<SimulationStep>,
    /// Metadata about the parsing
    pub metadata: ParseMetadata,
}

impl ParsedSimulationOutput {
    /// Create a new parsed simulation output
    pub fn new() -> Self {
        Self {
            code_snippets: Vec::new(),
            change_requests: Vec::new(),
            simulation_steps: Vec::new(),
            metadata: ParseMetadata {
                parsed_at: chrono::Utc::now(),
                parser_version: "1.0.0".to_string(),
            },
        }
    }

    /// Get all Rust code snippets
    pub fn rust_snippets(&self) -> Vec<&CodeSnippet> {
        self.code_snippets
            .iter()
            .filter(|snippet| snippet.language == "rust")
            .collect()
    }

    /// Get snippets by line number range
    pub fn snippets_in_range(&self, start: usize, end: usize) -> Vec<&CodeSnippet> {
        self.code_snippets
            .iter()
            .filter(|snippet| snippet.line_number >= start && snippet.line_number <= end)
            .collect()
    }
}

/// Code snippet extracted from simulation output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    /// Unique identifier
    pub id: String,
    /// Programming language
    pub language: String,
    /// Code content
    pub content: String,
    /// Line number in original output
    pub line_number: usize,
    /// Context information
    pub context: Option<String>,
}

/// Extracted change request from simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedChangeRequest {
    /// Unique identifier
    pub id: String,
    /// Change description
    pub description: String,
    /// Target file (if specified)
    pub target_file: Option<PathBuf>,
    /// Type of change
    pub change_type: ChangeType,
    /// Line number (if applicable)
    pub line_number: Option<usize>,
    /// Change priority
    pub priority: ChangePriority,
}

/// Simulation step extracted from output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStep {
    /// Unique identifier
    pub id: String,
    /// Phase of the step
    pub phase: SimulationPhase,
    /// Step description
    pub description: String,
    /// Line number in output
    pub line_number: usize,
    /// Step status
    pub status: StepStatus,
}

/// Metadata about the parsing operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseMetadata {
    /// When parsing occurred
    pub parsed_at: chrono::DateTime<chrono::Utc>,
    /// Parser version used
    pub parser_version: String,
}

/// Types of changes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    FunctionModification,
    TypeDefinition,
    Implementation,
    ImportChange,
    Unknown,
}

/// Change priority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Simulation phases
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulationPhase {
    Analysis,       // Phase A
    Planning,       // Phase B
    Implementation, // Phase C
    Validation,     // Phase D
    Unknown,
}

/// Step status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Converter from simulation output to validation test cases
#[derive(Debug, Clone)]
pub struct SimulationToValidationConverter {
    /// Conversion configuration
    config: ConversionConfig,
}

/// Configuration for converting simulation to validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    /// Whether to generate test cases for all snippets
    pub generate_tests_for_all_snippets: bool,
    /// Default validation expectations
    pub default_expectations: ValidationExpectations,
    /// Mapping of change types to validation types
    pub change_type_mapping: HashMap<ChangeType, Vec<ValidationType>>,
}

/// Default expectations for validation test cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExpectations {
    /// Expected syntax validation result
    pub expect_syntax_valid: bool,
    /// Expected type validation result
    pub expect_type_valid: bool,
    /// Expected compilation validation result
    pub expect_compilation_valid: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        let mut change_type_mapping = HashMap::new();

        change_type_mapping.insert(
            ChangeType::FunctionModification,
            vec![
                ValidationType::Syntax,
                ValidationType::Type,
                ValidationType::Compilation,
            ],
        );

        change_type_mapping.insert(
            ChangeType::TypeDefinition,
            vec![
                ValidationType::Syntax,
                ValidationType::Type,
                ValidationType::Compilation,
            ],
        );

        change_type_mapping.insert(
            ChangeType::Implementation,
            vec![
                ValidationType::Syntax,
                ValidationType::Type,
                ValidationType::BorrowChecker,
                ValidationType::Compilation,
            ],
        );

        Self {
            generate_tests_for_all_snippets: true,
            default_expectations: ValidationExpectations {
                expect_syntax_valid: true,
                expect_type_valid: true,
                expect_compilation_valid: true,
            },
            change_type_mapping,
        }
    }
}

impl SimulationToValidationConverter {
    /// Create a new converter with default configuration
    pub fn new() -> Self {
        Self {
            config: ConversionConfig::default(),
        }
    }

    /// Create a converter with custom configuration
    pub fn with_config(config: ConversionConfig) -> Self {
        Self { config }
    }

    /// Convert parsed simulation output to validation test cases
    pub fn convert_to_test_cases(
        &self,
        parsed_output: &ParsedSimulationOutput,
        base_path: &PathBuf,
    ) -> Result<Vec<ValidationTestCase>, Tool2IntegrationError> {
        let mut test_cases = Vec::new();

        // Convert code snippets to test cases
        if self.config.generate_tests_for_all_snippets {
            for snippet in &parsed_output.code_snippets {
                if snippet.language == "rust" && !snippet.content.trim().is_empty() {
                    let test_case = self.create_test_case_from_snippet(snippet, base_path)?;
                    test_cases.push(test_case);
                }
            }
        }

        // Convert change requests to test cases
        for change_request in &parsed_output.change_requests {
            if let Some(test_case) =
                self.create_test_case_from_change_request(change_request, base_path)?
            {
                test_cases.push(test_case);
            }
        }

        Ok(test_cases)
    }

    /// Create a validation test case from a code snippet
    fn create_test_case_from_snippet(
        &self,
        snippet: &CodeSnippet,
        base_path: &PathBuf,
    ) -> Result<ValidationTestCase, Tool2IntegrationError> {
        let file_path = base_path.join(format!("generated_{}.rs", snippet.id));
        let test_name = format!("validate_snippet_{}", snippet.id);

        Ok(ValidationTestCase::new(
            test_name,
            snippet.content.clone(),
            file_path,
            self.config.default_expectations.expect_syntax_valid,
            self.config.default_expectations.expect_type_valid,
            self.config.default_expectations.expect_compilation_valid,
        ))
    }

    /// Create a validation test case from a change request
    fn create_test_case_from_change_request(
        &self,
        change_request: &ExtractedChangeRequest,
        base_path: &PathBuf,
    ) -> Result<Option<ValidationTestCase>, Tool2IntegrationError> {
        // For GREEN phase, only create test cases if we have actual code
        if !change_request.description.contains("fn ")
            && !change_request.description.contains("struct ")
            && !change_request.description.contains("impl ")
        {
            return Ok(None);
        }

        let file_path = change_request
            .target_file
            .clone()
            .unwrap_or_else(|| base_path.join(format!("change_{}.rs", change_request.id)));

        let test_name = format!("validate_change_{}", change_request.id);

        // Determine expectations based on change type
        let default_types = vec![ValidationType::Syntax, ValidationType::Compilation];
        let validation_types = self
            .config
            .change_type_mapping
            .get(&change_request.change_type)
            .unwrap_or(&default_types);

        let expect_syntax_valid = validation_types.contains(&ValidationType::Syntax);
        let expect_type_valid = validation_types.contains(&ValidationType::Type);
        let expect_compilation_valid = validation_types.contains(&ValidationType::Compilation);

        Ok(Some(ValidationTestCase::new(
            test_name,
            change_request.description.clone(),
            file_path,
            expect_syntax_valid,
            expect_type_valid,
            expect_compilation_valid,
        )))
    }
}

/// Complete integration pipeline for Tool 2 to Tool 3
#[derive(Debug, Clone)]
pub struct Tool2ValidationPipeline {
    /// Parser for Tool 2 outputs
    parser: Tool2SimulationParser,
    /// Converter for creating validation tests
    converter: SimulationToValidationConverter,
    /// Pipeline configuration
    config: PipelineConfig,
}

/// Configuration for the integration pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Base path for generated test files
    pub base_path: PathBuf,
    /// Whether to run validation tests automatically
    pub auto_validate: bool,
    /// Maximum number of test cases to generate
    pub max_test_cases: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./generated_tests"),
            auto_validate: false,
            max_test_cases: 100,
        }
    }
}

impl Tool2ValidationPipeline {
    /// Create a new integration pipeline
    pub fn new() -> Self {
        Self {
            parser: Tool2SimulationParser::new(),
            converter: SimulationToValidationConverter::new(),
            config: PipelineConfig::default(),
        }
    }

    /// Create a pipeline with custom configuration
    pub fn with_config(
        parser_config: ParserConfig,
        conversion_config: ConversionConfig,
        pipeline_config: PipelineConfig,
    ) -> Self {
        Self {
            parser: Tool2SimulationParser::with_config(parser_config),
            converter: SimulationToValidationConverter::with_config(conversion_config),
            config: pipeline_config,
        }
    }

    /// Process Tool 2 simulation output and generate validation tests
    pub async fn process_simulation_output(
        &self,
        simulation_output: &str,
    ) -> Result<IntegrationResult, Tool2IntegrationError> {
        // Parse the simulation output
        let parsed_output = self.parser.parse_simulation_output(simulation_output)?;

        // Convert to validation test cases
        let test_cases = self
            .converter
            .convert_to_test_cases(&parsed_output, &self.config.base_path)?;

        // Limit the number of test cases if configured
        let test_cases = if test_cases.len() > self.config.max_test_cases {
            test_cases
                .into_iter()
                .take(self.config.max_test_cases)
                .collect()
        } else {
            test_cases
        };

        // Create the integration result
        let result = IntegrationResult {
            parsed_output,
            generated_test_cases: test_cases,
            validation_results: Vec::new(), // Will be filled if auto_validate is true
            processing_time_ms: 0,          // Would be measured in real implementation
            success: true,
            errors: Vec::new(),
        };

        Ok(result)
    }
}

/// Result of Tool 2 to Tool 3 integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResult {
    /// Parsed simulation output
    pub parsed_output: ParsedSimulationOutput,
    /// Generated validation test cases
    pub generated_test_cases: Vec<ValidationTestCase>,
    /// Validation results (if auto_validate is enabled)
    pub validation_results: Vec<ValidationReport>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Whether processing was successful
    pub success: bool,
    /// Any errors that occurred
    pub errors: Vec<String>,
}

impl IntegrationResult {
    /// Get the number of generated test cases
    pub fn test_case_count(&self) -> usize {
        self.generated_test_cases.len()
    }

    /// Get test cases by expected validation type
    pub fn test_cases_by_validation_type(
        &self,
        validation_type: ValidationType,
    ) -> Vec<&ValidationTestCase> {
        self.generated_test_cases
            .iter()
            .filter(|test_case| {
                match validation_type {
                    ValidationType::Syntax => test_case.expected_syntax_valid,
                    ValidationType::Type => test_case.expected_type_valid,
                    ValidationType::Compilation => test_case.expected_compilation_valid,
                    _ => false, // Other types not determined by test case expectations
                }
            })
            .collect()
    }

    /// Get total code size across all test cases
    pub fn total_code_size(&self) -> usize {
        self.generated_test_cases
            .iter()
            .map(|tc| tc.code_size_bytes())
            .sum()
    }

    /// Check if all test cases expect validation to pass
    pub fn all_expect_success(&self) -> bool {
        self.generated_test_cases.iter().all(|tc| {
            tc.expected_syntax_valid && tc.expected_type_valid && tc.expected_compilation_valid
        })
    }
}

/// Errors that can occur during Tool 2 integration
#[derive(Debug, thiserror::Error)]
pub enum Tool2IntegrationError {
    #[error("Empty simulation output provided")]
    EmptySimulationOutput,

    #[error("Failed to parse simulation output: {0}")]
    ParseError(String),

    #[error("Invalid code snippet format: {0}")]
    InvalidSnippetFormat(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Validation format for Tool 2 outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool2ValidationFormat {
    /// Format version
    pub version: String,
    /// Simulation data
    pub simulation_data: serde_json::Value,
    /// Validation metadata
    pub validation_metadata: ValidationMetadata,
}

/// Metadata for validation format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetadata {
    /// Source tool (Tool 2)
    pub source_tool: String,
    /// Target tool (Tool 3)
    pub target_tool: String,
    /// Format creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// ISGL1Key mappings (serialized as string for GREEN phase)
    #[serde(skip)]
    pub isgl1_key_mappings: HashMap<String, ISGL1Key>,
}

/// Converter from validation format back to Tool 2 format
#[derive(Debug, Clone)]
pub struct ValidationToTool2Converter;

impl ValidationToTool2Converter {
    /// Convert validation results back to Tool 2 format
    pub fn convert_validation_results(
        validation_reports: &[ValidationReport],
    ) -> Result<Tool2ValidationFormat, Tool2IntegrationError> {
        let simulation_data = serde_json::to_value(validation_reports)?;

        let validation_metadata = ValidationMetadata {
            source_tool: "parseltongue-03".to_string(),
            target_tool: "parseltongue-04".to_string(),
            created_at: chrono::Utc::now(),
            isgl1_key_mappings: HashMap::new(), // Would be populated in real implementation
        };

        Ok(Tool2ValidationFormat {
            version: "1.0.0".to_string(),
            simulation_data,
            validation_metadata,
        })
    }
}
