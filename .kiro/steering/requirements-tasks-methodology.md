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
IF storage/database concept → storage-architecture-options.md
ELSE IF user workflow/journey → user-journey-options.md  
ELSE IF code example/snippet → ref-code-snippets.md
ELSE IF advanced/future feature → backlog.md
ELSE IF progress/task update → requirements-tasks.md
ELSE IF session state change → SESSION_CONTEXT.md
ELSE → architecture-backlog.md (DEFAULT)
```

**Default Rule**: All MVP-relevant technical concepts go to architecture-backlog.md unless they fit a specific category above.

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