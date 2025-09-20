# Development Methodology Steering Options

## Purpose
This document captures software development methodologies, architectural patterns, and implementation strategies discovered during document analysis that can guide the Parseltongue AIM Daemon development approach.

## Core Development Principles

### TDD-First Approach
- **Compile-time validation**: Use Rust's type system to catch errors at compile time
- **Property-based testing**: Leverage proptest for comprehensive test coverage
- **Test-driven design**: Write tests before implementation to drive API design
- **One-shot correctness**: Aim for implementations that work correctly on first compile

### Pure Function Architecture
- **Functional decomposition**: Break complex operations into pure, testable functions
- **Immutable data structures**: Prefer immutable types where performance allows
- **Side-effect isolation**: Separate pure logic from I/O and state mutations
- **Composable operations**: Design functions that can be easily combined and reused

### Rust-Specific Patterns
- **Zero-cost abstractions**: Use Rust's type system for performance without runtime overhead
- **Ownership-driven design**: Let Rust's ownership model guide architectural decisions
- **Error propagation**: Use Result<T, E> and ? operator for clean error handling
- **Async patterns**: Structure concurrency with async/await and proper task management

## Implementation Strategies

### Incremental Development
- **Start with types**: Define data structures and interfaces first
- **Build from core**: Implement fundamental operations before complex features
- **Test at boundaries**: Focus testing on module interfaces and error conditions
- **Refactor fearlessly**: Use Rust's compiler to ensure refactoring safety

### Performance-First Design
- **Measure early**: Profile and benchmark from the beginning
- **Optimize hot paths**: Identify and optimize critical performance bottlenecks
- **Memory efficiency**: Design data structures for minimal memory footprint
- **Concurrent safety**: Use Arc<RwLock<T>> and DashMap for thread-safe operations

### Anti-Coordination Patterns
- **Direct function calls**: Avoid complex messaging or event systems
- **Simple state management**: Use straightforward data structures over complex patterns
- **Minimal dependencies**: Prefer standard library and essential crates only
- **Explicit over implicit**: Make dependencies and relationships clear in code

## Decision Framework

### When to Apply TDD
- **New algorithms**: Complex parsing or graph traversal logic
- **Critical paths**: Performance-sensitive operations like SigHash generation
- **Error handling**: Comprehensive coverage of failure scenarios
- **API boundaries**: Public interfaces that other components depend on

### When to Use Pure Functions
- **Data transformations**: Converting between different data representations
- **Calculations**: Mathematical operations and algorithmic computations
- **Validation logic**: Input validation and constraint checking
- **Query operations**: Read-only operations on data structures

### When to Optimize
- **After correctness**: Only optimize working, tested code
- **Measured bottlenecks**: Use profiling to identify actual performance issues
- **Critical constraints**: Operations that must meet <12ms update targets
- **Memory pressure**: When approaching memory usage limits

## Architectural Patterns

### Layered Architecture
```
CLI Layer (clap commands)
    ↓
Service Layer (business logic)
    ↓
Repository Layer (data access)
    ↓
Storage Layer (SQLite + DashMap)
```

### Error Handling Strategy
```rust
// Library errors: thiserror for structured error types
// Application errors: anyhow for context and error chains
// Recovery: Graceful degradation with fallback strategies
```

### Concurrency Model
```rust
// File monitoring: notify crate with crossbeam channels
// Graph updates: Arc<RwLock<HashMap<SigHash, Node>>>
// Query serving: DashMap for lock-free concurrent access
```

## Implementation Guidelines

### Code Organization
- **Module structure**: Organize by domain (parsing, graph, storage, cli)
- **Interface segregation**: Small, focused traits and interfaces
- **Dependency injection**: Use trait objects for testability
- **Configuration**: Centralized configuration with validation

### Testing Strategy
- **Unit tests**: Test individual functions and methods
- **Integration tests**: Test component interactions
- **Property tests**: Use proptest for edge case discovery
- **Performance tests**: Benchmark critical operations

### Documentation Approach
- **Code comments**: Explain why, not what
- **API documentation**: Comprehensive rustdoc for public interfaces
- **Architecture docs**: High-level design decisions and trade-offs
- **Usage examples**: Practical examples for common use cases

## Quality Gates

### Before Implementation
- [ ] Types and interfaces defined
- [ ] Test cases written
- [ ] Performance targets established
- [ ] Error scenarios identified

### During Implementation
- [ ] Tests passing
- [ ] Performance within targets
- [ ] Memory usage acceptable
- [ ] Error handling comprehensive

### Before Merge
- [ ] Code review completed
- [ ] Documentation updated
- [ ] Integration tests passing
- [ ] Performance benchmarks stable

## Methodology Evolution

This document should be updated as new development patterns and methodologies are discovered during the implementation of Parseltongue AIM Daemon. Key areas for evolution:

- **New Rust patterns**: Advanced type system usage and performance optimizations
- **Testing strategies**: Novel approaches to testing concurrent and performance-critical code
- **Architecture refinements**: Improvements to the layered architecture based on implementation experience
- **Tool integration**: Better integration with Rust ecosystem tools and workflows

## Cross-References

- **Requirements**: Links to specific requirements that drive methodology choices
- **Architecture**: References to architectural decisions that influence development approach
- **Performance**: Connections to performance targets and optimization strategies
- **Testing**: Integration with overall testing and quality assurance strategy## TDD I
mplementation Patterns (zz04MoreNotes.md)

### OptimizedISG Test-Driven Development

#### Core TDD Cycle Implementation
```rust
// Red -> Green -> Refactor cycle for OptimizedISG
pub struct OptimizedISG {
    state: Arc<RwLock<ISGState>>,
}

struct ISGState {
    graph: StableDiGraph<NodeData, EdgeKind>,
    id_map: FxHashMap<SigHash, NodeIndex>,
}
```

#### TDD Testing Strategy
- **Unit Tests**: Individual functions and methods with mock data
- **Integration Tests**: Component interactions with real graph structures  
- **Property Tests**: Use proptest for edge case discovery
- **Performance Tests**: Benchmark critical operations against <500μs targets
- **Fault Injection**: Crash testing for WAL recovery validation

#### Test Structure Patterns
```rust
#[cfg(test)]
mod tests {
    // Helper for creating consistent test nodes
    fn mock_node(id: u64, kind: NodeKind, name: &str) -> NodeData {
        NodeData {
            hash: SigHash(id),
            kind,
            name: Arc::from(name),
            signature: Arc::from(format!("sig_{}", name)),
        }
    }

    // Test initialization (Red -> Green)
    #[test]
    fn test_isg_initialization() {
        let isg = OptimizedISG::new();
        assert_eq!(isg.node_count(), 0);
        assert_eq!(isg.edge_count(), 0);
    }
}
```

### Performance-Driven Development Methodology

#### Decision Matrix Approach
- **Performance (40%)**: Query speed, update latency, memory efficiency
- **Simplicity (25%)**: Development effort, operational overhead  
- **Rust Integration (20%)**: Ecosystem fit, ergonomics
- **Scalability (15%)**: Growth path, distribution capability

#### Performance Projections by Scale
| Scale | Query Latency | Update Latency | Memory Usage |
|-------|---------------|----------------|--------------|
| Small (10K LOC) | <10μs | <3ms | <40MB |
| Medium (100K LOC) | <10μs | <5ms | <150MB |
| Large (500K LOC) | <15μs | <8ms | <700MB |
| Enterprise (10M+ LOC) | <20μs | <10ms | Distributed |

### Phased Implementation Strategy

#### Phase 1: MVP Foundation (0-6 months)
- **Architecture**: SQLite with WAL mode
- **Focus**: Development velocity and stability
- **Migration Triggers**: 
  - p99 latency >2ms for depth-3 blast-radius
  - Write queue backlog >5ms
  - Complex graph algorithms needed

#### Phase 2: Performance Scaling (6-18 months)  
- **Architecture**: Custom In-Memory Graph with WAL
- **Implementation**: 
  - Parallel development alongside v1.0
  - okaywal crate for WAL implementation
  - bincode for high-performance serialization
  - Shadow mode deployment for validation

#### Phase 3: Enterprise Distribution (18+ months)
- **Architecture**: Distributed Hybrid with tiered storage
- **Components**:
  - Hot/cold data separation
  - SurrealDB for cold storage backend
  - Federated query engine
  - Distributed hot cache with sharding

### Risk Mitigation Patterns

#### Performance Monitoring
- **Automated Alerts**: Latency and throughput triggers
- **Memory Profiling**: CI/CD integration with mem_dbg
- **Benchmarking**: Continuous performance regression testing

#### Data Integrity Assurance
- **WAL Testing**: Fault injection for crash recovery
- **Checksums**: CRC32 in log entries and snapshots
- **Fsync Correctness**: Proper durability guarantees

#### Memory Optimization Techniques
- **String Interning**: Arc<str> for repeated values
- **Arena Allocation**: Contiguous memory for cache locality
- **Custom Collections**: Replace Vec<Edge> with optimized structures
- **Profiling Tools**: jemallocator statistics, mem_dbg integration

### Rust-Specific Development Patterns

#### Concurrency Design
- **Single RwLock**: Atomic synchronization between graph and index
- **Inner Mutability**: RwLock within stored values for concurrent access
- **DashMap Alternative**: Avoid coordination complexity of separate locks

#### Error Handling Strategy
```rust
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ISGError {
    #[error("Node with SigHash {0:?} not found")]
    NodeNotFound(SigHash),
}
```

#### Memory-Efficient Data Structures
- **StableDiGraph**: Indices remain valid upon deletion
- **FxHashMap**: Fast lookups for integer-like keys
- **Arc<str>**: String interning for memory efficiency

### Implementation Quality Gates

#### Before Implementation
- [ ] Performance targets established (<500μs queries, <12ms updates)
- [ ] Test cases written for all core functionality
- [ ] Memory usage benchmarks defined
- [ ] Error scenarios identified and tested

#### During Implementation  
- [ ] TDD cycle maintained (Red -> Green -> Refactor)
- [ ] Performance benchmarks passing
- [ ] Memory usage within targets
- [ ] Concurrent access patterns validated

#### Before Deployment
- [ ] Fault injection testing completed
- [ ] Performance regression tests passing
- [ ] Memory profiling shows no leaks
- [ ] Recovery procedures validated

This methodology ensures that performance requirements drive architectural decisions while maintaining code quality through rigorous testing and measurement.
#
# TDD-First Development Methodology (tdd-patterns.md)

### Core Philosophy: Interface Contracts Before Implementation

**Fundamental Principle**: Define complete function signatures, type contracts, and property tests before writing any implementation code. This ensures one-shot correctness and prevents coordination complexity.

#### TDD Development Pipeline
```
TYPE CONTRACTS → PROPERTY TESTS → INTEGRATION CONTRACTS → IMPLEMENTATION → VALIDATION
       ↓               ↓                    ↓                  ↓             ↓
   Complete        Behavior           Service            Type-Guided    Comprehensive
   Interface       Properties         Boundaries         Implementation    Testing
   Design          Specification      Definition         Following         Validation
                                                        Contracts
```

### Phase 1: Type Contract Definition

#### Complete Function Signature Specification
```rust
/// Creates a node in the Interface Signature Graph with deduplication
/// 
/// # Type Contract
/// - Input: NodeData with validated SigHash and metadata
/// - Output: Result<GraphNode<Verified>, GraphError>
/// - Side Effects: Graph update, SQLite write, index update
/// 
/// # Properties
/// - Same SigHash always returns the same Node
/// - Node is atomically created and indexed
/// - Graph consistency maintained throughout operation
/// 
/// # Error Cases
/// - ValidationError: Invalid node data or SigHash collision
/// - GraphError: Graph consistency violation
/// - DatabaseError: SQLite operation failure
pub async fn upsert_node_with_deduplication(
    &self,
    data: NodeData,
) -> Result<GraphNode<Verified>, GraphError>;
```

#### Phantom Types for State Safety
```rust
// Prevent invalid graph state transitions at compile time
#[derive(Debug)]
pub struct InterfaceGraph<State> {
    nodes: HashMap<SigHash, NodeData>,
    edges: Vec<EdgeData>,
    _state: PhantomData<State>,
}

pub struct Building;
pub struct Validated;
pub struct Ready;

// Only validated graphs can be queried
impl InterfaceGraph<Building> {
    pub fn validate(self) -> Result<InterfaceGraph<Validated>, GraphError> {
        // Validate graph consistency
        self.check_node_references()?;
        self.check_edge_validity()?;
        
        Ok(InterfaceGraph {
            nodes: self.nodes,
            edges: self.edges,
            _state: PhantomData,
        })
    }
}

impl InterfaceGraph<Validated> {
    pub fn finalize(self) -> InterfaceGraph<Ready> {
        InterfaceGraph {
            nodes: self.nodes,
            edges: self.edges,
            _state: PhantomData,
        }
    }
}

// Only ready graphs can be queried
impl InterfaceGraph<Ready> {
    pub fn query_blast_radius(&self, start: SigHash) -> Vec<SigHash> {
        // Type system ensures graph is ready for queries
        self.traverse_dependencies(start)
    }
}
```

#### Session Types for Protocol Safety
```rust
// File parsing state machine in types
pub struct FileParser<State> {
    file_path: PathBuf,
    content: String,
    _state: PhantomData<State>,
}

pub struct Loaded;
pub struct Parsed { ast: syn::File }
pub struct Extracted { nodes: Vec<NodeData>, edges: Vec<EdgeData> }

// State transitions enforced by type system
impl FileParser<Loaded> {
    pub fn parse_rust_file(self) -> Result<FileParser<Parsed>, ParseError> {
        let ast = syn::parse_file(&self.content)?;
        Ok(FileParser {
            file_path: self.file_path,
            content: self.content,
            _state: PhantomData,
        })
    }
}

impl FileParser<Parsed> {
    pub fn extract_interface_data(self) -> FileParser<Extracted> {
        let (nodes, edges) = extract_from_ast(&self.ast);
        FileParser {
            file_path: self.file_path,
            content: self.content,
            _state: PhantomData,
        }
    }
}

// Only extracted data can be added to graph
impl GraphBuilder {
    pub fn add_file_data(&mut self, parser: FileParser<Extracted>) -> Result<(), GraphError> {
        // Type system ensures data is properly extracted
        self.add_nodes(parser.nodes)?;
        self.add_edges(parser.edges)?;
        Ok(())
    }
}
```

### Phase 2: Property-Based Test Specification

#### Graph Invariant Testing
```rust
#[cfg(test)]
mod graph_properties {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn graph_consistency_invariants(
            nodes in prop::collection::vec(arbitrary_node(), 1..100),
            edges in prop::collection::vec(arbitrary_edge(), 0..200)
        ) {
            let graph = build_test_graph(nodes, edges);
            
            // Invariant 1: All edges reference existing nodes
            for edge in graph.edges() {
                prop_assert!(graph.contains_node(edge.from));
                prop_assert!(graph.contains_node(edge.to));
            }
            
            // Invariant 2: No duplicate SigHashes
            let mut seen_hashes = HashSet::new();
            for node in graph.nodes() {
                prop_assert!(seen_hashes.insert(node.hash));
            }
            
            // Invariant 3: Blast radius is deterministic
            for node_hash in graph.node_hashes() {
                let radius1 = graph.query_blast_radius(node_hash);
                let radius2 = graph.query_blast_radius(node_hash);
                prop_assert_eq!(radius1, radius2);
            }
        }
        
        #[test]
        fn incremental_updates_preserve_consistency(
            initial_nodes in prop::collection::vec(arbitrary_node(), 10..50),
            updates in prop::collection::vec(graph_update(), 1..20)
        ) {
            let mut graph = build_test_graph(initial_nodes, vec![]);
            
            // Apply sequence of updates
            for update in updates {
                let _ = graph.apply_update(update);
                
                // Invariant: Graph remains consistent after each update
                prop_assert!(graph.validate_consistency().is_ok());
            }
        }
    }
}
```

#### Performance Property Testing
```rust
proptest! {
    #[test]
    fn query_performance_properties(
        graph_size in 100..10000usize,
        query_depth in 1..10usize
    ) {
        let graph = generate_test_graph(graph_size);
        let start_node = graph.random_node();
        
        let start_time = Instant::now();
        let result = graph.query_blast_radius_with_depth(start_node, query_depth);
        let duration = start_time.elapsed();
        
        // Property: Query completes within performance target
        prop_assert!(duration < Duration::from_micros(500));
        
        // Property: Result size is bounded by graph structure
        prop_assert!(result.len() <= graph.node_count());
    }
}
```

### Phase 3: Integration Contract Definition

#### Service Boundary Contracts
```rust
pub struct GraphServiceContracts {
    pub file_monitor: Arc<dyn FileMonitorService<Error = MonitorError>>,
    pub parser: Arc<dyn RustParserService<Error = ParseError>>,
    pub graph_store: Arc<dyn GraphStorageService<Error = StorageError>>,
    pub query_engine: Arc<dyn QueryEngineService<Error = QueryError>>,
}

#[tokio::test]
async fn file_update_integration_contract() {
    let contracts = create_test_service_contracts().await;
    
    // Given: A monitored Rust file
    let file_path = PathBuf::from("src/lib.rs");
    let initial_content = r#"
        pub struct TestStruct {
            field: i32,
        }
    "#;
    
    // Setup file monitoring
    let mut file_events = contracts.file_monitor
        .watch_directory("src/")
        .await
        .unwrap();
    
    // When: File is modified
    tokio::fs::write(&file_path, initial_content).await.unwrap();
    
    // Then: File change is detected and processed
    let file_event = tokio::time::timeout(
        Duration::from_millis(100),
        file_events.recv()
    ).await.unwrap().unwrap();
    
    assert_eq!(file_event.path, file_path);
    assert_eq!(file_event.event_type, FileEventType::Modified);
    
    // And: Graph is updated with new nodes
    let nodes = contracts.graph_store
        .get_nodes_from_file(&file_path)
        .await
        .unwrap();
    
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].kind, NodeKind::Struct);
    assert_eq!(nodes[0].name, "TestStruct");
}
```

### Phase 4: Type-Guided Implementation

#### Actor Pattern for Graph Updates
```rust
pub struct GraphUpdateActor {
    sender: mpsc::UnboundedSender<GraphCommand>,
}

impl GraphUpdateActor {
    pub fn new(storage: Arc<dyn GraphStorage>) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        
        // Single update task - no coordination needed
        tokio::spawn(async move {
            while let Some(command) = receiver.recv().await {
                match command {
                    GraphCommand::UpdateFile { path, content, reply } => {
                        let result = Self::process_file_update(&storage, path, content).await;
                        let _ = reply.send(result);
                    }
                    GraphCommand::QueryBlastRadius { start, reply } => {
                        let result = storage.query_blast_radius(start).await;
                        let _ = reply.send(result);
                    }
                }
            }
        });
        
        Self { sender }
    }
    
    pub async fn update_file(&self, path: PathBuf, content: String) -> Result<UpdateStats, GraphError> {
        let (reply_tx, reply_rx) = oneshot::channel();
        
        self.sender.send(GraphCommand::UpdateFile { 
            path, 
            content, 
            reply: reply_tx 
        }).map_err(|_| GraphError::ActorUnavailable)?;
            
        reply_rx.await
            .map_err(|_| GraphError::ActorUnavailable)?
    }
}
```

#### RAII Resource Management for File Monitoring
```rust
pub struct FileWatchGuard {
    path: PathBuf,
    watcher: Arc<RecommendedWatcher>,
}

impl FileWatchGuard {
    pub fn new(path: PathBuf, watcher: Arc<RecommendedWatcher>) -> Result<Self, MonitorError> {
        watcher.watch(&path, RecursiveMode::Recursive)?;
        Ok(Self { path, watcher })
    }
}

impl Drop for FileWatchGuard {
    fn drop(&mut self) {
        // Automatic cleanup - no coordination needed
        let _ = self.watcher.unwatch(&self.path);
    }
}

// Usage ensures file watching is always cleaned up
pub async fn monitor_rust_project(project_path: PathBuf) -> Result<(), MonitorError> {
    let watcher = create_file_watcher().await?;
    let _guard = FileWatchGuard::new(project_path, watcher)?;
    
    // File monitoring logic
    // Watcher automatically stopped when _guard is dropped
    Ok(())
}
```

### Phase 5: Comprehensive Validation

#### Performance Benchmarking
```rust
#[bench]
fn bench_graph_update_pipeline(b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let graph_service = rt.block_on(create_test_graph_service());
    
    b.iter(|| {
        rt.block_on(async {
            let file_content = generate_test_rust_file();
            let path = PathBuf::from("test.rs");
            
            let start = Instant::now();
            graph_service.update_file(path, file_content).await.unwrap();
            let duration = start.elapsed();
            
            // Ensure update completes within 12ms target
            assert!(duration < Duration::from_millis(12));
        })
    });
}

#[bench]
fn bench_query_performance(b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let graph_service = rt.block_on(create_large_test_graph());
    
    b.iter(|| {
        rt.block_on(async {
            let start_node = graph_service.random_node_hash();
            
            let start = Instant::now();
            let result = graph_service.query_blast_radius(start_node).await.unwrap();
            let duration = start.elapsed();
            
            // Ensure query completes within 500μs target
            assert!(duration < Duration::from_micros(500));
            assert!(!result.is_empty());
        })
    });
}
```

### Key Benefits for Parseltongue AIM Daemon

1. **One-Shot Correctness**: Complete interface design prevents graph consistency bugs
2. **Coordination Prevention**: Type system enforces simple patterns, prevents complex locking
3. **Performance Guarantees**: Property tests ensure <12ms updates and <500μs queries
4. **Documentation**: Function signatures serve as executable specifications
5. **Refactoring Safety**: Type contracts ensure graph operations remain consistent
6. **Comprehensive Coverage**: Property tests catch edge cases in graph traversal

### Anti-Patterns to Avoid in Graph Implementation

1. **Implementation Before Contracts**: Never write graph operations without complete type contracts
2. **Weak Error Types**: All graph error cases must be enumerated in Result types
3. **Untested Invariants**: All graph consistency rules must have property tests
4. **Coordination Complexity**: Type system should prevent complex locking patterns
5. **Incomplete Integration Tests**: All file monitoring → parsing → graph update flows must be tested

This TDD-first methodology ensures the Parseltongue AIM Daemon achieves its performance targets while maintaining graph consistency and preventing the coordination complexity that typically plagues real-time systems.## Exe
cutable Specifications Analysis (from Executable Specifications for LLM Code Generation.md)

**Source**: _refIdioms/Executable Specifications for LLM Code Generation.md (214 lines, truncated)
**Relevance**: High - specification methodology applicable to parseltongue daemon development

### Key Concepts for Parseltongue Development

#### L1-L4 Framework for Executable Specifications
1. **L1: constraints.md** - System-wide invariants and architectural rules
2. **L2: architecture.md** - Data models, error hierarchies, component relationships  
3. **L3: modules/*.md** - Method-level contracts with STUB → RED → GREEN → REFACTOR cycle
4. **L4: user_journeys.md** - End-to-end behavioral validation

#### Design by Contract (DbC) Application to Rust
```rust
// L3 STUB Example for parseltongue daemon
pub fn parse_rust_file(
    file_path: &Path,
    // Precondition: Valid .rs file, readable
) -> Result<Vec<Node>, ParseError> {
    // Postcondition: Returns parsed AST nodes or specific error
    // Invariant: <12ms execution time for files <10K LOC
}

// L2 Exhaustive Error Hierarchy
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    FileNotFound(PathBuf),
    InvalidSyntax(syn::Error),
    FileTooLarge(usize), // > 10K LOC
    PermissionDenied(PathBuf),
    Timeout(Duration), // > 12ms
}
```

#### TDD as Machine-Readable Protocol
- **RED Phase**: Failing tests define exact behavior expectations
- **GREEN Phase**: Decision tables for complex conditional logic
- **REFACTOR Phase**: Explicit anti-patterns and constraints

#### Property-Based Testing for ISG
```rust
// Property test example for ISG invariants
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn isg_update_preserves_graph_integrity(
            nodes in vec(any::<Node>(), 1..100)
        ) {
            let mut isg = ISG::new();
            for node in nodes {
                isg.insert(node.sig_hash(), node);
            }
            
            // Property: All nodes remain reachable
            // Property: No dangling references
            // Property: Update time < 12ms
            prop_assert!(isg.validate_integrity());
        }
    }
}
```

### Applicable Patterns for Parseltongue Daemon

#### 1. Formal Specification Structure
- **L1 Constraints**: MVP boundaries (Rust-only, <12ms, in-memory)
- **L2 Architecture**: ISG data model, file watching system, error handling
- **L3 Modules**: Individual parser functions with complete test coverage
- **L4 Journeys**: File save → parse → update → query workflows

#### 2. Verification Harness
```bash
#!/bin/bash
# verification.sh - Single script to validate entire system
set -e

echo "Running static analysis..."
cargo clippy -- -D warnings

echo "Running unit tests..."
cargo test --lib

echo "Running property tests..."
cargo test --test property_tests

echo "Running integration tests..."
cargo test --test integration

echo "Running performance benchmarks..."
cargo bench --bench parsing_speed

echo "All verification passed ✅"
```

#### 3. Decision Tables for Complex Logic
```markdown
## File Change Detection Logic

| File Extension | Size (LOC) | Syntax Valid | Action | Update ISG | Notify |
|---------------|------------|--------------|---------|------------|---------|
| .rs           | < 10K      | Yes          | Parse   | Yes        | Yes     |
| .rs           | < 10K      | No           | Skip    | No         | Error   |
| .rs           | > 10K      | *            | Skip    | No         | Warn    |
| .toml         | *          | *            | Skip    | No         | No      |
| *             | *          | *            | Skip    | No         | No      |
```

#### 4. Formal Error Handling
```rust
// Complete error taxonomy prevents LLM from inventing errors
#[derive(Debug, Clone, PartialEq)]
pub enum DaemonError {
    // File system errors
    FileSystem(FileSystemError),
    // Parsing errors  
    Parse(ParseError),
    // ISG errors
    IndexUpdate(IndexError),
    // Performance constraint violations
    Performance(PerformanceError),
}

impl DaemonError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            DaemonError::FileSystem(_) => true,
            DaemonError::Parse(_) => false,
            DaemonError::IndexUpdate(_) => true,
            DaemonError::Performance(_) => false,
        }
    }
}
```

### Benefits for Parseltongue Development

1. **Eliminates Ambiguity**: Formal specifications prevent misinterpretation
2. **Verifiable Correctness**: Automated verification harness ensures quality
3. **Performance Contracts**: <12ms constraints built into specifications
4. **Error Completeness**: Exhaustive error hierarchies prevent edge cases
5. **Maintainable Architecture**: Clear separation of concerns and contracts

### Implementation Strategy for Parseltongue

1. **Start with L3**: Define core parsing functions with complete test coverage
2. **Build L2**: Formalize ISG data model and error hierarchies
3. **Establish L1**: Document MVP constraints and system invariants
4. **Complete L4**: Define end-to-end user workflows
5. **Automate Verification**: Single script validates entire system

**MVP Relevance**: Very High - methodology directly applicable to daemon specification
**Routing Decision**: Development methodology → dev-steering-options.md ✅##
 TDD Documentation Enhancement Analysis (from Proposal_ Enhancing Documentation for TDD and Feature Specifications.docx.md)

**Source**: _refIdioms/Proposal_ Enhancing Documentation for TDD and Feature Specifications.docx.md (203 lines)
**Relevance**: High - TDD documentation patterns applicable to parseltongue daemon development

### Key TDD Documentation Patterns for Parseltongue

#### 1. Living Template Approach
- **Design Documents**: Include test contracts alongside interface definitions
- **Requirements**: Make acceptance criteria explicitly testable
- **Tasks**: Embed test development as first-class tasks
- **Architecture**: Define one-command verification flows

#### 2. Test Contract Integration in Design Documents
```rust
// Example for parseltongue daemon
/// FileParser - Complete Interface Contract
pub trait FileParser {
    /// Parses a Rust source file into AST nodes
    /// 
    /// # Arguments
    /// * `file_path` - Path to .rs file (must exist and be readable)
    /// 
    /// # Returns
    /// * `Ok(Vec<Node>)` - Parsed AST nodes with signature hashes
    /// * `Err(ParseError)` - Specific error type for failure cases
    /// 
    /// # Performance Contract
    /// * Must complete within 12ms for files <10K LOC
    /// * Memory usage must not exceed 25MB per file
    fn parse_file(&self, file_path: &Path) -> Result<Vec<Node>, ParseError>;
}

/// FileParser Test Plan
/// - **Scenario 1: Successful Parse**
///   **Given** a valid .rs file with functions and structs
///   **When** parse_file is called
///   **Then** returns Ok(Vec<Node>) with correct signature hashes
///   **And** completes within 12ms
/// 
/// - **Scenario 2: Invalid Syntax**
///   **Given** a .rs file with syntax errors
///   **When** parse_file is called  
///   **Then** returns Err(ParseError::InvalidSyntax)
///   **And** error contains specific location information
/// 
/// - **Scenario 3: Performance Constraint**
///   **Given** a .rs file larger than 10K LOC
///   **When** parse_file is called
///   **Then** returns Err(ParseError::FileTooLarge)
///   **And** does not attempt parsing
```

#### 3. Testable Acceptance Criteria Format
```markdown
## Requirement 1: File Change Detection
**User Story**: As a developer, I want the daemon to detect file changes immediately so that my queries reflect the latest code state.

**Acceptance Criteria**:
1. WHEN a .rs file is saved THEN the daemon SHALL detect the change within 100ms
2. WHEN a file change is detected THEN the daemon SHALL re-parse within 12ms  
3. WHEN parsing completes THEN the ISG SHALL be updated atomically
4. WHEN ISG update completes THEN query responses SHALL reflect new state

**Verification**: Requirements 1.1-1.4 validated by integration test simulating file save → query cycle, measuring latency at each step (see FileWatcher Test Plan).
```

#### 4. Feature Verification Checklist Template
```markdown
## Feature Verification Checklist: File Parsing System

- [ ] **Design Spec Completed**: All parser interfaces and error types defined in design.md
- [ ] **Acceptance Criteria Met**: Every AC has corresponding test scenario implemented
- [ ] **All Tests Passing**: Unit tests, integration tests, and property tests all green
- [ ] **Performance Verified**: <12ms parsing constraint validated by benchmarks
- [ ] **One-Command Flow OK**: `cargo test --test file_parsing_flow` passes end-to-end
- [ ] **Docs Updated**: All documentation reflects final implemented behavior
- [ ] **No Regressions**: Existing ISG functionality unaffected
- [ ] **MVP Constraints**: Rust-only, in-memory, <12ms requirements maintained
```

#### 5. Reusable Feature Spec Template for Parseltongue
```markdown
# Feature: <Feature Name>

## User Story
As a <developer/user>, I want <feature goal> so that <benefit>.

## Acceptance Criteria
1. WHEN <context> THEN daemon SHALL <behavior> within <time constraint>
2. WHEN <error condition> THEN daemon SHALL <error handling>
*(Include all key behaviors, edge cases, and performance constraints)*

## Design
**Data Model/Types**: New structs, enums for feature
**Service Interfaces**: Trait methods with Rust signatures and doc comments
**Error Handling**: Specific error variants for failure cases
**Performance Contracts**: Timing and memory constraints

## Test Plan
**Unit Tests**: Pure functions and isolated components
**Integration Tests**: End-to-end scenarios with preconditions/postconditions
**Property Tests**: Invariants that must hold across all inputs
**Performance Tests**: Benchmarks validating <12ms constraints

## Tasks
- [ ] **Design**: Update design.md with types and interface stubs
- [ ] **Tests**: Write failing tests for each scenario (TDD red phase)
- [ ] **Implementation**: Code to make tests pass (TDD green phase)
- [ ] **Refactor**: Optimize while maintaining test coverage
- [ ] **Verify**: Run `cargo test --test <feature_flow>` - all scenarios pass
```

#### 6. One-Command Verification Strategy
```bash
#!/bin/bash
# verify-parseltongue.sh - Single command to validate entire daemon

echo "🔍 Running static analysis..."
cargo clippy -- -D warnings

echo "⚡ Running unit tests..."
cargo test --lib

echo "🔗 Running integration tests..."
cargo test --test integration

echo "📊 Running property tests..."
cargo test --test property_tests

echo "⏱️  Running performance benchmarks..."
cargo bench --bench parsing_speed
if [ $? -ne 0 ]; then
    echo "❌ Performance benchmarks failed - <12ms constraint violated"
    exit 1
fi

echo "🎯 Running end-to-end flow tests..."
cargo test --test file_change_flow
cargo test --test query_response_flow

echo "✅ All verification passed - daemon ready for deployment"
```

#### 7. Traceability and Coverage Automation
```rust
// Test naming convention for requirement traceability
#[test]
fn test_file_change_detection_req_1_1() {
    // Covers: Req 1.1 - File change detection within 100ms
    // Given: A .rs file exists
    // When: File is modified
    // Then: Change detected within 100ms
}

#[test] 
fn test_parsing_performance_req_1_2() {
    // Covers: Req 1.2 - Parsing completes within 12ms
    // Given: Valid .rs file <10K LOC
    // When: parse_file called
    // Then: Completes within 12ms
}
```

### Benefits for Parseltongue Development

1. **Systematic Coverage**: Every requirement maps to specific tests
2. **Performance Contracts**: <12ms constraints built into test plans
3. **Clear Verification**: Single command validates entire system
4. **Documentation Sync**: Specs and code stay aligned through automation
5. **TDD Enforcement**: Tests written before implementation by design
6. **Requirement Traceability**: Clear mapping from user needs to test coverage

### Implementation Strategy

1. **Start with Templates**: Use feature spec template for new components
2. **Embed Test Plans**: Add test contracts to all design documents  
3. **Automate Verification**: Create single-command validation script
4. **Enforce Traceability**: Use requirement IDs in test naming
5. **Continuous Integration**: Run full verification on every commit

**MVP Relevance**: Very High - TDD methodology directly applicable to daemon development
**Routing Decision**: TDD patterns and documentation → dev-steering-options.md ✅#
# Documentation Hierarchy Analysis (from documentation-hierarchy-analysis.md)

**Source**: _refIdioms/documentation-hierarchy-analysis.md (198 lines)
**Relevance**: High - documentation strategy patterns applicable to parseltongue daemon development

### Key Documentation Hierarchy Patterns for Parseltongue

#### 1. 5-Level Documentation Hierarchy
```
requirements.md (L1 - Governing Rules & Critical Constraints)
    ↓
architecture.md (L2 - System Architecture & Component Design)
    ↓  
implementation-patterns.md (L3 - TDD Implementation Patterns)
    ↓
design.md (L4 - Complete Technical Contracts)
    ↓
tasks.md (L5 - Maximum Implementation Detail)
```

#### 2. Clear Document Boundaries for Parseltongue
- **requirements.md**: WHAT (user stories, MVP constraints, performance requirements)
- **architecture.md**: WHY (ISG design rationale, component relationships)
- **implementation-patterns.md**: HOW (TDD approach, Rust patterns, testing strategy)
- **design.md**: CONTRACTS (complete interfaces, types, error handling)
- **tasks.md**: DETAILS (test stubs, implementation steps, verification)

#### 3. Standardized TDD Methodology Across All Documents
```
TYPE CONTRACTS → PROPERTY TESTS → INTEGRATION TESTS → IMPLEMENTATION → PERFORMANCE VALIDATION
```

**Applied to Parseltongue**:
1. **Type Contracts**: Define all Rust interfaces and data structures
2. **Property Tests**: Invariants that must hold (ISG integrity, <12ms performance)
3. **Integration Tests**: End-to-end file parsing workflows
4. **Implementation**: Code that makes tests pass
5. **Performance Validation**: Benchmark against <12ms constraint

#### 4. Complete Interface Contracts in design.md
```rust
// Example: Complete FileParser interface contract
pub trait FileParser {
    /// Parses Rust source file into AST nodes
    /// 
    /// # Performance Contract
    /// - Must complete within 12ms for files <10K LOC
    /// - Memory usage must not exceed 25MB per file
    /// 
    /// # Error Contract
    /// - Returns ParseError::FileTooLarge for files >10K LOC
    /// - Returns ParseError::InvalidSyntax for syntax errors
    /// - Returns ParseError::Timeout for operations >12ms
    fn parse_file(&self, path: &Path) -> Result<Vec<Node>, ParseError>;
    
    /// Updates ISG with parsed nodes atomically
    /// 
    /// # Atomicity Contract
    /// - Either all nodes updated or none (no partial updates)
    /// - ISG remains queryable during update process
    /// - Update completes within 5ms of parsing completion
    fn update_isg(&self, nodes: Vec<Node>) -> Result<(), UpdateError>;
    
    /// Validates ISG integrity after updates
    /// 
    /// # Integrity Contract
    /// - All nodes have valid signature hashes
    /// - No dangling references between nodes
    /// - Graph remains connected and traversable
    fn validate_integrity(&self) -> Result<(), IntegrityError>;
}

// Complete error hierarchy referenced in tasks.md
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    FileTooLarge { path: PathBuf, size_loc: usize },
    InvalidSyntax { path: PathBuf, error: syn::Error },
    Timeout { path: PathBuf, duration: Duration },
    FileNotFound { path: PathBuf },
    PermissionDenied { path: PathBuf },
}
```

#### 5. Critical Constraint Implementation Consolidation
```markdown
### MVP Constraint #1: <12ms Update Latency
**Problem**: File changes must be reflected in queries within 12ms
**Implementation Logic**: 
- File watcher detects change within 100ms
- Parser processes file within 8ms
- ISG update completes within 3ms  
- Query readiness achieved within 1ms buffer
**Test Specifications**: 
- Property test: all file changes complete within 12ms
- Integration test: file save → query response timing
**Performance Validation**: Benchmark suite validates constraint
```

#### 6. Information Flow Validation Checklist
```markdown
### Downward Information Flow (Requirements → Tasks)
- [ ] All MVP constraints from requirements.md reflected in design.md contracts
- [ ] All performance requirements have corresponding test specifications
- [ ] All error conditions defined in architecture.md have error types in design.md
- [ ] All interface methods in design.md have test stubs in tasks.md

### Upward Consistency (Tasks → Requirements)  
- [ ] All implementation details in tasks.md traceable to design.md contracts
- [ ] All test specifications validate requirements from requirements.md
- [ ] No implementation complexity exceeds architectural constraints
- [ ] All performance benchmarks validate MVP constraints
```

#### 7. Traceability Matrix for Parseltongue
```markdown
| Requirement | Architecture Component | Design Interface | Task Implementation | Test Validation |
|-------------|----------------------|------------------|-------------------|-----------------|
| <12ms updates | FileWatcher + Parser | FileParser::parse_file | Task 2.1-2.3 | benchmark_parsing_speed |
| In-memory ISG | ISG data structure | ISG::update_node | Task 3.1-3.2 | test_isg_integrity |
| Rust-only parsing | syn crate integration | SynParser::parse | Task 1.1-1.4 | test_rust_parsing |
| LLM-terminal output | Context generator | ContextGen::generate | Task 4.1-4.2 | test_context_generation |
```

#### 8. Verification Harness Standardization
```bash
#!/bin/bash
# verify-parseltongue-complete.sh - Standardized across all documents

echo "1. Static Analysis (Type Contracts)"
cargo clippy -- -D warnings

echo "2. Property Tests (Invariants)"  
cargo test --test property_tests

echo "3. Integration Tests (End-to-End)"
cargo test --test integration

echo "4. Implementation Tests (Unit)"
cargo test --lib

echo "5. Performance Validation (<12ms)"
cargo bench --bench parsing_speed
if [ $? -ne 0 ]; then
    echo "❌ Performance constraint violated"
    exit 1
fi

echo "✅ All verification steps passed"
```

### Benefits for Parseltongue Development

1. **Consistency**: Same TDD methodology across all documents
2. **Completeness**: design.md contains all interfaces tasks.md references
3. **Traceability**: Clear path from requirements to implementation
4. **Validation**: Standardized verification process
5. **Clarity**: Each document has distinct, non-overlapping responsibility

### Implementation Strategy

1. **Establish Boundaries**: Define clear responsibility for each document level
2. **Standardize Methodology**: Use consistent TDD terminology throughout
3. **Complete Contracts**: Ensure design.md has all interfaces tasks.md needs
4. **Create Traceability**: Map requirements to implementation details
5. **Validate Flow**: Check information consistency up and down hierarchy

**MVP Relevance**: Very High - documentation strategy directly applicable to daemon development
**Routing Decision**: Documentation methodology → dev-steering-options.md ✅