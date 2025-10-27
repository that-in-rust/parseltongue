# Parseltongue PRD v0.2 - Universal Code/Document Ingestion Architecture

## Executive Summary

**User Segment**: Developers on large Rust codebases ONLY

**Reliability-First Principle**:
- Optimize for accurate 1-go fixes that feel trustworthy and increase user efficacy
- Prefer CPU-bound static analysis (rust-analyzer overlays, ISG traversals) and small, local, free subagents
- Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible

**Product Philosophy**:
- **Shreyas Doshi**: Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome.
- **Jeff Dean**: Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, RA, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates.

**User Promise**: "When I hit a Rust bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. Speed is a byproduct; correctness is the KPI."

## Universal Ingestion Architecture

### Two Reality-Forms for Content Ingestion

**Form 1: Structured Input** - Traditional folders, git repositories, individual files
**Form 2: Gitingest Text** - Unified text representation of entire codebases

### Core Philosophy

- **Universal Parser Trait**: Single interface for all input types and formats
- **No Default Values**: Every argument must be explicitly specified
- **Pure Functions**: Deterministic input → output transformations
- **Streaming Architecture**: Process data in chunks, never load entire datasets
- **Zero Configuration**: All parameters passed via CLI

### Global Options (All Tools)

```bash
--database <PATH>           # CozoDB database path (required)
--verbose, -v              # Verbose output
--quiet, -q                # Minimal output
--help, -h                 # Show help
--version                  # Show version
```

### Functional Programming Principles

**Pure Functions**: Each tool implements `fn(input) -> Result<output>`
**No Side Effects**: Only intended file/database operations
**Immutable Data**: Input data never modified in-place
**Streaming Architecture**: Process data in chunks, never load entire datasets

## Universal Parser Trait Architecture

### Core Parser Trait (`parse_trait` crate)

```rust
trait Parser: Clone + Send {
    type Input;
    type Output;

    fn parse(&self, input: &Self::Input) -> Result<Self::Output>;
    fn supports_format(&self, format: InputFormat) -> bool;
}

#[derive(Debug)]
enum InputFormat {
    Folder(PathBuf),
    GitRepository(String),
    GitingestText(String),
    DocumentFile(PathBuf),
}
```

### Parser Implementations

#### **Code Intelligence Parsers**
- **tree-sitter**: Multi-language syntax trees (100+ grammars)
- **syn**: Rust-specific AST with macros, attributes, desugaring
- **rust-analyzer**: Semantic analysis, type inference, LSP bridges

#### **Gitingest Text Parser**
```rust
struct GitingestTextParser {
    file_separator: String,  // "--------------------------------------------------------------------------------"
    header_pattern: Regex,    // r"^/path/to/file:$"
}

// Parses unified text format with:
// 1. Directory tree section
// 2. Sequential file content with headers and separators
// 3. Line-numbered content preservation
```

#### **Universal Document Parser**
```rust
struct UniversalDocumentParser {
    format_detectors: Vec<Box<dyn FormatDetector>>,
}

// Handles format conversion:
// PDF → structured TXT with layout analysis
// HTML → TXT with heading hierarchy preservation
// Markdown → structured TXT with section boundaries
// DOCX → TXT with style-based heading detection
// CSV/JSON → structured text representation
```

## Independent Crate Specifications

### Crate 1: `folder-to-cozoDB-streamer`

**Purpose**: Universal ingestion of folders, git repos, gitingest URLs, and documents into CozoDB

```bash
folder-to-cozoDB-streamer <INPUT_SOURCE> --parsing-library <LIBRARY> --output-db <DATABASE_PATH>

# Required Arguments:
<INPUT_SOURCE>              # Input: folder path, git URL, gitingest URL, or document file
--parsing-library <LIBRARY> # Parser: tree-sitter, syn, rust-analyzer, gitingest, universal
--output-db <DATABASE_PATH> # CozoDB database path
```

**Input Types Supported:**
```bash
# Local folder
folder-to-cozoDB-streamer /path/to/repo --parsing-library tree-sitter --output-db ./parseltongue.db

# Git repository
folder-to-cozoDB-streamer https://github.com/user/repo --parsing-library rust-analyzer --output-db ./parseltongue.db

# Gitingest unified text
folder-to-cozoDB-streamer "https://uithub.com/user/repo/tree/main?accept=text%2Fplain" --parsing-library gitingest --output-db ./parseltongue.db

# Document file
folder-to-cozoDB-streamer document.pdf --parsing-library universal --output-db ./parseltongue.db
```

**Functional Pattern**: `ingest_to_cozodb(input_source, parser, database) -> Result<IngestStats>`

### Crate 2: `chunk-to-codegraph-ingest`

**Purpose**: Structure-aware chunking of parsed artifacts into CozoDB CodeGraph

```bash
chunk-to-codegraph-ingest <PARSED_OUTPUT> --chunk-strategy <STRATEGY> --database <DATABASE_PATH>

# Required Arguments:
<PARSED_OUTPUT>              # Output from Tool 1 (parsed artifacts)
--chunk-strategy <STRATEGY>  # Strategy: ast-nodes, document-sections, semantic, fixed-size
--database <DATABASE_PATH>   # CozoDB database path
```

**Chunking Strategies:**
```bash
# AST-node based chunking (code)
chunk-to-codegraph-ingest artifacts.json --chunk-strategy ast-nodes --database ./parseltongue.db

# Document section chunking (hierarchical)
chunk-to-codegraph-ingest artifacts.json --chunk-strategy document-sections --database ./parseltongue.db

# Semantic chunking with overlap
chunk-to-codegraph-ingest artifacts.json --chunk-strategy semantic --database ./parseltongue.db

# Fixed-size chunking (fallback)
chunk-to-codegraph-ingest artifacts.json --chunk-strategy fixed-size --database ./parseltongue.db
```

**Functional Pattern**: `chunk_to_codegraph(artifacts, strategy, database) -> Result<ChunkStats>`

### Crate 3: `cozo-code-simulation-sorcerer`

**Purpose**: Simulate code changes based on micro-PRD using CozoDB CodeGraph

```bash
cozo-code-simulation-sorcerer <CHANGE_SPEC> --database <DATABASE_PATH> --confidence-threshold <THRESHOLD>

# Required Arguments:
<CHANGE_SPEC>               # Path to change specification file
--database <DATABASE_PATH>   # CozoDB database path
--confidence-threshold <THRESHOLD> # Confidence threshold (0-100)
```

**Functional Pattern**: `simulate_changes_with_cozodb(spec, database, confidence) -> Result<SimulationPlan>`

### Crate 4: `rust-preflight-code-simulator`

**Purpose**: Validate Rust code using rust-analyzer overlay

```bash
rust-preflight-code-simulator <SIMULATION_OUTPUT> --validation-type <TYPE> --timeout <SECONDS>

# Required Arguments:
<SIMULATION_OUTPUT>          # Path to simulation output from Tool 3
--validation-type <TYPE>     # Type: compile, check-types, check-borrow, all
--timeout <SECONDS>          # Timeout in seconds
```

**Functional Pattern**: `validate_simulated_code(simulation, validation_type, timeout) -> Result<ValidationReport>`

### Crate 5: `cozoDB-to-code-writer`

**Purpose**: Write validated code changes from CozoDB to actual files

```bash
cozoDB-to-code-writer <VALIDATION_OUTPUT> --database <DATABASE_PATH> --backup-dir <PATH>

# Required Arguments:
<VALIDATION_OUTPUT>          # Path to validation output from Tool 4
--database <DATABASE_PATH>   # CozoDB database path
--backup-dir <PATH>          # Backup directory path
```

**Functional Pattern**: `write_cozodb_changes_to_files(validation, database, backup_dir) -> Result<WriteStats>`

### Crate 6: `cozoDB-make-future-code-current`

**Purpose**: Reset CodeGraph state after successful changes (CRITICAL: handles data consistency)

```bash
cozoDB-make-future-code-current <WRITE_OUTPUT> --database <DATABASE_PATH> --source-of-trust <SOURCE> --metadata-strategy <STRATEGY>

# Required Arguments:
<WRITE_OUTPUT>               # Path to write output from Tool 5
--database <DATABASE_PATH>   # CozoDB database path
--source-of-trust <SOURCE>   # Source: files, cozodb, hybrid
--metadata-strategy <STRATEGY> # Strategy: preserve, regenerate, hybrid
```

**Functional Pattern**: `reset_cozodb_to_current_state(write_output, database, source_of_trust, metadata_strategy) -> Result<ResetStats>`

## Critical Data Consistency Challenge (Tool 6)

### The Fundamental Problem

**Tool 6 (`cozoDB-make-future-code-current`)** faces a critical architectural challenge in maintaining data consistency between three different sources of truth:

#### **Three Competing Data Sources:**

1. **CozoDB Future_Code** (Simulation Intent)
   - What the reasoning LLM intended to write
   - Pure, theoretical representation
   - May not match actual filesystem state

2. **Actual Files** (Written by Tool 5)
   - What `cozoDB-to-code-writer` actually created
   - Real filesystem state
   - May differ from simulation intent due to formatting, edge cases, or write failures

3. **Current CodeGraph Metadata** (Existing CozoDB Data)
   - Rich metadata from multiple parsers
   - `interface_signature` (tree-sitter)
   - `lsp_meta_data` (rust-analyzer)
   - `TDD_Classification` (automated analysis)
   - LLM summaries and other enriched data

#### **CodeGraph Structure Complexity:**

```
ISGL1 (Primary Key) | Current_Code | Future_Code | interface_signature | lsp_meta_data | TDD_Classification | current_id | future_id
```

### Architectural Decision Options

#### **Option 1: Trust CozoDB Future_Code**
```bash
cozoDB-make-future-code-current output.json --database ./parseltongue.db --source-of-trust cozodb --metadata-strategy preserve
```

**Pros:** Maintains CozoDB consistency, preserves rich metadata, fast operation
**Cons:** CozoDB state diverges from filesystem, outdated metadata

#### **Option 2: Trust Files (Re-parse Everything)**
```bash
cozoDB-make-future-code-current output.json --database ./parseltongue.db --source-of-trust files --metadata-strategy regenerate
```

**Pros:** CozoDB matches filesystem exactly, fresh accurate metadata
**Cons:** Computationally expensive, may lose irreplaceable metadata

#### **Option 3: Hybrid Strategy (Recommended)**
```bash
cozoDB-make-future-code-current output.json --database ./parseltongue.db --source-of-trust hybrid --metadata-strategy hybrid
```

**Approach:** Use actual file content, preserve compatible metadata, regenerate inconsistent metadata, flag mismatches

### Recommended Configuration

```bash
# Production-safe configuration
cozoDB-make-future-code-current output.json \
  --database ./parseltongue.db \
  --source-of-trust hybrid \
  --metadata-strategy hybrid \
  --backup-metadata true
```

## Minimalistic Workflow Examples

### Basic Repository Processing

```bash
# Step 1: Ingest folder to CozoDB
folder-to-cozoDB-streamer /path/to/repo --parsing-library tree-sitter --output-db ./parseltongue.db

# Step 2: Chunk into CodeGraph with structure-aware chunking
chunk-to-codegraph-ingest artifacts.json --chunk-strategy ast-nodes --database ./parseltongue.db

# Step 3: Create change specification
echo "Add async support to UserService" > changes.md

# Step 4: Simulate changes using CozoDB
cozo-code-simulation-sorcerer changes.md --database ./parseltongue.db --confidence-threshold 80

# Step 5: Validate proposed changes
rust-preflight-code-simulator simulation_output.json --validation-type all --timeout 300

# Step 6: Write validated changes to files
cozoDB-to-code-writer validation_output.json --database ./parseltongue.db --backup-dir ./backups

# Step 7: Reset CozoDB state to current
cozoDB-make-future-code-current write_output.json --database ./parseltongue.db --source-of-trust hybrid --metadata-strategy hybrid
```

### Gitingest URL Processing

```bash
# Step 1: Ingest gitingest unified text
folder-to-cozoDB-streamer "https://uithub.com/user/repo/tree/main?accept=text%2Fplain" --parsing-library gitingest --output-db ./parseltongue.db

# Step 2: Process with document-section chunking (for unified text)
chunk-to-codegraph-ingest artifacts.json --chunk-strategy document-sections --database ./parseltongue.db

# Continue with simulation workflow...
```

### Document Processing

```bash
# Step 1: Ingest document with universal parser
folder-to-cozoDB-streamer document.pdf --parsing-library universal --output-db ./parseltongue.db

# Step 2: Hierarchical chunking based on document structure
chunk-to-codegraph-ingest artifacts.json --chunk-strategy document-sections --database ./parseltongue.db

# Continue with analysis workflow...
```

### Batch Processing

```bash
# Process multiple repositories with different input types
for source in "/path/to/repo1" "https://github.com/user/repo2" "https://uithub.com/user/repo3/tree/main?accept=text%2Fplain"; do
    folder-to-cozoDB-streamer "$source" --parsing-library tree-sitter --output-db ./parseltongue.db
done

# Process changes across entire codebase
cozo-code-simulation-sorcerer batch_changes.md --database ./parseltongue.db --confidence-threshold 85
rust-preflight-code-simulator simulation_output.json --validation-type all --timeout 600
cozoDB-to-code-writer validation_output.json --database ./parseltongue.db --backup-dir ./batch_backups
cozoDB-make-future-code-current write_output.json --database ./parseltongue.db --source-of-trust hybrid --metadata-strategy hybrid
```

## Performance Characteristics

**Streaming Architecture**:
- Process data in chunks (500-1000 items)
- Never load entire datasets into memory
- Parallel processing where possible
- Lazy evaluation for large inputs

**Memory Efficiency**:
- `Cow<'a, str>` for borrow-or-clone patterns
- Arena allocators for large parsing tasks
- RAII resource management
- Zero-copy operations where possible

**Input Type Support**:
- **Local folders**: Direct filesystem access
- **Git repositories**: Clone and parse repositories
- **Gitingest URLs**: Fetch and parse unified text representations
- **Document files**: Convert PDF/HTML/MD/DOCX/CSV/JSON to structured text

## Error Handling Philosophy

**Pure Error Types**:
```rust
#[derive(Debug, thiserror::Error)]
enum ProcessError {
    #[error("Input source not found: {0}")]
    InputNotFound(String),

    #[error("Parser library '{0}' not supported")]
    UnsupportedParser(String),

    #[error("Chunking strategy '{0}' not available")]
    InvalidChunkStrategy(String),

    #[error("Database operation failed: {0}")]
    DatabaseError(String),
}
```

**Result-Oriented Design**:
- All functions return `Result<T, Error>`
- No panics in production code
- Graceful error propagation
- Clear error messages with context

## Migration from Original Architecture

**New Tool Names vs Original**:
- **folder-to-cozoDB-streamer**: Enhanced to support folders, git, gitingest, documents
- **chunk-to-codegraph-ingest**: NEW - handles structure-aware chunking (was missing)
- **cozo-code-simulation-sorcerer**: Enhanced with mandatory confidence threshold
- **rust-preflight-code-simulator**: Enhanced with mandatory validation type
- **cozoDB-to-code-writer**: Enhanced with mandatory backup directory
- **cozoDB-make-future-code-current**: Enhanced with mandatory consistency strategy

**Key Architectural Improvements**:
- Universal input support (folders, git, gitingest, documents)
- Structure-aware chunking (AST nodes, document sections, semantic)
- No default values - every argument mandatory
- Sophisticated data consistency handling
- Streaming architecture for large codebases

This universal architecture provides comprehensive support for ingesting and processing both traditional structured inputs and modern unified text representations, while maintaining the reliability-first approach required for complex codebase modifications.