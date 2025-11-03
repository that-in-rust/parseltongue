# Parseltongue Command Reference

**Version**: 1.0
**Last Updated**: 2025-11-01
**Purpose**: Comprehensive command-line reference for all Parseltongue tools

---

## Tool 1: folder-to-cozodb-streamer

**Purpose**: Index Rust codebase into CozoDB database

### Synopsis

```bash
folder-to-cozodb-streamer [OPTIONS] <DIRECTORY>
```

### Arguments

- `<DIRECTORY>` - Path to source code directory to index

### Options

- `--db <PATH>` - Database file path (default: `parseltongue.db`)
- `--verbose` - Enable verbose output with progress information
- `--quiet` - Suppress all output except errors
- `--filter <PATTERN>` - Glob pattern to filter files (default: `**/*.rs`)
- `--exclude <PATTERN>` - Glob pattern to exclude files (e.g., `target/**`)

### Examples

```bash
# Basic indexing
folder-to-cozodb-streamer ./src --db .parseltongue/db.cozo

# Verbose output
folder-to-cozodb-streamer ./src --db db.cozo --verbose

# Filter specific files
folder-to-cozodb-streamer . --db db.cozo --filter "src/**/*.rs" --exclude "target/**"

# Quiet mode (only errors)
folder-to-cozodb-streamer ./src --db db.cozo --quiet
```

### Output

- **Success**: Exit code 0, entity count printed to stdout
- **Failure**: Exit code 1, error message to stderr

### Context

**When to use**:
- Initial project setup (index entire codebase)
- After external changes (e.g., git pull, manual edits)
- After Tool 6 reset (automatic re-indexing)

**Performance**: ~16ms for 45 entities (small projects). Target: <30s for 50k LOC (unvalidated).

### Data Written

Populates `CodeGraph` table with:
- `ISGL1_key`: Line-based format `rust:fn:name:path:start-end`
- `Current_Code`: Full entity source code
- `interface_signature`: Function/struct signature
- `TDD_Classification`: TEST_IMPLEMENTATION | CODE_IMPLEMENTATION
- `current_ind`: true
- `future_ind`: true
- `Future_Action`: null

---

## Tool 2: llm-to-cozodb-writer

**Purpose**: Write LLM-proposed changes to CozoDB temporal state

### Synopsis

```bash
llm-to-cozodb-writer [OPTIONS] --entity <KEY> --action <ACTION>
```

### Required Options

- `--entity <KEY>` - ISGL1 key of entity to modify
- `--action <ACTION>` - Action type: `create`, `edit`, or `delete`

### Optional Options

- `--db <PATH>` - Database file path (default: `parseltongue.db`)
- `--future-code <CODE>` - New code content (required for create/edit, invalid for delete)
- `--batch` - Batch mode (no confirmation prompts)
- `--dry-run` - Show what would be written without writing

### Examples

```bash
# Edit existing entity
llm-to-cozodb-writer --db db.cozo \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "pub fn add(a: i32, b: i32) -> i32 { a + b }"

# Delete entity
llm-to-cozodb-writer --db db.cozo \
  --entity "rust:fn:old_func:src_lib_rs:10-15" \
  --action delete

# Create new entity (hash-based key)
llm-to-cozodb-writer --db db.cozo \
  --entity "src_lib_rs-new_feature-fn-abc12345" \
  --action create \
  --future-code "pub fn new_feature() -> i32 { 42 }"

# Batch mode (no prompts)
llm-to-cozodb-writer --db db.cozo \
  --entity "rust:fn:test:src_lib_rs:20-30" \
  --action edit \
  --future-code "..." \
  --batch

# Dry run (preview changes)
llm-to-cozodb-writer --db db.cozo \
  --entity "..." \
  --action edit \
  --future-code "..." \
  --dry-run
```

### Output

- **Success**: Exit code 0, confirmation message to stdout
- **Failure**: Exit code 1, error message to stderr

### Context

**When to use**:
- After LLM reasoning about code changes
- Iteratively (multiple calls for multiple entities)
- Before validation (Tool 4) and diff generation (Tool 5)

**Temporal State Updates**:
- `edit`: Sets `future_code`, `future_action=Edit`, `future_ind=true`
- `delete`: Sets `future_action=Delete`, `future_ind=false`
- `create`: Sets `future_code`, `future_action=Create`, `current_ind=false`, `future_ind=true`

### Error Conditions

- Entity not found (for edit/delete)
- Entity already exists (for create)
- Invalid ISGL1 key format
- Missing `--future-code` for create/edit
- `--future-code` provided for delete

---

## Tool 3: llm-cozodb-to-context-writer

**Purpose**: Generate minimal LLM context from CozoDB

### Synopsis

```bash
llm-cozodb-to-context-writer [OPTIONS] --output <FILE>
```

### Required Options

- `--output <FILE>` - Output JSON file path

### Optional Options

- `--db <PATH>` - Database file path (default: `parseltongue.db`)
- `--filter <FILTER>` - Filter entities: `all`, `changed`, or `current` (default: `all`)
- `--include-code` - Include `current_code` in output (bloats context)
- `--max-tokens <N>` - Maximum token limit (default: 100000)
- `--format <FORMAT>` - Output format: `json` or `compact` (default: `json`)

### Examples

```bash
# Generate full context
llm-cozodb-to-context-writer --db db.cozo --output context.json

# Only changed entities
llm-cozodb-to-context-writer --db db.cozo --output context_changed.json --filter changed

# Include code content (larger context)
llm-cozodb-to-context-writer --db db.cozo --output context_with_code.json --include-code

# Compact format (single line)
llm-cozodb-to-context-writer --db db.cozo --output context.json --format compact

# Token limit enforcement
llm-cozodb-to-context-writer --db db.cozo --output context.json --max-tokens 50000
```

### Output

**Success**: Exit code 0, JSON file written to `--output` path

**JSON Structure**:
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

**Failure**: Exit code 1, error message to stderr

### Context

**When to use**:
- Before LLM reasoning (get full codebase context)
- After Tool 2 writes (get context with proposed changes)
- Iteratively during refinement cycles

**Filter Options**:
- `all`: All entities with `current_ind=true` (default)
- `changed`: Only entities with `future_action != null`
- `current`: All current entities excluding future state

**Token Optimization**:
- Default: Excludes `current_code` and `future_code` (~37.5k tokens for 1500 entities)
- With `--include-code`: Includes `current_code` (may exceed 100k tokens)

### Performance

- Target: <500ms for context generation (unvalidated)
- Target: <100k tokens for 1500 entities (validated at ~37.5k without code)

---

## Tool 4: rust-preflight-code-simulator

**Purpose**: Syntax validation for proposed changes

### Synopsis

```bash
rust-preflight-code-simulator [OPTIONS]
```

### Options

- `--db <PATH>` - Database file path (default: `parseltongue.db`)
- `--validation-type <TYPE>` - Validation level: `syntax` (default: `syntax`, only option)
- `--verbose` - Show detailed error locations
- `--json` - Output errors in JSON format

### Examples

```bash
# Basic syntax validation
rust-preflight-code-simulator --db db.cozo

# Verbose output
rust-preflight-code-simulator --db db.cozo --verbose

# JSON output (for parsing)
rust-preflight-code-simulator --db db.cozo --json
```

### Output

**Success**: Exit code 0, "All entities valid" to stdout

**Syntax Errors**: Exit code 1, error list to stderr

**Error Format** (verbose):
```
Entity: rust:fn:add:src_lib_rs:2-4
Line 3, Column 15: Unexpected token '}'
  Expected: ';' or expression
```

**Error Format** (JSON):
```json
{
  "errors": [
    {
      "entity": "rust:fn:add:src_lib_rs:2-4",
      "line": 3,
      "column": 15,
      "message": "Unexpected token '}'",
      "suggestion": "Expected: ';' or expression"
    }
  ]
}
```

### Context

**When to use**:
- After Tool 2 writes (before generating diff)
- Before applying changes to files
- As fast pre-flight check (<20ms target, unvalidated)

**What it validates**:
- ✅ Syntax errors (missing brackets, malformed expressions)
- ❌ NOT type errors (use cargo build for this)
- ❌ NOT borrow checker (use cargo build for this)
- ❌ NOT import resolution (use cargo build for this)
- ❌ NOT logic validation (use cargo test for this)

**Scope**: Ultra-minimalist - syntax only (tree-sitter parsing)

**Rationale**: Keep validation fast. Let orchestrator run cargo build/test after file writes.

### Performance

- Target: <20ms for 50 entities (unvalidated)
- Validates only entities with `future_code != null`

---

## Tool 5: llm-cozodb-to-diff-writer

**Purpose**: Generate CodeDiff.json for file application

### Synopsis

```bash
llm-cozodb-to-diff-writer [OPTIONS] --output <FILE>
```

### Required Options

- `--output <FILE>` - Output JSON file path

### Optional Options

- `--db <PATH>` - Database file path (default: `parseltongue.db`)
- `--format <FORMAT>` - Output format: `json` or `compact` (default: `json`)
- `--include-context` - Include surrounding code context (not just changed lines)

### Examples

```bash
# Generate diff
llm-cozodb-to-diff-writer --db db.cozo --output diff.json

# Compact format
llm-cozodb-to-diff-writer --db db.cozo --output diff.json --format compact

# With context (for inspection)
llm-cozodb-to-diff-writer --db db.cozo --output diff.json --include-context
```

### Output

**Success**: Exit code 0, JSON file written to `--output` path

**JSON Structure**:
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
    },
    {
      "isgl1_key": "rust:fn:old_func:src_lib_rs:10-15",
      "operation": "Delete",
      "file_path": "src/lib.rs",
      "line_range": {"start": 10, "end": 15},
      "current_code": "pub fn old_func() { ... }",
      "future_code": null
    },
    {
      "isgl1_key": "src_lib_rs-new_feature-fn-abc12345",
      "operation": "Create",
      "file_path": "src/lib.rs",
      "line_range": null,
      "current_code": null,
      "future_code": "pub fn new_feature() -> i32 { 42 }"
    }
  ]
}
```

**Failure**: Exit code 1, error message to stderr

### Context

**When to use**:
- After Tool 4 validation passes
- Before orchestrator applies changes to files
- Once per workflow (after all Tool 2 writes)

**What it includes**:
- All entities with `future_action != null`
- Operation type (Create, Edit, Delete)
- File paths (desanitized: `src_lib_rs` → `src/lib.rs`)
- Line ranges (for Edit and Delete)
- Current and future code content

**Ultra-Minimalist Design**:
- ❌ NO file writing (orchestrator applies changes)
- ❌ NO backup files (orchestrator handles undo)
- ✅ Single JSON output for inspection/application

### Performance

- Target: <100ms for diff generation (unvalidated)
- Queries only entities with `future_action != null`

---

## Tool 6: cozodb-make-future-code-current

**Purpose**: Reset database state after changes applied

### Synopsis

```bash
cozodb-make-future-code-current [OPTIONS] --project <DIR>
```

### Required Options

- `--project <DIR>` - Project root directory (for re-indexing)

### Optional Options

- `--db <PATH>` - Database file path (default: `parseltongue.db`)
- `--verbose` - Show progress during re-indexing
- `--skip-reindex` - Skip re-indexing (just clear database)

### Examples

```bash
# Full reset (delete + re-index)
cozodb-make-future-code-current --db db.cozo --project ./

# Verbose output
cozodb-make-future-code-current --db db.cozo --project ./ --verbose

# Skip re-index (just clear)
cozodb-make-future-code-current --db db.cozo --project ./ --skip-reindex
```

### Output

**Success**: Exit code 0, "Database reset complete" to stdout

**Failure**: Exit code 1, error message to stderr

### Context

**When to use**:
- After successfully applying changes to files
- After orchestrator validates build/test passed
- Before starting a new workflow

**What it does**:
1. Deletes all entities from CodeGraph table
2. (If not `--skip-reindex`) Re-runs Tool 1 to re-index codebase
3. Resets temporal state: all entities become `(current_ind=true, future_ind=true, future_action=null)`

**Ultra-Minimalist Design**:
- ❌ NO backup metadata
- ❌ NO rollback mechanism
- ✅ Fresh rebuild from source files (simplest = most reliable)

**Why no backups?**
- Ultra-minimalist principle
- Trust orchestrator's undo mechanism (git, editor history)
- Adding backups adds complexity and failure modes

### Performance

- Same as Tool 1 (re-indexing): Target <30s for 50k LOC (unvalidated)

---

## Common Workflows

**See .domainDocs/D13-workflow-patterns.md for detailed workflow patterns**

### Quick Reference: Basic Bug Fix

```bash
# 1. Index
folder-to-cozodb-streamer ./src --db db.cozo

# 2. Get context
llm-cozodb-to-context-writer --db db.cozo --output context.json

# 3. (LLM reasons, proposes changes)

# 4. Write changes
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "..."

# 5. Validate
rust-preflight-code-simulator --db db.cozo

# 6. Generate diff
llm-cozodb-to-diff-writer --db db.cozo --output diff.json

# 7. (Orchestrator applies changes, runs cargo build/test)

# 8. Reset
cozodb-make-future-code-current --db db.cozo --project ./
```

---

## Exit Codes

All tools follow standard Unix exit code conventions:

- `0` - Success
- `1` - General error (see stderr for details)
- `2` - Invalid arguments
- `3` - Database error (file not found, permission denied, corruption)
- `4` - Validation error (Tool 4 only - syntax errors found)

---

## Environment Variables

- `PARSELTONGUE_DB_PATH` - Default database path (overrides `parseltongue.db` default)
- `PARSELTONGUE_VERBOSE` - Enable verbose output for all tools (set to `1`)
- `PARSELTONGUE_LOG_LEVEL` - Logging level: `error`, `warn`, `info`, `debug`, `trace`

### Examples

```bash
# Use custom default database path
export PARSELTONGUE_DB_PATH=".parseltongue/state.cozo"
folder-to-cozodb-streamer ./src  # Uses .parseltongue/state.cozo

# Enable verbose output globally
export PARSELTONGUE_VERBOSE=1
llm-cozodb-to-context-writer --output context.json  # Verbose output

# Debug logging
export PARSELTONGUE_LOG_LEVEL=debug
rust-preflight-code-simulator  # Shows debug logs
```

---

## Database Schema Reference

### CodeGraph Table

```
CodeGraph {
    ISGL1_key: String =>           // Primary key
    Current_Code: String?,         // Current version
    Future_Code: String?,          // Proposed version
    interface_signature: String,   // Function/struct signature
    TDD_Classification: String,    // TEST_IMPLEMENTATION | CODE_IMPLEMENTATION
    lsp_meta_data: String?,        // Optional LSP metadata
    current_ind: Bool,             // Exists in current codebase
    future_ind: Bool,              // Will exist after changes
    Future_Action: String?,        // "Create" | "Edit" | "Delete"
    file_path: String,             // Source file path
    language: String,              // "rust"
    last_modified: String,         // Timestamp
    entity_type: String            // "Function" | "Struct" | etc.
}
```

### Temporal State Matrix

| current_ind | future_ind | future_action | Meaning | Set By |
|-------------|------------|---------------|---------|--------|
| true | true | null | Unchanged entity | Tool 1 |
| true | true | "Edit" | Modification pending | Tool 2 |
| true | false | "Delete" | Deletion pending | Tool 2 |
| false | true | "Create" | Creation pending | Tool 2 |

---

## ISGL1 Key Formats

### Line-Based (Existing Entities)

**Format**: `{language}:{type}:{name}:{sanitized_path}:{start}-{end}`

**Examples**:
```
rust:fn:calculate_sum:src_lib_rs:42-56
rust:struct:Config:src_config_rs:10-25
rust:trait:Display:src_traits_rs:5-15
```

**Path Sanitization**: Replace `/`, `\`, `.` with `_`
- `src/lib.rs` → `src_lib_rs`
- `src/module/helper.rs` → `src_module_helper_rs`

### Hash-Based (New Entities)

**Format**: `{sanitized_filepath}-{entity_name}-{type_abbrev}-{hash8}`

**Examples**:
```
src_lib_rs-new_feature-fn-abc12345
src_config_rs-Settings-struct-def67890
```

**Hash Algorithm**: SHA-256(filepath + name + type + timestamp) → first 8 chars

**Type Abbreviations**:
- `fn` - Function
- `struct` - Struct
- `enum` - Enum
- `trait` - Trait
- `impl` - Impl block
- `mod` - Module

---

## Troubleshooting

### Database locked

**Error**: `Database file is locked`

**Cause**: Another process has the database open

**Solution**: Ensure only one workflow at a time, or use separate database files

---

### Entity not found

**Error**: `Entity with key '...' not found`

**Cause**: ISGL1 key doesn't exist in database (typo or not indexed)

**Solution**:
1. Check key format is correct
2. Verify entity exists: `llm-cozodb-to-context-writer --db db.cozo --output context.json` and inspect
3. Re-index if needed: `folder-to-cozodb-streamer ./src --db db.cozo`

---

### Validation failed

**Error**: Exit code 4 from Tool 4

**Cause**: Syntax errors in `future_code`

**Solution**:
1. Run with `--verbose` to see exact error locations
2. Check LLM-generated code for syntax issues
3. Re-run LLM reasoning to fix syntax
4. Use `--json` for programmatic error parsing

---

### Missing future_code

**Error**: `--future-code required for action 'edit'`

**Cause**: Forgot to provide `--future-code` argument

**Solution**: Include `--future-code "..."` for create/edit actions

---

### File paths don't match

**Error**: Generated diff has wrong file paths

**Cause**: ISGL1 keys have sanitized paths that need desanitization

**Solution**: Tool 5 handles this automatically (`src_lib_rs` → `src/lib.rs`)

---

## Performance Considerations

### Tool 1 (Indexing)

**Performance depends on**:
- Codebase size (LOC)
- Number of entities (functions, structs, etc.)
- Disk I/O speed

**Optimization**:
- Use `--filter` to index only relevant files
- Use `--exclude` to skip build artifacts (`target/**`)

---

### Tool 3 (Context Generation)

**Token budget management**:
- Default: ~37.5k tokens for 1500 entities (no code content)
- With `--include-code`: May exceed 100k tokens (use `--max-tokens` to limit)

**Optimization**:
- Use `--filter changed` to get only modified entities
- Avoid `--include-code` unless necessary

---

### Tool 4 (Validation)

**Fast by design**:
- Syntax-only validation (tree-sitter parsing)
- Target: <20ms for 50 entities (unvalidated)

**Not a bottleneck** - Run liberally before diff generation

---

### Tool 5 (Diff Generation)

**Fast operation**:
- Single database query
- JSON serialization
- Target: <100ms (unvalidated)

**Not a bottleneck** - Run once per workflow

---

### Tool 6 (Reset)

**Performance same as Tool 1**:
- Deletes entities (fast)
- Re-indexes codebase (dominant cost)

**Optimization**: Use `--skip-reindex` if you'll immediately re-index with different filters

---

## Version History

**v1.0 (2025-11-01)**:
- Initial comprehensive command reference
- All 6 tools documented
- Examples, context, troubleshooting added

---

**End of Command Reference**
