# Requirements Document

## Introduction

Parseltongue Architect v2.0 is a **Rust-only** architectural intelligence system that transforms code analysis from broken text parsing to deterministic, high-performance graph-based navigation. The system creates complete Interface Signature Graphs (ISG) from Rust codebases with **100% relationship extraction**, enabling sub-millisecond queries, real-time architectural awareness, and zero-hallucination LLM context generation.

**v2.0 Mission**: Fix the fundamental flaws in v1.0 while adding minimal viable architectural intelligence features. Ship a working system in 30 days.

**Core v2.0 Constraints:**
- **Complete Relationship Extraction**: Extract ALL CALLS, USES, and IMPLEMENTS relationships using full AST traversal
- **O(1) Performance Guarantees**: All operations must use indexed lookups, no O(N) scans
- **Deterministic Identification**: Stable hashing with Fully Qualified Names for cross-platform consistency
- **30-Day Ship Target**: Aggressive but achievable timeline focusing on core fixes + minimal enhancements

## v2.0 Requirements

### REQ-V2-001.0: Complete Relationship Extraction

**User Story:** As a Rust developer analyzing complex codebases, I want complete architectural relationship extraction so that blast-radius analysis and dependency tracking actually work correctly.

#### Acceptance Criteria

1. WHEN parsing Rust code THEN the system SHALL extract ALL function calls using `syn::visit::Visit` pattern with `visit_expr_call` and `visit_expr_method_call`
2. WHEN analyzing function bodies THEN the system SHALL identify ALL type usage relationships via `visit_type_path` traversal
3. WHEN processing impl blocks THEN the system SHALL extract ALL trait implementations using two-pass ingestion (nodes first, relationships second)
4. WHEN encountering method calls THEN the system SHALL resolve both direct function calls and method calls on types
5. WHEN building the ISG THEN the system SHALL create CALLS edges from functions to their dependencies and USES edges from functions to types they reference
6. WHEN ingestion completes THEN the system SHALL verify 100% relationship extraction with zero missing edges for parsed code

### REQ-V2-002.0: O(1) Performance Guarantees

**User Story:** As a Rust developer working on live codebases, I want guaranteed sub-millisecond performance so that the daemon meets the <12ms update and <1ms query constraints.

#### Acceptance Criteria

1. WHEN updating files THEN the system SHALL use reverse file index (`FxHashMap<Arc<str>, FxHashSet<SigHash>>`) to achieve O(1) node removal
2. WHEN querying by name THEN the system SHALL use name index (`FxHashMap<Arc<str>, FxHashSet<SigHash>>`) to achieve O(1) entity lookup
3. WHEN calculating blast radius THEN the system SHALL use bounded BFS with early termination to stay under 1ms for typical queries
4. WHEN performing any graph operation THEN the system SHALL maintain O(1) or O(log N) complexity using `FxHashMap` and `petgraph` indexed operations
5. WHEN monitoring file changes THEN the system SHALL complete updates in <12ms using indexed operations only
6. WHEN executing queries THEN the system SHALL respond in <1ms for simple traversals and <2ms for complex analysis

### REQ-V2-003.0: Deterministic Identification System

**User Story:** As a developer using the daemon across different platforms, I want stable, deterministic entity identification so that architectural analysis is consistent and reliable.

#### Acceptance Criteria

1. WHEN hashing entities THEN the system SHALL use `FxHasher` instead of `DefaultHasher` for cross-platform stability
2. WHEN generating signatures THEN the system SHALL include full module qualification (e.g., `my_crate::utils::Config` not just `Config`)
3. WHEN tracking module context THEN the system SHALL maintain current module path during AST traversal to generate Fully Qualified Names
4. WHEN processing identical code THEN the system SHALL produce identical `SigHash` values across different platforms and Rust versions
5. WHEN persisting state THEN the system SHALL ensure deterministic serialization and deserialization of the ISG
6. WHEN reloading snapshots THEN the system SHALL maintain identical graph structure and node identification

### REQ-V2-004.0: Two-Pass Ingestion Architecture

**User Story:** As a developer processing large codebases, I want reliable relationship extraction that handles forward references and complex dependencies correctly.

#### Acceptance Criteria

1. WHEN ingesting code dumps THEN the system SHALL use Pass 1 to extract and insert ALL nodes from ALL files before processing relationships
2. WHEN processing relationships THEN the system SHALL use Pass 2 to analyze impl blocks and function bodies after all nodes exist
3. WHEN encountering forward references THEN the system SHALL successfully resolve them because target nodes were created in Pass 1
4. WHEN building edges THEN the system SHALL guarantee that both source and target nodes exist before edge creation
5. WHEN ingestion fails THEN the system SHALL provide clear error messages indicating which pass failed and why
6. WHEN processing large dumps THEN the system SHALL complete two-pass ingestion in <5 seconds for 2.1MB codebases

### REQ-V2-005.0: Rust-Specific Architectural Analysis

**User Story:** As a Rust developer working with complex type systems, I want architectural analysis that understands Rust-specific patterns so that I can make informed design decisions.

#### Acceptance Criteria

1. WHEN analyzing trait implementations THEN the system SHALL extract generic bounds and constraints (e.g., `T: Send + Sync`)
2. WHEN detecting patterns THEN the system SHALL identify common Rust patterns including Builder, State Machine, and RAII patterns
3. WHEN analyzing error handling THEN the system SHALL track `Result<T, E>` usage patterns and error propagation chains
4. WHEN processing generic types THEN the system SHALL handle complex generic constraints and associated types correctly
5. WHEN querying patterns THEN the system SHALL provide `parseltongue analyze --pattern <type>` commands for architectural pattern detection
6. WHEN generating reports THEN the system SHALL output structured analysis of architectural patterns found in the codebase

### REQ-V2-006.0: Advanced Query Engine

**User Story:** As a developer maintaining large Rust codebases, I want sophisticated architectural queries that help identify technical debt and optimization opportunities.

#### Acceptance Criteria

1. WHEN querying unused code THEN the system SHALL identify functions with no incoming CALLS edges via `find_unused_functions()`
2. WHEN detecting circular dependencies THEN the system SHALL use Tarjan's algorithm to find strongly connected components in the dependency graph
3. WHEN analyzing trait hierarchies THEN the system SHALL trace multi-hop trait implementation chains with `find_trait_implementor_chains()`
4. WHEN calculating impact THEN the system SHALL provide enhanced blast-radius analysis showing both direct and transitive dependencies
5. WHEN querying relationships THEN the system SHALL support complex graph queries like "find all functions that call trait methods"
6. WHEN executing advanced queries THEN the system SHALL maintain <2ms response time for complex analysis operations

### REQ-V2-007.0: Enhanced CLI Interface

**User Story:** As a developer using the daemon daily, I want a comprehensive CLI that exposes all architectural analysis capabilities with clear, actionable output.

#### Acceptance Criteria

1. WHEN running analysis commands THEN the system SHALL support `parseltongue analyze --pattern builder|error|raii` for pattern detection
2. WHEN querying architecture THEN the system SHALL support `parseltongue query --unused-functions|--circular-deps|--trait-chains <trait>` for advanced analysis
3. WHEN generating output THEN the system SHALL provide both human-readable and JSON formats for all commands
4. WHEN displaying results THEN the system SHALL include performance metrics (execution time, node count, relationship count) in output
5. WHEN encountering errors THEN the system SHALL provide specific error messages with suggested fixes and context
6. WHEN running help THEN the system SHALL show comprehensive usage examples for all analysis and query capabilities

### REQ-V2-008.0: Production-Ready Performance and Reliability

**User Story:** As a developer deploying the daemon in production environments, I want guaranteed performance characteristics and robust error handling.

#### Acceptance Criteria

1. WHEN processing large codebases THEN the system SHALL maintain memory usage under 25MB for 100K LOC using string interning and efficient data structures
2. WHEN handling parse errors THEN the system SHALL continue processing other files and provide detailed error reports without crashing
3. WHEN daemon crashes THEN the system SHALL automatically save state and recover gracefully on restart
4. WHEN monitoring files THEN the system SHALL handle file system events reliably with automatic retry on temporary failures
5. WHEN persisting state THEN the system SHALL use incremental snapshots to minimize I/O overhead during daemon operation
6. WHEN validating performance THEN the system SHALL include built-in benchmarking and profiling capabilities for performance verification

## v2.0 Success Criteria

### Core Fixes (Foundation Repair)
1. **Complete Relationship Extraction**: 100% of CALLS, USES, and IMPLEMENTS relationships extracted and verified
2. **O(1) Performance**: All operations use indexed lookups, no O(N) scans, <12ms updates, <1ms queries
3. **Deterministic Hashing**: Stable `FxHasher` with FQNs, consistent across platforms
4. **Two-Pass Ingestion**: Reliable handling of forward references and complex dependencies

### Enhanced Capabilities (Architectural Intelligence)
1. **Rust Pattern Detection**: Identify Builder, State Machine, RAII, and error handling patterns
2. **Advanced Queries**: Unused code detection, circular dependency analysis, trait hierarchy traversal
3. **Enhanced CLI**: Comprehensive analysis commands with structured output
4. **Production Reliability**: Robust error handling, automatic recovery, performance monitoring

### Performance Validation (Measurable)
1. **Ingestion Speed**: <5s for 2.1MB dumps (measured with real Axum codebase)
2. **Update Latency**: <12ms for file changes (measured with instrumentation)
3. **Query Performance**: <1ms simple queries, <2ms complex analysis (measured with benchmarks)
4. **Memory Efficiency**: <25MB for 100K LOC (measured with profiling tools)
5. **Relationship Completeness**: 100% extraction verified with manual code review
6. **Cross-Platform Consistency**: Identical results on Linux, macOS, Windows

## v2.0 Scope Control

### In Scope (30-Day Sprint)
- ✅ Foundation fixes (hashing, indexing, relationship extraction)
- ✅ Rust-specific pattern detection
- ✅ Advanced query engine
- ✅ Enhanced CLI interface
- ✅ Production reliability features

### Out of Scope (Deferred to v3.0)
- ❌ Macro expansion and procedural macro analysis
- ❌ IDE integration and language server protocol
- ❌ Web interface and visualization tools
- ❌ Distributed analysis and horizontal scaling
- ❌ Multi-language support (staying Rust-focused)

**Core Validation**: Proves that deterministic, sub-millisecond architectural intelligence on live Rust codebases is achievable with complete relationship extraction and O(1) performance guarantees.