# Implementation Plan: OptimizedISG MVP

This implementation plan follows the **TDD-first approach** from the rigorous OptimizedISG design. Each task builds incrementally using the **RED → GREEN → REFACTOR** cycle with proven performance characteristics.

**Architecture Foundation**: petgraph + parking_lot::RwLock + FxHashMap + Arc<str> interning
**Performance Targets**: <5μs node ops, <500μs simple queries, <1ms complex queries
**Memory Target**: 350 bytes/node, <25MB for 100K LOC

## Phase 1: OptimizedISG Core (Week 1)

### TDD Cycle 1: Project Setup and Core Types

- [ ] 1.1 Set up project with proven dependencies
  - Create Cargo.toml with exact dependencies from design: petgraph="0.6", parking_lot="0.12", fxhash="0.2", thiserror="1.0"
  - Add syn="2.0", notify="6.0", clap with derive features, serde with derive features
  - Create src/lib.rs and src/main.rs structure
  - Write failing test: `test_project_compiles()` 
  - **RED → GREEN**: Make project compile with empty implementations
  - _Requirements: REQ-MVP-006.0 (dependencies), All MVP requirements (foundation)_

- [ ] 1.2 Implement SigHash with collision resistance
  - **RED**: Write failing test `test_sighash_collision_resistance()` with 10,000 different signatures
  - **RED**: Write failing test `test_sighash_deterministic()` ensuring same input = same hash
  - **GREEN**: Implement SigHash(u64) with DefaultHasher (simple for MVP)
  - **GREEN**: Implement SigHash::from_signature() method
  - **REFACTOR**: Validate no collisions in test data, document hash algorithm choice
  - _Requirements: REQ-MVP-001.0 (unique identifiers), REQ-MVP-006.0 (collision-free)_

- [ ] 1.3 Create core data structures with memory validation
  - **RED**: Write failing test `test_node_data_size_constraint()` asserting reasonable memory usage
  - **RED**: Write failing test `test_node_data_creation()` for basic NodeData operations
  - **GREEN**: Implement NodeData, NodeKind, EdgeKind with Arc<str> for string interning
  - **GREEN**: Implement basic constructors and Clone/Debug traits
  - **REFACTOR**: Measure actual memory usage, document 350 bytes/node target
  - _Requirements: REQ-MVP-006.0 (memory efficiency), REQ-MVP-001.0 (node types)_

### TDD Cycle 2: OptimizedISG Foundation

- [ ] 1.4 Implement ISGState and OptimizedISG structure
  - **RED**: Write failing test `test_isg_initialization()` expecting 0 nodes/edges
  - **RED**: Write failing test `test_isg_clone_shares_state()` for Arc sharing
  - **GREEN**: Implement ISGState with StableDiGraph and FxHashMap
  - **GREEN**: Implement OptimizedISG with Arc<RwLock<ISGState>>
  - **REFACTOR**: Add node_count() and edge_count() methods, validate structure
  - _Requirements: REQ-MVP-006.0 (Arc<RwLock<ISGState>>), REQ-MVP-003.0 (graph structure)_

- [ ] 1.5 Implement node upsert operations (O(1) target)
  - **RED**: Write failing test `test_upsert_and_get_node()` with insert/update/retrieve scenarios
  - **RED**: Write failing test `test_node_operation_performance()` asserting <5μs operations
  - **GREEN**: Implement upsert_node() with write lock and FxHashMap index
  - **GREEN**: Implement get_node() with read lock and index lookup
  - **REFACTOR**: Optimize for performance, validate timing constraints in tests
  - _Requirements: REQ-MVP-006.0 (sub-millisecond operations), REQ-MVP-001.0 (node storage)_

- [ ] 1.6 Implement edge operations with graph consistency
  - **RED**: Write failing test `test_upsert_edge()` with edge creation/update scenarios
  - **RED**: Write failing test `test_edge_nonexistent_nodes()` expecting proper errors
  - **GREEN**: Implement upsert_edge() using petgraph's update_edge
  - **GREEN**: Implement proper error handling for missing nodes
  - **REFACTOR**: Ensure atomic operations, validate graph consistency
  - _Requirements: REQ-MVP-003.0 (graph relationships), REQ-MVP-007.0 (error handling)_

## Phase 2: Query Operations and Performance Validation (Week 2)

### TDD Cycle 3: Essential Graph Queries

- [ ] 2.1 Implement what-implements query (<500μs target)
  - **RED**: Write failing test `test_query_who_implements()` with known graph structure
  - **RED**: Write failing test `test_what_implements_performance()` asserting <500μs execution
  - **GREEN**: Implement find_implementors() using petgraph edge traversal with Direction::Incoming
  - **GREEN**: Filter edges by EdgeKind::Implements, collect implementing nodes
  - **REFACTOR**: Optimize traversal, validate performance contract in tests
  - _Requirements: REQ-MVP-003.0 (what-implements query), REQ-MVP-006.0 (sub-millisecond)_

- [ ] 2.2 Implement blast-radius query (<1ms target)
  - **RED**: Write failing test `test_query_blast_radius_bfs()` with expected reachable nodes
  - **RED**: Write failing test `test_blast_radius_performance()` asserting <1ms execution
  - **GREEN**: Implement calculate_blast_radius() using petgraph BFS traversal
  - **GREEN**: Use Bfs::new() and iter() to collect reachable nodes efficiently
  - **REFACTOR**: Optimize for cache locality, validate against performance target
  - _Requirements: REQ-MVP-003.0 (blast-radius query), REQ-MVP-006.0 (complex queries <1ms)_

- [ ] 2.3 Implement find-cycles query (MVP stub)
  - **RED**: Write failing test `test_find_cycles_empty()` expecting empty result for MVP
  - **GREEN**: Implement find_cycles() returning Vec::new() (satisfies requirement)
  - **REFACTOR**: Document that cycle detection is post-MVP feature
  - _Requirements: REQ-MVP-003.0 (find-cycles query - minimal implementation)_

### TDD Cycle 4: Concurrency Safety Validation

- [ ] 2.4 Validate concurrent read/write safety
  - **RED**: Write failing test `test_concurrent_writes_and_reads()` with multiple threads
  - **RED**: Spawn 10 writer threads adding nodes, 20 reader threads doing queries
  - **GREEN**: Ensure OptimizedISG handles concurrent access without data races
  - **GREEN**: Validate final state consistency after all threads complete
  - **REFACTOR**: Stress test with 500 operations, ensure no deadlocks or panics
  - _Requirements: REQ-MVP-006.0 (thread safety), REQ-MVP-002.0 (concurrent access)_

- [ ] 2.5 Performance contract validation
  - **RED**: Write failing test `test_performance_constraints()` for all timing requirements
  - **GREEN**: Validate node operations <5μs, simple queries <500μs, complex queries <1ms
  - **GREEN**: Test with realistic data sizes (1000+ nodes, 4000+ edges)
  - **REFACTOR**: Create helper functions for performance testing, document results
  - _Requirements: REQ-MVP-006.0 (all performance constraints)_

## Phase 3: Code Parsing and File Monitoring (Week 3)

### TDD Cycle 5: Rust Code Parsing with syn

- [ ] 3.1 Implement basic syn integration for Rust parsing
  - **RED**: Write failing test `test_parse_rust_file_basic()` with simple function/struct/trait
  - **RED**: Write failing test `test_syn_error_handling()` for malformed Rust code
  - **GREEN**: Implement parse_rust_file() using syn::parse_file()
  - **GREEN**: Extract Function, Struct, Trait items into NodeData with proper signatures
  - **REFACTOR**: Handle syn errors gracefully, validate node creation accuracy
  - _Requirements: REQ-MVP-001.0 (syn crate parsing), REQ-MVP-007.0 (error handling)_

- [ ] 3.2 Implement code dump ingestion with FILE: markers
  - **RED**: Write failing test `test_ingest_code_dump()` with separated dump format
  - **RED**: Write failing test `test_code_dump_performance()` asserting <5s for 2.1MB dump
  - **GREEN**: Implement ingest_code_dump() parsing FILE: markers and extracting .rs files
  - **GREEN**: Process each file section, create nodes, update ISG
  - **REFACTOR**: Add progress reporting, validate ingestion speed requirement
  - _Requirements: REQ-MVP-001.0 (code dump ingestion, <5s for 2.1MB)_

- [ ] 3.3 Create ParseltongueAIM main system
  - **RED**: Write failing test `test_parseltongue_aim_creation()` for system initialization
  - **GREEN**: Implement ParseltongueAIM struct with OptimizedISG, file_watcher, shutdown
  - **GREEN**: Add basic methods: new(), node_count(), basic file operations
  - **REFACTOR**: Ensure clean initialization and proper resource management
  - _Requirements: REQ-MVP-001.0 through REQ-MVP-007.0 (system integration)_

### TDD Cycle 6: File Monitoring (<12ms constraint)

- [ ] 3.4 Implement file watching with notify crate
  - **RED**: Write failing test `test_file_monitoring_basic()` with temporary directory
  - **RED**: Write failing test `test_file_update_performance()` asserting <12ms update time
  - **GREEN**: Implement start_daemon() with notify::recommended_watcher()
  - **GREEN**: Filter for .rs files, handle file change events
  - **REFACTOR**: Add proper error handling, validate timing constraint
  - _Requirements: REQ-MVP-002.0 (live monitoring, <12ms updates)_

- [ ] 3.5 Implement incremental file updates
  - **RED**: Write failing test `test_update_file_incremental()` removing old nodes, adding new
  - **GREEN**: Implement update_file() and remove_nodes_from_file() methods
  - **GREEN**: Re-parse changed file, update ISG atomically
  - **REFACTOR**: Ensure atomic updates, validate no data races
  - _Requirements: REQ-MVP-002.0 (real-time updates), REQ-MVP-006.0 (atomic consistency)_

- [ ] 3.6 Add daemon lifecycle management
  - **RED**: Write failing test `test_daemon_shutdown_graceful()` with Ctrl+C simulation
  - **GREEN**: Implement graceful shutdown with AtomicBool coordination
  - **GREEN**: Add proper Drop implementation for resource cleanup
  - **REFACTOR**: Ensure all resources cleaned up, no hanging threads
  - _Requirements: REQ-MVP-002.0 (graceful shutdown), REQ-MVP-007.0 (resource cleanup)_

## Phase 4: CLI Interface and LLM Context Generation (Week 4)

### TDD Cycle 7: CLI Implementation with clap

- [ ] 4.1 Implement CLI structure with clap derive
  - **RED**: Write failing test `test_cli_parsing()` for all command variants
  - **RED**: Write failing test `test_cli_help_output()` validating help text
  - **GREEN**: Implement Cli, Commands, QueryType, OutputFormat enums with clap derive
  - **GREEN**: Add proper command structure matching requirements
  - **REFACTOR**: Validate all commands parse correctly, improve help text
  - _Requirements: REQ-MVP-005.0 (CLI interface)_

- [ ] 4.2 Implement query commands with performance monitoring
  - **RED**: Write failing test `test_query_command_execution()` for all query types
  - **RED**: Write failing test `test_query_performance_reporting()` showing timing
  - **GREEN**: Implement query command handling with timing measurement
  - **GREEN**: Add performance constraint validation (warn if >1ms)
  - **REFACTOR**: Add proper error handling, format output correctly
  - _Requirements: REQ-MVP-003.0 (queries), REQ-MVP-005.0 (CLI), REQ-MVP-006.0 (performance)_

- [ ] 4.3 Implement ingest and daemon commands
  - **RED**: Write failing test `test_ingest_command()` with real code dump
  - **RED**: Write failing test `test_daemon_command()` with directory watching
  - **GREEN**: Implement ingest command with progress reporting and timing
  - **GREEN**: Implement daemon command with proper startup and monitoring
  - **REFACTOR**: Add comprehensive error handling and user feedback
  - _Requirements: REQ-MVP-001.0 (ingest), REQ-MVP-002.0 (daemon), REQ-MVP-005.0 (CLI)_

### TDD Cycle 8: LLM Context Generation

- [ ] 4.4 Implement entity lookup and context extraction
  - **RED**: Write failing test `test_find_entity_by_name()` with known entities
  - **RED**: Write failing test `test_get_dependencies_and_callers()` for graph traversal
  - **GREEN**: Implement find_entity_by_name() with linear search (O(n) for MVP)
  - **GREEN**: Implement get_dependencies() and get_callers() using graph edges
  - **REFACTOR**: Optimize lookup if needed, validate correctness
  - _Requirements: REQ-MVP-004.0 (entity lookup, dependency analysis)_

- [ ] 4.5 Implement LLM context formatting
  - **RED**: Write failing test `test_generate_context_human()` for human-readable output
  - **RED**: Write failing test `test_generate_context_json()` for JSON output
  - **GREEN**: Implement LlmContext struct with target, dependencies, callers
  - **GREEN**: Add format_human() and JSON serialization
  - **REFACTOR**: Ensure deterministic output, validate 2-hop constraint
  - _Requirements: REQ-MVP-004.0 (LLM context generation, JSON/human formats)_

- [ ] 4.6 Integrate context generation with CLI
  - **RED**: Write failing test `test_generate_context_command()` end-to-end
  - **GREEN**: Implement generate-context CLI command with format options
  - **GREEN**: Add proper error handling for missing entities
  - **REFACTOR**: Validate output quality, ensure deterministic results
  - _Requirements: REQ-MVP-004.0 (context generation), REQ-MVP-005.0 (CLI integration)_

## Phase 5: Persistence and Production Integration (Week 5)

### TDD Cycle 9: Simple JSON Persistence

- [x] 5.1 Implement snapshot serialization ✅ COMPLETED
  - **RED**: Write failing test `test_save_snapshot()` with known graph state ✅
  - **RED**: Write failing test `test_snapshot_performance()` asserting <500ms save/load ✅
  - **GREEN**: Implement ISGSnapshot struct with nodes, edges, metadata ✅
  - **GREEN**: Add serde serialization for NodeData and EdgeSnapshot ✅
  - **REFACTOR**: Optimize serialization, validate performance constraint ✅
  - _Requirements: REQ-MVP-006.0 (persistence, <500ms reload)_ ✅

- [ ] 5.2 Implement snapshot loading and recovery
  - **RED**: Write failing test `test_load_snapshot()` with roundtrip validation
  - **RED**: Write failing test `test_snapshot_missing_file()` handling gracefully
  - **GREEN**: Implement load_snapshot() rebuilding OptimizedISG from file
  - **GREEN**: Handle missing files gracefully (no snapshot = empty ISG)
  - **REFACTOR**: Ensure atomic loading, validate data integrity
  - _Requirements: REQ-MVP-006.0 (crash recovery), REQ-MVP-007.0 (graceful handling)_

### TDD Cycle 10: Main Binary and Integration

- [ ] 5.3 Implement main.rs with command dispatch
  - **RED**: Write failing test `test_main_command_dispatch()` for all commands
  - **GREEN**: Implement main() function with clap parsing and command routing
  - **GREEN**: Add proper error handling and exit codes
  - **REFACTOR**: Ensure clean startup/shutdown, validate all commands work
  - _Requirements: REQ-MVP-005.0 (CLI entry point), REQ-MVP-007.0 (error handling)_

- [ ] 5.4 Add comprehensive end-to-end validation
  - **RED**: Write failing test `test_end_to_end_workflow()` with real file operations
  - **RED**: Write failing test `test_performance_requirements_met()` for all constraints
  - **GREEN**: Test complete workflow: ingest → daemon → query → context generation
  - **GREEN**: Validate all performance requirements with realistic data
  - **REFACTOR**: Create integration test helpers, document performance results
  - _Requirements: All MVP requirements (complete system validation)_

### TDD Cycle 11: Error Handling and Polish

- [ ] 5.5 Implement comprehensive error handling
  - **RED**: Write failing test `test_error_scenarios()` for all failure modes
  - **GREEN**: Ensure ISGError covers all error cases with clear messages
  - **GREEN**: Add proper error propagation and user-friendly error display
  - **REFACTOR**: Validate error messages are actionable, improve UX
  - _Requirements: REQ-MVP-007.0 (error handling, clear messages)_

- [ ] 5.6 Final performance validation and optimization
  - **RED**: Write failing test `test_all_performance_contracts()` for complete system
  - **GREEN**: Validate: <5s ingestion, <12ms updates, <1ms queries, <500ms persistence
  - **GREEN**: Test memory usage <25MB for 100K LOC
  - **REFACTOR**: Optimize any bottlenecks found, document final performance
  - _Requirements: REQ-MVP-006.0 (all performance constraints)_

### TDD Cycle 12: ISG Visualization and Debugging Tools

- [ ] 5.7 Create ISG visualization and debugging tools
  - **RED**: Write failing test `test_isg_debug_output()` expecting human-readable graph representation
  - **RED**: Write failing test `test_isg_export_dot_format()` for Graphviz visualization
  - **GREEN**: Implement debug_print() method showing nodes and edges in readable format
  - **GREEN**: Implement export_dot() method for Graphviz visualization
  - **REFACTOR**: Add CLI command `parseltongue debug --graph` for visualization
  - _Requirements: REQ-MVP-005.0 (CLI debugging), User Experience (visualization)_

- [ ] 5.8 Create sample data generator for learning
  - **RED**: Write failing test `test_generate_sample_graph()` creating example ISG
  - **GREEN**: Implement sample data generator with realistic Rust code patterns
  - **GREEN**: Add CLI command `parseltongue sample --generate` for learning
  - **REFACTOR**: Include documentation explaining each node and edge type
  - _Requirements: User Experience (learning and understanding)_

## Success Criteria and Validation

### Per-Phase Success Criteria

Each TDD cycle must demonstrate:

1. **RED → GREEN → REFACTOR**: All tasks follow strict TDD methodology
2. **Performance Contracts**: Every timing claim validated by automated tests
3. **Memory Constraints**: 350 bytes/node, <25MB for 100K LOC validated
4. **Concurrency Safety**: Thread safety proven through stress testing
5. **Requirements Traceability**: Each task explicitly references MVP requirements
6. **No Orphaned Code**: Every implementation integrates with previous work

### Final MVP Validation Checklist

- [ ] **REQ-MVP-001.0**: Code dump ingestion <5s for 2.1MB ✓
- [ ] **REQ-MVP-002.0**: Live file monitoring <12ms updates ✓
- [ ] **REQ-MVP-003.0**: Essential queries <1ms response time ✓
- [ ] **REQ-MVP-004.0**: LLM context generation with JSON/human formats ✓
- [ ] **REQ-MVP-005.0**: CLI interface with all required commands ✓
- [ ] **REQ-MVP-006.0**: In-memory performance <25MB, sub-millisecond queries ✓
- [ ] **REQ-MVP-007.0**: Error handling with clear messages ✓

### Performance Validation Summary

**Proven Architecture Guarantees** (from DeepThink analysis):
- **Node/Edge Operations**: 1-5μs (O(1) with parking_lot::RwLock)
- **Simple Queries**: <500μs (petgraph traversal)
- **Complex Queries**: <1ms (BFS with cache locality)
- **Memory Usage**: 350 bytes/node (Arc<str> interning + petgraph overhead)
- **Concurrency**: Thread-safe with single RwLock design

**Implementation Approach**:
- **TDD-First**: Every feature driven by failing tests
- **Performance-First**: Timing constraints validated before feature completion
- **MVP-Focused**: Direct implementation, no premature abstractions
- **Proven Components**: petgraph + parking_lot + FxHashMap architecture

This implementation plan delivers a **working, performant MVP** in 5 weeks with guaranteed performance characteristics and comprehensive test coverage.