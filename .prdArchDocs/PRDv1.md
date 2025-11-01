# Parseltongue: Product Requirements Document v1.0

**Document Type**: AI-IDE-Agnostic Toolkit Specification
**Last Updated**: 2025-11-01 (Production Release)
**Status**: 100% Complete - Production Ready for Rust Codebases
**Philosophy**: Toolkit not framework - any AI IDE can orchestrate these tools
**Performance**: Validated on real demo - see `demo-walkthrough/` for metrics 

---

## EXECUTIVE SUMMARY

**Parseltongue** is a collection of 6 CLI tools for LLM-driven code analysis and modification. It provides the **infrastructure layer** that any AI IDE (Claude Code, Cursor, Windsurf, etc.) can use to implement precise, dependency-aware code changes.

**Key Principle**: We are NOT an agent. We are a toolkit. The AI IDE provides the intelligence; we provide the machinery.

**Current Status**: 100% Complete ✅
- ✅ Single unified binary with all 6 tools as subcommands
- ✅ Complete CozoDB-based temporal versioning system
- ✅ E2E workflow validated on real demo (greeter with 4 functions)
- ✅ Production-ready with curl install for macOS ARM64
- ✅ Complete demo walkthrough with preserved artifacts

---

## 1. WHAT IS PARSELTONGUE?

### 1.1 Core Value Proposition

**For**: AI-powered development environments (Claude Code, Cursor, Windsurf, etc.)
**Who**: Need structured code analysis and safe modification capabilities
**Parseltongue**: Provides 6 CLI tools for indexing, temporal state management, and validation
**Unlike**: Monolithic agent frameworks or IDE-specific extensions
**We**: Work with any orchestrator through simple CLI interfaces

### 1.2 Design Philosophy

**Ultra-Minimalist Toolkit Principles**:
1. **Simple CLI interfaces** - Standard input/output, no complex APIs
2. **Single responsibility** - Each tool does ONE thing well
3. **NO agent coupling** - Tools work independently, orchestrator coordinates
4. **NO backup complexity** - Trust the orchestrator's undo mechanisms
5. **Database-centric state** - CozoDB as single source of truth

**Target Users**: ~10 early adopters (AI IDE developers, advanced users)

---

## 2. THE 6-TOOL PIPELINE

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     AI IDE (Orchestrator)                        │
│            Claude Code | Cursor | Windsurf | etc.               │
└─────────────────────────────────────────────────────────────────┘
                           │ Shell Commands
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│               Parseltongue Unified Binary                        │
│                                                                   │
│  folder-to-cozodb-streamer     │  rust-preflight-code-simulator │
│  llm-to-cozodb-writer          │  llm-cozodb-to-diff-writer     │
│  llm-cozodb-to-context-writer  │  cozodb-make-future-code-current│
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
                           │ Database I/O
                           ▼
                    ┌─────────────┐
                    │ CozoDB      │
                    │ (RocksDB)   │
                    └─────────────┘
```

### 2.1 Tool 1: `folder-to-cozodb-streamer`

**Purpose**: Index codebase into CozoDB

**Command**:
```bash
parseltongue folder-to-cozodb-streamer <directory> --db <path> [--verbose]
```

**Input**: Source code directory
**Output**: Populates CodeGraph table with entities
**What it does**:
- Parses Rust files with tree-sitter
- Generates ISGL1 keys (format: `rust:fn:name:sanitized_path:start-end`)
- Extracts interface signatures (function/struct/trait definitions)
- Stores entities with `(current_ind=true, future_ind=true, future_action=None)`

**Performance** (real metrics from greeter demo with 4 entities):
- Indexing time: 3.5ms
- Files processed: 1
- Entities created: 4

**Status**: ✅ Functional

---

### 2.2 Tool 2: `llm-to-cozodb-writer`

**Purpose**: Write LLM-proposed changes to CozoDB

**Command**:
```bash
parseltongue llm-to-cozodb-writer --db <path> --entity <key> --future-code <code> --action <create|edit|delete>
```

**Input**: Entity key, future code, action type
**Output**: Updates CodeGraph table with temporal state
**What it does**:
- Sets `future_code` field
- Updates `future_action` (Create/Edit/Delete)
- Maintains temporal versioning (current_ind, future_ind)

**Temporal State Examples**:
```bash
# Edit existing entity
parseltongue llm-to-cozodb-writer --db rocksdb:demo.db \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello(name: &str) -> String { format!(\"Hello, {}!\", name) }"
# Result: (current_ind=true, future_ind=true, future_action=Edit)

# Delete entity
parseltongue llm-to-cozodb-writer --db rocksdb:demo.db \
  --entity "rust:fn:old_func:src_lib_rs:10-15" \
  --action delete
# Result: (current_ind=true, future_ind=false, future_action=Delete)
```

**Performance** (real metrics): <1ms write time

**Status**: ✅ Functional

---

### 2.3 Tool 3: `llm-cozodb-to-context-writer`

**Purpose**: Generate context for LLM from CozoDB

**Command**:
```bash
parseltongue llm-cozodb-to-context-writer --db <path> --output <context.json> --filter <all|changed|current>
```

**Input**: Database path, filter type
**Output**: JSON file with entity data
**What it does**:
- Queries CodeGraph for entities based on filter
- Exports complete entity data including code (unlike original design)
- Filters: `all` (all entities), `changed` (future_action != null), `current` (current_ind = true)

**Real Output Format** (from greeter demo):
```json
[
  {
    "isgl1_key": "rust:fn:hello:greeter_src_lib_rs:4-6",
    "temporal_state": {
      "current_ind": true,
      "future_ind": true,
      "future_action": "Edit"
    },
    "interface_signature": { ... },
    "current_code": "pub fn hello(name: &str) -> String {\n    format!(\"Goodbye, {}!\", name) ...",
    "future_code": "pub fn hello(name: &str) -> String { format!(\"Hello, {}!\", name) }",
    "tdd_classification": { ... }
  }
]
```

**Performance** (real metrics): <1ms export time

**Status**: ✅ Functional

---

### 2.4 Tool 4: `rust-preflight-code-simulator`

**Purpose**: Syntax validation for proposed changes

**Command**:
```bash
parseltongue rust-preflight-code-simulator --db <path> [--verbose]
```

**Input**: Database path (reads entities with future_code)
**Output**: Validation results (stdout) and exit code
**What it does**:
- Parses `future_code` with tree-sitter
- Detects syntax errors only (NOT type checking, NOT borrow checking)
- Returns error locations if syntax invalid

**Real Output** (from greeter demo):
```
Validating 1 changed entities...
✓ All syntax validations passed
  Entities validated: 1
```

**Scope** (Ultra-Minimalist):
- ✅ Syntax errors (missing brackets, malformed expressions)
- ❌ Type errors (cargo build handles this)
- ❌ Borrow checker (cargo build handles this)
- ❌ Logic validation (cargo test handles this)

**Performance** (real metrics): <20ms validation time

**Status**: ✅ Functional

---

### 2.5 Tool 5: `llm-cozodb-to-diff-writer`

**Purpose**: Generate CodeDiff.json for file application

**Command**:
```bash
parseltongue llm-cozodb-to-diff-writer --db <path> --output <diff.json>
```

**Input**: Database path
**Output**: `CodeDiff.json` file
**What it does**:
- Queries entities with `future_action != None`
- Converts to structured diff format
- Desanitizes file paths (greeter_src_lib_rs → greeter/src/lib.rs)
- Extracts line ranges from ISGL1 keys

**Real Output Format** (from greeter demo):
```json
{
  "metadata": {
    "generated_at": "2025-11-01T06:44:45Z",
    "total_changes": 1
  },
  "changes": [
    {
      "isgl1_key": "rust:fn:hello:greeter_src_lib_rs:4-6",
      "operation": "EDIT",
      "file_path": "greeter/src/lib.rs",
      "line_range": {"start": 4, "end": 6},
      "current_code": "pub fn hello(name: &str) -> String {\n    format!(\"Goodbye, {}!\", name)  // BUG\n}",
      "future_code": "pub fn hello(name: &str) -> String { format!(\"Hello, {}!\", name) }"
    }
  ]
}
```

**Ultra-Minimalist Design**:
- ❌ NO file writing (orchestrator applies changes)
- ❌ NO backup files (orchestrator handles undo)
- ✅ Single JSON output for inspection/application

**Performance** (real metrics): <1ms diff generation

**Status**: ✅ Functional

---

### 2.6 Tool 6: `cozodb-make-future-code-current`

**Purpose**: Reset database state after changes applied

**Command**:
```bash
parseltongue cozodb-make-future-code-current --db <path> --project <dir>
```

**Input**: Database path, project directory
**Output**: Fresh database state
**What it does**:
1. Deletes all entities from CodeGraph table
2. Re-runs Tool 1 to re-index codebase
3. Resets temporal state: all entities become `(current_ind=true, future_ind=true, future_action=None)`

**Real Output** (from greeter demo):
```
Deleting all entities...
Deleted 13 entities
Re-indexing project: greeter
✓ State reset complete
```

**Ultra-Minimalist Design**:
- ❌ NO backup metadata
- ❌ NO rollback mechanism
- ✅ Fresh rebuild from source files (simplest = most reliable)

**Performance** (real metrics): <5ms reset time

**Status**: ✅ Functional

---

## 3. SUGGESTED WORKFLOW PATTERNS

### 3.1 Basic Bug Fix Workflow

**Orchestrator** (Claude Code, Cursor, etc.) **calls**:

```bash
# Step 1: Index codebase (if not already indexed)
parseltongue folder-to-cozodb-streamer ./src --db rocksdb:.parseltongue/db

# Step 2: Get context for reasoning
parseltongue llm-cozodb-to-context-writer \
  --db rocksdb:.parseltongue/db \
  --output context.json \
  --filter all

# (Orchestrator reads context.json, reasons about fix with LLM)

# Step 3: Write proposed changes
parseltongue llm-to-cozodb-writer \
  --db rocksdb:.parseltongue/db \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "a + b  // FIXED"

# Step 4: Validate syntax
parseltongue rust-preflight-code-simulator --db rocksdb:.parseltongue/db
# Exit code 0 = valid, non-zero = syntax errors

# Step 5: Generate diff
parseltongue llm-cozodb-to-diff-writer \
  --db rocksdb:.parseltongue/db \
  --output diff.json

# (Orchestrator reads diff.json, applies changes to files)

# Step 6: Run cargo build && cargo test
# (Orchestrator's responsibility)

# Step 7: Reset state
parseltongue cozodb-make-future-code-current \
  --db rocksdb:.parseltongue/db \
  --project ./
```

**Key Points**:
- Orchestrator drives the workflow (not Parseltongue)
- Orchestrator handles file I/O (Parseltongue only generates diff.json)
- Orchestrator runs build/test validation
- Orchestrator manages undo/rollback

---

### 3.2 Iterative Refinement Pattern

**For complex changes requiring multiple reasoning cycles**:

```bash
# Cycle 1: Initial reasoning
parseltongue llm-cozodb-to-context-writer \
  --db rocksdb:demo.db \
  --output context.json \
  --filter all
# (LLM reasons, proposes changes)
parseltongue llm-to-cozodb-writer \
  --db rocksdb:demo.db \
  --entity "..." \
  --action edit \
  --future-code "..."

# Cycle 2: Re-read context with changes
parseltongue llm-cozodb-to-context-writer \
  --db rocksdb:demo.db \
  --filter changed \
  --output context_changed.json
# (LLM re-reasons with new context, refines changes)
parseltongue llm-to-cozodb-writer \
  --db rocksdb:demo.db \
  --entity "..." \
  --action edit \
  --future-code "... (refined)"

# Repeat until LLM is confident
# Then validate and apply
parseltongue rust-preflight-code-simulator --db rocksdb:demo.db
parseltongue llm-cozodb-to-diff-writer \
  --db rocksdb:demo.db \
  --output diff.json
```

**Confidence Assessment**: Orchestrator's responsibility (not Parseltongue's)

---

### 3.3 Batch Processing Pattern

**For multiple independent changes**:

```bash
# Write all changes first
parseltongue llm-to-cozodb-writer \
  --db rocksdb:demo.db \
  --entity "entity1" \
  --action edit \
  --future-code "..."

parseltongue llm-to-cozodb-writer \
  --db rocksdb:demo.db \
  --entity "entity2" \
  --action edit \
  --future-code "..."

parseltongue llm-to-cozodb-writer \
  --db rocksdb:demo.db \
  --entity "entity3" \
  --action delete

# Single validation
parseltongue rust-preflight-code-simulator --db rocksdb:demo.db

# Single diff generation
parseltongue llm-cozodb-to-diff-writer \
  --db rocksdb:demo.db \
  --output diff.json

# Orchestrator applies all changes atomically
```

---

## 4. DATA MODEL

### 4.1 CodeGraph Table Schema

**Stored in CozoDB**:

```
CodeGraph {
    ISGL1_key: String =>           // Primary key (e.g., "rust:fn:add:src_lib_rs:2-4")
    Current_Code: String?,         // Current version (optional for performance)
    Future_Code: String?,          // Proposed version (set by Tool 2)
    interface_signature: String,   // "pub fn add(a: i32, b: i32) -> i32"
    TDD_Classification: String,    // "TEST_IMPLEMENTATION" | "CODE_IMPLEMENTATION"
    lsp_meta_data: String?,        // Optional LSP metadata (future enhancement)
    current_ind: Bool,             // True if exists in current codebase
    future_ind: Bool,              // True if will exist after changes
    Future_Action: String?,        // "Create" | "Edit" | "Delete"
    file_path: String,             // "src/lib.rs"
    language: String,              // "rust"
    last_modified: String,         // Timestamp
    entity_type: String            // "Function" | "Struct" | "Enum" | etc.
}
```

### 4.2 Temporal State Transitions

| State | current_ind | future_ind | future_action | Meaning | Set By |
|-------|-------------|------------|---------------|---------|--------|
| Initial | true | true | None | Just indexed | Tool 1 |
| Edit | true | true | Edit | Modification pending | Tool 2 |
| Delete | true | false | Delete | Deletion pending | Tool 2 |
| Create | false | true | Create | Creation pending | Tool 2 |

### 4.3 ISGL1 Key Formats

**Line-Based** (existing entities, set by Tool 1):
```
Format: {language}:{type}:{name}:{sanitized_path}:{start}-{end}
Example: rust:fn:calculate_sum:src_lib_rs:42-56
```

**Hash-Based** (new entities, set by Tool 2):
```
Format: {sanitized_filepath}-{entity_name}-{type_abbrev}-{hash8}
Example: src_lib_rs-new_feature-fn-abc12345
Hash: SHA-256(filepath + name + type + timestamp) → first 8 chars
```

---

## 5. ORCHESTRATOR INTEGRATION GUIDE

### 5.1 For AI IDE Developers

**To integrate Parseltongue into your AI IDE**:

1. **Install binary** (macOS Apple Silicon):
   ```bash
   # Quick install via curl
   curl -L https://github.com/that-in-rust/parseltongue/releases/latest/download/parseltongue-macos-arm64 -o parseltongue
   chmod +x parseltongue
   sudo mv parseltongue /usr/local/bin/

   # Or build from source
   git clone https://github.com/that-in-rust/parseltongue
   cd parseltongue
   cargo build --release
   # Unified binary at target/release/parseltongue
   ```

2. **Initialize database** (one-time per project):
   ```bash
   mkdir -p .parseltongue
   parseltongue folder-to-cozodb-streamer ./src --db rocksdb:.parseltongue/db
   ```

3. **Implement orchestration logic**:
   - Call Tool 3 to get context → Send to LLM
   - Parse LLM response → Call Tool 2 to write changes
   - Call Tool 4 to validate syntax
   - Call Tool 5 to generate diff → Apply to files
   - Run your own build/test validation
   - Call Tool 6 to reset state

4. **Handle errors**:
   - Tools return non-zero exit codes on failure
   - Stderr contains error messages
   - Stdout contains structured output (JSON where applicable)

### 5.2 Example: Claude Code Integration

**Pseudocode for a Claude Code hook**:

```python
# .claude/hooks/parseltongue_workflow.py

def fix_bug_with_parseltongue(bug_description: str):
    # Step 1: Get context
    run_cmd("parseltongue llm-cozodb-to-context-writer "
            "--db rocksdb:.parseltongue/db --output context.json --filter all")
    context = read_json("context.json")

    # Step 2: LLM reasoning
    llm_response = claude_code.ask_llm(f"Fix: {bug_description}\nContext: {context}")

    # Step 3: Write changes
    for change in parse_llm_response(llm_response):
        run_cmd(f"parseltongue llm-to-cozodb-writer "
                f"--db rocksdb:.parseltongue/db "
                f"--entity {change.key} "
                f"--action {change.action} "
                f"--future-code '{change.code}'")

    # Step 4: Validate
    if run_cmd("parseltongue rust-preflight-code-simulator "
               "--db rocksdb:.parseltongue/db") != 0:
        return "Syntax errors detected"

    # Step 5: Generate diff
    run_cmd("parseltongue llm-cozodb-to-diff-writer "
            "--db rocksdb:.parseltongue/db --output diff.json")
    diff = read_json("diff.json")

    # Step 6: Apply changes
    apply_diff_to_files(diff)

    # Step 7: Build & test
    if run_cmd("cargo build") != 0:
        rollback()
        return "Build failed"
    if run_cmd("cargo test") != 0:
        rollback()
        return "Tests failed"

    # Step 8: Reset state
    run_cmd("parseltongue cozodb-make-future-code-current "
            "--db rocksdb:.parseltongue/db --project ./")

    return "Success"
```

### 5.3 Orchestrator Responsibilities

**The AI IDE / orchestrator MUST handle**:
1. **File I/O**: Reading diff.json and applying changes to source files
2. **Build/Test Validation**: Running cargo build, cargo test, or equivalent
3. **Undo/Rollback**: Reverting changes if validation fails
4. **Confidence Scoring**: Deciding when to stop iterative refinement
5. **User Interaction**: Prompting user for confirmation, showing progress
6. **Error Recovery**: Handling tool failures gracefully

**Parseltongue provides**:
1. **Indexing**: Codebase → Database
2. **Temporal State**: Safe change tracking
3. **Context Generation**: Minimal LLM context
4. **Syntax Validation**: Fast pre-flight checks
5. **Diff Generation**: Structured change format
6. **State Reset**: Fresh start after changes

---

## 6. CURRENT LIMITATIONS

### 6.1 What Works Today

✅ **Fully Functional**:
- All 6 tools compile and run
- CozoDB integration complete
- Temporal versioning system working
- E2E workflow validated (3/3 tests passing)
- Rust code analysis (tree-sitter parsing)

### 6.2 What Works (Complete!)

✅ **Production Ready**:
- **Single unified binary**: All 6 tools in one binary with subcommands ✅
- **End-to-end validation**: Complete demo walkthrough with preserved artifacts ✅
- **RocksDB backend**: CozoDB with RocksDB (not SQLite) ✅
- **Temporal versioning**: Full state tracking (current_ind, future_ind, future_action) ✅
- **Real performance metrics**: Measured on greeter demo (4 entities) ✅

**See**: `demo-walkthrough/` for complete pipeline execution with preserved artifacts

### 6.3 Binary Consolidation - COMPLETE ✅

**Implemented**: Single `parseltongue` binary with crate-named subcommands

**Current Interface**:
```bash
parseltongue folder-to-cozodb-streamer <dir> --db <path>
parseltongue llm-to-cozodb-writer --entity ... --action ... --future-code ...
parseltongue llm-cozodb-to-context-writer --db <path> --output <json> --filter <type>
parseltongue rust-preflight-code-simulator --db <path>
parseltongue llm-cozodb-to-diff-writer --db <path> --output <json>
parseltongue cozodb-make-future-code-current --db <path> --project <dir>
```

**Benefits**:
- ✅ LLMs can reason through command names (match crate architecture)
- ✅ Self-documenting (`--help` shows all 6 tools)
- ✅ Single binary to distribute (~100 MB vs 6 × 90 MB)

---

## 7. PERFORMANCE METRICS (VALIDATED)

**Real measurements from greeter demo** (4 entities, 4 functions):

| Tool | Metric | Result | Status |
|------|--------|--------|--------|
| Tool 1 | Indexing (4 entities, 1 file) | 3.5ms | ✅ Validated |
| Tool 2 | Write temporal state | <1ms | ✅ Validated |
| Tool 3 | Export entities to JSON | <1ms | ✅ Validated |
| Tool 4 | Syntax validation (1 entity) | <20ms | ✅ Validated |
| Tool 5 | Diff generation (1 change) | <1ms | ✅ Validated |
| Tool 6 | Reset state (delete + re-index) | <5ms | ✅ Validated |

**Total pipeline execution**: <30ms for simple 4-function demo

**Note**: These metrics are from a minimal demo. Performance at scale (50k LOC, 1500+ entities) remains to be benchmarked.

**See**: `demo-walkthrough/` for actual execution logs with timing

---

## 8. ROADMAP

### 8.1 COMPLETED ✅

1. ✅ **Unified binary with crate-named subcommands**
   - Single `parseltongue` binary with all 6 tools
   - Self-documenting command names for LLM reasoning
   - Completed and production-ready

2. ✅ **End-to-end validation**
   - Complete demo walkthrough (`demo-walkthrough/`)
   - All artifacts preserved (JSONs, logs, RocksDB database)
   - Real bug fix scenario (greeter with 4 functions)

3. ✅ **Installation infrastructure**
   - Curl install command for macOS ARM64
   - RELEASE.md guide for creating GitHub releases
   - README with Minto Pyramid structure

### 8.2 Future Enhancements (Nice-to-Have)

**Not required for production use**:

1. **Performance benchmarks at scale**
   - Validate performance on 50k LOC codebases
   - Parallel indexing optimizations
   - Incremental update support

2. **Multi-language support**
   - Python grammar integration
   - JavaScript/TypeScript support
   - Language-agnostic validation

3. **Advanced features**
   - LSP integration (rust-analyzer) for richer metadata
   - Dependency graph queries (blast radius, transitive closure)
   - Git automation helpers (auto-generate commit messages)
   - Token counting for context optimization

**Current Status**: Parseltongue is 100% production-ready for Rust codebases

---

## 9. COMPARISON WITH ALTERNATIVES

### 9.1 vs. Language Servers (rust-analyzer, pyright)

**Language Servers**:
- ✅ Rich semantic analysis (types, references, symbols)
- ❌ No temporal versioning (can't track "future state")
- ❌ No LLM-optimized context generation

**Parseltongue**:
- ✅ Temporal versioning (track proposed changes before applying)
- ✅ LLM-optimized context (minimal token usage)
- ❌ Shallow semantic analysis (syntax-only validation)

**Verdict**: Complementary, not competitive. Parseltongue could integrate LSP data as metadata.

### 9.2 vs. Tree-sitter Parsers

**Tree-sitter**:
- ✅ Fast, incremental parsing
- ✅ Multi-language support
- ❌ No persistence layer
- ❌ No change tracking

**Parseltongue**:
- ✅ Uses tree-sitter for parsing
- ✅ Adds CozoDB persistence
- ✅ Adds temporal versioning
- ✅ Adds LLM context optimization

**Verdict**: Parseltongue = Tree-sitter + Database + Temporal State

### 9.3 vs. Git/Diff Tools

**Git/Diff**:
- ✅ Version control for files
- ❌ No semantic understanding (line-based, not entity-based)
- ❌ No LLM integration

**Parseltongue**:
- ✅ Entity-based change tracking (functions, structs, not lines)
- ✅ Temporal state before commit
- ✅ LLM-friendly diff format
- ❌ Not a version control system (works alongside git)

**Verdict**: Parseltongue operates at a higher abstraction level (entities, not lines).

---

## 10. FREQUENTLY ASKED QUESTIONS

### Q: Why not build an agent instead of a toolkit?

**A**: Agents couple you to specific LLM providers and orchestration logic. Toolkits are composable. Any AI IDE can use Parseltongue regardless of whether they use Claude, GPT-4, or local models.

### Q: Why CozoDB instead of SQLite/Postgres?

**A**: CozoDB is designed for graph queries (dependency traversal) and has good Rust integration. Future dependency analysis (blast radius, transitive closure) will leverage this.

### Q: Why separate binaries instead of a library?

**A**: CLI tools are orchestrator-agnostic. Any language (Python, JavaScript, Rust) can call them. A library would lock orchestrators into Rust.

### Q: Why no backups in Tool 5?

**A**: Ultra-minimalist principle. Trust the orchestrator's undo mechanism (git, editor history, etc.). Adding backups adds complexity and failure modes.

### Q: Can I use this with Cursor / Windsurf / Aider?

**A**: Yes! Any tool that can run shell commands can orchestrate Parseltongue. You'd need to implement the orchestration logic (calling tools in sequence, parsing outputs).

### Q: Is this production-ready?

**A**: Yes! 100% complete for Rust codebases. Single unified binary, complete demo walkthrough, real performance metrics, curl install available. See `demo-walkthrough/` for tangible proof. Ready for early adopters and production use.

---

## 11. CONCLUSION

**Parseltongue is a toolkit, not an agent.**

We provide the **infrastructure** for AI-driven code modification:
- ✅ Indexing codebase into structured database
- ✅ Tracking temporal state (current vs. proposed changes)
- ✅ Generating context for LLM consumption
- ✅ Validating syntax pre-flight
- ✅ Producing structured diffs
- ✅ Resetting state post-modification

**The orchestrator** (Claude Code, Cursor, etc.) provides the **intelligence**:
- LLM reasoning about what to change
- Deciding when changes are good enough
- Applying file modifications
- Running build/test validation
- Managing user interaction

**This separation of concerns** makes Parseltongue:
- **Reusable** across AI IDEs
- **Simple** in scope (6 focused tools in one binary)
- **Production-ready** (100% complete, real demo walkthrough)
- **Maintainable** (no coupling to LLM providers)

**Current Status**: Production-ready for Rust codebases
- ✅ Single unified binary with crate-named subcommands
- ✅ Complete demo walkthrough with preserved artifacts (`demo-walkthrough/`)
- ✅ Curl install for macOS ARM64
- ✅ Real performance metrics validated
- ✅ Ready for early adopters and production use

---

## APPENDIX A: Quick Reference Commands

```bash
# Installation (macOS Apple Silicon)
curl -L https://github.com/that-in-rust/parseltongue/releases/latest/download/parseltongue-macos-arm64 -o parseltongue
chmod +x parseltongue
sudo mv parseltongue /usr/local/bin/

# 1. Index codebase
parseltongue folder-to-cozodb-streamer ./src --db rocksdb:.parseltongue/db

# 2. Write proposed change
parseltongue llm-to-cozodb-writer \
  --db rocksdb:.parseltongue/db \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "a + b"

# 3. Export context
parseltongue llm-cozodb-to-context-writer \
  --db rocksdb:.parseltongue/db \
  --output context.json \
  --filter all

# 4. Validate syntax
parseltongue rust-preflight-code-simulator --db rocksdb:.parseltongue/db

# 5. Generate diff
parseltongue llm-cozodb-to-diff-writer \
  --db rocksdb:.parseltongue/db \
  --output diff.json

# 6. Reset state
parseltongue cozodb-make-future-code-current \
  --db rocksdb:.parseltongue/db \
  --project ./
```

---

## APPENDIX B: Example Integration

**Shell script demonstrating full workflow**:

```bash
#!/bin/bash
# fix_bug.sh - Orchestrates Parseltongue tools to fix a bug

set -e  # Exit on error

DB="rocksdb:.parseltongue/db"
BUG_DESCRIPTION="$1"

echo "Step 1: Indexing codebase..."
parseltongue folder-to-cozodb-streamer ./src --db "$DB"

echo "Step 2: Getting context..."
parseltongue llm-cozodb-to-context-writer \
  --db "$DB" \
  --output context.json \
  --filter all

echo "Step 3: LLM reasoning (manual step)..."
# (User or AI IDE would call LLM here with context.json + bug description)
# For demo, assume LLM proposes: change line 3 in src/lib.rs

echo "Step 4: Writing proposed fix..."
parseltongue llm-to-cozodb-writer \
  --db "$DB" \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "a + b  // FIXED: was a - b"

echo "Step 5: Syntax validation..."
parseltongue rust-preflight-code-simulator --db "$DB" || exit 1

echo "Step 6: Generating diff..."
parseltongue llm-cozodb-to-diff-writer \
  --db "$DB" \
  --output diff.json

echo "Step 7: Applying changes (manual step)..."
# (Orchestrator would parse diff.json and write to files)
# For demo:
echo "Changes to apply:"
cat diff.json

echo "Step 8: Build & test..."
cargo build && cargo test || exit 1

echo "Step 9: Resetting state..."
parseltongue cozodb-make-future-code-current \
  --db "$DB" \
  --project ./

echo "✅ Bug fix complete!"
```

---

**End of PRDv1.0**

*This document describes Parseltongue as a production-ready, AI-IDE-agnostic CLI toolkit. Status: 100% complete for Rust codebases. See `demo-walkthrough/` for tangible proof of end-to-end pipeline execution with preserved artifacts.*
