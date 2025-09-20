# Requirements-Tasks Methodology

## Information Classification

### MVP Constraints (Parseltongue AIM Daemon)
- **Rust-only**: .rs files, `syn` crate parsing
- **<12ms updates**: File save to query readiness
- **In-memory ISG**: Arc<RwLock<HashMap<SigHash, Node>>>
- **LLM-terminal**: Deterministic context generation

### Document Analysis Flow
1. **Read 1000 lines max** per chunk
2. **Extract MVP concepts** → architecture-backlog.md
3. **Non-MVP concepts** → version backlogs
4. **Update progress** in requirements-tasks.md

### Information Routing
- **Architecture patterns** → architecture-backlog.md
- **Implementation details** → design.md (when ready)
- **Advanced features** → backlog.md
- **Progress tracking** → requirements-tasks.md

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