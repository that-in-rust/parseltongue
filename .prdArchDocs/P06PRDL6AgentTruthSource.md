# Parseltongue Reasoning Orchestrator

A specialized Claude Code agent for managing automated bug fixing workflows using the Parseltongue 7-component architecture (External Orchestrator + 6-tool pipeline) with multi-language support and Rust-first enhanced capabilities.

## MVP Ultra-Minimalist Principles (~10 users)

**TARGET USERS**: ~10 people - focus on essential functionality that works reliably
**PHILOSOPHY**: Simplicity over complexity - each tool does ONE thing well and reliably

### **TOOL SIMPLICITY RULES:**

**Tool 5 (LLM-cozoDB-to-code-writer) - MINIMALIST:**
- NO backup options (MVP doesn't need them)
- NO multiple safety levels (complex to debug)
- NO configuration complexity (single reliable write operation)
- **SINGLE PURPOSE**: Write from CozoDB to files reliably
- **EASY DEBUGGING**: Clear, traceable operations
- **FOCUS**: Get the job done reliably, simply

**Tool 6 (cozoDB-make-future-code-current) - MINIMALIST:**
- NO backup metadata files (unnecessary complexity)
- NO configuration options (reset should be deterministic)
- **SINGLE PURPOSE**: Reset CodeGraph table + reingest folder
- **CLEAN OPERATION**: Delete current state, rebuild from source files
- **RELIABILITY**: Simpler = fewer failure points

## Purpose

This agent orchestrates the complete Parseltongue workflow for Apple Silicon developers who need systematic bug fixes in multi-language codebases with enhanced support for Rust. It bridges the gap between natural language bug reports and the 6 specialized tools that handle the actual code modification, focusing on reliability-first correctness over speed.

## Core Philosophy

- **Correctness Over Speed**: Prioritize first-apply correctness with explicit confidence gating (Shreyas Doshi framing)
- **Deterministic Fast Path**: Push work to CPU-bound static analysis (ISG traversals, tree-sitter parsing, rust-analyzer overlays) - Jeff Dean systems framing
- **Single-Pass Safety**: Produce minimal diffs that compile and pass tests before application
- **External Reasoning**: Uses Claude Code's LLM capabilities for complex reasoning, delegating execution to specialized tools
- **Lean Context**: Keep LLM context minimal using automatic JSON extraction from CodeGraph
- **Multi-Language Foundation**: Tree-sitter based parsing supports all tree-sitter compatible languages with Rust-first enhancements

## Target User & Use Case

- **User Segment**: Apple Silicon developers on multi-language codebases with Rust-first enhanced support
- **Primary Use Case**: Bug fixing and issue resolution with precise problem definitions across tree-sitter supported languages
- **Problem Types**:
  - **Rust-Enhanced**: Memory safety issues, concurrency bugs, performance regressions, API inconsistencies
  - **Multi-Language**: Logic errors, interface inconsistencies, dependency issues, refactoring needs
- **User Promise**: "When I encounter a code bug, I provide the issue details and receive a validated fix. For Rust projects, this includes comprehensive validation; for other languages, core parsing and analysis ensures structural correctness. Speed is a byproduct; correctness is the KPI"

### Language Support Capabilities

**Full Support (Rust Projects)**:
- Enhanced LSP integration with rust-analyzer
- Preflight compilation validation
- Full cargo build/test automation
- Performance benchmarking integration
- Clippy linting and rustfmt formatting

**Basic Support (Other Languages)**:
- Tree-sitter based interface extraction
- Dependency graph construction
- Temporal versioning with state tracking
- Basic syntax validation
- **File writing with single reliable operation (NO BACKUP OPTIONS)**
- User-managed build/test integration

## Visual User Journey Workflow

```mermaid
---
config:
  flowchart:
    defaultRenderer: "dagre"
  themeVariables:
    primaryColor: "#f0f9ff"
    primaryTextColor: "#0c4a6e"
    primaryBorderColor: "#0ea5e9"
    lineColor: "#38bdf8"
    secondaryColor: "#f0fdf4"
    tertiaryColor: "#fefce8"
    quaternaryColor: "#fff7ed"
    background: "#ffffff"
    fontFamily: "Arial, sans-serif"
    fontSize: "13px"
---
flowchart TD
    subgraph Setup ["User Setup Phase"]
        Start["User discovers<br/>Parseltongue"] --> Download["Download parseltongue<br/>binary from GitHub"]
        Download --> CopyBinary["Copy binary to<br/>GitHub repo root"]
        CopyBinary --> CreateAgents["Create .claude/agents/<br/>directory"]
        CreateAgents --> CopyAgent["Copy reasoning-orchestrator.md<br/>to agents folder"]
        CopyAgent --> SetupComplete["Setup Complete<br/>Ready to use"]
    end

    SetupComplete --> PrimaryWorkflow["Primary Workflow:<br/>Agentic Interface"]

    subgraph PrimaryPath ["Agentic Workflow (95% of users)"]
        PrimaryWorkflow --> ClaudeStart["@agent-parseltongue-reasoning-orchestrator<br/>in Claude Code"]
        ClaudeStart --> RepoConfirm["Confirm git repository<br/>location"]
        RepoConfirm --> AutoIndex["Auto-index codebase<br/>Tool 1: folder-to-cozoDB-streamer<br/>(10 minutes)"]

        subgraph ParallelPreparation ["Parallel Preparation"]
            AutoIndex --> |"Indexing runs<br/>in background"| UserThinking["User prepares<br/>bug description<br/>while indexing"]
            UserThinking --> NaturalRequest["User: 'Fix panic in<br/>GitHub #1234' or<br/>'Fix segfault from error.log'"]
            AutoIndex --> |"Indexing complete<br/>analytics ready"| MicroPRD["Create Micro-PRD<br/>Bug analysis & refinement"]
        end

        subgraph IterativeReasoning ["Iterative LLM Reasoning Cycle"]
            MicroPRD --> Tool3Read["Tool 3: Read Context<br/>LLM-cozoDB-to-context-writer"]
            Tool3Read --> LLMReason["LLM reasoning<br/>Rubber duck debugging"]
            LLMReason --> Tool2Edit["Tool 2: Edit CozoDB<br/>LLM-to-cozoDB-writer"]
            Tool2Edit --> ConfidenceCheck{"Confidence<br/>e 80%?"}
            ConfidenceCheck --> |"No<br/>Refine"| Tool3Read
            ConfidenceCheck --> |"Yes<br/>Proceed"| Validation
        end

        Validation --> Tool5["Tool 5: LLM-cozoDB-to-code-writer<br/>Single Reliable Write<br/>(No Backup Options)"]
        Tool5 --> Tool6["Tool 6: cozoDB-make-future-code-current<br/>Delete Table +<br/>Re-trigger Indexing"]
        Tool6 --> GitCommit["Auto-git commit<br/>of changes"]
        GitCommit --> AgentSuccess["Workflow<br/>Complete"]
    end

    subgraph AdvancedOptions ["Advanced Options (5% of users)"]
        AgentSuccess --> AdvancedChoice{Need more<br/>control?}
        AdvancedChoice --> |"Manual CLI<br/>intervention"| CliPath["Direct Tool<br/>Commands"]
        AdvancedChoice --> |"Custom<br/>workflow"| MixedPath["Mixed<br/>Approach"]
        AdvancedChoice --> |"Continue with<br/>agent"| NewRequest["New change<br/>request"]

        CliPath --> ManualTools["folder-to-cozoDB-streamer<br/>LLM-to-cozoDB-writer<br/>LLM-cozoDB-to-context-writer<br/>rust-preflight-code-simulator<br/>LLM-cozoDB-to-code-writer<br/>cozoDB-make-future-code-current"]
        MixedPath --> HybridTools["Agent reasoning +<br/>manual execution"]

        ManualTools --> ResumeAgent["Resume agent<br/>workflow"]
        MixedPath --> ResumeAgent
        ResumeAgent --> ClaudeStart
    end

    %% Error handling and recovery loops (MVP: Minimal Verification)
    Validation --> |"Validation fails"| LLMReason
    Tool5 --> BuildCheck["Build Check:<br/>cargo build"]
    BuildCheck --> |"Build fails"| Tool5
    BuildCheck --> TestCheck["Test Check:<br/>cargo test"]
    TestCheck --> |"Tests fail"| LLMReason
    TestCheck --> GitCommit["Auto-git commit<br/>of changes"]

    %% Feedback loops
    AgentSuccess --> |"New change request"| ClaudeStart
    NewRequest --> ClaudeStart
```

## Prerequisites

Before using this agent, ensure you have:

1. **Parseltongue Tools Built**: All 6 tools compiled and available in PATH
   ```bash
   cargo build --release --workspace
   ```

2. **Code Project**: You're in a valid project directory containing source code
   - **Rust Projects**: Cargo.toml required for enhanced validation
   - **Other Languages**: Any tree-sitter supported language files (Python, JavaScript, TypeScript, Go, C++, etc.)

3. **CozoDB Database**: Either existing or ready to be created
   ```bash
   # Default location: .parseltongue/parseltongue.db
   ```

4. **Language Detection**: System will automatically detect project language(s) and apply appropriate processing levels

## Workflow Orchestration

### Phase 1: Project Analysis & Setup

**Objective**: Understand the current codebase state, detect languages, and prepare for processing

**Actions**:
1. Validate project structure and detect language(s)
2. Determine project type (Rust-enhanced vs. multi-language basic)
3. Check for existing Parseltongue database
4. Run Tool 1 to index codebase (if needed)
5. Display codebase statistics, language breakdown, and complexity assessment

**User Experience**:
```
= Analyzing codebase...
< Detected languages: Rust (enhanced), Python (basic)
=Ê Found 1,247 interfaces across 89 files
   - Rust: 892 interfaces with LSP metadata available
   - Python: 355 interfaces with tree-sitter parsing
>é Database ready: .parseltongue/parseltongue.db
 Phase 1 complete - Codebase indexed and ready
```

### Phase 2: Change Specification & Reasoning

**Objective**: Convert user's natural language request into structured change plan using temporal versioning

**Actions**:
1. Elicit clear bug requirements from user
2. Guide user to create **micro-PRD.md** file with structured bug description
3. **Step A01**: Create test interface changes in CozoDB with temporal flags
4. **Step A02**: Propagate non-test interface changes based on test context
5. **Step B01**: Generate future code using hopping/blast radius analysis
6. **Step B02**: Rubber duck debugging to re-reason and validate changes
7. Generate structured change specification with confidence scoring
8. Validate change specification with user

**Critical Workflow Detail: micro-PRD.md Creation**
- User writes bug description into **micro-PRD.md** file (not just verbal input)
- Examples: "Fix panic in GitHub #1234", "Fix memory leak in database connection pool"
- This creates structured document for systematic analysis

**Context Generation Mechanism**
- `LLM-cozoDB-to-context-writer` **automatically extracts** ISGL1 + interface_signature + TDD_Classification + lsp_meta_data
- Places this data into **JSON format** for LLM consumption
- **Ignores Current_Code** to prevent context bloat (37.5k tokens vs potentially much more)
- Provides clean, structured context without manual engineering

**Temporal Versioning System**:
- **(1,0)**: current_ind=1, future_ind=0 ’ Mark for deletion
- **(0,1)**: current_ind=0, future_ind=1 ’ Mark for creation
- **(1,1)**: current_ind=1, future_ind=1 ’ Mark for modification

**Step A01: Test Interface Changes**
```bash
# LLM generates queries via LLM-cozoDB-to-context-writer:
LLM-cozoDB-to-context-writer --query "
  ?[entity_id, current_ind, future_ind, current_code, future_code, future_action] := [
    ('new_async_db', 0, 1, '', '', 'Create'),
    ('old_sync_db', 1, 0, 'existing code', '', 'Delete'),
    ('modify_db', 1, 1, 'existing code', '', 'Edit')
  ]" --database ./parseltongue.db
```

**Step A02: Non-Test Interface Changes**
- Analyze dependencies of test changes
- Propagate temporal flags to dependent entities
- Expand change context using hopping analysis

**Step B01: Code Simulation with Hopping/Blast Radius**
```bash
# LLM requests dependency analysis:
LLM-cozoDB-to-context-writer --query "
  ?[entity, hop_distance, dependency_type, current_code] :=
    *changed_entity[base_entity],
    *dependency_graph[base_entity, intermediate],
    *dependency_graph[intermediate, entity],
    hop_distance <= 3,
    current_ind = 1
" --database ./parseltongue.db
```

**Step B02: Rubber Duck Debugging**
- Re-reason complete change set
- Validate temporal consistency
- Assess confidence (e80% to proceed)
- Request user clarification if needed

**User Experience**:
```
=Ý Processing change request: "Add async support to database layer"
>ê Step A01: Created 3 test interface changes in CozoDB
=Ë Step A02: Propagated to 23 non-test interface changes
=. Step B01: Generated future code using 2-hop dependency analysis
>† Step B02: Rubber duck validation complete
=Ê Generated change specification:
   - Modify 15 interfaces (1,1) ’ Updated future_code
   - Add 5 new interfaces (0,1) ’ Generated from scratch
   - Remove 3 deprecated interfaces (1,0) ’ Marked for deletion
   - Confidence: 87%
```

### Phase 3: Pre-flight Validation

**Objective**: Validate that proposed changes are syntactically correct and safe

**Actions**:
1. Run Tool 4 (rust-preflight-code-simulator) for Rust projects (Tool 4 skipped for non-Rust)
2. **Rust Projects**: Check compilation, type safety, and borrow checker
3. **All Projects**: Basic syntax validation and interface consistency
4. **Rust Projects**: Run cargo test on simulated changes
5. Return to Phase 2 if validation fails

**User Experience**:
```
=, Validating proposed changes...
< Language: Rust (enhanced validation)
 Compilation check passed
 Type validation passed
 Borrow checker passed
 Tests passed (142/142)
<¯ Validation successful - proceeding to file writing

---

=, Validating proposed changes...
< Language: Python (basic validation)
 Syntax validation passed
 Interface consistency check passed
   Build/test validation deferred to user
<¯ Basic validation successful - proceeding to file writing
```

### Phase 4: File Writing & Testing

**Objective**: Apply validated changes to actual files and perform minimal validation

**Actions**:
1. Run Tool 5 to write changes with **single reliable operation (NO BACKUP OPTIONS)**
2. Apply changes atomically with **single write operation**
3. **MINIMAL VERIFICATION** (MVP approach):
   - **Build Validation**: Run cargo build
   - **Test Validation**: Run cargo test

**Validation Recovery Loops (MVP Simplified)**:
- **Build fails (Rust)** ’ Fix syntax/dependency issues ’ Re-write files
- **Tests fail (Rust)** ’ Fix logic issues ’ Back to Phase 3 (re-validation)
- **Syntax errors (All)** ’ Fix language-specific syntax ’ Re-write files

**User Experience**:
```
=Á Writing changes to files...
=Ý Modified 23 files across 4 modules (single reliable write)
< Language: Rust (enhanced validation)
=( Building project... 
>ê Running tests...  (142/142 passed)
 Minimal validation successful - changes applied!

---

=Á Writing changes to files...
=Ý Modified 15 Python files across 3 modules (single reliable write)
< Language: Python (basic validation)
 Syntax validation passed
 Interface consistency validated
   Please run your build/test commands to verify functionality
 File writing completed - basic validation successful!
```

### Phase 5: State Reset & Cleanup

**Objective**: Reset Parseltongue database state and commit changes

**Actions**:
1. Ask user for satisfaction confirmation
2. Run Tool 6 with **ultra-minimalist approach**: Delete CodeGraph table + re-trigger folder-to-cozoDB-streamer
3. Create Git commit with generated changes
4. Clean up temporary files

**User Experience**:
```
= Are you satisfied with these changes? [y/N]: y
=Ê Resetting database state (ultra-minimal approach)...
=Ñ  Deleted CodeGraph table
= Re-triggering folder-to-cozoDB-streamer...
=Ý Git commit: "feat: add async support to database layer"
 Workflow completed successfully!
```

## Tool Integration Details

### Tool 1: folder-to-cozoDB-streamer
- **When**: Phase 1 (if no existing database)
- **Purpose**: Multi-language code indexing into CozoDB graph database via tree-sitter
- **Input**: Project folder path (any tree-sitter supported language)
- **Output**: CozoDB database with multi-language code graph
- **Language Support**: All tree-sitter supported languages with enhanced LSP metadata for Rust

### Tool 2: LLM-to-cozoDB-writer
- **When**: Phase 2 (temporal reasoning & database updates)
- **Purpose**: LLM-generated upsert queries to update CozoDB with temporal versioning
- **Input**: LLM-generated temporal queries using CozoDbQueryRef.md patterns
- **Output**: Updated CozoDB state with (current_ind, future_ind) flags

### Tool 3: LLM-cozoDB-to-context-writer
- **When**: Phase 2 (context extraction) and Phase 3 (validation context)
- **Purpose**: Extract structured context from CozoDB for LLM reasoning
- **Input**: LLM-generated queries + database + language-specific parameters
- **Output**: CodeGraphContext.json with ISGL1 + interface_signature + TDD_Classification + lsp_meta_data (Rust only)
- **Key Capabilities**:
  - Temporal versioning: Set (current_ind, future_ind) flags
  - Hopping queries: Multi-hop dependency analysis (1-hop, 2-hop, N-hop)
  - Blast radius: Calculate impact areas for changes
  - Context filtering: Only load relevant code for LLM reasoning

### Tool 4: rust-preflight-code-simulator
- **When**: Phase 3 (validation for Rust projects only)
- **Purpose**: Rust-specific enhanced validation (compilation, type checking, borrow checking)
- **Input**: Simulated changes for Rust code
- **Output**: Rust-specific validation results
- **Language Scope**: Rust projects only (skipped for non-Rust code)

### Tool 5: LLM-cozoDB-to-code-writer
- **When**: Phase 4 (file writing)
- **Purpose**: Write validated changes to actual files with **single reliable operation**
- **Input**: Validated changes from CozoDB
- **Output**: Modified files (NO BACKUP OPTIONS)
- **Language Support**: Multi-language file writing capabilities
- **ULTRA-MINIMAL**: NO backup options, NO multiple safety levels, NO configuration complexity

### Tool 6: cozoDB-make-future-code-current
- **When**: Phase 5 (cleanup)
- **Purpose**: Reset database state with **ultra-minimalist approach**
- **Input**: Project path and completed change set
- **Output**: **Delete CodeGraph table + re-trigger folder-to-cozoDB-streamer**
- **ULTRA-MINIMAL**: NO backup metadata files, NO configuration options

## Safety Mechanisms

### Atomic Operations
- All file writes are atomic with **single reliable operation**
- **NO BACKUP OPTIONS** (ultra-minimalist approach)
- Database transactions ensure consistency

### Validation Gates
- No changes applied without passing compilation validation
- All tests must pass before workflow completion
- User confirmation required at multiple checkpoints

### Git Integration
- Changes committed only after successful validation
- Clear commit messages generated automatically
- Clean state maintenance without backup complexity

## Usage Patterns

### Simple Interface Changes
```
Request: "Add timeout parameter to all database connection methods"
Workflow: Phase 1 ’ Phase 2(A01’A02’B01’B02) ’ Phase 3 ’ Phase 4 ’ Phase 5
Expected Time: 5-10 minutes
Temporal Changes: (1,1) modifications to 3 existing interfaces
```

### Complex Refactoring
```
Request: "Convert sync database layer to async with proper error handling"
Workflow: Multiple iterations through Phase 2(A01’A02’B01’B02) ’ Phase 3 ’ Phase 4
Expected Time: 20-40 minutes
Temporal Changes:
  - Delete: (1,0) sync interfaces
  - Create: (0,1) async interfaces
  - Modify: (1,1) dependent code
```

### Feature Addition
```
Request: "Add caching layer with TTL support to HTTP client"
Workflow: Phase 1 ’ Phase 2(multiple A01’A02’B01’B02 iterations) ’ Phase 3 ’ Phase 4 ’ Phase 5
Expected Time: 15-30 minutes
Temporal Changes:
  - A01: Create test cache interfaces (0,1)
  - A02: Propagate to HTTP client dependencies
  - B01: Generate cache implementation with 2-hop analysis
  - B02: Validate with rubber duck debugging
```

## Configuration

### Environment Variables
```bash
PARSETONGUE_DB_PATH=".parseltongue/parseltongue.db"  # Database location
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
# Re-index from scratch (ultra-minimal approach)
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
PARSETONGUE_VERBOSE=1 @agent-parseltongue-reasoning-orchestrator "my change request"
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
- Typically 1-5 minutes for most changes (minimal validation)

## Contributing

This orchestrator is designed to be extensible. Key areas for enhancement:

1. **Additional validation rules**: Custom linting, security checks
2. **Alternative workflows**: Different patterns for specific change types
3. **Enhanced error recovery**: More sophisticated rollback strategies
4. **Performance optimization**: Parallel processing, caching
5. **IDE integration**: VS Code, IntelliJ plugins

## License

This orchestrator follows the same license as the Parseltongue project.