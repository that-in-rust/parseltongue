# Parseltongue User Journeys: Deterministic Architectural Navigation

**Core Innovation**: Parseltongue transforms Rust development by replacing probabilistic text searches with deterministic, graph-based architectural navigation through Interface Signature Graphs (ISG).

## Journey 1: Code Dump Analysis - "Deterministic Archaeological Discovery"

**Scenario**: Alice, a senior Rust developer, receives a 2.1MB code dump of the Axum web framework. She needs to understand its architecture deterministically, without probabilistic guessing or manual file traversal.

---

### **Moment 1: Code Dump Ingestion (0-5 seconds)**

**Alice's Action**: 
```bash
# Alice initiates deterministic analysis
parseltongue ingest-code --source CodeDump ./tokio-rs-axum-8a5edab282632443.txt
```

**System Action (AIM Daemon)**:
- **Input Registration**: System registers source as Code Dump format
- **Rust Parser Activation**: Uses `syn` crate + rustdoc JSON for high-fidelity parsing
- **Full Initial Extraction**: Processes all .rs files in the dump

**Real-Time Output**:
```
🐍 Parseltongue AIM Daemon v1.0.0
Ingesting code dump: tokio-rs-axum-8a5edab282632443.txt
✓ Detected separated dump format (FILE: markers)
✓ Parsing 89 Rust source files...
✓ Building Interface Signature Graph (ISG)...
✓ Processing: 247 files → 1,247 interface nodes
✓ Graph construction complete: 4.2s
```

### **Moment 2: ISG Construction & Compression (5-6 seconds)**

**Alice monitors processing status**:

**System Action (AIM Daemon)**:
- **Graph Construction**: Creates nodes for Function, Struct, Trait entities
- **Edge Definition**: Establishes CALLS, IMPL, USES relationships
- **Context Compression**: Achieves 95%+ token compression (2.1MB → 15KB)
- **Memory Optimization**: 100k LOC compresses to ~15-25MB in-memory graph

**Performance Metrics**:
```
🐍 ISG Construction Complete
Nodes created: 1,247 (Function: 634, Struct: 298, Trait: 315)
Edges established: 3,891 (CALLS: 1,456, IMPL: 892, USES: 1,543)
Compression achieved: 99.3% (2.1MB → 15KB architectural essence)
Memory footprint: 18.7MB in-memory graph
Query readiness: <1ms response time
```

### **Moment 3: Deterministic Contract Discovery (6-7 seconds)**

**Alice's Need**: Identify API request handling components

**Alice's Action**:
```bash
# Deterministic query - no probabilistic searching
parseltongue query what-implements axum::extract::FromRequestParts
```

**System Action (AIM Daemon)**:
- **Query Server Execution**: Traverses IMPL edges deterministically
- **Sub-millisecond Response**: Returns implementing structs/functions
- **Zero Hallucination**: Factual, graph-based results only

**Deterministic Output**:
```
🐍 Query: what-implements axum::extract::FromRequestParts
Execution time: 0.7ms

[S] AuthenticatedUser x IMPL x [T] FromRequestParts<AppState>
[S] ConnectInfo<T> x IMPL x [T] FromRequestParts<S>  
[S] MatchedPath x IMPL x [T] FromRequestParts<S>
[S] OriginalUri x IMPL x [T] FromRequestParts<S>
[S] State<T> x IMPL x [T] FromRequestParts<S>

🐍 5 deterministic implementations found
```

### **Moment 4: LLM-Assisted Bounded Context Analysis (7-8 seconds)**

**Alice's Need**: Understand dependencies of main route handler

**Alice's Action**:
```bash
# Generate bounded context slice for specific function
parseltongue generate-context v1_create_user
```

**System Action (AIM Daemon)**:
- **Bounded Context Retrieval**: Extracts relevant ISG slice for v1_create_user
- **Structured Prompt Generation**: Formats precise, constraint-aware context
- **Dependency Mapping**: Includes function signature, constraints, immediate dependencies

**LLM-Ready Context Output**:
```
🐍 Bounded Context: v1_create_user

**Function Signature**:
[F] v1_create_user x ACCEPTS x (State<AppState>, Json<CreateUserRequest>)
[F] v1_create_user x RETURNS x Result<Json<User>, ApiError>

**Direct Dependencies**:
[F] v1_create_user x CALLS x repository::create_user
[F] v1_create_user x CALLS x validation::validate_email
[F] v1_create_user x CALLS x auth::hash_password

**Architectural Constraints**:
- Must return Result<T, ApiError> where T: IntoResponse
- State<AppState> provides database connection pool
- Json<CreateUserRequest> must implement Deserialize
- Function is async and requires Send + 'static

**Blast Radius**:
Upstream callers: [F] routes::user_routes, [F] integration_tests::test_create_user
Downstream dependencies: [T] UserRepository, [T] PasswordHasher

Context tokens: 234 (compressed from 2,847 raw tokens)
🐍 Perfect for deterministic LLM analysis
```

### **Moment 5: Deterministic Dependency Navigation (8-9 seconds)**

**Alice's Question**: "Which functions depend on repository::create_user?"

**Alice's Action**:
```bash
# LLM sends graph query back to Daemon (implicit or explicit)
parseltongue query blast-radius repository::create_user
```

**System Action (AIM Daemon)**:
- **Blast-Radius Analysis**: Executes recursive graph traversal
- **Upstream Caller Detection**: Finds all functions that CALL repository::create_user
- **Factual Results**: Zero hallucination, pure graph-based truth

**Deterministic Navigation Output**:
```
🐍 Blast Radius Analysis: repository::create_user
Execution time: 1.2ms

Direct Impact (3 nodes):
  • [F] v1_create_user (HTTP handler)
  • [F] admin_create_user (Admin handler)  
  • [F] batch_create_users (Batch processor)

Secondary Impact (7 nodes):
  • [F] routes::user_routes (Router configuration)
  • [F] middleware::auth_required (Authentication check)
  • [F] validation::check_duplicate_email (Email validation)
  • [F] audit::log_user_creation (Audit logging)
  • [F] notification::send_welcome_email (Email service)
  • [F] metrics::increment_user_count (Metrics)
  • [F] cache::invalidate_user_cache (Cache management)

🐍 Total affected: 10 functions across 4 modules
🐍 Refactoring risk: Medium (cross-module dependencies detected)
```

### **Moment 6: LLM Integration with Architectural Truth**

**Alice feeds deterministic context to LLM**:
```
User: Using this precise Axum dependency graph, explain the impact of changing repository::create_user signature

LLM Response: Based on the deterministic blast-radius analysis, changing repository::create_user will require updates to exactly 10 functions across 4 modules:

```rust
use axum::{extract::Request, response::Response};
use tower::{Layer, Service};
use std::task::{Context, Poll};

// Following the Layer pattern from Axum
#[derive(Clone)]
pub struct RateLimitLayer {
    max_requests: u32,
    window_secs: u64,
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;
    
    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            max_requests: self.max_requests,
            window_secs: self.window_secs,
        }
    }
}

// Following the Service pattern from Axum
#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    max_requests: u32,
    window_secs: u64,
}

impl<S> Service<Request> for RateLimitService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Rate limiting logic here...
        let future = self.inner.call(request);
        Box::pin(async move {
            // Check rate limits, then call inner service
            future.await
        })
    }
}
```

**Result**: Perfect implementation following Axum patterns, generated in 30 seconds instead of hours of manual research.

---

---

## Journey 2: Real-Time Development - "Deterministic Architectural Companion"

**Scenario**: Bob, a Rust developer, is solving issues and building features with real-time architectural guidance. Parseltongue provides deterministic context on every file save through incremental ISG updates.

---

### **Moment 1: Daemon Initialization (Before Coding)**

**Bob's Action**: Ensures AIM Daemon is monitoring the live Rust filesystem

**System Action (AIM Daemon)**:
- **File System Watcher**: `notify` crate monitors codebase continuously
- **In-Memory Graph**: ISG ready for sub-millisecond queries
- **Query Server**: Listening for real-time architectural queries

**Daemon Status**:
```
🐍 Parseltongue AIM Daemon v1.0.0 - Live Mode
Project: /home/bob/rust-service
✓ File System Watcher: Active (monitoring src/, tests/, examples/)
✓ In-Memory ISG: Ready (current: 847 nodes, 2,134 edges)
✓ Query Server: Listening on localhost:8080
✓ Update Latency Target: <12ms (currently averaging 3.2ms)

🐍 Ready for deterministic architectural guidance
```

### **Moment 2: Code Change Detection (0ms - File Save)**

**Bob's Action**: Opens `src/core/models.rs`, modifies `struct Order` by adding required field, hits Save

**System Action (AIM Daemon)**:
- **(0ms)** File Save detected by OS
- **(0.1-0.8ms)** File System Watcher detects change event
- **Change registered** for incremental processing

**Real-Time Detection**:
```
🐍 Change Detected: src/core/models.rs
Timestamp: 2024-01-18T10:30:45.123Z
Change Type: MODIFY
File Size: 2,847 bytes → 2,923 bytes (+76 bytes)
Queuing for incremental update...
```

### **Moment 3: Incremental ISG Rebuild (0.5-6ms)**

**Bob waits for immediate feedback** (expects no flow interruption)

**System Action (AIM Daemon)**:
- **Rust Parser**: `syn`-based parsing of only the modified file
- **Delta Computation**: Compares old vs new ASTs via SigHash comparison
- **Graph Delta**: Identifies exactly what changed in the ISG

**Incremental Processing**:
```rust
// Old struct Order (SigHash: 0x8a3f2e1b)
pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub items: Vec<OrderItem>,
}

// New struct Order (SigHash: 0x9b4f3e2c) 
pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub items: Vec<OrderItem>,
    pub priority: OrderPriority,  // NEW FIELD
}
```

**Delta Analysis**:
```
🐍 Incremental Update: struct Order
Old SigHash: 0x8a3f2e1b → New SigHash: 0x9b4f3e2c
Changes detected:
  + Added field: priority: OrderPriority
  + Updated struct signature
Processing time: 2.3ms
```

### **Moment 4: Atomic Graph Update (<12ms total latency)**

**Bob expects immediate query readiness**

**System Action (AIM Daemon)**:
- **Atomic ISG Update**: In-memory graph updated with new struct Order signature
- **SQLite Sync**: Delta synced to embedded database using WAL mode
- **Query Readiness**: Graph immediately available for sub-millisecond queries

**Update Completion**:
```
🐍 Atomic Update Complete
Total latency: 8.7ms (target: <12ms)
  - File parsing: 2.3ms
  - Graph delta: 1.8ms  
  - Memory update: 0.4ms
  - SQLite sync: 4.2ms

ISG Status:
  - Nodes: 847 → 848 (+1 updated struct)
  - Edges: 2,134 → 2,156 (+22 new relationships)
  - Query readiness: <1ms response time

🐍 Ready for architectural queries
```

### **Moment 5: Impact Assessment (Blast Radius Analysis)**

**Bob's Need**: Understand all functions affected by Order struct modification

**Bob's Action**:
```bash
# Deterministic impact analysis across entire project
parseltongue query blast-radius struct Order
```

**System Action (AIM Daemon)**:
- **Recursive Graph Traversal**: BFS starting from Order's SigHash
- **Relationship Analysis**: Identifies functions that ACCEPT, RETURN, or USE Order
- **Cross-Module Impact**: Traces dependencies across entire Rust project

**Blast Radius Results**:
```
🐍 Blast Radius Analysis: struct Order
Execution time: 1.4ms

Direct Impact (8 functions):
  • [F] service::process_order x ACCEPTS x Order
  • [F] service::validate_order x ACCEPTS x &Order  
  • [F] repository::save_order x ACCEPTS x Order
  • [F] repository::find_order x RETURNS x Option<Order>
  • [F] api::create_order_handler x RETURNS x Json<Order>
  • [F] api::get_order_handler x RETURNS x Json<Order>
  • [F] serialization::order_to_json x ACCEPTS x &Order
  • [F] validation::check_order_items x ACCEPTS x &Order

Secondary Impact (12 functions):
  • Functions that call the above 8 functions
  • Integration tests that use Order struct
  • API documentation generators

🐍 Total affected: 20 functions across 6 modules
🐍 Breaking change detected: New required field needs handling
```

### **Moment 6: Constraint-Aware LLM Context Generation**

**Bob's Need**: Fix affected function while maintaining API constraints

**Bob's Action**:
```bash
# Generate structured prompt for specific function fix
parseltongue generate-prompt --task "Update process_order to handle new Order field" --context service::process_order
```

**System Action (AIM Daemon)**:
- **Bounded Context Extraction**: Retrieves function signature, struct definition, constraints
- **Architectural Constraint Inclusion**: Adds API requirements (e.g., must return Result<T, ServiceError>)
- **Compressed ISG Context**: Reserves LLM context window for implementation details

**LLM-Ready Prompt**:
```
🐍 Constraint-Aware Context: service::process_order

**Task**: Update process_order to handle new Order field

**Function Signature**:
[F] service::process_order x ACCEPTS x Order x RETURNS x Result<ProcessedOrder, ServiceError>

**Updated Struct Definition**:
```rust
pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId, 
    pub items: Vec<OrderItem>,
    pub priority: OrderPriority,  // NEW FIELD - must be handled
}
```

**Architectural Constraints**:
- Must return Result<ProcessedOrder, ServiceError>
- ServiceError must implement IntoResponse for API compatibility
- Function is async and requires Send + 'static
- Must maintain backward compatibility for existing callers
- New priority field affects processing logic order

**Integration Requirements**:
- Priority affects queue placement in process_queue
- High priority orders bypass standard validation
- Must update ProcessedOrder to include priority status

Context tokens: 156 (compressed from 3,247 raw tokens)
🐍 Perfect for deterministic LLM implementation
```

### **Moment 7: Final Architectural Validation**

**Bob's Need**: Confirm no architectural debt introduced after LLM-generated fixes

**Bob's Action**:
```bash
# Deterministic cycle detection across module/function call graph
parseltongue query find-cycles
```

**System Action (AIM Daemon)**:
- **Cycle Detection Algorithm**: Runs Tarjan's algorithm on ISG
- **Module-Level Analysis**: Checks for circular dependencies
- **Function-Level Analysis**: Validates call graph integrity

**Validation Results**:
```
🐍 Architectural Validation: find-cycles
Algorithm: Tarjan's Strongly Connected Components
Execution time: 3.2ms

Module-Level Analysis:
✓ No circular dependencies detected
✓ Clean dependency flow maintained:
  api → service → repository → database

Function-Level Analysis:  
✓ No recursive call cycles
✓ All async functions properly structured
✓ No deadlock potential detected

Dependency Health:
✓ service::process_order → repository::save_order (clean)
✓ api::create_order_handler → service::process_order (clean)
✓ All 20 affected functions updated successfully

🐍 Integration Status: SAFE
🐍 Architectural Debt: NONE DETECTED
```

### **Moment 8: Continuous Architectural Awareness**

**Bob continues coding with real-time guidance**:

**System provides ongoing insights**:
```
🐍 Real-Time Architectural Monitoring Active

File: src/api/handlers.rs (watching...)
Dependencies tracked: 47 functions across 8 modules
Update latency: 3.2ms average (target: <12ms)
Query response: <1ms average

🐍 Architectural Health Dashboard:
  - Cyclic dependencies: 0 ✓
  - Breaking changes: 1 (handled) ✓  
  - Test coverage gaps: 3 functions ⚠️
  - Performance bottlenecks: 0 ✓

🐍 Ready for next deterministic guidance
```

### **Phase 6: Continuous Architectural Awareness**

**As Marcus continues coding, Parseltongue provides ongoing insights**:

```bash
# After adding HTTP layer
parseltongue ask "blast-radius AuthService"
```

**Output**:
```
🐍 Changes to AuthService would affect:
Direct impact (2 nodes):
  • login_handler (HTTP layer)
  • register_handler (HTTP layer)

Secondary impact (4 nodes):  
  • Router configuration
  • Error handling middleware
  • Integration tests
  • API documentation

🐍 Refactoring safety: Medium risk (6 total affected components)
```

**Real-time dependency tracking**:
```
🐍 File changed: src/handlers.rs
✓ New HTTP handlers detected
✓ Dependencies: handlers → AuthService → UserRepository
✓ Architecture layers: HTTP → Service → Repository → Database
✓ Clean separation maintained

🐍 Suggestion: Consider adding input validation middleware
```

---

---

## **Revolutionary Benefits: Deterministic vs Probabilistic Development**

### **Journey 1: Code Dump Analysis - Performance Metrics**

| **Traditional Approach** | **Parseltongue (Deterministic)** | **Improvement** |
|-------------------------|----------------------------------|-----------------|
| **Manual file reading** (2-3 hours) | **ISG extraction** (4.2s) | **2,571x faster** |
| **Grep/text search** (30 min) | **Graph queries** (<1ms) | **1,800,000x faster** |
| **Pattern recognition** (1 hour) | **Bounded context** (30s) | **120x faster** |
| **Dependency tracing** (45 min) | **Blast-radius analysis** (1.4ms) | **1,928,571x faster** |
| **Total: 4+ hours** | **Total: <1 minute** | **24,000% improvement** |

### **Journey 2: Live Development - Architectural Intelligence**

| **Capability** | **Traditional IDE** | **Parseltongue AIM Daemon** | **Advantage** |
|----------------|-------------------|---------------------------|---------------|
| **Change Detection** | File-level only | **ISG-aware** (3ms updates) | Real-time architectural awareness |
| **Dependency Tracking** | Manual/incomplete | **Deterministic graph traversal** | Zero missed dependencies |
| **Impact Analysis** | Guess-based | **Precise blast-radius** (<2ms) | Factual refactoring safety |
| **LLM Context** | Raw code snippets | **Compressed ISG context** (95% reduction) | Perfect AI communication |
| **Architectural Validation** | Post-hoc discovery | **Real-time cycle detection** | Prevent architectural debt |

### **Core Innovation: Deterministic Navigation**

**Traditional Development (Probabilistic)**:
- ❌ Text-based searches miss architectural relationships
- ❌ LLMs hallucinate dependencies and constraints  
- ❌ Manual impact analysis leads to missed edge cases
- ❌ Architectural violations discovered during testing/production

**Parseltongue Development (Deterministic)**:
- ✅ **Graph-based queries** return factual, complete results
- ✅ **Zero-hallucination LLM context** with compressed ISG
- ✅ **Precise blast-radius analysis** catches all dependencies
- ✅ **Real-time architectural validation** prevents debt accumulation

### **The Parseltongue Advantage: Speaking Truth to AI**

**Interface Signature Graph (ISG) Benefits**:
1. **95%+ Compression**: 2.1MB codebase → 15KB architectural essence
2. **Sub-millisecond Queries**: Deterministic graph traversal
3. **Zero Hallucination**: LLMs receive factual architectural truth
4. **Real-time Updates**: <12ms latency from file save to query readiness
5. **Cross-module Awareness**: Complete dependency mapping

**Developer Transformation**:
- **From**: Hours of manual code archaeology
- **To**: Seconds of deterministic architectural discovery
- **From**: Probabilistic LLM guessing  
- **To**: Constraint-aware AI assistance with perfect context
- **From**: Fear-based refactoring
- **To**: Confident architectural evolution with blast-radius analysis

---

## **MVP 1.0 Requirements: Parseltongue AIM Daemon**

Based on these user journeys, here are the **MVP 1.0 requirements** for the first release:

### **Core Features (Must Have)**

#### **1. Code Dump Ingestion**
- ✅ Parse separated dump format (FILE: markers)
- ✅ Extract Rust interfaces using `syn` crate
- ✅ Build Interface Signature Graph (ISG) in <5 seconds
- ✅ Achieve 95%+ compression ratio

#### **2. Real-Time File Monitoring**  
- ✅ File system watcher using `notify` crate
- ✅ Incremental ISG updates in <12ms
- ✅ Atomic graph updates with SQLite WAL persistence

#### **3. Deterministic Queries**
- ✅ `what-implements <trait>` - Find trait implementations
- ✅ `blast-radius <entity>` - Impact analysis  
- ✅ `find-cycles` - Circular dependency detection
- ✅ Sub-millisecond query response times

#### **4. LLM Context Generation**
- ✅ `generate-context <function>` - Bounded context extraction
- ✅ `generate-prompt --task <task> --context <entity>` - Constraint-aware prompts
- ✅ Compressed ISG context (95% token reduction)

#### **5. CLI Interface**
- ✅ `parseltongue ingest-code --source CodeDump <file>`
- ✅ `parseltongue query <query-type> <target>`  
- ✅ `parseltongue daemon --watch <directory>`
- ✅ Real-time status reporting

### **Performance Targets (MVP 1.0)**

| **Metric** | **Target** | **Measurement** |
|------------|------------|-----------------|
| **Code Dump Processing** | <5 seconds | 50K LOC Rust codebase |
| **ISG Update Latency** | <12ms | File save to query readiness |
| **Query Response Time** | <1ms | Simple graph traversals |
| **Memory Footprint** | <25MB | 100K LOC in-memory ISG |
| **Compression Ratio** | >95% | Raw code to ISG essence |

### **Success Criteria**
- ✅ Process Axum codebase dump (2.1MB) in <5 seconds
- ✅ Real-time updates with <12ms latency
- ✅ Generate LLM context with 95%+ compression
- ✅ Zero false positives in dependency analysis
- ✅ Deterministic, reproducible results

**The MVP Promise**: *Transform Rust development from probabilistic guessing to deterministic architectural navigation.*

🐍 *"Parseltongue MVP 1.0 - Where code speaks truth to AI"* 🐍