# Implementation Plan - Parseltongue Architect v2.0

This implementation plan follows the **STUB → RED → GREEN → REFACTOR** TDD cycle for each component, ensuring tests drive the design and validate all performance contracts.

## Phase 1: Foundation (Data Structures, Hashing, Indexing)

- [ ] 1. Set up project structure and core interfaces
  - Create Cargo workspace with `parseltongue-core`, `parseltongue-cli`, and `parseltongue-daemon` crates
  - Define core trait interfaces that establish system boundaries
  - Set up test infrastructure with `cargo-nextest` for fast test execution
  - Configure CI pipeline with performance regression detection
  - _Requirements: REQ-V2-007.0 (Production-Ready Daemon)_

- [ ] 1.1 Implement SigHash with deterministic identification
  - **STUB**: Write failing test for `SigHash::from_fqn()` cross-platform consistency
  - **RED**: Test fails because SigHash doesn't exist
  - **GREEN**: Implement SigHash with FxHasher, make test pass
  - **REFACTOR**: Optimize hash computation, add serialization support
  - Write property-based tests for hash collision resistance
  - Validate deterministic behavior across Linux/macOS/Windows
  - _Requirements: REQ-V2-003.0 (Deterministic Identification System)_

- [ ] 1.2 Create NodeData with memory-optimized storage
  - **STUB**: Write failing test for NodeData creation and equality
  - **RED**: Test fails because NodeData doesn't exist
  - **GREEN**: Implement NodeData with Arc<str> string interning
  - **REFACTOR**: Optimize memory layout, validate size constraints
  - Test memory usage with 10K+ nodes, ensure <25MB at 100K LOC
  - Implement Debug, Clone, PartialEq traits with comprehensive tests
  - _Requirements: REQ-V2-007.0 (Production-Ready Daemon - Memory Efficiency)_

- [ ] 1.3 Implement ISGState with O(1) indexed operations
  - **STUB**: Write failing tests for all three indexes (id_map, name_map, file_index)
  - **RED**: Tests fail because ISGState doesn't exist
  - **GREEN**: Implement ISGState with FxHashMap indices and StableDiGraph
  - **REFACTOR**: Optimize index maintenance, ensure atomic updates
  - Write performance contract tests: <50μs for get/insert operations
  - Test concurrent access patterns with stress testing
  - _Requirements: REQ-V2-002.0 (O(1) Performance Guarantees)_

## Phase 2: The Engine (Ingestion and Extraction)

- [ ] 2. Implement two-pass ingestion framework
  - **STUB**: Write failing test for two-pass ingestion coordinator
  - **RED**: Test fails because ingestion framework doesn't exist
  - **GREEN**: Implement IngestionManager with Pass1 and Pass2 execution
  - **REFACTOR**: Add error recovery, progress tracking, and cancellation
  - Test with complex Rust files containing forward references
  - Validate that Pass 2 can resolve all entities created in Pass 1
  - _Requirements: REQ-V2-004.0 (Two-Pass Ingestion Architecture)_

- [ ] 2.1 Implement Pass 1: Node extraction with FQN tracking
  - **STUB**: Write failing tests for FQN generation in nested modules
  - **RED**: Tests fail because ModuleContext doesn't exist
  - **GREEN**: Implement ModuleContext with proper scope tracking
  - **REFACTOR**: Handle edge cases (anonymous modules, re-exports)
  - Test with real Rust codebases: tokio, serde, clap
  - Validate FQN uniqueness and consistency across files
  - _Requirements: REQ-V2-003.0 (Deterministic Identification), REQ-V2-004.0 (Two-Pass Ingestion)_

- [ ] 2.2 Implement basic syn parser for node extraction
  - **STUB**: Write failing tests for extracting functions, structs, traits, enums
  - **RED**: Tests fail because syn integration doesn't exist
  - **GREEN**: Implement syn::parse_file integration with Item matching
  - **REFACTOR**: Add error handling for malformed Rust code
  - Test parsing performance: <5ms for 1000-line files
  - Handle syn parse errors gracefully, continue processing other files
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction - Foundation)_

- [ ] 2.3 Implement DOT export for debug visualization
  - **STUB**: Write failing test for DOT format generation
  - **RED**: Test fails because DOT export doesn't exist
  - **GREEN**: Implement petgraph::dot::Dot integration with custom formatting
  - **REFACTOR**: Add node/edge styling, improve readability
  - Test DOT output with Graphviz rendering validation
  - Essential for debugging relationship extraction in next phase
  - _Requirements: REQ-V2-010.0 (Debug Visualization Export)_

- [ ] 2.4 Implement Pass 2: Relationship extraction with syn::visit::Visit
  - **STUB**: Write failing tests for CALLS, USES, IMPLEMENTS relationship detection
  - **RED**: Tests fail because RelationshipExtractor doesn't exist
  - **GREEN**: Implement syn::visit::Visit with visit_expr_call, visit_type_path, visit_item_impl
  - **REFACTOR**: Add method call resolution, improve accuracy for complex patterns
  - Test relationship extraction accuracy: target 95%+ on real codebases
  - Use DOT export to validate extracted relationships visually
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction)_

- [ ] 2.5 Validate relationship extraction accuracy with comprehensive tests
  - **STUB**: Write failing tests for complex relationship patterns
  - **RED**: Tests fail because accuracy is below 95%
  - **GREEN**: Improve relationship resolution to achieve 95%+ accuracy
  - **REFACTOR**: Handle edge cases, add warning logs for unresolved patterns
  - Test with trait objects, generic functions, macro-generated code
  - Implement the "95% rule": log warnings for complex constructs, don't block progress
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction)_

## Phase 3: Application Layer

- [ ] 3. Implement core query engine with performance contracts
  - **STUB**: Write failing performance tests for <1ms query response times
  - **RED**: Tests fail because query engine doesn't exist
  - **GREEN**: Implement blast-radius, what-implements, calls, uses queries
  - **REFACTOR**: Optimize graph traversal algorithms, add query result caching
  - Test query performance with 10K+ node graphs
  - Implement performance violation detection and reporting
  - _Requirements: REQ-V2-005.0 (Core Query Engine)_

- [ ] 3.1 Implement blast-radius query with bounded BFS
  - **STUB**: Write failing test for blast-radius calculation with depth limits
  - **RED**: Test fails because blast-radius query doesn't exist
  - **GREEN**: Implement BFS traversal with max_depth parameter
  - **REFACTOR**: Add early termination, optimize memory usage
  - Test performance contract: <1ms for 3-hop traversal on 10K nodes
  - Validate correctness with known dependency chains
  - _Requirements: REQ-V2-005.0 (Core Query Engine - blast-radius)_

- [ ] 3.2 Implement what-implements query for trait analysis
  - **STUB**: Write failing test for finding all trait implementors
  - **RED**: Test fails because what-implements query doesn't exist
  - **GREEN**: Implement reverse edge traversal for IMPLEMENTS relationships
  - **REFACTOR**: Add filtering, sorting by relevance
  - Test performance contract: <500μs for trait lookup
  - Validate with complex trait hierarchies and generic implementations
  - _Requirements: REQ-V2-005.0 (Core Query Engine - what-implements)_

- [ ] 3.3 Implement calls and uses queries for dependency analysis
  - **STUB**: Write failing tests for finding callers and type users
  - **RED**: Tests fail because calls/uses queries don't exist
  - **GREEN**: Implement edge filtering by EdgeKind (CALLS, USES)
  - **REFACTOR**: Add result ranking, performance optimization
  - Test with high-connectivity nodes (central functions, common types)
  - Validate accuracy against manual code analysis
  - _Requirements: REQ-V2-005.0 (Core Query Engine - calls, uses)_

- [ ] 4. Implement real-time daemon with file monitoring
  - **STUB**: Write failing test for <12ms file update latency
  - **RED**: Test fails because daemon doesn't exist
  - **GREEN**: Implement notify-based file watcher with incremental updates
  - **REFACTOR**: Add debouncing, batch processing, error recovery
  - Test with rapid file changes, large file modifications
  - Validate memory stability during continuous operation
  - _Requirements: REQ-V2-009.0 (Real-Time Daemon Integration)_

- [ ] 4.1 Implement incremental file updates with O(1) removal
  - **STUB**: Write failing test for O(1) file-based node removal
  - **RED**: Test fails because file_index isn't used for removal
  - **GREEN**: Implement remove_nodes_from_file using file_index
  - **REFACTOR**: Optimize index maintenance, ensure consistency
  - Test update performance: <12ms for 1000-line file changes
  - Validate graph consistency after incremental updates
  - _Requirements: REQ-V2-009.0 (Real-Time Daemon Integration - Incremental Updates)_

- [ ] 4.2 Implement daemon lifecycle management
  - **STUB**: Write failing tests for daemon startup, shutdown, error recovery
  - **RED**: Tests fail because lifecycle management doesn't exist
  - **GREEN**: Implement graceful startup/shutdown with resource cleanup
  - **REFACTOR**: Add health monitoring, automatic restart on failures
  - Test daemon stability over 24+ hour runs
  - Validate resource cleanup on shutdown (no memory leaks)
  - _Requirements: REQ-V2-007.0 (Production-Ready Daemon)_

- [ ] 5. Implement CLI interface with performance metrics
  - **STUB**: Write failing tests for CLI command parsing and execution
  - **RED**: Tests fail because CLI doesn't exist
  - **GREEN**: Implement clap-based CLI with query, daemon, context commands
  - **REFACTOR**: Add progress indicators, colored output, error formatting
  - Test CLI usability with real developer workflows
  - Validate performance metric reporting accuracy
  - _Requirements: REQ-V2-006.0 (Basic CLI Interface)_

- [ ] 5.1 Implement query commands with performance reporting
  - **STUB**: Write failing tests for blast-radius, what-implements CLI commands
  - **RED**: Tests fail because query commands don't exist
  - **GREEN**: Implement CLI query execution with timing and result formatting
  - **REFACTOR**: Add output formats (JSON, table, compact), result filtering
  - Test CLI performance reporting matches internal metrics
  - Validate command-line argument parsing and validation
  - _Requirements: REQ-V2-006.0 (Basic CLI Interface - Query Commands)_

- [ ] 5.2 Implement daemon control commands
  - **STUB**: Write failing tests for daemon start, stop, status commands
  - **RED**: Tests fail because daemon commands don't exist
  - **GREEN**: Implement daemon process management and communication
  - **REFACTOR**: Add daemon health checks, configuration management
  - Test daemon communication reliability and error handling
  - Validate daemon process isolation and resource management
  - _Requirements: REQ-V2-006.0 (Basic CLI Interface - Daemon Control)_

- [ ] 6. Implement basic LLM context generation
  - **STUB**: Write failing test for 1-hop context generation with <100ms latency
  - **RED**: Test fails because context generation doesn't exist
  - **GREEN**: Implement LlmContext with dependencies, callers, implementations
  - **REFACTOR**: Add multiple output formats, architectural role classification
  - Test context quality with manual validation on real codebases
  - Validate performance contract: <100ms for context generation
  - _Requirements: REQ-V2-008.0 (Basic LLM Context Generation)_

- [ ] 6.1 Implement compressed context format for token efficiency
  - **STUB**: Write failing test for compressed context format generation
  - **RED**: Test fails because format_compressed doesn't exist
  - **GREEN**: Implement token-efficient context formatting
  - **REFACTOR**: Optimize for LLM context windows, add relevance ranking
  - Test context usefulness with actual LLM interactions
  - Validate token count optimization vs. information density
  - _Requirements: REQ-V2-008.0 (Basic LLM Context Generation - Compressed Format)_

## Phase 4: Visualization and Polish

- [ ] 7. Implement interactive HTML visualization
  - **STUB**: Write failing test for self-contained HTML generation with <500ms latency
  - **RED**: Test fails because HTML visualization doesn't exist
  - **GREEN**: Implement HTML template with embedded JavaScript visualization
  - **REFACTOR**: Add interactivity, improve visual layout, optimize performance
  - Test HTML generation performance and browser compatibility
  - Validate self-contained nature (no external dependencies)
  - _Requirements: REQ-V2-011.0 (Interactive HTML Visualization)_

- [ ] 7.1 Implement graph data serialization for web visualization
  - **STUB**: Write failing test for JSON graph data export
  - **RED**: Test fails because graph serialization doesn't exist
  - **GREEN**: Implement graph-to-JSON conversion for web rendering
  - **REFACTOR**: Optimize JSON size, add filtering for large graphs
  - Test with large graphs (1000+ nodes), ensure browser performance
  - Validate JSON structure compatibility with visualization libraries
  - _Requirements: REQ-V2-011.0 (Interactive HTML Visualization - Data Export)_

- [ ] 8. Implement production-ready error handling and persistence
  - **STUB**: Write failing tests for graceful error recovery and data persistence
  - **RED**: Tests fail because production features don't exist
  - **GREEN**: Implement robust error handling, incremental snapshots
  - **REFACTOR**: Add monitoring, logging, performance optimization
  - Test error recovery scenarios: corrupted files, out-of-memory, disk full
  - Validate snapshot consistency and recovery reliability
  - _Requirements: REQ-V2-007.0 (Production-Ready Daemon)_

- [ ] 8.1 Implement incremental snapshot persistence
  - **STUB**: Write failing test for atomic snapshot save/load with <500ms latency
  - **RED**: Test fails because snapshot persistence doesn't exist
  - **GREEN**: Implement serde-based serialization with atomic file operations
  - **REFACTOR**: Add compression, incremental updates, corruption detection
  - Test snapshot performance and reliability under load
  - Validate startup time improvement with persistent snapshots
  - _Requirements: REQ-V2-007.0 (Production-Ready Daemon - Persistence)_

- [ ] 8.2 Implement comprehensive error handling with graceful degradation
  - **STUB**: Write failing tests for error scenarios and recovery behavior
  - **RED**: Tests fail because error handling is incomplete
  - **GREEN**: Implement ISGError hierarchy with contextual error information
  - **REFACTOR**: Add error recovery strategies, user-friendly error messages
  - Test error handling with malformed input, resource exhaustion, network issues
  - Validate graceful degradation: continue processing despite individual file errors
  - _Requirements: REQ-V2-007.0 (Production-Ready Daemon - Error Handling)_

## Phase 5: Integration and Validation

- [ ] 9. Implement end-to-end integration tests
  - **STUB**: Write failing tests for complete user workflows
  - **RED**: Tests fail because integration isn't complete
  - **GREEN**: Implement full workflow tests: ingest → query → update → visualize
  - **REFACTOR**: Add performance regression tests, cross-platform validation
  - Test with real Rust projects: tokio, serde, clap, bevy
  - Validate all performance contracts under realistic workloads
  - _Requirements: All requirements integrated_

- [ ] 9.1 Validate performance contracts with realistic workloads
  - **STUB**: Write failing tests for performance contracts on 100K+ LOC codebases
  - **RED**: Tests fail because performance targets aren't met
  - **GREEN**: Optimize bottlenecks to meet all performance contracts
  - **REFACTOR**: Add performance monitoring, regression detection
  - Test memory usage, query latency, update speed on large codebases
  - Validate cross-platform consistency (Linux, macOS, Windows)
  - _Requirements: REQ-V2-002.0 (O(1) Performance Guarantees), REQ-V2-009.0 (Real-Time Integration)_

- [ ] 9.2 Implement comprehensive documentation and examples
  - **STUB**: Write failing tests for documentation completeness and accuracy
  - **RED**: Tests fail because documentation doesn't exist
  - **GREEN**: Implement API documentation, CLI help, usage examples
  - **REFACTOR**: Add tutorials, troubleshooting guides, performance tuning tips
  - Test documentation with new users, validate example accuracy
  - Validate CLI help text and error message clarity
  - _Requirements: REQ-V2-006.0 (Basic CLI Interface - Usability)_

## Success Criteria Validation

Each task must pass these validation criteria before being marked complete:

### ✅ **Functional Validation**
- [ ] All tests pass (unit, integration, property-based)
- [ ] 95%+ relationship extraction accuracy on real Rust codebases
- [ ] All CLI commands work correctly with proper error handling
- [ ] DOT and HTML visualizations render correctly

### ✅ **Performance Validation**
- [ ] <1ms query response times (blast-radius, what-implements, calls, uses)
- [ ] <12ms file update latency for incremental changes
- [ ] <50μs node operations (get, insert, lookup)
- [ ] <25MB memory usage at 100K LOC

### ✅ **Reliability Validation**
- [ ] Graceful error handling for all failure scenarios
- [ ] Cross-platform consistency (Linux, macOS, Windows)
- [ ] 24+ hour daemon stability testing
- [ ] Automatic recovery from corrupted state

### ✅ **Usability Validation**
- [ ] Clean CLI interface with helpful error messages
- [ ] Debug visualization aids development and troubleshooting
- [ ] LLM context generation provides useful architectural insights
- [ ] Documentation enables new users to get started quickly

## TDD Discipline Enforcement

**Every task follows STUB → RED → GREEN → REFACTOR:**

1. **STUB**: Write the test interface and expected behavior
2. **RED**: Run tests, verify they fail for the right reasons
3. **GREEN**: Implement minimal code to make tests pass
4. **REFACTOR**: Optimize, clean up, improve without breaking tests

**Performance contracts are tested first:**
- Write performance tests before implementing functionality
- Use `std::time::Instant` for precise timing measurements
- Fail fast if performance contracts are violated
- Optimize only after correctness is established

**No implementation without tests:**
- Every public function has comprehensive test coverage
- Property-based tests validate invariants across input space
- Integration tests validate end-to-end workflows
- Performance tests validate all timing contracts

This plan ensures that Parseltongue Architect v2.0 delivers a reliable, high-performance foundation for architectural intelligence while maintaining strict adherence to TDD principles and the 30-day delivery timeline.