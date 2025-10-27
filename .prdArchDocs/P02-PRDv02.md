# Parseltongue PRD v0.2 - Detailed CLI Architecture

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

## Unified CLI Architecture

### Main Binary Structure

Instead of separate binaries, we use a unified `parseltongue` CLI following `cargo`/`git` patterns:

```bash
parseltongue <SUBCOMMAND> [OPTIONS] [ARGS]
```

### Global Options (available on all subcommands)

```bash
--config <PATH>              # Path to config file (default: ~/.parseltongue/config.toml)
--database <URL>             # CozoDB connection string (default: sqlite://parseltongue.db)
-v, --verbose                # Increase verbosity (use multiple times: -v, -vv, -vvv)
-q, --quiet                  # Suppress output except errors
--no-progress                # Disable progress bars for automated environments
--output-format <FORMAT>     # Output format: json, yaml, table, human (default: human)
--log-file <PATH>            # Write logs to file (default: none)
-h, --help                   # Show help
--version                    # Show version
```

### Configuration File Support

```toml
# ~/.parseltongue/config.toml
[database]
url = "sqlite://parseltongue.db"
timeout = 30  # seconds

[parsing]
default_granularity = "isgl1"
chunk_size = 1000
parallel_workers = 4
lsp_enabled = true

[simulation]
max_iterations = 3
context_limit = 100000  # tokens
timeout = 300  # seconds

[validation]
rust_toolchain = "stable"
check_format = true
run_tests = true
```

## Detailed CLI Specifications

### Tool 1: `parseltongue index` - Repository Indexing

**Purpose**: Parse and chunk Rust codebases, extract metadata, store in CozoDB

```bash
parseltongue index [OPTIONS] <REPOSITORY_PATH>

# Required Arguments:
<REPOSITORY_PATH>            # Path to Rust repository to index

# Options:
--granularity <LEVEL>        # Chunk granularity: isgl1, isgl2, function, module, crate
                            # (default: isgl1)
--parser <BACKEND>           # Parser backend: tree-sitter, syn, rust-analyzer
                            # (default: tree-sitter)
--lsp-enabled                # Enable rust-analyzer metadata extraction (default: true)
--lsp-path <PATH>            # Path to rust-analyzer executable
--include-tests              # Include test files in indexing (default: true)
--include-examples           # Include example files (default: false)
--include-targets <TARGETS>  # Specific targets to include (comma-separated)
--exclude-patterns <PATTERNS> # File patterns to exclude (glob syntax)
--parallel-jobs <N>          # Number of parallel parsing jobs (default: CPU count)
--chunk-size <N>             # Maximum lines per chunk (default: 1000)
--force-reindex              # Reindex even if already indexed
--dry-run                    # Show what would be indexed without processing
--batch-size <N>             # Batch size for database writes (default: 1000)
```

**Examples**:
```bash
# Index current repository with default settings
parseltongue index .

# Index specific repository with ISGL2 granularity
parseltongue index /path/to/repo --granularity isgl2

# Index with rust-analyzer metadata and custom chunk size
parseltongue index . --lsp-enabled --chunk-size 500 --verbose

# Dry run to see what would be processed
parseltongue index . --dry-run --output-format json
```

### Tool 2: `parseltongue ingest` - Graph Database Ingestion

**Purpose**: Create and populate CodeGraph from indexed chunks

```bash
parseltongue ingest [OPTIONS] <INPUT_SOURCE>

# Required Arguments:
<INPUT_SOURCE>               # Source: database path, JSON file, or directory of chunks

# Options:
--source-format <FORMAT>     # Input format: json, yaml, csv, database
                            # (default: database)
--graph-name <NAME>          # CodeGraph identifier (default: "default")
--create-if-missing          # Create database if it doesn't exist (default: true)
--append                     # Append to existing graph (default: false, replaces)
--validate-schema            # Validate data against schema (default: true)
--batch-size <N>             # Batch size for inserts (default: 1000)
--skip-duplicates            # Skip duplicate entries (default: true)
--update-existing            # Update existing ISGL1 entries (default: false)
--index-current-only         # Only mark entries as current (default: true)
```

**Examples**:
```bash
# Ingest from default database
parseltongue ingest sqlite://parseltongue.db

# Ingest from JSON file with validation
parseltongue ingest chunks.json --validate-schema --verbose

# Create new graph with custom name
parseltongue ingest . --graph-name "project_v2" --create-if-missing
```

### Tool 3: `parseltongue simulate` - Code Simulation

**Purpose**: Simulate code changes based on micro-PRD using reasoning LLM

```bash
parseltongue simulate [OPTIONS] <PRD_FILE>

# Required Arguments:
<PRD_FILE>                   # Path to micro-PRD file (markdown or text)

# Options:
--context-limit <TOKENS>     # Maximum context tokens (default: 100000)
--max-iterations <N>         # Maximum simulation iterations (default: 3)
--blast-radius <LEVEL>       # ISG traversal depth: 1, 2, 3, unlimited
--focus-area <PATTERNS>      # File/module patterns to focus on (comma-separated)
--exclude-area <PATTERNS>    # File/module patterns to exclude
--idiomatic-rust-docs <PATH> # Path to TDD idiomatic Rust documentation
--confidence-threshold <N>   # Minimum confidence to proceed (0-100, default: 80)
--interactive                # Interactive mode for unclear decisions
--save-context <PATH>        # Save reasoning context to file
--load-context <PATH>        # Load reasoning context from file
--simulation-type <TYPE>     # Type: full, test-only, code-only (default: full)
```

**Examples**:
```bash
# Simulate with default settings
parseltongue simulate micro_prd.md

# Interactive simulation with larger context
parseltongue simulate prd.md --interactive --context-limit 150000 --verbose

# Focus simulation on specific modules
parseltongue simulate prd.md --focus-area "src/**/*" --exclude-area "tests/**"

# Save and resume simulation context
parseltongue simulate prd.md --save-context session.json
parseltongue simulate prd.md --load-context session.json --max-iterations 5
```

### Tool 4: `parseltongue validate` - Rust Preflight Validation

**Purpose**: Validate proposed changes using rust-analyzer and cargo

```bash
parseltongue validate [OPTIONS] <CHANGES_SOURCE>

# Required Arguments:
<CHANGES_SOURCE>             # Source: simulation output, diff file, or changeset

# Options:
--toolchain <TOOLCHAIN>      # Rust toolchain: stable, beta, nightly (default: stable)
--features <FEATURES>        # Cargo features to enable (comma-separated)
--all-features               # Enable all cargo features
--no-default-features        # Disable default features
--check-format               # Run rustfmt formatting check (default: true)
--run-clippy                 # Run clippy lints (default: true)
--run-tests                  # Run cargo test (default: true)
--test-threads <N>           # Number of test threads (default: CPU count)
--allow-failing-tests        # Continue validation despite test failures (default: false)
--timeout <SECONDS>          # Per-operation timeout (default: 300)
--workspace                  # Validate entire workspace (default: current package)
--target <TARGET>            # Specific target to validate
--examples                   # Validate examples (default: false)
--benches                    # Validate benchmarks (default: false)
```

**Examples**:
```bash
# Validate simulation changes
parseltongue validate simulation_output.json

# Validate with all checks and features
parseltongue validate changes.diff --check-format --run-clippy --run-tests --all-features

# Validate specific target with custom toolchain
parseltongue validate changes.json --toolchain nightly --target "my_binary"

# Quick validation without tests
parseltongue validate changes.json --no-tests --check-format
```

### Tool 5: `parseltongue apply` - Apply Code Changes

**Purpose**: Write validated changes to code files and run verification

```bash
parseltongue apply [OPTIONS] <CHANGES_SOURCE>

# Required Arguments:
<CHANGES_SOURCE>             # Source: validated changeset or simulation output

# Options:
--backup                     # Create backup before applying changes (default: true)
--backup-dir <DIR>           # Backup directory (default: .parseltongue/backups)
--dry-run                    # Show changes without applying (default: false)
--force                      # Apply changes even with warnings (default: false)
--verify-after               # Run cargo build/test after applying (default: true)
--parallel-jobs <N>          # Number of parallel file operations (default: 4)
--create-dirs                # Create parent directories as needed (default: true)
--preserve-permissions       # Preserve file permissions (default: true)
--diff-format <FORMAT>       # Diff output format: unified, context, minimal
--interactive                # Interactive confirmation for each change
--rollback-on-failure        # Rollback changes if verification fails (default: true)
```

**Examples**:
```bash
# Apply changes with backup and verification
parseltongue apply validated_changes.json

# Dry run to preview changes
parseltongue apply changes.json --dry-run --diff-format unified

# Apply changes interactively
parseltongue apply changes.json --interactive --backup-dir ./my_backups

# Force apply with automatic rollback on failure
parseltongue apply changes.json --force --rollback-on-failure --verify-after
```

### Tool 6: `parseltongue commit` - Clean Slate Protocol

**Purpose**: Commit changes and reset CodeGraph state

```bash
parseltongue commit [OPTIONS] [MESSAGE]

# Optional Arguments:
[MESSAGE]                    # Commit message (if not provided, opens editor)

# Options:
--reset-graph                # Reset CodeGraph to current state (default: true)
--graph-name <NAME>          # Specific graph to reset (default: "default")
--amend                      # Amend previous commit (default: false)
--allow-empty                # Allow empty commits (default: false)
--sign                       # GPG sign commit (default: false)
--no-verify                  # Skip pre-commit hooks (default: false)
--backup-graph <PATH>        # Backup graph state before reset
--tag <TAG>                  # Create tag after commit
--push                       # Push to remote after commit (default: false)
--remote <REMOTE>            # Remote to push to (default: "origin")
--branch <BRANCH>            # Branch to push to (default: current branch)
```

**Examples**:
```bash
# Commit with automatic message and graph reset
parseltongue commit "Implement feature X with tests"

# Commit with custom message and tag
parseltongue commit "Fix critical bug in module Y" --tag "v1.2.3"

# Amend previous commit and push
parseltongue commit --amend --push

# Commit with graph backup
parseltongue commit "Major refactor" --backup-graph ./graph_backup.json
```

### Tool 7: `parseltongue status` - System Status and Analytics

**Purpose**: Show repository status, graph analytics, and system information

```bash
parseltongue status [OPTIONS]

# Options:
--format <FORMAT>            # Output format: table, json, yaml, markdown
--detailed                   # Show detailed statistics (default: false)
--graph-stats                # Show CodeGraph statistics (default: true)
--recent-changes             # Show recent changes (default: false)
--health-check               # Run system health check (default: false)
--performance                # Show performance metrics (default: false)
--last-simulation            # Show last simulation results (default: false)
--pending-changes            # Show pending uncommitted changes
```

**Examples**:
```bash
# Show basic status
parseltongue status

# Detailed status in JSON format
parseltongue status --detailed --format json

# Show recent changes and performance
parseltongue status --recent-changes --performance

# Full health check
parseltongue status --health-check --detailed
```

## Configuration and Environment Variables

### Environment Variable Support

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

This detailed CLI architecture provides a comprehensive, developer-friendly interface that follows Rust ecosystem best practices while maintaining the reliability-first approach required for complex codebase modifications.