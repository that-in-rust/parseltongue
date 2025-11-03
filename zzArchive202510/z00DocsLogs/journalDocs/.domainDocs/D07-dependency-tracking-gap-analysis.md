# D07: Dependency Tracking Gap Analysis

**Status**: Critical Gap Identified
**Date**: 2025-10-31
**Impact**: High - PRD Requirements Unmet
**Priority**: High

---

## Executive Summary

The Parseltongue codebase has **lost critical dependency tracking capabilities** during the transition from the Interface Signature Graph (ISG) architecture to the current CozoDB-based implementation. This represents a **high-priority gap** that prevents the system from meeting explicit PRD requirements for "hopping/blast-radius analysis."

### The Problem

**Current System CANNOT answer:**
- ❌ "Which functions does function X call?"
- ❌ "Which functions call function X?" (reverse dependencies)
- ❌ "What is the blast radius of changing entity X?"
- ❌ "What is the execution path from A to B?"
- ❌ "Which structs implement trait X?"

**Previous System COULD answer (in <1ms):**
- ✅ All the above queries with sub-millisecond performance
- ✅ Graph traversal (BFS/DFS)
- ✅ Blast radius calculation
- ✅ Path finding between entities

---

## Detailed Analysis

### 1. Current CozoDB Schema Limitations

**Location**: `crates/parseltongue-core/src/storage/cozo_client.rs:68-84`

```datalog
:create CodeGraph {
    ISGL1_key: String =>
    Current_Code: String?,
    Future_Code: String?,
    interface_signature: String,
    TDD_Classification: String,
    lsp_meta_data: String?,
    current_ind: Bool,
    future_ind: Bool,
    Future_Action: String?,
    file_path: String,
    language: String,
    last_modified: String,
    entity_type: String
}
```

**Critical Missing Elements:**
- ❌ No edge/relationship table for dependencies
- ❌ No "Calls" relationships between functions
- ❌ No "Uses" relationships between entities
- ❌ No "Implements" relationships (traits)
- ❌ No queryable graph structure

**Partial Workarounds (Insufficient):**
1. `TddClassification.dependencies: usize` - Only a count, no actual relationships
2. `lsp_meta_data.usage_analysis.dependents: Vec<String>` - Optional, Rust-only, names not ISGL1 keys

### 2. Previous ISG Implementation (Archive Analysis)

**Source**: `archive_that-in-rust-parseltongue-8a5edab282632443 (6).txt`

#### Graph Structure (Lines 4720-4989)

```rust
// High-performance directed graph
pub(crate) graph: StableDiGraph<NodeData, EdgeKind>

pub struct NodeData {
    hash: SigHash,
    kind: NodeKind,  // Function, Struct, Trait, Impl
    name: Arc<str>,
    signature: Arc<str>,
    file_path: Arc<str>,
    line: u32,
}

pub enum EdgeKind {
    Calls,       // Function A calls Function B
    Implements,  // Struct implements Trait
    Uses,        // Entity uses another entity
}
```

#### Query Methods (Lines 5281-5446)

```rust
// 1. Find all callers of a function
pub fn find_callers(&self, target_hash: SigHash) -> Vec<SigHash> {
    // Performance: <50μs
    // Returns all functions with incoming "Calls" edges
}

// 2. Find all functions called by a function
pub fn get_called_functions(&self, source_hash: SigHash) -> Vec<SigHash> {
    // Performance: <50μs
    // Returns all functions with outgoing "Calls" edges
}

// 3. Calculate blast radius (impact analysis)
pub fn calculate_blast_radius(&self, start_hash: SigHash) -> HashSet<SigHash> {
    // Performance: <1ms
    // BFS traversal to find all affected entities
}

// 4. Find execution path between two functions
pub fn get_execution_path(&self, from_hash: SigHash, to_hash: SigHash)
    -> Option<Vec<SigHash>> {
    // Performance: <100μs
    // BFS traversal following only "Calls" edges
}

// 5. Find trait implementors
pub fn find_implementors(&self, trait_hash: SigHash) -> Vec<SigHash> {
    // Performance: <500μs
    // Returns all structs/enums implementing trait
}
```

#### Call Graph Extraction (Lines 2331-2473)

```rust
// Dedicated AST visitor for call detection
impl CallGraphVisitor {
    fn visit_call_expr(&mut self, node: Node, source: &[u8]) {
        // Extract function calls during tree-sitter parsing
        let caller_hash = self.current_function_hash;
        let callee_hash = extract_function_hash(node, source);

        // Store edge in graph
        self.isg.upsert_edge(caller_hash, callee_hash, EdgeKind::Calls);
    }

    fn visit_method_call(&mut self, node: Node, source: &[u8]) {
        // Extract method calls
        self.isg.upsert_edge(caller_hash, method_hash, EdgeKind::Calls);
    }
}
```

### 3. What Current System CAN Do

**Simple Entity Queries** (via `CozoDbQueryRef.md`):

```datalog
# Get all functions from a specific file
?[isgl1_key, interface_signature] :=
    *CodeGraph{isgl1_key, interface_signature},
    isgl1_key ~ "rust:fn:.*:src_main_rs:.*"

# Get all test functions
?[isgl1_key, interface_signature, tdd_classification] :=
    *CodeGraph{isgl1_key, interface_signature, tdd_classification},
    tdd_classification.entity_class == "TEST"

# Get entities by type
?[isgl1_key, interface_signature, entity_type] :=
    *CodeGraph{isgl1_key, interface_signature, entity_type},
    entity_type == "function"

# Get changed entities (temporal versioning)
?[isgl1_key, future_action] :=
    *CodeGraph{isgl1_key, future_action, future_ind},
    future_ind == true
```

**What Works:**
- ✅ Entity retrieval by ISGL1 key
- ✅ Filtering by file path, entity type, TDD classification
- ✅ Temporal state queries (current/future indicators)
- ✅ Signature extraction
- ✅ Code content retrieval

### 4. What Current System CANNOT Do

**Dependency Queries** (ALL IMPOSSIBLE):

```datalog
# ❌ IMPOSSIBLE: Get functions called by X
?[caller, callee] :=
    *DependencyEdges{from_isgl1_key: caller, to_isgl1_key: callee},
    caller == "rust:fn:calculate_sum:src_main_rs:42-50"
# Error: Relation 'DependencyEdges' does not exist

# ❌ IMPOSSIBLE: Get functions that call X (reverse dependencies)
?[caller, callee] :=
    *DependencyEdges{from_isgl1_key: caller, to_isgl1_key: callee},
    callee == "rust:fn:database_connect:src_db_rs:100-120"
# Error: Relation 'DependencyEdges' does not exist

# ❌ IMPOSSIBLE: Blast radius calculation
# No recursive query support without edges
# Cannot traverse dependency graph

# ❌ IMPOSSIBLE: Path finding
# Cannot find execution path from main() to target function
# No graph traversal algorithms available
```

**Workarounds (Inadequate):**

1. **LLM Code Inspection**
   - Have LLM read `current_code` field and manually identify calls
   - **Problems**: High token usage, error-prone, not deterministic, slow

2. **LSP Metadata (Rust-only)**
   - Parse `lsp_meta_data.usage_analysis.dependents` if available
   - **Problems**: Optional, requires rust-analyzer, only available after indexing, not during tree-sitter parsing

3. **Manual Pattern Matching**
   - Use string matching on function names in code
   - **Problems**: Unreliable, misses indirect calls, false positives

### 5. Implementation Gaps

#### Gap 1: No Call Graph Extraction (Tool 1)

**Current Tool 1** (`folder-to-cozodb-streamer`):
- ✅ Parses files with tree-sitter
- ✅ Extracts entity signatures
- ✅ Generates ISGL1 keys
- ✅ Stores entities in CodeGraph
- ❌ **Does NOT extract function calls from AST**
- ❌ **Does NOT detect method invocations**
- ❌ **Does NOT track trait implementations**
- ❌ **Does NOT store dependency edges**

**Previous Tool 1 had:**
- ✅ Dedicated `call_graph.rs` module
- ✅ AST visitor for call expression detection
- ✅ Method call extraction
- ✅ Real-time edge insertion during parsing

**Missing Code**: The entire call graph visitor module is absent from current implementation.

#### Gap 2: No Edge Storage Schema

**Current CozoDB Schema:**
- Single table: `CodeGraph` (nodes only)
- No relationships/edges table
- No way to represent "A calls B"

**Required Schema Addition:**

```datalog
:create DependencyEdges {
    from_isgl1_key: String,
    to_isgl1_key: String =>
    edge_type: String,        # "Calls", "Uses", "Implements"
    source_location: String?, # Where in code this relationship exists
    confidence: Float?        # Optional: confidence score (0.0-1.0)
}

# Indices for performance
::index create DependencyEdges:from_idx {from_isgl1_key}
::index create DependencyEdges:to_idx {to_isgl1_key}
::index create DependencyEdges:type_idx {edge_type}
```

#### Gap 3: No Graph Query Implementation (Tool 3)

**Current Tool 3** (`llm-cozodb-to-context-writer`):
- ✅ Generates context JSON from CodeGraph
- ✅ Filters entities by optimization goals
- ✅ Token-aware output generation
- ❌ **Cannot traverse dependencies**
- ❌ **Cannot calculate blast radius**
- ❌ **No graph query helpers**
- ❌ **No multi-hop queries**

**Required Additions:**

```datalog
# 1-hop dependency query (what does X call?)
?[callee, edge_type] :=
    *DependencyEdges{
        from_isgl1_key: $caller_key,
        to_isgl1_key: callee,
        edge_type
    }

# Reverse 1-hop query (who calls X?)
?[caller, edge_type] :=
    *DependencyEdges{
        from_isgl1_key: caller,
        to_isgl1_key: $callee_key,
        edge_type
    }

# N-hop blast radius (recursive)
?[affected_entity, hop_distance] :=
    *DependencyEdges{
        from_isgl1_key: $start_key,
        to_isgl1_key: affected_entity
    },
    hop_distance = 1

?[affected_entity, hop_distance] :=
    ?[intermediate, prev_hop],
    *DependencyEdges{
        from_isgl1_key: intermediate,
        to_isgl1_key: affected_entity
    },
    hop_distance = prev_hop + 1,
    hop_distance <= $max_hops
```

### 6. PRD Requirements Gap

#### Explicit Requirements (UNMET)

**P02PRDL2Detailed.md** (Lines 107-109):
> "Generate future code using hopping/blast-radius analysis:
> - LLM queries CozoDB for dependency chains using ISG patterns
> - Applies proven hopping/blast-radius algorithms converted to CozoDB queries"

**Status**: ❌ **NOT IMPLEMENTED**
- No dependency chains queryable
- ISG patterns not converted to CozoDB
- Blast-radius algorithms missing

**P01PRDL1Minimal.md** (Lines 150-152):
> "The reasoning-LLM can use hopping or blast-radius actions on Code_Graph to fetch additional information"

**Status**: ❌ **NOT IMPLEMENTED**
- Hopping actions not available
- Blast-radius actions not available
- Code_Graph has no edges to traverse

**P06PRDL6AgentTruthSource.md** (Lines 431-432):
> "Hopping queries: Multi-hop dependency analysis (1-hop, 2-hop, N-hop)
> Blast radius: Calculate impact areas for changes"

**Status**: ❌ **NOT IMPLEMENTED**
- Multi-hop queries impossible
- Impact area calculation impossible

### 7. Comparison Table

| Feature | Previous ISG | Current CozoDB | Status |
|---------|-------------|----------------|--------|
| **Node Storage** | In-memory graph | CozoDB CodeGraph | ✅ Equivalent |
| **Edge Storage** | StableDiGraph | ❌ None | ❌ Critical Gap |
| **Call Detection** | AST visitor (call_graph.rs) | ❌ Not implemented | ❌ Critical Gap |
| **Query: "Who calls X?"** | find_callers() <50μs | ❌ Impossible | ❌ Critical Gap |
| **Query: "X calls who?"** | get_called_functions() <50μs | ❌ Impossible | ❌ Critical Gap |
| **Blast Radius** | calculate_blast_radius() <1ms | ❌ Impossible | ❌ Critical Gap |
| **Path Finding** | get_execution_path() <100μs | ❌ Impossible | ❌ Critical Gap |
| **Trait Implementors** | find_implementors() <500μs | ❌ Impossible | ❌ Critical Gap |
| **Persistence** | File dumps | RocksDB/SQLite | ✅ Better |
| **Concurrency** | RwLock | CozoDB native | ✅ Better |
| **Temporal Versioning** | Manual tracking | Built-in | ✅ Better |

### 8. Impact Assessment

#### High-Impact Missing Features

1. **Dependency-Aware Code Changes** ❌
   - **Problem**: Cannot identify what breaks when changing function X
   - **Impact**: No way to find all callers that need updates
   - **Workaround**: Manual code review (error-prone, time-consuming)
   - **Use Case**: Refactoring, API changes, interface modifications

2. **Blast Radius Analysis** ❌
   - **Problem**: Cannot calculate impact scope of changes
   - **Impact**: PRD requirement explicitly unmet
   - **Workaround**: LLM reads code manually (high token cost)
   - **Use Case**: Change impact assessment, risk evaluation

3. **Test-Code Correlation** ⚠️
   - **Problem**: Can identify TEST vs CODE entities ✅, but cannot trace which tests call which code ❌
   - **Impact**: Cannot find affected tests when code changes
   - **Workaround**: Run all tests (slow), or manual inspection
   - **Use Case**: TDD workflows, test coverage analysis

4. **Refactoring Support** ❌
   - **Problem**: Cannot identify all call sites for function rename
   - **Impact**: Incomplete refactorings, broken code
   - **Workaround**: Text search (unreliable for overloaded functions)
   - **Use Case**: Function renaming, signature changes, API evolution

5. **Dead Code Detection** ❌
   - **Problem**: Cannot detect unused functions (no reverse dependency tracking)
   - **Impact**: Code bloat, maintenance burden
   - **Workaround**: None reliable
   - **Use Case**: Code cleanup, optimization

#### Medium-Impact Limitations

1. **Optimization Opportunities Missed**
   - Cannot identify hot paths through call graph
   - Cannot find bottleneck functions by centrality
   - Cannot optimize import statements based on actual usage

2. **Documentation Generation**
   - Cannot auto-generate call hierarchy diagrams
   - Cannot produce dependency documentation
   - Cannot visualize code architecture

3. **Security Analysis**
   - Cannot trace data flow from user input to sensitive operations
   - Cannot identify attack surface by dependency analysis
   - Cannot verify isolation boundaries

### 9. Real-World Query Examples

#### Example 1: Function Rename Refactoring

**Scenario**: Rename `calculate_total` to `compute_sum`

**What's Needed**:
```datalog
# Find all callers of calculate_total
?[caller_key, caller_sig, source_location] :=
    *DependencyEdges{
        from_isgl1_key: caller_key,
        to_isgl1_key: "rust:fn:calculate_total:src_utils_rs:100-110",
        edge_type: "Calls",
        source_location
    },
    *CodeGraph{ISGL1_key: caller_key, interface_signature: caller_sig}
```

**Current Workaround**: Search all files for "calculate_total" (misses dynamic calls, includes comments)

#### Example 2: Impact Analysis Before Change

**Scenario**: Changing signature of `database_connect(url: &str)` to `database_connect(config: DbConfig)`

**What's Needed**:
```datalog
# Calculate blast radius
?[affected_entity, distance, path] :=
    # Recursive query to find all transitively affected entities
    # within 3 hops
```

**Current Workaround**: LLM manually reads code files, guesses dependencies (unreliable, expensive)

#### Example 3: Test Coverage Analysis

**Scenario**: Find which tests exercise the `UserAuth::login()` function

**What's Needed**:
```datalog
# Find all tests that call (directly or indirectly) UserAuth::login
?[test_key, test_name, call_path] :=
    *CodeGraph{ISGL1_key: test_key, TDD_Classification: tdd_class},
    tdd_class.entity_class == "TEST",
    # Find path from test to UserAuth::login through dependency edges
    path_exists(test_key, "rust:fn:login:src_auth_rs:50-100")
```

**Current Workaround**: Run all tests and hope (no way to identify subset)

#### Example 4: Dead Code Identification

**Scenario**: Find functions that are never called

**What's Needed**:
```datalog
# Find functions with no incoming Calls edges
?[unused_key, unused_sig] :=
    *CodeGraph{ISGL1_key: unused_key, interface_signature: unused_sig, entity_type},
    entity_type == "function",
    not *DependencyEdges{to_isgl1_key: unused_key, edge_type: "Calls"}
```

**Current Workaround**: None available

---

## Solution Approaches

### Option A: CozoDB-Native Edges (Recommended)

**Architecture**:
- Add `DependencyEdges` table to CozoDB schema
- Extract edges during tree-sitter parsing (Tool 1)
- Query edges using Datalog (Tool 3)

**Advantages**:
- ✅ Fully persistent (survives restarts)
- ✅ Concurrent access via CozoDB
- ✅ Datalog recursive queries for graph traversal
- ✅ Consistent with current architecture
- ✅ Temporal versioning applies to edges too

**Disadvantages**:
- ⚠️ Query performance unknown (needs benchmarking)
- ⚠️ Datalog learning curve for complex queries
- ⚠️ May be slower than in-memory graph (<50μs target)

**Implementation Effort**: Medium (3-5 days)

**Performance Target**:
- 1-hop queries: <1ms
- Blast radius: <10ms for typical codebases
- Path finding: <5ms

### Option B: Hybrid ISG + CozoDB

**Architecture**:
- Store nodes in CozoDB (entities persist)
- Load edges into in-memory ISG on startup
- Query graph in-memory for speed
- Persist edges periodically to CozoDB

**Advantages**:
- ✅ Fast queries (<50μs) - matches previous performance
- ✅ Proven architecture (previous implementation)
- ✅ Complex graph algorithms easy (Rust graph libraries)
- ✅ Best of both worlds (speed + persistence)

**Disadvantages**:
- ⚠️ More complex: two storage systems
- ⚠️ Memory overhead for large codebases
- ⚠️ Synchronization between ISG and CozoDB required
- ⚠️ Startup cost to load graph

**Implementation Effort**: Medium-High (5-7 days)

**Performance Target**:
- 1-hop queries: <50μs
- Blast radius: <1ms
- Path finding: <100μs

### Option C: Minimal MVP Approach

**Architecture**:
- Store edges in CozoDB
- Implement only essential queries (1-hop forward/reverse)
- Defer complex traversals (blast radius, path finding)

**Advantages**:
- ✅ Smallest implementation effort
- ✅ Meets minimal PRD requirements
- ✅ Can iterate based on actual usage patterns
- ✅ Simple architecture

**Disadvantages**:
- ⚠️ Limited functionality (no N-hop, no blast radius)
- ⚠️ May need rework later if advanced queries needed
- ⚠️ Doesn't fully restore previous capabilities

**Implementation Effort**: Low-Medium (2-3 days)

**Performance Target**:
- 1-hop queries: <5ms
- Blast radius: Deferred
- Path finding: Deferred

---

## Implementation Roadmap

### Phase 1: Schema and Storage (Tool 1, parseltongue-core)

**Tasks**:
1. Define `DependencyEdges` relation in CozoDB schema
2. Create edge types enum (`Calls`, `Uses`, `Implements`)
3. Add indices for from/to ISGL1 keys
4. Implement edge insertion API in `CozoDbStorage`

**Code Locations**:
- `crates/parseltongue-core/src/storage/cozo_client.rs` - Schema definition
- `crates/parseltongue-core/src/entities.rs` - Add `DependencyEdge` struct

**TDD Approach**:
```rust
// RED: Write test
#[tokio::test]
async fn test_insert_dependency_edge() {
    let storage = CozoDbStorage::new("mem").await.unwrap();
    let edge = DependencyEdge {
        from_key: "rust:fn:main:src_main_rs:1-10".into(),
        to_key: "rust:fn:helper:src_main_rs:20-30".into(),
        edge_type: EdgeType::Calls,
        source_location: Some("src/main.rs:5".into()),
    };

    storage.insert_edge(&edge).await.unwrap();

    let edges = storage.get_edges_from("rust:fn:main:src_main_rs:1-10").await.unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].to_key, "rust:fn:helper:src_main_rs:20-30");
}
```

**Estimated Time**: 1 day

### Phase 2: Call Graph Extraction (Tool 1)

**Tasks**:
1. Port/recreate `call_graph.rs` from archive
2. Add AST visitor for function call expressions
3. Add AST visitor for method call expressions
4. Extract call relationships during tree-sitter parsing
5. Insert edges into CozoDB during indexing

**Code Locations**:
- `crates/folder-to-cozodb-streamer/src/call_graph.rs` (new file)
- `crates/folder-to-cozodb-streamer/src/lib.rs` - Integrate visitor

**TDD Approach**:
```rust
// RED: Write test
#[tokio::test]
async fn test_extract_function_calls() {
    let storage = CozoDbStorage::new("mem").await.unwrap();
    let test_code = r#"
        fn main() {
            helper();
            calculate_sum(1, 2);
        }
        fn helper() {}
        fn calculate_sum(a: i32, b: i32) -> i32 { a + b }
    "#;

    let parser = FolderStreamer::new(storage.clone());
    parser.parse_file("test.rs", test_code).await.unwrap();

    // Verify edges were created
    let edges = storage.get_edges_from("rust:fn:main:test_rs:1-5").await.unwrap();
    assert_eq!(edges.len(), 2); // Calls to helper() and calculate_sum()

    let targets: HashSet<_> = edges.iter().map(|e| e.to_key.as_str()).collect();
    assert!(targets.contains("rust:fn:helper:test_rs:6-6"));
    assert!(targets.contains("rust:fn:calculate_sum:test_rs:7-7"));
}
```

**Estimated Time**: 2-3 days

### Phase 3: Graph Queries (Tool 3, parseltongue-core)

**Tasks**:
1. Implement 1-hop forward query ("what does X call?")
2. Implement 1-hop reverse query ("who calls X?")
3. Implement N-hop recursive query (blast radius)
4. Implement path finding query (optional MVP)
5. Add query helper functions to `CozoDbStorage`
6. Update Tool 3 to include dependency data in context JSON

**Code Locations**:
- `crates/parseltongue-core/src/storage/queries.rs` (new file for query templates)
- `crates/parseltongue-core/src/storage/cozo_client.rs` - Add query methods
- `crates/llm-cozodb-to-context-writer/src/lib.rs` - Integrate queries

**TDD Approach**:
```rust
// RED: Write test
#[tokio::test]
async fn test_get_called_functions() {
    let storage = setup_test_graph().await; // Pre-populated with test data

    let called = storage.get_called_functions("rust:fn:main:test_rs:1-10")
        .await
        .unwrap();

    assert_eq!(called.len(), 3);
    assert!(called.contains(&"rust:fn:helper:test_rs:20-30".to_string()));
}

#[tokio::test]
async fn test_blast_radius() {
    let storage = setup_test_graph().await;

    // Calculate blast radius from database_connect (2 hops)
    let affected = storage.calculate_blast_radius(
        "rust:fn:database_connect:src_db_rs:100-120",
        2 // max hops
    ).await.unwrap();

    assert!(affected.len() >= 5); // At least 5 entities affected
    assert!(affected.contains("rust:fn:query_users:src_db_rs:200-250"));
}
```

**Estimated Time**: 2-3 days

### Phase 4: Documentation and Testing

**Tasks**:
1. Update `CozoDbQueryRef.md` with dependency query examples
2. Add integration tests for full pipeline
3. Performance benchmarks (compare to targets)
4. Update `CLAUDE.md` with new capabilities
5. Add examples to domain docs

**Code Locations**:
- `CozoDbQueryRef.md` - Add new query section
- `crates/parseltongue-e2e-tests/` - Add dependency tests
- `.domainDocs/D04-MVP-queries.md` - Add dependency examples

**Estimated Time**: 1 day

---

## Performance Targets

Based on previous implementation and CozoDB capabilities:

| Query Type | Previous ISG | CozoDB Target | Acceptable |
|------------|-------------|---------------|------------|
| 1-hop forward ("X calls who?") | <50μs | <1ms | <5ms |
| 1-hop reverse ("who calls X?") | <50μs | <1ms | <5ms |
| Blast radius (2-3 hops) | <1ms | <10ms | <50ms |
| Path finding (BFS) | <100μs | <5ms | <20ms |
| Trait implementors | <500μs | <2ms | <10ms |

**Rationale**:
- CozoDB uses Datalog (logic programming) vs in-memory graph (imperative)
- Database roundtrip adds overhead vs in-memory access
- Acceptable performance still enables interactive LLM queries (<100ms total)

---

## Testing Strategy

### Unit Tests (TDD First)

```rust
// Storage layer
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_insert_edge() { /* ... */ }

    #[tokio::test]
    async fn test_get_edges_from() { /* ... */ }

    #[tokio::test]
    async fn test_get_edges_to() { /* ... */ }

    #[tokio::test]
    async fn test_query_blast_radius() { /* ... */ }
}
```

### Integration Tests

```rust
// Full pipeline test
#[tokio::test]
async fn test_dependency_extraction_pipeline() {
    // 1. Index test project with Tool 1
    // 2. Verify edges created
    // 3. Query dependencies with Tool 3
    // 4. Verify context JSON includes dependency info
}
```

### Performance Tests

```rust
#[tokio::test]
async fn test_performance_contracts() {
    let storage = setup_large_test_graph().await; // 10k nodes, 50k edges

    // 1-hop query performance
    let start = Instant::now();
    storage.get_called_functions("target_key").await.unwrap();
    assert!(start.elapsed() < Duration::from_millis(5));

    // Blast radius performance
    let start = Instant::now();
    storage.calculate_blast_radius("target_key", 3).await.unwrap();
    assert!(start.elapsed() < Duration::from_millis(50));
}
```

---

## Risks and Mitigation

### Risk 1: CozoDB Query Performance

**Risk**: Datalog queries may be too slow for interactive use

**Mitigation**:
- Benchmark early with realistic dataset
- Add indices on from/to keys
- Consider Option B (hybrid) if performance insufficient
- Profile queries and optimize Datalog patterns

### Risk 2: Call Graph Extraction Accuracy

**Risk**: Tree-sitter may miss complex call patterns (function pointers, trait objects)

**Mitigation**:
- Start with explicit function calls (high confidence)
- Add trait method calls in Phase 2
- Document limitations clearly
- Consider optional LSP integration for Rust (complete accuracy)

### Risk 3: Scope Creep

**Risk**: Implementing too many features delays MVP

**Mitigation**:
- Start with Option C (minimal MVP)
- Prioritize 1-hop queries first
- Defer complex traversals if not immediately needed
- Iterate based on actual usage

### Risk 4: Schema Evolution

**Risk**: Adding edges table may affect existing tools

**Mitigation**:
- Add new table, don't modify CodeGraph
- Ensure backward compatibility
- Test all existing tools after schema change
- Version schema if needed

---

## Code Locations Reference

### Current Implementation
- **Schema**: `crates/parseltongue-core/src/storage/cozo_client.rs:68-84`
- **Entities**: `crates/parseltongue-core/src/entities.rs:500-576`
- **Tool 1 Parser**: `crates/folder-to-cozodb-streamer/src/`
- **Tool 3 Context Gen**: `crates/llm-cozodb-to-context-writer/src/`
- **Query Patterns**: `CozoDbQueryRef.md`

### Previous Implementation (Archive)
- **ISG Graph**: Line 4720-4989
- **Edge Types**: Line 4865-4869
- **Call Graph Visitor**: Lines 2331-2473
- **Query Methods**: Lines 5281-5446
- **Blast Radius**: Lines 5304-5325
- **Find Callers**: Lines 5337-5357

### PRD Requirements
- **Hopping Analysis**: `.prdArchDocs/P02PRDL2Detailed.md:107-109`
- **Blast Radius**: `.prdArchDocs/P06PRDL6AgentTruthSource.md:431-432`
- **Dependency Chains**: `.prdArchDocs/P01PRDL1Minimal.md:150-152`

### Files to Create/Modify
- **New Schema**: `crates/parseltongue-core/src/storage/cozo_client.rs` (add DependencyEdges)
- **New Module**: `crates/folder-to-cozodb-streamer/src/call_graph.rs`
- **New Module**: `crates/parseltongue-core/src/storage/queries.rs`
- **Update**: `crates/llm-cozodb-to-context-writer/src/lib.rs` (add dependency queries)
- **Update**: `CozoDbQueryRef.md` (add dependency query examples)
- **Update**: `.domainDocs/D04-MVP-queries.md` (add examples)

---

## Conclusion

The loss of dependency tracking represents a **critical gap** between the previous ISG implementation and current CozoDB architecture. This gap:

1. **Prevents PRD Requirements**: Explicit hopping/blast-radius features are unmet
2. **Limits Functionality**: Cannot answer fundamental "who calls what" questions
3. **Reduces Usability**: LLM must manually inspect code instead of querying dependencies
4. **Impacts Performance**: High token costs for workarounds vs sub-millisecond queries

**Recommended Action**: Implement **Option A (CozoDB-Native Edges)** as the minimal viable solution to restore critical functionality while maintaining architectural consistency.

**Estimated Total Effort**: 6-8 days for complete implementation (Schema + Extraction + Queries + Docs)

**Priority**: **HIGH** - Core feature required for PRD compliance and system usability.

---

**Next Steps**:
1. Review this analysis and select solution approach (A, B, or C)
2. Create TDD implementation plan for chosen approach
3. Begin Phase 1 (Schema and Storage) with RED tests
4. Iterate through phases with continuous testing
5. Benchmark performance and adjust if needed
