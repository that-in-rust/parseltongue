# Parseltongue: Product Requirements Document v2.0

**Document Type**: Workflow-Ordered CLI Toolkit Specification
**Last Updated**: 2025-11-01
**Status**: Architecture Redesign - Commands First
**Philosophy**: Commands are the guiding light - everything else supports them

**Implementation Reference**: See `that-in-rust-parseltongue-8a5edab282632443 (8).txt` for detailed implementation context

---

## THE GUIDING LIGHT: COMMAND REFERENCE

**These 6 commands define the entire Parseltongue workflow.**

### **pt01: Ingest Codebase → Database**

```bash
parseltongue pt01-folder-to-cozodb-streamer <directory> \
  --db rocksdb:parseltongue.db \
  [--verbose] \
  [--quiet]
```

**What it does:**
- Parses code files with tree-sitter
- Generates ISGL1 keys: `{lang}:{type}:{name}:{path}:{lines}`
- Stores entities in CodeGraph table
- Sets initial state: `(current_ind=1, future_ind=1, future_action=None)`

**Example:**
```bash
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db
```

---

### **pt02: Read Database → JSON (for LLM)**

```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --db rocksdb:parseltongue.db \
  --output ./contexts/context.json \
  [--include-current-code <0|1>]  # Default: 0
  [--max-context-tokens 128000] \
  [--verbose]
```

**What it does:**
- Reads entities from CodeGraph with `current_ind=1`
- **Excludes `Current_Code` by default** (token optimization)
- Generates JSON for LLM consumption
- Enforces token limit

**Example (Default - No Code):**
```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --db rocksdb:analysis.db \
  --output context.json \
  --include-current-code 0
```

**Example (With Code - Rare):**
```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --db rocksdb:analysis.db \
  --output context.json \
  --include-current-code 1
```

---

### **pt03: Edit Database (LLM Writes Changes)**

```bash
# Simple Interface (80% of use cases)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "<ISGL1_KEY>" \
  --action <create|edit|delete> \
  --future-code "<CODE>" \
  --db rocksdb:parseltongue.db

# Advanced Interface (Power Users)
parseltongue pt03-llm-to-cozodb-writer \
  --query "<DATALOG_QUERY>" \
  --db rocksdb:parseltongue.db
```

**What it does:**
- Updates temporal state in CodeGraph
- Sets `future_code`, `future_ind`, `future_action`
- Supports CREATE/EDIT/DELETE operations
- Advanced mode: Execute raw Datalog

**Example 1: EDIT**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() { println!(\"Fixed!\"); }" \
  --db rocksdb:analysis.db
```

**Example 2: CREATE**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_lib_rs-new_function-fn-abc12345" \
  --action create \
  --future-code "pub fn new_function(x: i32) -> i32 { x * 2 }" \
  --db rocksdb:analysis.db
```

**Example 3: DELETE**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:old_func:lib_rs:20-25" \
  --action delete \
  --db rocksdb:analysis.db
```

---

### **pt04: Validate Syntax (Pre-Flight Check)**

```bash
parseltongue pt04-syntax-preflight-validator \
  --db rocksdb:parseltongue.db \
  [--verbose]
```

**What it does:**
- Reads all entities with `future_action != None`
- Validates `future_code` syntax using tree-sitter
- Reports errors (file, line, issue)
- Exit code 0 = valid, non-zero = errors

**Example:**
```bash
parseltongue pt04-syntax-preflight-validator --db rocksdb:analysis.db
```

---

### **pt05: Generate Diff → Code (JSON Output)**

```bash
parseltongue pt05-llm-cozodb-to-diff-writer \
  --db rocksdb:parseltongue.db \
  --output CodeDiff.json \
  [--verbose]
```

**What it does:**
- Reads all entities with `future_action != None`
- Generates CodeDiff.json with operation-specific fields
- Parses line ranges from ISGL1 keys
- Desanitizes file paths

---

### **pt06: Reset Database (Make Future → Current)**

```bash
parseltongue pt06-cozodb-make-future-code-current \
  --project <directory> \
  --db rocksdb:parseltongue.db
```

**What it does:**
- Deletes ALL entities from CodeGraph (NO backups - S01 principle)
- Recreates schema
- Re-runs pt01 to re-index current state
- Resets temporal indicators

---

## WORKFLOW: THE 6-STEP PIPELINE

```
┌─────────────────────────────────────────────────────────────────┐
│ Step 1: Ingest Codebase                                         │
│ $ parseltongue pt01-folder-to-cozodb-streamer ./src --db ...   │
│ → Creates: 1,247 entities with state (1,1,None)                 │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2: Read → JSON (for LLM)                                   │
│ $ parseltongue pt02-llm-cozodb-to-context-writer \             │
│     --output context.json --include-current-code 0              │
│ → Generates: context.json (37.5k tokens, no code)               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
         (LLM analyzes context.json, decides on changes)
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 3: Edit Database (LLM writes changes)                      │
│ $ parseltongue pt03-llm-to-cozodb-writer \                     │
│     --entity "rust:fn:hello:lib_rs:4-6" \                       │
│     --action edit \                                              │
│     --future-code "pub fn hello() { println!(\"Fixed!\"); }"    │
│ → Updates: 1 entity to state (1,1,Edit)                         │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 4: Validate Syntax                                         │
│ $ parseltongue pt04-syntax-preflight-validator --db ...        │
│ → Checks: 1 entity with future_code                            │
│ → Result: ✓ All syntax valid (exit code 0)                     │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 5: Generate Diff                                           │
│ $ parseltongue pt05-llm-cozodb-to-diff-writer \                │
│     --output CodeDiff.json                                      │
│ → Generates: CodeDiff.json with 1 EDIT operation               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
        (Orchestrator applies CodeDiff.json to files)
        (Orchestrator runs: cargo build && cargo test)
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 6: Reset State                                             │
│ $ parseltongue pt06-cozodb-make-future-code-current \          │
│     --project ./src --db ...                                    │
│ → Deletes: 1,247 entities                                      │
│ → Re-indexes: 1,247 entities with fresh state (1,1,None)       │
└─────────────────────────────────────────────────────────────────┘
```

---

## ARCHITECTURE CHANGES FROM V1

### **1. Tool Numbering: Workflow Order**

**Old (PRDv1 - Alphabetical):**
```
folder-to-cozodb-streamer           (Tool 1)
llm-to-cozodb-writer                (Tool 2)
llm-cozodb-to-context-writer        (Tool 3)
rust-preflight-code-simulator       (Tool 4)
llm-cozodb-to-diff-writer           (Tool 5)
cozodb-make-future-code-current     (Tool 6)
```

**New (PRDv2 - Workflow Order):**
```
pt01-folder-to-cozodb-streamer      (Ingest)
pt02-llm-cozodb-to-context-writer   (Read)
pt03-llm-to-cozodb-writer           (Edit)
pt04-syntax-preflight-validator     (Validate)
pt05-llm-cozodb-to-diff-writer      (Diff)
pt06-cozodb-make-future-code-current (Reset)
```

**Benefits:**
- ✅ Sequential numbering matches execution order
- ✅ `pt01` → `pt02` → `pt03` is self-documenting
- ✅ LLMs can reason about pipeline flow
- ✅ Consistent `pt##-` prefix

---

### **2. Tool 2 ↔ Tool 3 Swap (Workflow Logic)**

**Rationale:** Reading happens BEFORE editing in the workflow.

**Old:**
- Tool 2: `llm-to-cozodb-writer` (editing)
- Tool 3: `llm-cozodb-to-context-writer` (reading)

**New:**
- Tool 2: `pt02-llm-cozodb-to-context-writer` (reading)
- Tool 3: `pt03-llm-to-cozodb-writer` (editing)

---

### **3. pt04 Renamed: Remove "Rust" Prefix**

**Old:** `rust-preflight-code-simulator`
**New:** `pt04-syntax-preflight-validator`

**Rationale:**
- Tree-sitter is multi-language by design
- Current implementation is Rust-only, but architecture supports Python/JS/TS/Go
- "Syntax" is more accurate than "Code Simulator" (no execution, just parsing)

---

### **4. pt03 Restored: Simple + Advanced Interface**

**Problem Identified:**
- Forcing users to write 13-field Datalog queries is absurd
- S01 ultra-minimalism means "simplest thing that works", not "rawest interface"

**Solution: Progressive Disclosure**

**Simple Mode (80% of use cases):**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() {}" \
  --db rocksdb:analysis.db
```
*Tool builds 13-field Datalog query internally.*

**Advanced Mode (20% of use cases):**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --query "?[ISGL1_key, ...] := [...] :put CodeGraph {...}" \
  --db rocksdb:analysis.db
```

**Validation:** Mutual exclusion - EITHER `--entity/--action` OR `--query`

---

### **5. pt02 Enhanced: Token Optimization Flag**

**New Flag:** `--include-current-code <0|1>` (default: 0)

**Rationale:**
- Default export: Exclude `Current_Code` and `Future_Code` (saves ~500k tokens)
- Rare cases: Include code for deep analysis

**Impact:**
- **Default**: 37.5k tokens (1,500 entities, signatures only)
- **With code**: 537.5k tokens (1,500 entities × ~350 tokens/entity)

---

## QUERY PATTERNS: THE COMPLETE REFERENCE

### **Pattern A: CREATE - New Entity (Hash-Based ISGL1)**

**Simple Interface:**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_lib_rs-new_function-fn-abc12345" \
  --action create \
  --future-code "pub fn new_function(x: i32) -> i32 { x * 2 }" \
  --db demo.db
```

**Temporal State:** `(current_ind=0, future_ind=1, future_action='Create')`

**ISGL1 Key Format:** `{sanitized_path}-{name}-{type}-{hash8}`

---

### **Pattern B: EDIT - Modify Existing Entity**

**Simple Interface:**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() { println!(\"Fixed!\"); }" \
  --db demo.db
```

**Temporal State:** `(current_ind=1, future_ind=1, future_action='Edit')`

**ISGL1 Key Format:** `{lang}:{type}:{name}:{sanitized_path}:{start}-{end}`

---

### **Pattern C: DELETE - Remove Existing Entity**

**Simple Interface:**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:old_function:lib_rs:20-25" \
  --action delete \
  --db demo.db
```

**Temporal State:** `(current_ind=1, future_ind=0, future_action='Delete')`

---

### **Pattern D: READ - Export to JSON**

**Default (No Code):**
```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --output context.json \
  --db demo.db \
  --include-current-code 0
```

**With Code (Rare):**
```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --output context.json \
  --db demo.db \
  --include-current-code 1
```

---

## MULTI-LANGUAGE SUPPORT ROADMAP

### **Current Status: Rust-Only**

**Implemented:**
- ✅ Rust: tree-sitter-rust grammar installed
- ✅ Rust: Entity extraction (functions, structs, traits, enums)
- ✅ Rust: Syntax validation via pt04

**Defined but Not Implemented:**
- ❌ Python, JavaScript, TypeScript, Go, Java, C++, etc.

### **Architecture: Multi-Language Ready** ✅

**Evidence:**
1. Language enum supports 13 languages
2. ISGL1 keys include language prefix: `python:fn:process:utils_py:42-50`
3. Database schema has `language` field
4. Parser registry supports `HashMap<Language, Parser>`

### **What's Missing: Grammar Dependencies + Extraction Logic**

**Per-Language Requirements:**
1. Add tree-sitter grammar (Cargo.toml)
2. Implement entity extraction (AST walking)
3. Update pt04 validator (language-aware)

**Estimate:** 3-4 days per language

---

## BREAKING CHANGES FROM PRDv1

### **1. Crate Renames**

| Old Name | New Name |
|----------|----------|
| `folder-to-cozodb-streamer` | `pt01-folder-to-cozodb-streamer` |
| `llm-cozodb-to-context-writer` | `pt02-llm-cozodb-to-context-writer` |
| `llm-to-cozodb-writer` | `pt03-llm-to-cozodb-writer` |
| `rust-preflight-code-simulator` | `pt04-syntax-preflight-validator` |
| `llm-cozodb-to-diff-writer` | `pt05-llm-cozodb-to-diff-writer` |
| `cozodb-make-future-code-current` | `pt06-cozodb-make-future-code-current` |

### **2. Tool Numbering Swap**

| PRDv1 | PRDv2 | Workflow Position |
|-------|-------|-------------------|
| Tool 2: Write | Tool 3: Write | Position 3 (Edit) |
| Tool 3: Read | Tool 2: Read | Position 2 (Read) |

### **3. New CLI Arguments**

**pt02 (Read):**
- `--include-current-code <0|1>` (NEW, default: 0)

**pt03 (Write):**
- `--entity`, `--action`, `--future-code` (RESTORED)
- `--query` (KEPT for power users)

---

## IMPLEMENTATION PLAN

> **Implementation Note**: Refer to `that-in-rust-parseltongue-8a5edab282632443 (8).txt` for detailed implementation context, patterns, and code examples from previous refactoring work.

### **Phase 1: Rename Crates**
1. Rename all 6 crate directories
2. Update `Cargo.toml` workspace members
3. Update unified binary routing
4. Run full test suite

### **Phase 2: Restore pt03 Simple Interface**
1. Add CLI arguments: `--entity`, `--action`, `--future-code`
2. Implement mutual exclusion with `--query`
3. Build 13-field Datalog internally for CREATE/EDIT/DELETE
4. Write comprehensive tests (simple + advanced modes)

### **Phase 3: Add --include-current-code to pt02**
1. Add CLI flag (default: 0)
2. Modify query builder to exclude `Current_Code` and `Future_Code` when flag is 0
3. Update JSON serialization
4. Test token count differences

### **Phase 4: Update All Documentation**
1. README.md - Update all command examples
2. Parseltongue-SOP.md - Add query patterns
3. CLAUDE.md - Update crate names and references
4. demo-walkthrough/ - Regenerate logs with new commands
5. refCommandsQueries.md - Update CLI syntax

### **Phase 5: Cleanup Unnecessary CLI**
- Remove redundant arguments across all tools
- Simplify option parsing
- Consolidate common patterns
- Ensure consistency across pt01-pt06

### **Phase 6: Multi-Language (Future)**
- Python support (proof-of-concept)
- JavaScript/TypeScript
- Language pack plugin system

---

## SUMMARY

**PRDv2 represents workflow-first thinking:**
- Commands define the architecture
- Tool numbers follow execution order (not alphabetical)
- Simple interfaces for common cases, advanced for power users
- Multi-language architecture ready, Rust implemented
- Progressive disclosure pattern (simple → advanced)

**Core Principle:** The 6 commands are the guiding light. Everything else exists to support them.

**Implementation Reference:** See `that-in-rust-parseltongue-8a5edab282632443 (8).txt` for context

**End of PRDv2.0**
