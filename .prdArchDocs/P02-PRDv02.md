# Parseltongue PRD v0.2 - Local Folder Code Architecture

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

## Local Folder Architecture

### Core Philosophy

- **Folder-First**: Process local Rust codebases via direct folder input
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

## Parser Architecture

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
}
```

### Parser Implementations

#### **Code Intelligence Parsers**
- **tree-sitter**: Multi-language syntax trees (100+ grammars)
- **syn**: Rust-specific AST with macros, attributes, desugaring
- **rust-analyzer**: Semantic analysis, type inference, LSP bridges

## Independent Crate Specifications

### Crate 1: `folder-to-cozoDB-streamer`

**Purpose**: Process local Rust codebase folders into CozoDB

```bash
folder-to-cozoDB-streamer <FOLDER_PATH> --parsing-library <LIBRARY> --output-db <DATABASE_PATH>

# Required Arguments:
<FOLDER_PATH>               # Local folder path containing Rust code
--parsing-library <LIBRARY> # Parser: tree-sitter, syn, rust-analyzer
--output-db <DATABASE_PATH> # CozoDB database path
```

**Usage Example:**
```bash
# Local folder processing
folder-to-cozoDB-streamer /path/to/rust/repo --parsing-library tree-sitter --output-db ./parseltongue.db
```

**Functional Pattern**: `ingest_folder_to_cozodb(folder_path, parser, database) -> Result<IngestStats>`

### Crate 2: `cozo-code-simulation-sorcerer`

**Purpose**: Simulate code changes based on micro-PRD using CozoDB CodeGraph

```bash
cozo-code-simulation-sorcerer <CHANGE_SPEC> --database <DATABASE_PATH> --confidence-threshold <THRESHOLD>

# Required Arguments:
<CHANGE_SPEC>               # Path to change specification file
--database <DATABASE_PATH>   # CozoDB database path
--confidence-threshold <THRESHOLD> # Confidence threshold (0-100)
```

**Functional Pattern**: `simulate_changes_with_cozodb(spec, database, confidence) -> Result<SimulationPlan>`

### Crate 3: `rust-preflight-code-simulator`

**Purpose**: Validate Rust code using rust-analyzer overlay

```bash
rust-preflight-code-simulator <SIMULATION_OUTPUT> --validation-type <TYPE> --timeout <SECONDS>

# Required Arguments:
<SIMULATION_OUTPUT>          # Path to simulation output from Tool 2
--validation-type <TYPE>     # Type: compile, check-types, check-borrow, all
--timeout <SECONDS>          # Timeout in seconds
```

**Functional Pattern**: `validate_simulated_code(simulation, validation_type, timeout) -> Result<ValidationReport>`

### Crate 4: `cozoDB-to-code-writer`

**Purpose**: Write validated code changes from CozoDB to actual files

```bash
cozoDB-to-code-writer <VALIDATION_OUTPUT> --database <DATABASE_PATH> --backup-dir <PATH>

# Required Arguments:
<VALIDATION_OUTPUT>          # Path to validation output from Tool 3
--database <DATABASE_PATH>   # CozoDB database path
--backup-dir <PATH>          # Backup directory path
```

**Functional Pattern**: `write_cozodb_changes_to_files(validation, database, backup_dir) -> Result<WriteStats>`

### Crate 5: `cozoDB-make-future-code-current`

**Purpose**: Reset CodeGraph state after successful changes (CRITICAL: handles data consistency)

```bash
cozoDB-make-future-code-current <WRITE_OUTPUT> --database <DATABASE_PATH> --source-of-trust <SOURCE> --metadata-strategy <STRATEGY>

# Required Arguments:
<WRITE_OUTPUT>               # Path to write output from Tool 4
--database <DATABASE_PATH>   # CozoDB database path
--source-of-trust <SOURCE>   # Source: files, cozodb, hybrid
--metadata-strategy <STRATEGY> # Strategy: preserve, regenerate, hybrid
```

**Functional Pattern**: `reset_cozodb_to_current_state(write_output, database, source_of_trust, metadata_strategy) -> Result<ResetStats>`

## Critical Data Consistency Challenge (Tool 5)

### The Fundamental Problem

**Tool 5 (`cozoDB-make-future-code-current`)** faces a critical architectural challenge in maintaining data consistency between three different sources of truth:

#### **Three Competing Data Sources:**

1. **CozoDB Future_Code** (Simulation Intent)
   - What the reasoning LLM intended to write
   - Pure, theoretical representation
   - May not match actual filesystem state

2. **Actual Files** (Written by Tool 4)
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

## Minimalistic Workflow Example

### Basic Folder Processing

```bash
# Step 1: Ingest local folder to CozoDB
folder-to-cozoDB-streamer /path/to/rust/repo --parsing-library tree-sitter --output-db ./parseltongue.db

# Step 2: Create change specification
echo "Add async support to UserService" > changes.md

# Step 3: Simulate changes using CozoDB
cozo-code-simulation-sorcerer changes.md --database ./parseltongue.db --confidence-threshold 80

# Step 4: Validate proposed changes
rust-preflight-code-simulator simulation_output.json --validation-type all --timeout 300

# Step 5: Write validated changes to files
cozoDB-to-code-writer validation_output.json --database ./parseltongue.db --backup-dir ./backups

# Step 6: Reset CozoDB state to current
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
- **Local folders**: Direct filesystem access for Rust codebases

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

## Architecture Summary

**5-Tool Pipeline for Local Rust Codebases**:

1. **folder-to-cozoDB-streamer**: Complete parsing + chunking + ingestion pipeline for local folders
2. **cozo-code-simulation-sorcerer**: Reasoning engine for change simulation
3. **rust-preflight-code-simulator**: Validation using rust-analyzer overlay
4. **cozoDB-to-code-writer**: Safe code generation with backups
5. **cozoDB-make-future-code-current**: State management with data consistency

**Key Design Principles**:
- Folder-first approach for simplicity and reliability
- No default values - every argument mandatory
- Streaming architecture for large codebases
- Sophisticated data consistency handling
- Pure functional programming patterns

This architecture provides focused support for local Rust codebase modification while maintaining the reliability-first approach required for complex code changes.

## Backlog Items for Future Research

### Domain Research Backlog

**Remote Repository Processing**
- Git repository cloning and parsing: `https://github.com/user/repo --parsing-library rust-analyzer`
- Rationale: Local folder processing prioritized for immediate delivery
- Research needed: Authentication mechanisms, shallow cloning, large repository optimization

**Gitingest Text Processing**
- Unified text representation: `"https://uithub.com/user/repo/tree/main?accept=text%2Fplain" --parsing-library gitingest`
- Rationale: Niche use case, focus on core functionality first
- Research needed: Protocol handling, text parsing optimization, format validation

**Universal Document Parser**
- Multi-format document conversion: `document.pdf --parsing-library universal`
- PDF → structured TXT with layout analysis
- HTML → TXT with heading hierarchy preservation
- Markdown → structured TXT with section boundaries
- DOCX → TXT with style-based heading detection
- CSV/JSON → structured text representation
- Rationale: Code files are priority, document processing can be added later
- Research needed: Format detection algorithms, layout analysis, section boundary detection

**Advanced Chunking Strategies**
- Document-section chunking for hierarchical content
- Semantic chunking with overlap for better context preservation
- Fixed-size chunking as fallback for edge cases
- Rationale: AST-node chunking covers primary code use cases
- Research needed: Embedding models, semantic similarity algorithms, overlap strategies

### Parser Extensions Backlog

**Gitingest Text Parser Implementation**
```rust
struct GitingestTextParser {
    file_separator: String,  // "--------------------------------------------------------------------------------"
    header_pattern: Regex,    // r"^/path/to/file:$"
}
// Handles unified text format with directory tree, sequential files, line numbering
```

**Universal Document Parser Implementation**
```rust
struct UniversalDocumentParser {
    format_detectors: Vec<Box<dyn FormatDetector>>,
}
// Format conversion pipeline with pluggable detectors
```

### Enhanced Workflow Backlog

**Multi-Input-Type Batch Processing**
```bash
for source in "/path/to/repo1" "https://github.com/user/repo2" "https://uithub.com/user/repo3/tree/main?accept=text%2Fplain"; do
    folder-to-cozoDB-streamer "$source" --parsing-library tree-sitter --output-db ./parseltongue.db
done
```

**Cross-Repository Change Management**
- Batch changes across multiple repositories
- Dependency-aware modification planning
- Impact analysis across repository boundaries

### Multi-Language Support Backlog

**Enhanced Parser Trait**
```rust
#[derive(Debug)]
enum InputFormat {
    Folder(PathBuf),
    GitRepository(String),
    GitingestText(String),
    DocumentFile(PathBuf),
}
```

**Language-Specific Validation**
- Multi-language compilation checking
- Language-specific LSP integration
- Cross-language dependency resolution