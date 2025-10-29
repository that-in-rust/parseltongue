# 1.0 Minimal User Journey

## 1.1 Executive Summary for Parseltongue
- **User Segment**: Apple Silicon developers on multi-language codebases with Rust-first support
- **Language Support**: Tree-sitter based parsing for all supported languages, with enhanced LSP integration for Rust
- **Reliability-First Principle**:
    - Optimize for accurate 1-go fixes that feel trustworthy and increase user efficacy
    - Prefer CPU-bound static analysis (tree-sitter parsing, ISG traversals) and small, local, free subagents
    - Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible
- **Shreyas Doshi (product framing)**: Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome
- **Jeff Dean (systems framing)**: Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, tree-sitter, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates
- **User Promise**: "When I encounter a code bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. For Rust projects, this includes full LSP-enhanced validation; for other languages, core parsing and analysis is provided. Speed is a byproduct; correctness is the KPI"

## 1.2 Step-by-Step User Journey (Practical Implementation Guide)

**Purpose**: Detailed command-by-command walkthrough for implementers and users who need hands-on guidance.
**Audience**: Developers who prefer step-by-step instructions with specific tool commands and exact parameters.
**Format**: Granular bullet points with tool names, commands, and expected outputs.

*For users who need detailed commands and granular step-by-step instructions*
- User downloads parseltongue binary and sets up Claude agent
- User confirms they are in the relevant Rust repository
    - If no, ask them to share absolute path of git repo and cd there
    - If yes:
        - Tell user that code indexing has begun and will take 10 minutes
            - For the Rust repository:
                - Trigger the following tools:
                    - Tool 1: `folder-to-cozoDB-streamer ./src --parsing-library tree-sitter --chunking ISGL1 --output-db ./parseltongue.db`
                        - Tool will read code from the git repo where it's located, using tree-sitter
                        - Tool will choose granularity of chunks (ISGL1 method)
                        - Tool will call LSP (rust-analyzer) for creating lsp-meta-data
                        - Tool will output aggregated-primarykey + code-chunk-raw + tree-sitter-signature + TDD_classification + lsp_meta_data (optional)
                    - `folder-to-cozoDB-streamer` creates CodeGraph (single write surface):
                        - Indexed by ISGL1 key (filepath-filename-InterfaceName)
                        - Columns (minimal, opinionated):
                            - ISGL1 primary key (aggregated-primarykey)
                                - Current_Code (code-chunk from parsing, can be empty if upsert of new ISGL1 + other fields happen)
                                - interface_signature (tree-sitter-signature, optional)
                                - TDD_Classification (TEST_IMPLEMENTATION or CODE_IMPLEMENTATION)
                                - current_ind (1 by default at time of ingestion)
                                - lsp_meta_data (LSP metadata, optional)
                            - Empty columns:
                                - Future_Code (by default empty, edited by action of reasoning LLM)
                                - Future_Action (by default None, edited by action of reasoning LLM to be None|Create|Edit|Delete)
                                - future_ind (0/1: 0 meaning NOT in future code, 1 meaning in future code)
                - Tell user that code indexing is completed and basic analytics of the CodeGraph table is shared
                - User is now asked to describe their bug/micro-PRD in micro-PRD.md
                - User describes the bug in text form (examples: "Fix panic in GitHub #1234", "Fix memory leak in database connection pool")
                    - The reasoning-LLM (default LLM via ANTHROPIC_KEY) analyzes the micro-PRD using CodeGraphContext.json which contains ISGL1 + interface_signature + TDD_Classification + lsp_meta_data; we will ignore the Current_Code because it would unnecessarily bloat the context
                        - Rough calculation of context in the reasoning-LLM = 1250000 tokens at 300 lines:
                            - Avg interface size is 1000 to 1500 nodes
                            - 1500 nodes x 3 tokens for ISGL1 = 4500 tokens
                            - 1500 nodes x 7 tokens for interface_signature = 10500 tokens
                            - 1500 nodes x 1 tokens for TDD_Classification = 1500 tokens
                            - 1500 nodes x 15 tokens for lsp_meta_data = 22500 tokens
                        - Total = 37.5k tokens
                        - And micro-PRD = 5k tokens + 3 iterations = 20k tokens
                        - Under 100k tokens
                    - The reasoning-LLM will analyze then suggest changes to the micro-PRD to make it clearer in terms of what changes the user wants:
                        - Tests wise
                        - Behavior wise
                        - Functionality wise
                    - After 2 iterations the reasoning-LLM will accept the micro-PRD
                    - Ask the reasoning LLM to reset the context because likely it will overflow and micro-PRD final needs to be isolated
                - Tool 3: `LLM-cozoDB-to-context-writer --query "Select * from Code_Graph where current_ind=1" --database ./parseltongue.db --output-context CodeGraphContext.json` is triggered
                    - Use TDD_idiomatic_rust_steering_doc for all LLM-cozoDB-to-context-writer operations while reasoning through code
                    - Tool 3 creates CodeGraphContext.json containing base-context-area which is micro-PRD + filter(Code_Graph with current_ind=1)=>(ISGL1 + interface_signature + TDD_Classification + lsp_meta_data)
                    - Tool 2: `LLM-to-cozoDB-writer` enables the reasoning-LLM to update CozoDB with temporal versioning using upsert queries generated from CozoDbQueryRef.md patterns
                    - Tool 2 asks the reasoning-LLM to suggest the following to the Code-Graph based on base-context-area:
                        - Step A: ISG level simulations (Temporal Versioning)
                            - Step A01: Create Edit Delete Test Interface Rows; call these changes test-interface-changes:
                                - Addition Interfaces: New ISGL1 rows with current_ind = 0 & future_ind = 1 & Current_Code = empty & Future_Code = empty & Future_Action = Create
                                - Deletion Interfaces: Old ISGL1 rows with current_ind = 1 & future_ind = 0 & Future_Code = empty & Future_Action = Delete
                                - Edit Interfaces: Old ISGL1 rows with current_ind = 1 & future_ind = 1 & Future_Action = Edit
                            - Step A02: Based on test-interface-changes + base-context-area, create edit delete non-test interfaces; call these rows non-test-interface-changes:
                                - Addition Interfaces: New ISGL1 rows with current_ind = 0 & future_ind = 1 & Current_Code = empty & Future_Code = empty & Future_Action = Create
                                - Deletion Interfaces: Old ISGL1 rows with current_ind = 1 & future_ind = 0 & Future_Code = empty & Future_Action = Delete
                                - Edit Interfaces: Old ISGL1 rows with current_ind = 1 & future_ind = 1 & Future_Action = Edit
                        - Step B: Code Simulation
                            - Step B01: Based on filter(Future_Action != None)=>(all fields of Code_Graph including current code) + base-context-area, update future_code for all the rows that are changing:
                                - The reasoning-LLM can use hopping or blast-radius actions on Code_Graph to fetch all information for rows where (Future_Action = None); meaning for rows which are not changing, current_code should not bloat the reasoning-LLM context
                                    - Hopping or blast-radius actions can be CLI options but preferably since our LLM is smart enough they need not be, and we can define them precisely in our supporting MD files
                            - Step B02: Follow rubber duck debugging to re-reason filter(Future_Action != None)=>(all fields of Code_Graph including current code) + base-context-area:
                                - If the LLM thinks that we need to refine the solutioning further, repeat Steps A01 A02 and then basis them repeat Steps B01
                                - If the LLM doesn't feel confident of the changes, it should speak to the user to get additional context or web help, sharing their current understanding in an MD file
                                - If the LLM feels confident of the changes, we move to next step
                        - Step C: Tool 4: `rust-preflight-code-simulator validation_output.json --validation-type all` triggered for Rust use cases with rust-analyzer overlay:
                            - If the rust-preflight-code-simulator tool fails then we go back to previous steps A01 onwards
                            - If the rust-preflight-code-simulator tool passes then we move to next step
                        - Step D: Run Tool 5: `LLM-cozoDB-to-code-writer validation.json --database ./parseltongue.db`:
                            - Step D01: Write the changes to code files with automatic backups
                            - Step D02: Run cargo build
                            - Step D03: Run cargo test
                            - Step D04: Run runtime validation (integration tests)
                            - Step D05: Run performance benchmarks
                            - Step D06: Run code quality checks (clippy/rustfmt)
                            - Step D07: Run CI/CD validation
                            - Step D08: If any validation fails, go back to previous steps A01 onwards with specific error details
                            - Step D09: If all validations pass, we move to next step
            - Ask user if they are satisfied with how the code is working:
                - If yes, trigger Tool 6: `cozoDB-make-future-code-current --project-path . --database ./parseltongue.db`:
                    - `cozoDB-make-future-code-current` creates a git commit with list of changes
                    - `cozoDB-make-future-code-current` resets the CodeGraph and updates all rows in CozoDB database, making future_code the new current_code

# 2.0 Detailed User Journey - Updated for Current Architecture

## 2.1 Executive Summary
- **User Segment**: Apple Silicon developers fixing bugs in multi-language codebases with Rust-first enhanced support
- **Primary Use Case**: Bug fixing and issue resolution with precise problem definitions across tree-sitter supported languages
- **Language Capabilities**:
  - **Full Support (Rust)**: Enhanced LSP integration, preflight validation, full build/test automation
  - **Basic Support (Other Languages)**: Tree-sitter parsing, interface extraction, temporal versioning, basic syntax validation
- **User Promise**: "When I encounter a code bug, I provide the issue details and receive a validated fix. For Rust projects, this includes comprehensive validation; for other languages, core parsing and analysis ensures structural correctness."

## 2.2 User Journey v0.9 (Updated for Current Architecture)

**Purpose**: Architectural overview focusing on system components and data flow patterns.
**Audience**: System architects and technical leads who need to understand the 4-entity architecture and component interactions.
**Format**: Phase-based workflow with emphasis on architectural decisions and tool responsibilities.

- Phase 1: Setup & Code Indexing
    - User downloads parseltongue binary and sets up Claude agent
    - User confirms they are in the relevant Rust repository
    - Code indexing begins (approximately 10 minutes)
        - Tool 1: `folder-to-cozoDB-streamer` processes codebase
            - Uses tree-sitter parsing with ISGL1 chunking
            - Creates CodeGraph database with interface-level indexing
            - Optional LSP metadata extraction via rust-analyzer

- Phase 2: Bug Analysis & Micro-PRD
    - Code indexing completes, basic analytics shared
    - User provides bug details in natural language
        - Examples: "Fix panic in GitHub #1234", "Fix segfault from error.log"
        - Or describes issue: "Fix memory leak in database connection pool"
    - LLM (Entity 1) analyzes bug using 4-entity architecture:
        - **Step 1**: Uses `LLM-cozoDB-to-context-writer` to query CozoDB (Entity 2)
        - **Step 2**: Receives structured context via CodeGraphContext.json (Entity 3)
        - **Step 3**: Refines requirements through 2 iterations using CozoDbQueryRef.md patterns
    - Final micro-PRD isolated for processing

- Phase 3: Temporal Code Simulation
    - Tool 2: `LLM-to-cozoDB-writer` enables LLM to update CozoDB with temporal versioning using CozoDbQueryRef.md patterns
        - **Step A01**: LLM generates temporal upsert queries to create test interface changes (current_ind=0, future_ind=1, Future_Action="Create")
        - **Step A02**: LLM propagates changes to non-test interfaces based on dependency analysis
    - Tool 3: `LLM-cozoDB-to-context-writer` extracts updated context using LLM-generated queries
        - **Step B01**: Generate future code using hopping/blast-radius analysis:
            - LLM queries CozoDB for dependency chains using ISG patterns from archive
            - Applies proven hopping/blast-radius algorithms converted to CozoDB queries
            - Generates minimal, focused future_code for (current_ind=1, future_ind=1) entities
        - **Step B02**: Rubber duck debugging and confidence validation (≥80% to proceed)

- Phase 4: Validation & Testing
    - Tool 4: `rust-preflight-code-simulator` validates proposed changes
    - If validation fails, return to Phase 3 for refinement
    - Tool 5: `LLM-cozoDB-to-code-writer` applies changes with safety checks using 4-entity flow:
        - LLM generates queries to extract validated future_code from CozoDB (Entity 2)
        - Tool 5 writes code to actual files in codebase (Entity 4) with automatic backups
        - Multi-layer validation:
          - Build validation: cargo build
          - Test validation: cargo test
          - Runtime validation: integration tests
          - Performance validation: benchmarks
          - Code quality validation: clippy/rustfmt
          - CI/CD validation: pipeline compatibility

- Phase 5: State Reset & Completion
    - User confirms satisfaction with changes
    - Tool 6: `cozoDB-make-future-code-current` resets database state
    - Git commit created with list of changes
    - CodeGraph updated with current state

## 2.3 Tool Mapping to Current Architecture

**Purpose**: Reference guide mapping the complete 7-component tool pipeline to architectural entities.
**Audience**: Implementation teams and developers who need to understand tool responsibilities and data flow.
**Format**: Structured list with clear mapping between tools and the 4-entity architecture.

- **Complete Tool Pipeline (7 components)**:
    - **Orchestrator**: `agent-parseltongue-reasoning-orchestrator` (External LLM coordination & workflow management)
    - Tool 1: `folder-to-cozoDB-streamer` (Multi-language code indexing via tree-sitter)
    - Tool 2: `LLM-to-cozoDB-writer` (LLM upsert queries → CozoDB temporal updates)
    - Tool 3: `LLM-cozoDB-to-context-writer` (LLM queries → CozoDB → CodeGraphContext.json)
    - Tool 4: `rust-preflight-code-simulator` (Rust-specific enhanced validation)
    - Tool 5: `LLM-cozoDB-to-code-writer` (LLM queries → CozoDB → Code files)
    - Tool 6: `cozoDB-make-future-code-current` (State reset)

**Language Support Levels**:
- **Multi-Language Core**: Tools 1, 2, 3, 5, 6 work with any tree-sitter supported language
- **Rust-Enhanced**: Tool 4 provides Rust-specific validation and LSP integration
- **Graceful Degradation**: Non-Rust projects get core functionality without Rust-specific validation

## 2.4 Four-Entity Data Flow Architecture

**Purpose**: Technical deep-dive into the 4-entity architecture that enables systematic LLM↔CozoDB communication.
**Audience**: System architects and senior developers who need to understand the fundamental data flow patterns.
**Format**: Entity-by-entity breakdown with responsibilities, interfaces, and data flow diagrams.

The Parseltongue system enables bidirectional LLM↔CozoDB communication through a clean 4-entity architecture:

### **Entity 1: LLM (Claude Code + Orchestrator Agent)**
- Role: Natural language reasoning and change specification
- Interface: Uses `agent-parseltongue-reasoning-orchestrator.md`
- Cannot read CozoDB directly - requires intermediate entities
- Responsible for generating all queries using CozoDbQueryRef.md patterns

### **Entity 2: CozoDB (Graph Database)**
- Role: Stores CodeGraph with temporal versioning (current_ind, future_ind, Future_Action)
- Schema: ISGL1 primary key + Current_Code + Future_Code + interface_signature + TDD_Classification + lsp_meta_data
- Cannot be read directly by LLM - requires context extraction
- Passive storage that responds to LLM-generated queries

### **Entity 3: CodeGraphContext.json (Context Bridge)**
- Role: Structured context transfer between CozoDB and LLM
- Created by: `LLM-cozoDB-to-context-writer` (LLM queries → CozoDB → JSON)
- Contains: ISGL1 + interface_signature + TDD_Classification + lsp_meta_data (excludes Current_Code to prevent context bloat)
- LLM reads this directly for reasoning

### **Entity 4: Codebase (Rust Source Files)**
- Role: Actual code implementation
- Updated by: `LLM-cozoDB-to-code-writer` (LLM queries → CozoDB → Files)
- Read by: `folder-to-cozoDB-streamer` (Files → CozoDB)

### **Data Flow Patterns**
```
LLM → [LLM-to-cozoDB-writer] → CozoDB (temporal upserts)
CozoDB → [LLM-cozoDB-to-context-writer] → CodeGraphContext.json
CodeGraphContext.json → [LLM reads directly] → LLM
LLM → [LLM-cozoDB-to-code-writer] → CozoDB → Codebase
Codebase → [folder-to-cozoDB-streamer] → CozoDB
```

### **Tool Responsibilities**
- **Tool 1**: Codebase → CozoDB (indexing)
- **Tool 2**: LLM → CozoDB via `LLM-to-cozoDB-writer` (temporal upserts)
- **Tool 3**: LLM queries → CozoDB → CodeGraphContext.json (context extraction)
- **Tool 4**: Validation of proposed changes
- **Tool 5**: LLM queries → CozoDB → Code files (code writing)
- **Tool 6**: Database state reset

## 2.5 Temporal Versioning System

**Purpose**: Technical reference for the temporal versioning system that enables safe code transitions.
**Audience**: Developers who need to understand state management and versioning patterns in CozoDB.
**Format**: State-by-state breakdown with transition patterns and use cases.

- **State Tracking in CozoDB**:
    - **(1,1)**: Code exists now and continues (unchanged)
    - **(1,0)**: Code exists now but will be deleted
    - **(0,1)**: Code doesn't exist but will be created
    - **(1,1)**: Code exists and will be modified

- **Current_Code → Future_Code Flow**:
    - Phase 2: LLM sets future_code based on bug analysis
    - Phase 4: Future_code becomes actual code in files
    - Phase 5: Database reset makes future_code the new current_code

## 2.6 Command Interface (Current)

**Purpose**: Practical reference for command-line usage across different user expertise levels.
**Audience**: End users and developers who need to interact with the Parseltongue system.
**Format**: Command examples separated by user expertise level with usage patterns.

- **Primary Interface (95% of users)**:
    ```bash
    @agent-parseltongue-reasoning-orchestrator "Fix panic in GitHub #1234"
    ```

- **Manual Tools (5% of users)**:
    ```bash
    # Tool 1: Index codebase
    folder-to-cozoDB-streamer ./src --parsing-library tree-sitter --chunking ISGL1 --output-db ./parseltongue.db

    # Tool 2: LLM upserts temporal changes to CozoDB
    LLM-to-cozoDB-writer --query-temporal "INSERT INTO Code_Graph VALUES (...)" --database ./parseltongue.db

    # Tool 3: LLM extracts context from CozoDB
    LLM-cozoDB-to-context-writer --query "SELECT * FROM Code_Graph WHERE current_ind=1" --database ./parseltongue.db --output-context CodeGraphContext.json

    # Tool 4: Validate proposed changes
    rust-preflight-code-simulator validation_output.json --validation-type all

    # Tool 5: LLM extracts and writes code to files
    LLM-cozoDB-to-code-writer validation.json --database ./parseltongue.db

    # Tool 6: Reset database state
    cozoDB-make-future-code-current --project-path . --database ./parseltongue.db
    ```

    **Query Generation**: All LLM-generated queries use patterns from `CozoDbQueryRef.md` for consistency and correctness

## 2.7 Integration with Current Architecture

**Purpose**: Validation checklist showing how the minimalPRD workflow aligns with current system implementation.
**Audience**: Implementation teams and validation engineers who need to verify architectural consistency.
**Format**: Point-by-point alignment verification with current architecture components.

The minimalPRD workflow aligns with the current 7-component architecture:
- **External Orchestrator + 6-Tool Pipeline**: Claude Code agent coordinates specialized tools
- **5-Phase Process**: Matches current orchestrator workflow
- **Temporal Versioning**: Enhanced with (current_ind, future_ind) state management
- **Apple Silicon Focus**: Current platform strategy
- **Bug-Fixing Priority**: Current primary use case

## 2.8 Language Scope and Limitations

**Purpose**: Clear definition of language capabilities and system behavior across different programming languages.
**Audience**: Developers and architects who need to understand what functionality is available for their specific language stack.
**Format**: Language-by-language breakdown with capability matrix.

### Core Multi-Language Support (Tree-Sitter Based)
**Supported Languages**: All languages with tree-sitter grammars (Python, JavaScript, TypeScript, Go, C++, Java, etc.)

**Available Capabilities**:
- Interface extraction and chunking (functions, classes, methods, etc.)
- Dependency graph construction
- Temporal versioning with state tracking
- Context-aware code generation
- Basic syntax validation
- File writing with atomic backups

### Rust-Enhanced Support (LSP + Preflight Validation)
**Additional Rust-Specific Capabilities**:
- Enhanced type information via rust-analyzer LSP
- Semantic completion and error detection
- Preflight compilation validation
- Borrow checker integration
- Cargo build/test automation
- Clippy linting and rustfmt formatting
- Performance benchmarking integration

### System Behavior by Language Type

**Rust Projects**:
```
🔍 Analysis: tree-sitter parsing + rust-analyzer LSP metadata
🧪 Validation: preflight compilation + cargo test + linting
🚀 Automation: Full build/test/deployment pipeline
```

**Non-Rust Projects**:
```
🔍 Analysis: tree-sitter parsing (no LSP metadata)
🧪 Validation: Basic syntax validation (no preflight compilation)
🚀 Automation: File writing + user-managed build/test integration
```

### Graceful Degradation Strategy
1. **Automatic Language Detection**: System detects file types and applies appropriate processing
2. **Conditional LSP Integration**: rust-analyzer only activates for `.rs` files
3. **Transparent User Communication**: Clear messaging about available capabilities per language
4. **Validation Adaptation**: Skips Rust-specific validation steps for non-Rust code
5. **Build System Integration**: User responsible for non-Rust build/test execution

### Future Language Enhancement Path
- **Language-Specific LSP Integration**: Add Python (PyLance), TypeScript (TSLS), Go (gopls) support
- **Language-Specific Validation**: Add pytest, npm test, go test integration
- **Multi-Language Projects**: Cross-language dependency analysis
- **Performance Optimization**: Language-specific caching strategies

## 2.9 Success Criteria

**Purpose**: Definition of done for bug fixes with measurable validation criteria.
**Audience**: Quality assurance teams and developers who need to verify fix completeness.
**Format**: Checklist of validation requirements with specific success metrics.

A bug is considered fixed when:
1. Error no longer occurs (verified through testing)
2. Code compiles successfully
3. All tests pass
4. Performance regressions are resolved
5. Code quality checks pass
6. CI/CD pipelines complete successfully