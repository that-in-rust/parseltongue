# Demo Walkthrough: Fixing a Bug with Parseltongue

**A tangible, feelable journey through the 6-tool pipeline.**

---

## The Bug

We have a greeter library with 4 functions. The `hello()` function has a bug - it says "Goodbye" instead of "Hello"!

```rust
// greeter/src/lib.rs
pub fn hello(name: &str) -> String {
    format!("Goodbye, {}!", name)  // BUG: Should say "Hello"
}
```

---

## Step 1: Index the Codebase

**Command:**
```bash
parseltongue folder-to-cozodb-streamer greeter --db rocksdb:demo.db
```

**Result:** [`step1-index.log`](./step1-index.log)
```
✓ Indexing completed
  Files processed: 1
  Entities created: 4
  Duration: 3.5ms
```

**What happened:**
- Parsed `greeter/src/lib.rs` with tree-sitter
- Discovered 4 functions: `hello`, `goodbye`, `good_morning`, `good_night`
- Generated ISGL1 keys for each
- Stored in RocksDB at `demo.db/`

---

## Step 2: See What Was Indexed

**Command:**
```bash
parseltongue llm-cozodb-to-context-writer \
  --output step2-all-entities.json \
  --db rocksdb:demo.db
# Uses default query: SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph WHERE current_ind=1
```

**Result:** [`step2-all-entities.json`](./step2-all-entities.json) + [`step2-export.log`](./step2-export.log)
```
✓ Context JSON written
  Entities exported: 4
```

**What's inside the JSON:**
- All 4 functions with their ISGL1 keys
- Current code for each function
- Line ranges (4-6, 9-11, 14-16, 19-21)
- Temporal state: `(current_ind=true, future_ind=true, future_action=null)` → unchanged

**Key insight:** The ISGL1 key for hello() is:
```
rust:fn:hello:greeter_src_lib_rs:4-6
```

---

## Step 3: Fix the Bug

**Command:**
```bash
# Tool 2 is for LLM batch processing with --query to select entities.
# For this demo, the entity was updated directly in CozoDB:
#   - Set future_code to the corrected version
#   - Set future_action to "Edit"
#   - Set future_ind to true
```

**Result:** [`step3-edit.log`](./step3-edit.log)
```
✓ Entity updated with future code
  Temporal state: Edit pending (future_ind=true)
```

**What happened:**
- Fetched the hello() entity from database
- Updated `future_code` field with the fix
- Set `temporal_state.future_action = "Edit"`
- Persisted back to database

**Temporal state is now:**
```
current_ind=true    (exists in current codebase)
future_ind=true     (will exist after changes)
future_action="Edit" (action to perform)
```

---

## Step 4: Validate the Fix

**Command:**
```bash
parseltongue rust-preflight-code-simulator --db rocksdb:demo.db
```

**Result:** [`step4-validate.log`](./step4-validate.log)
```
Validating 1 changed entities...
✓ All syntax validations passed
  Entities validated: 1
```

**What happened:**
- Found 1 entity with `future_action != null`
- Parsed `future_code` with tree-sitter
- Checked for syntax errors (missing brackets, typos, etc.)
- ✓ No errors found!

**Note:** This is syntax-only validation. Type errors, imports, lifetimes are checked by `cargo build` later.

---

## Step 5: Generate the Diff

**Command:**
```bash
parseltongue llm-cozodb-to-diff-writer \
  --output step5-CodeDiff.json \
  --db rocksdb:demo.db
```

**Result:** [`step5-CodeDiff.json`](./step5-CodeDiff.json) + [`step5-diff.log`](./step5-diff.log)
```
✓ CodeDiff.json generated
  Changes included: 1
    Creates: 0
    Edits: 1
    Deletes: 0
```

**What's in CodeDiff.json:**
```json
{
  "metadata": {
    "generated_at": "2025-11-01T...",
    "total_changes": 1
  },
  "changes": [
    {
      "isgl1_key": "rust:fn:hello:greeter_src_lib_rs:4-6",
      "operation": "EDIT",
      "file_path": "greeter/src/lib.rs",
      "line_range": { "start": 4, "end": 6 },
      "current_code": "pub fn hello(name: &str) -> String {\n    format!(\"Goodbye, {}!\", name)  // BUG\n}",
      "future_code": "pub fn hello(name: &str) -> String { format!(\"Hello, {}!\", name) }"
    }
  ]
}
```

**This is what the LLM reads to apply changes to files.**

---

## Step 6: Verify the Change

**Command:**
```bash
parseltongue llm-cozodb-to-context-writer \
  --output step6-changed-entities.json \
  --db rocksdb:demo.db \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL"
# Query loads ONLY changed entities (with code included)
```

**Result:** [`step6-changed-entities.json`](./step6-changed-entities.json) + [`step6-changed.log`](./step6-changed.log)
```
✓ Context JSON written
  Entities exported: 1
```

**What's inside:**
Only the hello() function, showing:
- `current_code`: The buggy version (says "Goodbye")
- `future_code`: The fixed version (says "Hello")
- `temporal_state`: `{current_ind: true, future_ind: true, future_action: "Edit"}`

**Perfect! The before/after state is captured.**

---

## The Complete Artifact Trail

Every step is preserved in this folder:

| File | What It Is |
|------|-----------|
| `greeter/` | The source code with the bug |
| `demo.db/` | The RocksDB database (touch it, feel it!) |
| `step1-index.log` | Indexing output (4 entities created) |
| `step2-all-entities.json` | All 4 functions with full metadata |
| `step2-export.log` | Export confirmation |
| `step3-edit.log` | Temporal write confirmation |
| `step4-validate.log` | Syntax validation passed |
| `step5-CodeDiff.json` | **The diff for LLM to apply** |
| `step5-diff.log` | Diff generation confirmation |
| `step6-changed-entities.json` | Before/after state of hello() |
| `step6-changed.log` | Changed entities export confirmation |

---

## Key Insights

### 1. Temporal Versioning in Action
The database doesn't modify the original `current_code`. It tracks the proposed `future_code` alongside it with temporal indicators. This enables:
- Safe exploration of changes
- Rollback capability
- Diff generation
- Validation before applying

### 2. ISGL1 Keys Are Stable
```
rust:fn:hello:greeter_src_lib_rs:4-6
```
This key is deterministic - same file, same function name, same line range → same key. Perfect for graph queries.

### 3. Ultra-Minimalist Design
- NO intermediate files during indexing
- NO backups (Tool 6 deletes permanently)
- NO configuration files
- Single database, single binary, single source of truth

### 4. LLM-Centric Workflow
Parseltongue **generates** CodeDiff.json. The **LLM reads and applies** it to files. Clean separation:
- **Parseltongue**: Understand structure, track changes, validate syntax
- **LLM**: Apply changes to files, handle imports, fix types

---

## What's Next?

After generating CodeDiff.json, the LLM would:
1. Read `step5-CodeDiff.json`
2. Open `greeter/src/lib.rs`
3. Navigate to lines 4-6
4. Replace `current_code` with `future_code`
5. Save the file
6. Run `cargo build` to verify types/imports
7. Run `cargo test` to verify behavior

**Then you can reset the database:**
```bash
parseltongue cozodb-make-future-code-current \
  --project greeter \
  --db rocksdb:demo.db
```

And re-index to start fresh!

---

**This is Parseltongue - code analysis you can touch, feel, and inspect.**
**Every artifact preserved. Every step traceable. Every decision explicit.**
