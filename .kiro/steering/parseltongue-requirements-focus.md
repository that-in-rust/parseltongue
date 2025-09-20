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

## Success Metrics
- **Update Latency**: <12ms
- **Query Performance**: <500μs
- **Memory**: <25MB for 100K LOC
- **Compression**: >95% token reduction