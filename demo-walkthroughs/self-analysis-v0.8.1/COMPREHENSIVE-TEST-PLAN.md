# Parseltongue v0.8.1 - COMPREHENSIVE Test Plan

**Date**: 2025-11-01
**Version**: v0.8.1
**Test Subject**: Parseltongue codebase (self-analysis)
**Codebase**: 63 files, 17,721 LOC

---

## DISCOVERED FEATURES (from code review)

### Graph Query API (parseltongue-core):
1. ✅ **`calculate_blast_radius(key, N)`** - Find all entities within N hops
2. ✅ **`get_forward_dependencies(key)`** - What does X depend on? (1-hop outgoing)
3. ✅ **`get_reverse_dependencies(key)`** - Who depends on X? (1-hop incoming)
4. ✅ **`get_transitive_closure(key)`** - All reachable entities (unbounded)

### Tool 2 (pt02) Advanced Features:
- `--query` - Custom CozoDB Datalog queries
- `--include-current-code 0|1` - Toggle code inclusion
- `--max-context-tokens` - Token limit (default: 128k)
- `--relevance-threshold` - Entity filtering (0.0-1.0)
- `--focus-areas` - Comma-separated areas
- `--optimization-goals` - LLM optimization parameters
- `--dry-run` - Test mode
- `--verbose` / `--quiet` - Output control

---

## COMPREHENSIVE TEST MATRIX (35+ Commands)

### **Phase 1: INGEST (Tool 1)** - 5 commands

```bash
# 1.1: Basic index with verbose
parseltongue pt01-folder-to-cozodb-streamer ../../crates \
  --db rocksdb:test.db --verbose

# 1.2: Index single crate for comparison
parseltongue pt01-folder-to-cozodb-streamer ../../crates/parseltongue-core \
  --db rocksdb:test-core-only.db --verbose

# 1.3: Quiet mode
parseltongue pt01-folder-to-cozodb-streamer ../../crates \
  --db rocksdb:test-quiet.db --quiet

# 1.4: Default database (parseltongue.db)
parseltongue pt01-folder-to-cozodb-streamer ../../crates

# 1.5: Current directory (.)
parseltongue pt01-folder-to-cozodb-streamer . --db rocksdb:dot-test.db
```

---

### **Phase 2: GRAPH QUERIES (Core API)** - 6 commands

Using Rust/Python scripts to test core API:

```rust
// 2.1: Forward dependencies
let deps = storage.get_forward_dependencies("rust:fn:insert_entity:...").await?;
// What does insert_entity depend on?

// 2.2: Reverse dependencies
let callers = storage.get_reverse_dependencies("rust:fn:insert_entity:...").await?;
// Who calls insert_entity?

// 2.3: Blast radius (1-hop)
let affected_1 = storage.calculate_blast_radius("rust:fn:insert_entity:...", 1).await?;

// 2.4: Blast radius (3-hop)
let affected_3 = storage.calculate_blast_radius("rust:fn:insert_entity:...", 3).await?;

// 2.5: Blast radius (5-hop) - Performance test
let affected_5 = storage.calculate_blast_radius("rust:fn:insert_entity:...", 5).await?;

// 2.6: Transitive closure (unbounded)
let all_reachable = storage.get_transitive_closure("rust:fn:insert_entity:...").await?;
```

---

### **Phase 3: READ (Tool 2)** - 10 commands

```bash
# 3.1: Default query (signatures only, no current/future code)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db

# 3.2: Include current code (debugging mode)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --include-current-code 1

# 3.3: Custom query - Only changed entities
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL"

# 3.4: Custom query - Only functions
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --query "SELECT * FROM CodeGraph WHERE entity_type = 'Function'"

# 3.5: Max context tokens test
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --max-context-tokens 50000

# 3.6: Relevance threshold
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --relevance-threshold 0.5

# 3.7: Focus areas
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --focus-areas "storage,temporal,entities"

# 3.8: Optimization goals
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --optimization-goals "minimize_size,focus_on_types"

# 3.9: Dry run (no file write)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --dry-run

# 3.10: Verbose output
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:test.db \
  --verbose
```

---

### **Phase 4: EDIT (Tool 3)** - 8 commands

```bash
# 4.1: Simple - Create new test function
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:test_new_feature:parseltongue_core_src_lib_rs:999-1005" \
  --action create \
  --future-code "#[test] fn test_new_feature() { assert!(true); }" \
  --db rocksdb:test.db

# 4.2: Simple - Edit existing function
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:insert_entity:parseltongue_core_src_storage_cozo_client_rs:662-689" \
  --action edit \
  --future-code "pub async fn insert_entity_v2(&self, entity: &CodeEntity) -> Result<()> { ... }" \
  --db rocksdb:test.db

# 4.3: Simple - Delete function
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:test_old_feature:parseltongue_core_tests_tool1_verification_rs:10-20" \
  --action delete \
  --db rocksdb:test.db

# 4.4: Advanced - Raw Datalog (batch update)
parseltongue pt03-llm-to-cozodb-writer \
  --query "?[ISGL1_key, Future_Code, Future_Action] <- [[...]] :put CodeGraph {...}" \
  --db rocksdb:test.db

# 4.5: Error case - Invalid ISGL1 key
parseltongue pt03-llm-to-cozodb-writer \
  --entity "invalid-key-format" \
  --action edit \
  --future-code "test" \
  --db rocksdb:test.db
# (Should fail gracefully)

# 4.6: Error case - Missing future-code for create
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:test:test_rs:1-5" \
  --action create \
  --db rocksdb:test.db
# (Should error: --future-code required)

# 4.7: Multiple edits simulation (3 commands in sequence)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "key1" --action edit --future-code "code1" --db rocksdb:test.db
parseltongue pt03-llm-to-cozodb-writer \
  --entity "key2" --action edit --future-code "code2" --db rocksdb:test.db
parseltongue pt03-llm-to-cozodb-writer \
  --entity "key3" --action delete --db rocksdb:test.db

# 4.8: Temporal state verification (read after write)
# After editing, verify state is (1,1,Edit)
```

---

### **Phase 5: VALIDATE (Tool 4)** - 3 commands

```bash
# 5.1: Basic validation (all changed entities)
parseltongue pt04-syntax-preflight-validator --db rocksdb:test.db

# 5.2: Verbose mode (show all errors)
parseltongue pt04-syntax-preflight-validator --db rocksdb:test.db --verbose

# 5.3: Performance test (measure validation time)
time parseltongue pt04-syntax-preflight-validator --db rocksdb:test.db
```

---

### **Phase 6: DIFF (Tool 5)** - 3 commands

```bash
# 6.1: Generate CodeDiff.json
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output CodeDiff.json \
  --db rocksdb:test.db

# 6.2: Verbose output
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output CodeDiff-verbose.json \
  --db rocksdb:test.db \
  --verbose

# 6.3: Alternative output location
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output ./diffs/changes-$(date +%Y%m%d).json \
  --db rocksdb:test.db
```

---

### **Phase 7: DATABASE INSPECTION** - 3 commands

```bash
# 7.1: Query all entities
echo "SELECT COUNT(*) FROM CodeGraph" | cozo run rocksdb:test.db

# 7.2: Query changed entities
echo "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL" | cozo run rocksdb:test.db

# 7.3: Query dependency edges
echo "SELECT COUNT(*) FROM DependencyEdges" | cozo run rocksdb:test.db
```

---

### **Phase 8: ERROR CASES & EDGE CASES** - 5 commands

```bash
# 8.1: Non-existent database
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:nonexistent.db
# (Should fail gracefully)

# 8.2: Empty directory
mkdir empty && parseltongue pt01-folder-to-cozodb-streamer empty \
  --db rocksdb:empty-test.db

# 8.3: Invalid rocksdb path
parseltongue pt01-folder-to-cozodb-streamer . \
  --db rocksdb:/invalid/path/test.db
# (Should error with helpful message)

# 8.4: Conflicting args (verbose + quiet)
parseltongue pt01-folder-to-cozodb-streamer . \
  --db rocksdb:test.db --verbose --quiet
# (Should error or pick one)

# 8.5: Very large codebase simulation
# (Use entire parseltongue repo including target/)
parseltongue pt01-folder-to-cozodb-streamer ../.. \
  --db rocksdb:large-test.db --verbose
```

---

## EXPECTED RESULTS

### Tool 1 (pt01):
- **Entities**: ~500-1000 (functions, structs, traits, impls)
- **Performance**: <5s for 17k LOC
- **Files processed**: 63

### Tool 2 (pt02):
- **Output**: JSON files in ./contexts/
- **Token count**: ~30k-50k without current_code, ~500k with
- **File size**: ~100KB signatures only, ~2MB with code

### Tool 3 (pt03):
- **Temporal states**: (1,1,Edit), (0,1,Create), (1,0,Delete)
- **Verification**: Query changed entities after write

### Tool 4 (pt04):
- **Validation**: All syntax checks pass (valid Rust)
- **Performance**: <100ms for typical changes

### Tool 5 (pt05):
- **Output**: CodeDiff.json with before/after
- **Operations**: Create (N), Edit (M), Delete (P)

### Graph Queries:
- **Blast radius**: Distance-sorted list of affected entities
- **Forward deps**: Direct dependencies (1-hop out)
- **Reverse deps**: Direct dependents (1-hop in)
- **Transitive**: All reachable (unbounded)

---

## PERFORMANCE CONTRACTS

From PRDv2 and code comments:

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Tool 1 Index | <30s for 50k LOC | time pt01 ... |
| Tool 2 Export | <500ms | time pt02 ... |
| Tool 3 Write | <1ms per entity | time pt03 ... |
| Tool 4 Validate | <20ms per entity | time pt04 ... |
| Tool 5 Diff | <1ms | time pt05 ... |
| Blast radius (5-hop, 10k nodes) | <50ms | Rust benchmark |
| Query (typical) | <500μs | CozoDB metrics |

---

## TEST EXECUTION ORDER

1. **Setup**: Clean state, remove old test DBs
2. **Tool 1**: Index codebase (creates baseline)
3. **Graph Queries**: Test dependency analysis
4. **Tool 2**: Export entities (various filters)
5. **Tool 3**: Make sample changes
6. **Tool 4**: Validate changes
7. **Tool 5**: Generate diffs
8. **Tool 2 again**: Export changed entities
9. **Error cases**: Test failure modes
10. **Cleanup**: Archive results

---

## ARTIFACTS TO CAPTURE

For each command:
- ✅ Command executed
- ✅ Exit code
- ✅ stdout (full output)
- ✅ stderr (if any)
- ✅ Execution time
- ✅ Generated files (JSON, logs)
- ✅ Database state snapshots

File naming convention:
```
step-{N}-{tool}-{description}.log
step-{N}-{tool}-{description}.json
```

---

## READY TO EXECUTE

**Total commands**: 35+
**Estimated time**: 15-20 minutes
**Database**: rocksdb:test.db (primary)

Let's begin! 🚀

