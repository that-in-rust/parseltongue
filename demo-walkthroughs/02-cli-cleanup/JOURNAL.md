# Example 2: CLI Cleanup - Using Parseltongue on Itself

## Goal
Remove unused command-line arguments from Parseltongue codebase.

## Planning: Commands & Reasoning

### Step 1: Index Codebase
```bash
../target/release/parseltongue folder-to-cozodb-streamer ../crates \
  --db rocksdb:parseltongue-analysis.db
```
**Why**: Create ISG of all CLI definitions across 6 tools. ISGL1 keys will identify all `clap` argument definitions.

**Expected**: ~200-300 entities (functions, structs, CLI args from all 6 tools).

---

### Step 2: Export All Entities
```bash
../target/release/parseltongue llm-cozodb-to-context-writer \
  --output step2-all-entities.json \
  --db rocksdb:parseltongue-analysis.db \
  --filter all
```
**Why**: Inspect what was indexed. Look for CLI argument structs (clap derives) and their usage.

**Expected**: JSON with interface signatures showing all command-line args.

---

### Step 3: Identify Unused Args
**Manual Analysis**: Review `step2-all-entities.json` and source code to identify:
- Which CLI args are defined but never accessed
- Which fields in `Cli` structs are unused

**Expected Findings**: Document specific args to remove (e.g., `--verbose` flags that aren't checked).

---

### Step 4: Propose Deletions (Tool 2)
```bash
../target/release/parseltongue llm-to-cozodb-writer \
  --entity "rust:field:unused_arg:crates_tool1_src_cli_rs:15-17" \
  --action "delete" \
  --db rocksdb:parseltongue-analysis.db
```
**Why**: Mark unused CLI fields for deletion. Set `future_ind=0`, `future_action=Delete`.

**Expected**: Temporal state updated for each unused arg.

---

### Step 5: Validate Syntax (Tool 4)
```bash
../target/release/parseltongue rust-preflight-code-simulator \
  --db rocksdb:parseltongue-analysis.db
```
**Why**: Verify deletions don't break struct syntax.

**Expected**: ✓ Syntax valid (or errors to fix).

---

### Step 6: Generate Diff (Tool 5)
```bash
../target/release/parseltongue llm-cozodb-to-diff-writer \
  --output step6-CodeDiff.json \
  --db rocksdb:parseltongue-analysis.db
```
**Why**: Get structured diff showing what to delete.

**Expected**: `CodeDiff.json` with Delete operations for unused args.

---

### Step 7: Apply Changes
**Manual**: Read `step6-CodeDiff.json` and remove the identified unused arguments from source files.

**Verify**:
```bash
cargo build --workspace
cargo test --workspace
```

---

## Status
🟡 Ready to start - awaiting execution
