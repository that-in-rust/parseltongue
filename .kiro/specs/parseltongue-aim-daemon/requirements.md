# Requirements Document

## Introduction

Parseltongue AIM Daemon is a revolutionary **Rust-only** development tool that transforms code analysis from probabilistic text searches to deterministic, graph-based architectural navigation. The system creates Interface Signature Graphs (ISG) exclusively from Rust codebases, enabling sub-millisecond queries, real-time architectural awareness, and zero-hallucination LLM context generation. The daemon provides developers with factual dependency analysis, blast-radius impact assessment, and constraint-aware AI assistance through compressed architectural intelligence.

**Core Constraints:**
- **Rust-Only Focus**: Exclusively designed for Rust codebases using `syn` crate for high-fidelity parsing
- **High-Speed Updates**: Interface graph updates must complete in <12ms for real-time development workflow
- **LLM-Terminal Integration**: Optimized for LLMs querying from terminal during active development sessions

## Requirements

### Requirement 1

**User Story:** As a Rust developer analyzing unfamiliar Rust codebases, I want to ingest Rust-only code dumps and extract architectural intelligence deterministically, so that I can understand complex Rust systems in seconds rather than hours.

#### Acceptance Criteria

1. WHEN a user provides a Rust code dump file THEN the system SHALL parse it using ONLY the `syn` crate and extract all Rust interface signatures
2. WHEN processing a 2.1MB Rust code dump THEN the system SHALL complete ISG construction in less than 5 seconds
3. WHEN building the Interface Signature Graph THEN the system SHALL create nodes exclusively for Rust Function, Struct, and Trait entities with CALLS, IMPL, and USES relationships
4. WHEN compression is applied THEN the system SHALL achieve 95%+ token compression ratio from raw Rust code to architectural essence
5. WHEN ISG construction completes THEN the system SHALL provide real-time status updates showing Rust nodes created, edges established, and compression achieved
6. WHEN encountering non-Rust files THEN the system SHALL ignore them and focus exclusively on .rs files

### Requirement 2

**User Story:** As a Rust developer working on live Rust codebases, I want high-speed real-time architectural monitoring with incremental updates using SigHash-based deterministic identification, so that I can receive immediate feedback on Rust architectural changes without workflow interruption.

#### Acceptance Criteria

1. WHEN the daemon is initialized THEN the system SHALL monitor the filesystem using the `notify` crate exclusively for .rs file changes with platform-specific optimizations (inotify/kqueue)
2. WHEN a Rust file is modified and saved THEN the system SHALL detect the change within 1ms and queue it for high-speed processing using crossbeam channels
3. WHEN processing incremental updates THEN the system SHALL complete ISG updates in less than 12ms from .rs file save to query readiness (CRITICAL PERFORMANCE CONSTRAINT)
4. WHEN updating the ISG THEN the system SHALL perform atomic in-memory graph updates using SigHash-based O(1) lookups with SQLite WAL persistence
5. WHEN changes are processed THEN the system SHALL maintain sub-millisecond query response times throughout the update cycle using Arc<RwLock<HashMap<SigHash, Node>>>
6. WHEN monitoring files THEN the system SHALL ignore all non-Rust files and focus exclusively on src/, tests/, and examples/ directories
7. WHEN generating SigHash THEN the system SHALL create deterministic 64-bit hashes from full Rust signatures for O(1) node/edge lookup

### Requirement 3

**User Story:** As a Rust developer needing dependency analysis, I want deterministic graph-based queries that return factual results with Rust-specific pattern recognition, so that I can make confident architectural decisions without probabilistic guessing.

#### Acceptance Criteria

1. WHEN querying trait implementations THEN the system SHALL traverse IMPL edges and return all implementing structs/functions in sub-millisecond time
2. WHEN performing blast-radius analysis THEN the system SHALL execute recursive graph traversal to identify all affected functions and modules with Rust ownership implications
3. WHEN detecting circular dependencies THEN the system SHALL run Tarjan's algorithm on the ISG and report any cycles found in Rust module hierarchy
4. WHEN executing any query THEN the system SHALL respond in less than 1ms for simple graph traversals
5. WHEN returning query results THEN the system SHALL provide zero false positives and complete dependency coverage
6. WHEN analyzing Rust patterns THEN the system SHALL recognize newtype patterns, smart pointer usage (Arc, Rc, Box), and async patterns
7. WHEN detecting anti-patterns THEN the system SHALL identify blocking calls in async contexts, unnecessary cloning, and error swallowing

### Requirement 4

**User Story:** As a developer using LLMs for code assistance, I want compressed ISG context with deterministic SigHash-based relationships that eliminates hallucination, so that AI tools receive perfect architectural truth instead of raw code snippets.

#### Acceptance Criteria

1. WHEN generating bounded context THEN the system SHALL extract relevant ISG slices for specific Rust functions with their dependencies using SigHash traversal
2. WHEN creating LLM prompts THEN the system SHALL include Rust function signatures, trait constraints, ownership patterns, and dependency relationships
3. WHEN compressing context THEN the system SHALL achieve 95%+ token reduction (100k LOC → 15-25MB in-memory, 200-token summaries) while preserving all architectural relationships
4. WHEN formatting for LLMs THEN the system SHALL structure prompts with clear sections for Rust signatures, trait bounds, ownership constraints, and blast radius
5. WHEN providing context THEN the system SHALL include upstream callers, downstream dependencies, trait implementations, and integration requirements
6. WHEN generating context THEN the system SHALL use deterministic SigHash-based node identification to ensure consistent, reproducible results
7. WHEN creating prompts THEN the system SHALL include Rust-specific constraints like Send + Sync bounds, lifetime parameters, and error propagation patterns

### Requirement 5

**User Story:** As a Rust developer and LLMs working from terminal, I want a comprehensive CLI interface optimized for quick queries during active development, so that architectural intelligence can be accessed instantly during coding sessions.

#### Acceptance Criteria

1. WHEN ingesting Rust code dumps THEN the system SHALL provide `parseltongue ingest-code --source CodeDump <file>` command for Rust-only processing
2. WHEN LLMs or developers query the ISG THEN the system SHALL support `parseltongue query <query-type> <target>` with multiple Rust-specific query types
3. WHEN starting daemon mode THEN the system SHALL provide `parseltongue daemon --watch <directory>` for real-time Rust file monitoring
4. WHEN executing commands THEN the system SHALL provide real-time progress updates optimized for terminal consumption
5. WHEN operations complete THEN the system SHALL display performance metrics including execution time and Rust-specific resource usage
6. WHEN LLMs query from terminal THEN the system SHALL provide structured output formats suitable for AI consumption and human readability

### Requirement 6

**User Story:** As a developer working with large Rust projects, I want efficient memory management and performance optimization using Rust's zero-cost abstractions, so that the daemon can handle enterprise-scale codebases without resource constraints.

#### Acceptance Criteria

1. WHEN processing 100K lines of Rust code THEN the system SHALL maintain in-memory ISG footprint under 25MB using efficient Rust data structures
2. WHEN handling concurrent queries THEN the system SHALL maintain sub-millisecond response times under load using Arc<RwLock<T>> for thread-safe access
3. WHEN persisting data THEN the system SHALL use SQLite with WAL mode for atomic updates and crash recovery with sqlx compile-time query validation
4. WHEN managing memory THEN the system SHALL implement efficient graph data structures using Rust's ownership system and zero-cost abstractions
5. WHEN scaling up THEN the system SHALL handle codebases up to 500K LOC while maintaining performance targets
6. WHEN using smart pointers THEN the system SHALL follow the decision matrix: Box<T> for unique ownership, Arc<T> for shared ownership, RwLock<T> for interior mutability
7. WHEN processing iteratively THEN the system SHALL use iterator chains for zero-cost functional programming patterns

### Requirement 7

**User Story:** As a developer integrating Parseltongue with other tools, I want structured data output and high-performance API interfaces, so that I can build additional tooling and integrations on top of the architectural intelligence.

#### Acceptance Criteria

1. WHEN outputting query results THEN the system SHALL provide structured JSON format alongside human-readable output
2. WHEN running as a daemon THEN the system SHALL expose HTTP/gRPC query server API on configurable port (default 8080) with <1 second startup time
3. WHEN generating context THEN the system SHALL support multiple output formats including markdown and structured data
4. WHEN providing status updates THEN the system SHALL emit machine-readable progress information
5. WHEN integrating with IDEs THEN the system SHALL support language server protocol for real-time architectural feedback
6. WHEN handling concurrent requests THEN the system SHALL support at least 1000 concurrent connections using async Rust (tokio)
7. WHEN serving gRPC requests THEN the system SHALL respond with binary-encoded results for maximum performance with Rust clients

### Requirement 8

**User Story:** As an LLM querying Parseltongue from terminal during active Rust development, I want a variety of specialized query types with instant responses, so that I can provide accurate architectural guidance without interrupting the developer's flow.

#### Acceptance Criteria

1. WHEN querying trait implementations THEN the system SHALL support `what-implements <trait>` returning all Rust implementations instantly
2. WHEN analyzing impact THEN the system SHALL support `blast-radius <entity>` showing all affected Rust functions and modules
3. WHEN checking architecture health THEN the system SHALL support `find-cycles` detecting circular dependencies in Rust code
4. WHEN generating context THEN the system SHALL support `generate-context <function>` for bounded Rust context extraction
5. WHEN creating prompts THEN the system SHALL support `generate-prompt --task <task> --context <entity>` for constraint-aware Rust assistance
6. WHEN executing any query THEN the system SHALL respond in <1ms to maintain development flow
7. WHEN providing results THEN the system SHALL format output for both human developers and LLM consumption

### Requirement 9

**User Story:** As a developer concerned with Rust code quality, I want architectural validation and debt detection specific to Rust patterns, so that I can maintain clean Rust architecture and prevent technical debt accumulation.

#### Acceptance Criteria

1. WHEN analyzing Rust architecture THEN the system SHALL detect circular dependencies at both module and function levels using Rust-specific analysis
2. WHEN Rust changes are made THEN the system SHALL validate Rust architectural constraints and report violations
3. WHEN performing impact analysis THEN the system SHALL identify breaking changes specific to Rust interfaces and their scope
4. WHEN monitoring continuously THEN the system SHALL track Rust architectural health metrics over time
5. WHEN detecting issues THEN the system SHALL provide actionable recommendations for Rust architectural improvements

### Requirement 10

**User Story:** As a Rust developer following idiomatic patterns, I want the daemon to recognize and validate Rust-specific type safety and error handling patterns, so that I can maintain compile-first success and prevent runtime errors.

#### Acceptance Criteria

1. WHEN analyzing type definitions THEN the system SHALL recognize newtype patterns for domain IDs (UserId, RoomId, MessageId) and validate their usage
2. WHEN examining error handling THEN the system SHALL identify thiserror usage for library errors and anyhow for application context
3. WHEN detecting async patterns THEN the system SHALL recognize actor patterns with message passing and structured concurrency with JoinSet
4. WHEN analyzing ownership THEN the system SHALL validate borrowing patterns and identify unnecessary clones or ownership transfers
5. WHEN checking collections THEN the system SHALL verify the "accept slices, store owned, return owned" pattern compliance
6. WHEN examining smart pointers THEN the system SHALL validate appropriate usage of Box<T>, Arc<T>, Rc<T>, and interior mutability patterns
7. WHEN detecting anti-patterns THEN the system SHALL flag blocking calls in async contexts, error swallowing with let _, and manual loops instead of iterators

### Requirement 11

**User Story:** As a Rust developer following TDD-first development, I want the daemon to support compile-time validation and property-based testing patterns, so that I can achieve one-shot correctness and prevent coordination complexity.

#### Acceptance Criteria

1. WHEN analyzing function signatures THEN the system SHALL extract complete type contracts including all error cases for TDD validation
2. WHEN examining database queries THEN the system SHALL recognize sqlx compile-time query validation patterns and flag runtime-only queries
3. WHEN detecting test patterns THEN the system SHALL identify property-based tests using proptest and integration tests with real database connections
4. WHEN analyzing type safety THEN the system SHALL validate that invalid states are made unrepresentable through enum and struct design
5. WHEN examining async code THEN the system SHALL recognize structured concurrency patterns and flag unstructured async spawning
6. WHEN checking error propagation THEN the system SHALL validate proper use of ? operator and Result<T, E> patterns throughout call chains
7. WHEN analyzing dependencies THEN the system SHALL flag forbidden coordination dependencies (Redis, message queues, ORMs) that violate TDD-first simplicity

### Requirement 12

**User Story:** As a Rust developer needing precise architectural representation, I want a comprehensive graph schema with 7 node types and 9 relationship types optimized for Rust semantics, so that I can query exact structural relationships with deterministic results.

#### Acceptance Criteria

1. WHEN creating nodes THEN the system SHALL support exactly 7 node types: File, Module, Struct, Trait, Function, Impl, and Type with Rust-specific metadata
2. WHEN establishing relationships THEN the system SHALL support exactly 9 edge types: CONTAINS, DEFINES, CALLS, IMPL, IMPL_TARGET, INHERITS, USES, DEPENDS, and OVERRIDE
3. WHEN storing nodes THEN the system SHALL include SigHash (64-bit), NodeKind, name, file_path, full_signature, and visibility for each entity
4. WHEN storing edges THEN the system SHALL include from_sig, to_sig, EdgeKind, and metadata with line number information
5. WHEN querying implementations THEN the system SHALL use IMPL edges to traverse from traits to implementing structs with O(1) lookup
6. WHEN analyzing dependencies THEN the system SHALL use USES and DEPENDS edges to identify module and function dependencies
7. WHEN detecting inheritance THEN the system SHALL use INHERITS edges for struct inheritance patterns in Rust
8. WHEN tracking calls THEN the system SHALL use CALLS edges to map function invocation relationships with precise line numbers

### Requirement 13

**User Story:** As a Rust developer working with multiple code sources (live filesystem, code dumps, git repositories), I want unified graph merging with deterministic conflict resolution, so that I can analyze architectural relationships across different Rust codebases simultaneously.

#### Acceptance Criteria

1. WHEN ingesting multiple sources THEN the system SHALL support LiveFs (filesystem), CodeDump (separated format), and GitRepo input sources for Rust code
2. WHEN merging graphs THEN the system SHALL use deterministic conflict resolution strategies: LatestTimestamp, HighestSigHash, or ManualPrompt
3. WHEN processing code dumps THEN the system SHALL parse separated dump format with FILE: markers and extract all .rs files
4. WHEN resolving conflicts THEN the system SHALL use SigHash comparison for deterministic, reproducible merge results
5. WHEN querying merged graphs THEN the system SHALL provide source attribution showing which input source contributed each node/edge
6. WHEN updating sources THEN the system SHALL support incremental updates to individual sources without full re-extraction
7. WHEN handling conflicts THEN the system SHALL preserve all source metadata to enable rollback and source-specific queries

### Requirement 14

**User Story:** As a Rust developer requiring enterprise-grade performance, I want specific latency targets and technical implementation details that guarantee sub-millisecond query performance with deterministic SigHash-based operations, so that the daemon can handle production-scale Rust codebases without performance degradation.

#### Acceptance Criteria

1. WHEN detecting file changes THEN the system SHALL achieve <1ms latency from file save event to daemon event queue using platform-specific optimizations (inotify/kqueue/ReadDirectoryChangesW)
2. WHEN parsing Rust AST THEN the system SHALL complete parsing in 2-5ms for typical Rust files (500 lines) using `syn` crate with incremental parsing
3. WHEN updating the graph THEN the system SHALL perform atomic updates in 1-3ms using Arc<Mutex<InterfaceGraph>> with HashMap<SigHash, Node> for O(1) lookups
4. WHEN persisting to SQLite THEN the system SHALL complete database writes in 1-4ms using WAL mode with transaction batching and optimized indexes
5. WHEN serving queries THEN the system SHALL respond in <500μs for simple graph traversals using in-memory HashMap lookups with SigHash keys
6. WHEN processing total pipeline THEN the system SHALL maintain 3-12ms total latency from file save to query readiness (CRITICAL PERFORMANCE TARGET)
7. WHEN handling concurrent access THEN the system SHALL use DashMap for lock-free concurrent access and crossbeam channels for event queuing with bounded capacity (1k items)
8. WHEN managing memory THEN the system SHALL compress 100k LOC Rust codebases to 15-25MB in-memory footprint using deterministic SigHash compression

### Requirement 15

**User Story:** As a Rust developer requiring system resilience, I want comprehensive error handling and recovery mechanisms specific to Rust development workflows, so that temporary issues don't disrupt my development process or architectural analysis.

#### Acceptance Criteria

1. WHEN Rust file parsing fails THEN the system SHALL log the `syn` crate error details and continue processing other .rs files
2. WHEN SQLite operations fail THEN the system SHALL retry up to 3 times with exponential backoff before logging failure
3. WHEN memory usage exceeds thresholds THEN the system SHALL trigger Rust's garbage collection and clear SigHash caches
4. WHEN the `notify` crate file watcher fails THEN the system SHALL attempt to restart .rs file monitoring after a brief delay
5. WHEN unrecoverable errors occur THEN the system SHALL log detailed Rust diagnostics and shut down gracefully
6. WHEN database corruption is detected THEN the system SHALL trigger automatic rebuild from live Rust filesystem
7. WHEN concurrent access conflicts occur THEN the system SHALL use Rust's Arc<RwLock<T>> patterns to handle contention gracefully

### Requirement 16

**User Story:** As a Rust software architect enforcing design patterns, I want advanced architectural constraint validation and rule enforcement, so that I can maintain clean Rust architecture and prevent violations of established patterns.

#### Acceptance Criteria

1. WHEN performing blast-radius analysis THEN the system SHALL identify all Rust nodes affected by changes to a target node using graph traversal
2. WHEN detecting cycles THEN the system SHALL use Tarjan's algorithm to find strongly connected components in Rust module dependencies
3. WHEN querying trait implementations THEN the system SHALL find all structs implementing a given trait with <100 microseconds response time
4. WHEN analyzing Rust dependencies THEN the system SHALL identify module-level dependency relationships and crate boundaries
5. WHEN checking architectural constraints THEN the system SHALL validate Rust-specific rules (e.g., "no service layer calls repository directly", "no blocking calls in async functions")
6. WHEN constraint violations are detected THEN the system SHALL provide specific Rust code locations and suggested fixes
7. WHEN validating patterns THEN the system SHALL recognize and enforce Rust idioms like newtype patterns, error propagation with ?, and proper ownership transfer

### Requirement 17

**User Story:** As a Rust developer analyzing unfamiliar codebases from code dumps, I want comprehensive support for separated dump formats with virtual file system capabilities, so that I can perform the same architectural analysis on dumped code as I do on live filesystems.

#### Acceptance Criteria

1. WHEN processing separated dump format THEN the system SHALL detect FILE: markers and parse individual .rs files from the dump with 99.3% compression (2.1MB → 15KB)
2. WHEN creating virtual file system THEN the system SHALL provide the same interface as live file monitoring for seamless integration
3. WHEN processing large dumps THEN the system SHALL support streaming mode for dumps larger than available RAM without performance degradation
4. WHEN querying dump interfaces THEN the system SHALL maintain identical performance to live mode (<100μs for trait implementations, <500μs for blast-radius)
5. WHEN extracting from dumps THEN the system SHALL complete processing in <60 seconds for dumps up to 500MB using efficient Rust parsing
6. WHEN detecting dump formats THEN the system SHALL support separated (FILE: markers), concatenated, archive (tar.gz/zip), and git bundle formats
7. WHEN generating metadata THEN the system SHALL extract project information, file counts, language detection, and file tree structure from dumps
8. WHEN providing CLI access THEN the system SHALL support `aim extract-dump`, `aim query-dump`, and `aim dump-context` commands for complete dump workflow

### Requirement 18

**User Story:** As a Rust developer working with complex production codebases, I want optimized parsing strategies that handle real-world Rust complexity patterns efficiently, so that the daemon can process enterprise-scale Rust code with trait-heavy architectures and complex generics within performance targets.

#### Acceptance Criteria

1. WHEN parsing complex generic constraints THEN the system SHALL handle multiple generic parameters, where clauses, and lifetime constraints using `syn` crate with 85-90% pattern coverage
2. WHEN extracting trait implementations THEN the system SHALL process trait objects (Box<dyn Trait<Generic>>), complex bounds, and associated types with perfect accuracy
3. WHEN handling async patterns THEN the system SHALL extract async functions, async closures, and Future types with no performance impact over sync functions
4. WHEN processing function pointers THEN the system SHALL extract function signatures as struct fields and generic parameters in function types
5. WHEN scaling to enterprise codebases THEN the system SHALL maintain performance targets: 10K LOC (0.5-1.5s), 50K LOC (2-6s), 200K LOC (8-20s), 500K LOC (20-60s)
6. WHEN encountering complex patterns THEN the system SHALL use 80/20 approach: 80% pure `syn` parsing, 20% selective `rustdoc` JSON for edge cases
7. WHEN processing trait-heavy codebases THEN the system SHALL leverage Rust's explicit interface relationships for superior architectural intelligence extraction
8. WHEN updating complex files THEN the system SHALL maintain 3-12ms incremental update latency even for files with deep generic nesting and complex trait bounds

## MVP 1.0 Scope

The above 18 requirements represent the **MVP 1.0 scope** for Parseltongue AIM Daemon. Additional advanced features have been moved to the [backlog](./backlog.md) to ensure focused delivery of core functionality.

### MVP 1.0 Success Criteria

1. **Core Functionality**: Extract architectural intelligence from Rust codebases using `syn` crate
2. **Real-time Updates**: <12ms latency from file save to query readiness
3. **Essential Queries**: Support who-implements, blast-radius, find-cycles, and generate-context
4. **LLM Integration**: Provide compressed context for AI tools with 95%+ token reduction
5. **Code Dump Support**: Handle separated dump format for analyzing unfamiliar codebases
6. **CLI Interface**: Terminal commands optimized for developer and LLM usage
7. **Performance**: Handle up to 100K LOC Rust projects with sub-millisecond query responses
8. **Reliability**: Robust error handling and graceful degradation

### Post-MVP Features

Advanced features including lock-free data structures, plugin architecture, intelligent file discovery, advanced hashing algorithms, multi-source merging, and enterprise-scale optimizations have been moved to the [backlog](./backlog.md) for future releases.