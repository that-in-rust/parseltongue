# 1.0 Minimal User Journey

## 1.1 Executive Summary for Parseltongue
- **User Segment**: Apple Silicon developers on large Rust codebases ONLY
- **Reliability-First Principle**:
    - Optimize for accurate 1-go fixes that feel trustworthy and increase user efficacy
    - Prefer CPU-bound static analysis (rust-analyzer overlays, ISG traversals) and small, local, free subagents
    - Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible
- **Shreyas Doshi (product framing)**: Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome
- **Jeff Dean (systems framing)**: Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, RA, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates
- **User Promise**: "When I encounter a Rust bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. Speed is a byproduct; correctness is the KPI"

## 1.2 User Journey v0.8 (Updated for Current Architecture)
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
                        - Optional: Tool will call LSP (rust-analyzer) for metadata about code-chunk-raw
                        - Tool will output aggregated-primarykey + code-chunk-raw + tree-sitter-signature + TDD_classification + lsp-meta-data (optional)
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
                    - The reasoning-LLM (default LLM via ANTHROPIC_KEY) analyzes the micro-PRD in context of ISGL1 + interface_signature + TDD_Classification + lsp_meta_data because cozo-to-context-writer places them in a json; we will ignore the Current_Code because it would unnecessarily bloat the context
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
                - Tool 2: `cozo-to-context-writer --query "temporal reasoning query" --database ./parseltongue.db` is triggered
                    - Use TDD_idiomatic_rust_steering_doc for all cozo-to-context-writer operations while reasoning through code
                    - Tool 2 creates a base-context-area which is micro-PRD + filter(Code_Graph with current_ind=1)=>(ISGL1 + interface_signature + TDD_Classification + lsp_meta_data)
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
                        - Step C: Tool 3: `rust-preflight-code-simulator validation_output.json --validation-type all` triggered for Rust use cases with rust-analyzer overlay:
                            - If the rust-preflight-code-simulator tool fails then we go back to previous steps A01 onwards
                            - If the rust-preflight-code-simulator tool passes then we move to next step
                        - Step D: Run Tool 4: `cozoDB-to-code-writer validation.json --database ./parseltongue.db`:
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
                - If yes, trigger Tool 5: `cozoDB-make-future-code-current --project-path . --database ./parseltongue.db`:
                    - `cozoDB-make-future-code-current` creates a git commit with list of changes
                    - `cozoDB-make-future-code-current` resets the CodeGraph and updates all rows in CozoDB database, making future_code the new current_code

# 2.0 Detailed User Journey - Updated for Current Architecture

## 2.1 Executive Summary
- **User Segment**: Apple Silicon developers fixing bugs in large Rust codebases
- **Primary Use Case**: Bug fixing and issue resolution with precise problem definitions
- **User Promise**: "When I encounter a Rust bug, I provide the issue details and receive a validated fix that compiles and passes tests."

## 2.2 User Journey v1.0
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
    - Agent analyzes bug against CodeGraph context
    - Agent refines requirements through 2 iterations
    - Final micro-PRD isolated for processing

- Phase 3: Temporal Code Simulation
    - Tool 2: `cozo-to-context-writer` with temporal versioning
        - **Step A01**: Create test interface changes (current_ind=0, future_ind=1)
        - **Step A02**: Propagate changes to non-test interfaces
        - **Step B01**: Generate future code using hopping/blast-radius analysis
        - **Step B02**: Rubber duck debugging and confidence validation

- Phase 4: Validation & Testing
    - Tool 3: `rust-preflight-code-simulator` validates proposed changes
    - If validation fails, return to Phase 3 for refinement
    - Tool 4: `cozoDB-to-code-writer` applies changes with safety checks
        - Build validation: cargo build
        - Test validation: cargo test
        - Runtime validation: integration tests
        - Performance validation: benchmarks
        - Code quality validation: clippy/rustfmt
        - CI/CD validation: pipeline compatibility

- Phase 5: State Reset & Completion
    - User confirms satisfaction with changes
    - Tool 5: `cozoDB-make-future-code-current` resets database state
    - Git commit created with list of changes
    - CodeGraph updated with current state

## 2.3 Tool Mapping to Current Architecture
- **Complete Tool Pipeline (6 components)**:
    - **Orchestrator**: `agent-parseltongue-reasoning-orchestrator` (External LLM coordination & workflow management)
    - Tool 1: `folder-to-cozoDB-streamer` (Code indexing)
    - Tool 2: `cozo-to-context-writer` (Temporal reasoning & context extraction)
    - Tool 3: `rust-preflight-code-simulator` (Validation)
    - Tool 4: `cozoDB-to-code-writer` (File writing)
    - Tool 5: `cozoDB-make-future-code-current` (State reset)

## 2.4 Temporal Versioning System
- **State Tracking in CozoDB**:
    - **(1,1)**: Code exists now and continues (unchanged)
    - **(1,0)**: Code exists now but will be deleted
    - **(0,1)**: Code doesn't exist but will be created
    - **(1,1)**: Code exists and will be modified

- **Current_Code → Future_Code Flow**:
    - Phase 2: LLM sets future_code based on bug analysis
    - Phase 4: Future_code becomes actual code in files
    - Phase 5: Database reset makes future_code the new current_code

## 2.5 Command Interface (Current)
- **Primary Interface (95% of users)**:
    ```bash
    @agent-parseltongue-reasoning-orchestrator "Fix panic in GitHub #1234"
    ```

- **Manual Tools (5% of users)**:
    ```bash
    folder-to-cozoDB-streamer ./src --parsing-library tree-sitter --chunking ISGL1 --output-db ./parseltongue.db
    cozo-to-context-writer --query "context extraction query" --database ./parseltongue.db
    rust-preflight-code-simulator validation_output.json --validation-type all
    cozoDB-to-code-writer validation.json --database ./parseltongue.db
    cozoDB-make-future-code-current --project-path . --database ./parseltongue.db
    ```

## 2.6 Integration with Current Architecture
The minimalPRD workflow aligns with the current 6-component architecture:
- **External Orchestrator + 5-Tool Pipeline**: Claude Code agent coordinates specialized tools
- **5-Phase Process**: Matches current orchestrator workflow
- **Temporal Versioning**: Enhanced with (current_ind, future_ind) state management
- **Apple Silicon Focus**: Current platform strategy
- **Bug-Fixing Priority**: Current primary use case

## 2.7 Success Criteria
A bug is considered fixed when:
1. Error no longer occurs (verified through testing)
2. Code compiles successfully
3. All tests pass
4. Performance regressions are resolved
5. Code quality checks pass
6. CI/CD pipelines complete successfully