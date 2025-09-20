# Design Document

## Introduction

This document defines the technical architecture for Parseltongue AIM Daemon based on the OptimizedISG design pattern. The system implements a high-performance, in-memory Interface Signature Graph optimized for Rust-only codebases with sub-millisecond query performance.

## Primary Design: OptimizedISG Architecture

**Source**: DeepThink20250920v1.md analysis  
**Status**: ✅ **SELECTED** as MVP architecture

### Core Architecture

```rust
pub struct OptimizedISG {
    state: Arc<RwLock<ISGState>>,
}

struct ISGState {
    graph: StableDiGraph<NodeData, EdgeKind>,  // petgraph for algorithms
    id_map: FxHashMap<SigHash, NodeIndex>,     // O(1) lookups
}
```

### Data Models

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SigHash(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind { Function, Struct, Trait }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeData {
    pub hash: SigHash,
    pub kind: NodeKind,
    pub name: Arc<str>,
    pub signature: Arc<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeKind { Calls, Implements, Uses }
```

### Performance Characteristics

- **Memory**: 350 bytes/node, L3 cache resident up to 1M LOC
- **Updates**: 1-5μs (O(1) operations) - meets <12ms constraint
- **Simple queries**: <5μs (O(1) lookups)
- **Complex queries**: <500μs (BFS traversal) - meets sub-millisecond target
- **Concurrency**: Single RwLock ensures atomic consistency

### Key Operations

1. **Node Management**: `upsert_node()`, `get_node()` - O(1) operations
2. **Edge Management**: `upsert_edge()` - atomic updates
3. **Query Patterns**: 
   - `find_implementors()` - who-implements queries
   - `calculate_blast_radius()` - impact analysis via BFS

### Dependencies

```toml
[dependencies]
petgraph = "0.6"      # Graph algorithms
parking_lot = "0.12"  # High-performance RwLock
fxhash = "0.2"        # Fast hashing
thiserror = "1.0"     # Error handling
```

## Requirements Alignment

- ✅ **REQ-FUNC-001.0**: Function/Struct/Trait nodes with CALLS/IMPL/USES edges
- ✅ **REQ-PERF-001.0**: SigHash-based identification, <12ms updates, sub-ms queries
- ✅ **Rust-only focus**: Native Rust types and `syn` crate integration
- ✅ **LLM-terminal integration**: Deterministic context generation

## Implementation Status

- ✅ **Complete TDD implementation** available in DeepThink20250920v1.md
- ✅ **Comprehensive test suite** with concurrency validation
- ✅ **Production-ready code** with proper error handling
- 🔄 **Integration pending**: File monitoring, `syn` parsing, CLI interface