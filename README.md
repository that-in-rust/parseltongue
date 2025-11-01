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

## The 6 Tools

**All in one unified binary:**
1. `folder-to-cozodb-streamer` - Index codebase
2. `llm-to-cozodb-writer` - Write temporal changes
3. `llm-cozodb-to-context-writer` - Export entities to JSON
4. `rust-preflight-code-simulator` - Validate syntax
5. `llm-cozodb-to-diff-writer` - Generate CodeDiff.json
6. `cozodb-make-future-code-current` - Reset database state

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
# 1. Index the codebase (4 functions discovered)
parseltongue folder-to-cozodb-streamer greeter --db rocksdb:demo.db
# → 4 entities created

# 2. Export all entities to see what was indexed
parseltongue llm-cozodb-to-context-writer \
  --output all-entities.json \
  --db rocksdb:demo.db
# → Uses default query (excludes code, signatures only)

# 3. Fix the hello() function (manual write to database)
# Note: Tool 2 is for LLM batch processing. For demos, entities are updated directly in CozoDB.
# The temporal state is updated: future_action="Edit", future_code contains the fix
# → Temporal state: Edit pending (future_ind=true)

# 4. Validate syntax of the fix
parseltongue rust-preflight-code-simulator --db rocksdb:demo.db
# → ✓ All syntax validations passed

# 5. Generate CodeDiff.json for LLM to apply
parseltongue llm-cozodb-to-diff-writer \
  --output CodeDiff.json \
  --db rocksdb:demo.db
# → CodeDiff.json generated (1 edit)

# 6. (Optional) Reset database to start fresh
parseltongue cozodb-make-future-code-current \
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

### Data Flow

```
Codebase → Tool 1 (Index) → CozoDB
                              ↓
                    Tool 2 (Write Changes)
                              ↓
           Tool 3 (Export) ← CozoDB → Tool 4 (Validate)
                              ↓
                    Tool 5 (Generate Diff)
                              ↓
                    Tool 6 (Reset State)
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
│   ├── parseltongue/           # Unified binary (all 6 tools)
│   ├── parseltongue-core/      # Shared types, storage, entities
│   ├── folder-to-cozodb-streamer/       # Tool 1
│   ├── llm-to-cozodb-writer/            # Tool 2
│   ├── llm-cozodb-to-context-writer/    # Tool 3
│   ├── rust-preflight-code-simulator/   # Tool 4
│   ├── llm-cozodb-to-diff-writer/       # Tool 5
│   └── cozodb-make-future-code-current/ # Tool 6
├── demo-walkthrough/           # Complete example with artifacts
└── examples/calculator/        # Additional example (deliberate bug)
```

---

## Command Reference

### Tool 1: folder-to-cozodb-streamer
```bash
parseltongue folder-to-cozodb-streamer <directory> --db <database>
```
Indexes codebase into CozoDB with ISGL1 keys.

### Tool 2: llm-to-cozodb-writer
```bash
parseltongue llm-to-cozodb-writer \
  --entity <isgl1-key> \
  --action <create|edit|delete> \
  --future-code <code> \
  --db <database>
```
Writes temporal changes to database.

### Tool 3: llm-cozodb-to-context-writer
```bash
parseltongue llm-cozodb-to-context-writer \
  --output <json-file> \
  --db <database> \
  --filter <all|changed|current>
```
Exports entities to JSON for inspection.

### Tool 4: rust-preflight-code-simulator
```bash
parseltongue rust-preflight-code-simulator --db <database> [--verbose]
```
Validates syntax of future_code using tree-sitter.

### Tool 5: llm-cozodb-to-diff-writer
```bash
parseltongue llm-cozodb-to-diff-writer \
  --output <json-file> \
  --db <database>
```
Generates CodeDiff.json with current_code vs. future_code.

### Tool 6: cozodb-make-future-code-current
```bash
parseltongue cozodb-make-future-code-current \
  --project <directory> \
  --db <database>
```
Resets database state (deletes all entities, NO backups).

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
