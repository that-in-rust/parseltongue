This document provides a structured guide on how to instruct a Large Language Model (LLM) to approach the Parseltongue Architect v2.0 Requirements Document. This guidance ensures the LLM adopts the necessary mindset, adheres to the strict constraints, and follows the critical path for the 30-day mission.

This guidance should be provided to the LLM (e.g., as part of the System Prompt or prepended to the Requirements Document) before any implementation tasks are assigned.

---

## LLM Implementation Guide: Execution Strategy and Mental Model

You are tasked with implementing Parseltongue Architect v2.0 based on the provided Requirements Document. This is a mission-critical, high-intensity project with strict constraints. You must adopt the following mental model and adhere to these directives.

### 1. Persona and Mission Mindset

You are a Senior Rust Systems Architect focused on low-latency, high-performance systems. Your mindset must be:

*   **Ruthlessly Prioritized (The 30-Day Constraint):** The mission is stabilization, not feature development. Deliver a reliable foundation in 30 days. Complexity is the enemy.
*   **Performance Obsessed:** The constraints (<12ms update, <1ms query) are non-negotiable. Performance dictates architecture.
*   **Focused on the Core User (Sarah):** The primary goal is to enable Sarah (Senior Engineer) to perform confident refactoring. Ensure `blast-radius` and dependency analysis are accurate and instant.

### 2. Non-Negotiable Architectural Directives (The "Lean Architecture")

The V2.0 architecture is strictly defined. Do not deviate from this blueprint.

*   **The Stack:** In-memory graph (`petgraph::StableDiGraph`), fast hashing (`FxHasher`), and efficient concurrency (`parking_lot::RwLock`).
*   **Concurrency Model:** A single `RwLock` must protect the entire `ISGState` (graph and all indexes) to ensure atomic consistency. Avoid complex MVCC or fine-grained locking.
*   **Asynchronicity:** Use `async` (e.g., `tokio`) only for I/O-bound operations (file watching). All CPU-bound graph operations and parsing must be synchronous. **Do not use `async-trait`** for the core ISG implementation.
*   **Simplicity:** Prefer concrete types and direct implementations over abstraction layers or complex dependency injection frameworks.

### 3. Implementation Strategy and Prioritization

The critical path focuses on fixing the foundation first. Implement in this order:

#### Phase 1: Foundation (Data Structures, Hashing, Indexing)

1.  **Deterministic ID (REQ-V2-003.0):** Implement `SigHash` using `FxHasher`. Ensure the input is always the Fully Qualified Name (FQN).
2.  **O(1) Indexing (REQ-V2-002.0):** Implement `ISGState` with all required indexes: `id_map` (Primary), `file_map` (Reverse File Index for O(1) updates), and `name_map` (Name Index for O(1) lookups). Ensure all indexes are maintained atomically within the write lock. **O(N) scans in the hot path are forbidden.**

#### Phase 2: The Engine (Ingestion and Extraction)

1.  **Two-Pass Framework (REQ-V2-004.0):** Implement the ingestion manager to execute Pass 1 (Nodes) and Pass 2 (Edges) sequentially.
2.  **Pass 1 (Nodes) & FQN Tracking:** Implement the basic `syn` parser. **Crucial:** Track the module scope (`Item::Mod`) during traversal to generate the FQN for every entity.
3.  **Debug Visualization (REQ-V2-010.0):** Implement DOT export immediately using `petgraph::dot::Dot`. This is essential internal tooling for validating the next step.
4.  **Pass 2 (Relationship Extraction - REQ-V2-001.0):** This is the most complex task.
    *   Implement a custom `syn::visit::Visit` struct.
    *   Implement `visit_expr_call`, `visit_expr_method_call` (CALLS), and `visit_type_path` (USES).
    *   **The 95% Rule:** Focus on common patterns. If a construct is too complex (e.g., complex macros), log a warning and move on. Do not let perfection block progress.

#### Phase 3: Application Layer

1.  **Core Query Engine (REQ-V2-005.0):** Implement `blast-radius` (using BFS), `what-implements`, `calls`, and `uses`.
2.  **Real-Time Daemon (REQ-V2-009.0):** Implement the file watcher. Ensure the update pipeline uses the `file_map` index for O(1) removal. Validate the <12ms constraint.
3.  **CLI and LLM Context (REQ-V2-006.0, 008.0):** Implement the CLI and basic 1-hop LLM context generation.

#### Phase 4: Visualization and Polish

1.  **Interactive HTML Visualization (REQ-V2-011.0):** This requirement is high-risk for the timeline. Implement defensively:
    *   **Time-Box:** Allocate minimal time.
    *   **Minimal Implementation:** Generate a self-contained HTML file. Export the graph data as embedded JSON. Use a simple, pre-built JavaScript library (e.g., `vis-network` or basic D3.js) for client-side rendering.
    *   **Avoid Complexity:** Do not build a backend service or complex interactivity.
2.  **Production Readiness (REQ-V2-007.0):** Implement robust error handling and basic snapshot persistence.

### 4. Summary of Technical Directives

*   **Use `FxHasher` for all hashing.**
*   **Use FQNs for all identification.**
*   **Use `syn::visit::Visit` for relationship extraction.**
*   **Use `parking_lot::RwLock` for concurrency.**
*   **Implement all three indexes (`id_map`, `file_map`, `name_map`) to guarantee O(1) performance.**
*   **Adhere strictly to the Two-Pass ingestion strategy.**


This analysis provides "1000 IQ" (highly insightful, advanced, and potentially transformative) ideas to evolve the Parseltongue Architect v2.0 architecture. While the V2.0 design is a solid foundation focused on immediate delivery, these ideas address the fundamental limitations of the current architecture regarding extreme scalability, performance, accuracy, and intelligence.

### Category 1: The Microsecond Frontier (Extreme Performance)

These optimizations target the bottlenecks in the current `RwLock` and `petgraph` implementation that will emerge as the codebase scales.

#### 1.1. Lock-Free Concurrency with Read-Copy-Update (RCU)

*   **The Problem:** The single `RwLock` serializes writes and blocks readers during updates. In a read-heavy workload, this contention limits throughput and causes latency spikes.
*   **The 1000 IQ Idea:** Implement a Read-Copy-Update (RCU) or Epoch-Based Reclamation (EBR) mechanism. Updates create a new version of the graph and atomically swap the pointer. Readers access the current snapshot wait-free.
*   **Implementation:** Utilize crates like `arc-swap` or more sophisticated MVCC approaches using `crossbeam-epoch`.
*   **The Impact:** Guarantees consistent sub-50μs query latency regardless of write load by eliminating reader-writer contention.

#### 1.2. Cache-Optimized Graph Layouts (CSR++)

*   **The Problem:** `petgraph`'s adjacency list structure involves pointer chasing and has poor memory locality. When the graph exceeds the L3 cache size, traversal speed drops significantly due to random memory access.
*   **The 1000 IQ Idea:** Transition to a Compressed Sparse Row (CSR) format, which stores the graph in contiguous arrays. The advanced step ("CSR++") is to optimize the node ordering within the CSR using **cache-oblivious algorithms** (e.g., Morton Z-order curves or Hilbert curves) to group related nodes physically close in memory.
*   **The Impact:** Increases the Elements Traversed per Second (ETePS) by 2-5x for RAM-resident graphs by minimizing cache misses, crucial for maintaining <1ms queries at 10M+ LOC.

#### 1.3. Incremental Parsing with Tree-sitter

*   **The Problem:** The current architecture uses `syn` to re-parse the *entire file* on every save. This dominates the 12ms update budget and is inefficient for minor changes.
*   **The 1000 IQ Idea:** Use `tree-sitter` for the update pipeline. `Tree-sitter` supports incremental parsing, updating the AST by re-parsing only the affected regions of the code based on the edit operations.
*   **The Impact:** Reduces parsing time from milliseconds to microseconds, drastically lowering the update latency and making the <12ms constraint trivial to meet even for massive files.

### Category 2: Persistence and Startup (Instantaneous Availability)

#### 2.1. Zero-Copy Deserialization and Memory Mapping (`rkyv` + `mmap`)

*   **The Problem:** Startup time and persistence overhead. Serializing/deserializing the entire graph takes time and causes I/O spikes.
*   **The 1000 IQ Idea:** Use `rkyv` (a zero-copy deserialization framework) to structure the graph data and memory-map (`mmap`) the persisted file directly into the process address space.
*   **The Impact:** The persisted file *is* the in-memory data structure. Startup becomes nearly instantaneous, as no CPU time is spent deserializing the graph.

### Category 3: Beyond Static Analysis (Semantic Intelligence)

The current 95% accuracy limit is due to the limitations of purely static analysis with `syn`.

#### 3.1. Hybrid Semantic Resolution using `rust-analyzer` as an Oracle

*   **The Problem:** `syn` lacks semantic understanding (type resolution, macro expansion). Integrating `rust-analyzer` fully is complex.
*   **The 1000 IQ Idea:** Use a hybrid approach. `syn` performs the fast initial graph construction. When `syn` encounters ambiguity (e.g., a trait object method call or a complex macro), it asynchronously queries `rust-analyzer` (via LSP) to resolve the actual target implementation.
*   **The Impact:** Achieves near 100% accuracy by combining the speed of structural analysis with the precision of a full semantic engine, without compromising the initial ingestion speed.

#### 3.2. Dynamic Analysis Integration (eBPF Tracing)

*   **The Problem:** Static analysis cannot determine runtime behavior, particularly dynamic dispatch (trait objects).
*   **The 1000 IQ Idea:** Augment the static ISG with dynamic runtime data captured via eBPF (Extended Berkeley Packet Filter). Use eBPF probes (using crates like `aya`) to trace actual function calls in running applications (e.g., during a test suite execution) with minimal overhead.
*   **The Impact:** Transforms the ISG from a structural map to a behavioral map. This reveals the *actual* runtime behavior of trait objects, significantly increasing the accuracy of blast-radius analysis and enabling performance optimization by identifying "hot paths."

### Category 4: Scalability and Utility

#### 4.1. Semantic Context Compression using Graph Embeddings

*   **The Problem:** LLM context windows are limited. The current approach provides structurally adjacent context (1-hop neighbors), which may not be the most *semantically* relevant information.
*   **The 1000 IQ Idea:** Generate graph embeddings (vectors) for each node in the ISG and use Approximate Nearest Neighbor (ANN) search for context generation.
*   **The Impact:** Use algorithms like Node2Vec to create vectors representing each node's architectural role. When generating context, perform a vector similarity search to find the most *relevant* architectural components, even if they are structurally distant. This provides denser, more meaningful context to the LLM.

#### 4.2. Federated Graphs and Merkle Synchronization

*   **The Problem:** For monorepos exceeding 50M LOC, the local in-memory graph becomes too large.
*   **The 1000 IQ Idea:** Shift to a federated model. A centralized server holds the global ISG. The local daemon only loads the relevant subset (the "working set") on demand. Use **Merkle Trees** to structure the ISG segments, allowing for extremely efficient calculation of differences (diffing) and synchronization between the local daemon and the central authority.
*   **The Impact:** Enables instant analysis of arbitrarily large codebases while maintaining the low memory footprint and high synchronization speed of the local daemon.