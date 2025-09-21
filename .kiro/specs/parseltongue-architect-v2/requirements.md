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

### REQ-V2-009.0: Terminal-Based LLM Context Generation

**User Story:** As a Rust developer using LLM assistants from the terminal, I want compressed, deterministic architectural context that eliminates hallucination, so that AI tools receive factual architectural information for accurate code assistance.

#### Acceptance Criteria

1. WHEN I run `parseltongue generate-context --focus <entity>` THEN the system SHALL perform targeted, high-speed queries against the ISG to assemble a perfectly tailored architectural slice
2. WHEN generating context THEN the system SHALL include canonical definitions, complete implementation lists, and public signatures of all key entities within 2 hops
3. WHEN outputting context THEN the system SHALL produce highly compressed, minimal text blocks representing global architectural context using only 1% of typical LLM context windows
4. WHEN I add `--format llm-prompt` THEN the system SHALL structure output with clear sections optimized for LLM consumption with deterministic facts
5. WHEN context includes trait relationships THEN the system SHALL show all IMPL relationships, generic bounds, and associated types for complete architectural picture
6. WHEN generating context for functions THEN the system SHALL include all CALLS relationships, parameter types, and return types with full qualification

### REQ-V2-010.0: Real-Time Daemon Integration for Terminal Workflows

**User Story:** As a Rust developer working primarily from terminal, I want seamless daemon integration that provides instant architectural intelligence during my coding sessions, so that I can query live architectural state without interrupting my workflow.

#### Acceptance Criteria

1. WHEN I start `parseltongue daemon --watch <directory>` THEN the system SHALL initialize with sub-second state hydration from existing database and begin monitoring all .rs files
2. WHEN I save any .rs file THEN the daemon SHALL complete the 3-12ms update pipeline (debounce → parse → diff → apply → persist) within the performance target
3. WHEN I run `parseltongue query <type> <target>` while daemon is running THEN the system SHALL query the live in-memory graph and respond in <1ms
4. WHEN the daemon detects file changes THEN it SHALL use Tree-sitter incremental parsing to reuse unchanged portions of syntax trees for maximum performance
5. WHEN I query architectural state THEN the system SHALL always reflect the current state of my code, never stale or cached data
6. WHEN daemon is running THEN all queries SHALL use the hybrid storage model (in-memory DashMap for writes, SQLite for complex analytical queries)

### REQ-V2-011.0: Terminal-Based Architectural Debugging

**User Story:** As a Rust developer debugging complex trait bound and compilation errors from terminal, I want architectural intelligence to help me understand and resolve errors quickly using deterministic facts about my codebase.

#### Acceptance Criteria

1. WHEN I encounter trait bound errors THEN I can run `parseltongue debug --error-context <entity>` to get architectural analysis of the error using live ISG data
2. WHEN analyzing "trait bound not satisfied" errors THEN the system SHALL show which traits are actually implemented and what's missing using IMPL relationship traversal
3. WHEN debugging complex generic constraints THEN the system SHALL display the full trait hierarchy and bounds in readable format with `parseltongue query trait-hierarchy <trait>`
4. WHEN I have orphan rule violations THEN the system SHALL identify conflicting implementations using `parseltongue query conflicts <trait>` and suggest resolution strategies
5. WHEN compilation fails due to missing dependencies THEN the system SHALL show dependency chains with `parseltongue query dependency-chain <entity>` 
6. WHEN error analysis completes THEN the system SHALL provide actionable suggestions with specific function signatures and trait bounds needed

### REQ-V2-012.0: Live Architectural Exploration and Navigation

**User Story:** As a Rust developer exploring unfamiliar codebases from terminal, I want interactive architectural navigation tools that leverage the live daemon state, so that I can quickly understand complex systems and their relationships.

#### Acceptance Criteria

1. WHEN I run `parseltongue explore --interactive` THEN the system SHALL start an interactive terminal session with live architectural queries
2. WHEN in interactive mode THEN I can use commands like `find <pattern>`, `show <entity>`, `blast-radius <entity>`, and `implementations <trait>` with tab completion
3. WHEN I execute `blast-radius <entity>` THEN the system SHALL show real-time impact analysis using bounded BFS traversal of the live graph
4. WHEN I run `parseltongue map --module <module>` THEN the system SHALL generate ASCII art or structured text representation of module relationships
5. WHEN exploring trait hierarchies THEN I can use `trace-implementations <trait>` to see multi-hop implementation chains with full qualification
6. WHEN navigating results THEN the system SHALL provide file paths and line numbers for quick navigation with editor integration

### REQ-V2-013.0: Deterministic Context for Terminal LLM Workflows

**User Story:** As a developer collaborating with LLM assistants from terminal, I want to provide deterministic architectural context that grounds AI responses in factual information, eliminating the "Stochastic Fog" of probabilistic code analysis.

#### Acceptance Criteria

1. WHEN I run `parseltongue context --entity <entity> --llm-format` THEN the system SHALL generate context blocks with preconditions, postconditions, and architectural relationships
2. WHEN providing context to LLMs THEN the system SHALL include deterministic facts like "Entity X IMPL Trait Y", "Function A CALLS Function B", with no probabilistic language
3. WHEN generating context for refactoring THEN the system SHALL provide complete architectural slices showing all affected entities and their relationships
4. WHEN I add `--include-patterns` THEN the system SHALL analyze and include relevant Rust patterns (Builder, State Machine, RAII) detected in the architectural context
5. WHEN context includes error handling THEN the system SHALL show `Result<T, E>` usage patterns and error propagation chains with full type information
6. WHEN outputting for LLM consumption THEN the system SHALL structure context with clear sections: Signatures, Dependencies, Relationships, Patterns, and Constraints

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

### Terminal-Based Symbiotic Development (New)
1. **LLM Context Generation**: Compressed, deterministic architectural context using <1% of LLM context windows
2. **Real-Time Daemon Integration**: Live architectural state with 3-12ms update pipeline for terminal workflows
3. **Architectural Debugging**: Terminal-based error analysis with deterministic trait bound and dependency resolution
4. **Interactive Exploration**: Live architectural navigation with tab completion and real-time graph traversal
5. **Deterministic LLM Workflows**: Zero-hallucination context generation with factual architectural relationships

### Performance Validation (Measurable)
1. **Ingestion Speed**: <5s for 2.1MB dumps (measured with real Axum codebase)
2. **Update Latency**: <12ms for file changes (measured with instrumentation)
3. **Query Performance**: <1ms simple queries, <2ms complex analysis (measured with benchmarks)
4. **Memory Efficiency**: <25MB for 100K LOC (measured with profiling tools)
5. **Relationship Completeness**: 100% extraction verified with manual code review
6. **Cross-Platform Consistency**: Identical results on Linux, macOS, Windows
7. **Context Compression**: 95%+ token reduction for LLM context (architectural facts vs raw code)
8. **Real-Time Responsiveness**: <100ms for interactive terminal commands during live development

## v2.0 Scope Control

### In Scope (30-Day Sprint)
- ✅ Foundation fixes (hashing, indexing, relationship extraction)
- ✅ Rust-specific pattern detection
- ✅ Advanced query engine
- ✅ Enhanced CLI interface
- ✅ Production reliability features
- ✅ Terminal-based LLM context generation with deterministic facts
- ✅ Real-time daemon integration for live terminal workflows
- ✅ Architectural debugging and error analysis from terminal
- ✅ Interactive exploration with live graph navigation
- ✅ Zero-hallucination LLM workflows with compressed context

### Out of Scope (Deferred to v3.0)
- ❌ Macro expansion and procedural macro analysis
- ❌ IDE integration and language server protocol
- ❌ Web interface and visualization tools
- ❌ Distributed analysis and horizontal scaling
- ❌ Multi-language support (staying Rust-focused)
- ❌ Advanced team collaboration features
- ❌ Custom architectural rule engines

**Core Validation**: Proves that deterministic, sub-millisecond architectural intelligence on live Rust codebases is achievable with complete relationship extraction and O(1) performance guarantees, enabling "Symbiotic Developer" workflows where LLM assistants receive live, factual architectural context from terminal-based development sessions.