# Implementation Plan - Parseltongue Architect v2.0

This implementation plan follows the **STUB → RED → GREEN → REFACTOR** TDD cycle for each component, ensuring tests drive the design and validate all performance contracts.



## IMPORTANT FOR VISUALS AND DIAGRAMS

ALL DIAGRAMS WILL BE IN MERMAID ONLY TO ENSURE EASE WITH GITHUB - DO NOT SKIP THAT

## Current Status Summary

✅ **COMPLETED - Core Foundation Working**:
- Project structure with Cargo workspace (parseltongue-core)
- SigHash deterministic identification with FxHasher
- NodeData memory-optimized storage with Arc<str> interning
- ISGState with O(1) indexed operations (id_map, name_map)
- Basic syn parser for node extraction (functions, structs, traits)
- DOT export for debug visualization
- CLI interface with performance reporting
- Real-time daemon with file monitoring
- Basic LLM context generation
- Snapshot persistence (save/load)
- Comprehensive error handling
- what-implements and blast-radius queries working

🔴 **CRITICAL MISSING - Relationship Extraction**:
- CALLS relationship detection (function calls in bodies)
- USES relationship detection (type usage in signatures/bodies)
- calls and uses CLI query commands
- 95%+ relationship extraction accuracy validation

The system currently only extracts IMPLEMENTS relationships from impl blocks. The core missing piece is syn::visit::Visit traversal to detect function calls and type usage within function bodies, which is essential for the 95%+ relationship extraction accuracy requirement.

## Phase 1: Critical Missing Functionality

- [ ] 1. Implement comprehensive relationship extraction with syn::visit::Visit
  - **STUB**: Write failing tests for CALLS, USES, IMPLEMENTS relationship detection
  - **RED**: Tests fail because RelationshipExtractor doesn't exist
  - **GREEN**: Implement syn::visit::Visit with visit_expr_call, visit_type_path, visit_item_impl
  - **REFACTOR**: Add method call resolution, improve accuracy for complex patterns
  - Test relationship extraction accuracy: target 95%+ on real codebases
  - Use DOT export to validate extracted relationships visually
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction)_

- [ ] 1.1 Implement function call detection (CALLS relationships)
  - **STUB**: Write failing test for detecting function calls in function bodies
  - **RED**: Test fails because visit_expr_call is not implemented
  - **GREEN**: Implement RelationshipExtractor with visit_expr_call and visit_expr_method_call
  - **REFACTOR**: Add method call resolution and cross-module call detection
  - Test with complex call patterns: method chains, closures, macros
  - Validate CALLS edges are created correctly between functions
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction - CALLS)_

- [ ] 1.2 Implement type usage detection (USES relationships)
  - **STUB**: Write failing test for detecting type usage in function signatures and bodies
  - **RED**: Test fails because visit_type_path is not implemented
  - **GREEN**: Implement type path traversal to detect parameter and return types
  - **REFACTOR**: Add generic type resolution and complex type pattern detection
  - Test with generic types, trait objects, and complex type expressions
  - Validate USES edges are created correctly between functions and types
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction - USES)_

- [ ] 1.3 Enhance module-aware FQN generation
  - **STUB**: Write failing test for generating fully qualified names with module paths
  - **RED**: Test fails because ModuleContext doesn't track nested modules correctly
  - **GREEN**: Implement proper module path tracking during AST traversal
  - **REFACTOR**: Handle edge cases like re-exports, use statements, and crate roots
  - Test with complex module hierarchies and cross-module references
  - Validate FQN uniqueness and consistency across different parsing contexts
  - _Requirements: REQ-V2-003.0 (Deterministic Identification System)_

- [ ] 1.4 Validate relationship extraction accuracy with comprehensive tests
  - **STUB**: Write failing tests for complex relationship patterns
  - **RED**: Tests fail because accuracy is below 95%
  - **GREEN**: Improve relationship resolution to achieve 95%+ accuracy
  - **REFACTOR**: Handle edge cases, add warning logs for unresolved patterns
  - Test with trait objects, generic functions, macro-generated code
  - Implement the "95% rule": log warnings for complex constructs, don't block progress
  - _Requirements: REQ-V2-001.0 (High-Accuracy Relationship Extraction)_

## Phase 2: Query Engine Enhancement

- [ ] 2. Implement missing query types (calls, uses)
  - **STUB**: Write failing tests for finding callers and type users
  - **RED**: Tests fail because calls/uses queries don't exist
  - **GREEN**: Implement edge filtering by EdgeKind (CALLS, USES)
  - **REFACTOR**: Add result ranking, performance optimization
  - Test with high-connectivity nodes (central functions, common types)
  - Validate accuracy against manual code analysis
  - _Requirements: REQ-V2-005.0 (Core Query Engine - calls, uses)_

- [ ] 2.1 Add calls and uses query types to CLI
  - **STUB**: Write failing test for CLI query type parsing
  - **RED**: Test fails because QueryType enum doesn't include Calls/Uses
  - **GREEN**: Add Calls and Uses variants to QueryType enum
  - **REFACTOR**: Update CLI help text and query execution logic
  - Test CLI parsing and execution of new query types
  - Validate performance reporting for new queries
  - _Requirements: REQ-V2-006.0 (Basic CLI Interface)_

- [ ] 2.2 Implement query result formatting and performance reporting
  - **STUB**: Write failing test for query result display
  - **RED**: Test fails because result formatting is incomplete
  - **GREEN**: Implement human-readable and JSON output for all query types
  - **REFACTOR**: Add performance metrics and result ranking
  - Test output formatting with large result sets
  - Validate JSON structure for LLM consumption
  - _Requirements: REQ-V2-006.0 (Basic CLI Interface - Performance Reporting)_



## Phase 3: Visualization and Polish

- [ ] 3. Implement interactive HTML visualization
  - **STUB**: Write failing test for self-contained HTML generation with <500ms latency
  - **RED**: Test fails because HTML visualization doesn't exist
  - **GREEN**: Implement HTML template with embedded JavaScript visualization
  - **REFACTOR**: Add interactivity, improve visual layout, optimize performance
  - Test HTML generation performance and browser compatibility
  - Validate self-contained nature (no external dependencies)
  - _Requirements: REQ-V2-011.0 (Interactive HTML Visualization)_

- [ ] 3.1 Implement graph data serialization for web visualization
  - **STUB**: Write failing test for JSON graph data export
  - **RED**: Test fails because graph serialization doesn't exist
  - **GREEN**: Implement graph-to-JSON conversion for web rendering
  - **REFACTOR**: Optimize JSON size, add filtering for large graphs
  - Test with large graphs (1000+ nodes), ensure browser performance
  - Validate JSON structure compatibility with visualization libraries
  - _Requirements: REQ-V2-011.0 (Interactive HTML Visualization - Data Export)_





## Phase 4: Integration and Validation

- [ ] 4. Implement end-to-end integration tests
  - **STUB**: Write failing tests for complete user workflows
  - **RED**: Tests fail because integration isn't complete
  - **GREEN**: Implement full workflow tests: ingest → query → update → visualize
  - **REFACTOR**: Add performance regression tests, cross-platform validation
  - Test with real Rust projects: tokio, serde, clap, bevy
  - Validate all performance contracts under realistic workloads
  - _Requirements: All requirements integrated_

- [ ] 4.1 Validate performance contracts with realistic workloads
  - **STUB**: Write failing tests for performance contracts on 100K+ LOC codebases
  - **RED**: Tests fail because performance targets aren't met
  - **GREEN**: Optimize bottlenecks to meet all performance contracts
  - **REFACTOR**: Add performance monitoring, regression detection
  - Test memory usage, query latency, update speed on large codebases
  - Validate cross-platform consistency (Linux, macOS, Windows)
  - _Requirements: REQ-V2-002.0 (O(1) Performance Guarantees), REQ-V2-009.0 (Real-Time Integration)_

- [ ] 4.2 Implement comprehensive documentation and examples
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