# pt08-semantic-atom-cluster-builder

**Status**: 🚧 Foundation Phase (Tests Written, Implementation Deferred)

## Minimalist Approach (Shreyas Doshi Principle)

This crate follows **executable specifications** over immediate implementation:

✅ **Core types defined** - Entity, Edge, Cluster, QualityMetrics
✅ **Error handling structured** - thiserror for library errors
✅ **Test contracts written** - 7 tests documenting API expectations
⏳ **Implementation deferred** - Will be built when actually needed

## Why Tests Are Ignored

Following S01-README-MOSTIMP.md:
- Tests with `#[ignore]` are **living documentation**, not stubs
- They define the contract without forcing immediate implementation
- This avoids building features before they're needed

## Architecture (Layered L1→L2→L3)

```
L1 Core:    Pure clustering algorithms (no external deps in core logic)
L2 Standard: Collections (FnvHashMap), iterators, error propagation
L3 External: Petgraph (graphs), Serde (serialization), Rayon (parallel)
```

## Planned Algorithms (v0.9.5+)

When implementation begins, these algorithms will be built **in order**:

1. **Label Propagation (LPA)** - Fast baseline (~300 LOC)
   - Runtime: <0.5s for 1,500 entities
   - No hyperparameters
   - Tests: 7 contracts already written

2. **Louvain Modularity** - Production quality (~800 LOC)
   - Runtime: <1.5s for 1,500 entities
   - Hierarchical output
   - Tests: TBD

3. **Hierarchical Agglomerative** - Multi-level clustering (~700 LOC)
   - Runtime: <4.0s for 1,500 entities
   - Dendrogram structure
   - Tests: TBD

## Test Contracts (Documented)

All tests follow **precondition/postcondition/error** format:

```rust
/// Preconditions: entities.len() > 0
/// Postconditions: Returns 1-N clusters, each entity assigned once
/// Performance: <500ms for 1,500 entities
/// Error Conditions: EmptyGraph if no entities
```

Run ignored tests to see contracts:
```bash
cargo test --package pt08-semantic-atom-cluster-builder -- --ignored
```

## 4-Word Naming Convention

All public APIs use exactly 4 words:

```rust
run_label_propagation_algorithm_fast()
cluster_unique_identifier_string
quality_metrics_overall_computed
cohesion_internal_density_score
```

## Current API (Types Only)

```rust
use pt08_semantic_atom_cluster_builder::*;

// Types ready to use
let entity = EntityForClustering { ... };
let edge = EdgeForClustering { ... };

// Algorithm (not yet implemented - returns todo!())
let result = run_label_propagation_algorithm_fast(&entities, &edges);
// This will panic until implementation in v0.9.5
```

## Next Steps

When clustering is actually needed:
1. Remove `#[ignore]` from tests
2. Run tests (they'll fail - RED phase)
3. Implement LPA algorithm (GREEN phase)
4. Refactor for clarity (REFACTOR phase)

## Dependencies

- **thiserror**: Structured error handling (L1)
- **itertools**: Functional iteration (L2)
- **fnv**: Fast hashing (L2)
- **petgraph**: Graph algorithms (L3)
- **rayon**: Parallel processing (L3)
- **serde/serde_json**: Serialization (L3)
- **chrono**: Timestamps (L3)

## License

MIT OR Apache-2.0
