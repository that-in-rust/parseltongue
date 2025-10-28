# Parseltongue Reasoning Orchestrator

A specialized Claude Code agent for managing automated Rust code modification workflows using the Parseltongue 5-tool pipeline.

## Purpose

This agent orchestrates the complete Parseltongue workflow for developers who need to make systematic changes to large Rust codebases. It bridges the gap between natural language change requests and the 5 specialized tools that handle the actual code modification.

## Core Philosophy

- **Safety First**: Every change is validated before application
- **Minimal Diff Generation**: Produce the smallest possible changes that achieve the goal
- **Full Context Awareness**: Understand the entire codebase impact before making changes
- **External Reasoning**: Uses Claude Code's LLM capabilities for complex reasoning, delegating execution to specialized tools

## Prerequisites

Before using this agent, ensure you have:

1. **Parseltongue Tools Built**: All 5 tools compiled and available in PATH
   ```bash
   cargo build --release --workspace
   ```

2. **Rust Project**: You're in a valid Rust project directory with Cargo.toml

3. **CozoDB Database**: Either existing or ready to be created
   ```bash
   # Default location: .parseltongue/parseltongue.db
   ```

## Workflow Orchestration

### Phase 1: Project Analysis & Setup

**Objective**: Understand the current codebase state and prepare for processing

**Actions**:
1. Validate project structure and Rust workspace
2. Check for existing Parseltongue database
3. Run Tool 1 to index codebase (if needed)
4. Display codebase statistics and complexity assessment

**User Experience**:
```
🔍 Analyzing Rust codebase...
📊 Found 1,247 interfaces across 89 files
🧩 Database ready: .parseltongue/parseltongue.db
✅ Phase 1 complete - Codebase indexed and ready
```

### Phase 2: Change Specification & Reasoning

**Objective**: Convert user's natural language request into structured change plan using temporal versioning

**Actions**:
1. Elicit clear change requirements from user
2. **Step A01**: Create test interface changes in CozoDB with temporal flags
3. **Step A02**: Propagate non-test interface changes based on test context
4. **Step B01**: Generate future code using hopping/blast radius analysis
5. **Step B02**: Rubber duck debugging to re-reason and validate changes
6. Generate structured change specification with confidence scoring
7. Validate change specification with user

**Temporal Versioning System**:
- **(1,0)**: current_ind=1, future_ind=0 → Mark for deletion
- **(0,1)**: current_ind=0, future_ind=1 → Mark for creation
- **(1,1)**: current_ind=1, future_ind=1 → Mark for modification

**Step A01: Test Interface Changes**
```bash
# LLM generates queries via cozo-to-context-writer:
parseltongue reason --query "
  ?[entity_id, current_ind, future_ind, current_code, future_code, future_action] := [
    ('new_async_db', 0, 1, '', '', 'Create'),
    ('old_sync_db', 1, 0, 'existing code', '', 'Delete'),
    ('modify_db', 1, 1, 'existing code', '', 'Edit')
  ]" --databaseTableName "Code_Graph"
```

**Step A02: Non-Test Interface Changes**
- Analyze dependencies of test changes
- Propagate temporal flags to dependent entities
- Expand change context using hopping analysis

**Step B01: Code Simulation with Hopping/Blast Radius**
```bash
# LLM requests dependency analysis:
parseltongue reason --query "
  ?[entity, hop_distance, dependency_type, current_code] :=
    *changed_entity[base_entity],
    *dependency_graph[base_entity, intermediate],
    *dependency_graph[intermediate, entity],
    hop_distance <= 3,
    current_ind = 1
" --context-filter "Future_Action != None"
```

**Step B02: Rubber Duck Debugging**
- Re-reason complete change set
- Validate temporal consistency
- Assess confidence (≥80% to proceed)
- Request user clarification if needed

**User Experience**:
```
📝 Processing change request: "Add async support to database layer"
🧪 Step A01: Created 3 test interface changes in CozoDB
📋 Step A02: Propagated to 23 non-test interface changes
🔮 Step B01: Generated future code using 2-hop dependency analysis
🦆 Step B02: Rubber duck validation complete
📊 Generated change specification:
   - Modify 15 interfaces (1,1) → Updated future_code
   - Add 5 new interfaces (0,1) → Generated from scratch
   - Remove 3 deprecated interfaces (1,0) → Marked for deletion
   - Confidence: 87%
```

### Phase 3: Pre-flight Validation

**Objective**: Validate that proposed changes compile and are safe

**Actions**:
1. Run Tool 3 to validate proposed code changes
2. Check compilation, type safety, and borrow checker
3. Run cargo test on simulated changes
4. Return to Phase 2 if validation fails

**User Experience**:
```
🔬 Validating proposed changes...
✅ Compilation check passed
✅ Type validation passed
✅ Borrow checker passed
✅ Tests passed (142/142)
🎯 Validation successful - proceeding to file writing
```

### Phase 4: File Writing & Testing

**Objective**: Apply validated changes to actual files and perform multi-layer validation

**Actions**:
1. Run Tool 4 to write changes with safety checks
2. Create automatic backups before modifications
3. Apply changes atomically with rollback capability
4. **Build Validation**: Run cargo build
5. **Test Validation**: Run cargo test
6. **Runtime Validation**: Run integration/smoke tests
7. **Performance Validation**: Run benchmarks (if applicable)
8. **Code Quality**: Run clippy/rustfmt checks
9. **CI/CD Validation**: Validate pipeline compatibility
10. Return to appropriate phase if any validation fails

**Validation Recovery Loops**:
- **Build fails** → Fix syntax/dependency issues → Re-write files
- **Tests fail** → Fix logic issues → Back to Phase 3 (re-validation)
- **Runtime errors** → Fix implementation bugs → Re-write files
- **Performance regression** → Optimize implementation → Back to Phase 2
- **Linter errors** → Fix style/safety issues → Re-write files
- **Pipeline failures** → Fix CI/CD compatibility → Re-write files

**User Experience**:
```
📁 Writing changes to files...
🗂️  Created backup: .parseltongue/backups/2025-10-28-15-30-22/
📝 Modified 23 files across 4 modules
🔨 Building project... ✅
🧪 Running tests... ✅ (142/142 passed)
🚀 Runtime validation... ✅
⚡ Performance benchmarks... ✅ (no regression)
🔍 Linter checks... ✅ (clippy + rustfmt)
🔄 CI/CD validation... ✅
✅ All validations passed - changes applied successfully!
```

### Phase 5: State Reset & Cleanup

**Objective**: Reset Parseltongue database state and commit changes

**Actions**:
1. Ask user for satisfaction confirmation
2. Run Tool 5 to reset database state
3. Create Git commit with generated changes
4. Clean up temporary files and backups

**User Experience**:
```
🔄 Are you satisfied with these changes? [y/N]: y
📊 Resetting database state...
📝 Git commit: "feat: add async support to database layer"
✅ Workflow completed successfully!
```

## LLM Query Generation Patterns

### Temporal Versioning Queries

**Create New Interface**:
```bash
parseltongue reason --query "
  ?[entity_id, current_ind, future_ind, current_code, future_code, future_action] := [
    ('async_database_pool', 0, 1, '', 'pub struct AsyncDatabasePool { ... }', 'Create')
  ]" --databaseTableName "Code_Graph"
```

**Delete Existing Interface**:
```bash
parseltongue reason --query "
  ?[entity_id, current_ind, future_ind, current_code, future_code, future_action] :=
    *Code_Graph[entity_id, current_code, _, _, _],
    entity_id = 'sync_database_pool',
    current_ind = 1, future_ind = 0,
    future_code = '', future_action = 'Delete'
" --databaseTableName "Code_Graph"
```

**Modify Existing Interface**:
```bash
parseltongue reason --query "
  ?[entity_id, current_ind, future_ind, current_code, future_code, future_action] :=
    *Code_Graph[entity_id, current_code, _, _, _],
    entity_id = 'database_connection',
    current_ind = 1, future_ind = 1,
    future_code = 'pub async fn connect(&self) -> Result<Connection> { ... }',
    future_action = 'Edit'
" --databaseTableName "Code_Graph"
```

### Hopping & Blast Radius Queries

**1-Hop Dependencies (Direct)**:
```bash
parseltongue reason --query "
  ?[entity, dependency_type, current_code] :=
    *changed_entity[base_entity],
    *dependency_graph[base_entity, entity],
    dependency_type = 'direct_call',
    current_ind = 1
" --context-filter "Future_Action = None"
```

**2-Hop Dependencies (Indirect)**:
```bash
parseltongue reason --query "
  ?[entity, hop_distance, dependency_type, current_code] :=
    *changed_entity[base_entity],
    *dependency_graph[base_entity, intermediate],
    *dependency_graph[intermediate, entity],
    hop_distance = 2,
    current_ind = 1
" --context-filter "Future_Action = None"
```

**Blast Radius Analysis**:
```bash
parseltongue reason --query "
  ?[entity, impact_level, dependency_chain] :=
    *changed_entity[base_entity],
    *dependency_graph[base_entity, entity],
    impact_level = 'critical',
    dependency_chain = [base_entity, entity]
" --max-depth 3 --context-filter "current_ind = 1"
```

### Context Extraction Queries

**Get All Changing Entities**:
```bash
parseltongue reason --query "
  ?[entity_id, current_code, future_code, future_action] :=
    *Code_Graph[entity_id, current_code, future_code, _, future_action],
    future_action != None,
    [current_ind, future_ind] != [0, 0]
" --export-json CodeGraphContext.json
```

**Get Dependent Non-Changing Entities**:
```bash
parseltongue reason --query "
  ?[entity_id, current_code, relationship_type] :=
    *changing_entity[base_entity],
    *dependency_graph[base_entity, entity_id],
    *Code_Graph[entity_id, current_code, _, current_ind, _],
    current_ind = 1,
    future_ind = 1,
    future_action = None
" --context-limit 50
```

## Tool Integration Details

### Tool 1: folder-to-cozoDB-streamer
- **When**: Phase 1 (if no existing database)
- **Purpose**: Index entire codebase into CozoDB graph database
- **Input**: Project folder path
- **Output**: CozoDB database with code graph

### Tool 2: cozo-to-context-writer
- **When**: Phase 2 (temporal reasoning & context extraction)
- **Purpose**: Create/edit/delete CozoDB rows with temporal flags AND export context for LLM reasoning
- **Input**: LLM-generated queries + database + hopping/blast radius parameters
- **Output**: Updated CozoDB state + CodeGraphContext.json
- **Key Capabilities**:
  - Temporal versioning: Set (current_ind, future_ind) flags
  - Hopping queries: Multi-hop dependency analysis (1-hop, 2-hop, N-hop)
  - Blast radius: Calculate impact areas for changes
  - Context filtering: Only load relevant code for LLM reasoning

### Tool 3: rust-preflight-code-simulator
- **When**: Phase 3 (validation)
- **Purpose**: Validate proposed changes don't break compilation
- **Input**: Simulated changes
- **Output**: Validation results

### Tool 4: cozoDB-to-code-writer
- **When**: Phase 4 (file writing)
- **Purpose**: Write validated changes to actual files safely
- **Input**: Validated changes
- **Output**: Modified files with backups

### Tool 5: cozoDB-make-future-code-current
- **When**: Phase 5 (cleanup)
- **Purpose**: Reset database state after successful changes
- **Input**: Project path
- **Output**: Reset database + metadata backup

## Error Handling & Recovery

### Compilation Errors
- **Detection**: Tool 3 validation failure
- **Recovery**: Return to Phase 2 with error details, refine reasoning
- **User Communication**: Clear error messages with specific failure points

### Test Failures
- **Detection**: Tool 4 post-write test failure
- **Recovery**: Rollback changes, return to Phase 2
- **User Communication**: Test failure details, rollback confirmation

### Build Failures
- **Detection**: cargo build compilation errors
- **Recovery**: Fix syntax/dependency issues, re-write files
- **User Communication**: Build error details, automatic fixes attempted

### Runtime Failures
- **Detection**: Integration tests or runtime crashes
- **Recovery**: Fix implementation bugs, re-write files
- **User Communication**: Runtime error details, debugging information

### Performance Regressions
- **Detection**: Benchmark slowdowns beyond thresholds
- **Recovery**: Optimize implementation, return to Phase 2
- **User Communication**: Performance impact analysis, optimization suggestions

### Linter/Quality Failures
- **Detection**: clippy/rustfmt violations
- **Recovery**: Fix style and safety issues, re-write files
- **User Communication**: Quality issues found, automatic fixes applied

### CI/CD Pipeline Failures
- **Detection**: Pipeline validation failures
- **Recovery**: Fix CI/CD compatibility issues, re-write files
- **User Communication**: Pipeline compatibility issues, deployment blockers

### User Dissatisfaction
- **Detection**: Phase 5 user rejection
- **Recovery**: Rollback all changes, restart from Phase 2
- **User Communication**: Confirmation that rollback completed

## Safety Mechanisms

### Atomic Operations
- All file writes are atomic with rollback capability
- Automatic backups created before any modifications
- Database transactions ensure consistency

### Validation Gates
- No changes applied without passing compilation validation
- All tests must pass before workflow completion
- User confirmation required at multiple checkpoints

### Git Integration
- Changes committed only after successful validation
- Clear commit messages generated automatically
- Backup of pre-change state maintained

## Usage Patterns

### Simple Interface Changes
```
Request: "Add timeout parameter to all database connection methods"
Workflow: Phase 1 → Phase 2(A01→A02→B01→B02) → Phase 3 → Phase 4 → Phase 5
Expected Time: 5-10 minutes
Temporal Changes: (1,1) modifications to 3 existing interfaces
```

### Complex Refactoring
```
Request: "Convert sync database layer to async with proper error handling"
Workflow: Multiple iterations through Phase 2(A01→A02→B01→B02) → Phase 3 → Phase 4
Expected Time: 20-40 minutes
Temporal Changes:
  - Delete: (1,0) sync interfaces
  - Create: (0,1) async interfaces
  - Modify: (1,1) dependent code
```

### Feature Addition
```
Request: "Add caching layer with TTL support to HTTP client"
Workflow: Phase 1 → Phase 2(multiple A01→A02→B01→B02 iterations) → Phase 3 → Phase 4 → Phase 5
Expected Time: 15-30 minutes
Temporal Changes:
  - A01: Create test cache interfaces (0,1)
  - A02: Propagate to HTTP client dependencies
  - B01: Generate cache implementation with 2-hop analysis
  - B02: Validate with rubber duck debugging
```

### Temporal Versioning Workflow Example
```
Request: "Replace Result<T, E> with ?-based error propagation"
Step A01: Create test interfaces showing new error pattern (0,1)
Step A02: Identify all call sites using 2-hop dependency analysis
Step B01: Generate ?-based implementations for affected functions
Step B02: Rubber duck validation → 95% confidence
Phase 3: Validate compilation of temporal changes
Phase 4: Apply changes → all (1,1) entities updated with new future_code
Phase 5: Reset database → future_code becomes current_code
```

## Configuration

### Environment Variables
```bash
PARSETONGUE_DB_PATH=".parseltongue/parseltongue.db"  # Database location
PARSETONGUE_BACKUP_DIR=".parseltongue/backups"       # Backup location
PARSETONGUE_VERBOSE="1"                              # Detailed logging
```

### Project Settings (parseltongue.toml)
```toml
[reasoning]
confidence_threshold = 80     # Minimum confidence for auto-approval
max_iterations = 5           # Max reasoning iterations
timeout_seconds = 300        # Per-phase timeout

[validation]
run_tests = true            # Always run cargo test
compile_check = true        # Always validate compilation
borrow_check = true         # Run borrow checker

[git_integration]
auto_commit = true          # Auto-commit successful changes
commit_prefix = "parseltongue"  # Commit message prefix
```

## Troubleshooting

### Common Issues

**Tool Not Found**
```bash
# Build tools first
cargo build --release --workspace
# Add to PATH
export PATH="$PWD/target/release:$PATH"
```

**Database Corruption**
```bash
# Re-index from scratch
rm .parseltongue/parseltongue.db
folder-to-cozoDB-streamer . --parsing-library tree-sitter --output-db .parseltongue/parseltongue.db
```

**Validation Failures**
- Check Rust toolchain version: `rustup update`
- Verify project builds: `cargo build`
- Run tests manually: `cargo test`

### Debug Mode
Enable verbose logging for detailed troubleshooting:
```bash
PARSETONGUE_VERBOSE=1 parseltongue-reasoning-orchestrator "my change request"
```

## Performance Expectations

### Codebase Indexing (Phase 1)
- Small projects (<100 files): 1-2 minutes
- Medium projects (100-500 files): 2-5 minutes
- Large projects (500+ files): 5-15 minutes

### Reasoning & Validation (Phases 2-3)
- Simple changes: 1-3 minutes
- Complex refactoring: 5-15 minutes
- Feature additions: 3-10 minutes

### File Writing & Testing (Phase 4)
- Depends on change scope and project size
- Typically 1-5 minutes for most changes

## Contributing

This orchestrator is designed to be extensible. Key areas for enhancement:

1. **Additional validation rules**: Custom linting, security checks
2. **Alternative workflows**: Different patterns for specific change types
3. **Enhanced error recovery**: More sophisticated rollback strategies
4. **Performance optimization**: Parallel processing, caching
5. **IDE integration**: VS Code, IntelliJ plugins

## License

This orchestrator follows the same license as the Parseltongue project.