# D04-MVP-queries: Critical Questions & Implementation Strategy

## Document Overview

This document addresses critical implementation questions for the Parseltongue MVP, focusing on ISGL1 Method chunking granularity, interface boundary detection, edge cases for imperfect codebases, and LSP metadata extraction integration with rust-analyzer.

## Executive Summary

Based on comprehensive analysis of the existing codebase architecture, archived implementations, and current tool status, this document provides actionable answers to MVP development questions with specific implementation recommendations and risk mitigation strategies.

---

## 1. ISGL1 Method Questions

### 1.1 Chunking Granularity Strategy

**Question**: What is the optimal chunking granularity for ISGL1 (Interface Signature Graph Level 1) processing?

**Answer**: **Interface-level granularity** with specific boundary rules:

#### 1.1.1 Primary Chunking Rules

Based on existing implementation in `folder-to-cozoDB-streamer`:
- **ISGL1 Format**: `filepath-filename-InterfaceName`
- **Granularity**: One chunk per interface (function, method, struct, impl block, trait)
- **File Boundaries**: Separate chunks per file entry point (main.rs, lib.rs, mod.rs)

#### 1.1.2 Interface Detection Patterns

```rust
// From existing codebase analysis
interface_types = [
    "fn function_name(...) -> ReturnType",
    "impl StructName { ... }",
    "trait TraitName { ... }",
    "struct StructName { ... }",
    "enum EnumName { ... }",
    "mod module_name { ... }"
]
```

#### 1.1.3 Chunk Size Targets

- **Target**: 50-200 lines per interface chunk
- **Maximum**: 500 lines for large impl blocks
- **Minimum**: 10 lines for simple functions
- **Performance**: <5s processing for 2.1MB codebase dumps

### 1.2 Interface Boundary Detection

**Question**: How do we accurately detect interface boundaries in complex Rust code?

**Answer**: **Hybrid parsing approach** with tree-sitter + syn crate validation:

#### 1.2.1 Primary Detection Method

**Tree-sitter AST Parsing** (Recommended):
- **Error Recovery**: Handles imperfect/syntax-error code
- **Performance**: Fast parsing with good accuracy
- **Robustness**: Continues parsing despite syntax errors
- **Implementation**: Use `tree-sitter-rust` grammar

#### 1.2.2 Secondary Validation Method

**Syn Crate Complement**:
- **Semantic Accuracy**: Precise Rust AST understanding
- **Validation**: Verify tree-sitter results
- **Error Cases**: Handle complex macro expansions
- **Integration**: Use for critical path validation

#### 1.2.3 Boundary Detection Algorithm

```rust
// Pseudocode from existing analysis
detect_interfaces(code) {
    // 1. Parse with tree-sitter (error-tolerant)
    tree_sitter_ast = parse_with_error_recovery(code);
    
    // 2. Extract interface nodes
    interfaces = extract_function_impl_struct_nodes(tree_sitter_ast);
    
    // 3. Validate with syn crate (when possible)
    if code_compiles_cleanly {
        syn_ast = syn::parse_file(code);
        interfaces = validate_with_syn(interfaces, syn_ast);
    }
    
    // 4. Create ISGL1 chunks
    chunks = create_isgl1_chunks(interfaces);
    
    return chunks;
}
```

### 1.3 Edge Cases for Imperfect Codebases

**Question**: How do we handle syntax errors, incomplete code, and malformed files?

**Answer**: **Graceful degradation strategy** with progressive error handling:

#### 1.3.1 Error Handling Hierarchy

**Level 1: Syntax Errors (Most Common)**
- **Strategy**: Continue parsing with tree-sitter error recovery
- **Action**: Flag affected interfaces as `HAS_SYNTAX_ERRORS`
- **Metadata**: Store error information in `lsp_meta_data` field
- **Example**: Missing braces, incomplete function signatures

**Level 2: Compilation Errors**
- **Strategy**: Parse AST but mark as `COMPILE_ERROR`
- **Action**: Extract interface signatures where possible
- **Metadata**: Store compilation error details
- **Example**: Type mismatches, missing imports

**Level 3: Parsing Failures**
- **Strategy**: Fallback to text-based chunking
- **Action**: Use line-based or paragraph-based segmentation
- **Metadata**: Mark as `PARSE_FALLBACK`
- **Example**: Corrupted files, encoding issues

#### 1.3.2 Implementation Strategy

```rust
// From existing codebase patterns
enum ChunkQuality {
    HighQuality,      // Clean parse, full semantic info
    HasSyntaxErrors,  // Tree-sitter recovered, partial info
    HasCompileErrors, // AST ok, semantic issues
    ParseFallback,    // Text-based segmentation only
}

interface_chunk {
    code: String,
    quality: ChunkQuality,
    errors: Vec<String>,
    lsp_meta_data: Option<LspMetadata>,
    is_recoverable: bool,
}
```

#### 1.3.3 Quality Metrics

- **Target**: 85%+ chunks as `HighQuality` in production codebases
- **Minimum**: 60%+ chunks as `HighQuality` for MVP viability
- **Fallback**: All files produce at least `ParseFallback` chunks
- **Performance**: <12ms for incremental file updates

---

## 2. LSP Metadata Extraction Questions

### 2.1 Rust-Analyzer Integration Strategy

**Question**: How do we integrate rust-analyzer for semantic information extraction?

**Answer**: **Two-phase integration** with optional enhancement:

#### 2.1.1 Phase 1: Basic LSP Integration (MVP)

**Implementation Requirements**:
- **rust-analyzer binary**: Use as external process
- **LSP Protocol**: Standard JSON-RPC communication
- **Request Types**: Text document synchronization, semantic tokens, hover
- **Metadata**: Types, dependencies, function signatures, macro expansions

```rust
// Simplified LSP integration pattern
struct RustAnalyzerClient {
    process: Child,
    next_id: u64,
}

impl RustAnalyzerClient {
    async fn initialize(&mut self, workspace_root: &Path) -> Result<(), Error> {
        // Send initialize request
        self.send_request("initialize", json!({
            "rootUri": workspace_root.to_string(),
            "capabilities": {
                "textDocument": {
                    "semanticTokens": true,
                    "hover": true,
                    "completion": true
                }
            }
        })).await?;
        
        // Wait for initialized response
        self.wait_for_response().await?;
        Ok(())
    }
    
    async fn extract_semantic_info(&mut self, file_path: &Path) -> Result<LspMetadata, Error> {
        // Open document
        self.send_notification("textDocument/didOpen", json!({
            "textDocument": {
                "uri": file_path.to_string(),
                "languageId": "rust",
                "version": 1,
                "text": std::fs::read_to_string(file_path)?
            }
        })).await?;
        
        // Request semantic tokens
        let tokens = self.send_request("textDocument/semanticTokens/full", json!({
            "textDocument": {"uri": file_path.to_string()}
        })).await?;
        
        // Request hover information for key positions
        let hover_info = self.extract_hover_information(file_path).await?;
        
        Ok(LspMetadata::from_tokens_and_hover(tokens, hover_info))
    }
}
```

#### 2.1.2 Phase 2: Enhanced Integration (Post-MVP)

**Advanced Features**:
- **Type Inference**: Full type resolution and dependency tracking
- **Call Graph**: Function call relationship analysis
- **Macro Expansion**: Resolved macro code analysis
- **Cross-reference**: Module dependency mapping

### 2.2 LSP Metadata Schema

**Question**: What metadata should we extract and store?

**Answer**: **Structured metadata schema** optimized for query performance:

#### 2.2.1 Core Metadata Fields

```rust
struct LspMetadata {
    // Type information
    type_definitions: Vec<TypeDefinition>,
    function_signatures: Vec<FunctionSignature>,
    trait_implementations: Vec<TraitImplementation>,
    
    // Dependency information
    imports: Vec<Import>,
    module_dependencies: Vec<ModuleDependency>,
    crate_dependencies: Vec<CrateDependency>,
    
    // Semantic information
    variable_types: HashMap<String, TypeInfo>,
    lifetime_annotations: Vec<LifetimeInfo>,
    generic_constraints: Vec<GenericConstraint>,
    
    // Code quality metrics
    complexity_score: f64,
    test_coverage: Option<f64>,
    documentation_coverage: f64,
}

struct TypeDefinition {
    name: String,
    kind: TypeKind, // Struct, Enum, Trait, Function
    generics: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    visibility: Visibility,
    location: Location,
}
```

#### 2.2.2 Storage Strategy

**CozoDB Integration**:
- **JSON Field**: Store `lsp_meta_data` as JSON in CozoDB
- **Indexing**: Create secondary relations for queryable metadata
- **Compression**: Use JSON compression for large metadata objects
- **Caching**: Cache frequently accessed metadata in memory

#### 2.2.3 Query Performance

**Target Performance**:
- **Metadata Extraction**: <500ms per file (average)
- **Storage Overhead**: <50% increase over code-only storage
- **Query Speed**: <100ms for typical context queries
- **Memory Usage**: <200MB for large codebase metadata

---

## 3. Technical Risk Analysis

### 3.1 High-Risk Areas

#### 3.1.1 Tree-sitter Performance Risk

**Risk**: Tree-sitter parsing may be too slow for large codebases
**Probability**: Medium
**Impact**: High
**Mitigation**: 
- Implement chunked streaming parsing
- Use arena allocators for memory efficiency
- Add progress reporting for long operations
- Fallback to syn crate for small files

#### 3.1.2 Rust-Analyzer Integration Complexity

**Risk**: LSP integration may be complex and unstable
**Probability**: High
**Impact**: Medium
**Mitigation**:
- Start with basic LSP features only
- Implement robust error handling and recovery
- Use rust-analyzer as optional enhancement
- Provide text-only fallback mode

#### 3.1.3 Error Recovery Edge Cases

**Risk**: Imperfect code may cause parsing failures
**Probability**: Medium
**Impact**: Medium
**Mitigation**:
- Implement progressive fallback strategies
- Store error information for debugging
- Allow manual intervention for critical cases
- Provide quality metrics and warnings

### 3.2 Performance Risks

#### 3.2.1 Memory Usage

**Risk**: Large codebases may exceed memory limits
**Target**: <1GB RAM for 100k LOC codebase
**Mitigation**:
- Streaming architecture with chunked processing
- Use `Cow<'a, str>` for borrow-or-clone patterns
- Implement memory-efficient data structures
- Add memory monitoring and limits

#### 3.2.2 Processing Speed

**Risk**: Code indexing may be too slow for user experience
**Target**: <30s for 50k LOC initial indexing
**Mitigation**:
- Parallel processing of multiple files
- Incremental updates for file changes
- Caching of parsing results
- Progress reporting and cancellation support

---

## 4. Implementation Recommendations

### 4.1 MVP Priority Order

#### 4.1.1 Phase 1: Core Functionality (Week 1-2)

1. **Tree-sitter Integration**: Basic Rust parsing with error recovery
2. **ISGL1 Chunking**: Interface-level chunk creation
3. **Basic CozoDB Storage**: Code chunks without LSP metadata
4. **Error Handling**: Graceful degradation for parse failures

#### 4.1.2 Phase 2: Enhancement (Week 3-4)

1. **LSP Integration**: Basic rust-analyzer metadata extraction
2. **Quality Metrics**: Parse quality assessment and reporting
3. **Performance Optimization**: Streaming and parallel processing
4. **Testing**: Comprehensive test suite with real codebases

#### 4.1.3 Phase 3: Polish (Week 5-6)

1. **Advanced Features**: Type inference, dependency tracking
2. **Performance Tuning**: Memory optimization and speed improvements
3. **Documentation**: User guides and API documentation
4. **Integration Testing**: End-to-end workflow validation

### 4.2 Development Strategy

#### 4.2.1 Test-Driven Development

**Mandatory Tests for Each Component**:
- **Unit Tests**: Individual function behavior
- **Integration Tests**: Component interaction
- **Performance Tests**: Speed and memory usage
- **Error Tests**: Failure scenarios and recovery

#### 4.2.2 Incremental Delivery

**Weekly Deliverables**:
- **Week 1**: Basic tree-sitter parsing working
- **Week 2**: ISGL1 chunking with CozoDB storage
- **Week 3**: LSP integration basics
- **Week 4**: Error handling and quality metrics
- **Week 5**: Performance optimization
- **Week 6**: Production-ready implementation

### 4.3 Success Criteria

#### 4.3.1 MVP Success Metrics

**Functional Requirements**:
- ✅ Parse 95%+ of real Rust codebases
- ✅ Create meaningful ISGL1 chunks for 80%+ interfaces
- ✅ Store and retrieve code from CozoDB reliably
- ✅ Handle syntax errors gracefully

**Performance Requirements**:
- ✅ <30s indexing time for 50k LOC codebase
- ✅ <1GB RAM usage for large codebases
- ✅ <12ms incremental update time
- ✅ <100ms query response time

**Quality Requirements**:
- ✅ 80%+ test coverage
- ✅ Zero data loss during operations
- ✅ Comprehensive error reporting
- ✅ User-friendly progress indication

---

## 5. Sample Implementation Code

### 5.1 Tree-sitter Interface Detection

```rust
use tree_sitter::{Language, Parser, Node};

extern "C" { fn tree_sitter_rust() -> Language; }

struct InterfaceDetector {
    parser: Parser,
}

impl InterfaceDetector {
    fn new() -> Self {
        let language = unsafe { tree_sitter_rust() };
        let mut parser = Parser::new();
        parser.set_language(language).expect("Error loading Rust grammar");
        Self { parser }
    }
    
    fn detect_interfaces(&mut self, source_code: &str) -> Vec<InterfaceInfo> {
        let tree = self.parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();
        self.extract_interfaces(root_node, source_code)
    }
    
    fn extract_interfaces(&self, node: Node, source: &str) -> Vec<InterfaceInfo> {
        let mut interfaces = Vec::new();
        
        match node.kind() {
            "function_item" => {
                if let Some(interface) = self.parse_function(node, source) {
                    interfaces.push(interface);
                }
            }
            "impl_item" => {
                if let Some(interface) = self.parse_impl(node, source) {
                    interfaces.push(interface);
                }
            }
            "struct_item" => {
                if let Some(interface) = self.parse_struct(node, source) {
                    interfaces.push(interface);
                }
            }
            "trait_item" => {
                if let Some(interface) = self.parse_trait(node, source) {
                    interfaces.push(interface);
                }
            }
            _ => {
                // Recursively check child nodes
                for child in node.children(&mut node.walk()) {
                    interfaces.extend(self.extract_interfaces(child, source));
                }
            }
        }
        
        interfaces
    }
    
    fn parse_function(&self, node: Node, source: &str) -> Option<InterfaceInfo> {
        // Extract function name, parameters, return type
        let name_node = node.child_by_field_name("name")?;
        let name = &source[name_node.byte_range()];
        
        let signature_node = node.child_by_field_name("signature")?;
        let signature = &source[signature_node.byte_range()];
        
        let body_node = node.child_by_field_name("body")?;
        let body = &source[body_node.byte_range()];
        
        Some(InterfaceInfo {
            name: name.to_string(),
            kind: InterfaceKind::Function,
            signature: signature.to_string(),
            body: body.to_string(),
            location: Location::from_node(node),
            quality: ChunkQuality::HighQuality,
        })
    }
}
```

### 5.2 LSP Metadata Extraction

```rust
use serde_json::{json, Value};
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, BufReader};

struct RustAnalyzerWrapper {
    process: Child,
    next_id: u64,
}

impl RustAnalyzerWrapper {
    async fn new(workspace_root: &Path) -> Result<Self, Error> {
        let mut process = Command::new("rust-analyzer")
            .arg("--stdio")
            .current_dir(workspace_root)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
            
        let mut wrapper = Self {
            process,
            next_id: 1,
        };
        
        // Initialize LSP connection
        wrapper.initialize(workspace_root).await?;
        Ok(wrapper)
    }
    
    async fn extract_metadata(&mut self, file_path: &Path) -> Result<LspMetadata, Error> {
        // Open document
        self.send_notification("textDocument/didOpen", json!({
            "textDocument": {
                "uri": file_path.to_string(),
                "languageId": "rust",
                "version": 1,
                "text": tokio::fs::read_to_string(file_path).await?
            }
        })).await?;
        
        // Get semantic tokens
        let tokens = self.request_semantic_tokens(file_path).await?;
        
        // Get hover information for key locations
        let hover_info = self.extract_hover_information(file_path).await?;
        
        // Build metadata
        Ok(LspMetadata::from_lsp_data(tokens, hover_info))
    }
    
    async fn request_semantic_tokens(&mut self, file_path: &Path) -> Result<Value, Error> {
        let response = self.send_request("textDocument/semanticTokens/full", json!({
            "textDocument": {"uri": file_path.to_string()}
        })).await?;
        
        Ok(response)
    }
}
```

### 5.3 Error Recovery Implementation

```rust
#[derive(Debug, Clone)]
enum ParseStrategy {
    TreeSitter,    // Primary: Fast, error-tolerant
    SynCrate,      // Secondary: Accurate, strict
    TextBased,     // Fallback: Line-based chunking
}

struct RobustParser {
    tree_sitter: InterfaceDetector,
    syn_parser: Option<SynParser>,
}

impl RobustParser {
    async fn parse_with_recovery(&self, file_path: &Path) -> Result<Vec<InterfaceChunk>, ParseError> {
        let source_code = tokio::fs::read_to_string(file_path).await?;
        
        // Strategy 1: Try tree-sitter parsing
        match self.tree_sitter.detect_interfaces(&source_code) {
            Ok(interfaces) if !interfaces.is_empty() => {
                return Ok(self.create_chunks(interfaces, ChunkQuality::HighQuality));
            }
            Err(_) => {
                // Continue to next strategy
            }
        }
        
        // Strategy 2: Try syn crate parsing
        if let Some(syn_parser) = &self.syn_parser {
            match syn_parser.parse(&source_code) {
                Ok(interfaces) => {
                    return Ok(self.create_chunks(interfaces, ChunkQuality::HasCompileErrors));
                }
                Err(_) => {
                    // Continue to fallback
                }
            }
        }
        
        // Strategy 3: Fallback to text-based chunking
        let chunks = self.text_based_chunking(&source_code);
        Ok(chunks)
    }
    
    fn text_based_chunking(&self, source: &str) -> Vec<InterfaceChunk> {
        // Split by lines, group by heuristics
        let lines: Vec<&str> = source.lines().collect();
        let mut chunks = Vec::new();
        
        let mut current_chunk = Vec::new();
        let mut current_function = None;
        
        for (line_num, line) in lines.iter().enumerate() {
            // Detect potential function starts
            if line.trim().starts_with("fn ") || 
               line.trim().starts_with("impl ") ||
               line.trim().starts_with("struct ") ||
               line.trim().starts_with("trait ") {
                
                // Save previous chunk if exists
                if !current_chunk.is_empty() {
                    chunks.push(InterfaceChunk {
                        code: current_chunk.join("\n"),
                        quality: ChunkQuality::ParseFallback,
                        interface_name: current_function.unwrap_or_else(|| "unknown".to_string()),
                        line_range: (line_num - current_chunk.len(), line_num),
                    });
                }
                
                current_chunk.clear();
                current_function = self.extract_function_name(line);
            }
            
            current_chunk.push(*line);
        }
        
        // Don't forget the last chunk
        if !current_chunk.is_empty() {
            chunks.push(InterfaceChunk {
                code: current_chunk.join("\n"),
                quality: ChunkQuality::ParseFallback,
                interface_name: current_function.unwrap_or_else(|| "unknown".to_string()),
                line_range: (lines.len() - current_chunk.len(), lines.len()),
            });
        }
        
        chunks
    }
}
```

---

## 6. Conclusion

The D04-MVP-queries analysis provides a clear path forward for implementing the ISGL1 chunking and LSP metadata extraction components of Parseltongue. The key insights are:

1. **Tree-sitter + syn crate hybrid approach** provides the best balance of performance and accuracy
2. **Progressive error recovery** ensures robustness with imperfect codebases
3. **Optional LSP integration** allows for incremental enhancement without blocking MVP
4. **Interface-level granularity** is optimal for targeted code modifications
5. **Performance targets** are achievable with streaming architecture and parallel processing

The implementation recommendations prioritize core functionality first, followed by enhancement and polish phases. This approach ensures MVP delivery while maintaining a clear path to production-ready quality.

**Next Steps**:
1. Begin Phase 1 implementation with tree-sitter integration
2. Set up comprehensive testing framework
3. Implement basic CozoDB storage and retrieval
4. Add error recovery and quality metrics
5. Integrate rust-analyzer for LSP metadata extraction

---

*Document Version: 1.0*  
*Last Updated: 2025-10-29*  
*Author: MVP Development Team*
