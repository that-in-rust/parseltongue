# Requirements Document

## Acronyms and Terminology

### Core System Acronyms
- **ISG**: Interface Signature Graph - The complete architectural graph representation of a Rust codebase
- **FQN**: Fully Qualified Name - Complete module path for entities (e.g., `my_crate::utils::Config`)
- **AST**: Abstract Syntax Tree - Rust's parsed code representation used for analysis
- **CLI**: Command Line Interface - Terminal-based user interaction system
- **LLM**: Large Language Model - AI systems that benefit from architectural context
- **DOT**: Graphviz DOT format - Standard graph visualization format
- **BFS**: Breadth-First Search - Graph traversal algorithm used for relationship queries

### Technical Terms
- **SigHash**: Signature Hash - Deterministic identifier for code entities using FxHasher
- **Blast Radius**: Complete set of code that could be affected by changes to a target entity
- **Two-Pass Ingestion**: Architecture where nodes are created first, then relationships are established
- **O(1) Performance**: Constant-time operations that don't scale with codebase size
- **Daemon**: Background service that monitors files and maintains live architectural state

### Relationship Types
- **CALLS**: Function A invokes Function B
- **USES**: Function A references Type B (parameters, variables, return types)
- **IMPLEMENTS**: Type A implements Trait B

### Performance Metrics
- **95%+ Relationship Extraction**: Target accuracy for capturing architectural relationships
- **<12ms Update Latency**: Maximum time for processing file changes
- **<1ms Query Response**: Maximum time for simple architectural queries
- **<25MB Memory Usage**: Target memory footprint for 100K lines of code

## Introduction

Parseltongue Architect v2.0 is a **Rust-only** architectural intelligence system that transforms code analysis from broken text parsing to deterministic, high-performance graph-based navigation. The system creates complete Interface Signature Graphs (ISG) from Rust codebases with **95%+ relationship extraction**, enabling sub-millisecond queries, real-time architectural awareness, and zero-hallucination LLM context generation.

**v2.0 Mission**: Fix the fundamental flaws in v1.0 by delivering a working foundation. Ship a reliable system in 30 days that enables confident refactoring through accurate dependency analysis.
**Core v2.0 Constraints:**
- **High-Accuracy Relationship Extraction**: Extract CALLS, USES, and IMPLEMENTS relationships using full AST traversal with 95%+ accuracy
- **O(1) Performance Guarantees**: All operations must use indexed lookups, no O(N) scans
- **Deterministic Identification**: Stable hashing with Fully Qualified Names for cross-platform consistency
- **30-Day Ship Target**: Ruthlessly focused on foundation fixes that enable Sarah's core workflow
## User Journey: The Frustrated Senior Rust Developer

**Persona**: Sarah, Senior Rust Engineer at a fintech startup
**Context**: Managing a 200K LOC Rust codebase with 15 microservices, constant refactoring pressure, and tight deadlines
**Pain Points**: Broken tooling, unreliable dependency analysis, time wasted on manual code archaeology

### Journey Stage 1: The Breaking Point (Current State)
Sarah opens her terminal at 9 AM, coffee in hand, ready to tackle a critical refactoring. The product team needs to extract payment processing into a separate service by Friday. She runs `grep -r "PaymentProcessor"` and gets 847 matches across 200 files. Her heart sinks.

**Emotional State**: Frustrated, overwhelmed, already behind schedule
**Tools Used**: grep, ripgrep, manual code reading, unreliable IDE "find references"
**Time Wasted**: 3+ hours just understanding what calls what
**Business Impact**: Feature delivery delayed, technical debt accumulates

### Journey Stage 2: The Discovery (Parseltongue Introduction)
A colleague mentions Parseltongue. Sarah is skeptical—she's tried every code analysis tool. But the promise of "95%+ relationship extraction" and "sub-millisecond queries" intrigues her. She installs it during lunch.

**Emotional State**: Cautiously optimistic, but expecting disappointment
**First Interaction**: `parseltongue daemon --watch ./src`
**Surprise Moment**: Daemon starts in 800ms, processes entire codebase in 4.2 seconds
**Validation**: `parseltongue query calls PaymentProcessor::process` returns accurate call graph in 0.3ms

### Journey Stage 3: The Transformation (Power User Emergence)
Within a week, Sarah's workflow is completely transformed. She starts every refactoring session with architectural queries. Her confidence in code changes skyrockets because she can see the complete blast radius before making any changes.

**New Morning Routine**:
1. `parseltongue daemon --watch .` (muscle memory)
2. `parseltongue query blast-radius PaymentProcessor` (understand impact)
3. `parseltongue generate-context --focus PaymentProcessor --format llm-prompt` (prep for AI assistance)
4. Refactor with confidence, knowing every dependency

**Emotional State**: Empowered, confident, in control
**Productivity Gain**: 70% reduction in code archaeology time
**Quality Improvement**: Zero surprise breakages from missed dependencies

### Journey Stage 4: The Advocate (Team Transformation)
Sarah becomes the internal champion. She demonstrates Parseltongue in architecture reviews, uses it to onboard new team members, and integrates it into the CI/CD pipeline. The entire team adopts it within a month.

**Team Impact**: 
- Code reviews become architectural discussions, not dependency hunts
- New developers understand the codebase in days, not weeks
- Refactoring velocity increases 3x with confidence

**Business Impact**: 
- Feature delivery accelerates
- Technical debt becomes manageable
- System reliability improves

## v2.0 Requirements

### REQ-V2-001.0: High-Accuracy Relationship Extraction

**User Story:** As a Rust developer analyzing complex codebases, I want highly accurate architectural relationship extraction so that blast-radius analysis and dependency tracking work reliably for confident refactoring.

**Journey Context**: Sarah's refactoring confidence depends on seeing the vast majority of dependencies with high accuracy. Missing 5% of relationships is acceptable; missing 30% breaks trust.

#### Acceptance Criteria

1. WHEN parsing Rust code THEN the system SHALL extract function calls using `syn::visit::Visit` pattern with `visit_expr_call` and `visit_expr_method_call`
2. WHEN analyzing function bodies THEN the system SHALL identify type usage relationships via `visit_type_path` traversal
3. WHEN processing impl blocks THEN the system SHALL extract trait implementations using two-pass ingestion (nodes first, relationships second)
4. WHEN encountering method calls THEN the system SHALL resolve both direct function calls and method calls on types
5. WHEN building the ISG THEN the system SHALL create CALLS edges from functions to their dependencies and USES edges from functions to types they reference
6. WHEN ingestion completes THEN the system SHALL achieve 95%+ relationship extraction accuracy for parsed code, verified through manual spot-checking

### REQ-V2-002.0: O(1) Performance Guarantees

**User Story:** As a Rust developer working on live codebases, I want guaranteed sub-millisecond performance so that the daemon meets the <12ms update and <1ms query constraints.

**Journey Context**: Sarah's morning workflow requires instant architectural queries. Any delay breaks the flow state required for complex refactoring work.

#### Acceptance Criteria

1. WHEN updating files THEN the system SHALL use reverse file index (`FxHashMap<Arc<str>, FxHashSet<SigHash>>`) to achieve O(1) node removal
2. WHEN querying by name THEN the system SHALL use name index (`FxHashMap<Arc<str>, FxHashSet<SigHash>>`) to achieve O(1) entity lookup
3. WHEN calculating blast radius THEN the system SHALL use bounded BFS with early termination to stay under 1ms for typical queries
4. WHEN performing any graph operation THEN the system SHALL maintain O(1) or O(log N) complexity using `FxHashMap` and `petgraph` indexed operations
5. WHEN monitoring file changes THEN the system SHALL complete updates in <12ms using indexed operations only
6. WHEN executing queries THEN the system SHALL respond in <1ms for simple traversals and <2ms for complex analysis

### REQ-V2-003.0: Deterministic Identification System

**User Story:** As a developer using the daemon across different platforms, I want stable, deterministic entity identification so that architectural analysis is consistent and reliable.

**Journey Context**: Sarah's team collaboration requires identical architectural views across different development machines. Inconsistent results break team trust in the tool.
#### Acceptance Criteria

1. WHEN hashing entities THEN the system SHALL use `FxHasher` instead of `DefaultHasher` for cross-platform stability
2. WHEN generating signatures THEN the system SHALL include full module qualification (e.g., `my_crate::utils::Config` not just `Config`)
3. WHEN tracking module context THEN the system SHALL maintain current module path during AST traversal to generate Fully Qualified Names
4. WHEN processing identical code THEN the system SHALL produce identical `SigHash` values across different platforms and Rust versions
5. WHEN persisting state THEN the system SHALL ensure deterministic serialization and deserialization of the ISG
6. WHEN reloading snapshots THEN the system SHALL maintain identical graph structure and node identification

### REQ-V2-004.0: Two-Pass Ingestion Architecture

**User Story:** As a developer processing large codebases, I want reliable relationship extraction that handles forward references and complex dependencies correctly.

**Journey Context**: Sarah's 200K LOC codebase has complex forward references that must be resolved correctly for accurate dependency analysis.
#### Acceptance Criteria

1. WHEN ingesting code dumps THEN the system SHALL use Pass 1 to extract and insert ALL nodes from ALL files before processing relationships
2. WHEN processing relationships THEN the system SHALL use Pass 2 to analyze impl blocks and function bodies after all nodes exist
3. WHEN encountering forward references THEN the system SHALL successfully resolve them because target nodes were created in Pass 1
4. WHEN building edges THEN the system SHALL guarantee that both source and target nodes exist before edge creation
5. WHEN ingestion fails THEN the system SHALL provide clear error messages indicating which pass failed and why
6. WHEN processing large dumps THEN the system SHALL complete two-pass ingestion in <5 seconds for 2.1MB codebases

### REQ-V2-005.0: Core Query Engine

**User Story:** As a developer maintaining large Rust codebases, I want essential architectural queries that enable confident refactoring decisions.

**Journey Context**: Sarah needs blast-radius analysis and trait implementation queries for her daily refactoring work. These are the core queries that enable confident code changes.
#### Acceptance Criteria

1. WHEN querying blast radius THEN the system SHALL provide `parseltongue query blast-radius <entity>` showing all direct and transitive dependencies
2. WHEN querying implementations THEN the system SHALL provide `parseltongue query what-implements <trait>` showing all trait implementors
3. WHEN querying calls THEN the system SHALL provide `parseltongue query calls <entity>` showing all functions that call the target
4. WHEN querying uses THEN the system SHALL provide `parseltongue query uses <entity>` showing all functions that use the target type
5. WHEN executing core queries THEN the system SHALL maintain <1ms response time for simple queries
6. WHEN displaying results THEN the system SHALL provide clear, actionable output with file paths and line numbers

### REQ-V2-006.0: Basic CLI Interface

**User Story:** As a developer using the daemon daily, I want a clean CLI that exposes core architectural analysis capabilities with clear output.

**Journey Context**: Sarah's daily workflow depends on intuitive CLI commands that provide the essential information needed for refactoring decisions.
#### Acceptance Criteria

1. WHEN running queries THEN the system SHALL support `parseltongue query <type> <target>` for blast-radius, calls, uses, and what-implements
2. WHEN generating output THEN the system SHALL provide human-readable format with optional JSON output via `--json` flag
3. WHEN displaying results THEN the system SHALL include basic performance metrics (execution time, result count) in output
4. WHEN encountering errors THEN the system SHALL provide specific error messages with context
5. WHEN running help THEN the system SHALL show clear usage examples for all core query capabilities
6. WHEN daemon is not running THEN the system SHALL provide clear instructions for starting the daemon

### REQ-V2-007.0: Production-Ready Daemon
**User Story:** As a developer deploying the daemon in production environments, I want guaranteed performance characteristics and robust error handling.

**Journey Context**: Sarah's team depends on reliable daemon operation for daily workflows. Tool crashes or performance degradation break the entire team's productivity.

#### Acceptance Criteria

1. WHEN processing large codebases THEN the system SHALL maintain memory usage under 25MB for 100K LOC using string interning and efficient data structures
2. WHEN handling parse errors THEN the system SHALL continue processing other files and provide detailed error reports without crashing
3. WHEN daemon crashes THEN the system SHALL automatically save state and recover gracefully on restart
4. WHEN monitoring files THEN the system SHALL handle file system events reliably with automatic retry on temporary failures
5. WHEN persisting state THEN the system SHALL use incremental snapshots to minimize I/O overhead during daemon operation
6. WHEN validating performance THEN the system SHALL include built-in benchmarking capabilities for performance verification

### REQ-V2-008.0: Basic LLM Context Generation

**User Story:** As a Rust developer using LLM assistants, I want compressed, accurate architectural context that provides factual information for AI code assistance.

**Journey Context**: Sarah occasionally uses AI tools for complex refactoring. She needs factual architectural context that eliminates AI hallucinations about her codebase structure.
#### Acceptance Criteria

1. WHEN I run `parseltongue generate-context --focus <entity>` THEN the system SHALL query the ISG to assemble architectural information for the target entity
2. WHEN generating context THEN the system SHALL include entity definitions, direct relationships, and key dependencies within 1 hop
3. WHEN outputting context THEN the system SHALL produce compressed text blocks using minimal tokens while preserving essential architectural facts
4. WHEN I add `--format llm-prompt` THEN the system SHALL structure output with clear sections optimized for LLM consumption
5. WHEN context includes relationships THEN the system SHALL show CALLS, USES, and IMPLEMENTS relationships with full qualification
6. WHEN generating context THEN the system SHALL complete in <100ms for typical entities

### REQ-V2-009.0: Real-Time Daemon Integration

**User Story:** As a Rust developer working from terminal, I want seamless daemon integration that provides instant architectural intelligence during coding sessions.

**Journey Context**: Sarah's morning routine requires instant daemon startup and live architectural queries that reflect the current state of her code.
#### Acceptance Criteria

1. WHEN I start `parseltongue daemon --watch <directory>` THEN the system SHALL initialize in <2 seconds and begin monitoring all .rs files
2. WHEN I save any .rs file THEN the daemon SHALL complete the update pipeline within 12ms (debounce → parse → diff → apply)
3. WHEN I run `parseltongue query <type> <target>` while daemon is running THEN the system SHALL query the live in-memory graph and respond in <1ms
4. WHEN the daemon detects file changes THEN it SHALL use incremental parsing to reuse unchanged portions for maximum performance
5. WHEN I query architectural state THEN the system SHALL always reflect the current state of my code, never stale data
6. WHEN daemon is running THEN all queries SHALL use the in-memory graph for consistent sub-millisecond performance

### REQ-V2-010.0: Debug Visualization Export

**User Story:** As a developer debugging relationship extraction logic, I want to export the ISG structure for visual analysis so that I can validate parsing accuracy and accelerate development iteration.

**Journey Context**: The development team needs to validate that 95%+ relationship extraction is working correctly. Debugging complex graph structures through logs is inefficient and slows down the critical path to shipping v2.0.

#### Acceptance Criteria

1. WHEN I run `parseltongue debug --export-dot` THEN the system SHALL export the complete ISG as Graphviz DOT format
2. WHEN generating DOT output THEN the system SHALL include meaningful node labels showing FQN and entity kind (e.g., "my_crate::utils::Config (Struct)")
3. WHEN generating DOT output THEN the system SHALL include edge labels showing relationship types (CALLS, USES, IMPLEMENTS)
4. WHEN exporting the graph THEN the system SHALL use `petgraph::dot::Dot` with custom attribute getters for readable visualization
5. WHEN processing the DOT file THEN standard tools (Graphviz, online viewers) SHALL render the architectural structure clearly
6. WHEN debugging relationship extraction THEN the visual output SHALL enable rapid validation of parsing logic and FQN resolution

### REQ-V2-011.0: Interactive HTML Visualization

**User Story:** As a developer showcasing Parseltongue's architectural intelligence capabilities, I want elegant HTML visualization of Interface Signature Graphs so that I can demonstrate the system's core value proposition to stakeholders and potential users.

**Journey Context**: Marketing and demonstration scenarios require compelling visual proof that Parseltongue accurately captures architectural relationships. Static DOT exports are insufficient for interactive exploration and professional presentation.

#### Acceptance Criteria

1. WHEN I run `parseltongue visualize --html <entity>` THEN the system SHALL generate an interactive HTML page showing the ISG structure around the target entity
2. WHEN generating HTML output THEN the system SHALL use modern web technologies (D3.js, vis.js, or similar) to create interactive node-link diagrams
3. WHEN displaying nodes THEN the system SHALL show entity types with distinct visual styling (functions, structs, traits, enums) and meaningful labels
4. WHEN displaying edges THEN the system SHALL use different colors and styles for CALLS, USES, and IMPLEMENTS relationships with clear legends
5. WHEN interacting with the visualization THEN users SHALL be able to zoom, pan, and click nodes to explore architectural relationships
6. WHEN generating visualizations THEN the system SHALL complete in <500ms and produce self-contained HTML files suitable for sharing and presentation

## v2.0 Success Criteria

### Core Foundation (30-Day Deliverables)
1. **High-Accuracy Relationship Extraction**: 95%+ of CALLS, USES, and IMPLEMENTS relationships extracted and verified
2. **O(1) Performance**: All operations use indexed lookups, <12ms updates, <1ms queries
3. **Deterministic Hashing**: Stable `FxHasher` with FQNs, consistent across platforms
4. **Two-Pass Ingestion**: Reliable handling of forward references and complex dependencies

### Essential Capabilities (Core Value)
1. **Core Query Engine**: blast-radius, what-implements, calls, uses queries working reliably
2. **Basic CLI**: Clean interface exposing core architectural queries
3. **Production Daemon**: Robust error handling, automatic recovery, performance monitoring
4. **Basic LLM Context**: Compressed, factual architectural context generation
5. **Debug Visualization**: DOT export for development team validation and debugging
6. **Interactive HTML Visualization**: Elegant web-based ISG visualization for marketing and demonstration

### Performance Validation (Measurable)
1. **Ingestion Speed**: <5s for 2.1MB dumps (measured with real Axum codebase)
2. **Update Latency**: <12ms for file changes (measured with instrumentation)
3. **Query Performance**: <1ms simple queries, <2ms complex analysis (measured with benchmarks)
4. **Memory Efficiency**: <25MB for 100K LOC (measured with profiling tools)
5. **Relationship Accuracy**: 95%+ extraction verified with manual spot-checking and visual validation
6. **Cross-Platform Consistency**: Identical results on Linux, macOS, Windows
### User Journey Success Metrics
1. **Sarah's Core Workflow**: Reliable blast-radius analysis enables confident refactoring
2. **Team Adoption**: Tool becomes essential part of daily development workflow
3. **Trust Building**: Consistent, accurate results build confidence in architectural analysis
## v2.0 Scope Control

### In Scope (30-Day Sprint)
- ✅ Foundation fixes (hashing, indexing, relationship extraction)
- ✅ Core query engine (blast-radius, what-implements, calls, uses)
- ✅ Basic CLI interface with essential commands
- ✅ Production-ready daemon with robust error handling
- ✅ Basic LLM context generation with factual architectural data
- ✅ Real-time daemon integration for live terminal workflows
- ✅ Debug visualization export (DOT format) for development validation
- ✅ Interactive HTML visualization of the whole codebase SIPG data structure

### Deliberately Cut (Deferred to v3.0)
- ❌ Advanced pattern detection (Builder, State Machine, RAII)
- ❌ Architectural debugging and error analysis
- ❌ Interactive exploration and navigation
- ❌ Advanced queries (unused code, circular dependencies)
- ❌ Hybrid storage models (SQLite integration)
- ❌ Complex LLM workflow features
- ❌ IDE integration and language server protocol
- ❌ Web interface and visualization tools
- ❌ User-facing scoped visualization (Mermaid, interactive graphs)
- ❌ Obsidian integration and Markdown vault generation

**Core Validation**: Proves that deterministic, sub-millisecond architectural intelligence on live Rust codebases is achievable with high-accuracy relationship extraction and O(1) performance guarantees, enabling Sarah's core workflow of confident refactoring through reliable dependency analysis.
