# ISGL1 Chunking Algorithm Specification

## Overview

The ISGL1 chunking algorithm converts source code files into structured interface chunks that can be stored in CozoDB and used for LLM reasoning. This specification defines the exact algorithms for language detection, interface boundary identification, and chunk creation using tree-sitter parsing.

## Algorithm Overview

### High-Level Process

```
File Input → Language Detection → Tree-sitter Parsing → Interface Extraction → ISGL1 Key Generation → Chunk Creation → Database Storage
```

### Core Components

1. **Language Detection**: Identify programming language and appropriate parser
2. **Tree-sitter Parsing**: Parse source code into abstract syntax tree
3. **Interface Extraction**: Identify code entities (functions, structs, traits, etc.)
4. **ISGL1 Key Generation**: Create unique identifiers for each entity
5. **Chunk Creation**: Assemble structured data for each entity
6. **Metadata Enrichment**: Add TDD classification and LSP data

## Language Detection

### File Extension Mapping

```rust
fn detect_language(file_path: &Path) -> Option<Language> {
    match file_path.extension()?.to_str()? {
        "rs" => Some(Language::Rust),
        "js" => Some(Language::JavaScript),
        "jsx" => Some(Language::JavaScript), // React JSX
        "ts" => Some(Language::TypeScript),
        "tsx" => Some(Language::TypeScript), // React TSX
        "py" => Some(Language::Python),
        "java" => Some(Language::Java),
        "cpp" | "cc" | "cxx" => Some(Language::Cpp),
        "c" => Some(Language::C),
        "go" => Some(Language::Go),
        "rb" => Some(Language::Ruby),
        "php" => Some(Language::Php),
        "cs" => Some(Language::CSharp),
        "swift" => Some(Language::Swift),
        "kt" => Some(Language::Kotlin),
        "scala" => Some(Language::Scala),
        "rs" => Some(Language::Rust),
        _ => None, // Unsupported language
    }
}
```

### Content-Based Detection (Fallback)

For files without extensions or ambiguous extensions:

```rust
fn detect_language_by_content(content: &str) -> Option<Language> {
    // Look for language-specific patterns
    if content.contains("fn ") && content.contains("->") {
        return Some(Language::Rust);
    }
    if content.contains("function ") && content.contains("=>") {
        return Some(Language::JavaScript);
    }
    if content.contains("def ") && content.contains(":") {
        return Some(Language::Python);
    }
    if content.contains("public class ") {
        return Some(Language::Java);
    }
    // ... additional patterns
    None
}
```

## Tree-sitter Parser Setup

### Parser Initialization

```rust
use tree_sitter::{Language, Parser};

fn get_parser(language: Language) -> Result<Parser, ChunkingError> {
    let mut parser = Parser::new();

    let tree_sitter_lang = match language {
        Language::Rust => tree_sitter_rust::language(),
        Language::JavaScript => tree_sitter_javascript::language(),
        Language::TypeScript => tree_sitter_typescript::language_typescript(),
        Language::Python => tree_sitter_python::language_python(),
        Language::Java => tree_sitter_java::language(),
        Language::Cpp => tree_sitter_cpp::language(),
        // ... other languages
    };

    parser.set_language(tree_sitter_lang)
        .map_err(ChunkingError::ParserInitFailed)?;

    Ok(parser)
}
```

### Parsing Process

```rust
fn parse_file(content: &str, parser: &mut Parser) -> Result<tree_sitter::Tree, ChunkingError> {
    parser.parse(content, None)
        .ok_or(ChunkingError::ParseFailed)
}
```

## Interface Extraction by Language

### Rust Entity Extraction

#### Functions

```rust
fn extract_rust_functions(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_rust::language(),
        r#"
        (function_item
            name: (identifier) @function_name
            parameters: (parameters) @params
            return_type: (type_identifier)? @return_type
            body: (block) @body
        ) @function
        "#
    ).unwrap();

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source.as_bytes());

    for m in matches {
        let function_node = m.captures[0].node;
        let name_node = m.captures[1].node;
        let params_node = m.captures[2].node;
        let return_type_node = m.captures[3].node;

        let chunk = InterfaceChunk {
            entity_type: EntityType::Function,
            name: extract_text(name_node, source),
            signature: extract_function_signature(&function_node, source),
            line_range: LineRange {
                start: function_node.start_position().row + 1,
                end: function_node.end_position().row + 1,
            },
            visibility: extract_visibility(&function_node, source),
            documentation: extract_documentation(&function_node, source),
            // ... other fields
        };

        chunks.push(chunk);
    }

    chunks
}
```

#### Structs

```rust
fn extract_rust_structs(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_rust::language(),
        r#"
        (struct_item
            name: (type_identifier) @struct_name
            fields: (field_declaration_list)? @fields
        ) @struct
        "#
    ).unwrap();

    // ... similar extraction logic for structs
    chunks
}
```

#### Traits

```rust
fn extract_rust_traits(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_rust::language(),
        r#"
        (trait_item
            name: (type_identifier) @trait_name
            body: (declaration_list) @body
        ) @trait
        "#
    ).unwrap();

    // ... similar extraction logic for traits
    chunks
}
```

#### impl Blocks

```rust
fn extract_rust_impls(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_rust::language(),
        r#"
        (impl_item
            type: (type_identifier) @impl_type
            trait: (type_identifier)? @impl_trait
            body: (declaration_list) @body
        ) @impl
        "#
    ).unwrap();

    // ... extraction logic for impl blocks
    chunks
}
```

### JavaScript/TypeScript Entity Extraction

#### Functions

```rust
fn extract_js_functions(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_javascript::language(),
        r#"
        (function_declaration
            name: (identifier) @function_name
            parameters: (formal_parameters) @params
            body: (statement_block) @body
        ) @function

        (variable_declaration
            (variable_declarator
                name: (identifier) @function_name
                value: (arrow_function)
            ) @arrow_function
        ) @function
        "#
    ).unwrap();

    // ... extraction logic for JavaScript functions
    chunks
}
```

#### Classes

```rust
fn extract_js_classes(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_javascript::language(),
        r#"
        (class_declaration
            name: (identifier) @class_name
            body: (class_body) @body
        ) @class
        "#
    ).unwrap();

    // ... extraction logic for JavaScript classes
    chunks
}
```

### Python Entity Extraction

#### Functions

```rust
fn extract_python_functions(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_python::language(),
        r#"
        (function_definition
            name: (identifier) @function_name
            parameters: (parameters) @params
            return_type: (type)? @return_type
            body: (block) @body
        ) @function
        "#
    ).unwrap();

    // ... extraction logic for Python functions
    chunks
}
```

#### Classes

```rust
fn extract_python_classes(tree: &Tree, source: &str) -> Vec<InterfaceChunk> {
    let mut chunks = Vec::new();
    let query = Query::new(
        tree_sitter_python::language(),
        r#"
        (class_definition
            name: (identifier) @class_name
            superclasses: (argument_list)? @superclasses
            body: (block) @body
        ) @class
        "#
    ).unwrap();

    // ... extraction logic for Python classes
    chunks
}
```

## ISGL1 Key Generation

### Key Generation Algorithm

```rust
fn generate_isgl1_key(
    file_path: &Path,
    entity_name: &str,
    entity_type: EntityType
) -> String {
    // 1. Get relative path from repository root
    let relative_path = get_relative_path(file_path);

    // 2. Normalize path separators to forward slashes
    let normalized_path = normalize_path_separators(&relative_path);

    // 3. Extract filename without extension
    let filename = file_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    // 4. Sanitize entity name
    let sanitized_name = sanitize_entity_name(entity_name);

    // 5. Apply entity type prefixes for special cases
    let prefixed_name = apply_entity_type_prefix(sanitized_name, entity_type);

    // 6. Construct ISGL1 key
    format!("{}-{}-{}", normalized_path, filename, prefixed_name)
}
```

### Entity Type Prefixes

```rust
fn apply_entity_type_prefix(name: String, entity_type: EntityType) -> String {
    match entity_type {
        EntityType::ImplBlock { trait_name, struct_name } => {
            if let Some(trait) = trait_name {
                format!("impl_{}_for_{}", trait, struct_name)
            } else {
                format!("impl_{}", struct_name)
            }
        }
        EntityType::Module => format!("mod_{}", name),
        EntityType::Macro => format!("macro_{}", name),
        EntityType::ProcMacro => format!("proc_macro_{}", name),
        EntityType::TestFunction => format!("test_{}", name),
        EntityType::Interface { language } => {
            match language {
                Language::TypeScript => format!("interface_{}", name),
                _ => name,
            }
        }
        _ => name,
    }
}
```

## Chunk Creation Process

### Complete Chunk Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceChunk {
    // Primary identification
    pub isgl1_key: String,
    pub entity_type: EntityType,
    pub name: String,

    // Location information
    pub file_path: PathBuf,
    pub line_range: LineRange,
    pub byte_range: Range<usize>,

    // Signature information
    pub signature: InterfaceSignature,
    pub visibility: Visibility,
    pub documentation: Option<String>,

    // Metadata
    pub module_path: Vec<String>,
    pub attributes: Vec<String>,
    pub generics: Vec<String>,

    // Language-specific data
    pub language: Language,
    pub language_specific: LanguageSpecificData,
}
```

### Interface Signature Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceSignature {
    pub entity_type: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeInfo>,
    pub generics: Vec<GenericParam>,
    pub where_clauses: Vec<WhereClause>,
    pub attributes: Vec<String>,
}
```

### Parameter Extraction

```rust
fn extract_parameters(params_node: Node, source: &str, language: Language) -> Vec<Parameter> {
    match language {
        Language::Rust => extract_rust_parameters(params_node, source),
        Language::JavaScript => extract_js_parameters(params_node, source),
        Language::Python => extract_python_parameters(params_node, source),
        // ... other languages
    }
}
```

## Metadata Enrichment

### TDD Classification

```rust
fn classify_tdd(chunk: &InterfaceChunk, context: &AnalysisContext) -> TDDClassification {
    let complexity = calculate_complexity(chunk);
    let testability = assess_testability(chunk);
    let dependencies = count_dependencies(chunk, context);

    TDDClassification {
        testability,
        complexity,
        dependencies,
        test_coverage_estimate: estimate_test_coverage(chunk),
        critical_path: is_critical_path(chunk, context),
        change_risk: assess_change_risk(chunk),
    }
}
```

### LSP Metadata Extraction (Rust Only)

```rust
fn extract_lsp_metadata(chunk: &InterfaceChunk, file_path: &Path) -> Option<LspMetadata> {
    // Only available for Rust projects with rust-analyzer
    if chunk.language != Language::Rust {
        return None;
    }

    let lsp_client = get_lsp_client()?;

    LspMetadata {
        type_information: lsp_client.get_type_info(&chunk.isgl1_key)?,
        usage_analysis: lsp_client.get_usage_analysis(&chunk.isgl1_key)?,
        semantic_tokens: lsp_client.get_semantic_tokens(&chunk.isgl1_key)?,
    }
}
```

## Dependency Analysis

### Import/Use Statement Extraction

```rust
fn extract_dependencies(chunk: &InterfaceChunk, tree: &Tree, source: &str) -> Vec<Dependency> {
    let mut dependencies = Vec::new();

    match chunk.language {
        Language::Rust => extract_rust_dependencies(tree, source),
        Language::JavaScript => extract_js_dependencies(tree, source),
        Language::Python => extract_python_dependencies(tree, source),
        // ... other languages
    }
}

fn extract_rust_dependencies(tree: &Tree, source: &str) -> Vec<Dependency> {
    let mut dependencies = Vec::new();
    let query = Query::new(
        tree_sitter_rust::language(),
        r#"
        (use_declaration
            path: (scoped_identifier) @import_path
        ) @use_import

        (use_declaration
            path: (use_as_clause
                path: (scoped_identifier) @import_path
            )
        ) @use_as_import
        "#
    ).unwrap();

    // ... extraction logic for Rust dependencies
    dependencies
}
```

## Performance Optimizations

### Chunking Strategy

1. **File-Level Chunking**: Process one file at a time
2. **Lazy Parsing**: Only parse files that have changed
3. **Batch Processing**: Process multiple files in parallel where possible
4. **Memory Management**: Release tree-sitter resources after each file

### Caching Strategy

```rust
struct ChunkingCache {
    parsed_files: HashMap<PathBuf, CachedParseResult>,
    language_detectors: HashMap<String, Language>,
    query_cache: HashMap<Language, Query>,
}

struct CachedParseResult {
    tree: Tree,
    chunks: Vec<InterfaceChunk>,
    file_hash: String,
    last_modified: SystemTime,
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Parse failed for file: {0}")]
    ParseFailed(PathBuf),

    #[error("Parser initialization failed: {0}")]
    ParserInitFailed(String),

    #[error("Query compilation failed: {0}")]
    QueryCompilationFailed(String),

    #[error("File read error: {0}")]
    FileReadError(std::io::Error),

    #[error("Invalid ISGL1 key generated: {0}")]
    InvalidISGL1Key(String),
}
```

### Recovery Strategies

1. **Graceful Degradation**: Continue processing other files if one fails
2. **Partial Results**: Return successfully processed chunks from a file
3. **Error Logging**: Log detailed error information for debugging
4. **Fallback Parsing**: Try alternative parsing methods if tree-sitter fails

## MVP Constraints

Following ultra-minimalist principles for MVP (~10 users):

1. **Limited Language Support**: Rust (full), JavaScript/TypeScript (basic), Python (basic)
2. **Simple Entity Types**: Functions, structs, classes, traits/interfaces only
3. **Basic Interface Extraction**: Essential signature information only
4. **No Advanced Features**: No macro expansion, no template specialization
5. **Single-Pass Processing**: No iterative refinement or cross-file analysis

This ISGL1 chunking specification provides a robust foundation for code parsing and entity extraction while maintaining simplicity and reliability for the MVP target audience.