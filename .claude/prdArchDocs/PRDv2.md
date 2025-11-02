# Parseltongue: Product Requirements Document v2.0

**Document Type**: Workflow-Ordered CLI Toolkit Specification
**Last Updated**: 2025-11-01
**Status**: Architecture Redesign - Commands First
**Philosophy**: Commands are the guiding light - everything else supports them

**Implementation Reference**: See `that-in-rust-parseltongue-8a5edab282632443 (8).txt` for detailed implementation context

---

## COMMAND REFERENCE - Not yet validated

**These 6 commands define the entire Parseltongue workflow.**

### **pt01: Ingest Codebase → Database**

```bash
parseltongue pt01-folder-to-cozodb-streamer <directory> \
  --db rocksdb:parseltongue.db \
  [--verbose] \
  [--quiet]
```

**What it does:**
- Parses code files with tree-sitter
- Generates ISGL1 keys: `{lang}:{type}:{name}:{path}:{lines}`
- Stores entities in CodeGraph table
- Sets initial state: `(current_ind=1, future_ind=1, future_action=None)`

**Example:**
```bash
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db
```

---


### **pt03: Edit Database (LLM Writes Changes)**

```bash
# Simple Interface (80% of use cases)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "<ISGL1_KEY>" \
  --action <create|edit|delete> \
  --future-code "<CODE>" \
  --db rocksdb:parseltongue.db

# Advanced Interface (Power Users)
parseltongue pt03-llm-to-cozodb-writer \
  --query "<DATALOG_QUERY>" \
  --db rocksdb:parseltongue.db
```

**What it does:**
- Updates temporal state in CodeGraph
- Sets `future_code`, `future_ind`, `future_action`
- Supports CREATE/EDIT/DELETE operations
- Advanced mode: Execute raw Datalog

**Example 1: EDIT**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() { println!(\"Fixed!\"); }" \
  --db rocksdb:analysis.db
```

**Example 2: CREATE**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_lib_rs-new_function-fn-abc12345" \
  --action create \
  --future-code "pub fn new_function(x: i32) -> i32 { x * 2 }" \
  --db rocksdb:analysis.db
```

**Example 3: DELETE**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:old_func:lib_rs:20-25" \
  --action delete \
  --db rocksdb:analysis.db
```

---

### **pt04: Validate Syntax (Pre-Flight Check)**

```bash
parseltongue pt04-syntax-preflight-validator \
  --db rocksdb:parseltongue.db \
  [--verbose]
```

**What it does:**
- Reads all entities with `future_action != None`
- Validates `future_code` syntax using tree-sitter
- Reports errors (file, line, issue)
- Exit code 0 = valid, non-zero = errors

**Example:**
```bash
parseltongue pt04-syntax-preflight-validator --db rocksdb:analysis.db
```

---

### **pt05: Generate Diff → Code (JSON Output)**

```bash
parseltongue pt05-llm-cozodb-to-diff-writer \
  --db rocksdb:parseltongue.db \
  --output CodeDiff.json \
  [--verbose]
```

**What it does:**
- Reads all entities with `future_action != None`
- Generates CodeDiff.json with operation-specific fields
- Parses line ranges from ISGL1 keys
- Desanitizes file paths

---

### **pt06: Reset Database (Make Future → Current)**

```bash
parseltongue pt06-cozodb-make-future-code-current \
  --project <directory> \
  --db rocksdb:parseltongue.db
```

**What it does:**
- Deletes ALL entities from CodeGraph (NO backups - S01 principle)
- Recreates schema
- Re-runs pt01 to re-index current state
- Resets temporal indicators

---

## WORKFLOW: THE 6-STEP PIPELINE

```
┌─────────────────────────────────────────────────────────────────┐
│ Step 1: Ingest Codebase                                         │
│ $ parseltongue pt01-folder-to-cozodb-streamer ./src --db ...   │
│ → Creates: 1,247 entities with state (1,1,None)                 │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2: Read → JSON (for LLM)                                   │
│ $ parseltongue pt02-llm-cozodb-to-context-writer \             │
│     --output context.json --include-current-code 0              │
│ → Generates: context.json (37.5k tokens, no code)               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
         (LLM analyzes context.json, decides on changes)
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 3: Edit Database (LLM writes changes)                      │
│ $ parseltongue pt03-llm-to-cozodb-writer \                     │
│     --entity "rust:fn:hello:lib_rs:4-6" \                       │
│     --action edit \                                              │
│     --future-code "pub fn hello() { println!(\"Fixed!\"); }"    │
│ → Updates: 1 entity to state (1,1,Edit)                         │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 4: Validate Syntax                                         │
│ $ parseltongue pt04-syntax-preflight-validator --db ...        │
│ → Checks: 1 entity with future_code                            │
│ → Result: ✓ All syntax valid (exit code 0)                     │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 5: Generate Diff                                           │
│ $ parseltongue pt05-llm-cozodb-to-diff-writer \                │
│     --output CodeDiff.json                                      │
│ → Generates: CodeDiff.json with 1 EDIT operation               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
        (Orchestrator applies CodeDiff.json to files)
        (Orchestrator runs: cargo build && cargo test)
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 6: Reset State                                             │
│ $ parseltongue pt06-cozodb-make-future-code-current \          │
│     --project ./src --db ...                                    │
│ → Deletes: 1,247 entities                                      │
│ → Re-indexes: 1,247 entities with fresh state (1,1,None)       │
└─────────────────────────────────────────────────────────────────┘
```

---


## QUERY PATTERNS: THE COMPLETE REFERENCE

### **Pattern A: CREATE - New Entity (Hash-Based ISGL1)**

**Simple Interface:**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_lib_rs-new_function-fn-abc12345" \
  --action create \
  --future-code "pub fn new_function(x: i32) -> i32 { x * 2 }" \
  --db demo.db
```

**Temporal State:** `(current_ind=0, future_ind=1, future_action='Create')`

**ISGL1 Key Format:** `{sanitized_path}-{name}-{type}-{hash8}`

---

### **Pattern B: EDIT - Modify Existing Entity**

**Simple Interface:**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() { println!(\"Fixed!\"); }" \
  --db demo.db
```

**Temporal State:** `(current_ind=1, future_ind=1, future_action='Edit')`

**ISGL1 Key Format:** `{lang}:{type}:{name}:{sanitized_path}:{start}-{end}`

---

### **Pattern C: DELETE - Remove Existing Entity**

**Simple Interface:**
```bash
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:old_function:lib_rs:20-25" \
  --action delete \
  --db demo.db
```

**Temporal State:** `(current_ind=1, future_ind=0, future_action='Delete')`

---



## ANECDOTALLY WORKS (v0.8.1 - Live Testing Results)

**Last Updated**: 2025-11-01
**Test Artifacts**: `/demo-walkthroughs/self-analysis-v0.8.1/`

This section documents what has been **empirically verified to work** through live testing on the parseltongue codebase itself (recursive self-analysis).

### Test Environment
- **Codebase**: Parseltongue v0.8.1 (63 Rust files, 17,721 LOC)
- **Database**: `rocksdb:test.db`
- **Binary**: `/target/release/parseltongue`
- **Test Date**: 2025-11-01

---

### ✅ **pt01-folder-to-cozodb-streamer** - VERIFIED WORKING

```bash
parseltongue pt01-folder-to-cozodb-streamer ./crates \
  --db rocksdb:test.db \
  --verbose
```

**Verified Results:**
- ✅ **Files processed**: 63 (all .rs files in crates/)
- ✅ **Entities created**: 661 (functions, structs, traits, impls, modules)
- ✅ **Performance**: 106.9ms for 17,721 LOC
  - **Target**: <30s for 50k LOC
  - **Actual**: **280x faster than target** (extrapolated: 17k LOC in 106ms → 50k LOC in ~312ms)
- ✅ **Errors**: 14 (non-Rust files like .toml, expected behavior)
- ✅ **Database**: RocksDB created successfully, ~4KB compressed
- ✅ **Flags tested**: `--verbose`, `--quiet`, `--db`

**Status**: ✅ **PRODUCTION READY**

---

# amuldotexe's Implementation Priorities - Immediate next steps

## Command List new

Current approach of
`/crates/pt02-llm-cozodb-to-context-writer/src/main.rs:76-159`
- is Dual interface: Simple (--include-current-code + --where) + Advanced (--query)

SimpleQuery means
- you can pick if --include-current-code + --where flags where both are mandatory even default values have to be entered because LLMs need to know they want everything explicit

AdvancedQuery means
- you use --query and it overrides both --where --include-current-code flags even if you mention them - the datalog query is the only thing that matters 

NewSimpleQuery
1. pt02-llm-cozodb-to-context-writer-isg-only-essential-fields
2. pt02-llm-cozodb-to-context-writer-isg-and-code-fields



## RAW variable info 

1. Dependency Graphs - ✅ EXTRACTED, ✅ STORED, ❌ **NOT EXPOSED IN PT02 CLI**

Status: PT01 extracts during parsing, stores in DependencyEdges relation, 4 graph operations tested
Gap: PT02 CLI has NO way to export dependency-only JSON
Action Required: Add `--export-dependencies` flag to PT02


| Variable | Type | Size (bytes) | Description | Nullable | Derivable | Example |
|----------|------|--------------|-------------|----------|-----------|---------|
| from_key | String | ~60 | Source entity ISGL1 key | No | No | `rust:fn:main:src_main_rs:1-10` |
| to_key | String | ~60 | Target entity ISGL1 key | No | No | `rust:fn:helper:src_lib_rs:20-30` |
| edge_type | Enum | ~8 | Relationship type: Calls, Uses, Implements | No | No | `Calls` |
| source_location | String | ~20 | Where relationship occurs in source | Yes | No | `src/main.rs:5` |

**Total per edge**: ~148 bytes
**Criticality**: **HIGH** - Core graph structure, answers "what depends on what?"

Why HIGH Criticality - 
- Blast radius calculation: Find all affected entities when one changes
- Dependency traversal: Navigate call graphs, usage graphs
- Test impact analysis: Which tests need to run when code changes
- Refactoring safety: Understand what breaks when modifying an entity
- Architecture understanding: See module coupling and cohesion



| Variable | Status | Location | Tests |
|----------|--------|----------|-------|
| from_key, to_key, edge_type, source_location | ✅ Extracted & Stored | PT01: `crates/pt01-folder-to-cozodb-streamer/src/isgl1_generator.rs:540-612` | 3 tests |
| Blast radius query | ✅ Implemented | Core: `crates/parseltongue-core/src/storage/cozo_client.rs:305-372` | 4 tests |
| Forward deps query | ✅ Implemented | Core: `crates/parseltongue-core/src/storage/cozo_client.rs:420-443` | 5 tests |
| Reverse deps query | ✅ Implemented | Core: `crates/parseltongue-core/src/storage/cozo_client.rs:491-514` | 4 tests |
| Transitive closure query | ✅ Implemented | Core: `crates/parseltongue-core/src/storage/cozo_client.rs:588-625` | 4 tests |
| PT02 CLI exposure | ❌ **MISSING** | - | - |

Expected Output (50-80KB for 590 entities, 8-13x smaller than current ISG exports):
```json
{
  "nodes": [{"key": "rust:fn:main:...", "name": "main", "type": "fn", "entity_class": "CODE"}],
  "edges": [{"from": "rust:fn:main:...", "to": "rust:fn:helper:...", "type": "Calls", "location": "src/main.rs:5"}]
}
```

---

2. Temporal State - ✅ EXTRACTED, ✅ STORED, ✅ **QUERYABLE** (via --query flag)

Status: Fully working, PT01 initializes, PT03 updates, PT02 can export via --query
Variables: current_ind, future_ind, future_action (12 bytes per entity)
Criticality: **HIGH** - Essential for change planning, blast radius of modifications

| current_ind | future_ind | future_action | Meaning |
|-------------|------------|---------------|---------|
| true | true | None | Unchanged entity |
| true | true | Edit | Entity will be modified |
| true | false | Delete | Entity will be removed |
| false | true | Create | Entity will be added |

---

3. TDD Classification - ✅ EXTRACTED, ⚠️ **BLOATED** (6/7 fields are defaults)

ONLY FIELD WE NEED 

| Variable | Criticality | Default Value | Usefulness |
|----------|-------------|---------------|------------|
| entity_class | HIGH | CodeImplementation | Essential for test impact analysis |


---

4. LSP Essential Data where it exists so we can save precious tokens

We need following in essential, ideally do not include these if they are null

 
- has_tests
- isg_neighbors
- generic_params
- where_clauses
- return_type
- param_types
- trait_bounds
- lifetime_params
- impl_trait_for
- associated_types
- type_aliases
- derived_traits
- const_generics
- forward_deps
- reverse_deps
- blast_radius_count
- blast_radius_files
- module_dependencies
- transitive_deps_forward
- transitive_deps_reverse
- import_statements
- macro_invocations
- trait_object_usage

total_references	textDocument/references	 Yes	 Yes	HIGH	Count references
usage_locations	textDocument/references	 Yes	 Yes	MEDIUM	Location mapping
dependents (ISGL1 keys)	textDocument/references

