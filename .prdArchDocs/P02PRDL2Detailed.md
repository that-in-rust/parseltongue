# 2.0 Detailed User Journey - Updated for Current Architecture

## 2.1 MVP Ultra-Minimalist Principles

**Purpose**: Define the ultra-minimalist approach that ensures reliability and simplicity for ~10 users.
**Audience**: Implementation teams who need to understand the simplicity-first philosophy.
**Format**: Clear rules that prioritize reliability over complexity.

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

## 2.2 Context Optimization Rules

**Purpose**: Define the context bloat prevention strategy that keeps LLM context under 100k tokens.
**Audience**: System architects and developers who need to understand LLM context management.
**Format**: Clear rules for reading strategies and bloat prevention.

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

**TOKEN CALCULATION (Optimized without current_code):**
- Avg interface size is 1000 to 1500 nodes
- 1500 nodes x 3 tokens for ISGL1 = 4500 tokens
- 1500 nodes x 7 tokens for interface_signature = 10500 tokens
- 1500 nodes x 1 tokens for TDD_Classification = 1500 tokens
- 1500 nodes x 15 tokens for lsp_meta_data = 22500 tokens
- **SAFE TOTAL**: 37.5k tokens + micro-PRD (5k tokens) + iterations (15k tokens) = ~57.5k tokens
- **DANGER ZONE**: Including current_code could push context to 500k+ tokens, causing failures
- **CONTEXT LIMIT**: Must stay under 100k tokens for reliable LLM operation

**IMPLEMENTATION PATTERN:**
```
Step B01: filter(Future_Action != None) => (minimal data + future_code + current_code_if_needed)
Step B02: Rubber duck debugging with optimized context
Step B03: Write changes with minimal verification
```

This ensures LLM has exactly what it needs - no more, no less.

## 2.3 Executive Summary
- **User Segment**: Apple Silicon developers fixing bugs in multi-language codebases with Rust-first enhanced support
- **Primary Use Case**: Bug fixing and issue resolution with precise problem definitions across tree-sitter supported languages
- **Language Capabilities**:
  - **Full Support (Rust)**: Enhanced LSP integration, preflight validation, full build/test automation
  - **Basic Support (Other Languages)**: Tree-sitter parsing, interface extraction, temporal versioning, basic syntax validation
- **User Promise**: "When I encounter a code bug, I provide the issue details and receive a validated fix. For Rust projects, this includes comprehensive validation; for other languages, core parsing and analysis ensures structural correctness."

## 2.4 User Journey v0.9 (Updated for Current Architecture)

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

- Phase 4: Validation & Testing
    - Tool 4: `rust-preflight-code-simulator` validates proposed changes
    - If validation fails, return to Phase 3 for refinement
    - Tool 5: `LLM-cozoDB-to-code-writer` applies changes using ultra-minimalist approach:
        - **SINGLE PURPOSE**: Write from CozoDB to files reliably (no backup options)
        - LLM generates queries to extract validated future_code from CozoDB (Entity 2)
        - Tool 5 writes code to actual files in codebase (Entity 4) with single reliable operation
        - **MINIMAL VERIFICATION**: Basic build/test validation (MVP approach)
          - Build validation: cargo build
          - Test validation: cargo test

- Phase 5: State Reset & Completion
    - User confirms satisfaction with changes
    - Tool 6: `cozoDB-make-future-code-current` resets database state using ultra-minimalist approach:
        - **ULTRA-MINIMAL**: Delete CodeGraph table + re-trigger folder-to-cozoDB-streamer (that's it!)
        - **CLEANEST POSSIBLE**: No temporal state management, just fresh rebuild
        - **MAXIMUM RELIABILITY**: Simplest operation = fewest failure points
    - Git commit created with list of changes
    - CodeGraph updated with current state

## 2.5 Tool Mapping to Current Architecture

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

## 2.6 Four-Entity Data Flow Architecture

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

## 2.7 Temporal Versioning System

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

## 2.8 Command Interface (Current)

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

## 2.9 Integration with Current Architecture

**Purpose**: Validation checklist showing how the minimalPRD workflow aligns with current system implementation.
**Audience**: Implementation teams and validation engineers who need to verify architectural consistency.
**Format**: Point-by-point alignment verification with current architecture components.

The minimalPRD workflow aligns with the current 7-component architecture:
- **External Orchestrator + 6-Tool Pipeline**: Claude Code agent coordinates specialized tools
- **5-Phase Process**: Matches current orchestrator workflow
- **Temporal Versioning**: Enhanced with (current_ind, future_ind) state management
- **Apple Silicon Focus**: Current platform strategy
- **Bug-Fixing Priority**: Current primary use case

## 2.10 Language Scope and Limitations

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
