
# Commands list and examples

## MVP Ultra-Minimalist Principles (~10 users)
**Target**: ~10 users - focus on essential functionality that works reliably
**Philosophy**: Simplicity over complexity - each tool does ONE thing well
**Tool 5 (LLM-cozodb-to-diff-writer)**: NO backup options, NO multiple safety levels, NO configuration complexity (generates CodeDiff.json for LLM)
**Tool 6 (cozoDB-make-future-code-current)**: NO backup metadata files, NO configuration options - Delete table + re-trigger indexing
**Goal**: Maximum reliability through ultra-minimalist approach

#### **Primary Interface: Bug Fixing Workflow (95% of users)**
```bash
# Interactive conversational interface - designed for bug fixing
@agent-parseltongue-reasoning-orchestrator "Fix panic in GitHub #1234"

# Or provide error details directly
@agent-parseltongue-reasoning-orchestrator "Fix segfault from error.log: thread 'main' panicked at 'src/main.rs:42:5'"

# Or describe the issue
@agent-parseltongue-reasoning-orchestrator "Fix memory leak in database connection pool - connections not being dropped"
```

#### **Advanced Options (5% of users)**
The agent workflow includes optional manual intervention for specific needs:

```bash
# Manual tool commands (for power users who need direct control)
folder-to-cozoDB-streamer ./src --parsing-library tree-sitter --chunking ISGL1 --output-db ./parseltongue.db
LLM-to-cozoDB-writer --query-temporal "INSERT INTO Code_Graph VALUES (...)" --database ./parseltongue.db
LLM-cozoDB-to-context-writer --query "SELECT * FROM Code_Graph WHERE current_ind=1" --database ./parseltongue.db --output-context CodeGraphContext.json
rust-preflight-code-simulator validation_output.json --validation-type all
LLM-cozodb-to-diff-writer --database ./parseltongue.db --output CodeDiff.json
cozoDB-make-future-code-current --project-path . --database ./parseltongue.db

# Mixed approach (agent reasoning + manual execution)
@agent-parseltongue-reasoning-orchestrator "Analyze impact of changing auth system"
# Review agent analysis, then execute specific commands as needed
```

### Jobs To Be Done (JTBD)

| **Bug Type** | **Primary: Agent Workflow** | **Advanced: Manual CLI** | **Power Users: Mixed Approach** |
|------------|----------------------------|--------------------------|---------------------------|
| **Panic/Segfault** | ✅ Primary use case - crash analysis | ⚠️ For specific manual debugging | ⚠️ For custom debugging strategies |
| **Logic Errors** | ✅ Root cause analysis + fix | ⚠️ For precise tracing | ⚠️ For complex logic flows |
| **Memory Issues** | ✅ Leak detection + cleanup | ⚠️ For manual memory profiling | ⚠️ For custom optimization |
| **Build Errors** | ✅ Dependency/compilation fixes | ✅ For specific build steps | ❌ Overkill for simple builds |
| **Test Failures** | ✅ Test debugging + fixes | ⚠️ For specific test scenarios | ⚠️ For custom test strategies |
| **Performance Issues** | ✅ Bottleneck analysis + optimization | ⚠️ For manual profiling | ✅ For custom optimization |
| **Security Vulnerabilities** | ✅ Vulnerability analysis + patch | ⚠️ For specific security checks | ⚠️ For custom security workflows |
| **Multiple Bug Fixes** | ❌ One bug at a time | ✅ Scriptable batch fixes | ✅ Custom batch workflows |


