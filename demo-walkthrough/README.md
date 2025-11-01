# Demo Walkthrough - Touch, Feel, Inspect

**A complete Parseltongue pipeline execution preserved as tangible artifacts.**

## What's Here

This folder contains a real execution of the 6-tool pipeline on a simple greeter library.

```
demo-walkthrough/
├── JOURNAL.md                      ← Read this first! Step-by-step walkthrough
├── greeter/                        ← Source code (4 functions, 1 bug)
│   ├── Cargo.toml
│   └── src/lib.rs
├── demo.db/                        ← The RocksDB database (TOUCH IT!)
├── step1-index.log                 ← Tool 1 output
├── step2-all-entities.json         ← All 4 functions indexed
├── step2-export.log                ← Tool 3 output
├── step3-edit.log                  ← Tool 2 output (edit confirmation)
├── step4-validate.log              ← Tool 4 output (syntax valid!)
├── step5-CodeDiff.json             ← THE DIFF (current vs future code)
├── step5-diff.log                  ← Tool 5 output
├── step6-changed-entities.json     ← Before/after state
└── step6-changed.log               ← Tool 3 output (changed only)
```

## Start Here

1. **Read [`JOURNAL.md`](./JOURNAL.md)** - The complete narrative
2. **Inspect [`step5-CodeDiff.json`](./step5-CodeDiff.json)** - The actual diff
3. **Touch `demo.db/`** - A real RocksDB database you can explore
4. **Read [`greeter/src/lib.rs`](./greeter/src/lib.rs)** - The code with the bug

## The Bug

Line 5 in `greeter/src/lib.rs`:
```rust
format!("Goodbye, {}!", name)  // BUG: Should say "Hello"
```

The fix (in `step5-CodeDiff.json`):
```rust
format!("Hello, {}!", name)  // FIXED
```

## Key Files to Inspect

### step5-CodeDiff.json
**This is what the LLM reads to apply changes.** Contains:
- ISGL1 key: `rust:fn:hello:greeter_src_lib_rs:4-6`
- Operation: `EDIT`
- File path: `greeter/src/lib.rs`
- Current code (with bug)
- Future code (fixed)

### step6-changed-entities.json
**Shows the temporal state.** Contains:
- `current_code`: Original buggy version
- `future_code`: Fixed version
- `temporal_state`: `{current_ind: true, future_ind: true, future_action: "Edit"}`

### demo.db/
**A real RocksDB database.** You can:
- See the size: `du -sh demo.db/`
- List files: `ls -la demo.db/`
- **It's real data, not a mock!**

## Reproduce This

From the repository root:

```bash
# Build the binary
cargo build --release

# Run the complete pipeline
cd demo-walkthrough

# Step 1: Index
../target/release/parseltongue folder-to-cozodb-streamer greeter --db rocksdb:my-demo.db

# Step 2: Export
../target/release/parseltongue llm-cozodb-to-context-writer \
  --output my-entities.json --db rocksdb:my-demo.db --filter all

# Step 3: Fix bug
../target/release/parseltongue llm-to-cozodb-writer \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action "edit" \
  --future-code "pub fn hello(name: &str) -> String { format!(\"Hello, {}!\", name) }" \
  --db rocksdb:my-demo.db

# Step 4: Validate
../target/release/parseltongue rust-preflight-code-simulator --db rocksdb:my-demo.db

# Step 5: Generate diff
../target/release/parseltongue llm-cozodb-to-diff-writer \
  --output my-diff.json --db rocksdb:my-demo.db

# Inspect your own diff!
cat my-diff.json | jq
```

## Questions?

- **What are ISGL1 keys?** See the main README.md
- **What is temporal versioning?** Read JOURNAL.md Step 3
- **Why RocksDB?** It's the default CozoDB backend
- **Can I delete demo.db/?** Yes! Run the pipeline again to recreate it

---

**This is Parseltongue in action - not screenshots, not logs, but real artifacts.**
