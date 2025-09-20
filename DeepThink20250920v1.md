This response provides a Test-Driven Development (TDD) implementation of the `OptimizedISG` MVP (Phase 1). We adopt the recommended architecture: a custom in-memory Rust graph storage using `petgraph` for core algorithms and a single `parking_lot::RwLock` to protect the entire state (graph structure and index map).

This design prioritizes simplicity and correctness by ensuring atomic synchronization between the graph and its index, avoiding the complexities and potential deadlocks of coordinating separate locks (e.g., `RwLock` + `DashMap`).

We will follow the TDD cycle: Write a failing test (Red), implement the minimum code to pass (Green), and then Refactor.

### Project Setup

Dependencies in `Cargo.toml`:

```toml
[dependencies]
petgraph = "0.6"
parking_lot = "0.12"
fxhash = "0.2"
thiserror = "1.0"
```

### Implementation (`src/isg.rs`)

```rust
use fxhash::FxHashMap;
use parking_lot::RwLock;
use petgraph::graph::{NodeIndex, StableDiGraph};
use petgraph::Direction;
use petgraph::visit::{Bfs, Walker, EdgeRef};
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;

// --- Domain Models ---

// Strong typing for the unique identifier.
// Using u64 for simplicity in this example; production should use a robust hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SigHash(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Function,
    Struct,
    Trait,
}

// Data stored in the graph nodes. Arc<str> is used for memory efficiency (string interning).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeData {
    pub hash: SigHash,
    pub kind: NodeKind,
    pub name: Arc<str>,
    pub signature: Arc<str>,
}

// Data stored in the graph edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    Calls,
    Implements, // Direction: Struct -> Trait
    Uses,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ISGError {
    #[error("Node with SigHash {0:?} not found")]
    NodeNotFound(SigHash),
}

// --- OptimizedISG Structure ---

// The internal mutable state, protected by the RwLock.
struct ISGState {
    // StableDiGraph ensures indices remain valid upon deletion.
    graph: StableDiGraph<NodeData, EdgeKind>,
    // FxHashMap provides fast lookups.
    id_map: FxHashMap<SigHash, NodeIndex>,
}

/// Optimized In-Memory Interface Signature Graph.
// Derive Clone to allow easy sharing of the ISG instance across threads.
#[derive(Clone)]
pub struct OptimizedISG {
    state: Arc<RwLock<ISGState>>,
}

impl Default for OptimizedISG {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizedISG {
    // TDD Cycle 1: Initialization
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(ISGState {
                graph: StableDiGraph::new(),
                id_map: FxHashMap::default(),
            })),
        }
    }

    pub fn node_count(&self) -> usize {
        // Acquire a fast read lock.
        self.state.read().graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.state.read().graph.edge_count()
    }

    // TDD Cycle 2: Node Upsert and Retrieval

    /// Inserts a node or updates it if the SigHash already exists.
    pub fn upsert_node(&self, node: NodeData) {
        // Acquire write lock for the entire operation to ensure atomicity between graph and map.
        let mut state = self.state.write();
        let hash = node.hash;

        match state.id_map.get(&hash) {
            Some(&index) => {
                // Update existing node data in the graph.
                state.graph[index] = node;
            }
            None => {
                // Insert new node.
                let index = state.graph.add_node(node);
                state.id_map.insert(hash, index);
            }
        }
    }

    /// Retrieves a node by its SigHash.
    pub fn get_node(&self, hash: SigHash) -> Result<NodeData, ISGError> {
        // Acquire read lock.
        let state = self.state.read();
        
        let index = state.id_map.get(&hash).ok_or(ISGError::NodeNotFound(hash))?;
        
        // Clone the data (cheap due to Arc<str>) to release the read lock quickly.
        Ok(state.graph[*index].clone())
    }

    // TDD Cycle 3: Edge Upsert

    /// Inserts or updates a directed edge between two nodes.
    pub fn upsert_edge(&self, from: SigHash, to: SigHash, kind: EdgeKind) -> Result<(), ISGError> {
        // Acquire write lock.
        let mut state = self.state.write();

        // 1. Resolve indices inside the lock to ensure they exist.
        let from_idx = *state.id_map.get(&from).ok_or(ISGError::NodeNotFound(from))?;
        let to_idx = *state.id_map.get(&to).ok_or(ISGError::NodeNotFound(to))?;

        // 2. Insert/Update the edge using petgraph's update_edge.
        state.graph.update_edge(from_idx, to_idx, kind);
        
        Ok(())
    }

    // TDD Cycle 4: Query Patterns (Traversal)

    /// Query Pattern: who-implements
    /// Finds all nodes that have an 'Implements' relationship pointing TO the target hash.
    pub fn find_implementors(&self, trait_hash: SigHash) -> Result<Vec<NodeData>, ISGError> {
        // Acquire read lock for traversal.
        let state = self.state.read();

        // 1. Resolve index.
        let trait_idx = *state.id_map.get(&trait_hash).ok_or(ISGError::NodeNotFound(trait_hash))?;

        // 2. Traverse incoming edges (reverse traversal).
        let implementors = state.graph.edges_directed(trait_idx, Direction::Incoming)
            .filter_map(|edge| {
                if edge.weight() == &EdgeKind::Implements {
                    let source_idx = edge.source();
                    // Clone the node data.
                    Some(state.graph[source_idx].clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(implementors)
    }

    /// Query Pattern: blast-radius
    /// Calculates all reachable nodes starting from a given node using BFS.
    pub fn calculate_blast_radius(&self, start_hash: SigHash) -> Result<HashSet<SigHash>, ISGError> {
        // Acquire read lock, held for the duration of the traversal.
        let state = self.state.read();

        // 1. Resolve start index.
        let start_idx = *state.id_map.get(&start_hash)
            .ok_or(ISGError::NodeNotFound(start_hash))?;

        let mut reachable = HashSet::new();

        // 2. Initialize BFS traversal walker.
        let bfs = Bfs::new(&state.graph, start_idx);

        // 3. Iterate over the graph.
        // This loop is critical for the <500μs target.
        for node_idx in bfs.iter(&state.graph) {
            // Skip the starting node itself
            if node_idx == start_idx {
                continue;
            }

            // Retrieve the hash from the node weight.
            reachable.insert(state.graph[node_idx].hash);
        }

        Ok(reachable)
    }
}

// --- TDD Test Suite ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // Helper for creating nodes
    fn mock_node(id: u64, kind: NodeKind, name: &str) -> NodeData {
        NodeData {
            hash: SigHash(id),
            kind,
            name: Arc::from(name),
            signature: Arc::from(format!("sig_{}", name)),
        }
    }

    // TDD Cycle 1: Initialization (Red -> Green)
    #[test]
    fn test_isg_initialization() {
        let isg = OptimizedISG::new();
        assert_eq!(isg.node_count(), 0);
        assert_eq!(isg.edge_count(), 0);
    }

    // TDD Cycle 2: Node Upsert and Retrieval (Red -> Green)
    #[test]
    fn test_upsert_and_get_node() {
        let isg = OptimizedISG::new();
        let node1 = mock_node(1, NodeKind::Function, "func_v1");
        let hash1 = node1.hash;

        // 1. Insert
        isg.upsert_node(node1.clone());
        assert_eq!(isg.node_count(), 1);

        // 2. Retrieve
        let retrieved = isg.get_node(hash1);
        assert_eq!(retrieved, Ok(node1));

        // 3. Update (Upsert)
        let node1_v2 = mock_node(1, NodeKind::Function, "func_v2");
        isg.upsert_node(node1_v2.clone());
        assert_eq!(isg.node_count(), 1); // Count should not change
        assert_eq!(isg.get_node(hash1), Ok(node1_v2));

        // 4. Get non-existent
        let result = isg.get_node(SigHash(99));
        assert_eq!(result, Err(ISGError::NodeNotFound(SigHash(99))));
    }

    // TDD Cycle 3: Edge Upsert (Red -> Green)
    #[test]
    fn test_upsert_edge() {
        let isg = OptimizedISG::new();
        let node_a = mock_node(10, NodeKind::Struct, "A");
        let node_b = mock_node(11, NodeKind::Struct, "B");
        isg.upsert_node(node_a.clone());
        isg.upsert_node(node_b.clone());

        // 1. Insert edge
        let result = isg.upsert_edge(node_a.hash, node_b.hash, EdgeKind::Uses);
        assert!(result.is_ok());
        assert_eq!(isg.edge_count(), 1);

        // 2. Idempotency (same edge kind)
        isg.upsert_edge(node_a.hash, node_b.hash, EdgeKind::Uses).unwrap();
        assert_eq!(isg.edge_count(), 1);

        // 3. Update (different edge kind)
        isg.upsert_edge(node_a.hash, node_b.hash, EdgeKind::Calls).unwrap();
        assert_eq!(isg.edge_count(), 1);

        // 4. Non-existent nodes
        let missing = SigHash(99);
        let result_fail = isg.upsert_edge(node_a.hash, missing, EdgeKind::Uses);
        assert_eq!(result_fail, Err(ISGError::NodeNotFound(missing)));
    }
    
    // Helper for setting up a standardized graph structure for queries.
    fn setup_query_graph() -> OptimizedISG {
        let isg = OptimizedISG::new();
        // Setup:
        // FuncA (1) Calls FuncB (2)
        // FuncB (2) Calls StructC (3)
        // StructD (4) Implements TraitT (6)
        // StructE (5) Implements TraitT (6)
        // FuncA (1) Calls TraitT (6)

        isg.upsert_node(mock_node(1, NodeKind::Function, "FuncA"));
        isg.upsert_node(mock_node(2, NodeKind::Function, "FuncB"));
        isg.upsert_node(mock_node(3, NodeKind::Struct, "StructC"));
        isg.upsert_node(mock_node(4, NodeKind::Struct, "StructD"));
        isg.upsert_node(mock_node(5, NodeKind::Struct, "StructE"));
        isg.upsert_node(mock_node(6, NodeKind::Trait, "TraitT"));

        let h = |id| SigHash(id);
        isg.upsert_edge(h(1), h(2), EdgeKind::Calls).unwrap();
        isg.upsert_edge(h(2), h(3), EdgeKind::Calls).unwrap();
        isg.upsert_edge(h(4), h(6), EdgeKind::Implements).unwrap();
        isg.upsert_edge(h(5), h(6), EdgeKind::Implements).unwrap();
        isg.upsert_edge(h(1), h(6), EdgeKind::Calls).unwrap();
        
        // Noise: StructD Uses StructC (should not affect Implementors query)
        isg.upsert_edge(h(4), h(3), EdgeKind::Uses).unwrap();

        isg
    }

    // TDD Cycle 4: Query Patterns (Red -> Green)
    #[test]
    fn test_query_who_implements() {
        let isg = setup_query_graph();
        let trait_hash = SigHash(6);

        // Action: Find implementors of TraitT (6)
        let implementors = isg.find_implementors(trait_hash).unwrap();

        // Assertion: Should be StructD (4) and StructE (5)
        let mut implementor_hashes: Vec<SigHash> = implementors.iter().map(|n| n.hash).collect();
        implementor_hashes.sort();
        assert_eq!(implementor_hashes, vec![SigHash(4), SigHash(5)]);
        
        // Test non-existent trait
        assert_eq!(isg.find_implementors(SigHash(99)), Err(ISGError::NodeNotFound(SigHash(99))));
    }

    #[test]
    fn test_query_blast_radius_bfs() {
        let isg = setup_query_graph();
        let start_hash = SigHash(1); // FuncA

        // Action: Calculate blast radius from FuncA (1)
        let radius = isg.calculate_blast_radius(start_hash).unwrap();

        // Assertion: Should reach B(2), C(3), T(6). D(4) and E(5) are not reachable downstream from A.
        let expected: HashSet<SigHash> = vec![
            SigHash(2), SigHash(3), SigHash(6),
        ].into_iter().collect();
        assert_eq!(radius, expected);

        // Test starting from a leaf node (StructC (3))
        let radius_c = isg.calculate_blast_radius(SigHash(3)).unwrap();
        assert!(radius_c.is_empty());
    }

    // TDD Cycle 5: Concurrency Validation (Red -> Green)
    #[test]
    fn test_concurrent_writes_and_reads() {
        let isg = OptimizedISG::new();
        let isg_w1 = isg.clone();
        let isg_r = isg.clone();
        
        // Writer thread 1 (Nodes 1-100)
        let writer1 = thread::spawn(move || {
            for i in 1..=100 {
                let node = mock_node(i, NodeKind::Struct, &format!("Node_{}", i));
                isg_w1.upsert_node(node);
                // Add an edge from node 1 to this node if i > 1
                if i > 1 {
                    isg_w1.upsert_edge(SigHash(1), SigHash(i), EdgeKind::Uses).unwrap();
                }
            }
        });

        // Reader thread (Continuously attempts traversal from node 1)
        let reader = thread::spawn(move || {
            for _ in 0..500 {
                // Acquiring a read lock and traversing should not cause data races or deadlocks.
                // We might get an error if node 1 hasn't been inserted yet.
                if let Ok(radius) = isg_r.calculate_blast_radius(SigHash(1)) {
                     assert!(radius.len() <= 99);
                }
            }
        });

        writer1.join().unwrap();
        reader.join().unwrap();

        // Final state verification
        assert_eq!(isg.node_count(), 100);
        assert_eq!(isg.edge_count(), 99);
        assert_eq!(isg.calculate_blast_radius(SigHash(1)).unwrap().len(), 99);
    }
}
```


This analysis simulates the expected performance and resource utilization of the `OptimizedISG` architecture across various codebase sizes, from 10K LOC to 50M LOC. We evaluate the memory footprint, graph size, and operation latency against the strict performance constraints (<500μs simple queries, <1ms complex queries).

### 1. Simulation Methodology and Assumptions

We base the simulation on realistic assumptions regarding code density, the memory overhead of the MVP implementation (using `petgraph`, `FxHashMap`, `parking_lot::RwLock`), and the impact of CPU caching on in-memory graph traversal speeds.

1.  **Graph Density**:
    *   Nodes: 1 Node per 15 LOC (Functions, Structs, Traits, etc.).
    *   Edges: Average out-degree of 4 edges per node (E = 4N).
2.  **Memory Footprint**:
    *   Estimated **350 Bytes per Node**. This accounts for `NodeData` (including estimated string data with interning), `petgraph` structural overhead, `FxHashMap` index entry, and allocator overhead.
3.  **Update/Lookup Latency (O(1))**:
    *   Estimated at **1μs - 5μs**. These operations are dominated by lock acquisition and HashMap access.
4.  **Traversal Speed (Variable)**:
    *   Graph traversal speed is heavily dependent on memory locality. We model this using ETePS (Elements Traversed per Second), considering Nodes+Edges as elements.
    *   **L2/L3 Cache Resident (< 50MB)**: 100 Million ETePS.
    *   **Main RAM Resident (> 50MB)**: 30 Million ETePS (Speed limited by memory latency).
5.  **Query Scenarios**:
    *   We analyze latency based on the absolute number of elements traversed, as this is the primary determinant of query time, independent of the total codebase size.

### 2. Simulation Results: Scale and Resources

The following table details the projected metrics across the different scales.

| Scale | LOC | Est. Nodes (V) | Est. Edges (E) | Total RAM (350 B/N) | Cache Behavior | Update/Lookup Latency |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Small | 10K | 667 | 2,668 | 233 KB | L3 Resident | 1μs - 5μs |
| Medium | 50K | 3,333 | 13,332 | 1.17 MB | L3 Resident | 1μs - 5μs |
| Medium+ | 100K | 6,667 | 26,668 | 2.33 MB | L3 Resident | 1μs - 5μs |
| Large | 1M | 66,667 | 266,668 | 23.3 MB | L3 Resident | 1μs - 5μs |
| Enterprise| 10M | 666,667 | 2.67 M | 233 MB | **RAM Resident** | 1μs - 5μs |
| Massive | 50M | 3.33 M | 13.33 M | **1.17 GB** | **RAM Resident** | 1μs - 5μs |

#### Analysis: Resources and O(1) Operations

*   **Memory Usage**: The architecture is memory efficient. Even at 50M LOC, the required RAM (~1.17 GB) is easily manageable by standard developer workstations. The in-memory approach is viable across all scales.
*   **Update/Lookup Latency**: O(1) operations are extremely fast (<5μs). This easily satisfies the <12ms update pipeline requirement.

### 3. Simulation Results: Complex Query Latency

Complex traversals (e.g., Blast Radius BFS) are the critical bottleneck. The latency depends on the ETePS rate and the number of elements visited (V'+E').

We analyze the latency based on the scope of the traversal.

| Traversal Scope (V'+E') | Description | Latency (L3 Resident) 100M ETePS | Latency (RAM Resident) 30M ETePS |
| :--- | :--- | :--- | :--- |
| 500 | Localized impact (e.g., private function) | 5 μs | 16.7 μs |
| 5,000 | Medium impact (e.g., module-level change) | 50 μs | 167 μs |
| 15,000 | Significant impact (e.g., internal library) | 150 μs | 500 μs |
| 30,000 | Major impact (e.g., core utility) | 300 μs | **1000 μs (1ms)** |
| 50,000 | Massive impact (e.g., foundational trait) | 500 μs | **1667 μs (1.67ms)** |

#### Analysis: The Performance Cliff

The simulation reveals a critical performance dynamic:

1.  **Up to 1M LOC (L3 Resident)**: Performance is excellent. The graph (~23MB) fits within L3 cache. The architecture can handle traversals of up to 100,000 elements within the 1ms budget. This provides significant headroom.
2.  **10M LOC and Beyond (RAM Resident)**: When the graph size (~233MB+) exceeds the L3 cache, traversal speed drops by ~3x (from 100M to 30M ETePS) due to main memory latency.
    *   The maximum traversal scope within the 1ms budget drops to 30,000 elements.

**The Bottleneck**: In large codebases (10M+ LOC), it is common for changes to core components to affect more than 30,000 elements. The MVP architecture **fails the <1ms constraint** for such queries at the Enterprise scale.

### 4. Optimization Path and Refined Projections

To meet the constraints at Enterprise scale, the Phase 2 optimizations are necessary to improve memory locality and increase the ETePS rate for RAM-resident graphs.

#### Optimization: Compressed Sparse Row (CSR)

Migrating from `petgraph`'s default adjacency list structure to a Compressed Sparse Row (CSR) format is crucial. CSR uses contiguous arrays, maximizing spatial locality and reducing cache misses.

*   **Projected Impact**: We estimate a conservative 2.5x improvement in traversal speed for RAM-resident graphs (30M ETePS → 75M ETePS).

#### Projected Latency with CSR Optimization (RAM Resident)

| Traversal Scope (V'+E') | Latency (MVP) 30M ETePS | Latency (CSR Optimized) 75M ETePS | Meets <1ms? |
| :--- | :--- | :--- | :--- |
| 15,000 | 500 μs | 200 μs | Yes |
| 30,000 | 1000 μs | 400 μs | Yes |
| 50,000 | 1667 μs | 667 μs | Yes |
| 75,000 | 2500 μs | 1000 μs (1ms) | Yes |
| 100,000 | 3333 μs | 1333 μs (1.3ms) | No |

### 5. Conclusion and Strategic Assessment

The simulation confirms the `OptimizedISG` architecture is sound and resource-efficient.

1.  **MVP (Up to 1M LOC)**: The current implementation meets all constraints with excellent performance.
2.  **Enterprise Scale (10M LOC)**: The MVP implementation fails the <1ms constraint for large-impact queries. Implementing **CSR optimization** is required. With CSR, the architecture can handle traversals up to 75,000 elements within the budget, which should cover most typical complex queries at this scale.
3.  **Massive Scale (50M LOC)**: Even with CSR, the sheer scale means that significant changes (impacting >75,000 elements) will exceed the 1ms budget. To guarantee real-time performance at this scale, the v3.0 architecture (**On-Demand Hydration/Federation**) must be implemented. This ensures the local daemon only manages a relevant subset of the graph, keeping the local structure small and fast.
4.  **Bounded Queries**: For any scale, unbounded "worst-case" queries (e.g., traversing the entire graph) will exceed 1ms. Real-time features should use depth-bounded or time-bounded queries.

This is an insightful exercise that demonstrates how shifting architectural priorities impacts the decision-making process. By heavily increasing the emphasis on Performance and Scalability, and removing Simplicity as a weighted factor, the evaluation shifts to prioritize raw capability and future-proofing.

Here is the recalculated Decision Matrix based on the new weights:

*   **Performance**: 60%
*   **Scalability**: 30%
*   **Rust Integration**: 10%

### Methodology

To ensure a fair comparison, we calculate the average score for the Performance category based on the underlying scores (Query Speed, Update Latency, Memory Efficiency) provided in the initial analysis. We assume equal weighting for these sub-criteria. Scores for Scalability and Rust Integration are carried over directly.

### Recalculated Decision Matrix

| Feature | Weight | SQLite (1.1) | In-Memory (1.2) | SurrealDB (1.3) | OptimizedISG (1.5) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Performance (Raw Scores)** | | | | | |
| Query Speed | | 3 | 9 | 7 | 10 |
| Update Latency | | 8 | 10 | 8 | 10 |
| Memory Efficiency | | 9 | 6 | 7 | 9 |
| **Performance (Avg Score)** | | *(6.67)* | *(8.33)* | *(7.33)* | *(9.67)* |
| **Scalability (Score)** | | *(3.0)* | *(6.0)* | *(9.0)* | *(7.0)* |
| **Rust Integration (Score)**| | *(9.0)* | *(10.0)* | *(10.0)* | *(10.0)* |
| **Weighted Calculation** | | | | | |
| Performance | 60% | 4.00 | 5.00 | 4.40 | 5.80 |
| Scalability | 30% | 0.90 | 1.80 | 2.70 | 2.10 |
| Rust Integration | 10% | 0.90 | 1.00 | 1.00 | 1.00 |
| **TOTAL SCORE** | **100%**| **5.80** | **7.80** | **8.10** | **8.90** |

*(Note: Calculations use exact fractions for precision. For example, SQLite Performance weighted score = (20/3) * 0.6 = 4.00).*

### Analysis of the Outcome

The revised weighting solidifies the leading recommendation but significantly alters the relative standing of the alternatives.

1.  **OptimizedISG (8.90)**
2.  **SurrealDB (8.10)**
3.  **In-Memory (Generic) (7.80)**
4.  **SQLite (5.80)**

#### Key Observations

**1. OptimizedISG Extends Its Lead**
The **Custom OptimizedISG (1.5)** remains the clear winner and increases its lead. This is driven by two major factors:
*   **Performance Emphasis:** The massive 60% weight on Performance aligns perfectly with this architecture's greatest strength (scoring 9.67/10). When the primary goal is meeting the stringent <500μs query latency, this architecture is the definitive choice.
*   **Simplicity Removed:** The removal of the "Simplicity" criteria (previously 25%) eliminates OptimizedISG's main weakness—implementation and maintenance complexity.

**2. SurrealDB Emerges as a Strong Contender**
The most significant shift is **SurrealDB (1.3)** moving into a strong second place, overtaking the Generic In-Memory approach. The doubling of the Scalability weight (from 15% to 30%) heavily benefits SurrealDB, which scored the highest in that category (9/10) due to its native support for distributed, horizontal scaling. If the project anticipated reaching the 10M+ LOC enterprise scale very quickly, SurrealDB would become highly competitive.

**3. Generic In-Memory Declines**
The Generic In-Memory approach (1.2) drops to third place. While fast, its scalability is limited (6/10) compared to solutions designed for distribution, which hurts its score when scalability accounts for 30% of the total.

### Conclusion and Strategic Implications

With the revised weights heavily emphasizing Performance (60%) and Scalability (30%), the **Custom OptimizedISG** architecture is still the definitive recommendation. The overwhelming requirement for raw speed makes it the only viable option to guarantee the project's stringent latency constraints.

However, the strong showing of SurrealDB highlights a critical consideration for the long-term roadmap (v3.0 Enterprise Scale). This suggests a potential evolution for the v3.0 architecture:

*   **Local Daemon (High Performance):** Continue using OptimizedISG to handle the developer's working set and deliver the required sub-millisecond latency.
*   **Centralized Backend (High Scalability):** When handling 10M+ LOC projects, instead of building a complex custom distribution layer, consider introducing a scalable backend like SurrealDB to hold the global graph. The local OptimizedISG would then synchronize subsets of the graph from this backend (On-Demand Hydration).

