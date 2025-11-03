# v0.8.6 Release Test Plan

**Date**: 2025-11-02
**Binary**: `/target/release/parseltongue`
**Test Subject**: Parseltongue codebase (self-analysis)
**Database**: `rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db`

---

## Test Strategy

### Unit Tests (KEEP - Fast feedback)
- ✅ **31 PT02 unit tests** pass (query builder, exporters, models)
- ✅ Test exporter logic with mocks (no database required)
- ✅ Fast iteration during development

### Integration Tests (THIS IS WHAT WE'RE DOING NOW)
- Test REAL CozoDB with actual Parseltongue data
- Verify all 8 commands work end-to-end
- Document actual performance and results

---

## Commands to Test

| Tool | Status | Purpose |
|------|--------|---------|
| PT01 | ⏳ Testing | Index codebase → CozoDB |
| PT02-level00 | ⏳ Testing | Export dependency edges |
| PT02-level01 | ⏳ Testing | Export entities + ISG |
| PT02-level02 | ⏳ Testing | Export + type system |
| PT03 | ⏳ Testing | Write temporal changes |
| PT04 | ⏳ Testing | Validate syntax |
| PT05 | ⏳ Testing | Generate diffs |
| PT06 | ⏳ Testing | Reset database |

---

## Test Execution

### Test 1: PT01 - Index Codebase
```bash
parseltongue pt01-folder-to-cozodb-streamer ./crates \
  --db rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db \
  --verbose
```

**Expected**: Index all `.rs` files, create entities in CodeGraph

---

### Test 2: PT02-level00 - Export Edges
```bash
parseltongue pt02-level00 \
  --where-clause "ALL" \
  --output demo-walkthroughs/v0.8.6-release-testing/edges.json \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db" \
  --verbose
```

**Expected**: JSON file with dependency edges

---

### Test 3: PT02-level01 - Export Entities (Signatures Only)
```bash
parseltongue pt02-level01 \
  --include-code 0 \
  --where-clause "ALL" \
  --output demo-walkthroughs/v0.8.6-release-testing/entities-l1.json \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db" \
  --verbose
```

**Expected**: JSON with 14 fields per entity (~30K tokens)

---

### Test 4: PT02-level02 - Export with Type System
```bash
parseltongue pt02-level02 \
  --include-code 0 \
  --where-clause "is_public = true" \
  --output demo-walkthroughs/v0.8.6-release-testing/public-api.json \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db" \
  --verbose
```

**Expected**: JSON with 22 fields per entity (type info included)

---

### Test 5: PT03 - Create Temporal Change
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:main:__crates_parseltongue_src_main_rs:146-195" \
  --action edit \
  --future-code "pub fn main() { println!(\"Test edit\"); }" \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db"
```

**Expected**: Entity marked with future_action="Edit"

---

### Test 6: PT04 - Validate Syntax
```bash
parseltongue pt04-syntax-preflight-validator \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db" \
  --verbose
```

**Expected**: Validation passes for edited entity

---

### Test 7: PT05 - Generate Diff
```bash
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output demo-walkthroughs/v0.8.6-release-testing/CodeDiff.json \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db"
```

**Expected**: CodeDiff.json with 1 edit operation

---

### Test 8: PT06 - Reset Database
```bash
parseltongue pt06-cozodb-make-future-code-current \
  --project ./crates \
  --db "rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db"
```

**Expected**: Database reset, entities deleted and re-indexed

---

## Success Criteria

✅ All 8 commands execute without errors
✅ Files are created where expected
✅ JSON output is valid and contains expected data
✅ Database operations persist correctly
✅ Performance is acceptable (<10s total for all operations)

---

## Documentation Updates After Testing

1. ✅ Update README.md with verified commands
2. ✅ Update PRDv2.md with PT02 status
3. ✅ Update PT02PRDv1.md with implementation details
4. ✅ Update Parseltongue-SOP.md with working examples
5. ✅ Create v0.8.6 release notes

