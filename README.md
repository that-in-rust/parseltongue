# Parseltongue

**Ultra-minimalist CLI toolkit for code analysis and temporal modification** - Parse code, track changes with temporal versioning, validate syntax, and generate diffs for LLM-driven code transformation.

---

## Quick Install (macOS Apple Silicon)

```bash
# Download the latest binary for macOS ARM64 (M1/M2/M3)
curl -L https://github.com/that-in-rust/parseltongue/releases/latest/download/parseltongue-macos-arm64 -o parseltongue

# Make it executable
chmod +x parseltongue

# Move to your PATH (optional)
sudo mv parseltongue /usr/local/bin/

# Verify installation
parseltongue --help
```

**Or build from source:**
```bash
cargo build --release
./target/release/parseltongue --help
```

---

## What Problem Does It Solve?

LLMs need to understand and modify code across large codebases. Parseltongue provides a 6-tool pipeline that:
- **Indexes** code into a queryable graph database (CozoDB)
- **Tracks** proposed changes with temporal versioning (current vs. future state)
- **Validates** syntax before applying changes
- **Generates** structured diffs for LLM consumption

All in **one unified binary** with self-documenting command names.

---

## The 6 Tools (Workflow-Ordered)

**All in one unified binary:**
1. `pt01-folder-to-cozodb-streamer` - Index codebase (Ingest)
2. `pt02-llm-cozodb-to-context-writer` - Export entities to JSON (Read)
3. `pt03-llm-to-cozodb-writer` - Write temporal changes (Edit)
4. `pt04-syntax-preflight-validator` - Validate syntax (Validate)
5. `pt05-llm-cozodb-to-diff-writer` - Generate CodeDiff.json (Diff)
6. `pt06-cozodb-make-future-code-current` - Reset database state (Reset)

---

## Complete Walkthrough: Fix a Bug in 4 Functions

**See the full demo:** [`demo-walkthrough/`](./demo-walkthrough/)

**Watch video demos:** [Parseltongue Video Tutorials](https://photos.app.goo.gl/eyHCSPBCWb1oaN4d8)

A tangible example with all artifacts preserved (JSONs, logs, database).

### The Scenario

You have a simple greeter library with 4 functions:
- `hello()` - **BUG: says "Goodbye" instead of "Hello"**
- `goodbye()` - works correctly
- `good_morning()` - works correctly
- `good_night()` - works correctly

### The Pipeline

```bash
# 1. INGEST: Index the codebase (4 functions discovered)
parseltongue pt01-folder-to-cozodb-streamer greeter --db rocksdb:demo.db
# → 4 entities created

# 2. READ: Export all entities to see what was indexed
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:demo.db
# → Uses default query (excludes Current_Code, signatures only)
# → Generates: ./contexts/context_{uuid}_{timestamp}.json

# 3. EDIT: Fix the hello() function (simple interface)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action edit \
  --future-code 'pub fn hello() -> &'"'"'static str { "Hello!" }' \
  --db rocksdb:demo.db
# → Temporal state: Edit pending (future_ind=true)

# 4. VALIDATE: Check syntax of the fix
parseltongue pt04-syntax-preflight-validator --db rocksdb:demo.db
# → ✓ All syntax validations passed

# 5. DIFF: Generate CodeDiff.json for LLM to apply
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output CodeDiff.json \
  --db rocksdb:demo.db
# → CodeDiff.json generated (1 edit with before/after)

# 6. RESET: (Optional) Reset database to start fresh
parseltongue pt06-cozodb-make-future-code-current \
  --project greeter \
  --db rocksdb:demo.db
# → 4 entities deleted, schema recreated
```

### What You Get

The `demo-walkthrough/` folder contains:
- **greeter/** - The source code with the bug
- **step1-index.log** - Indexing output (4 entities created)
- **step2-all-entities.json** - All 4 functions with metadata
- **step3-edit.log** - Temporal write confirmation
- **step4-validate.log** - Syntax validation passed
- **step5-CodeDiff.json** - The diff showing current_code vs. future_code
- **step6-changed-entities.json** - The hello() function with before/after state
- **demo.db/** - The RocksDB database (tangible proof!)

**👉 Everything is preserved - touch it, feel it, inspect it.**

---

## Architecture

### Temporal Versioning System

Every code entity has three temporal indicators:
- `current_ind` - Does it exist in current codebase? (bool)
- `future_ind` - Will it exist after changes? (bool)
- `future_action` - What to do? (Create/Edit/Delete)

**State Transitions:**
```
(1,1,null)   → Unchanged entity
(1,1,Edit)   → Modification pending
(1,0,Delete) → Deletion pending
(0,1,Create) → Creation pending
```

### ISGL1 Keys

Unique identifiers for code entities:
```
rust:fn:hello:greeter_src_lib_rs:4-6
│    │   │     │                 │
│    │   │     │                 └─ Line range (start-end)
│    │   │     └─ File path (sanitized with underscores)
│    │   └─ Function name
│    └─ Entity type (fn/struct/trait/etc)
└─ Language
```

### Data Flow (Workflow Order)

```
Codebase → pt01 (Ingest) → CozoDB
                             ↓
                   pt02 (Read/Export) → JSON for LLM
                             ↓
                   pt03 (Edit/Write) ← LLM Changes
                             ↓
                   pt04 (Validate) → Syntax Check
                             ↓
                   pt05 (Diff) → CodeDiff.json
                             ↓
                   pt06 (Reset) → Clean State
```

---

## Dependencies

- **Rust 2021 Edition**
- **CozoDB** (embedded graph database with RocksDB backend)
- **tree-sitter** (syntax parsing)
- **clap** (CLI framework)
- **serde_json** (JSON serialization)

---

## Design Principles

Following **S01 (Steering Doc #1)**:
1. **TDD-First**: RED → GREEN → REFACTOR cycle
2. **Executable Specifications**: Tests define contracts
3. **Dependency Injection**: Traits, not concrete types
4. **anyhow** for applications, **thiserror** for libraries
5. **Functional Composition**: Pure transformations
6. **Ultra-Minimalist**: NO backups, NO complexity, single reliable operations

---

## Performance

Tool performance on greeter demo (4 entities):
- **Tool 1 (Index)**: 3.5ms
- **Tool 2 (Write)**: <1ms
- **Tool 3 (Export)**: <1ms
- **Tool 4 (Validate)**: <20ms
- **Tool 5 (Diff)**: <1ms
- **Tool 6 (Reset)**: <5ms

**Total pipeline: <30ms** for simple project.

---

## Project Structure

```
parseltongue/
├── crates/
│   ├── parseltongue/                         # Unified binary (all 6 tools)
│   ├── parseltongue-core/                    # Shared types, storage, entities
│   ├── pt01-folder-to-cozodb-streamer/       # Tool 1: Ingest
│   ├── pt02-llm-cozodb-to-context-writer/    # Tool 2: Read
│   ├── pt03-llm-to-cozodb-writer/            # Tool 3: Edit
│   ├── pt04-syntax-preflight-validator/      # Tool 4: Validate
│   ├── pt05-llm-cozodb-to-diff-writer/       # Tool 5: Diff
│   └── pt06-cozodb-make-future-code-current/ # Tool 6: Reset
├── demo-walkthrough/           # Complete example with artifacts
└── examples/calculator/        # Additional example (deliberate bug)
```

---

## Command Reference

### pt01: folder-to-cozodb-streamer (INGEST)
```bash
# Index current directory (default)
parseltongue pt01-folder-to-cozodb-streamer .

# Index specific directory with custom database
parseltongue pt01-folder-to-cozodb-streamer ./crates --db rocksdb:analysis.db --verbose
```
**What it does:** Indexes codebase into CozoDB with ISGL1 keys. Processes ALL files - tree-sitter determines what it can parse.

**Arguments:**
- `<directory>` - Directory to index [default: `.`]
- `--db` - Database path [default: `parseltongue.db`]
- `--verbose` - Show detailed output
- `--quiet` - Suppress output

---

### pt02: llm-cozodb-to-context-writer (READ)
```bash
# Default: Export signatures only (token-optimized ~37.5k for 1500 entities)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:analysis.db

# Include Current_Code for debugging (~500k tokens - use sparingly!)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:analysis.db \
  --include-current-code 1

# Advanced: Custom query to filter specific entities
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL" \
  --output ./contexts \
  --db rocksdb:analysis.db
```

**What it does:** Exports entities to JSON for LLM consumption. Uses default query that excludes `Current_Code` to save tokens.

**Arguments:**
- `--output` - Output directory [default: `./contexts`]
- `--db` - Database path [default: `parseltongue.db`]
- `--include-current-code` - Include Current_Code field (0=exclude, 1=include) [default: 0]
- `--query` - Custom SQL query (overrides default)

**Default query:**
```sql
SELECT * EXCEPT (Current_Code, Future_Code)
FROM CodeGraph
WHERE current_ind=1
```

---

### pt03: llm-to-cozodb-writer (EDIT)

**Simple Interface (80% of use cases):**
```bash
# Create new entity
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:new_func:src_lib_rs:10-15" \
  --action create \
  --future-code "pub fn new_func() { println!(\"Hello\"); }" \
  --db rocksdb:analysis.db

# Edit existing entity
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() -> &'static str { \"Hello!\" }" \
  --db rocksdb:analysis.db

# Delete entity
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:old_func:src_lib_rs:20-25" \
  --action delete \
  --db rocksdb:analysis.db
```

**Advanced Interface (20% - raw Datalog):**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --query "?[...] := [[...]] :put CodeGraph {...}" \
  --db rocksdb:analysis.db
```

**Arguments:**
- `--entity` - ISGL1 key of entity to modify
- `--action` - Action: create, edit, or delete
- `--future-code` - Future code content (required for create/edit)
- `--query` - Raw Datalog query (advanced users)
- `--db` - Database path [default: `parseltongue.db`]

---

### pt04: syntax-preflight-validator (VALIDATE)
```bash
parseltongue pt04-syntax-preflight-validator --db rocksdb:analysis.db [--verbose]
```

**What it does:** Validates syntax of all `Future_Code` using tree-sitter. Multi-language ready (currently Rust implemented).

**Arguments:**
- `--db` - Database path [default: `parseltongue.db`]
- `--verbose` - Show detailed validation output

---

### pt05: llm-cozodb-to-diff-writer (DIFF)
```bash
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output CodeDiff.json \
  --db rocksdb:analysis.db
```

**What it does:** Generates CodeDiff.json with current_code vs. future_code for all entities with Future_Action set.

**Arguments:**
- `--output` - Output file path [default: `CodeDiff.json`]
- `--db` - Database path [default: `parseltongue.db`]

---

### pt06: cozodb-make-future-code-current (RESET)
```bash
parseltongue pt06-cozodb-make-future-code-current \
  --project ./greeter \
  --db rocksdb:analysis.db
```

**What it does:** Resets database state (deletes CodeGraph table, re-indexes project). **NO backups** - ultra-minimalist.

**Arguments:**
- `--project` - Project directory to re-index
- `--db` - Database path [default: `parseltongue.db`]

---

## FAQ

**Q: Why "Parseltongue"?**
A: Speaking to code like speaking to snakes - understanding its structure and transforming it.

**Q: Why one unified binary instead of 6 separate tools?**
A: Better for LLM reasoning - command names match crate architecture exactly. Self-documenting and consistent.

**Q: Why RocksDB instead of SQLite?**
A: RocksDB is the default compiled backend for CozoDB. Provides better performance for graph queries.

**Q: Can I use this with non-Rust code?**
A: Currently optimized for Rust. Tree-sitter supports multiple languages, but tool implementation focuses on Rust first.

**Q: What's the "ultra-minimalist" principle?**
A: NO backups, NO configuration complexity, NO safety levels. Direct operations only. Trust the LLM and validate syntax.

**Q: How do I apply the changes from CodeDiff.json?**
A: That's the LLM's job! Tool 5 generates the diff, the LLM reads it and writes files. Ultra-minimalist separation of concerns.

---

## License

MIT

---

**Built with functional Rust, TDD-first principles, and ultra-minimalist design.**
**For LLM-driven code transformation workflows.**
