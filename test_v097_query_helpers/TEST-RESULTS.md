# v0.9.7 Query Helpers - Test Results

**Date**: 2025-11-14
**Test Location**: `/test_v097_query_helpers/`

---

## Test Setup

### Test Code (test.rs)
```rust
fn main() {
    process_payment();
}

fn process_payment() {
    validate_payment();
}

fn validate_payment() {
    check_balance();
}

fn check_balance() {
    // Check if user has sufficient balance
}

fn handle_checkout() {
    validate_payment();
}
```

### Call Graph
```
main → process_payment → validate_payment → check_balance
handle_checkout → validate_payment
```

**Expected reverse_deps for validate_payment**: `[process_payment, handle_checkout]`

---

## Test Results

### ✅ WORKING: Ingestion (pt01)

```bash
../target/release/parseltongue pt01-folder-to-cozodb-streamer . --db "rocksdb:test.db"
```

**Result**:
```
Entities created: 5 (CODE only)
Duration: 17.57ms
✓ Indexing completed
```

**Status**: ✅ PASS

---

### ✅ WORKING: Level 0 Export (edges)

```bash
../target/release/parseltongue pt02-level00 --where-clause "ALL" --output edges.json
```

**Result**:
```json
{
  "edges": [
    {"from_key": "rust:fn:handle_checkout:...", "to_key": "rust:fn:validate_payment:...", "edge_type": "Calls"},
    {"from_key": "rust:fn:main:...", "to_key": "rust:fn:process_payment:...", "edge_type": "Calls"},
    {"from_key": "rust:fn:process_payment:...", "to_key": "rust:fn:validate_payment:...", "edge_type": "Calls"},
    {"from_key": "rust:fn:validate_payment:...", "to_key": "rust:fn:check_balance:...", "edge_type": "Calls"}
  ]
}
```

**Status**: ✅ PASS
- All 4 edges correctly identified
- Edge direction correct (caller → callee)

---

### ⚠️  ISSUE FOUND: Level 1 Export (missing reverse_deps)

```bash
../target/release/parseltongue pt02-level01 --include-code 0 --where-clause "ALL" --output entities.json
```

**Result**:
```json
{
  "entities": [
    {
      "entity_name": "validate_payment",
      "isgl1_key": "rust:fn:validate_payment:__test_rs:9-11",
      "interface_signature": "...",
      // ❌ MISSING: "reverse_deps" field
      // ❌ MISSING: "forward_deps" field
    }
  ]
}
```

**Status**: ⚠️  PARTIAL FAIL
- Entities exported successfully
- **Missing**: `reverse_deps` and `forward_deps` fields
- These fields are required by query helpers:
  - `find_reverse_dependencies_by_key()` needs `reverse_deps`
  - `build_call_chain_from_root()` needs edges (works from Level 0)

---

## v0.9.7 Query Helpers Compatibility

### Query Helper Status

| Function | Required Data | Available? | Status |
|----------|--------------|------------|--------|
| `find_reverse_dependencies_by_key()` | `entities` with `reverse_deps` | ❌ No | ⚠️  BLOCKED |
| `build_call_chain_from_root()` | `edges` array | ✅ Yes (Level 0) | ✅ WORKS |
| `filter_edges_by_type_only()` | `edges` array with `edge_type` | ✅ Yes | ✅ WORKS |
| `collect_entities_in_file_path()` | `entities` with `file_path` | ✅ Yes | ✅ WORKS |

### Analysis

**Working Queries** (3/4):
1. ✅ **`build_call_chain_from_root()`**: Can use `edges.json` from Level 0
2. ✅ **`filter_edges_by_type_only()`**: Can use `edges.json` from Level 0
3. ✅ **`collect_entities_in_file_path()`**: Can use `entities.json` from Level 1

**Blocked Query** (1/4):
1. ⚠️ **`find_reverse_dependencies_by_key()`**: Needs `reverse_deps` field in `entities.json`

---

## Workaround for reverse_deps

**Manual Calculation** from `edges.json`:

```javascript
// Given target_key = "rust:fn:validate_payment:..."
// Find all edges where to_key = target_key
const reverse_deps = edges.filter(e => e.to_key === target_key)
                          .map(e => e.from_key);

// Result: ["rust:fn:process_payment:...", "rust:fn:handle_checkout:..."]
```

This is exactly what `find_reverse_dependencies_by_key()` should do, but it expects the data to be pre-computed in the JSON.

---

## Root Cause

**Expected JSON Structure** (from contract tests):
```json
{
  "entities": [
    {
      "isgl1_key": "rust:fn:validate_payment:src_payment_rs:89-112",
      "name": "validate_payment",
      "reverse_deps": [
        "rust:fn:process_payment:src_payment_rs:145-167",
        "rust:fn:handle_checkout:src_checkout_rs:200-245"
      ]
    }
  ]
}
```

**Actual JSON from pt02-level01**:
```json
{
  "entities": [
    {
      "isgl1_key": "rust:fn:validate_payment:__test_rs:9-11",
      "entity_name": "validate_payment",
      // Missing reverse_deps field entirely
    }
  ]
}
```

**Issue**: pt02-level01 doesn't populate `reverse_deps` or `forward_deps` in the export.

---

## Recommendations

### Short Term (H2 Sprint)

1. **Update pt02-level01** to include `reverse_deps` and `forward_deps` fields
   - Query CozoDB for edges where `to_key = entity.isgl1_key` (reverse_deps)
   - Query CozoDB for edges where `from_key = entity.isgl1_key` (forward_deps)
   - Add to JSON export

2. **Alternative**: Update query helpers to work with combined JSON
   - Accept `{entities: [...], edges: [...]}` format
   - Calculate reverse_deps dynamically from edges array
   - Tradeoff: Slower (<100ms guarantee harder to meet)

### Long Term (H3)

- **Unified Export Format**: Single JSON with entities + edges + metadata
- **Pre-computed fields**: reverse_deps, forward_deps, cluster_id (from pt08)
- **Hierarchical structure**: Clusters → Entities → Code

---

## Test Conclusion

**v0.9.7 Query Helpers**: 75% functional (3/4 work)

**Blocker**: Missing `reverse_deps` field in Level 1 exports

**Impact**: High (blast radius analysis is a killer feature)

**Effort to Fix**: Low (2-4 hours to update pt02-level01)

**Priority**: P0 (should be fixed before v0.9.7 release)

---

## Files Generated

```
test_v097_query_helpers/
├── test.rs           # Test code (5 functions)
├── test.db/          # RocksDB database
├── edges.json        # Level 0 export (✅ works)
├── edges_test.json   # Test edges
├── entities.json     # Level 1 export (⚠️ missing reverse_deps)
├── entities_test.json
└── TEST-RESULTS.md   # This file
```

---

**Next Steps**:
1. Fix pt02-level01 to populate reverse_deps/forward_deps
2. Re-run tests to verify all 4 query helpers work
3. Update contract tests to validate against real pt02 output format
