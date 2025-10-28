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

**Objective**: Convert user's natural language request into structured change plan

**Actions**:
1. Elicit clear change requirements from user
2. Run Tool 2 to extract relevant code context as JSON
3. Perform LLM reasoning on the change requirements
4. Generate structured change specification with confidence scoring
5. Validate change specification with user

**User Experience**:
```
📝 Processing change request: "Add async support to database layer"
📤 Extracted relevant code context (23 interfaces)
🧠 Reasoning about change impact...
📋 Generated change specification:
   - Modify 15 interfaces
   - Add 5 new interfaces
   - Update 3 modules
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

**Objective**: Apply validated changes to actual files

**Actions**:
1. Run Tool 4 to write changes with safety checks
2. Create automatic backups before modifications
3. Apply changes atomically with rollback capability
4. Run cargo build and cargo test on real codebase
5. Return to Phase 2 if tests fail

**User Experience**:
```
📁 Writing changes to files...
🗂️  Created backup: .parseltongue/backups/2025-10-28-15-30-22/
📝 Modified 23 files across 4 modules
🔨 Building project... ✅
🧪 Running tests... ✅ (142/142 passed)
✅ Changes applied successfully!
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

## Tool Integration Details

### Tool 1: folder-to-cozoDB-streamer
- **When**: Phase 1 (if no existing database)
- **Purpose**: Index entire codebase into CozoDB graph database
- **Input**: Project folder path
- **Output**: CozoDB database with code graph

### Tool 2: cozo-reasoning-writer
- **When**: Phase 2 (context extraction)
- **Purpose**: Export relevant code context as JSON for LLM reasoning
- **Input**: Micro-PRD + database
- **Output**: JSON context file

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
Workflow: Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5
Expected Time: 5-10 minutes
```

### Complex Refactoring
```
Request: "Convert sync database layer to async with proper error handling"
Workflow: Multiple iterations through Phase 2 → Phase 3 → Phase 4
Expected Time: 20-40 minutes
```

### Feature Addition
```
Request: "Add caching layer with TTL support to HTTP client"
Workflow: Phase 1 → Phase 2 (multiple iterations) → Phase 3 → Phase 4 → Phase 5
Expected Time: 15-30 minutes
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