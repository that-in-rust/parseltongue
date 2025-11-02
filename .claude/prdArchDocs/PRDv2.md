# Parseltongue: Product Requirements Document v2.0

**Document Type**: Workflow-Ordered CLI Toolkit Specification
**Last Updated**: 2025-11-02
**Status**: v0.8.6 Released - All 8 Commands Working
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

### **pt02: Export Database → JSON (Progressive Disclosure Design)**

**Status (v0.8.6)**: ✅ Fully working with real CozoDB, 31/31 PT02 tests GREEN, all 8 commands verified on parseltongue repo

PT02 provides 3 export levels integrated into the main `parseltongue` binary:

#### **pt02-level00: Pure Edge List (MINIMAL)**

```bash
pt02-level00 --where "ALL" --output edges.json [--db parseltongue.db] [--verbose]
```

**What it does:**
- Exports dependency edges only (from_key, to_key, edge_type)
- Token estimate: ~2-5K tokens for ~2000 edges
- Use case: Pure dependency analysis, graph visualization, architectural overview

**Examples:**
```bash
# Export all edges
pt02-level00 --where "ALL" --output edges.json

# Filter by edge type (Datalog syntax: comma for AND)
pt02-level00 --where "edge_type = 'depends_on', from_key ~ 'rust:fn'" --output deps.json
```

---

#### **pt02-level01: Entity + ISG + Temporal (RECOMMENDED)**

```bash
pt02-level01 --include-code <0|1> --where "FILTER" --output entities.json [--db parseltongue.db] [--verbose]
```

**What it does:**
- Exports entities with Interface Signature Graph (ISG) + temporal state
- Token estimate: ~30K tokens (signatures only), ~500-700K (with code)
- Use case: Code understanding, refactoring planning, temporal state tracking

**Mandatory Flags:**
- `--include-code 0` = Signatures only (CHEAP - ~30K tokens)
- `--include-code 1` = Full code (EXPENSIVE - 100× more tokens)
- `--where` = Datalog WHERE clause (use `"ALL"` for everything)

**Examples:**
```bash
# Export all entities (signatures only - CHEAP)
pt02-level01 --include-code 0 --where "ALL" --output entities.json

# Export public API surface
pt02-level01 --include-code 0 --where "is_public = true, entity_type = 'fn'" --output api.json

# Export entities with planned changes (temporal)
pt02-level01 --include-code 0 --where "future_action != null" --output changes.json

# Export with full code (EXPENSIVE - use sparingly!)
pt02-level01 --include-code 1 --where "ALL" --output entities_with_code.json
```

**Output Fields (14 total):**
- `isgl1_key`, `forward_deps`, `reverse_deps`
- `current_ind`, `future_ind` (temporal indicators)
- `entity_name`, `entity_type`, `file_path`, `line_number`
- `interface_signature`, `doc_comment`
- `current_code` (if --include-code 1)
- `future_action`, `future_code` (if temporal state set)

---

#### **pt02-level02: Entity + ISG + Temporal + Type System (ADVANCED)**

```bash
pt02-level02 --include-code <0|1> --where "FILTER" --output typed_entities.json [--db parseltongue.db] [--verbose]
```

**What it does:**
- Exports entities with full type system information
- Token estimate: ~60K tokens (signatures only), ~500-700K (with code)
- Use case: Type-safe refactoring, API compatibility analysis, safety audits

**Mandatory Flags:**
- `--include-code 0` = Signatures only (CHEAP - ~60K tokens)
- `--include-code 1` = Full code (EXPENSIVE - 100× more tokens)
- `--where` = Datalog WHERE clause

**Examples:**
```bash
# Export all entities with type information (signatures only)
pt02-level02 --include-code 0 --where "ALL" --output typed_entities.json

# Find all async functions
pt02-level02 --include-code 0 --where "is_async = true" --output async_fns.json

# Find unsafe code
pt02-level02 --include-code 0 --where "is_unsafe = true" --output unsafe_code.json

# Export public API with types
pt02-level02 --include-code 0 --where "is_public = true" --output public_api.json
```

**Output Fields (22 total):**
- All Level 1 fields (14) +
- `return_type`, `param_types`, `param_names`
- `is_public`, `is_async`, `is_unsafe`
- `generic_constraints`, `trait_impls`

---

#### **Datalog WHERE Clause Syntax**

**CRITICAL:** Use Datalog syntax, NOT SQL!

| SQL (WRONG) | Datalog (CORRECT) |
|-------------|-------------------|
| `x = 5 AND y = 10` | `x = 5, y = 10` |
| `x = 5 OR y = 10` | `x = 5; y = 10` |
| `x == 5` | `x = 5` |
| `x LIKE '%pattern%'` | `x ~ 'pattern'` |

**Common Filters:**
```bash
# All entities
--where "ALL"

# Public functions only
--where "is_public = true, entity_type = 'fn'"

# Async functions
--where "is_async = true"

# Entities with planned changes (temporal)
--where "future_action != null"

# Pattern matching (entity name contains "test")
--where "entity_name ~ 'test'"

# Complex (public OR async functions)
--where "(is_public = true; is_async = true), entity_type = 'fn'"
```

---

#### **Progressive Disclosure Model**

**Level 0 ⊂ Level 1 ⊂ Level 2** (subset relationship)

```
Level 0: edges only (3 fields) → ~2-5K tokens
  └─> Level 1: + entities (14 fields) → ~30K tokens
        └─> Level 2: + type system (22 fields) → ~60K tokens
```

**When to use each level:**
- **Level 0**: Dependency-only analysis, graph visualization, "what depends on what?"
- **Level 1**: Code understanding, refactoring planning, temporal state tracking
- **Level 2**: Type-safe refactoring, API compatibility, safety audits (async/unsafe)

**Token Cost Management:**
- Signatures only (`--include-code 0`): **CHEAP** (2-60K tokens)
- With code (`--include-code 1`): **EXPENSIVE** (500-700K tokens, 100× more)
- **Recommendation**: Start with signatures, add code only when needed

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
│ Step 2: Read → JSON (for LLM) - Progressive Disclosure          │
│                                                                 │
│ LEVEL 0 (Minimal - ~2-5K tokens):                               │
│ $ pt02-level00 --where "ALL" --output edges.json               │
│ → Dependency graph edges only                                   │
│                                                                 │
│ LEVEL 1 (Recommended - ~30K tokens):                            │
│ $ pt02-level01 --include-code 0 --where "ALL" --output ent.json│
│ → Entities + ISG + temporal (signatures only)                   │
│                                                                 │
│ LEVEL 2 (Advanced - ~60K tokens):                               │
│ $ pt02-level02 --include-code 0 --where "ALL" --output typed.js│
│ → Entities + ISG + temporal + type system                       │
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



## VERIFIED WORKING (v0.8.6 - Comprehensive Testing)

**Last Updated**: 2025-11-02
**Test Artifacts**: `/demo-walkthroughs/v0.8.6-release-testing/`
**Full Test Report**: [TEST-RESULTS.md](../../demo-walkthroughs/v0.8.6-release-testing/TEST-RESULTS.md)

This section documents what has been **empirically verified to work** through comprehensive testing on the parseltongue codebase itself (self-analysis).

### Test Environment
- **Codebase**: Parseltongue v0.8.6 (765 entities indexed)
- **Database**: `rocksdb:demo-walkthroughs/v0.8.6-release-testing/test.db`
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

