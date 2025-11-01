# Parseltongue: Product Requirements Document v1.0

**Document Type**: AI-IDE-Agnostic Toolkit Specification
**Last Updated**: 2025-11-01
**Target**: Production-ready CLI toolkit for code analysis and modification
**Philosophy**: Toolkit not framework - any AI IDE can orchestrate these tools
**Performance**: Under 2 minutes to index & sub-second response on queries, not a concern for MVP 

---

## EXECUTIVE SUMMARY

**Parseltongue** is a collection of 6 CLI tools for LLM-driven code analysis and modification. It provides the **infrastructure layer** that any AI IDE (Claude Code, Cursor, Windsurf, etc.) can use to implement precise, dependency-aware code changes.

**Key Principle**: We are NOT an agent. We are a toolkit. The AI IDE provides the intelligence; we provide the machinery.

**Current Status**: 87.5% Complete
- ✅ 6/6 CLI tools functional (106/108 tests passing)
- ✅ Complete CozoDB-based temporal versioning system
- ✅ E2E workflow validated
- ❌ Single unified binary (currently 6 separate binaries)

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
│                  Parseltongue CLI Toolkit                        │
│  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐   │
│  │Tool 1│  │Tool 2│  │Tool 3│  │Tool 4│  │Tool 5│  │Tool 6│   │
│  │Index │  │Write │  │Read  │  │Check │  │Diff  │  │Reset │   │
│  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘   │
└─────────────────────────────────────────────────────────────────┘
                           │ Database I/O
                           ▼
                    ┌─────────────┐
                    │   CozoDB    │
                    │ (State DB)  │
                    └─────────────┘
```

### 2.1 Tool 1: `folder-to-cozodb-streamer`

**Purpose**: Index codebase into CozoDB

**Command**:
```bash
folder-to-cozodb-streamer <directory> --db <path> [--verbose]
```

**Input**: Source code directory
**Output**: Populates CodeGraph table with entities
**What it does**:
- Parses Rust files with tree-sitter
- Generates ISGL1 keys (line-based format: `rust:fn:name:path:start-end`)
- Extracts interface signatures (function/struct/trait definitions)
- Stores entities with `(current_ind=true, future_ind=true, future_action=None)`

**Status**: ✅ Functional (18/18 tests passing)
**Binary**: `folder-to-cozodb-streamer` (92 MB)

---

### 2.2 Tool 2: `llm-to-cozodb-writer`

**Purpose**: Write LLM-proposed changes to CozoDB

**Command**:
```bash
llm-to-cozodb-writer --db <path> --entity <key> --future-code <code> --action <create|edit|delete>
```

**Input**: Entity key, future code, action type
**Output**: Updates CodeGraph table with temporal state
**What it does**:
- Sets `future_code` field
- Updates `future_action` (Create/Edit/Delete)
- Maintains temporal versioning (current_ind, future_ind)

**Temporal State Examples**:
```
# Edit existing entity
--entity "rust:fn:add:src_lib_rs:2-4" --action edit --future-code "a + b"
Result: (current_ind=true, future_ind=true, future_action=Edit)

# Delete entity
--entity "rust:fn:old_func:src_lib_rs:10-15" --action delete
Result: (current_ind=true, future_ind=false, future_action=Delete)

# Create new entity (hash-based key)
--entity "src_lib_rs-new_feature-fn-abc12345" --action create --future-code "..."
Result: (current_ind=false, future_ind=true, future_action=Create)
```

**Status**: ✅ Functional (12/12 tests passing)
**Binary**: `llm-to-cozodb-writer` (40 MB)

---

### 2.3 Tool 3: `llm-cozodb-to-context-writer`

**Purpose**: Generate minimal context for LLM from CozoDB

**Command**:
```bash
llm-cozodb-to-context-writer --db <path> --output <context.json> [--filter changed]
```

**Input**: Database path, optional filter
**Output**: `CodeGraphContext.json` file
**What it does**:
- Queries CodeGraph for entities (optionally filtered by `future_action != None`)
- Generates minimal representation (EXCLUDES current_code/future_code by default)
- Outputs JSON with: isgl1_key, interface_signature, tdd_classification, lsp_metadata

**Output Format**:
```json
{
  "entities": [
    {
      "isgl1_key": "rust:fn:add:src_lib_rs:2-4",
      "interface_signature": "pub fn add(a: i32, b: i32) -> i32",
      "tdd_classification": "CODE_IMPLEMENTATION",
      "lsp_metadata": null
    }
  ],
  "entity_count": 1,
  "token_count": 37500,
  "generated_at": "2025-11-01T10:30:00Z"
}
```

**Context Optimization**: Excludes code content → ~37.5k tokens for 1500 entities (vs 500k+ if code included)

**Status**: ✅ Functional (19/19 tests passing)
**Binary**: `llm-cozodb-to-context-writer` (93 MB)

---

### 2.4 Tool 4: `rust-preflight-code-simulator`

**Purpose**: Syntax validation for proposed changes

**Command**:
```bash
rust-preflight-code-simulator --db <path> [--validation-type syntax]
```

**Input**: Database path (reads entities with future_code)
**Output**: Validation results (JSON or exit code)
**What it does**:
- Parses `future_code` with tree-sitter
- Detects syntax errors only (NOT type checking, NOT borrow checking)
- Returns error locations if syntax invalid

**Scope** (Ultra-Minimalist):
- ✅ Syntax errors (missing brackets, malformed expressions)
- ❌ Type errors (cargo build handles this)
- ❌ Borrow checker (cargo build handles this)
- ❌ Logic validation (cargo test handles this)

**Rationale**: Keep validation fast (<20ms). Let the orchestrator run cargo build/test after file writes.

**Status**: ✅ Functional (15/15 tests passing)
**Binary**: `rust-preflight-code-simulator` (91 MB)

---

### 2.5 Tool 5: `llm-cozodb-to-diff-writer`

**Purpose**: Generate CodeDiff.json for file application

**Command**:
```bash
llm-cozodb-to-diff-writer --db <path> --output <diff.json>
```

**Input**: Database path
**Output**: `CodeDiff.json` file
**What it does**:
- Queries entities with `future_action != None`
- Converts to structured diff format
- Desanitizes file paths (src_lib_rs → src/lib.rs)
- Extracts line ranges from ISGL1 keys

**Output Format**:
```json
{
  "entities": [
    {
      "isgl1_key": "rust:fn:add:src_lib_rs:2-4",
      "operation": "Edit",
      "file_path": "src/lib.rs",
      "line_range": {"start": 2, "end": 4},
      "current_code": "a - b  // BUG",
      "future_code": "a + b  // FIXED"
    }
  ]
}
```

**Ultra-Minimalist Design**:
- ❌ NO file writing (orchestrator applies changes)
- ❌ NO backup files (orchestrator handles undo)
- ✅ Single JSON output for inspection/application

**Status**: ✅ Functional (13/13 tests passing)
**Binary**: `llm-cozodb-to-diff-writer` (90 MB)

---

### 2.6 Tool 6: `cozodb-make-future-code-current`

**Purpose**: Reset database state after changes applied

**Command**:
```bash
cozodb-make-future-code-current --db <path> --project <dir>
```

**Input**: Database path, project directory
**Output**: Fresh database state
**What it does**:
1. Deletes all entities from CodeGraph table
2. Re-runs Tool 1 to re-index codebase
3. Resets temporal state: all entities become `(current_ind=true, future_ind=true, future_action=None)`

**Ultra-Minimalist Design**:
- ❌ NO backup metadata
- ❌ NO rollback mechanism
- ✅ Fresh rebuild from source files (simplest = most reliable)

**Status**: ✅ Functional (6/6 tests passing)
**Binary**: `cozodb-make-future-code-current` (93 MB)

---

## 3. SUGGESTED WORKFLOW PATTERNS

### 3.1 Basic Bug Fix Workflow

**Orchestrator** (Claude Code, Cursor, etc.) **calls**:

```bash
# Step 1: Index codebase (if not already indexed)
folder-to-cozodb-streamer ./src --db .parseltongue/db.cozo

# Step 2: Get context for reasoning
llm-cozodb-to-context-writer --db .parseltongue/db.cozo --output context.json

# (Orchestrator reads context.json, reasons about fix with LLM)

# Step 3: Write proposed changes
llm-to-cozodb-writer --db .parseltongue/db.cozo \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "a + b  // FIXED"

# Step 4: Validate syntax
rust-preflight-code-simulator --db .parseltongue/db.cozo
# Exit code 0 = valid, non-zero = syntax errors

# Step 5: Generate diff
llm-cozodb-to-diff-writer --db .parseltongue/db.cozo --output diff.json

# (Orchestrator reads diff.json, applies changes to files)

# Step 6: Run cargo build && cargo test
# (Orchestrator's responsibility)

# Step 7: Reset state
cozodb-make-future-code-current --db .parseltongue/db.cozo --project ./
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
llm-cozodb-to-context-writer --db db.cozo --output context.json
# (LLM reasons, proposes changes)
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "..."

# Cycle 2: Re-read context with changes
llm-cozodb-to-context-writer --db db.cozo --filter changed --output context_changed.json
# (LLM re-reasons with new context, refines changes)
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "... (refined)"

# Repeat until LLM is confident
# Then validate and apply
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff.json
```

**Confidence Assessment**: Orchestrator's responsibility (not Parseltongue's)

---

### 3.3 Batch Processing Pattern

**For multiple independent changes**:

```bash
# Write all changes first
llm-to-cozodb-writer --db db.cozo --entity "entity1" --action edit --future-code "..."
llm-to-cozodb-writer --db db.cozo --entity "entity2" --action edit --future-code "..."
llm-to-cozodb-writer --db db.cozo --entity "entity3" --action delete

# Single validation
rust-preflight-code-simulator --db db.cozo

# Single diff generation
llm-cozodb-to-diff-writer --db db.cozo --output diff.json

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

1. **Install binaries** (or build from source):
   ```bash
   git clone https://github.com/that-in-rust/parseltongue
   cd parseltongue
   cargo build --release --workspace
   # Binaries in target/release/
   ```

2. **Initialize database** (one-time per project):
   ```bash
   mkdir -p .parseltongue
   folder-to-cozodb-streamer ./src --db .parseltongue/db.cozo
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
    run_cmd("llm-cozodb-to-context-writer --db .parseltongue/db.cozo --output context.json")
    context = read_json("context.json")

    # Step 2: LLM reasoning
    llm_response = claude_code.ask_llm(f"Fix: {bug_description}\nContext: {context}")

    # Step 3: Write changes
    for change in parse_llm_response(llm_response):
        run_cmd(f"llm-to-cozodb-writer --db .parseltongue/db.cozo "
                f"--entity {change.key} --action {change.action} --future-code '{change.code}'")

    # Step 4: Validate
    if run_cmd("rust-preflight-code-simulator --db .parseltongue/db.cozo") != 0:
        return "Syntax errors detected"

    # Step 5: Generate diff
    run_cmd("llm-cozodb-to-diff-writer --db .parseltongue/db.cozo --output diff.json")
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
    run_cmd("cozodb-make-future-code-current --db .parseltongue/db.cozo --project ./")

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

### 6.2 What Doesn't Work Yet

❌ **Not Implemented**:
- **Single unified binary**: Currently 6 separate binaries (could be combined with subcommands)
- **Performance benchmarks**: Claims (8ms blast radius, <30s indexing) are unvalidated
- **Multi-language support**: Only Rust parsing configured (Python/JS/TS not implemented)
- **LSP metadata**: Stubbed (no actual rust-analyzer integration)
- **Token counting**: Structure exists, actual counting unimplemented
- **Dependency extraction**: Partial implementation (DependencyEdges table exists, extraction incomplete)

❌ **Known Issues**:
- 2 failing tests in parseltongue-core (temporal state validation edge cases)
- LLM API integration untested with real OpenAI API (all tests use mocks)

### 6.3 Binary Consolidation Opportunity

**Current**: 6 separate binaries (~90 MB each)
**Desired**: 1 binary with subcommands

**Proposed Interface**:
```bash
parseltongue index ./src --db db.cozo           # Tool 1
parseltongue write --entity ... --action ...    # Tool 2
parseltongue read --db db.cozo --output ctx.json # Tool 3
parseltongue check --db db.cozo                 # Tool 4
parseltongue diff --db db.cozo --output diff.json # Tool 5
parseltongue reset --db db.cozo --project ./    # Tool 6
```

**Implementation**: Create a main binary that dispatches to tool modules based on subcommand.

**Estimated Effort**: ~4 hours (refactor main.rs files, create dispatcher)

---

## 7. PERFORMANCE TARGETS (ASPIRATIONAL)

**WARNING**: The following performance targets are UNVALIDATED. No benchmarks exist.

| Metric | Target | Status |
|--------|--------|--------|
| Tool 1 indexing | <30s for 50k LOC | ❌ Untested at scale |
| Tool 3 context gen | <500ms | ❌ No benchmark |
| Tool 3 token count | <100k tokens | 🟡 Calculated ~37.5k (unverified) |
| Tool 4 validation | <20ms for 50 entities | ❌ No benchmark |
| Tool 5 diff gen | <100ms | ❌ No benchmark |

**Recommendation**: Either implement benchmarks or remove specific performance numbers.

---

## 8. ROADMAP

### 8.1 Immediate (P0 - This Week)

**Effort**: ~4 hours

1. Create unified binary with subcommands (4h)
2. Fix 2 failing core tests (included in above)

**Deliverable**: Single `parseltongue` binary that replaces 6 separate binaries

### 8.2 High Priority (P1 - Next Week)

**Effort**: ~15 hours

1. Implement performance benchmarks (8h)
   - Validate or remove performance claims
2. Add real LLM API integration tests (4h)
3. Documentation updates (3h)
   - CLI reference for all 6 tools
   - Orchestrator integration examples

### 8.3 Medium Priority (P2 - Following 2 Weeks)

**Effort**: ~15 hours

1. Complete dependency extraction (6h)
2. Implement token counting (3h)
3. Add git automation helpers (3h)
   - `parseltongue commit` - auto-generate commit messages
4. Multi-language support exploration (3h)
   - Add Python grammar as proof-of-concept

### 8.4 Future Enhancements

- LSP integration (rust-analyzer) for richer metadata
- Multi-language support (Python, JavaScript, TypeScript)
- Dependency graph queries (blast radius, transitive closure)
- Performance optimizations (parallel indexing, incremental updates)

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

**A**: 87.5% complete. Core pipeline works. Missing: unified binary, performance validation, multi-language support. Good for early adopters; not ready for general release.

---

## 11. CONCLUSION

**Parseltongue is a toolkit, not an agent.**

We provide the **infrastructure** for AI-driven code modification:
- ✅ Indexing codebase into structured database
- ✅ Tracking temporal state (current vs. proposed changes)
- ✅ Generating minimal LLM context
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
- **Simple** in scope (6 focused tools)
- **Testable** (106/108 tests passing)
- **Maintainable** (no coupling to LLM providers)

**Path to 1.0**:
1. Create unified binary (4 hours)
2. Add performance benchmarks (8 hours)
3. Production-ready for early adopters (~2 weeks)

---

## APPENDIX A: Quick Reference Commands

```bash
# 1. Index codebase
parseltongue index ./src --db .parseltongue/db.cozo

# 2. Write proposed change
parseltongue write --db db.cozo --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit --future-code "a + b"

# 3. Read context
parseltongue read --db db.cozo --output context.json [--filter changed]

# 4. Check syntax
parseltongue check --db db.cozo

# 5. Generate diff
parseltongue diff --db db.cozo --output diff.json

# 6. Reset state
parseltongue reset --db db.cozo --project ./
```

---

## APPENDIX B: Example Integration

**Shell script demonstrating full workflow**:

```bash
#!/bin/bash
# fix_bug.sh - Orchestrates Parseltongue tools to fix a bug

set -e  # Exit on error

DB=".parseltongue/db.cozo"
BUG_DESCRIPTION="$1"

echo "Step 1: Indexing codebase..."
parseltongue index ./src --db "$DB"

echo "Step 2: Getting context..."
parseltongue read --db "$DB" --output context.json

echo "Step 3: LLM reasoning (manual step)..."
# (User or AI IDE would call LLM here with context.json + bug description)
# For demo, assume LLM proposes: change line 3 in src/lib.rs

echo "Step 4: Writing proposed fix..."
parseltongue write --db "$DB" \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "a + b  // FIXED: was a - b"

echo "Step 5: Syntax validation..."
parseltongue check --db "$DB" || exit 1

echo "Step 6: Generating diff..."
parseltongue diff --db "$DB" --output diff.json

echo "Step 7: Applying changes (manual step)..."
# (Orchestrator would parse diff.json and write to files)
# For demo:
echo "Changes to apply:"
cat diff.json

echo "Step 8: Build & test..."
cargo build && cargo test || exit 1

echo "Step 9: Resetting state..."
parseltongue reset --db "$DB" --project ./

echo "✅ Bug fix complete!"
```

---

**End of PRDv1.0**

*This document describes Parseltongue as an AI-IDE-agnostic CLI toolkit. For detailed implementation status and code audit, see ConsolidatedPRDv01.md in zzArchivePRDs/.*
