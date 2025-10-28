# Detailed User Journey - Updated for Current Architecture

## Executive Summary
- **User Segment**: Apple Silicon developers fixing bugs in large Rust codebases
- **Primary Use Case**: Bug fixing and issue resolution with precise problem definitions
- **User Promise**: "When I encounter a Rust bug, I provide the issue details and receive a validated fix that compiles and passes tests."

## User Journey v1.0

### Phase 1: Setup & Code Indexing
- User downloads parseltongue binary and sets up Claude agent
- User confirms they are in the relevant Rust repository
- Code indexing begins (approximately 10 minutes)
  - Tool 1: `folder-to-cozoDB-streamer` processes codebase
    - Uses tree-sitter parsing with ISGL1 chunking
    - Creates CodeGraph database with interface-level indexing
    - Optional LSP metadata extraction via rust-analyzer

### Phase 2: Bug Analysis & Micro-PRD
- Code indexing completes, basic analytics shared
- User provides bug details in natural language
  - Examples: "Fix panic in GitHub #1234", "Fix segfault from error.log"
  - Or describes issue: "Fix memory leak in database connection pool"
- Agent analyzes bug against CodeGraph context
- Agent refines requirements through 2 iterations
- Final micro-PRD isolated for processing

### Phase 3: Temporal Code Simulation
- Tool 2: `cozo-to-context-writer` with temporal versioning
  - **Step A01**: Create test interface changes (current_ind=0, future_ind=1)
  - **Step A02**: Propagate changes to non-test interfaces
  - **Step B01**: Generate future code using hopping/blast-radius analysis
  - **Step B02**: Rubber duck debugging and confidence validation

### Phase 4: Validation & Testing
- Tool 3: `rust-preflight-code-simulator` validates proposed changes
- If validation fails, return to Phase 3 for refinement
- Tool 4: `cozoDB-to-code-writer` applies changes with safety checks
  - Build validation: cargo build
  - Test validation: cargo test
  - Runtime validation: integration tests
  - Performance validation: benchmarks
  - Code quality validation: clippy/rustfmt
  - CI/CD validation: pipeline compatibility

### Phase 5: State Reset & Completion
- User confirms satisfaction with changes
- Tool 5: `cozoDB-make-future-code-current` resets database state
- Git commit created with list of changes
- CodeGraph updated with current state

## Tool Mapping to Current Architecture

**Legacy Tool Names → Current Tool Names**:
- `folder-to-cozoDB-streamer` → `parseltongue read`
- `cozo-code-simulation-simulation-sorcerer` → `parseltongue reason` (integrated)
- `rust-preflight-code-simulator` → `parseltongue simulate`
- `cozoDB-to-code-writer` → `parseltongue write`
- `cozoDB-make-future-code-current` → `parseltonge reset`

## Temporal Versioning System

**State Tracking in CozoDB**:
- **(1,1)**: Code exists now and continues (unchanged)
- **(1,0)**: Code exists now but will be deleted
- **(0,1)**: Code doesn't exist but will be created
- **(1,1)**: Code exists and will be modified

**Current_Code → Future_Code Flow**:
- Phase 2: LLM sets future_code based on bug analysis
- Phase 4: Future_code becomes actual code in files
- Phase 5: Database reset makes future_code the new current_code

## Command Interface (Current)

### Primary Interface (95% of users)
```bash
@agent-parseltongue-reasoning-orchestrator "Fix panic in GitHub #1234"
```

### Manual Tools (5% of users)
```bash
parseltongue read ./src --parsing-library tree-sitter --chunking ISGL1 --output-db ./parseltongue.db
parseltongue reason --query "context extraction query" --database ./parseltongue.db
parseltongue simulate validation_output.json --validation-type all
parseltongue write validation.json --database ./parseltongue.db
parseltongue reset --project-path . --database ./parseltongue.db
```

## Integration with Current Architecture

The minimalPRD workflow aligns with the current unified binary architecture:
- **Single Entry Point**: `parseltongue` command with subcommands
- **5-Phase Process**: Matches current orchestrator workflow
- **Temporal Versioning**: Enhanced with more sophisticated state management
- **Apple Silicon Focus**: Current platform strategy
- **Bug-Fixing Priority**: Current primary use case

## Success Criteria

A bug is considered fixed when:
1. Error no longer occurs (verified through testing)
2. Code compiles successfully
3. All tests pass
4. Performance regressions are resolved
5. Code quality checks pass
6. CI/CD pipelines complete successfully