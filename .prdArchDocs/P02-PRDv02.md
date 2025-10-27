# Parseltongue PRD v0.2 - Minimalistic CLI Architecture

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

## Minimalistic CLI Architecture

### Core Philosophy

- **Independent Crates**: Each tool is a standalone binary with 4-word naming
- **Mandatory Arguments Only**: No optional configuration files
- **Pure Functions**: Deterministic input → output transformations
- **Zero Configuration**: All parameters passed via CLI
- **Simple Options**: Only essential flags, no complex configuration

### Global Options (All Tools)

```bash
--db <path>          # Database directory (default: ./parseltongue.db)
--verbose, -v        # Verbose output
--quiet, -q          # Minimal output
--help, -h           # Show help
--version            # Show version
```

### Functional Programming Principles

**Pure Functions**: Each tool implements `fn(input) -> Result<output>`
**No Side Effects**: Only intended file/database operations
**Immutable Data**: Input data never modified in-place
**Streaming Architecture**: Process data in chunks, never load entire datasets

## Independent Crate Specifications

### Crate 1: `folder-to-cozoDB-streamer`

**Purpose**: Stream folder contents to CozoDB using pure functional parsing

```bash
folder-to-cozoDB-streamer [OPTIONS] <FOLDER_PATH>

# Required Arguments:
<FOLDER_PATH>                # Path to folder to process

# Options:
--parsing-library tree-sitter # Parser: tree-sitter, syn, rust-analyzer
--granularity isgl1          # Chunk granularity: isgl1, isgl2, function, module
--include-tests              # Include test files (default: true)
--exclude-patterns "tests/**" # Exclude patterns
--batch-size 500             # Database batch size (default: 500)
--workers auto               # Worker count (default: CPU cores)
--output-db <PATH>           # CozoDB database path (required)
```

**Functional Pattern**: `stream_folder_to_cozodb(path, db) -> Result<StreamStats>`

### Crate 2: `txt-to-cozoDB-streamer`

**Purpose**: Stream text file contents to CozoDB

```bash
txt-to-cozoDB-streamer [OPTIONS] <FILE_PATH>

# Required Arguments:
<FILE_PATH>                  # Path to text file

# Options:
--encoding utf-8             # File encoding (default: utf-8)
--chunk-size 1000            # Processing chunk size (default: 1000)
--format auto                # Auto-detect format (default: auto)
--include-metadata           # Include file metadata (default: false)
--output-db <PATH>           # CozoDB database path (required)
```

**Functional Pattern**: `stream_text_to_cozodb(path, db) -> Result<TextStats>`

### Crate 3: `cozo-code-simulation-sorcerer`

**Purpose**: Simulate code changes based on micro-PRD using CozoDB data

```bash
cozo-code-simulation-sorcerer [OPTIONS] <CHANGE_SPEC>

# Required Arguments:
<CHANGE_SPEC>                # Path to change specification file
--database <PATH>            # CozoDB database path (required)

# Options:
--dry-run                    # Preview changes only (default: false)
--impact-analysis true        # Show impact analysis (default: true)
--confidence-threshold 80    # Confidence threshold (default: 80)
--max-iterations 3           # Maximum simulation iterations (default: 3)
--context-limit 100000       # Context token limit (default: 100000)
```

**Functional Pattern**: `simulate_changes_with_cozodb(spec, db) -> Result<SimulationPlan>`

### Crate 4: `rust-preflight-code-simulator`

**Purpose**: Validate Rust code using rust-analyzer overlay

```bash
rust-preflight-code-simulator [OPTIONS] <SIMULATION_OUTPUT>

# Required Arguments:
<SIMULATION_OUTPUT>          # Path to simulation output from Tool 3

# Options:
--check-types                # Type checking only (default: false)
--check-borrow               # Borrow checking only (default: false)
--compile                    # Full compilation check (default: true)
--target debug               # Build target (default: debug)
--timeout 300                # Timeout seconds (default: 300)
```

**Functional Pattern**: `validate_simulated_code(simulation) -> Result<ValidationReport>`

### Crate 5: `cozoDB-to-code-writer`

**Purpose**: Write validated code changes from CozoDB to actual files

```bash
cozoDB-to-code-writer [OPTIONS] <VALIDATION_OUTPUT>

# Required Arguments:
<VALIDATION_OUTPUT>          # Path to validation output from Tool 4
--database <PATH>            # CozoDB database path (required)

# Options:
--backup-dir <PATH>          # Backup directory (default: ./backups)
--dry-run false              # Preview only (default: false)
--parallel-jobs 4            # Parallel file operations (default: 4)
--verify-after true          # Run cargo build/test after apply (default: true)
```

**Functional Pattern**: `write_cozodb_changes_to_files(validation, db) -> Result<WriteStats>`

### Crate 6: `cozoDB-make-future-code-current`

**Purpose**: Reset CodeGraph state after successful changes (CRITICAL: handles data consistency)

```bash
cozoDB-make-future-code-current [OPTIONS] <WRITE_OUTPUT>

# Required Arguments:
<WRITE_OUTPUT>               # Path to write output from Tool 5
--database <PATH>            # CozoDB database path (required)

# Options:
--source-of-trust files      # Data source: files|cozodb|reparse (default: files)
--metadata-strategy preserve # Metadata handling: preserve|regenerate|hybrid (default: preserve)
--commit-message ""          # Auto commit message (optional)
--verify-consistency true    # Verify data consistency (default: true)
```

**Functional Pattern**: `reset_cozodb_to_current_state(write_output, db) -> Result<ResetStats>`

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

```bash
# Database configuration
PARSELTOngUE_DATABASE_URL="sqlite://parseltongue.db"
PARSELTOngUE_DB_TIMEOUT=30

# Authentication (if using remote CozoDB)
PARSELTOngUE_DB_USER="username"
PARSELTOngue_DB_PASSWORD="password"

# LLM Configuration
ANTHROPIC_API_KEY="your-key-here"
PARSELTOngue_DEFAULT_MODEL="claude-3-5-sonnet-20241022"
PARSELTOngue_MAX_TOKENS=100000

# Rust Configuration
RUST_TOOLCHAIN="stable"
RUST_LOG="info"

# System Configuration
PARSELTOngue_CONFIG_DIR="/path/to/config"
PARSELTOngue_DATA_DIR="/path/to/data"
PARSELTOngue_LOG_LEVEL="info"
```

#### **Option 1: Trust CozoDB Future_Code**
```bash
cozoDB-make-future-code-current --source-of-trust cozodb --metadata-strategy preserve
```
**Approach:**
- Use `Future_Code` from CozoDB as new `Current_Code`
- Preserve all existing metadata
- Ignore actual filesystem differences

**Pros:**
- Maintains data consistency within CozoDB
- Preserves rich metadata
- Fast operation (no re-parsing)

**Cons:**
- CozoDB state diverges from filesystem
- Future operations based on outdated metadata
- Risk of database vs reality mismatch

#### **Option 2: Trust Files (Re-parse Everything)**
```bash
cozoDB-make-future-code-current --source-of-trust files --metadata-strategy regenerate
```
**Approach:**
- Read actual files written by Tool 5
- Re-parse with all tools (tree-sitter, rust-analyzer, LLM)
- Regenerate all metadata from scratch

**Pros:**
- CozoDB matches filesystem exactly
- Fresh, accurate metadata
- No data drift over time

**Cons:**
- Computationally expensive
- May lose metadata that can't be regenerated
- Risk of regeneration failures

#### **Option 3: Hybrid Strategy (Recommended)**
```bash
cozoDB-make-future-code-current --source-of-trust hybrid --metadata-strategy hybrid
```
**Approach:**
- Use actual file content as new `Current_Code`
- Preserve metadata that's still valid
- Regenerate metadata that's inconsistent
- Flag and handle mismatches

### Recommended Configuration

```bash
# Production-safe configuration
cozoDB-make-future-code-current \
  --source-of-trust hybrid \
  --metadata-strategy hybrid \
  --verify-consistency true \
  --backup-metadata true \
  --on-mismatch prompt \
  --on-metadata-fallback preserve \
  --consistency-report detailed
```

This approach balances data integrity with practical considerations, ensuring CozoDB remains the authoritative source while acknowledging the complexity of maintaining consistency with real-world filesystem operations.

### Shell Completion

```bash
# Generate completion scripts
parseltongue completions bash > /etc/bash_completion.d/parseltongue
parseltongue completions zsh > /usr/local/share/zsh/site-functions/_parseltongue
parseltongue completions fish > ~/.config/fish/completions/parseltongue.fish

# Install system-wide (requires sudo)
parseltongue completions install --shell bash
```

## Output Formats and Integration

### JSON Output Structure

```json
{
  "command": "simulate",
  "timestamp": "2024-10-27T14:53:00Z",
  "success": true,
  "data": {
    "changes_planned": 42,
    "confidence_score": 92,
    "simulation_iterations": 2
  },
  "metadata": {
    "duration_ms": 15420,
    "tokens_used": 87350,
    "graph_nodes_processed": 1500
  }
}
```

### Pipe Integration Support

```bash
# Chain commands through pipes
parseltongue simulate prd.md --format json | \
parseltongue validate --format json | \
parseltongue apply --interactive

# Use with other tools
parseltongue status --format json | jq '.data.graph_stats.total_nodes'

# Integration with CI/CD
parseltongue simulate prd.md && \
parseltongue validate simulation.json && \
parseltongue apply simulation.json
```

## Detailed User Journey v0.8

### Initial Setup

```bash
# User clones parseltongue repository
git clone https://github.com/that-in-rust/parseltongue.git
cd parseltongue

# Installation
cargo install --path .
parseltongue --help
```

### Repository Analysis Workflow

```bash
# User navigates to target repository
cd /path/to/their/rust/project

# Step 1: Index the repository
parseltongue index . --granularity isgl1 --verbose
# Output: Indexing completed. Processed 1,247 interfaces in 3 minutes.

# Step 2: Review indexing status
parseltongue status --graph-stats
# Output: CodeGraph contains 1,247 nodes, 8,932 edges

# Step 3: Create micro-PRD
cat > feature_request.md << EOF
I need to add async support to the UserService module.
- Convert database operations to async
- Add proper error handling with thiserror
- Include comprehensive tests
- Maintain backward compatibility
EOF

# Step 4: Simulate changes
parseltongue simulate feature_request.md --interactive
# Output: Simulation complete. Planning 15 changes with 94% confidence.

# Step 5: Validate changes
parseltongue validate simulation_output.json --run-tests
# Output: Validation passed. All tests compile and pass.

# Step 6: Apply changes
parseltongue apply validated_changes.json --backup
# Output: Applied 15 changes. Backup created.

# Step 7: Verify and commit
cargo test  # User runs their own verification
parseltongue commit "Add async support to UserService" --tag "v2.1.0"
```

### Advanced Usage Scenarios

#### Iterative Development

```bash
# Focus on specific modules
parseltongue simulate prd.md --focus-area "src/services/**" --exclude-area "tests/**"

# Use larger context for complex changes
parseltongue simulate complex_refactor.md --context-limit 200000 --max-iterations 5

# Save and resume sessions
parseltongue simulate prd.md --save-context session_v1.json
# Later...
parseltongue simulate prd.md --load-context session_v1.json --max-iterations 3
```

#### CI/CD Integration

```bash
#!/bin/bash
# ci_check.sh

set -e

echo "Running Parseltongue validation..."
parseltongue simulate prd.md --quiet
parseltongue validate simulation.json --quiet
parseltongue apply simulation.json --dry-run --quiet

echo "All checks passed!"
```

#### Batch Operations

```bash
# Process multiple repositories
for repo in $(cat repos.txt); do
    echo "Processing $repo..."
    cd "$repo"
    parseltongue index . --quiet
    parseltongue simulate ../global_prd.md --quiet
    cd -
done
```

## Error Handling and Diagnostics

### Common Error Scenarios

```bash
# Missing repository
parseltongue index /nonexistent/path
# Error: Repository path does not exist: /nonexistent/path

# Database connection failure
parseltongue simulate prd.md --database invalid://url
# Error: Cannot connect to database: invalid://url

# Context overflow
parseltongue simulate large_prd.md --context-limit 1000
# Error: Context limit exceeded. Increase --context-limit or reduce PRD scope.

# Validation failures
parseltongue validate changes.json --run-tests
# Error: Validation failed: 3 tests failed. Use --verbose for details.
```

### Diagnostic Commands

```bash
# Health check
parseltongue status --health-check --detailed

# Verify database integrity
parseltongue validate --database-only

# Check tool versions
parseltongue --version
# Output: parseltongue 0.2.0 (rustc 1.75.0, CozoDB 0.7.0)
```

## Migration Path from v0.1

### Breaking Changes

1. **Unified CLI**: Separate binaries replaced by `parseltongue <subcommand>` structure
2. **Configuration**: New TOML-based configuration system
3. **Output Formats**: Consistent structured output across all commands

### Migration Commands

```bash
# Old command structure
folder-to-cozoDB-streamer --input . --output db.sqlite
ingest-chunks-to-codegraph --input db.sqlite

# New unified structure
parseltongue index . --database sqlite://db.sqlite
parseltongue ingest sqlite://db.sqlite
```

## Functional Programming Architecture

### Core Traits (Pure Functional)

#### Parser Trait (`pure_parse` crate)
```rust
trait Parser: Clone + Send {
    type Input;
    type Output;

    fn parse(&self, input: &Self::Input) -> Result<Self::Output>;
}

// Implementation: Immutable input -> parsed output
// No side effects, no global state, pure functions only
```

#### Chunker Trait (`fold_chunk` crate)
```rust
trait Chunker: Clone + Send {
    type Input;
    type Output;

    fn chunk(&self, input: &Self::Input) -> Vec<Self::Output>;
}

// Implementation: Fold parsed data into immutable chunks
// Monoidal reduction, associative, parallelizable
```

### Minimalistic Workflow Examples

#### Basic Repository Processing
```bash
# Step 1: Stream folder to CozoDB
folder-to-cozoDB-streamer . --output-db ./parseltongue.db --granularity isgl1

# Step 2: Create change specification
echo "Add async support to UserService" > changes.md

# Step 3: Simulate changes using CozoDB
cozo-code-simulation-sorcerer changes.md --database ./parseltongue.db

# Step 4: Validate proposed changes
rust-preflight-code-simulator simulation_output.json

# Step 5: Write validated changes to files
cozoDB-to-code-writer validation_output.json --database ./parseltongue.db

# Step 6: Reset CozoDB state to current
cozoDB-make-future-code-current write_output.json --database ./parseltongue.db
```

#### Text Document Processing
```bash
# Stream text document to CozoDB
txt-to-cozoDB-streamer document.md --output-db ./parseltongue.db --encoding utf-8

# Process document-based changes
cozo-code-simulation-sorcerer doc_changes.md --database ./parseltongue.db

# Validate and apply changes
rust-preflight-code-simulator simulation_output.json
cozoDB-to-code-writer validation_output.json --database ./parseltongue.db
cozoDB-make-future-code-current write_output.json --database ./parseltongue.db
```

### Performance Characteristics

**Streaming Architecture**:
- Process data in chunks (default 500-1000 items)
- Never load entire datasets into memory
- Parallel processing where possible
- Lazy evaluation for large inputs

**Memory Efficiency**:
- Immutable data structures with `Cow<'a, str>` for borrow-or-clone
- Arena allocators for large parsing tasks
- Automatic memory cleanup via RAII
- Zero-copy operations where possible

### Migration from v0.1

**Old Separate Binaries**:
```bash
folder-to-cozoDB-streamer --input . --output db.sqlite
txt-to-cozoDB-streamer --file doc.txt --output db.sqlite
cozo-code-simulation-sorcerer --spec changes.md
run-rust-preflight-code-simulator --code proposal.rs
cozoDB-to-code-writer --template Template --output file.rs
cozoDB-make-future-code-current --template Template --target src/
```

**New Independent Crates**:
```bash
code-folder-to-stream . --batch-size 500
text-file-to-stream doc.txt --encoding utf-8
code-change-to-simulate changes.md --target Interface
rust-code-to-validate proposal.rs --compile
code-templates-to-write Template --output file.rs
code-templates-to-apply Template --target src/
```

This minimalistic architecture provides pure functional interfaces with mandatory arguments only, focusing on streaming data processing and deterministic transformations suitable for large-scale Rust codebase analysis and modification.