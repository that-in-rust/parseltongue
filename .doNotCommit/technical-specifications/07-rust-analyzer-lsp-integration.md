# Rust-Analyzer LSP Integration Specification

## Overview

This specification defines the executable contract for rust-analyzer LSP integration in Parseltongue, providing enhanced validation and metadata extraction for Rust projects. Following TDD-first principles with measurable outcomes, structured error handling, and performance validation.

## Executable Specification Contract

### Performance Requirements (Test-Validated)

```rust
#[cfg(test)]
mod performance_contracts {
    use super::*;
    use std::time::Instant;

    /// **Executable Contract**: LSP server startup must complete within 10 seconds
    #[test]
    fn lsp_startup_performance_contract() {
        let start = Instant::now();

        let lsp_client = LspClient::new("./test_rust_project").unwrap();
        lsp_client.start_server().unwrap();

        let duration = start.elapsed();
        assert!(duration < Duration::from_secs(10),
            "LSP startup took {:?}, expected < 10s", duration);

        lsp_client.shutdown().unwrap();
    }

    /// **Executable Contract**: Semantic token requests must complete within 2 seconds
    #[tokio::test]
    async fn semantic_token_performance_contract() {
        let lsp_client = create_test_lsp_client().await;
        let test_file = "./test_rust_project/src/main.rs";

        let start = Instant::now();
        let tokens = lsp_client.get_semantic_tokens(test_file).await.unwrap();
        let duration = start.elapsed();

        assert!(duration < Duration::from_secs(2),
            "Semantic token request took {:?}, expected < 2s", duration);
        assert!(!tokens.is_empty(), "Expected non-empty token list");
    }

    /// **Executable Contract**: Type information must be accurate for all test cases
    #[tokio::test]
    async fn type_information_accuracy_contract() {
        let lsp_client = create_test_lsp_client().await;
        let test_cases = load_type_information_test_cases();

        for test_case in test_cases {
            let type_info = lsp_client.get_type_info(&test_case.position).await.unwrap();

            assert_eq!(type_info.resolved_type, test_case.expected_type,
                "Type mismatch for {:?}: expected {}, got {}",
                test_case.position, test_case.expected_type, type_info.resolved_type);
        }
    }
}
```

## Core Data Structures

### LSP Client Interface

```rust
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::process::Child;

/// **Executable Contract**: LSP client must provide all required metadata extraction
pub trait LspClient: Send + Sync {
    /// Start the LSP server for the given project
    async fn start_server(&mut self) -> Result<(), LspError>;

    /// Get semantic tokens for a file
    async fn get_semantic_tokens(&self, file_path: &str) -> Result<Vec<SemanticToken>, LspError>;

    /// Get type information for a position
    async fn get_type_info(&self, position: &Position) -> Result<TypeInformation, LspError>;

    /// Get usage analysis for an entity
    async fn get_usage_analysis(&self, isgl1_key: &str) -> Result<UsageAnalysis, LspError>;

    /// Get implementation locations for a trait
    async fn get_implementations(&self, position: &Position) -> Result<Vec<Location>, LspError>;

    /// Shutdown the LSP server
    async fn shutdown_server(&mut self) -> Result<(), LspError>;

    /// Check server health
    async fn health_check(&self) -> Result<HealthStatus, LspError>;
}

/// **Executable Contract**: Position must be unambiguous and testable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub file_path: String,
    pub line: u32,        // 0-based
    pub character: u32,   // 0-based
}

impl Position {
    /// Create position from ISGL1 key and line offset
    pub fn from_isgl1_key(isgl1_key: &str, line_offset: u32) -> Result<Self, PositionError> {
        let file_path = extract_file_path_from_isgl1(isgl1_key)?;
        Ok(Position {
            file_path,
            line: line_offset,
            character: 0,
        })
    }

    /// Convert to LSP position format
    pub fn to_lsp_position(&self) -> lsp_types::Position {
        lsp_types::Position {
            line: self.line,
            character: self.character,
        }
    }
}
```

### Semantic Information Structures

```rust
/// **Executable Contract**: Semantic tokens must provide complete type information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticToken {
    /// Token position in the file
    pub position: Position,

    /// Token length in characters
    pub length: u32,

    /// Token type (function, struct, etc.)
    pub token_type: SemanticTokenType,

    /// Token modifiers (public, private, etc.)
    pub modifiers: Vec<SemanticTokenModifier>,

    /// Resolved type information
    pub resolved_type: Option<String>,

    /// Module path for this token
    pub module_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SemanticTokenType {
    Function,
    Method,
    Struct,
    Enum,
    Interface,
    Variable,
    Parameter,
    Property,
    TypeParameter,
    Keyword,
    Comment,
    String,
    Number,
    Regexp,
    Operator,
    Namespace,
    Type,
    Struct,
    Class,
    Interface,
    Parameter,
    Variable,
    Property,
    EnumMember,
    Event,
}

/// **Executable Contract**: Type information must be comprehensive and accurate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeInformation {
    /// The resolved type of the symbol
    pub resolved_type: String,

    /// Full module path
    pub module_path: Vec<String>,

    /// Generic parameters if any
    pub generic_parameters: Vec<String>,

    /// Lifetime parameters if any
    pub lifetime_parameters: Vec<String>,

    /// Where clauses if any
    pub where_clauses: Vec<String>,

    /// Documentation comment
    pub documentation: Option<String>,

    /// Source location of type definition
    pub definition_location: Option<Location>,
}

/// **Executable Contract**: Usage analysis must be comprehensive
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsageAnalysis {
    /// Total number of references
    pub total_references: usize,

    /// Locations where this entity is used
    pub usage_locations: Vec<Location>,

    /// Types of usage (read, write, call, etc.)
    pub usage_types: Vec<UsageType>,

    /// Dependent entities
    pub dependents: Vec<String>,

    /// Import statements that bring this entity into scope
    pub imports: Vec<String>,

    /// Re-exports of this entity
    pub re_exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UsageType {
    Read,
    Write,
    Call,
    TypeReference,
    PatternMatch,
    MacroExpansion,
}
```

## Rust-Analyzer Client Implementation

### Primary Client Implementation

```rust
/// **Executable Contract**: Rust-analyzer client with full feature support
pub struct RustAnalyzerClient {
    /// LSP server process
    server_process: Option<Child>,

    /// Communication channels
    sender: UnboundedSender<lsp_types::Request>,
    receiver: Arc<Mutex<UnboundedReceiver<lsp_types::Notification>>>,

    /// Project configuration
    project_root: PathBuf,
    cargo_manifest: CargoManifest,

    /// Performance tracking
    metrics: Arc<RwLock<LspMetrics>>,

    /// Request tracking
    pending_requests: Arc<RwLock<HashMap<i32, oneshot::Sender<Result<lsp_types::Response, LspError>>>>>,

    /// Next request ID
    next_request_id: Arc<AtomicU64>,
}

impl RustAnalyzerClient {
    /// Create new client for given project
    pub async fn new(project_root: &Path) -> Result<Self, LspError> {
        // Validate project structure
        let cargo_manifest = CargoManifest::load(project_root)?;
        self.validate_project_structure(&cargo_manifest)?;

        // Initialize client
        let client = Self {
            server_process: None,
            sender: create_lsp_channel()?,
            receiver: Arc::new(Mutex::new(create_lsp_receiver())),
            project_root: project_root.to_path_buf(),
            cargo_manifest,
            metrics: Arc::new(RwLock::new(LspMetrics::new())),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            next_request_id: Arc::new(AtomicU64::new(1)),
        };

        Ok(client)
    }

    /// **Executable Contract**: Server startup with comprehensive error handling
    async fn start_server(&mut self) -> Result<(), LspError> {
        // Find rust-analyzer binary
        let rust_analyzer_path = self.find_rust_analyzer()?;

        // Start server process
        let mut server_process = tokio::process::Command::new(rust_analyzer_path)
            .arg("--stdio")
            .current_dir(&self.project_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| LspError::ServerStartFailed {
                source: e,
                path: rust_analyzer_path,
            })?;

        // Initialize LSP connection
        self.initialize_lsp_connection(&mut server_process).await?;

        self.server_process = Some(server_process);
        Ok(())
    }

    /// **Executable Contract**: LSP initialization with timeout and validation
    async fn initialize_lsp_connection(&mut self, process: &mut Child) -> Result<(), LspError> {
        // Take stdin/stdout
        let stdin = process.stdin.take().ok_or(LspError::MissingStdio)?;
        let stdout = process.stdout.take().ok_or(LspError::MissingStdio)?;

        // Create JSON-RPC transport
        let (transport, _handle) = LspTransport::new(stdin, stdout);

        // Send initialize request
        let initialize_params = lsp_types::InitializeParams {
            root_uri: Some(lsp_types::Url::from_file_path(&self.project_root).unwrap()),
            initialization_options: Some(self.create_initialization_options()),
            capabilities: self.create_client_capabilities(),
            ..Default::default()
        };

        let initialize_result = self
            .send_request::<lsp_types::request::Initialize>(initialize_params)
            .await?;

        // Validate server capabilities
        self.validate_server_capabilities(&initialize_result.capabilities)?;

        // Send initialized notification
        self.send_notification(lsp_types::notification::Initialized::default()).await?;

        // Open workspace files
        self.open_workspace_files().await?;

        Ok(())
    }
}

impl LspClient for RustAnalyzerClient {
    async fn get_semantic_tokens(&self, file_path: &str) -> Result<Vec<SemanticToken>, LspError> {
        let start = Instant::now();

        let text_document_identifier = lsp_types::TextDocumentIdentifier {
            uri: lsp_types::Url::from_file_path(file_path).map_err(|e| LspError::InvalidPath {
                path: file_path.to_string(),
                source: e,
            })?,
        };

        let request = lsp_types::SemanticTokensParams {
            text_document: text_document_identifier,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        let response = self
            .send_request::<lsp_types::request::SemanticTokensFullRequest>(request)
            .await?;

        let tokens = self.convert_semantic_tokens_response(response, file_path)?;

        // Record metrics
        self.metrics.write().unwrap().record_semantic_token_request(start.elapsed());

        Ok(tokens)
    }

    async fn get_type_info(&self, position: &Position) -> Result<TypeInformation, LspError> {
        let text_document_position = lsp_types::TextDocumentPositionParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: lsp_types::Url::from_file_path(&position.file_path)
                    .map_err(|e| LspError::InvalidPath {
                        path: position.file_path.clone(),
                        source: e,
                    })?,
            },
            position: position.to_lsp_position(),
        };

        let request = lsp_types::HoverParams {
            text_document_position_params: text_document_position,
            work_done_progress_params: Default::default(),
        };

        let hover_response = self
            .send_request::<lsp_types::request::HoverRequest>(request)
            .await?;

        self.convert_hover_response(hover_response, position).await
    }

    async fn get_usage_analysis(&self, isgl1_key: &str) -> Result<UsageAnalysis, LspError> {
        // Parse ISGL1 key to get file and entity name
        let (file_path, entity_name) = parse_isgl1_key(isgl1_key)?;

        // Find entity definition
        let definition_position = self.find_entity_definition(&file_path, &entity_name).await?;

        // Get references
        let references = self.find_all_references(&definition_position).await?;

        // Analyze usage patterns
        self.analyze_usage_patterns(references, isgl1_key).await
    }

    async fn get_implementations(&self, position: &Position) -> Result<Vec<Location>, LspError> {
        let text_document_position = lsp_types::TextDocumentPositionParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: lsp_types::Url::from_file_path(&position.file_path)
                    .map_err(|e| LspError::InvalidPath {
                        path: position.file_path.clone(),
                        source: e,
                    })?,
            },
            position: position.to_lsp_position(),
        };

        let request = lsp_types::GotoImplementationParams {
            text_document_position_params: text_document_position,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        let response = self
            .send_request::<lsp_types::request::GotoImplementationRequest>(request)
            .await?;

        self.convert_locations_response(response)
    }

    async fn health_check(&self) -> Result<HealthStatus, LspError> {
        // Send a simple request to check if server is responsive
        let request = lsp_types::request::WorkspaceFoldersRequest {};

        match tokio::time::timeout(Duration::from_secs(5),
            self.send_request::<lsp_types::request::WorkspaceFoldersRequest>(request)
        ).await {
            Ok(Ok(_)) => Ok(HealthStatus::Healthy),
            Ok(Err(e)) => Ok(HealthStatus::Unhealthy { reason: format!("Request failed: {}", e) }),
            Err(_) => Ok(HealthStatus::Unhealthy { reason: "Request timeout".to_string() }),
        }
    }
}
```

### Request/Response Handling

```rust
impl RustAnalyzerClient {
    /// **Executable Contract**: Generic request handling with timeout and error recovery
    async fn send_request<R>(&self, params: R::Params) -> Result<R::Result, LspError>
    where
        R: lsp_types::request::Request,
        R::Params: serde::Serialize,
        R::Result: serde::de::DeserializeOwned,
    {
        let request_id = self.next_request_id.fetch_add(1, Ordering::SeqCst) as i32;

        // Create response channel
        let (response_sender, response_receiver) = oneshot::channel();

        // Register pending request
        self.pending_requests.write().unwrap().insert(request_id, response_sender);

        // Build LSP request
        let lsp_request = lsp_types::Request {
            id: request_id.into(),
            method: R::METHOD.to_string(),
            params: serde_json::to_value(params)
                .map_err(|e| LspError::SerializationError {
                    details: format!("Failed to serialize request: {}", e),
                })?,
        };

        // Send request
        self.sender.send(lsp_request).map_err(|_| LspError::ConnectionLost)?;

        // Wait for response with timeout
        match tokio::time::timeout(Duration::from_secs(30), response_receiver).await {
            Ok(Ok(response)) => response,
            Ok(Err(_)) => Err(LspError::RequestCanceled),
            Err(_) => {
                // Remove pending request on timeout
                self.pending_requests.write().unwrap().remove(&request_id);
                Err(LspError::RequestTimeout)
            }
        }
    }

    /// **Executable Contract**: Response conversion with validation
    fn convert_semantic_tokens_response(
        &self,
        response: lsp_types::SemanticTokensResult,
        file_path: &str,
    ) -> Result<Vec<SemanticToken>, LspError> {
        match response {
            lsp_types::SemanticTokensResult::Tokens(tokens) => {
                self.decode_semantic_tokens(tokens, file_path)
            }
            lsp_types::SemanticTokensResult::Partial(_) => {
                Err(LspError::UnsupportedFeature {
                    feature: "Partial semantic tokens".to_string(),
                })
            }
        }
    }

    /// **Executable Contract**: Semantic token decoding with comprehensive validation
    fn decode_semantic_tokens(
        &self,
        tokens: lsp_types::SemanticTokens,
        file_path: &str,
    ) -> Result<Vec<SemanticToken>, LspError> {
        let mut decoded_tokens = Vec::new();
        let mut current_line = 0u32;
        let mut current_char = 0u32;

        // Read file content for token text extraction
        let file_content = std::fs::read_to_string(file_path)
            .map_err(|e| LspError::FileReadError {
                path: file_path.to_string(),
                source: e,
            })?;

        for chunk in tokens.data.chunks_exact(5) {
            let delta_line = chunk[0];
            let delta_char = chunk[1];
            let length = chunk[2];
            let token_type = chunk[3];
            let token_modifiers = chunk[4];

            // Update position
            if delta_line != 0 {
                current_line += delta_line;
                current_char = delta_char;
            } else {
                current_char += delta_char;
            }

            // Extract token text
            let token_text = self.extract_token_text(
                &file_content,
                current_line,
                current_char,
                length,
            )?;

            // Convert token type and modifiers
            let semantic_type = self.convert_token_type(token_type)?;
            let modifiers = self.convert_token_modifiers(token_modifiers);

            let token = SemanticToken {
                position: Position {
                    file_path: file_path.to_string(),
                    line: current_line,
                    character: current_char,
                },
                length,
                token_type: semantic_type,
                modifiers,
                resolved_type: None, // Will be filled by additional analysis
                module_path: Vec::new(), // Will be filled by additional analysis
            };

            decoded_tokens.push(token);
        }

        Ok(decoded_tokens)
    }
}
```

## Error Handling Contract

### Structured Error Types

```rust
/// **Executable Contract**: All LSP errors must be structured and actionable
#[derive(Debug, Error)]
pub enum LspError {
    #[error("LSP server start failed: {source}")]
    ServerStartFailed { source: std::io::Error, path: String },

    #[error("LSP server not running")]
    ServerNotRunning,

    #[error("Connection to LSP server lost")]
    ConnectionLost,

    #[error("Request timeout after 30 seconds")]
    RequestTimeout,

    #[error("Request canceled by client")]
    RequestCanceled,

    #[error("Invalid file path: {path}")]
    InvalidPath { path: String, source: url::ParseError },

    #[error("File read error: {path}")]
    FileReadError { path: String, source: std::io::Error },

    #[error("Missing stdio for LSP process")]
    MissingStdio,

    #[error("Unsupported LSP feature: {feature}")]
    UnsupportedFeature { feature: String },

    #[error("Serialization error: {details}")]
    SerializationError { details: String },

    #[error("Deserialization error: {details}")]
    DeserializationError { details: String },

    #[error("Invalid server capabilities")]
    InvalidServerCapabilities,

    #[error("Project validation failed: {reason}")]
    ProjectValidationFailed { reason: String },
}

/// **Executable Contract**: Position parsing errors must be specific
#[derive(Debug, Error)]
pub enum PositionError {
    #[error("Invalid ISGL1 key format: {key}")]
    InvalidIsgl1Format { key: String },

    #[error("File path not found in ISGL1 key: {key}")]
    FilePathNotFound { key: String },

    #[error("Invalid line offset: {offset}")]
    InvalidLineOffset { offset: u32 },
}
```

## Test Infrastructure

### Mock LSP Client

```rust
/// **Executable Contract**: Test double implementation for unit testing
pub struct MockLspClient {
    semantic_tokens: Arc<RwLock<HashMap<String, Vec<SemanticToken>>>>,
    type_information: Arc<RwLock<HashMap<Position, TypeInformation>>>,
    usage_analysis: Arc<RwLock<HashMap<String, UsageAnalysis>>>,
    call_log: Arc<RwLock<Vec<String>>>,
}

impl MockLspClient {
    pub fn new() -> Self {
        Self {
            semantic_tokens: Arc::new(RwLock::new(HashMap::new())),
            type_information: Arc::new(RwLock::new(HashMap::new())),
            usage_analysis: Arc::new(RwLock::new(HashMap::new())),
            call_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Setup semantic tokens for test file
    pub fn set_semantic_tokens(&self, file_path: &str, tokens: Vec<SemanticToken>) {
        self.semantic_tokens.write().unwrap().insert(file_path.to_string(), tokens);
    }

    /// Setup type information for test position
    pub fn set_type_info(&self, position: Position, type_info: TypeInformation) {
        self.type_information.write().unwrap().insert(position, type_info);
    }

    /// Get call log for test verification
    pub fn get_call_log(&self) -> Vec<String> {
        self.call_log.read().unwrap().clone()
    }
}

impl LspClient for MockLspClient {
    async fn start_server(&mut self) -> Result<(), LspError> {
        self.call_log.write().unwrap().push("start_server".to_string());
        Ok(())
    }

    async fn get_semantic_tokens(&self, file_path: &str) -> Result<Vec<SemanticToken>, LspError> {
        self.call_log.write().unwrap().push(format!("get_semantic_tokens: {}", file_path));

        let tokens = self.semantic_tokens.read().unwrap();
        Ok(tokens.get(file_path).cloned().unwrap_or_default())
    }

    async fn get_type_info(&self, position: &Position) -> Result<TypeInformation, LspError> {
        self.call_log.write().unwrap().push(format!("get_type_info: {:?}", position));

        let type_info = self.type_information.read().unwrap();
        Ok(type_info.get(position).cloned().unwrap_or_else(|| TypeInformation {
            resolved_type: "Unknown".to_string(),
            module_path: Vec::new(),
            generic_parameters: Vec::new(),
            lifetime_parameters: Vec::new(),
            where_clauses: Vec::new(),
            documentation: None,
            definition_location: None,
        }))
    }

    async fn get_usage_analysis(&self, isgl1_key: &str) -> Result<UsageAnalysis, LspError> {
        self.call_log.write().unwrap().push(format!("get_usage_analysis: {}", isgl1_key));

        let usage = self.usage_analysis.read().unwrap();
        Ok(usage.get(isgl1_key).cloned().unwrap_or_default())
    }

    async fn get_implementations(&self, position: &Position) -> Result<Vec<Location>, LspError> {
        self.call_log.write().unwrap().push(format!("get_implementations: {:?}", position));
        Ok(Vec::new())
    }

    async fn shutdown_server(&mut self) -> Result<(), LspError> {
        self.call_log.write().unwrap().push("shutdown_server".to_string());
        Ok(())
    }

    async fn health_check(&self) -> Result<HealthStatus, LspError> {
        self.call_log.write().unwrap().push("health_check".to_string());
        Ok(HealthStatus::Healthy)
    }
}
```

### Integration Test Examples

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// **Executable Contract**: Complete LSP workflow validation
    #[tokio::test]
    async fn complete_lsp_workflow_contract() {
        // Setup test project
        let test_project = create_test_rust_project();
        let mut lsp_client = RustAnalyzerClient::new(&test_project.path).await.unwrap();

        // Start server
        lsp_client.start_server().await.unwrap();

        // Test semantic tokens
        let tokens = lsp_client.get_semantic_tokens("src/main.rs").await.unwrap();
        assert!(!tokens.is_empty(), "Expected semantic tokens for main.rs");

        // Validate token structure
        for token in &tokens {
            assert!(!token.position.file_path.is_empty());
            assert!(token.length > 0);
            assert!(!matches!(token.token_type, SemanticTokenType::Comment));
        }

        // Test type information
        if let Some(first_token) = tokens.first() {
            let type_info = lsp_client.get_type_info(&first_token.position).await.unwrap();
            assert!(!type_info.resolved_type.is_empty());
        }

        // Test usage analysis
        let usage = lsp_client.get_usage_analysis("src/main.rs-main-main").await.unwrap();
        assert!(usage.total_references >= 0);

        // Cleanup
        lsp_client.shutdown_server().await.unwrap();
    }
}
```

This rust-analyzer LSP integration specification provides a comprehensive, testable foundation for enhanced Rust project validation while following the steering docs principles of TDD-first development, executable specifications, and structured error handling.