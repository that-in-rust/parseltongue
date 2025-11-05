# Quick Index - visualSummary090

## 📊 Core Analysis Files
- `readme-edges.json` - Level 0: 4,164 dependency edges
- `readme-public.json` - Level 1: 1,318 entities with ISG
- `readme-level2-all.json` - Level 2: Full type system

## 🎯 v0.9.0 EntityClass Files  
- `final-export-v090.json` - Complete export with EntityClass
- `level1-export-v090.json` - Level 1 with EntityClass field
- `all-entities-debug.json` - Debug export (no code)

## 🔍 Query Examples & Tests
- `level1-export-fixed.json` - Fixed export with EntityClass
- `level1-export.json` - Original export attempt
- `test-only-export.json` - TEST-only query (0 results)
- `readme-payment-entities.json` - Payment pattern search
- `readme-all-functions.json` - Function query attempt

## 💾 Database Files
- `parseltongue-v090.db/` - v0.9.0 analysis database (primary)
- `parseltongue.db/` - Previous analysis database

## 🛠️ Temporary Files
- `temp-files/debug_test.rs` - Test detection debug script
- `temp-files/test_debug.rs` - Rust compilation test

## 📖 Documentation
- `README.md` - Complete analysis summary
- `INDEX.md` - This quick reference file

## 🚀 Quick Commands

### ✅ VERIFIED Commands (v0.9.0)
```bash
# Level 0: 4,164 edges (~5K tokens)
parseltongue pt02-level00 --where-clause "ALL" --output edges.json --db "rocksdb:parseltongue-v090.db"

# Level 1: All entities (1,318 entities, ~30K tokens)
parseltongue pt02-level01 --include-code 0 --where-clause "ALL" --output entities.json --db "rocksdb:parseltongue-v090.db"

# Level 1: Functions only (457 functions)
parseltongue pt02-level01 --include-code 0 --where-clause "entity_type = 'function'" --output functions.json --db "rocksdb:parseltongue-v090.db"

# Level 1: EntityClass filtering (v0.9.0 feature)
parseltongue pt02-level01 --include-code 0 --where-clause "entity_class = 'CODE'" --output code.json --db "rocksdb:parseltongue-v090.db"

# Level 2: Full type system (~60K tokens)
parseltongue pt02-level02 --include-code 0 --where-clause "ALL" --output typed.json --db "rocksdb:parseltongue-v090.db"
```

### 📊 Analyze Results
```bash
# View dependency graph
jq '.edges[:5]' readme-edges.json

# Check EntityClass field (v0.9.0)
jq '.entities[0].entity_class' final-export-v090.json

# Count entity types
jq '.entities | group_by(.entity_type) | map({type: .[0].entity_type, count: length})' readme-public.json

# Test query results
jq '.export_metadata' test-only-export.json
```
