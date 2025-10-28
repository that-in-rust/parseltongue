# Parseltongue PRD - Unified Document

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
--project-path <PATH>         # Path to Rust project (required)
--verbose, -v                  # Verbose output
--quiet, -q                    # Minimal output
--help, -h                     # Show help
--version                      # Show version
```

## Command-Summary

### Overview
This section provides a high-level command reference for all Parseltongue tools. For detailed specifications, see Section 02: Individual Tool Specifications.

### Tool Commands Matrix

#### folder-to-cozoDB-streamer
```bash
command-01: folder-to-cozoDB-streamer <FOLDER_PATH> --parsing-library <LIBRARY> --chunking-method <METHOD> --output-db <DATABASE_PATH>
```

#### cozo-to-context-writer
```bash
command-01: cozo-to-context-writer <MICRO_PRD> --database <DATABASE_PATH> --query <COZO_QUERY> --output-context <JSON_FILE>
```

#### rust-preflight-code-simulator
```bash
command-01: rust-preflight-code-simulator <SIMULATION_OUTPUT> --validation-type <TYPE> --timeout <SECONDS>
```

#### cozoDB-to-code-writer
```bash
command-01: cozoDB-to-code-writer <VALIDATION_OUTPUT> --database <DATABASE_PATH> --backup-dir <PATH>
```

#### cozoDB-make-future-code-current
```bash
command-01: cozoDB-make-future-code-current --project-path <PATH>
command-02: cozoDB-make-future-code-current reset --force
command-03: cozoDB-make-future-code-current list-backups
command-04: cozoDB-make-future-code-current stats
command-05: cozoDB-make-future-code-current validate
```

### Global Options (All Tools)
```bash
--project-path <PATH>    # Path to Rust project (required)
--verbose, -v            # Verbose output
--quiet, -q              # Minimal output
--help, -h               # Show help
--version                # Show version
```

## Parseltongue Architecture Overview

### Core Design Philosophy

**External Orchestration + Internal Tools**
- **External Agent**: `agent-parseltongue-reasoning-orchestrator.md` handles complex reasoning and workflow orchestration
- **Internal Tools**: 5 specialized Rust executables handle focused, deterministic operations
- **Clean Separation**: LLM reasoning (external) vs tool execution (internal)
- **Shreyas Doshi Minimalism**: Simple tools, smart orchestration

### Complete System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                 External LLM Agent                         │
│  agent-parseltongue-reasoning-orchestrator.md             │
│  ┌─────────────────┐  ┌─────────────────┐                 │
│  │   Claude Code   │  │  Natural        │                 │
│  │   LLM Reasoning │  │  Language       │                 │
│  │   & Workflow    │◄─┤  Change         │                 │
│  │   Orchestration │  │  Request        │                 │
│  └─────────────────┘  └─────────────────┘                 │
└─────────────────────────────────────────────────────────────┘
                           │ orchestrates
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   5-Tool Pipeline                          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tool 1: folder-to-cozoDB-streamer                  │   │
│  │  ┌─────────────┐    ┌────────────────────────────┐  │   │
│  │  │  Rust Code  │───►│  CozoDB Graph Database     │  │   │
│  │  │  Files      │    │  (CodeGraph + Metadata)    │  │   │
│  │  └─────────────┘    └────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tool 2: cozo-reasoning-writer                      │   │
│  │  ┌─────────────┐    ┌────────────────────────────┐  │   │
│  │  │  Micro-PRD  │───►│  JSON Context for LLM       │  │   │
│  │  │  + Query    │    │  (Current Code State)      │  │   │
│  │  └─────────────┘    └────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tool 3: rust-preflight-code-simulator              │   │
│  │  ┌─────────────┐    ┌────────────────────────────┐  │   │
│  │  │  Proposed   │───►│  Validation Results         │  │   │
│  │  │  Changes    │    │  (Compile/Test/Type)        │  │   │
│  │  └─────────────┘    └────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tool 4: cozoDB-to-code-writer                      │   │
│  │  ┌─────────────┐    ┌────────────────────────────┐  │   │
│  │  │  Validated  │───►│  Modified Files +           │  │   │
│  │  │  Changes    │    │  Backups                    │  │   │
│  │  └─────────────┘    └────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tool 5: cozoDB-make-future-code-current            │   │
│  │  ┌─────────────┐    ┌────────────────────────────┐  │   │
│  │  │  Successful │───►│  Reset Database State +     │  │   │
│  │  │  Changes    │    │  Git Integration            │  │   │
│  │  └─────────────┘    └────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### 5-Phase User Journey

**Phase 1: Project Analysis & Setup**
- Agent validates Rust project structure
- Triggers Tool 1 to index codebase into CozoDB
- Displays codebase statistics and complexity assessment

**Phase 2: Change Specification & Reasoning**
- Agent elicits clear change requirements from user
- Triggers Tool 2 to extract relevant code context as JSON
- Agent performs LLM reasoning on change requirements
- Generates structured change specification with confidence scoring

**Phase 3: Pre-flight Validation**
- Agent triggers Tool 3 to validate proposed changes
- Checks compilation, type safety, borrow checker
- Runs cargo test on simulated changes
- Returns to Phase 2 if validation fails

**Phase 4: File Writing & Testing**
- Agent triggers Tool 4 to write changes with safety checks
- Creates automatic backups, applies changes atomically
- Runs cargo build and cargo test on real codebase
- Returns to Phase 2 if tests fail

**Phase 5: State Reset & Cleanup**
- Agent asks user for satisfaction confirmation
- Triggers Tool 5 to reset database state
- Creates Git commit with generated changes
- Cleans up temporary files and backups

### Tool Integration Details

#### External Orchestration Agent
**File**: `agent-parseltongue-reasoning-orchestrator.md`
**Purpose**: Claude Code agent that orchestrates the complete 5-tool pipeline
**Installation**: Copy to `.claude/agents/` directory
**Capabilities**:
- Natural language change request processing
- 5-phase workflow management with safety gates
- Error recovery and rollback mechanisms
- Git integration and user interaction

#### Individual Tool Specifications

#### Tool 1: folder-to-cozoDB-streamer

**Purpose**: Process local Rust codebase folders into CozoDB

```bash
folder-to-cozoDB-streamer <FOLDER_PATH> --parsing-library <LIBRARY> --chunking-method <METHOD> --output-db <DATABASE_PATH>

# Required Arguments:
<FOLDER_PATH>               # Local folder path containing Rust code
--parsing-library <LIBRARY> # Parser: tree-sitter, txt-parsing
--chunking-method <METHOD>  # Chunking: ISGL1, 300-sentences
--output-db <DATABASE_PATH> # CozoDB database path

# Examples:
folder-to-cozoDB-streamer /path/to/rust/repo --parsing-library tree-sitter --chunking-method ISGL1 --output-db ./parseltongue.db
folder-to-cozoDB-streamer /path/to/rust/repo --parsing-library txt-parsing --chunking-method 300-sentences --output-db ./parseltongue.db
```

**Key Functionality**:
- Recursive file discovery with Rust-specific filtering
- Tree-sitter AST parsing with configurable chunking strategies
- LSP metadata extraction (optional rust-analyzer integration)
- TDD classification (TEST_IMPLEMENTATION vs CODE_IMPLEMENTATION)
- CozoDB ingestion with interface signatures and metadata

**Output Schema**:
```
ISGL1 (Primary Key) | Current_Code | Future_Code | interface_signature | lsp_meta_data | TDD_Classification | current_id | future_id
```

#### Tool 2: cozo-to-context-writer

**Purpose**: Query CozoDB and export JSON context files for external LLM reasoning

```bash
cozo-to-context-writer <MICRO_PRD> --database <DATABASE_PATH> --query <COZO_QUERY> --output-context <JSON_FILE>

# Required Arguments:
<MICRO_PRD>                  # Path to micro-PRD text file
--database <DATABASE_PATH>   # CozoDB database path
--query <COZO_QUERY>         # CozoDB query string (see CozoDbQueryRef.md)
--output-context <JSON_FILE> # Output JSON context file for LLM (default: CodeGraphContext.json)

# Example:
cozo-to-context-writer ./micro-prd.md --database ./parseltongue.db --query "?[interface_signature {...}]" --output-context CodeGraphContext.json
```

**LLM Integration Workflow**:
1. **Query CozoDB**: Extract relevant CodeGraph data using LLM-specified query
2. **Context Export**: Create CodeGraphContext.json with current code state
3. **External Reasoning**: LLM agent reads PRD.md + CodeGraphContext.json + CozoDbQueryRef.md
4. **Change Planning**: LLM performs reasoning and generates structured change specification

**JSON Context Structure**:
```json
{
  "micro_prd": "user change request",
  "current_code": [
    {
      "ISGL1": "filepath-filename-InterfaceName",
      "current_code": "existing implementation",
      "metadata": {...}
    }
  ],
  "relationships": [...],
  "timestamp": "2025-10-28T..."
}
```

#### Tool 3: rust-preflight-code-simulator

**Purpose**: Validate Rust code using rust-analyzer overlay

```bash
rust-preflight-code-simulator <SIMULATION_OUTPUT> --validation-type <TYPE> --timeout <SECONDS>

# Required Arguments:
<SIMULATION_OUTPUT>          # Path to simulation output from Tool 2
--validation-type <TYPE>     # Type: compile, check-types, check-borrow, all
--timeout <SECONDS>          # Timeout in seconds
```

**Validation Types**:
- **compile**: Syntax and basic compilation checking
- **check-types**: Type validation and inference
- **check-borrow**: Borrow checker verification
- **all**: Comprehensive validation (recommended)

#### Tool 4: cozoDB-to-code-writer

**Purpose**: Write validated code changes from CozoDB to actual files

```bash
cozoDB-to-code-writer <VALIDATION_OUTPUT> --database <DATABASE_PATH> --backup-dir <PATH>

# Required Arguments:
<VALIDATION_OUTPUT>          # Path to validation output from Tool 3
--database <DATABASE_PATH>   # CozoDB database path
--backup-dir <PATH>          # Backup directory path
```

**Safety Features**:
- Automatic backup creation before any file changes
- Safety checking (file size, permissions, overwrite protection)
- Atomic write operations with rollback capability
- Detailed operation reporting

#### Tool 5: cozoDB-make-future-code-current (IMPLEMENTED ✅)

**Purpose**: Reset database state and manage metadata backups after successful code changes

**Simplified State Reset Strategy**:
- Re-triggers Tool 1 to re-ingest current file state
- Creates timestamped metadata backups in MD files
- Resets all current/future flags appropriately
- Git-integrated backup tracking

```bash
cozoDB-make-future-code-current --project-path /path/to/rust/repo

# Options:
--project-path <PATH>         # Path to Rust project
--backup-dir <PATH>            # Backup directory (default: .parseltongue/metadata-backups)
--skip-backup                  # Skip metadata backup (not recommended)
--git-integrated               # Enable Git integration
--verbose                      # Detailed output
```

**Commands**:
```bash
cozoDB-make-future-code-current reset --force    # Reset state
cozoDB-make-future-code-current list-backups     # List backups
cozoDB-make-future-code-current stats            # Show database stats
cozoDB-make-future-code-current validate         # Validate project
```

**Backup Structure**:
```
.parseltongue/metadata-backups/2025-10-28-14-30-15/
├── metadata-chunks.md          # Chunk metadata backup
├── metadata-relationships.md   # Relationship metadata backup
├── backup-manifest.md          # Summary of changes
└── .gitkeep                   # Track in Git
```

## Detailed User Journey

### Phase 1: Code Indexing

1. **User Setup**: User arrives at Parseltongue repository
2. **Project Detection**: Ask if user is in relevant Rust repository
3. **Indexing Process**: Trigger Tool 1 for code indexing (≈10 minutes)
4. **Analytics Display**: Show basic CodeGraph statistics

### Phase 2: Change Specification

1. **Micro-PRD Creation**: User describes desired changes in text form
2. **Context Analysis**: LLM analyzes micro-PRD against CodeGraph metadata
3. **Refinement Iterations**: 2-3 iterations to clarify requirements
4. **Context Reset**: Isolate final micro-PRD for processing

### Phase 3: Code Simulation

1. **Tool 2 Execution**: Run cozo-code-simulation-sorcerer
2. **ISG Simulation**: Create future interface rows with proper flags
3. **Code Generation**: Simulate actual code changes
4. **Validation**: Rubber duck debugging and confidence scoring

### Phase 4: Validation

1. **Tool 3 Execution**: Run rust-preflight-code-simulator
2. **Rust Validation**: Compile, type-check, borrow-check analysis
3. **Failure Handling**: Return to Phase 3 if validation fails
4. **Success**: Proceed to file writing

### Phase 5: File Writing

1. **Tool 4 Execution**: Run cozoDB-to-code-writer
2. **Safety Checks**: Backup creation and validation
3. **File Operations**: Atomic writes with rollback protection
4. **Testing**: Run cargo build and cargo test
5. **Failure Handling**: Return to Phase 3 if tests fail

### Phase 6: State Reset (COMPLETED ✅)

1. **User Confirmation**: Ask if user is satisfied with changes
2. **Tool 5 Execution**: Run cozoDB-make-future-code-current
3. **Metadata Backup**: Create timestamped MD file backups
4. **State Reset**: Re-ingest current file state, reset flags
5. **Git Integration**: Commit backups to repository

## Context Management and Performance

### Token Usage Calculation

For typical codebase with 1500 interfaces:
- ISGL1: 4500 tokens
- Interface signatures: 10,500 tokens
- TDD classification: 1,500 tokens
- LSP metadata: 22,500 tokens
- **CodeGraph total**: 37.5k tokens
- Micro-PRD + iterations: 20k tokens
- **Total context**: <100k tokens

### Performance Characteristics

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

## Architecture Principles

### 8 Core Principles

1. **Dependency Injection** for testability
2. **Structured Error Handling** with thiserror patterns
3. **Async/Await** throughout for performance
4. **Trait-Based Design** for modularity
5. **Performance Monitoring** with ValidationPerformanceContract
6. **Configuration-Driven** behavior
7. **Structured Logging** throughout
8. **Safety-First** operations with comprehensive checks

### TDD-First Principles

Following strict RED → GREEN → REFACTOR cycle:
- **RED**: Write failing tests first
- **GREEN**: Implement minimal working solution
- **REFACTOR**: Clean up and optimize

## Backlog Items for Future Research

### Domain Research Backlog

**Text Input Processing**
- Direct text file input processing (moved to backlog)
- **Rationale**: Folder-based approach provides sufficient coverage for immediate needs
- **Research needed**: Performance comparison vs folder-based processing

**Remote Repository Support**
- Git repository cloning and processing
- **Rationale**: Local folder processing is priority for immediate delivery
- **Research needed**: Authentication handling, large repo optimization

**Document Format Support**
- PDF → structured text conversion
- HTML/Markdown → structured text with hierarchy
- DOCX → text with style-based structure
- **Rationale**: Focus on code files first, documents later
- **Research needed**: Format detection, layout analysis algorithms

### Architectural Enhancements Backlog

**Multi-Repository Batch Processing**
- Process multiple codebases in single workflow
- **Rationale**: Single repo focus for MVP
- **Research needed**: Parallel processing, dependency resolution

**Advanced Parser Integration**
- Additional language support beyond Rust
- Custom parser plugins
- **Rationale**: Rust-only focus for initial release
- **Research needed**: Parser abstraction, language-specific AST handling

**Enhanced Validation**
- Multi-language compilation checking
- Integration testing pipeline
- **Rationale**: Rust-specific validation is priority
- **Research needed**: Language-specific toolchains, sandboxing

## Implementation Status

### ✅ **COMPLETED TOOLS**

**Tool 1**: folder-to-cozoDB-streamer
- ✅ Complete with real CozoDB integration
- ✅ Tree-sitter parsing, chunking, metadata extraction
- ✅ Performance optimized streaming architecture

**Tool 2**: cozo-reasoning-writer
- 🟡 **NEEDS REFACTOR** - Current implementation has mock LLM
- ❌ Missing: JSON context export functionality
- ❌ Missing: External LLM integration pattern
- ✅ ISG simulation, confidence scoring (existing)

**Tool 3**: rust-preflight-code-simulator
- 🟡 **PARTIAL** - rust-analyzer integration incomplete
- ✅ Framework and validation contracts ready
- ❌ Missing real rust-analyzer LSP communication

**Tool 4**: cozoDB-to-code-writer
- ✅ Complete with comprehensive safety features
- ✅ Backup management, atomic operations
- ✅ Performance monitoring and reporting

**Tool 5**: cozoDB-make-future-code-current
- ✅ **COMPLETE** - Simplified state reset implemented
- ✅ Git-integrated metadata backup system
- ✅ CLI interface with comprehensive commands
- ✅ Real CozoDB operations and state management

### 🔄 **IN PROGRESS**

- Tool 3 rust-analyzer completion
- End-to-end pipeline testing
- Performance optimization

### ❌ **BLOCKING ISSUES**

- Tool 2: Mock LLM → JSON context writer refactor (NEEDS ARCHITECTURAL CHANGE)
- Tool 3: rust-analyzer API compatibility
- CozoDB API resolution in some components

## Success Metrics

### Phase Completion Criteria

- [ ] All 5 tools fully functional
- [ ] End-to-end pipeline working on real Rust codebases
- [ ] Performance within defined contracts
- [ ] Comprehensive test coverage
- [ ] Complete documentation

### Project Success Criteria

- [ ] Single-pass, safe, minimal diff generation
- [ ] Real Rust codebase analysis capability
- [ ] Sub-100k token context management
- [ ] Git-integrated workflow
- [ ] Zero data loss during state resets

## Conclusion

The Parseltongue project implements a sophisticated 5-tool pipeline for automated Rust code modification with strong emphasis on reliability, correctness, and data safety. The simplified Tool 5 implementation provides a practical foundation that can be enhanced based on real-world usage patterns.

The architecture successfully balances complexity with practicality, leveraging existing tools (Tree-sitter, rust-analyzer, CozoDB) while maintaining clean separation of concerns and comprehensive error handling.

**Next Priority**: Refactor Tool 2 from mock LLM to JSON context writer, then complete Tool 3 rust-analyzer integration.

## Installation & Usage

### Quick Start

1. **Build Tools**:
   ```bash
   cargo build --release --workspace
   export PATH="$PWD/target/release:$PATH"
   ```

2. **Setup Agent**:
   ```bash
   cp agent-parseltongue-reasoning-orchestrator.md ~/.claude/agents/
   ```

3. **Use in Claude Code**:
   ```
   @agent-parseltongue-reasoning-orchestrator Add async support to database layer
   ```

### Architecture Benefits

- **Simple Tools**: Each tool has a single, focused responsibility
- **Smart Orchestration**: Complex reasoning handled by external LLM agent
- **Safety First**: Multiple validation gates prevent breaking changes
- **Git Integration**: Automatic commits and rollback capability
- **Extensible**: Easy to add new tools or modify orchestration logic

### User Experience

The orchestrator provides a conversational interface for complex code changes:

```
🔍 Analyzing Rust codebase...
📊 Found 1,247 interfaces across 89 files
📝 Processing change request: "Add async support to database layer"
🧠 Reasoning about change impact...
✅ Changes applied successfully!
```

---

*Last Updated: 2025-10-28*
*Version: Unified PRD v1.0*