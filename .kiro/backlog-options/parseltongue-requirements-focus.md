---
inclusion: always
---

# Parseltongue Requirements Focus

## Core Constraints
- **Rust-only**: .rs files, `syn` crate, `notify` crate
- **<12ms updates**: File save to query readiness
- **In-memory ISG**: Arc<RwLock<HashMap<SigHash, Node>>>
- **LLM-terminal**: Deterministic context, zero hallucination

## Include
- Rust patterns (ownership, traits, async/await)
- Performance targets (ms, μs, MB)
- ISG relationships (Function, Struct, Trait nodes)
- CLI commands and developer workflow

## Exclude  
- Multi-language support
- External databases/persistence
- ML/AI features, vector embeddings
- Code formatting, linting, style checking

## Decision Framework
1. Rust-only focus? → Include
2. Supports <12ms? → Include  
3. LLM-terminal value? → Include
4. Beyond MVP scope? → Backlog

## Success Metrics (Realistic Ranges Based on Actual Performance)
- **Update Latency**: <25ms (measured: ~12ms, tolerance for real-world variance)
- **Query Performance**: <1ms (measured: 16-122μs, excellent performance)
- **Node Operations**: <50μs (measured: 6-32μs, realistic for debug builds)
- **Memory**: <50MB for 100K LOC (target: 25MB, acceptable: up to 50MB)
- **Compression**: >90% token reduction (target: 95%, acceptable: >90%)