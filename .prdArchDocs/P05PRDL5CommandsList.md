
# Commands list and examples

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
parseltongue read ./src --parsing-library tree-sitter --chunking-method ISGL1 --output-db ./parseltongue.db
parseltongue reason --query "context extraction query" --database ./parseltongue.db
parseltongue simulate validation_output.json --validation-type all --timeout 300
parseltongue write validation_output.json --database ./parseltongue.db --backup-dir ./backups
parseltongue reset --project-path . --database ./parseltongue.db

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


