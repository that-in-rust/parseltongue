# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**Parseltongue** is a Rust-based code analysis toolkit implementing a 6-tool pipeline for LLM-driven code understanding and modification. The project follows **ultra-minimalist MVP principles** with **TDD-first development** (RED → GREEN → REFACTOR) and emphasizes executable specifications over traditional documentation.

**Current Status**: 15% Complete | 3/6 Tools Functional
- ✅ Tool 1 (parseltongue-01): folder-to-cozoDB-streamer
- ✅ Tool 2 (parseltongue-02): LLM-to-cozoDB-writer
- 🟡 Tool 3 (parseltongue-03): LLM-cozoDB-to-context-writer (1 failing test)
- ❌ Tool 4-6: Not yet implemented

---

## Commands

### Building and Testing

```bash
# Build entire workspace
cargo build --workspace

# Build specific tool
cargo build --package parseltongue-01

# Run all tests
cargo test --workspace

# Run tests for specific tool
cargo test --package parseltongue-01

# Run single test
cargo test --package parseltongue-01 --test test_name

# Check code without building
cargo check --workspace

# Clean build artifacts
cargo clean
```

### Development Commands

```bash
# Format code
cargo fmt --all

# Lint with clippy
cargo clippy --workspace -- -D warnings

# Generate documentation
cargo doc --workspace --open

# Run specific tool binary
cargo run --package parseltongue-01 -- --dir ./test --verbose
cargo run --package parseltongue-02 -- --help
cargo run --package parseltongue-03 -- --help
```

### Performance Testing

```bash
# Run benchmarks (when available)
cargo bench --package parseltongue-01

# Run with performance profiling
cargo build --release
RUST_BACKTRACE=1 cargo test --release
```

---

## Architecture

### The 6-Tool Pipeline

Parseltongue operates as a sequential pipeline where each tool has a single, well-defined responsibility:

```
Codebase → [Tool 1] → CozoDB → [Tool 2] → CozoDB → [Tool 3] → Context.json
                                             ↓
                                    [Tool 4: Validate]
                                             ↓
                                    [Tool 5: Write Files]
                                             ↓
                                    [Tool 6: State Reset]
```

**Tool Responsibilities**:

1. **parseltongue-01** (folder-to-cozoDB-streamer): Parse codebase with tree-sitter, extract interface signatures (ISGL1 keys), store in CozoDB. Performance target: <30s for 50k LOC.

2. **parseltongue-02** (LLM-to-cozoDB-writer): Receive LLM reasoning about code changes, update CozoDB with temporal versioning (current_ind, future_ind, Future_Action). Supports Create/Edit/Delete operations.

3. **parseltongue-03** (LLM-cozoDB-to-context-writer): Generate CodeGraphContext.json from CozoDB for LLM consumption. Enforces <100k token limit, excludes Current_Code for size optimization.

4. **parseltongue-04** (rust-preflight-code-simulator): Validate proposed changes through syntax → build → test pipeline. Rust-first with graceful degradation for other languages. **NOT YET IMPLEMENTED**.

5. **parseltongue-05** (LLM-cozoDB-to-code-writer): Write Future_Code to actual files. Ultra-minimalist: NO backups, direct file operations. **NOT YET IMPLEMENTED**.

6. **parseltongue-06** (cozoDB-make-future-code-current): Reset temporal state by deleting CodeGraph table and re-indexing. Ultra-minimalist: NO backup metadata. **NOT YET IMPLEMENTED**.

### Core Data Model

**CodeGraph Table Schema** (CozoDB):
```
isgl1_key           : String (primary key) - Format: "filepath-filename-InterfaceName"
current_code        : String? - Current version of code entity
future_code         : String? - Proposed version after LLM edits
interface_signature : String - Extracted function/struct/trait signature
tdd_classification  : String - Classification: Test, CodeImplementation, Specification
lsp_meta_data       : Json? - Optional LSP metadata from rust-analyzer
current_ind         : Bool - True if entity exists in current codebase
future_ind          : Bool - True if entity will exist after changes
future_action       : String? - "Create", "Edit", or "Delete"
```

**Temporal State Transitions**:
- (1,1,null) → Unchanged entity
- (1,1,Edit) → Modification pending
- (1,0,Delete) → Deletion pending
- (0,1,Create) → Creation pending

### Workspace Structure

```
parseltongue/
├── Cargo.toml              # Workspace configuration with shared dependencies
├── crates/
│   ├── parseltongue-core/  # Shared types, traits, CozoDB storage layer
│   ├── parseltongue-01/    # Tool 1: Indexing
│   ├── parseltongue-02/    # Tool 2: LLM writing
│   ├── parseltongue-03/    # Tool 3: Context generation
│   ├── parseltongue-04/    # Tool 4: Validation (NOT IMPLEMENTED)
│   ├── parseltongue-05/    # Tool 5: File writing (NOT IMPLEMENTED)
│   └── parseltongue-06/    # Tool 6: State reset (NOT IMPLEMENTED)
├── .steeringDocs/          # Architecture principles and design docs
└── TDD-Tracker.md          # Implementation status and task tracking
```

**Key Files**:
- `/crates/parseltongue-core/src/storage/`: Real CozoDB integration (completed Oct 29, 2025)
- `/crates/parseltongue-core/src/entities.rs`: CodeEntity type definitions
- `/crates/parseltongue-core/src/temporal.rs`: Temporal versioning logic
- `/.steeringDocs/S01-README-MOSTIMP.md`: Core development philosophy
- `/.steeringDocs/S02-code-conventions.md`: Idiomatic Rust patterns (12 layers)
- `/.steeringDocs/S06-design101-tdd-architecture-principles.md`: TDD architecture
- `/TDD-Tracker.md`: Detailed implementation status and ULTRATHINK plan

---

## Development Philosophy

### 1. TDD-First: RED → GREEN → REFACTOR

**Always write tests first**:
```rust
// RED: Write failing test
#[tokio::test]
async fn test_feature_name() {
    let system = create_test_system().await;
    let result = system.new_feature().await;
    assert!(result.is_ok()); // This will fail initially
}

// GREEN: Minimal implementation to pass
pub async fn new_feature(&self) -> Result<Output> {
    Ok(Output::default()) // Simplest thing that works
}

// REFACTOR: Improve with idiomatic patterns
pub async fn new_feature(&self) -> Result<Output> {
    // Use functional patterns, proper error handling, etc.
}
```

### 2. Executable Specifications

Replace ambiguous user stories with testable contracts:

```rust
/// Message creation with deduplication contract
///
/// # Preconditions
/// - Valid entity with ISGL1 key
/// - Future_Code contains valid syntax
///
/// # Postconditions
/// - Returns Ok(()) on success
/// - Entity persisted to CozoDB
/// - Temporal state updated correctly
///
/// # Error Conditions
/// - StorageError::InvalidKey if ISGL1 key malformed
/// - StorageError::Database on persistence failure
pub async fn insert_entity(&self, entity: &CodeEntity) -> Result<()>;
```

### 3. Ultra-Minimalist Constraints

- **NO backup options** in file writing (Tool 5)
- **NO configuration complexity** - single reliable operations
- **NO multiple safety levels** - direct operations
- Focus: 10 users, simplicity over features

### 4. Layered Rust Architecture

**L1 (Core)**: Ownership, lifetimes, Result/Option, newtype pattern
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Isgl1Key(String); // Newtype for type safety
```

**L2 (Standard Library)**: Collections, iterators, Arc/Mutex
```rust
use std::sync::Arc;
use std::collections::HashMap;
```

**L3 (External)**: Async/await (tokio), CozoDB, tree-sitter
```rust
use tokio::sync::RwLock;
use cozo::DbInstance;
```

### 5. Performance Contracts

Every performance claim must have an automated test:

```rust
#[tokio::test]
async fn test_indexing_performance_contract() {
    let test_dir = create_50k_loc_test_repo().await;

    let start = Instant::now();
    let result = indexer.index_directory(&test_dir).await.unwrap();
    let elapsed = start.elapsed();

    // Performance contract: <30s for 50k LOC
    assert!(elapsed < Duration::from_secs(30),
            "Indexing took {:?}, expected <30s", elapsed);
}
```

### 6. Idiomatic Rust Patterns

**Follow the 12 layers from S02-code-conventions.md**:
- Use `thiserror` for library errors, `anyhow` for application context
- Accept `&str` and `&[T]`, store `String` and `Vec<T>`, return owned types
- Use `Arc<Mutex<T>>` for shared mutable state across threads
- Implement `Drop` for resource cleanup (RAII)
- Use `?` operator for error propagation
- Prefer iterator chains over explicit loops

---

## Testing Strategy

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests: Individual function correctness
    #[test]
    fn test_isgl1_key_parsing() { }

    // Integration tests: Component interaction
    #[tokio::test]
    async fn test_storage_integration() { }

    // Performance tests: Validate contracts
    #[tokio::test]
    async fn test_performance_contract() { }
}
```

### Test Requirements

1. **Every public function must have tests**
2. **Performance-critical paths need benchmark tests**
3. **Error paths must be tested** (not just happy path)
4. **Integration tests for database operations**
5. **Concurrency tests for shared state** (when applicable)

### Current Test Status

```bash
# As of Oct 29, 2025:
# Total: 34 tests
# Passing: 33
# Failing: 1 (parseltongue-03 binary integration test)

# Run failing test:
cargo test --package parseltongue-03 --bin parseltongue-03 -- --nocapture
```

---

## Common Development Tasks

### Adding a New Feature

1. **Check TDD-Tracker.md** for current implementation status
2. **Write test first** (RED phase)
3. **Implement minimal solution** (GREEN phase)
4. **Refactor with idiomatic patterns** (REFACTOR phase)
5. **Run full test suite**: `cargo test --workspace`
6. **Update TDD-Tracker.md** with progress

### Implementing Missing Tools (4, 5, 6)

**Before starting**:
- Read `/TDD-Tracker.md` sections for the specific tool
- Review architecture in `/.steeringDocs/`
- Follow established patterns from Tools 1-3

**Pattern to follow**:
```rust
// 1. Create crate structure
// crates/parseltongue-04/
// ├── Cargo.toml
// ├── src/
// │   ├── lib.rs
// │   ├── main.rs
// │   ├── cli.rs
// │   └── errors.rs
// └── tests/

// 2. Define traits in parseltongue-core if needed
// 3. Write failing tests
// 4. Implement with functional patterns
// 5. Integration test with CozoDB
```

### Working with CozoDB

**Storage layer location**: `/crates/parseltongue-core/src/storage/`

```rust
use parseltongue_core::storage::CozoDbStorage;

// Initialize storage
let storage = CozoDbStorage::new("mem").await?; // or "sqlite:path.db"

// Insert entity
storage.insert_entity(&entity).await?;

// Query by key
let entity = storage.get_entity("isgl1-key").await?;

// Update temporal state
storage.update_temporal_state("isgl1-key", TemporalUpdate {
    future_ind: false,
    future_action: Some(FutureAction::Delete),
}).await?;
```

### Debugging Performance Issues

```bash
# Build with release optimizations
cargo build --release --package parseltongue-01

# Run with timing
time cargo run --release --package parseltongue-01 -- --dir ./large-repo

# Profile with flamegraph (if installed)
cargo flamegraph --package parseltongue-01 -- --dir ./test
```

---

## Code Style Guidelines

### Error Handling

```rust
// ✅ Library code: Use thiserror
#[derive(Error, Debug)]
pub enum ParseltongueError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Invalid ISGL1 key: {key}")]
    InvalidKey { key: String },
}

// ✅ Application code: Use anyhow
pub async fn run_tool(&self) -> anyhow::Result<()> {
    let data = fetch_data()
        .await
        .with_context(|| "Failed to fetch data")?;
    Ok(())
}
```

### Async Patterns

```rust
// ✅ Use tokio runtime features
use tokio::time::{timeout, Duration};

pub async fn fetch_with_timeout(url: &str) -> Result<Data> {
    timeout(
        Duration::from_secs(30),
        fetch_data(url)
    ).await??
}

// ✅ Use JoinSet for concurrent tasks
use tokio::task::JoinSet;

let mut tasks = JoinSet::new();
for item in items {
    tasks.spawn(async move { process_item(item).await });
}
```

### Naming Conventions

- Crates: `parseltongue-NN` (where NN is tool number)
- Modules: snake_case (`temporal_writer`, `llm_client`)
- Types: PascalCase (`CodeEntity`, `Isgl1Key`)
- Functions: snake_case (`insert_entity`, `get_changed_entities`)
- Constants: SCREAMING_SNAKE_CASE (`DEFAULT_TOKEN_LIMIT`)

---

## Important Constraints

### Ultra-Minimalist Principles

From `.steeringDocs/S01-README-MOSTIMP.md`:

1. **NO backup options** - Tools 5 & 6 must not create backup files
2. **NO configuration complexity** - Single reliable operations only
3. **NO multiple safety levels** - Direct operations, trust the LLM
4. **Target: 10 users** - Simplicity over feature richness
5. **One thing well** - Each tool has exactly one responsibility

### Performance Targets

- **Indexing** (Tool 1): <30s for 50k LOC
- **Context Generation** (Tool 3): <100k tokens, <500ms generation
- **Query Performance**: <500μs for typical graph queries
- **Memory Usage**: <1GB for typical codebases

### Database Constraints

- **Backend**: CozoDB with SQLite storage (or in-memory for tests)
- **Schema**: Fixed CodeGraph table structure (see Architecture section)
- **No migrations**: Schema is immutable for MVP
- **Temporal state**: Never modify (current_ind, future_ind, Future_Action) outside Tool 2

---

## Troubleshooting

### Common Issues

**Build fails with CozoDB errors**:
```bash
# Clean and rebuild
cargo clean
cargo build --workspace
```

**Tests fail with database errors**:
```bash
# Use in-memory database for tests
let storage = CozoDbStorage::new("mem").await?;
```

**Slow test execution**:
```bash
# Run tests in release mode
cargo test --release --workspace
```

**Import/dependency errors**:
```bash
# Update workspace dependencies
cargo update
```

### Getting Help

1. Check `/TDD-Tracker.md` for implementation status
2. Read relevant `.steeringDocs/` files for architecture
3. Review existing tool implementations (01, 02, 03)
4. Follow TDD-first: Write tests to understand expected behavior

---

## Critical Files for Context

When working on this codebase, always review:

1. **TDD-Tracker.md** - Current implementation status, task breakdown
2. **.steeringDocs/S01-README-MOSTIMP.md** - Core philosophy
3. **.steeringDocs/S02-code-conventions.md** - Rust patterns (comprehensive)
4. **.steeringDocs/S06-design101-tdd-architecture-principles.md** - TDD approach
5. **crates/parseltongue-core/src/entities.rs** - Core data structures
6. **crates/parseltongue-core/src/storage/** - CozoDB integration

---

## Key Principles Summary

1. **TDD-First**: RED → GREEN → REFACTOR always
2. **Executable Specifications**: Tests define contracts
3. **Ultra-Minimalist**: Simplicity over features
4. **Layered Architecture**: L1 (core) → L2 (std) → L3 (external)
5. **Performance Validation**: Every claim needs a test
6. **Idiomatic Rust**: Follow 12-layer conventions
7. **Monorepo Structure**: Extractable crates design
8. **Functional Patterns**: Immutability, composition, pure functions

---

*This file is maintained as the single source of truth for Claude Code interactions with the Parseltongue codebase. Last updated: 2025-10-29.*
