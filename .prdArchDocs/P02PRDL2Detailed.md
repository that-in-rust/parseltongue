# 2.0 Detailed User Journey - Updated for Current Architecture

## 2.1 MVP Ultra-Minimalist Principles

**Purpose**: Define the ultra-minimalist approach that ensures reliability and simplicity for ~10 users.
**Audience**: Implementation teams who need to understand the simplicity-first philosophy.
**Format**: Clear rules that prioritize reliability over complexity.

**TARGET USERS**: ~10 people - focus on essential functionality that works reliably
**PHILOSOPHY**: Simplicity over complexity - each tool does ONE thing well and reliably

### **TOOL SIMPLICITY RULES:**

**Tool 5 (LLM-cozodb-to-diff-writer) - MINIMALIST:**
- NO backup options (MVP doesn't need them)
- NO multiple safety levels (complex to debug)
- NO configuration complexity (single reliable JSON generation)
- **SINGLE PURPOSE**: Generate CodeDiff.json from CozoDB for LLM to apply changes
- **EASY DEBUGGING**: Clear JSON output, inspectable by humans and LLMs
- **FOCUS**: Provide LLM with exactly what code changes to apply

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
Step B03: Generate CodeDiff.json, LLM applies changes with minimal verification
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

- Phase 4: Validation & Diff Generation
    - Tool 4: `rust-preflight-code-simulator` performs **simplified syntax validation**:
        - **Scope**: Tree-sitter syntax checks ONLY for entities with future_code
        - **Speed**: <20ms for typical change set (ultra-fast feedback)
        - **Does NOT validate**: Types, imports, lifetimes (cargo build handles these in Phase 4)
        - **Purpose**: Quick syntax sanity check before file writes
    - If syntax errors detected, return to Phase 3 with line/column details for refinement
    - Tool 5: `LLM-cozodb-to-diff-writer` generates diff context for LLM:
        - **SINGLE PURPOSE**: Generate CodeDiff.json from CozoDB for LLM consumption
        - Tool 5 queries CozoDB for entities with Future_Action != None
        - Generates structured CodeDiff.json with file paths, operations, and future_code
        - LLM (Entity 1) reads CodeDiff.json and applies changes to codebase (Entity 4)
        - **MINIMAL VERIFICATION**: Basic build/test validation by LLM (MVP approach)
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
    - Tool 4: `rust-preflight-code-simulator` (Simplified syntax validation for entities with future_code)
    - Tool 5: `LLM-cozodb-to-diff-writer` (CozoDB → CodeDiff.json → LLM applies changes)
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
- **Dual ISGL1 Key Format**:
  - **Existing Entities** (Tool 1): `rust:fn:calculate_sum:src_lib_rs:42-56` (line-based)
  - **New Entities** (Tool 2): `src_lib_rs-new_feature-fn-abc12345` (hash-based, SHA-256 first 8 chars)
- Cannot be read directly by LLM - requires context extraction
- Passive storage that responds to LLM-generated queries

### **Entity 3: CodeGraphContext.json (Context Bridge)**
- Role: Structured context transfer between CozoDB and LLM
- Created by: `LLM-cozoDB-to-context-writer` (LLM queries → CozoDB → JSON)
- Contains: ISGL1 + interface_signature + TDD_Classification + lsp_meta_data (excludes Current_Code to prevent context bloat)
- LLM reads this directly for reasoning

### **Entity 4: Codebase (Rust Source Files)**
- Role: Actual code implementation
- Updated by: LLM reading CodeDiff.json (generated by `LLM-cozodb-to-diff-writer`)
- Read by: `folder-to-cozoDB-streamer` (Files → CozoDB)

### **Data Flow Patterns**
```
LLM → [LLM-to-cozoDB-writer] → CozoDB (temporal upserts)
CozoDB → [LLM-cozoDB-to-context-writer] → CodeGraphContext.json
CodeGraphContext.json → [LLM reads directly] → LLM
CozoDB → [LLM-cozodb-to-diff-writer] → CodeDiff.json → [LLM reads and applies] → Codebase
Codebase → [folder-to-cozoDB-streamer] → CozoDB
```

### **Tool Responsibilities**
- **Tool 1**: Codebase → CozoDB (indexing)
  - Generates line-based ISGL1 keys for existing entities
  - Format: `{language}:{type}:{name}:{sanitized_path}:{start_line}-{end_line}`
- **Tool 2**: LLM → CozoDB via `LLM-to-cozoDB-writer` (temporal upserts)
  - Generates hash-based ISGL1 keys for new entities (Create operations)
  - Format: `{sanitized_filepath}-{entity_name}-{entity_type}-{hash8}`
- **Tool 3**: LLM queries → CozoDB → CodeGraphContext.json (context extraction)
- **Tool 4**: Validation of proposed changes
- **Tool 5**: CozoDB → CodeDiff.json via `LLM-cozodb-to-diff-writer` (diff generation, LLM applies changes)
- **Tool 6**: Database state reset

### **Hash-Based Key Generation Specification (Tool 2)**

**Purpose**: Provide stable identity for new code entities before they exist in the codebase.

**Problem Solved**: CRUD Create operations cannot use line-based keys because new entities don't have line numbers yet.

**Implementation Location**: `parseltongue-core/src/entities.rs`

**Function Signature**:
```rust
pub fn generate_new_entity_key(
    file_path: &str,
    entity_name: &str,
    entity_type: &EntityType,
    timestamp: DateTime<Utc>
) -> String
```

**Algorithm**:
1. Hash Input: Concatenate `file_path + entity_name + entity_type + timestamp`
2. Hash Function: SHA-256
3. Hash Output: Take first 8 characters of hex representation
4. Key Format: `{sanitized_filepath}-{entity_name}-{type_abbrev}-{hash8}`

**Type Abbreviations**:
- Function → `fn`
- Struct → `struct`
- Enum → `enum`
- Trait → `trait`
- Impl → `impl`
- Module → `mod`

**Examples**:
```
Input: file_path="src/lib.rs", entity_name="new_feature", entity_type=Function, timestamp=2025-10-30T12:00:00Z
Output: "src_lib_rs-new_feature-fn-abc12345"

Input: file_path="src/models/user.rs", entity_name="UserProfile", entity_type=Struct, timestamp=2025-10-30T12:01:00Z
Output: "src_models_user_rs-UserProfile-struct-def67890"
```

**Path Sanitization Rules**:
- Replace `/` with `_`
- Replace `\` with `_`
- Replace `.` with `_`

**Collision Handling**:
- Timestamp provides uniqueness across multiple entities with same name
- 8-character hash provides 4.3 billion possible values
- Collision probability: negligible for typical codebase sizes

**Integration Points**:
- `LLM-to-cozoDB-writer/src/temporal_writer.rs`: Detects Create operations (current_ind=0, future_ind=1) and calls hash generator
- `parseltongue-core/src/entities.rs`: Implements hash generation function with comprehensive tests

**Dependencies Required**:
```toml
sha2 = "0.10"
chrono = { version = "0.4", features = ["serde"] }
```

## 2.7 Temporal Versioning System

**Purpose**: Technical reference for the temporal versioning system that enables safe code transitions.
**Audience**: Developers who need to understand state management and versioning patterns in CozoDB.
**Format**: State-by-state breakdown with transition patterns and use cases.

- **State Tracking in CozoDB**:
    - **(1,1, None)**: Code exists now and continues (unchanged)
    - **(1,0, Delete)**: Code exists now but will be deleted
    - **(0,1, Create)**: Code doesn't exist but will be created
        - **KEY FORMAT**: Hash-based ISGL1 key (e.g., `src_lib_rs-new_feature-fn-abc12345`)
        - **GENERATION**: Tool 2 generates key using SHA-256 hash at creation time
        - **RATIONALE**: No line numbers available for new entities yet
    - **(1,1, Edit)**: Code exists and will be modified
        - **KEY FORMAT**: Line-based ISGL1 key from Tool 1 (e.g., `rust:fn:calculate_sum:src_lib_rs:42-56`)

- **Current_Code → Future_Code Flow**:
    - Phase 2: LLM sets future_code based on bug analysis
    - Phase 4: Tool 5 generates CodeDiff.json, LLM applies future_code to actual files
    - Phase 5: Database reset makes future_code the new current_code
        - **Note**: After reset, newly created entities are re-indexed by Tool 1 with line-based keys
        - **Key Transition**: Hash-based keys (Create) → Line-based keys (after re-indexing)

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

    # Tool 5: Generate CodeDiff.json for LLM to apply changes
    LLM-cozodb-to-diff-writer --database ./parseltongue.db --output CodeDiff.json

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

### Rust-Enhanced Support (LSP Metadata)
**Additional Rust-Specific Capabilities**:
- Enhanced type information via rust-analyzer LSP (Tool 1 indexing)
- Semantic metadata stored in lsp_meta_data column
- **Note**: Validation happens via cargo build/test AFTER file writes (Step D04-D05)
- **Rationale**: Real compilation catches all errors that matter; syntax-only validation is fast sanity check

### System Behavior by Language Type

**Rust Projects**:
```
🔍 Analysis: tree-sitter parsing + rust-analyzer LSP metadata (Tool 1)
🧪 Syntax Check: Tree-sitter validation for future_code entities (Tool 4, <20ms)
✅ Real Validation: cargo build + cargo test AFTER file writes (Step D04-D05)
🚀 Automation: LLM applies changes, then runs cargo for validation
```

**Non-Rust Projects**:
```
🔍 Analysis: tree-sitter parsing (no LSP metadata)
🧪 Syntax Check: Tree-sitter validation for future_code entities (Tool 4)
✅ Real Validation: Language-specific toolchain after file writes
🚀 Automation: File writing + user-managed build/test integration
```

### Graceful Degradation Strategy
1. **Automatic Language Detection**: System detects file types and applies appropriate processing
2. **Conditional LSP Integration**: rust-analyzer only activates for `.rs` files
3. **Transparent User Communication**: Clear messaging about available capabilities per language
4. **Validation Adaptation**: Skips Rust-specific validation steps for non-Rust code
