# 1.0 Minimal User Journey

## 1.0.1 MVP PRINCIPLES: MINIMALISM & RELIABILITY FIRST

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

### **CONTEXT OPTIMIZATION FOR MVP:**

**PRINCIPLE**: MINIMIZE LLM CONTEXT BLOAT - THIS IS EXTREMELY IMPORTANT

**READING STRATEGY:**
1. **EXCLUDE** current_code by default (major bloat source)
2. **ALLOW** reading future_code ONLY for rows being changed
3. **ALLOW** reading current_code ONLY for rows being changed (when absolutely needed)
4. **FILTER**: Only load rows where `Future_Action != None` for detailed analysis

**CONTEXT BLOAT PREVENTION:**
- Default: Load only interface signatures + metadata (no code content)
- Exception: Load future_code for rows that will actually change
- Exception: Load current_code for changing rows (only when absolutely necessary)
- Result: Dramatically reduced context while maintaining all necessary information

**IMPLEMENTATION PATTERN:**
```
Step B01: filter(Future_Action != None) => (minimal data + future_code + current_code_if_needed)
Step B02: Rubber duck debugging with optimized context
Step B03: Write changes with minimal verification
```

This ensures LLM has exactly what it needs - no more, no less.

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
                    - The reasoning-LLM (default LLM via ANTHROPIC_KEY) analyzes the micro-PRD using CodeGraphContext.json which contains ISGL1 + interface_signature + TDD_Classification + lsp_meta_data; we will **STRICTLY** exclude the Current_Code because it would catastrophically bloat the context
                        - **CONTEXT BLOAT WARNING**: Including current_code would exponentially increase context size (potentially 500k+ tokens for 1500 interfaces)
                        - **TOKEN CALCULATION** (Optimized without current_code):
                            - Avg interface size is 1000 to 1500 nodes
                            - 1500 nodes x 3 tokens for ISGL1 = 4500 tokens
                            - 1500 nodes x 7 tokens for interface_signature = 10500 tokens
                            - 1500 nodes x 1 tokens for TDD_Classification = 1500 tokens
                            - 1500 nodes x 15 tokens for lsp_meta_data = 22500 tokens
                        - **SAFE TOTAL**: 37.5k tokens + micro-PRD (5k tokens) + iterations (15k tokens) = ~57.5k tokens
                        - **DANGER ZONE**: Including current_code could push context to 500k+ tokens, causing failures
                        - **CONTEXT LIMIT**: Must stay under 100k tokens for reliable LLM operation
                    - The reasoning-LLM will analyze then suggest changes to the micro-PRD to make it clearer in terms of what changes the user wants:
                        - Tests wise
                        - Behavior wise
                        - Functionality wise
                    - After 2 iterations the reasoning-LLM will accept the micro-PRD
                    - Ask the reasoning LLM to reset the context because likely it will overflow and micro-PRD final needs to be isolated
                - Tool 3: `LLM-cozoDB-to-context-writer --query "Select * EXCEPT (current_code,future_code) from Code_Graph where current_ind=1" --database ./parseltongue.db --output-context CodeGraphContext.json` is triggered
                    - **CRITICAL CONTEXT OPTIMIZATION**: We EXCLUDE current_code and future_code from ALL context extraction
                    - **NEVER access current_code directly** - This will bloat context exponentially (current_code can be thousands of lines)
                    - **Current_Code ACCESS RULE**: Only access current_code when absolutely necessary for specific line-level analysis, and even then, extract only the minimal required lines
                    - **CONTEXT BLOAT PREVENTION**: current_code column is treated as "write-only" during reasoning phase
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
                        - Step B: Code Simulation (Context-Optimized for MVP)
                            - Step B01: Based on filter(Future_Action != None)=>(minimal data + future_code + current_code_if_needed) + base-context-area, update future_code for all rows that are changing:
                                - **CONTEXT OPTIMIZATION**: Only load current_code/future_code for rows that will actually change
                                - **DEFAULT**: Load only interface signatures + metadata (minimal bloat)
                                - **EXCEPTION**: Load future_code for changing rows (less bloat than current_code)
                                - **EXCEPTION**: Load current_code only when absolutely necessary for changing rows
                                - **RESULT**: Drastically reduced context while maintaining necessary information
                                - The reasoning-LLM can use hopping or blast-radius actions on Code_Graph to fetch additional information, but only for changing rows to prevent bloat
                                - **CONTEXT RULE**: Non-changing rows should never load current_code or future_code into context
                                    - Hopping or blast-radius actions can be CLI options but preferably since our LLM is smart enough they need not be, and we can define them precisely in our supporting MD files
                            - Step B02: Follow rubber duck debugging to re-reason filter(Future_Action != None)=>(all fields of Code_Graph including current code) + base-context-area:
                                - **ITERATIVE REASONING CYCLE**: This step embodies the core read-edit-read-edit mindset:
                                - **READ**: LLM analyzes current state from context and temporal changes
                                - **EDIT**: LLM updates CozoDB with improved temporal changes via Tool 2
                                - **READ**: LLM extracts updated context via Tool 3 to verify changes
                                - **REPEAT**: Continue read-edit-read-edit cycle until confident
                                - **Confidence Threshold**: Stop when LLM confidence ≥ 80% and solution is coherent
                                - **Iteration Count**: May repeat A01→A02→B01→B02 cycle multiple times until satisfactory
                                - If the LLM thinks that we need to refine the solutioning further, repeat Steps A01 A02 and then basis them repeat Steps B01
                                - If the LLM doesn't feel confident of the changes, it should speak to the user to get additional context or web help, sharing their current understanding in an MD file
                                - **ITERATION COMPLETION**: When LLM feels confident of the changes (≥80% confidence), move to next step
                        - Step C: Tool 4: `rust-preflight-code-simulator validation_output.json --validation-type all` triggered for Rust use cases with rust-analyzer overlay:
                            - If the rust-preflight-code-simulator tool fails then we go back to previous steps A01 onwards
                            - If the rust-preflight-code-simulator tool passes then we move to next step
                        - Step D: Run Tool 5: `LLM-cozoDB-to-code-writer validation.json --database ./parseltongue.db`:
                            - **MVP SIMPLIFIED**: Write changes from CozoDB to code files (single reliable operation)
                            - Step D01: Write the validated future_code to actual files (atomic operations)
                            - Step D02: Run cargo build to verify compilation
                            - Step D03: Run cargo test to verify functionality
                            - **MINIMAL VERIFICATION**: Basic build/test validation (MVP approach)
                            - If validation fails, go back to previous steps A01 onwards with specific error details
                            - If all validations pass, we move to next step
            - Ask user if they are satisfied with how the code is working:
                - If yes, trigger Tool 6: `cozoDB-make-future-code-current --project-path . --database ./parseltongue.db`:
                    - **ULTRA-MINIMAL**: Delete CodeGraph table + re-trigger folder-to-cozoDB-streamer (that's it!)
                    - **CLEANEST POSSIBLE**: No temporal state management, just fresh rebuild
                    - **MAXIMUM RELIABILITY**: Simplest operation = fewest failure points


