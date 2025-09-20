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

### Information Routing
- **MVP architecture patterns** → architecture-backlog.md
- **Storage/database concepts** → storage-architecture-options.md
- **Code examples/snippets** → ref-code-snippets.md
- **User workflows/journeys** → requirements.md (enhance user stories)
- **Advanced/future features** → backlog.md
- **Progress tracking** → requirements-tasks.md
- **Session state** → SESSION_CONTEXT.md

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