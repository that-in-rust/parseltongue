# Requirements-Tasks Methodology

## Information Classification

### MVP Constraints (Parseltongue AIM Daemon)
- **Rust-only**: .rs files, `syn` crate parsing
- **<12ms updates**: File save to query readiness
- **In-memory ISG**: Arc<RwLock<HashMap<SigHash, Node>>>
- **LLM-terminal**: Deterministic context generation

### Document Analysis Flow
1. **Read 1000 lines max** per chunk
2. **Classify information** by type and MVP relevance
3. **Route to appropriate docs** (see Information Routing)
4. **Update progress** in requirements-tasks.md
5. **Mark chunk complete** with ✅ status

### Information Routing (Decision Tree)

```
IF (database, storage, persistence, SQLite, in-memory, caching, CRUD, transactions, durability, WAL, indexes) 
   → storage-architecture-options.md

ELSE IF (user story, workflow, CLI commands, terminal usage, developer experience, use cases, scenarios, personas)
   → user-journey-options.md

ELSE IF (code samples, implementation examples, Rust snippets, function signatures, struct definitions, trait impls)
   → ref-code-snippets.md

ELSE IF (Rust idioms, ownership patterns, borrowing, lifetimes, Arc/Rc, async/await, error handling, Result<T,E>, Option<T>, trait objects, generics, macros)
   → rust-patterns-analysis.md

ELSE IF (v2.0, v3.0, enterprise, distributed, multi-language, complex features, nice-to-have, post-MVP)
   → backlog.md

ELSE IF (task completion, progress updates, milestone tracking, phase transitions, analysis status)
   → requirements-tasks.md

ELSE IF (current session, next actions, priority tasks, context recovery, live status)
   → SESSION_CONTEXT.md

ELSE IF (performance, concurrency, memory, algorithms, data structures, Rust patterns, ISG design, <12ms constraints)
   → architecture-backlog.md (DEFAULT)
```

**Default Rule**: MVP-relevant technical architecture concepts default to architecture-backlog.md

### Task Hierarchy
- **Phase 1**: Document Analysis (current)
- **Phase 2**: Design Document Creation
- **Phase 3**: Implementation Planning

### Backlog Decision
Move to backlog if:
1. Not Rust-only
2. Compromises <12ms performance
3. Requires external storage
4. Beyond MVP scope