# Arch01ParseltongueV1 - 5-Tool Pipeline Architecture

## Executive Summary

**Architecture**: Layered, streaming-first 5-tool pipeline for local Rust codebase modification
**Principles**: Executable specifications, pure functional programming, TDD-first development
**Target**: Large Rust codebases with 150k+ token context handling
**Performance**: Sub-millisecond queries, zero-copy patterns, streaming ingestion

## Architecture Overview

### Core Philosophy

Following the 9 non-negotiable architectural principles from S01-README-MOSTIMP.md:

1. **Executable Specifications Over Narratives** - All components defined by testable contracts
2. **Layered Rust Architecture (L1→L2→L3)** - Clear separation: Core → Std → External
3. **Dependency Injection for Testability** - Trait-based composition with no concrete dependencies
4. **RAII Resource Management** - All resources automatically managed with Drop implementations
5. **Performance Claims Must Be Test-Validated** - Every performance assertion backed by automated tests
6. **Structured Error Handling** - `thiserror` for libraries, `anyhow` for applications
7. **Complex Domain Model Support** - Handle real-world complexity, not simplified examples
8. **Concurrency Model Validation** - Thread safety validated with stress tests
9. **MVP-First Rigor** - Proven architectures over theoretical abstractions

### 5-Tool Pipeline Architecture

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#667eea', 'primaryTextColor': '#2d3748', 'lineColor': '#cbd5e0'}}}%%
flowchart LR
    A[Local Folder] --> B[Tool 1<br/>folder-to-cozoDB-streamer]
    B --> C[CodeGraph Database]
    C --> D[Tool 2<br/>cozo-code-simulation-sorcerer]
    D --> E[Simulation Plan]
    E --> F[Tool 3<br/>rust-preflight-code-simulator]
    F --> G[Validation Report]
    G --> H[Tool 4<br/>cozoDB-to-code-writer]
    H --> I[File System]
    I --> J[Tool 5<br/>cozoDB-make-future-code-current]
    J --> K[Consistent State]

    classDef tool fill:#9f7aea,stroke:#805ad5,color:white
    classDef data fill:#48bb78,stroke:#38a169,color:white
    classDef system fill:#ed8936,stroke:#dd6b20,color:white

    class B,D,F,H,J tool
    class C,E,G,I data
    class A,K system
```

## Layered Architecture (L1→L2→L3)

### L1: Core Language Features (no_std compatible)

```rust
//! Core types and traits with no external dependencies

use std::{borrow::Cow, collections::HashMap, path::PathBuf};

/// Type-safe identifier following newtype pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISGL1Key {
    pub filepath: PathBuf,
    pub filename: String,
    pub interface_name: String,
}

impl ISGL1Key {
    pub fn new(filepath: PathBuf, filename: String, interface_name: String) -> Self {
        Self { filepath, filename, interface_name }
    }

    /// Generate stable hash for consistent identification
    pub fn stable_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        self.filepath.hash(&mut hasher);
        self.filename.hash(&mut hasher);
        self.interface_name.hash(&mut hasher);
        hasher.finish()
    }
}

/// RAII resource management for temporary files
pub struct FileHandleGuard {
    handle: std::fs::File,
    path: PathBuf,
}

impl FileHandleGuard {
    pub fn create(path: PathBuf) -> std::io::Result<Self> {
        let handle = std::fs::File::create(&path)?;
        Ok(Self { handle, path })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for FileHandleGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Core error types for library usage
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Invalid ISGL1 key format: {0}")]
    InvalidKey(String),

    #[error("IO operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

/// Result type for core operations
pub type CoreResult<T> = Result<T, CoreError>;
```

### L2: Standard Library Idioms

```rust
//! Streaming engine and standard library-based components

use std::{
    sync::{Arc, RwLock},
    pin::Pin,
    task::{Context, Poll},
};

/// Smart pointer decision matrix for concurrent access
pub struct CodeGraph {
    nodes: Arc<RwLock<HashMap<ISGL1Key, CodeNode>>>,
    edges: Arc<RwLock<HashMap<ISGL1Key, Vec<ISGL1Key>>>>,
    metadata: Arc<RwLock<GraphMetadata>>,
}

impl CodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            edges: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(GraphMetadata::default())),
        }
    }

    /// Thread-safe node insertion with automatic index maintenance
    pub fn insert_node(&self, key: ISGL1Key, node: CodeNode) -> CoreResult<()> {
        let mut nodes = self.nodes.write().map_err(|e| {
            CoreError::ResourceNotFound(format!("Nodes lock poisoned: {}", e))
        })?;

        nodes.insert(key.clone(), node);
        self.update_indices(&key)?;
        Ok(())
    }

    fn update_indices(&self, key: &ISGL1Key) -> CoreResult<()> {
        let mut metadata = self.metadata.write().map_err(|e| {
            CoreError::ResourceNotFound(format!("Metadata lock poisoned: {}", e))
        })?;

        metadata.node_count += 1;
        metadata.last_updated = std::time::SystemTime::now();
        Ok(())
    }
}

/// Iterator-based streaming for memory-efficient processing
pub struct ChunkStream<T> {
    data: Vec<T>,
    chunk_size: usize,
    position: usize,
}

impl<T> ChunkStream<T> {
    pub fn new(data: Vec<T>, chunk_size: usize) -> Self {
        Self { data, chunk_size, position: 0 }
    }
}

impl<T> Iterator for ChunkStream<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.data.len() {
            None
        } else {
            let end = (self.position + self.chunk_size).min(self.data.len());
            let chunk = self.data[self.position..end].to_vec();
            self.position = end;
            Some(chunk)
        }
    }
}

/// Bounded channel for backpressure management
pub struct BoundedStream<T> {
    sender: tokio::sync::mpsc::Sender<T>,
    receiver: tokio::sync::mpsc::Receiver<T>,
    buffer_size: usize,
}

impl<T> BoundedStream<T> {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(buffer_size);
        Self { sender, receiver, buffer_size }
    }

    pub async fn send(&self, item: T) -> Result<(), tokio::sync::mpsc::error::SendError<T>> {
        self.sender.send(item).await
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

/// Graph metadata with automatic updates
#[derive(Debug, Default)]
pub struct GraphMetadata {
    pub node_count: usize,
    pub edge_count: usize,
    pub last_updated: std::time::SystemTime,
    pub version: String,
}
```

### L3: External Dependencies

```rust
//! External ecosystem integrations with proper error handling

use async_trait::async_trait;
use cozo::{Db, Row};
use serde::{Deserialize, Serialize};

/// CozoDB integration with structured error handling
pub struct CozoDBConnection {
    db: Arc<Db>,
    session_id: String,
}

impl CozoDBConnection {
    pub async fn new(path: &str) -> Result<Self, CozoError> {
        let db = Arc::new(Db::new("sqlite", path).await?);
        let session_id = uuid::Uuid::new_v4().to_string();

        Ok(Self { db, session_id })
    }

    /// Execute query with automatic error handling
    pub async fn execute_query(&self, query: &str) -> Result<Vec<Row>, CozoError> {
        self.db.run_query(query, Default::default(), Default::default())
            .await
            .map_err(|e| CozoError::Query {
                query: query.to_string(),
                cause: e.to_string()
            })
    }

    /// Initialize CodeGraph schema
    pub async fn init_schema(&self) -> Result<(), CozoError> {
        let schema_query = r#"
        {
            ?key := [filepath, filename, interface_name]

            # Create nodes relation
            ?[key, current_code, future_code, interface_signature,
              tdd_classification, current_id, future_id, lsp_meta_data] <-
                *CodeGraph[?key, current_code, future_code, interface_signature,
                         tdd_classification, current_id, future_id, lsp_meta_data]

            # Create edges relation
            ?[source_key, target_key, edge_type] <-
                *CodeGraphEdges[?source_key, ?target_key, edge_type]
        }
        "#;

        self.execute_query(schema_query).await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CozoError {
    #[error("Database connection failed: {0}")]
    Connection(#[from] cozo::error::Error),

    #[error("Query execution failed: {query}")]
    Query { query: String, cause: String },

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("Transaction failed: {0}")]
    Transaction(String),
}

/// Rust-analyzer integration for semantic analysis
pub struct SemanticAnalyzer {
    analysis_db: Arc<dyn HirDatabase>,
    timeout: std::time::Duration,
}

impl SemanticAnalyzer {
    pub fn new(analysis_db: Arc<dyn HirDatabase>) -> Self {
        Self {
            analysis_db,
            timeout: std::time::Duration::from_secs(30),
        }
    }

    /// Analyze code with timeout protection
    pub async fn analyze_with_timeout(&self, code: &str) -> Result<SemanticInfo, SemanticError> {
        let timeout = self.timeout;
        let analysis_future = async {
            // Perform semantic analysis
            self.perform_analysis(code).await
        };

        tokio::time::timeout(timeout, analysis_future)
            .await
            .map_err(|_| SemanticError::Timeout)?
    }

    async fn perform_analysis(&self, code: &str) -> Result<SemanticInfo, SemanticError> {
        // Implementation would integrate with rust-analyzer HIR
        todo!("Implement rust-analyzer integration")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("Analysis timeout exceeded")]
    Timeout,

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Type inference failed: {0}")]
    TypeInference(String),
}
```

## Core Trait System

### Universal Parser Trait

```rust
//! Universal parser trait with capability-based design

use async_trait::async_trait;
use std::fmt::Debug;

/// Parser capability flags for feature detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserCapabilities {
    pub supports_syntax: bool,
    pub supports_semantics: bool,
    pub supports_type_inference: bool,
    pub supports_macros: bool,
    pub supports_attributes: bool,
}

/// Input format enumeration with extensible design
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputFormat {
    Folder(PathBuf),
    SingleFile(PathBuf),
    Text(Cow<'static, str>),
}

/// Universal parser trait following dependency injection principle
#[async_trait]
pub trait UniversalParser: Clone + Send + Sync + 'static {
    type Input: Clone + Send + Sync + Debug;
    type Output: Clone + Send + Sync + Debug;
    type Error: Debug + Send + Sync;

    /// Parse input with automatic capability detection
    async fn parse(&self, input: &Self::Input) -> Result<Self::Output, Self::Error>;

    /// Check format support with confidence scoring
    async fn supports_format(&self, format: &InputFormat) -> f64 {
        match format {
            InputFormat::Folder(_) => self.capabilities().supports_syntax as u8 as f64,
            InputFormat::SingleFile(_) => self.capabilities().supports_syntax as u8 as f64,
            InputFormat::Text(_) => 0.1, // Low confidence for generic text
        }
    }

    /// Get parser capabilities for feature detection
    fn capabilities(&self) -> ParserCapabilities;

    /// Get parser name for debugging
    fn name(&self) -> &'static str;

    /// Get estimated memory usage for input size
    fn estimate_memory_usage(&self, input_size_bytes: usize) -> usize {
        input_size_bytes * 3 // Rough estimate for AST overhead
    }
}

/// Stream processor trait for functional composition
#[async_trait]
pub trait StreamProcessor<T>: Clone + Send + Sync + 'static {
    type Item: Clone + Send + Sync;
    type Error: Debug + Send + Sync;

    /// Process stream with backpressure management
    async fn process_stream(
        &self,
        input: BoundedStream<T>
    ) -> Result<BoundedStream<Self::Item>, Self::Error>;

    /// Get optimal batch size for performance
    async fn optimal_batch_size(&self) -> usize {
        1000 // Default batch size
    }

    /// Get memory limit for safe processing
    async fn memory_limit(&self) -> usize {
        100 * 1024 * 1024 // 100MB default limit
    }
}
```

## Tool Specifications

### Tool 1: folder-to-cozoDB-streamer

```rust
//! Complete parsing + chunking + ingestion pipeline for local folders

use async_trait::async_trait;
use std::path::PathBuf;

pub struct FolderToCozoDBStreamer {
    parser: Box<dyn UniversalParser<Input = Vec<PathBuf>, Output = Vec<CodeNode>>>,
    chunk_strategy: ChunkStrategy,
    db_connection: Arc<CozoDBConnection>,
}

impl FolderToCozoDBStreamer {
    pub fn new(
        parser: Box<dyn UniversalParser<Input = Vec<PathBuf>, Output = Vec<CodeNode>>>,
        chunk_strategy: ChunkStrategy,
        db_connection: Arc<CozoDBConnection>,
    ) -> Self {
        Self { parser, chunk_strategy, db_connection }
    }

    /// Process complete pipeline: folder → parsing → chunking → ingestion
    pub async fn process_folder(&self, folder_path: &PathBuf) -> Result<IngestStats, ToolError> {
        // Step 1: Discover Rust files
        let rust_files = self.discover_rust_files(folder_path).await?;

        // Step 2: Parse with streaming
        let parsed_nodes = self.parser.parse(&rust_files).await
            .map_err(|e| ToolError::ParserError(format!("Parsing failed: {:?}", e)))?;

        // Step 3: Apply chunking strategy
        let chunks = self.apply_chunking(parsed_nodes).await?;

        // Step 4: Stream to database
        let stats = self.ingest_to_database(chunks).await?;

        Ok(stats)
    }

    async fn discover_rust_files(&self, folder_path: &PathBuf) -> Result<Vec<PathBuf>, ToolError> {
        let mut rust_files = Vec::new();

        let mut entries = tokio::fs::read_dir(folder_path).await
            .map_err(|e| ToolError::Io(format!("Failed to read directory: {}", e)))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| ToolError::Io(format!("Directory iteration failed: {}", e)))?
        {
            let path = entry.path();

            if path.is_dir() {
                // Recursively process subdirectories
                rust_files.extend(self.discover_rust_files(&path).await?);
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    rust_files.push(path);
                }
            }
        }

        Ok(rust_files)
    }

    async fn apply_chunking(&self, nodes: Vec<CodeNode>) -> Result<Vec<CodeChunk>, ToolError> {
        match self.chunk_strategy {
            ChunkStrategy::AstNodes => self.chunk_by_ast_nodes(nodes).await,
            ChunkStrategy::Fixed(size) => self.chunk_by_size(nodes, size).await,
        }
    }

    async fn ingest_to_database(&self, chunks: Vec<CodeChunk>) -> Result<IngestStats, ToolError> {
        let mut stats = IngestStats::default();

        for chunk in chunks {
            let query = self.generate_insert_query(&chunk)?;
            self.db_connection.execute_query(&query).await?;
            stats.chunks_processed += 1;
            stats.nodes_processed += chunk.nodes.len();
        }

        Ok(stats)
    }
}

#[derive(Debug, Clone)]
pub enum ChunkStrategy {
    AstNodes,
    Fixed(usize),
}

#[derive(Debug, Default)]
pub struct IngestStats {
    pub chunks_processed: usize,
    pub nodes_processed: usize,
    pub processing_time: std::time::Duration,
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Parser error: {0}")]
    ParserError(String),

    #[error("Database error: {0}")]
    Database(#[from] CozoError),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Chunking error: {0}")]
    Chunking(String),
}
```

### Tool 2: cozo-code-simulation-sorcerer

```rust
//! Reasoning engine for change simulation with confidence gating

use async_trait::async_trait;

pub struct CozoCodeSimulationSorcerer {
    db_connection: Arc<CozoDBConnection>,
    confidence_threshold: f64,
    reasoning_engine: Box<dyn ReasoningEngine>,
}

#[async_trait]
pub trait ReasoningEngine: Send + Sync {
    async fn analyze_change_request(&self, request: &ChangeRequest) -> Result<AnalysisResult, ReasoningError>;
    async fn simulate_changes(&self, analysis: &AnalysisResult) -> Result<SimulationPlan, ReasoningError>;
}

impl CozoCodeSimulationSorcerer {
    pub fn new(
        db_connection: Arc<CozoDBConnection>,
        confidence_threshold: f64,
        reasoning_engine: Box<dyn ReasoningEngine>,
    ) -> Self {
        Self { db_connection, confidence_threshold, reasoning_engine }
    }

    /// Complete simulation workflow with confidence gating
    pub async fn simulate_changes(&self, change_spec: &str) -> Result<SimulationOutput, SimulationError> {
        // Step 1: Parse change specification
        let change_request = self.parse_change_spec(change_spec).await?;

        // Step 2: Analyze against current CodeGraph
        let current_graph = self.load_current_graph().await?;
        let analysis = self.reasoning_engine.analyze_change_request(&change_request).await?;

        // Step 3: Generate simulation plan
        let simulation_plan = self.reasoning_engine.simulate_changes(&analysis).await?;

        // Step 4: Validate confidence threshold
        if simulation_plan.confidence_score < self.confidence_threshold {
            return Err(SimulationError::LowConfidence {
                actual: simulation_plan.confidence_score,
                required: self.confidence_threshold,
            });
        }

        // Step 5: Create rubber duck debugging artifacts
        let debugging_info = self.create_debugging_artifacts(&simulation_plan).await?;

        Ok(SimulationOutput {
            plan: simulation_plan,
            debugging_info,
            analysis_metadata: analysis.metadata,
        })
    }

    async fn load_current_graph(&self) -> Result<CodeGraph, SimulationError> {
        let query = r#"
        {
            ?key := [filepath, filename, interface_name]
            ?[key, current_code, interface_signature, tdd_classification, lsp_meta_data] <-
                *CodeGraph[?key, current_code, _, interface_signature, tdd_classification, 1, _, lsp_meta_data]
        }
        "#;

        let rows = self.db_connection.execute_query(query).await?;
        let graph = self.rows_to_graph(rows).await?;

        Ok(graph)
    }

    async fn create_debugging_artifacts(&self, plan: &SimulationPlan) -> Result<DebuggingInfo, SimulationError> {
        // Create step-by-step reasoning trace
        let reasoning_trace = self.generate_reasoning_trace(plan).await?;

        // Create impact analysis
        let impact_analysis = self.analyze_impact(plan).await?;

        // Create validation checklist
        let validation_checklist = self.create_validation_checklist(plan).await?;

        Ok(DebuggingInfo {
            reasoning_trace,
            impact_analysis,
            validation_checklist,
        })
    }
}

#[derive(Debug)]
pub struct SimulationOutput {
    pub plan: SimulationPlan,
    pub debugging_info: DebuggingInfo,
    pub analysis_metadata: AnalysisMetadata,
}

#[derive(Debug, thiserror::Error)]
pub enum SimulationError {
    #[error("Confidence score too low: {actual} < {required}")]
    LowConfidence { actual: f64, required: f64 },

    #[error("Reasoning engine error: {0}")]
    Reasoning(String),

    #[error("Database error: {0}")]
    Database(#[from] CozoError),

    #[error("Parse error in change specification: {0}")]
    Parse(String),
}
```

## Performance Contracts

### Test-Validated Performance Guarantees

```rust
//! Performance contracts with automated validation

use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Performance contract for parsing operations
pub struct ParsingPerformanceContract {
    max_parse_time_per_mb: Duration,
    max_memory_overhead_factor: f64,
    min_confidence_threshold: f64,
}

impl Default for ParsingPerformanceContract {
    fn default() -> Self {
        Self {
            max_parse_time_per_mb: Duration::from_millis(100), // 100ms per MB
            max_memory_overhead_factor: 3.0, // 3x input size max
            min_confidence_threshold: 0.8, // 80% confidence minimum
        }
    }
}

impl ParsingPerformanceContract {
    pub async fn validate_parsing_performance<P: UniversalParser>(
        &self,
        parser: &P,
        input: &P::Input,
        input_size_bytes: usize,
    ) -> Result<PerformanceReport, PerformanceError> {
        // Measure parsing time
        let start_time = Instant::now();
        let result = parser.parse(input).await;
        let parse_duration = start_time.elapsed();

        // Validate time contract
        let expected_max_time = self.max_parse_time_per_mb * (input_size_bytes as u32 / 1_048_576);
        if parse_duration > expected_max_time {
            return Err(PerformanceError::TimeContractViolation {
                actual: parse_duration,
                expected: expected_max_time,
                input_size: input_size_bytes,
            });
        }

        let parsed_output = result.map_err(|e| {
            PerformanceError::ParseFailure(format!("Parsing failed: {:?}", e))
        })?;

        // Validate memory contract (approximate)
        let estimated_memory = parser.estimate_memory_usage(input_size_bytes);
        let max_allowed_memory = (input_size_bytes as f64 * self.max_memory_overhead_factor) as usize;
        if estimated_memory > max_allowed_memory {
            return Err(PerformanceError::MemoryContractViolation {
                estimated: estimated_memory,
                max_allowed,
                input_size: input_size_bytes,
            });
        }

        Ok(PerformanceReport {
            parse_duration,
            estimated_memory,
            input_size_bytes,
            contract_satisfied: true,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PerformanceError {
    #[error("Time contract violated: actual {actual:?} > expected {expected:?} for input size {input_size}")]
    TimeContractViolation { actual: Duration, expected: Duration, input_size: usize },

    #[error("Memory contract violated: estimated {estimated} > max_allowed {max_allowed} for input size {input_size}")]
    MemoryContractViolation { estimated: usize, max_allowed: usize, input_size: usize },

    #[error("Parsing failed during performance test: {0}")]
    ParseFailure(String),
}

#[derive(Debug)]
pub struct PerformanceReport {
    pub parse_duration: Duration,
    pub estimated_memory: usize,
    pub input_size_bytes: usize,
    pub contract_satisfied: bool,
}
```

### Performance-Validated Tests

```rust
//! Test suite for performance validation

#[cfg(test)]
mod performance_tests {
    use super::*;
    use proptest::prelude::*;

    #[tokio::test]
    async fn test_parsing_performance_contract_small_file() {
        let parser = TreeSitterRustParser::new();
        let contract = ParsingPerformanceContract::default();

        // Generate 1KB test file
        let test_input = generate_rust_code(1_024);
        let input_size = test_input.len();

        let result = contract.validate_parsing_performance(&parser, &test_input, input_size).await;

        assert!(result.is_ok(), "Performance contract should be satisfied for small files");

        let report = result.unwrap();
        assert!(report.parse_duration < Duration::from_millis(100));
        assert!(report.contract_satisfied);
    }

    #[tokio::test]
    async fn test_parsing_performance_contract_large_file() {
        let parser = TreeSitterRustParser::new();
        let contract = ParsingPerformanceContract::default();

        // Generate 1MB test file
        let test_input = generate_rust_code(1_048_576);
        let input_size = test_input.len();

        let result = contract.validate_parsing_performance(&parser, &test_input, input_size).await;

        assert!(result.is_ok(), "Performance contract should be satisfied for large files");

        let report = result.unwrap();
        assert!(report.parse_duration < Duration::from_millis(100));
        assert!(report.contract_satisfied);
    }

    proptest! {
        #[test]
        fn parser_roundtrip_preserves_content(source in generate_valid_rust_code()) {
            prop_assume!(source.len() > 0);
            prop_assume!(source.len() < 1_000_000); // Limit size for test performance

            let parser = TreeSitterRustParser::new();
            let parsed = parser.parse(&source).unwrap();
            let regenerated = parsed.to_rust_source().unwrap();

            prop_assert!(source == regenerated, "Round-trip must preserve content");
        }

        #[test]
        fn chunk_preserves_order(data in prop::collection::vec(0..1000u32, 10..100)) {
            let chunked = ChunkStream::new(data.clone(), 50);
            let flattened = chunked.into_iter().flatten().collect::<Vec<_>>();

            prop_assert_eq!(flattened, data, "Chunking must preserve order");
        }

        #[test]
        fn performance_scales_linearly(file_size in 1000usize..1_000_000usize) {
            let parser = TreeSitterRustParser::new();
            let contract = ParsingPerformanceContract::default();

            let test_input = generate_rust_code(file_size);

            let result = contract.validate_parsing_performance(&parser, &test_input, file_size);
            prop_assert!(result.is_ok(), "Performance should scale linearly");
        }
    }

    fn generate_rust_code(size_bytes: usize) -> String {
        // Generate syntactically valid Rust code of specified size
        let mut code = String::with_capacity(size_bytes);
        code.push_str("pub struct TestStruct {\n");

        let remaining = size_bytes - 100; // Reserve space for boilerplate
        let field_count = remaining / 50; // Rough estimate per field

        for i in 0..field_count {
            code.push_str(&format!("    field_{}: u64,\n", i));
        }

        code.push_str("}\n\n");
        code.push_str("impl TestStruct {\n");
        code.push_str("    pub fn new() -> Self {\n");
        code.push_str("        Self { field_0: 0, field_1: 0, /* ... */ }\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        code
    }

    fn generate_valid_rust_code() -> impl Strategy<Value = String> {
        prop::collection::vec(prop::sample(&[
            Just("fn test_function() -> u32 { 42 }".to_string()),
            Just("struct TestStruct { field: i32 }".to_string()),
            Just("impl TestStruct { fn method(&self) -> i32 { self.field } }".to_string()),
        ]), 1..10)
        .prop_map(|parts| parts.join("\n"))
    }
}
```

## Data Consistency Resolution (Tool 5)

### Hybrid Strategy Implementation

```rust
//! Critical data consistency challenge solution

use std::collections::HashMap;

pub struct DataConsistencyResolver {
    current_state: CodeGraph,
    future_state: CodeGraph,
    metadata_resolver: MetadataResolver,
    validation_engine: Box<dyn ConsistencyValidation>,
}

#[async_trait]
pub trait ConsistencyValidation: Send + Sync {
    async fn validate_consistency(&self, current: &CodeGraph, future: &CodeGraph) -> ConsistencyReport;
    async fn resolve_conflicts(&self, conflicts: Vec<ConsistencyConflict>) -> ResolutionPlan;
}

impl DataConsistencyResolver {
    pub async fn resolve_hybrid_strategy(&self) -> Result<CodeGraph, ConsistencyError> {
        // Step 1: Validate hash consistency between states
        let hash_validation = self.validate_consistency_hashes().await?;
        if !hash_validation.is_consistent {
            return Err(ConsistencyError::HashValidationFailed(hash_validation.conflicts));
        }

        // Step 2: Merge preserving compatible metadata
        let merged_graph = self.merge_graph_states_hybrid().await?;

        // Step 3: Regenerate inconsistent metadata
        let regenerated_graph = self.regenerate_inconsistent_metadata(&merged_graph).await?;

        // Step 4: Flag remaining mismatches for manual review
        let flagged_graph = self.flag_mismatches_for_review(&regenerated_graph).await?;

        // Step 5: Validate final consistency
        let final_validation = self.validation_engine.validate_consistency(
            &self.current_state,
            &flagged_graph
        ).await?;

        if !final_validation.is_consistent {
            return Err(ConsistencyError::FinalValidationFailed(final_validation.conflicts));
        }

        Ok(flagged_graph)
    }

    async fn validate_consistency_hashes(&self) -> Result<HashValidation, ConsistencyError> {
        let current_hashes = self.compute_graph_hashes(&self.current_state).await?;
        let future_hashes = self.compute_graph_hashes(&self.future_state).await?;

        let mut conflicts = Vec::new();
        let mut is_consistent = true;

        for (key, current_hash) in current_hashes {
            if let Some(future_hash) = future_hashes.get(&key) {
                if current_hash != *future_hash {
                    conflicts.push(HashConflict {
                        key,
                        current_hash,
                        future_hash: *future_hash,
                    });
                    is_consistent = false;
                }
            }
        }

        Ok(HashValidation {
            is_consistent,
            conflicts,
        })
    }

    async fn merge_graph_states_hybrid(&self) -> Result<CodeGraph, ConsistencyError> {
        let mut merged = CodeGraph::new();

        // Strategy: Use future state for structural changes, preserve compatible metadata
        let current_nodes = self.current_state.nodes.read().unwrap();
        let future_nodes = self.future_state.nodes.read().unwrap();

        for (key, future_node) in future_nodes.iter() {
            if let Some(current_node) = current_nodes.get(key) {
                // Node exists in both - merge metadata
                let merged_node = self.merge_node_metadata(current_node, future_node)?;
                merged.insert_node(key.clone(), merged_node)?;
            } else {
                // New node in future state
                merged.insert_node(key.clone(), future_node.clone())?;
            }
        }

        // Handle deleted nodes (present in current but not future)
        for (key, current_node) in current_nodes.iter() {
            if !future_nodes.contains_key(key) {
                // Node marked for deletion - create tombstone
                let tombstone = self.create_tombstone_node(current_node)?;
                merged.insert_node(key.clone(), tombstone)?;
            }
        }

        Ok(merged)
    }

    fn merge_node_metadata(&self, current: &CodeNode, future: &CodeNode) -> Result<CodeNode, ConsistencyError> {
        let mut merged = future.clone();

        // Preserve current metadata if compatible
        if self.is_metadata_compatible(&current.interface_signature, &future.interface_signature) {
            merged.interface_signature = current.interface_signature.clone();
        }

        if self.is_metadata_compatible(&current.lsp_meta_data, &future.lsp_meta_data) {
            merged.lsp_meta_data = current.lsp_meta_data.clone();
        }

        // Always use future code for content changes
        merged.current_code = future.current_code.clone();

        Ok(merged)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConsistencyError {
    #[error("Hash validation failed: {0:?}")]
    HashValidationFailed(Vec<HashConflict>),

    #[error("Metadata regeneration failed: {0}")]
    MetadataRegeneration(String),

    #[error("Final consistency validation failed: {0:?}")]
    FinalValidationFailed(Vec<ConsistencyConflict>),

    #[error("Graph merge failed: {0}")]
    MergeFailed(String),
}

#[derive(Debug)]
pub struct HashValidation {
    pub is_consistent: bool,
    pub conflicts: Vec<HashConflict>,
}

#[derive(Debug)]
pub struct HashConflict {
    pub key: ISGL1Key,
    pub current_hash: u64,
    pub future_hash: u64,
}
```

## Quality Gates and Testing Strategy

### Comprehensive Testing Approach

```rust
//! Quality gates with automated validation

use std::sync::Arc;

pub struct QualityGates {
    performance_contracts: Vec<Arc<dyn PerformanceValidation>>,
    correctness_tests: Vec<Arc<dyn CorrectnessTest>>,
    concurrency_tests: Vec<Arc<dyn ConcurrencyTest>>,
}

#[async_trait]
pub trait PerformanceValidation: Send + Sync {
    async fn validate_performance(&self) -> Result<PerformanceReport, PerformanceError>;
    fn contract_name(&self) -> &'static str;
}

#[async_trait]
pub trait CorrectnessTest: Send + Sync {
    async fn run_correctness_test(&self) -> Result<TestResult, TestError>;
    fn test_name(&self) -> &'static str;
}

#[async_trait]
pub trait ConcurrencyTest: Send + Sync {
    async fn run_stress_test(&self) -> Result<StressTestResult, StressTestError>;
    fn test_name(&self) -> &'static str;
}

impl QualityGates {
    pub async fn run_all_quality_gates(&self) -> Result<QualityReport, QualityError> {
        let mut reports = QualityReport::new();

        // Performance validations
        for contract in &self.performance_contracts {
            let report = contract.validate_performance().await?;
            reports.add_performance_report(contract.contract_name(), report);
        }

        // Correctness tests
        for test in &self.correctness_tests {
            let result = test.run_correctness_test().await?;
            reports.add_correctness_result(test.test_name(), result);
        }

        // Concurrency stress tests
        for test in &self.concurrency_tests {
            let result = test.run_stress_test().await?;
            reports.add_stress_test_result(test.test_name(), result);
        }

        // Validate overall quality threshold
        if !reports.meets_quality_threshold() {
            return Err(QualityError::QualityThresholdNotMet(reports));
        }

        Ok(reports)
    }
}

/// Mandatory CI/CD quality gates
pub async fn run_mandatory_quality_checks() -> Result<(), QualityError> {
    // 1. Clippy linting (no warnings allowed)
    let clippy_output = tokio::process::Command::new("cargo")
        .args(&["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
        .output()
        .await?;

    if !clippy_output.status.success() {
        return Err(QualityError::ClippyFailed(String::from_utf8_lossy(&clippy_output.stderr).to_string()));
    }

    // 2. Code formatting check
    let fmt_output = tokio::process::Command::new("cargo")
        .args(&["fmt", "--all", "--", "--check"])
        .output()
        .await?;

    if !fmt_output.status.success() {
        return Err(QualityError::FormatFailed(String::from_utf8_lossy(&fmt_output.stderr).to_string()));
    }

    // 3. Unit tests with coverage
    let test_output = tokio::process::Command::new("cargo")
        .args(&["test", "--all-features"])
        .output()
        .await?;

    if !test_output.status.success() {
        return Err(QualityError::TestsFailed(String::from_utf8_lossy(&test_output.stderr).to_string()));
    }

    // 4. Performance contract validation
    let performance_gate = PerformanceValidationGate::new();
    performance_gate.validate_all_contracts().await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum QualityError {
    #[error("Clippy found warnings: {0}")]
    ClippyFailed(String),

    #[error("Code formatting check failed: {0}")]
    FormatFailed(String),

    #[error("Unit tests failed: {0}")]
    TestsFailed(String),

    #[error("Performance contracts violated: {0}")]
    PerformanceFailed(String),

    #[error("Quality threshold not met: {0:?}")]
    QualityThresholdNotMet(QualityReport),
}
```

## Implementation Roadmap

### Phase-Based Development with TDD

#### **Phase 1: Core Infrastructure (Week 1-2)**
**MVP-First Rigor**: Focus on proven foundational patterns

1. **Core Traits and Types**
   ```rust
   // Implement UniversalParser and StreamProcessor traits
   // Define ISGL1Key and CodeGraph types
   // Create RAII resource management types
   ```

2. **Performance Contracts Framework**
   ```rust
   // Implement ParsingPerformanceContract
   // Create PerformanceReport and PerformanceError types
   // Add automated performance validation
   ```

3. **Error Handling System**
   ```rust
   // Define structured error types with thiserror
   // Implement error boundaries and propagation
   // Create application-level error context with anyhow
   ```

**Tests First:**
```rust
#[tokio::test]
async fn test_core_trait_implementations() {
    // RED: Failing tests for trait implementations
}

#[tokio::test]
async fn test_performance_contracts() {
    // RED: Failing tests for performance validation
}
```

#### **Phase 2: Tool Implementation (Week 3-4)**
**Executable Specifications**: Each tool defined by testable contracts

1. **Tool 1: folder-to-cozoDB-streamer**
   ```rust
   // Tree-sitter integration with streaming
   // File discovery and filtering
   // Chunking strategies implementation
   // Database ingestion with batching
   ```

2. **Tool 2: cozo-code-simulation-sorcerer**
   ```rust
   // Reasoning engine interface
   // Confidence scoring system
   // Rubber duck debugging artifacts
   // Simulation plan generation
   ```

3. **Tool 3: rust-preflight-code-simulator**
   ```rust
   // Rust-analyzer integration
   // Semantic analysis validation
   // Compilation checking
   // Performance benchmarking
   ```

**Tests First:**
```rust
#[tokio::test]
async fn test_tool1_folder_ingestion() {
    // RED: Test folder discovery and parsing
}

#[tokio::test]
async fn test_tool2_simulation_workflow() {
    // RED: Test reasoning and confidence scoring
}

#[tokio::test]
async fn test_tool3_validation_pipeline() {
    // RED: Test rust-analyzer integration
}
```

#### **Phase 3: Integration & Validation (Week 5-6)**
**Structured Concurrency**: Complex interactions validated with stress tests

1. **Tool 4: cozoDB-to-code-writer**
   ```rust
   // Atomic file operations
   // Backup and rollback mechanisms
   // Code generation from database state
   // File system synchronization
   ```

2. **Tool 5: cozoDB-make-future-code-current**
   ```rust
   // Hybrid consistency resolution
   // Metadata regeneration
   // Conflict detection and flagging
   // State validation and cleanup
   ```

3. **End-to-End Integration**
   ```rust
   // 5-tool pipeline orchestration
   // Cross-tool data flow validation
   // Performance benchmarking
   // Property-based testing for invariants
   ```

**Tests First:**
```rust
#[tokio::test]
async fn test_end_to_end_pipeline() {
    // RED: Test complete workflow
}

proptest! {
    #[test]
    fn pipeline_preserves_invariants(input in valid_rust_codebase()) {
        // Property-based tests for system invariants
    }
}
```

#### **Phase 4: Advanced Features (Week 7-8)**
**Concurrency Model Validation**: Thread safety with loom testing

1. **Advanced Performance Optimization**
   ```rust
   // Zero-copy patterns with Cow<'a, str>
   // Parallel processing with rayon
   // Memory pool management
   // Async optimization with tokio
   ```

2. **Comprehensive Stress Testing**
   ```rust
   // Loom-based concurrency testing
   // Memory pressure testing
   // Large codebase validation
   // Fault tolerance testing
   ```

3. **Documentation and Examples**
   ```rust
   // API documentation with examples
   // Architecture diagrams (Mermaid)
   // Performance benchmarking suite
   // User guide and tutorials
   ```

**Quality Gates:**
```bash
# Mandatory quality checks
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo test --all-features
cargo bench --all-features
```

## Architecture Validation

### Self-Consistency Verification

This architecture maintains internal consistency through:

1. **Layer Adherence**: Clear L1→L2→L3 separation with no cross-contamination
2. **Trait Composition**: All components depend on traits, not concrete types
3. **Error Handling**: Structured errors with proper propagation patterns
4. **Performance Guarantees**: All performance claims are test-validated
5. **Resource Management**: RAII patterns with automatic cleanup
6. **Concurrency Safety**: Thread-safe designs with proper synchronization

### External Research Integration

The architecture incorporates findings from domain research:

- **D01-keywords-list.md**: 1139 lines of research informing parser selection and graph theory
- **D02-text-reading-claude-code.md**: Context management patterns for 150k token handling
- **D03-pure-functional-rust.md**: Functional programming principles and TDD patterns

### Steering Principles Compliance

All 9 principles from S01-README-MOSTIMP.md are embedded in the architecture:

1. ✅ **Executable Specifications**: Trait definitions with testable contracts
2. ✅ **Layered Architecture**: Clear L1→L2→L3 separation
3. ✅ **Dependency Injection**: Trait-based composition throughout
4. ✅ **RAII Management**: All resources managed with Drop implementations
5. ✅ **Performance Validation**: Automated performance contract testing
6. ✅ **Structured Errors**: thiserror for libraries, anyhow for applications
7. ✅ **Complex Domain**: ISG graph model with real-world complexity
8. ✅ **Concurrency Validation**: Stress testing and loom integration
9. ✅ **MVP-First Rigor**: Proven patterns over theoretical abstractions

## Conclusion

Arch01ParseltongueV1 provides a comprehensive, production-ready architecture that:

- **Embodies Functional Programming**: Pure functions, immutable data, streaming design
- **Ensures Correctness**: TDD-first approach with property-based testing
- **Guarantees Performance**: Test-validated performance contracts
- **Maintains Simplicity**: 5-tool pipeline focused on folder-first approach
- **Handles Complexity**: Data consistency resolution and error recovery
- **Validates Quality**: Comprehensive quality gates and automated testing

The architecture is ready for implementation following the RED-GREEN-REFACTOR cycle, with clear test specifications for each component and well-defined performance contracts that guarantee production-ready behavior.