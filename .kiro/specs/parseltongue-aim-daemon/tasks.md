# Implementation Plan

Convert the feature design into a series of prompts for a code-generation LLM that will implement each step in a test-driven manner. Prioritize best practices, incremental progress, and early testing, ensuring no big jumps in complexity at any stage. Make sure that each prompt builds on the previous prompts, and ends with wiring things together. There should be no hanging or orphaned code that isn't integrated into a previous step. Focus ONLY on tasks that involve writing, modifying, or testing code.

## Phase 1: Core Data Models and Traits (Week 1)

- [ ] 1. Set up project structure and core data types
  - Create Cargo.toml with all required dependencies (tokio, syn, petgraph, etc.)
  - Implement SigHash with 128-bit collision-resistant hashing
  - Create NodeData, EdgeData, and related enums with memory layout validation tests
  - Write test_node_data_memory_layout() and test_sighash_collision_resistance()
  - _Requirements: REQ-MVP-006.0 (memory efficiency), REQ-MVP-001.0 (node types)_

- [ ] 1.1 Implement string interning system
  - Create InternedString type with thread-local string interner
  - Implement StringInterner with FxHashSet for deduplication
  - Write tests validating string interning efficiency (30-50% memory reduction)
  - Add test_string_interning_works() to verify pointer equality for identical strings
  - _Requirements: REQ-MVP-006.0 (memory optimization)_

- [ ] 1.2 Create complex Rust signature support
  - Implement RustSignature, GenericParams, WhereClause, and TraitBound types
  - Handle complex generics like `impl<H, S> ErasedIntoRoute<S, Infallible> for MakeErasedHandler<H, S>`
  - Write test_complex_generic_signature_parsing() with real Rust code examples
  - Ensure signature parsing handles where clauses and trait bounds correctly
  - _Requirements: REQ-MVP-001.0 (Rust interface signatures)_

- [ ] 1.3 Define core provider traits
  - Create PersistenceProvider, FileMonitorProvider, QueryProvider, and CliProvider traits
  - Define async trait methods with proper error types
  - Implement basic mock versions (MockPersistence, MockFileMonitor, etc.)
  - Write tests validating mock implementations work correctly
  - _Requirements: REQ-MVP-001.0 through REQ-MVP-007.0 (all core functionality)_

## Phase 2: Graph Storage and Persistence (Week 2)

- [ ] 2. Implement versioned graph storage with lock-free reads
  - Create VersionedGraphStorage with AtomicU64 versioning and DashMap for snapshots
  - Implement single-writer pattern with mpsc channel for updates
  - Write GraphSnapshot type with immutable graph data
  - Add test_concurrent_read_write_safety() with stress testing (10 writers, 20 readers)
  - _Requirements: REQ-MVP-006.0 (sub-millisecond queries, thread safety)_

- [ ] 2.1 Build GraphStorageProvider implementation
  - Implement all GraphStorageProvider trait methods (add_node, get_node, etc.)
  - Create efficient bulk operations (add_nodes_batch, add_edges_batch)
  - Write property-based tests using proptest for graph invariants
  - Add test_graph_invariants_hold() validating edge consistency and node counts
  - _Requirements: REQ-MVP-003.0 (graph queries), REQ-MVP-006.0 (performance)_

- [ ] 2.2 Implement SQLite persistence with WAL
  - Create SqlitePersistence with connection pooling and migrations
  - Implement save_snapshot() and load_snapshot() with batched inserts
  - Add WriteAheadLog for crash recovery with operation replay
  - Write test_snapshot_save_load_roundtrip() and test_wal_crash_recovery()
  - _Requirements: REQ-MVP-006.0 (persistence), REQ-MVP-007.0 (crash recovery)_

- [ ] 2.3 Add performance validation for storage operations
  - Write test_storage_performance_contracts() validating <500ms snapshot loading
  - Implement test_query_performance_contracts() ensuring <500μs complex queries
  - Add memory pressure testing and cleanup validation
  - Verify storage operations meet all performance requirements
  - _Requirements: REQ-MVP-006.0 (performance targets)_

## Phase 3: Query Engine and File Monitoring (Week 3)

- [ ] 3. Create bounded query execution engine
  - Implement GraphQueryProcessor with circuit breaker and timeout enforcement
  - Create specific executors: WhatImplementsExecutor, BlastRadiusExecutor, CycleDetectionExecutor
  - Add performance monitoring and bounded execution (max nodes, max depth, timeouts)
  - Write test_blast_radius_performance_bound() validating <500μs execution time
  - _Requirements: REQ-MVP-003.0 (essential queries), REQ-MVP-006.0 (performance)_

- [ ] 3.1 Implement what-implements and blast-radius queries
  - Create execute_what_implements() using edge filtering for Implements relationships
  - Implement execute_blast_radius() with BFS traversal and depth limits
  - Add execute_find_cycles() using Tarjan's algorithm for cycle detection
  - Write comprehensive tests for each query type with performance validation
  - _Requirements: REQ-MVP-003.0 (what-implements, blast-radius, find-cycles)_

- [ ] 3.2 Build file monitoring with debouncing
  - Implement NotifyFileMonitor with RecommendedWatcher and RAII cleanup
  - Create EventDebouncer with 10ms batching window to prevent ISG thrashing
  - Add file filtering for .rs files only and parallel parsing pool
  - Write test_file_change_debouncing() and test_batch_update_processing()
  - _Requirements: REQ-MVP-002.0 (live monitoring), REQ-MVP-006.0 (<12ms updates)_

- [ ] 3.3 Integrate file changes with graph updates
  - Connect file change events to graph storage updates via update channels
  - Implement incremental graph updates for modified files
  - Add test_file_change_to_query_latency() validating end-to-end <12ms constraint
  - Ensure file monitoring integrates seamlessly with versioned storage
  - _Requirements: REQ-MVP-002.0 (real-time updates), REQ-MVP-006.0 (performance)_

## Phase 4: CLI and Rust Parser Integration (Week 4)

- [ ] 4. Implement CLI command processing
  - Create ClapCliHandler with command routing and structured error handling
  - Implement individual command handlers: IngestCommand, DaemonCommand, QueryCommand, ContextCommand
  - Add comprehensive error formatting with actionable suggestions
  - Write test_cli_command_routing() and test_error_message_formatting()
  - _Requirements: REQ-MVP-005.0 (CLI interface), REQ-MVP-007.0 (error handling)_

- [ ] 4.1 Build code dump parser with FILE: marker support
  - Create CodeDumpParser handling separated dump format with FILE: markers
  - Implement parse_separated_format() extracting individual files from dumps
  - Add progress reporting and error handling for malformed dumps
  - Write test_code_dump_parsing_with_file_markers() using real dump examples
  - _Requirements: REQ-MVP-001.0 (code dump ingestion)_

- [ ] 4.2 Integrate syn crate for Rust AST parsing
  - Create RustParser using syn crate for high-fidelity Rust parsing
  - Implement extract_nodes_from_ast() converting syn::Item to NodeData
  - Handle functions, structs, traits, impls, and modules with proper relationships
  - Write test_syn_integration_with_complex_rust() using real Rust code samples
  - _Requirements: REQ-MVP-001.0 (syn crate parsing), REQ-MVP-002.0 (live parsing)_

- [ ] 4.3 Implement LLM context generation
  - Create ContextGenerator with ISG slice extraction for entities
  - Implement bounded context generation (max 2 hops, token limits)
  - Add structured output formatting for LLM consumption (JSON and human-readable)
  - Write test_llm_context_generation() validating deterministic output
  - _Requirements: REQ-MVP-004.0 (LLM context generation)_

## Phase 5: Production Integration and End-to-End Testing (Week 5)

- [ ] 5. Wire together production daemon system
  - Create ProductionDaemon type with all concrete implementations
  - Implement DaemonSystem startup, shutdown, and signal handling
  - Add graceful shutdown with resource cleanup and state persistence
  - Write test_production_daemon_startup() and test_graceful_shutdown()
  - _Requirements: REQ-MVP-002.0 (daemon operation), REQ-MVP-007.0 (graceful shutdown)_

- [ ] 5.1 Add comprehensive end-to-end testing
  - Create test_end_to_end_code_dump_processing() with real 2.1MB Rust dump
  - Implement test_live_file_monitoring_workflow() with temporary directories
  - Add test_query_after_file_modification() validating complete workflow
  - Write test_crash_recovery_integration() ensuring data persistence
  - _Requirements: REQ-MVP-001.0 through REQ-MVP-007.0 (complete system validation)_

- [ ] 5.2 Performance validation against all requirements
  - Create test_end_to_end_performance_requirements() validating all timing constraints
  - Test 2.1MB code dump processing in <5 seconds (REQ-MVP-001.0)
  - Validate <12ms file update to query readiness (REQ-MVP-002.0)
  - Verify <1ms simple queries and <500μs complex queries (REQ-MVP-003.0)
  - _Requirements: All performance requirements validation_

- [ ] 5.3 Create main binary and CLI entry point
  - Implement main.rs with clap argument parsing and command dispatch
  - Add proper logging configuration and error reporting
  - Create help text and usage examples for all commands
  - Write integration tests using the actual binary with real file systems
  - _Requirements: REQ-MVP-005.0 (CLI interface), REQ-MVP-007.0 (error handling)_

## Success Criteria Validation

Each completed phase must demonstrate:

1. **All tests pass**: Unit tests, integration tests, and performance tests
2. **Performance requirements met**: Validated through automated testing
3. **Memory constraints satisfied**: <25MB for 100K LOC validated
4. **Error handling works**: Graceful failure and recovery demonstrated
5. **TDD principles followed**: Tests written first, driving implementation
6. **Rust idioms applied**: RAII, structured errors, zero-cost abstractions
7. **Requirements coverage**: Each task explicitly references requirements

The implementation plan ensures incremental progress with no orphaned code, comprehensive testing at each step, and validation of all performance claims through automated tests.