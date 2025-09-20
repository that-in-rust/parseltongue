# Parseltongue AIM Daemon - Architecture Backlog

> **Purpose**: This document captures all technical architecture concepts extracted from reference document analysis. These concepts inform our design decisions and future development, organized by implementation priority.

## Document Sources Analyzed

### ✅ Completed Analysis (_refDocs)
- 18/18 documents analyzed (~13,000+ lines)
- All MVP-relevant concepts extracted and categorized
- Advanced concepts moved to appropriate backlog versions

### 🟡 Pending Analysis (_refIdioms)  
- 0/24 documents analyzed
- Contains Rust patterns, TDD methodologies, and advanced architectural concepts
- Analysis needed to complete architecture research

---

## Core Technical Architecture (MVP-Relevant)

### 1. Hybrid Storage Model
**Source**: Notes06.md, ideation20250918.md, aim-daemon-analysis.md
**Concept**: Dual storage system optimized for different workloads
```rust
pub struct HybridStorage {
    // Hot path: In-memory for real-time updates
    memory_graph: DashMap<SigHash, Node>,
    
    // Cold path: SQLite for complex queries and persistence
    sqlite_db: SqlitePool,
}
```
**MVP Implementation**:
- DashMap for concurrent real-time updates (<1ms)
- SQLite with WAL mode for persistence and complex queries (<200μs)
- Atomic synchronization between layers

### 2. Performance Pipeline (3-12ms Total)
**Source**: rust-parsing-complexity-analysis.md, Notes06.md, ideation20250918.md
**Breakdown**:
- File System Watcher: <1ms (OS-native inotify/kqueue)
- AST Parsing: 2-8ms (syn crate, incremental parsing)
- Graph Update: 1-3ms (atomic in-memory operations)
- SQLite Sync: 1-2ms (WAL mode, prepared statements)

**Implementation Strategy**:
```rust
pub struct UpdatePipeline {
    watcher: RecommendedWatcher,      // notify crate
    parser: SynParser,                // syn-based AST parsing
    graph: Arc<RwLock<InMemoryGraph>>, // DashMap-based
    db: SqlitePool,                   // WAL mode
}
```

### 3. SigHash System
**Source**: Notes06.md, interface-stub-analysis-summary.md, ideation20250918.md
**Concept**: Blake3-based deterministic hashing for O(1) lookups
```rust
pub struct SigHash(u64);

impl SigHash {
    pub fn from_signature(fqp: &str, signature: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(fqp.as_bytes());
        hasher.update(signature.as_bytes());
        let hash = hasher.finalize();
        SigHash(u64::from_le_bytes(hash.as_bytes()[0..8].try_into().unwrap()))
    }
}
```

### 4. Graph Schema (7 Node Types, 9 Relationship Types)
**Source**: interface-stub-analysis-summary.md, Notes06.md, aim-daemon-analysis.md

**Node Types**:
- File: Source file metadata
- Module: Logical namespace
- Struct: Data structures
- Trait: Interface contracts
- Function: Executable logic
- Impl: Implementation blocks
- Type: Generic/alias types

**Relationship Types**:
- IMPL: Type implements trait
- CALLS: Function invokes function
- ACCEPTS: Function parameter type
- RETURNS: Function return type
- CONTAINS: Module/file contains item
- BOUND_BY: Generic constrained by trait
- DEFINES: Trait defines method
- EXTENDS: Inheritance relationship
- USES: Dependency relationship

### 5. File System Monitoring
**Source**: aim-daemon-file-discovery.md, ideation20250918.md, CLAUDE.md
**Implementation**:
```rust
pub struct FileMonitor {
    watcher: RecommendedWatcher,
    event_queue: mpsc::Receiver<FileEvent>,
    debounce_delay: Duration, // 100ms default
}
```
**Features**:
- Cargo.toml detection for Rust projects
- Smart filtering (ignore target/, .git/, node_modules/)
- Debounced event processing
- Batch event handling for performance

### 6. Code Dump Parser
**Source**: aim-daemon-code-dump-parser.md, parseltongue-user-journeys.md
**Format Support**: FILE: marker separated dumps
```rust
pub struct CodeDumpParser {
    files: HashMap<PathBuf, CodeDumpFile>,
    virtual_fs: VirtualFileSystem,
}
```
**Performance**: Same query performance as live files (<500μs)

### 7. CLI Interface
**Source**: parseltongue-user-journeys.md, parseltongue-brand-identity.md
**Commands**:
- `parseltongue extract` / `aim extract`: Full codebase analysis
- `parseltongue query` / `aim query`: Architectural queries  
- `parseltongue generate-context` / `aim generate-context`: LLM context
- `parseltongue extract-dump` / `aim extract-dump`: Code dump processing

**Query Types**:
- `blast-radius`: Impact analysis for refactoring safety
- `what-implements`: Find trait implementations
- `find-cycles`: Circular dependency detection
- `generate-context`: Bounded context for LLM integration

---

## Implementation Patterns (MVP-Relevant)

### 1. Anti-Coordination Principles
**Source**: SESSION_CONTEXT.md, CLAUDE.md, code-conventions.md
**Rules**:
- NO coordination layers, coordinators, or event buses
- NO distributed transactions, sagas, or event sourcing
- NO circuit breakers, retry queues, or complex error recovery
- Simple SQLite operations with direct function calls

### 2. Error Handling Patterns
**Source**: code-conventions.md, CLAUDE.md, Notes05.md
**Conventions**:
- Result<T, E> only - no custom error types unless necessary
- Flat error handling - avoid nested Result chains
- User-friendly messages - convert technical errors
- Graceful degradation - continue processing on individual failures

### 3. File Organization
**Source**: code-conventions.md, CLAUDE.md
**Rules**:
- Maximum 500 lines per file
- Clear module boundaries - no circular dependencies
- Single responsibility - each file has one clear purpose
- Rails-style modules: models/, handlers/, services/

### 4. Database Patterns
**Source**: code-conventions.md, Notes06.md
**Conventions**:
- sqlx::query! macros for compile-time SQL validation
- Direct SQL - no query builders beyond sqlx
- Prepared statements for performance
- WAL mode for concurrent access

---

## Success Metrics (Validated)

### Performance Targets
**Source**: Multiple documents validation
- **Compression**: >95% token reduction (2.1MB → 15KB architectural essence)
- **Update Latency**: <12ms from file save to query readiness
- **Query Performance**: <500μs for simple traversals, <1ms for complex
- **Memory Efficiency**: <25MB for 100K LOC Rust codebase
- **Accuracy**: 85-90% pattern coverage with syn parsing, zero false positives

### Scalability Targets
- Handle codebases up to 1M+ lines of code
- Support concurrent access without blocking
- Maintain performance with large graphs
- Graceful handling of parsing errors

---

## Advanced Concepts (Post-MVP Backlog)

### Version 1.5 Features (3-6 months post-MVP)
**Source**: backlog20250918.md, aim-backlog.md
- In-memory caching layer for hot queries
- Advanced Rust pattern recognition (macros, lifetimes)
- Enhanced error recovery and resilience
- Performance monitoring and alerting
- Basic configuration system (aim.toml)
- Git integration for file discovery

### Version 2.0 Features (6-12 months post-MVP)
**Source**: Multiple documents
- Multi-language support (TypeScript, Python via pluggable parsers)
- Advanced architectural pattern detection
- Code quality metrics and technical debt analysis
- CI/CD integration and automation
- HTTP API for external integrations
- Real-time daemon mode with background processing

### Version 3.0+ Features (12+ months post-MVP)
**Source**: ideation20250918.md, backlog20250918.md
- Graph database migration (MemGraph/SurrealDB)
- Distributed codebase analysis
- Enterprise security and access control
- Advanced LLM integration patterns
- Machine learning integration for predictions
- IDE integrations (LSP, VS Code extension)
- Visualization and documentation generation

---

## Research Areas (Future Investigation)

### Graph Theory Applications
**Source**: backlog20250918.md, Notes06.md
- Optimal graph compression algorithms
- Efficient shortest path algorithms for blast radius
- Community detection in code modules
- Graph neural networks for code understanding

### Performance Research
**Source**: rust-parsing-complexity-analysis.md, Notes06.md
- Sub-millisecond query optimization
- Memory-mapped file techniques
- Lock-free concurrent data structures
- SIMD optimizations for graph operations

### LLM Integration Research
**Source**: parseltongue-user-journeys.md, Notes06.md
- Optimal context window utilization
- Fine-tuning models on architectural patterns
- Prompt engineering for code generation
- Multi-modal code understanding (text + graph)

---

## Technology Stack Validation

### Core Technologies (MVP)
**Source**: Multiple documents validation
- **Language**: Rust for performance and safety
- **Parsing**: syn crate for Rust AST parsing
- **Storage**: SQLite with WAL mode
- **Concurrency**: DashMap, Arc<RwLock<T>>
- **File Watching**: notify crate
- **CLI**: clap for argument parsing
- **Hashing**: Blake3 for SigHash generation

### Future Technologies (Post-MVP)
- **Multi-language**: swc (TypeScript), tree-sitter (universal)
- **Graph DB**: MemGraph, SurrealDB for advanced queries
- **Serialization**: MessagePack, Protocol Buffers
- **Web Interface**: WASM + React/Svelte
- **ML**: Graph Neural Networks, embeddings

---

## Competitive Differentiation

### Unique Value Proposition
**Source**: backlog20250918.md, parseltongue-user-journeys.md
- **Real-time updates** vs batch processing
- **LLM-optimized output** vs human-readable reports
- **Architectural focus** vs code quality focus
- **Millisecond response times** vs minute-long analysis
- **Deterministic navigation** vs probabilistic search

### Target Markets
- Individual developers using LLMs
- Development teams with large codebases
- Enterprise organizations with complex architectures
- Code analysis and consulting services

---

## Next Steps

### Immediate (Complete Task 1)
1. Analyze remaining _refIdioms documents (24 files)
2. Extract additional Rust patterns and TDD methodologies
3. Validate architectural decisions against advanced patterns
4. Complete architecture research phase

### Short Term (Task 2)
1. Requirements quality assurance review
2. Integration of architecture concepts into requirements.md
3. Validation of technical feasibility

### Medium Term (Phase 2)
1. Detailed technical design based on validated architecture
2. API specification design
3. Implementation planning with specific technology choices

This architecture backlog ensures no valuable technical insights are lost while maintaining clear separation between MVP implementation and future enhancements.
## S
torage Architecture Decisions - DEFERRED

**Status**: All storage architecture decisions marked as **TBD** in requirements.md

**Rationale**: Storage technology selection is premature at this stage. Focus should remain on:
1. Finalizing functional requirements
2. Establishing performance benchmarks  
3. Validating core use cases

**Research Completed**: Comprehensive analysis of SQLite, SurrealDB, MemGraph, TigerGraph, and in-memory options documented in `storage-architecture-options.md`

**Decision Timeline**: Storage architecture will be decided during design phase after requirements are finalized.

**Key Insight**: Three-phase evolution path (SQLite → In-Memory → Distributed) provides clear migration strategy regardless of initial choice.

---
**Added**: 2025-09-20 - Storage decisions deferred to design phase