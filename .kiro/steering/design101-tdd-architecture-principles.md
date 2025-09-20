# Design101: TDD-First Architecture Principles for Kiro Workflows

## Purpose

This document establishes architectural principles derived from the Parseltongue AIM Daemon design process and enhanced with executable specification patterns. These principles prevent common design flaws and ensure systems are testable, maintainable, and follow Rust idioms from the ground up. 

**This is Design101** - the fundamental principles every architect should know when working with Kiro's requirements → design → tasks workflow.

## Core Principles

### 0. Executable Specifications: From Narrative to Algorithm

**Principle**: Specifications must be executable blueprints, not ambiguous narratives.

**The Problem**: Traditional user stories are "intentionally lightweight" and designed for human conversation. LLMs cannot participate in clarifying conversations - they need explicit, unambiguous instructions.

**The Solution**: Transform specifications into formal, executable artifacts:

```rust
// ❌ Bad: Ambiguous user story
// "As a user, I want to send messages so that I can communicate"

// ✅ Good: Executable specification with contracts
/// Message creation with deduplication contract
/// 
/// # Preconditions
/// - User must be authenticated and have room access
/// - Content must be 1-10000 chars, sanitized HTML
/// - client_message_id must be valid UUID
/// 
/// # Postconditions  
/// - Returns Ok(Message<Persisted>) on success
/// - Inserts row into 'messages' table
/// - Updates room.last_message_at timestamp
/// - Broadcasts to room subscribers via WebSocket
/// - If client_message_id exists, returns existing message (deduplication)
/// 
/// # Error Conditions
/// - MessageError::Authorization if user lacks room access
/// - MessageError::InvalidContent if content violates constraints
/// - MessageError::Database on persistence failure
pub async fn create_message_with_deduplication(
    &self,
    content: String,
    room_id: RoomId,
    user_id: UserId,
    client_message_id: Uuid,
) -> Result<Message<Persisted>, MessageError>;
```

**Implementation Pattern**:
- **L1 Constraints**: System-wide invariants and architectural rules
- **L2 Architecture**: Complete data models, error hierarchies, interface contracts
- **L3 Modules**: Method-level contracts with STUB → RED → GREEN → REFACTOR cycle
- **L4 User Journeys**: End-to-end behavioral confirmation

**Benefits**:
- Eliminates ambiguity that causes LLM hallucination
- Provides verifiable definition of correctness
- Enables automated validation of implementation
- Creates living documentation that stays current

### 1. Layered Rust Architecture (L1-L3 Pattern)

**Principle**: Structure Rust systems in layers that build upon each other with clear idiom boundaries.

**Layer Structure**:
```rust
// L1: Core Language Features (no_std compatible)
// - Ownership, lifetimes, traits, Result/Option
// - RAII, newtype pattern, zero-cost abstractions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SigHash(pub u128); // Newtype for type safety

// L2: Standard Library Idioms  
// - Collections, iterators, smart pointers
// - Thread safety (Send/Sync), error handling
use std::sync::Arc;
use std::collections::HashMap;

// L3: External Ecosystem
// - Async/await with Tokio
// - Serialization with Serde
// - Database with SQLx
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
```

**Idiomatic Patterns by Layer**:

**L1 Core Idioms**:
- Use `Result<T, E>` for fallible operations (never panic in libraries)
- Leverage RAII with `Drop` for automatic resource cleanup
- Apply newtype pattern for domain-specific types
- Make invalid states unrepresentable at compile time

**L2 Standard Library Idioms**:
- Prefer borrowing (`&str`) over owning (`String`) in function parameters
- Use iterators instead of manual loops for better performance and safety
- Apply `Arc<RwLock<T>>` for shared mutable state across threads
- Implement `From`/`Into` traits for ergonomic conversions

**L3 Ecosystem Idioms**:
- Use `#[tokio::main]` for async entry points
- Apply `#[derive(Serialize, Deserialize)]` for data transfer objects
- Confine `unsafe` code to small, audited library boundaries
- Prefer established crates over custom implementations

**Benefits**:
- Clear separation of concerns and dependencies
- Enables incremental learning and implementation
- Provides upgrade path from simple to complex systems
- Leverages Rust's compile-time guarantees at each layer

### 2. Dependency Injection for Testability

**Principle**: Every component depends on traits, not concrete types.

**Implementation Pattern**:
```rust
// ❌ Bad: Hard dependencies
pub struct SystemComponent {
    database: SqliteConnection,
    file_watcher: NotifyWatcher,
}

// ✅ Good: Trait-based dependencies
pub struct SystemComponent<D, F> 
where
    D: DatabaseProvider + Send + Sync,
    F: FileWatchProvider + Send + Sync,
{
    database: Arc<D>,
    file_watcher: Arc<F>,
}

// Production and test implementations
pub type ProductionSystem = SystemComponent<SqliteDatabase, NotifyFileWatcher>;
pub type TestSystem = SystemComponent<MockDatabase, MockFileWatcher>;
```

**Benefits**:
- Isolated unit testing with mocks
- Parallel development of components
- Easy integration testing
- Clear component boundaries

### 3. RAII Resource Management

**Principle**: All resources must be automatically managed with Drop implementations.

**Implementation Pattern**:
```rust
pub struct ResourceManager {
    connection: Option<Connection>,
    watcher: Option<FileWatcher>,
    _cleanup: CleanupGuard,
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            if let Err(e) = conn.close() {
                eprintln!("Failed to close connection: {}", e);
            }
        }
        // File watcher cleanup handled by CleanupGuard
    }
}

// RAII cleanup guard pattern
struct CleanupGuard {
    cleanup_fn: Box<dyn FnOnce() + Send>,
}

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        (self.cleanup_fn)();
    }
}
```

**Benefits**:
- No resource leaks
- Graceful shutdown under all conditions
- Exception safety
- Clear resource ownership

### 4. Performance Claims Must Be Test-Validated

**Principle**: Every performance claim must be backed by automated tests.

**Implementation Pattern**:
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_query_performance_contract() {
        let system = create_test_system().await;
        
        // Load test data
        for i in 0..10_000 {
            system.add_node(create_test_node(i)).await.unwrap();
        }
        
        let start = Instant::now();
        let result = system.execute_query(test_query()).await.unwrap();
        let elapsed = start.elapsed();
        
        // Validate performance contract
        assert!(elapsed < Duration::from_micros(500), 
                "Query took {:?}, expected <500μs", elapsed);
        assert!(!result.is_empty());
    }
    
    #[test]
    fn test_memory_layout_validation() {
        use std::mem;
        
        // Validate claimed memory usage
        assert_eq!(mem::size_of::<NodeData>(), 72);
        assert_eq!(mem::align_of::<NodeData>(), 8);
        
        // Test string interning efficiency
        let str1 = InternedString::new("common_name");
        let str2 = InternedString::new("common_name");
        assert_eq!(str1.as_ptr(), str2.as_ptr()); // Same pointer = interned
    }
}
```

**Benefits**:
- No unsubstantiated performance claims
- Regression detection
- Confidence in system behavior
- Measurable optimization targets

### 5. Structured Error Handling Following Rust Patterns

**Principle**: Use thiserror for library errors, anyhow for application context.

**Implementation Pattern**:
```rust
// Library errors: Structured with thiserror
#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    
    #[error("Query failed: {query} - {cause}")]
    QueryFailed { query: String, cause: String },
    
    #[error("Timeout after {elapsed:?} (limit: {limit:?})")]
    Timeout { elapsed: Duration, limit: Duration },
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {host}:{port}")]
    ConnectionFailed { host: String, port: u16 },
    
    #[error("Query syntax error: {sql}")]
    SyntaxError { sql: String },
}

// Application errors: Use anyhow for context
pub async fn process_request(req: Request) -> anyhow::Result<Response> {
    let data = fetch_data(&req.id)
        .await
        .with_context(|| format!("Failed to fetch data for request {}", req.id))?;
    
    let result = process_data(data)
        .with_context(|| "Data processing failed")?;
    
    Ok(Response::new(result))
}
```

**Benefits**:
- Clear error boundaries
- Rich context information
- Actionable error messages
- Proper error propagation

### 6. Memory Layout Validation

**Principle**: All memory usage claims must be validated with tests.

**Implementation Pattern**:
```rust
#[cfg(test)]
mod memory_validation {
    use super::*;
    use std::mem;
    
    #[test]
    fn validate_data_structure_sizes() {
        // Validate struct sizes match design claims
        assert_eq!(mem::size_of::<NodeData>(), 72);
        assert_eq!(mem::size_of::<EdgeData>(), 32);
        
        // Validate enum sizes
        assert_eq!(mem::size_of::<NodeKind>(), 1);
        assert_eq!(mem::size_of::<EdgeKind>(), 1);
        
        // Validate alignment requirements
        assert_eq!(mem::align_of::<NodeData>(), 8);
    }
    
    #[test]
    fn validate_memory_efficiency_techniques() {
        // Test string interning
        let name1 = InternedString::new("function_name");
        let name2 = InternedString::new("function_name");
        assert_eq!(name1.as_ptr(), name2.as_ptr());
        
        // Test memory pool efficiency
        let pool = ObjectPool::new();
        let obj1 = pool.acquire();
        let ptr1 = obj1.as_ptr();
        drop(obj1);
        
        let obj2 = pool.acquire();
        let ptr2 = obj2.as_ptr();
        assert_eq!(ptr1, ptr2); // Reused from pool
    }
}
```

**Benefits**:
- Accurate memory usage predictions
- Prevention of memory bloat
- Validation of optimization techniques
- Early detection of memory regressions

### 7. Complex Domain Model Support

**Principle**: Data models must handle real-world complexity, not simplified examples.

**Implementation Pattern**:
```rust
// ❌ Bad: Oversimplified
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub signature: String,
}

// ✅ Good: Handles real complexity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustFunction {
    pub name: InternedString,
    pub signature: RustSignature,
    pub generics: Option<GenericParams>,
    pub where_clause: Option<WhereClause>,
    pub visibility: Visibility,
    pub async_kind: AsyncKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParams {
    pub params: Vec<GenericParam>,
    pub bounds: Vec<TraitBound>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
}

// Test with real-world complexity
#[cfg(test)]
mod complexity_tests {
    #[test]
    fn test_complex_generic_parsing() {
        let code = r#"
            impl<H, S> ErasedIntoRoute<S, Infallible> for MakeErasedHandler<H, S>
            where 
                H: Clone + Send + Sync + 'static,
                S: 'static,
            {
                fn into_route(self) -> Route { todo!() }
            }
        "#;
        
        let parsed = parse_rust_code(code).unwrap();
        let impl_node = parsed.find_impl_node().unwrap();
        
        assert!(impl_node.generics.is_some());
        assert!(impl_node.where_clause.is_some());
        assert_eq!(impl_node.generics.unwrap().params.len(), 2); // H, S
    }
}
```

**Benefits**:
- Handles real-world code complexity
- Prevents oversimplification failures
- Comprehensive feature coverage
- Production-ready from day one

### 8. Concurrency Model Validation

**Principle**: Concurrency designs must be validated with stress tests.

**Implementation Pattern**:
```rust
#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::task::JoinSet;
    
    #[tokio::test]
    async fn test_concurrent_read_write_safety() {
        let storage = Arc::new(create_concurrent_storage().await);
        let mut join_set = JoinSet::new();
        
        // Spawn multiple writers
        for i in 0..10 {
            let storage_clone = Arc::clone(&storage);
            join_set.spawn(async move {
                for j in 0..100 {
                    let node = create_test_node(i * 100 + j);
                    storage_clone.add_node(node).await.unwrap();
                }
            });
        }
        
        // Spawn multiple readers
        for _ in 0..20 {
            let storage_clone = Arc::clone(&storage);
            join_set.spawn(async move {
                for _ in 0..50 {
                    let _ = storage_clone.get_random_node().await;
                }
            });
        }
        
        // Wait for all tasks to complete
        while let Some(result) = join_set.join_next().await {
            result.unwrap(); // Panic if any task failed
        }
        
        // Verify data consistency
        let final_count = storage.node_count().await.unwrap();
        assert_eq!(final_count, 1000); // 10 writers * 100 nodes each
    }
    
    #[tokio::test]
    async fn test_lock_free_read_performance() {
        let storage = create_concurrent_storage().await;
        
        // Add test data
        for i in 0..10_000 {
            storage.add_node(create_test_node(i)).await.unwrap();
        }
        
        let start = Instant::now();
        
        // Concurrent reads should not block each other
        let mut join_set = JoinSet::new();
        for _ in 0..100 {
            let storage_clone = storage.clone();
            join_set.spawn(async move {
                storage_clone.get_node(SigHash(42)).await.unwrap()
            });
        }
        
        while let Some(_) = join_set.join_next().await {}
        
        let elapsed = start.elapsed();
        
        // 100 concurrent reads should complete quickly
        assert!(elapsed < Duration::from_millis(10),
                "Concurrent reads took {:?}, expected <10ms", elapsed);
    }
}
```

**Benefits**:
- Validates thread safety
- Detects race conditions
- Ensures performance under load
- Prevents deadlocks and contention

## Kiro Workflow Integration

### Requirements → Design → Tasks Pattern

**Requirements Phase**:
- Write acceptance criteria in testable "WHEN...THEN...SHALL" format
- Each criterion must be verifiable with automated tests
- Tag criteria with IDs for traceability (REQ-MVP-001.0, etc.)

```markdown
#### Acceptance Criteria
1. WHEN I run `parseltongue ingest <file>` THEN the system SHALL parse separated dump format with FILE: markers and extract all Rust interface signatures using `syn` crate
2. WHEN processing a 2.1MB Rust code dump THEN the system SHALL complete ISG construction in less than 5 seconds
```

**Design Phase**:
- Include test contracts alongside interface definitions
- Define exhaustive error hierarchies upfront
- Specify performance contracts with measurable bounds
- Use executable specifications with preconditions/postconditions

```rust
/// Test Plan for MessageService
/// 
/// Scenario 1: Successful Message Creation
/// Given: valid user in room and valid content
/// When: create_message_with_deduplication is called  
/// Then: returns Ok(Message<Persisted>) and broadcasts via WebSocket
/// 
/// Scenario 2: Deduplication
/// Given: message with client_message_id X already exists
/// When: new message with same client ID X is created
/// Then: returns Ok(existing Message) - no duplicate created
```

**Tasks Phase**:
- Structure as STUB → RED → GREEN → REFACTOR cycle
- Each task references specific requirements
- Include verification steps and success criteria
- Provide one-command validation for each feature

### Living Documentation Pattern

**Principle**: Documentation and code must stay synchronized automatically.

**Implementation**:
```rust
// Code includes references to requirements
#[test]
fn test_blast_radius_performance_req_mvp_003() { // References REQ-MVP-003.0
    // Test validates <500μs execution time requirement
}

// Documentation includes executable examples
/// # Example
/// ```rust
/// let result = storage.calculate_blast_radius(start_hash, 3).await?;
/// assert!(result.len() <= 1000); // Bounded result size
/// ```
```

**Automation**:
- CI checks that all requirement IDs are referenced in tests
- Scripts validate that design.md interfaces exist in codebase
- One-command smoke tests verify end-to-end user journeys

## Design Review Checklist

Before finalizing any architecture design, verify:

### Executable Specifications
- [ ] Requirements written in testable "WHEN...THEN...SHALL" format
- [ ] All acceptance criteria have corresponding automated tests
- [ ] Design includes preconditions, postconditions, and error conditions
- [ ] Performance claims backed by measurable contracts

### Testability
- [ ] All components are trait-based with mock implementations
- [ ] Dependency injection enables isolated testing
- [ ] No hard dependencies on external systems
- [ ] Clear interfaces between components

### Layered Architecture
- [ ] L1 core features properly isolated (no_std compatible where applicable)
- [ ] L2 standard library usage follows Rust idioms
- [ ] L3 external dependencies well-justified and minimal
- [ ] Clear upgrade path from simple to complex

### Resource Management
- [ ] All resource-holding types implement Drop
- [ ] RAII patterns used throughout
- [ ] No potential resource leaks
- [ ] Graceful shutdown under all conditions

### Performance Validation
- [ ] All performance claims backed by tests
- [ ] Memory layout validated with tests
- [ ] Benchmark tests for critical paths
- [ ] Regression detection in place

### Error Handling
- [ ] Structured error hierarchy with thiserror
- [ ] Application context with anyhow
- [ ] Clear error boundaries
- [ ] Actionable error messages

### Domain Complexity
- [ ] Data models handle real-world complexity
- [ ] No oversimplified examples
- [ ] Comprehensive feature coverage
- [ ] Production-ready design

### Concurrency Safety
- [ ] Thread safety validated with tests
- [ ] Lock-free patterns where appropriate
- [ ] Stress testing under concurrent load
- [ ] No potential deadlocks or race conditions

### Kiro Workflow Compliance
- [ ] Requirements reference specific acceptance criteria IDs
- [ ] Design includes test plans for each interface
- [ ] Tasks follow STUB → RED → GREEN → REFACTOR pattern
- [ ] One-command verification available for each feature

## Anti-Patterns to Avoid

### 0. Ambiguous Specifications
```rust
// ❌ Bad: Vague user story
// "As a user, I want better performance"

// ✅ Good: Executable specification
/// Performance Contract: Query execution must complete within 500μs
/// Test validates this constraint with 10,000 node dataset
#[test]
fn test_query_performance_contract() {
    // Measurable, verifiable performance requirement
}
```

### 1. God Objects
```rust
// ❌ Bad: Monolithic component
pub struct SystemManager {
    database: SqliteConnection,
    file_watcher: NotifyWatcher,
    query_engine: QueryEngine,
    cli_handler: CliHandler,
    // ... 20 more fields
}
```

### 2. Unsubstantiated Performance Claims
```rust
// ❌ Bad: Claims without validation
// "This operation takes 5μs" - no test to verify
pub fn fast_operation() -> Result<Data> {
    // Implementation that might not meet claim
}
```

### 3. Hard Dependencies
```rust
// ❌ Bad: Cannot be tested in isolation
pub struct Component {
    db: SqliteConnection, // Hard dependency
}
```

### 4. Resource Leaks
```rust
// ❌ Bad: No cleanup strategy
pub struct FileProcessor {
    files: Vec<File>, // Never closed
    connections: Vec<TcpStream>, // Never closed
}
```

### 5. Oversimplified Models
```rust
// ❌ Bad: Won't handle real code
pub struct Function {
    name: String, // What about generics? Visibility? Async?
}
```

### 6. Layer Violations
```rust
// ❌ Bad: L3 async code in L1 core
pub struct CoreProcessor {
    // Mixing tokio (L3) with core logic (L1)
    runtime: tokio::Runtime,
}

// ✅ Good: Clear layer separation
pub trait AsyncProcessor {
    async fn process(&self, data: CoreData) -> Result<CoreResult>;
}
```

## Application Guidelines

### For Requirements Phase
1. **Write Executable Acceptance Criteria**: Use "WHEN...THEN...SHALL" format that translates directly to tests
2. **Tag for Traceability**: Assign IDs (REQ-MVP-001.0) to enable requirement-to-test mapping
3. **Avoid Ambiguous Language**: Replace "better", "faster", "easier" with measurable criteria

### For Design Phase  
4. **Start with Traits**: Define interfaces before implementations
5. **Include Test Contracts**: Specify preconditions, postconditions, and error conditions
6. **Layer Appropriately**: Respect L1 (core) → L2 (std) → L3 (external) boundaries
7. **Design for Real Complexity**: Handle actual domain complexity, not toy examples

### For Implementation Phase
8. **Write Tests First**: Let tests drive the design (STUB → RED → GREEN → REFACTOR)
9. **Validate Claims**: Every performance assertion needs a test
10. **Manage Resources**: Use RAII patterns consistently
11. **Structure Errors**: Clear hierarchy with proper context
12. **Test Concurrency**: Validate thread safety with stress tests

### For Maintenance Phase
13. **Keep Docs Synchronized**: Use automation to ensure code and documentation stay aligned
14. **One-Command Verification**: Provide simple commands to validate entire features
15. **Continuous Validation**: Run full test suites in CI to catch regressions

## The 20/80 Rule for Rust Idioms

**Principle**: ~20% of Rust patterns enable writing 99% of production code with minimal bugs.

**Core Vital Patterns**:
- **L1**: Ownership/borrowing, RAII, Result/Option, newtype pattern
- **L2**: Iterator patterns, smart pointers (Arc/Rc), error propagation (?)
- **L3**: Async/await, derive macros, established crate patterns

**Compile-First Success Strategy**: 
- Use idiomatic patterns that leverage Rust's type system
- Make invalid states unrepresentable
- Let the compiler catch errors before runtime
- Average 1.6 compile attempts vs 4.9 without patterns (67% faster development)

These principles ensure architectures are testable, maintainable, performant, and production-ready from the start. They transform the traditional requirements → design → tasks workflow into an executable, verifiable process that eliminates ambiguity and reduces bugs through systematic application of proven patterns.