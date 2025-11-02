# All Extracted Metadata Dictionary

**Version**: v0.8.3
**Date**: 2025-11-02
**Purpose**: Comprehensive catalog of ALL variables extracted and queryable via PT02

---


# amuldotexe's Implementation Priorities

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

Status: All 7 fields extracted, but only `entity_class` has value
Recommendation: Default export should include ONLY `entity_class`, rest via --query

| Variable | Criticality | Default Value | Usefulness |
|----------|-------------|---------------|------------|
| entity_class | HIGH | CodeImplementation | Essential for test impact analysis |

Current problem: Exporting all 7 fields wastes ~27K tokens (23.7% of "minimal" export)

---

4. LSP Essential Data where it exists so we can save precious tokens



# Longer Documentation for Reference ONLY

## Priority Actions for v0.8.3

### Priority 1: Expose Dependency Graph Queries in PT02 CLI
**Effort**: 4-6 hours
**Impact**: 8-13x token reduction vs current exports
**Files**: `crates/pt02-llm-cozodb-to-context-writer/src/{cli.rs, query_builder.rs, main.rs}`
**Action**:
1. Add `--export-mode dependencies` flag (default to dependency graph export)
2. Create graph-optimized JSON output format
3. Wire up existing core methods to PT02 CLI

### Priority 2: Simplify TDD Classification Default Export
**Effort**: 2 hours
**Impact**: 23.7% size reduction of minimal exports
**Action**: Only export `entity_class` by default, rest via --export-level standard/full

### Priority 3: Add Export Level Presets
**Effort**: 4 hours
**Impact**: Clear cost model for users (essential/standard/full/bulk)
**Action**: Implement variable-level criticality filtering per `/ALL_EXTRACTED_METADATA_DICTIONARY.md`

---

## What Does NOT Need Reimplementation

- ❌ Dependency extraction (PT01 already does this)
- ❌ Dependency storage (DependencyEdges relation exists)
- ❌ Graph queries (blast radius, forward/reverse deps, transitive closure all tested)
- ❌ Temporal state tracking (fully working)

## What DOES Need Implementation

- ✅ PT02 CLI flag for dependency-only export
- ✅ Graph-optimized JSON output format (nodes + edges)
- ✅ Export level presets (essential, standard, full, bulk)
- ✅ Documentation and examples for dependency-first workflows



## Table of Contents

1. [Variable Criticality Classification](#variable-criticality-classification)
2. [Dependency Graph Variables](#1-dependency-graph-variables-high-criticality)
3. [Core Identity Variables](#2-core-identity-variables-high-criticality)
4. [Temporal State Variables](#3-temporal-state-variables-high-criticality)
5. [Interface Signature Variables](#4-interface-signature-variables-mixed-criticality)
6. [TDD Classification Variables](#5-tdd-classification-variables-medium-criticality)
7. [LSP Type Information Variables](#6-lsp-type-information-variables-medium-criticality)
8. [LSP Usage Analysis Variables](#7-lsp-usage-analysis-variables-high-criticality)
9. [LSP Semantic Tokens Variables](#8-lsp-semantic-tokens-variables-low-criticality)
10. [Language-Specific Variables](#9-language-specific-variables-low-criticality)
11. [Entity Metadata Variables](#10-entity-metadata-variables-low-criticality)
12. [Export Presets](#export-presets-by-criticality)

---

## Variable Criticality Classification

### Criticality Levels

| Level | Name | Description | Typical Size | Use Case |
|-------|------|-------------|--------------|----------|
| **HIGH** | Essential | Core relationships and identity. Required for 95% of queries. | ~20-30KB for 590 entities | Code navigation, blast radius, dependency analysis |
| **MEDIUM** | Standard | Type information and analysis metadata. Useful for deeper understanding. | ~80-100KB for 590 entities | Refactoring, test planning, API analysis |
| **LOW** | Full | Complete metadata including LSP tokens, timestamps, language details. | ~170-200KB for 590 entities | IDE features, comprehensive analysis, debugging |
| **BULK** | With Code | Includes full source code (`current_code`, `future_code`). | ~400-450KB for 590 entities | Code generation, detailed debugging |


---

## 1. Dependency Graph Variables (HIGH Criticality)

**Source**: `DependencyEdges` CozoDB relation
**Extracted by**: PT01 during tree-sitter parsing
**Storage**: Separate relation with composite key `(from_key, to_key, edge_type)`

### Variables

| Variable | Type | Size (bytes) | Description | Nullable | Derivable | Example |
|----------|------|--------------|-------------|----------|-----------|---------|
| **from_key** | String | ~60 | Source entity ISGL1 key | No | No | `rust:fn:main:src_main_rs:1-10` |
| **to_key** | String | ~60 | Target entity ISGL1 key | No | No | `rust:fn:helper:src_lib_rs:20-30` |
| **edge_type** | Enum | ~8 | Relationship type: Calls, Uses, Implements | No | No | `Calls` |
| **source_location** | String | ~20 | Where relationship occurs in source | Yes | No | `src/main.rs:5` |

**Total per edge**: ~148 bytes
**Criticality**: **HIGH** - Core graph structure, answers "what depends on what?"

### Why HIGH Criticality

- **Blast radius calculation**: Find all affected entities when one changes
- **Dependency traversal**: Navigate call graphs, usage graphs
- **Test impact analysis**: Which tests need to run when code changes
- **Refactoring safety**: Understand what breaks when modifying an entity
- **Architecture understanding**: See module coupling and cohesion

### Query Examples

```datalog
# Find all dependencies of main function
?[from_key, to_key, edge_type] :=
  *DependencyEdges{from_key, to_key, edge_type},
  from_key ~ "main"

# Find all dependents (who calls this function?)
?[from_key, to_key, edge_type] :=
  *DependencyEdges{from_key, to_key, edge_type},
  to_key ~ "helper"

# Count outgoing dependencies per entity
?[from_key, dep_count] :=
  *DependencyEdges{from_key, to_key, edge_type},
  dep_count = count(to_key)
```

---

## 2. Core Identity Variables (HIGH Criticality)

**Source**: `CodeGraph` CozoDB relation (top-level CodeEntity fields)
**Extracted by**: PT01 during parsing
**Storage**: Primary key = `ISGL1_key`

### Variables

| Variable | Type | Size (bytes) | Description | Nullable | Derivable | Example |
|----------|------|--------------|-------------|----------|-----------|---------|
| **isgl1_key** | String | ~60 | Unique entity identifier with structure: `{lang}:{type}:{name}:{path}:{lines}` | No | No | `rust:fn:calculate_total:src_lib_rs:42-58` |
| **name** | String | ~20 | Entity name (function, struct, etc.) | No | Yes (from isgl1_key) | `calculate_total` |
| **entity_type** | String | ~10 | Type: Function, Struct, Enum, Trait, etc. | No | Yes (from isgl1_key) | `Function` |
| **language** | String | ~8 | Programming language | No | Yes (from isgl1_key) | `rust` |
| **file_path** | String | ~40 | Absolute file path | No | Yes (from isgl1_key) | `src/lib.rs` |
| **current_code** | String (BULK) | ~500-5000 | Full source code of entity (current state) | Yes | No | `fn calculate_total() { ... }` |
| **future_code** | String (BULK) | ~500-5000 | Full source code of entity (future state after changes) | Yes | No | `fn calculate_total() { ... }` |

**Total per entity (without code)**: ~138 bytes
**Total per entity (with code)**: ~1000-10000 bytes
**Criticality**:
- **HIGH**: isgl1_key, name, entity_type, language, file_path
- **BULK**: current_code, future_code

### ISGL1 Key Structure (Parseable)

The ISGL1 key encodes multiple variables in one string:

```
rust:fn:calculate_total:src_lib_rs:42-58
│    │   │              │            │
│    │   │              │            └─ line_range (start-end)
│    │   │              └─ file_path (encoded: / → _)
│    │   └─ name
│    └─ entity_type (abbreviated: fn, struct, enum, trait, mod, impl)
└─ language
```

**Derivable fields**: language, entity_type, name, file_path, line_range
**Why keep separate**: Performance (no parsing), clarity

### Why HIGH Criticality

- **Unique identification**: Every query needs isgl1_key
- **Human readability**: name for display
- **Filtering**: Filter by entity_type, language
- **Navigation**: Jump to file_path and line_range

---

## 3. Temporal State Variables (HIGH Criticality)

**Source**: `CodeGraph.temporal_state` (TemporalState struct)
**Extracted by**: PT01 (initial), PT03 (updates during planning)
**Size**: ~12 bytes per entity

### Variables

| Variable | Type | Size (bytes) | Description | Nullable | Derivable | Example |
|----------|------|--------------|-------------|----------|-----------|---------|
| **current_ind** | Boolean | ~1 | Entity exists in current state | No | No | `true` |
| **future_ind** | Boolean | ~1 | Entity will exist in future state | No | No | `true` |
| **future_action** | Enum | ~10 | Planned action: Create, Edit, Delete | Yes | No | `Edit` |

**Total per entity**: ~12 bytes
**Criticality**: **HIGH** - Essential for change planning and temporal queries

### Temporal State Combinations

| current_ind | future_ind | future_action | Meaning |
|-------------|------------|---------------|---------|
| true | true | None | Unchanged entity |
| true | true | Edit | Entity will be modified |
| true | false | Delete | Entity will be removed |
| false | true | Create | Entity will be added |
| true | false | None | ❌ Invalid state |
| false | false | * | ❌ Invalid state |

### Why HIGH Criticality

- **Change detection**: Find all modified entities
- **Test planning**: Which entities need new/updated tests
- **Blast radius**: Only changed entities trigger downstream impacts
- **Rollback**: Distinguish current vs future state
- **Small size**: Only 12 bytes, huge value

### Query Examples

```datalog
# Find all changed entities
?[isgl1_key, future_action] :=
  *CodeGraph{ISGL1_key: isgl1_key, Future_Action: action},
  action != null

# Find entities to be deleted
?[isgl1_key] :=
  *CodeGraph{ISGL1_key: isgl1_key, current_ind: cur, future_ind: fut},
  cur == true,
  fut == false

# Find new entities to be created
?[isgl1_key] :=
  *CodeGraph{ISGL1_key: isgl1_key, current_ind: cur, future_ind: fut},
  cur == false,
  fut == true
```

---

## 4. Interface Signature Variables (MIXED Criticality)

**Source**: `CodeGraph.interface_signature` (InterfaceSignature struct)
**Extracted by**: PT01 during tree-sitter parsing

### Variables

| Variable | Type | Size (bytes) | Criticality | Description | Nullable | Derivable | Example |
|----------|------|--------------|-------------|-------------|----------|-----------|---------|
| **visibility** | Enum | ~8 | **HIGH** | Public, Private, Protected, Crate, Module | No | No | `Public` |
| **line_range.start** | u32 | ~4 | **HIGH** | Start line (1-based) | No | Yes (from isgl1_key) | `42` |
| **line_range.end** | u32 | ~4 | **HIGH** | End line (1-based, inclusive) | No | Yes (from isgl1_key) | `58` |
| **module_path** | Vec<String> | ~30 | **MEDIUM** | Module hierarchy | No | Partial (from file_path) | `["parseltongue", "core", "entities"]` |
| **documentation** | String | ~200 | **MEDIUM** | Doc comments (/// or /** */) | Yes | No | `"Calculates total from items"` |

**Total per entity**: ~246 bytes
**Overall Criticality**: **MEDIUM** (visibility is HIGH, rest is MEDIUM-LOW)

### Why Mixed Criticality

**HIGH (visibility, line_range)**:
- **API surface analysis**: Find all public functions/structs
- **Encapsulation checks**: Detect public items calling private helpers
- **IDE navigation**: Jump to exact line in editor

**MEDIUM (module_path, documentation)**:
- **Code organization**: Understand module structure
- **Documentation generation**: Extract doc comments
- **Context for LLMs**: Doc strings help understand intent

### Query Examples

```datalog
# Find all public APIs
?[isgl1_key, name, visibility] :=
  *CodeGraph{ISGL1_key: isgl1_key, interface_signature: sig},
  sig.visibility == "Public",
  sig.name: name

# Find large functions (>100 lines)
?[isgl1_key, span] :=
  *CodeGraph{ISGL1_key: isgl1_key, interface_signature: sig},
  sig.line_range.start: start,
  sig.line_range.end: end,
  span = end - start,
  span > 100
```

---

## 5. TDD Classification Variables (MEDIUM Criticality)

**Source**: `CodeGraph.TDD_Classification` (TddClassification struct)
**Extracted by**: PT01 (initial defaults), PT03 (analysis updates)
**Size**: ~40 bytes per entity

### Variables

| Variable | Type | Size (bytes) | Criticality | Description | Default | Example |
|----------|------|--------------|-------------|-------------|---------|---------|
| **entity_class** | Enum | ~4 | **HIGH** | TestImplementation or CodeImplementation | CodeImplementation | `CodeImplementation` |
| **testability** | Enum | ~8 | **MEDIUM** | High, Medium, Low | Medium | `Medium` |
| **complexity** | Enum | ~8 | **MEDIUM** | Simple, Moderate, Complex | Simple | `Simple` |
| **dependencies** | usize | ~8 | **LOW** | Number of dependencies (count) | 0 | `5` |
| **test_coverage_estimate** | f64 | ~8 | **LOW** | Estimated coverage % (0.0-1.0) | 0.0 | `0.75` |
| **critical_path** | Boolean | ~1 | **MEDIUM** | Whether on critical execution path | false | `true` |
| **change_risk** | Enum | ~8 | **MEDIUM** | Low, Medium, High risk | Medium | `High` |

**Total per entity**: ~45 bytes
**Overall Criticality**: **MEDIUM** (entity_class is HIGH, rest is MEDIUM-LOW)

### Why MEDIUM Criticality

**HIGH (entity_class)**:
- **Test impact analysis**: Find all test entities that depend on changed code
- **Test planning**: Separate production code from tests
- **Coverage gaps**: Find code entities without corresponding tests

**MEDIUM (complexity, critical_path, change_risk)**:
- **Prioritization**: Focus on high-complexity, critical-path, high-risk code
- **Effort estimation**: Complex entities need more testing
- **Risk assessment**: High-risk changes need extra scrutiny

**LOW (dependencies count, test_coverage_estimate)**:
- **Nice to have**: Duplicates data from DependencyEdges (dependencies)
- **Estimates**: test_coverage_estimate is often inaccurate

### Default Value Problem

**Issue**: 6 of 7 fields have default values that are often wrong
- `testability: Medium` - Not analyzed, just default
- `complexity: Simple` - Not analyzed, just default
- `dependencies: 0` - Wrong if entity has dependencies
- `test_coverage_estimate: 0.0` - Always default
- `critical_path: false` - Needs flow analysis
- `change_risk: Medium` - Not analyzed

**Recommendation**: Only export `entity_class` by default, rest via --query

### Query Examples

```datalog
# Find all test entities
?[isgl1_key, name] :=
  *CodeGraph{ISGL1_key: isgl1_key, TDD_Classification: tdd, interface_signature: sig},
  tdd.entity_class == "TestImplementation",
  sig.name: name

# Find high-risk, complex code on critical path
?[isgl1_key, complexity, change_risk] :=
  *CodeGraph{ISGL1_key: isgl1_key, TDD_Classification: tdd},
  tdd.entity_class == "CodeImplementation",
  tdd.complexity == "Complex",
  tdd.critical_path == true,
  tdd.change_risk: change_risk
```

---

## 6. LSP Type Information Variables ~~(MEDIUM Criticality)~~ ❌ **PLANNED (Not Implemented)**

**Source**: `CodeGraph.lsp_meta_data.type_information` (TypeInformation struct)
**Extracted by**: ~~PT01 via rust-analyzer LSP server~~ **NOT CURRENTLY EXTRACTED**
**Size**: ~150 bytes per entity (if implemented)
**Availability**: ~~Rust files only~~ **Infrastructure exists but stubbed - always returns None**

> **⚠️ IMPLEMENTATION STATUS**: LSP integration is planned but not implemented. PT01's LSP client (`lsp_client.rs:67-74`) is a graceful degradation stub that always returns `None`. The `lsp_meta_data` column exists in storage but is **always NULL**. No LSP dependencies (tower-lsp, lsp-types) in Cargo.toml. Tests use mocks, not real implementation.
>
> **Evidence**: `crates/pt01-folder-to-cozodb-streamer/src/lsp_client.rs:67-92` (stub), `src/streamer.rs:490-513` (always None)
>
> **Current Extraction**: PT01 uses **tree-sitter AST parsing only** - no LSP enrichment.

### Variables

| Variable | Type | Size (bytes) | Criticality | Description | Nullable | Example |
|----------|------|--------------|-------------|-------------|----------|---------|
| **resolved_type** | String | ~40 | **MEDIUM** | Fully qualified type name | No | `std::vec::Vec<Item>` |
| **module_path** | Vec<String> | ~50 | **MEDIUM** | Canonical module location | No | `["std", "vec"]` |
| **generic_parameters** | Vec<String> | ~30 | **LOW** | Resolved generic types | No | `["Item"]` |
| **definition_location.file_path** | PathBuf | ~40 | **MEDIUM** | Where type is defined | Yes | `~/.rustup/.../vec.rs` |
| **definition_location.line** | u32 | ~4 | **MEDIUM** | Definition line number | Yes | `295` |
| **definition_location.character** | u32 | ~4 | **LOW** | Definition column number | Yes | `12` |

**Total per entity**: ~168 bytes
**Criticality**: **MEDIUM** - Useful for refactoring and type analysis

### Why MEDIUM Criticality

**Useful for**:
- **Type-aware refactoring**: Find all usages of a specific type
- **Cross-crate navigation**: Jump to type definitions in dependencies
- **Generic type resolution**: Understand concrete types in generic code
- **Import analysis**: See where types are imported from

**Not essential because**:
- Only available for Rust (not JavaScript, Python, etc.)
- Requires rust-analyzer running during indexing
- Can be expensive to compute
- Often derivable from static analysis of imports

### Query Examples

```datalog
# Find all usages of Vec<T>
?[isgl1_key, resolved_type] :=
  *CodeGraph{ISGL1_key: isgl1_key, lsp_meta_data: lsp},
  lsp.type_information.resolved_type ~ "Vec<",
  lsp.type_information.resolved_type: resolved_type

# Find types defined in external crates
?[isgl1_key, module_path] :=
  *CodeGraph{ISGL1_key: isgl1_key, lsp_meta_data: lsp},
  lsp.type_information.module_path: mod_path,
  mod_path[0] != "crate",
  module_path = mod_path
```

---

## 7. LSP Usage Analysis Variables ~~(HIGH Criticality)~~ ❌ **PLANNED (Not Implemented)**

**Source**: `CodeGraph.lsp_meta_data.usage_analysis` (UsageAnalysis struct)
**Extracted by**: ~~PT01 via rust-analyzer LSP server~~ **NOT CURRENTLY EXTRACTED**
**Size**: ~100-500 bytes per entity (if implemented)

> **⚠️ IMPLEMENTATION STATUS**: Not extracted - LSP client stub always returns `None`.
>
> **ALTERNATIVE (Already Implemented)**: Use **DependencyEdges relation** for dependency/dependent analysis:
> - ✅ `DependencyEdges` provides reverse lookup functionality (who depends on entity X)
> - ✅ Forward/reverse dependency queries fully implemented (`cozo_client.rs:420-514`)
> - ✅ Includes edge_type (Calls vs Uses vs Implements)
> - ⚠️ LSP would add: cross-crate references from external dependencies (not yet available)

### Variables

| Variable | Type | Size (bytes) | Criticality | Description | Nullable | Example |
|----------|------|--------------|-------------|-------------|----------|---------|
| **total_references** | usize | ~8 | **HIGH** | Number of times referenced | No | `42` |
| **usage_locations** | Vec<Location> | ~100-400 | **MEDIUM** | All reference locations (file:line:char) | No | `[{file: "main.rs", line: 10, char: 5}, ...]` |
| **dependents** | Vec<String> | ~50-100 | **HIGH** | ISGL1 keys of entities that reference this | No | `["rust:fn:main:...", "rust:fn:helper:..."]` |

**Total per entity**: ~158-508 bytes
**Criticality**: **HIGH** - Critical for impact analysis and dead code detection

### Why HIGH Criticality

**total_references**:
- **Dead code detection**: 0 references = potentially unused
- **Refactoring risk**: High reference count = high impact change
- **API popularity**: Understand which APIs are widely used

**dependents** (array of ISGL1 keys):
- **Blast radius**: Who will break if this changes?
- **Test impact**: Which tests use this entity?
- **Reverse dependencies**: Navigate call graph backwards

**Why this overlaps with DependencyEdges**:
- LSP `dependents` = reverse lookup of DependencyEdges `to_key`
- **Advantage of LSP**: Includes cross-crate references from dependencies
- **Advantage of DependencyEdges**: Includes edge_type (Calls vs Uses vs Implements)
- **Recommendation**: Use DependencyEdges for intra-project, LSP for cross-crate

### Query Examples

```datalog
# Find dead code (0 references, not public)
?[isgl1_key, name] :=
  *CodeGraph{ISGL1_key: isgl1_key, lsp_meta_data: lsp, interface_signature: sig},
  lsp.usage_analysis.total_references == 0,
  sig.visibility != "Public",
  sig.name: name

# Find high-impact entities (>50 references)
?[isgl1_key, ref_count] :=
  *CodeGraph{ISGL1_key: isgl1_key, lsp_meta_data: lsp},
  lsp.usage_analysis.total_references: ref_count,
  ref_count > 50
```

---

## 8. LSP Semantic Tokens Variables ~~(LOW Criticality)~~ ❌ **PLANNED (Not Implemented)**

**Source**: `CodeGraph.lsp_meta_data.semantic_tokens` (Vec<SemanticToken>)
**Extracted by**: ~~PT01 via rust-analyzer LSP server~~ **NOT CURRENTLY EXTRACTED**
**Size**: ~500-2000 bytes per entity (if implemented)

> **⚠️ IMPLEMENTATION STATUS**: Not extracted - LSP client stub always returns `None`. Primarily useful for IDE features (syntax highlighting, code folding), not core dependency analysis.

### Variables (per token)

| Variable | Type | Size (bytes) | Criticality | Description | Example |
|----------|------|--------------|-------------|-------------|---------|
| **position.file_path** | PathBuf | ~40 | **LOW** | File containing token | `src/main.rs` |
| **position.line** | u32 | ~4 | **LOW** | Line number | `42` |
| **position.character** | u32 | ~4 | **LOW** | Column number | `8` |
| **length** | u32 | ~4 | **LOW** | Token length in characters | `4` |
| **token_type** | String | ~15 | **LOW** | Type: keyword, variable, function, type, etc. | `function` |
| **modifiers** | Vec<String> | ~20 | **LOW** | Modifiers: mutable, static, async, etc. | `["async", "public"]` |

**Total per token**: ~87 bytes
**Tokens per entity**: ~10-50 (depends on code size)
**Total per entity**: ~870-4350 bytes
**Criticality**: **LOW** - Primarily for IDE features, rarely needed in LLM context

### Why LOW Criticality

**Use cases (mostly IDE)**:
- Syntax highlighting (better than regex)
- Semantic search ("find all async functions")
- Code folding hints
- Inline hints (type annotations)

**Why not useful for LLM**:
- Too granular (individual tokens, not high-level structure)
- Large size (10x larger than interface signatures)
- Better alternatives exist (dependency graphs, interface signatures)

**Recommendation**: Never export by default, only via explicit --query

---

## 9. Language-Specific Variables (LOW Criticality)

**Source**: `CodeGraph.interface_signature.language_specific`
**Extracted by**: PT01 during tree-sitter parsing
**Size**: ~50-200 bytes per entity (depends on language)

### Rust-Specific Variables (RustSignature)

| Variable | Type | Size (bytes) | Criticality | Description | Example |
|----------|------|--------------|-------------|-------------|---------|
| **generics** | Vec<String> | ~20 | **MEDIUM** | Generic type parameters | `["T", "E"]` |
| **lifetimes** | Vec<String> | ~15 | **LOW** | Lifetime parameters | `["'a", "'static"]` |
| **where_clauses** | Vec<String> | ~40 | **LOW** | Where clause constraints | `["T: Clone", "E: Error"]` |
| **attributes** | Vec<String> | ~30 | **LOW** | Attributes like `#[derive(...)]` | `["derive(Debug)", "test"]` |
| **trait_impl.trait_name** | String | ~20 | **MEDIUM** | Trait being implemented | `Display` |
| **trait_impl.for_type** | String | ~20 | **MEDIUM** | Type implementing trait | `MyStruct` |

**Total per entity**: ~145 bytes

### JavaScript-Specific Variables (JavascriptSignature)

| Variable | Type | Size (bytes) | Criticality | Description | Example |
|----------|------|--------------|-------------|-------------|---------|
| **parameters** | Vec<Parameter> | ~40 | **MEDIUM** | Function parameters | `[{name: "x", type: null}]` |
| **return_type** | String | ~20 | **MEDIUM** | Return type (JSDoc) | `Promise<void>` |
| **is_async** | Boolean | ~1 | **MEDIUM** | Async function flag | `true` |
| **is_arrow** | Boolean | ~1 | **LOW** | Arrow function flag | `false` |

### TypeScript-Specific Variables (TypeScriptSignature)

| Variable | Type | Size (bytes) | Criticality | Description | Example |
|----------|------|--------------|-------------|-------------|---------|
| **parameters** | Vec<TypedParameter> | ~60 | **MEDIUM** | Typed parameters | `[{name: "x", type: "number", optional: false}]` |
| **return_type** | String | ~20 | **MEDIUM** | Return type annotation | `Promise<User>` |
| **generics** | Vec<String> | ~20 | **MEDIUM** | Generic type parameters | `["T", "K"]` |
| **is_async** | Boolean | ~1 | **MEDIUM** | Async function flag | `true` |

### Python-Specific Variables (PythonSignature)

| Variable | Type | Size (bytes) | Criticality | Description | Example |
|----------|------|--------------|-------------|-------------|---------|
| **parameters** | Vec<PythonParameter> | ~80 | **MEDIUM** | Parameters with defaults | `[{name: "x", type: "int", default: "0", is_varargs: false}]` |
| **return_type** | String | ~20 | **MEDIUM** | Return type hint | `List[User]` |
| **is_async** | Boolean | ~1 | **MEDIUM** | Async function flag | `false` |
| **decorators** | Vec<String> | ~40 | **MEDIUM** | Decorators | `["@staticmethod", "@lru_cache"]` |

### Why MEDIUM-LOW Criticality

**MEDIUM (parameters, return_type, is_async, generics)**:
- **Type-aware code generation**: Generate type-correct calls
- **Refactoring**: Understand function signatures when changing APIs
- **Cross-language analysis**: Compare similar patterns across languages

**LOW (lifetimes, where_clauses, attributes, is_arrow)**:
- **Language-specific details**: Mostly for language-specific tooling
- **Not universal**: Only meaningful for that language
- **Derivable**: Can often be re-parsed from current_code

---

## 10. Entity Metadata Variables (LOW Criticality)

**Source**: `CodeGraph.metadata` (EntityMetadata struct)
**Extracted by**: PT01 during indexing
**Size**: ~120 bytes per entity

### Variables

| Variable | Type | Size (bytes) | Criticality | Description | Example |
|----------|------|--------------|-------------|-------------|---------|
| **created_at** | DateTime | ~30 | **LOW** | When entity was first indexed | `2025-11-02T10:30:00Z` |
| **modified_at** | DateTime | ~30 | **LOW** | When entity was last updated | `2025-11-02T14:22:00Z` |
| **content_hash** | String | ~40 | **LOW** | SHA-256 hash of entity code | `a3f5b8c...` |
| **additional** | HashMap<String, String> | ~20 | **LOW** | Custom key-value pairs | `{"version": "0.8.3"}` |

**Total per entity**: ~120 bytes
**Criticality**: **LOW** - Metadata about metadata, rarely queried

### Why LOW Criticality

**Use cases**:
- **Change detection**: Compare content_hash to detect modifications
- **Timestamp filtering**: "Show entities modified in last week"
- **Custom metadata**: Store arbitrary key-value data

**Why not essential**:
- Not needed for code understanding or navigation
- Temporal state (current_ind, future_ind) is better for change detection
- File modification times are often more accurate than created_at

---

## Export Presets by Criticality

### Preset 1: Essential (HIGH Only)

**Export level**: `--export-level essential` (default)
**Size**: ~20-30KB for 590 entities (~34-51 bytes per entity)
**Token estimate**: ~5-8K tokens

**Included variables**:
- **Dependency Graph** (all 4 variables): from_key, to_key, edge_type, source_location
- **Core Identity**: isgl1_key, name, entity_type, language, file_path
- **Temporal State** (all 3): current_ind, future_ind, future_action
- **Interface Signature**: visibility, line_range.start, line_range.end
- **TDD Classification**: entity_class ONLY
- **LSP Usage Analysis**: total_references, dependents

**Example output**:
```json
{
  "nodes": [
    {
      "key": "rust:fn:main:src_main_rs:1-10",
      "name": "main",
      "type": "fn",
      "language": "rust",
      "file_path": "src/main.rs",
      "line_range": {"start": 1, "end": 10},
      "visibility": "Public",
      "entity_class": "CODE",
      "temporal": {
        "current_ind": true,
        "future_ind": true,
        "future_action": null
      },
      "usage": {
        "total_references": 0,
        "dependents": []
      }
    }
  ],
  "edges": [
    {
      "from": "rust:fn:main:src_main_rs:1-10",
      "to": "rust:fn:helper:src_lib_rs:20-30",
      "type": "Calls",
      "location": "src/main.rs:5"
    }
  ]
}
```

---

### Preset 2: Standard (HIGH + MEDIUM)

**Export level**: `--export-level standard`
**Size**: ~80-100KB for 590 entities (~136-169 bytes per entity)
**Token estimate**: ~20-25K tokens

**Added variables** (beyond Essential):
- **Interface Signature**: module_path, documentation
- **TDD Classification**: testability, complexity, critical_path, change_risk
- **LSP Type Information** (if available): resolved_type, module_path, definition_location
- **Language-Specific**: parameters, return_type, generics, is_async, trait_impl

**Use cases**:
- Refactoring with type awareness
- Test planning with complexity/risk data
- API documentation generation
- Cross-crate navigation

---

### Preset 3: Full (HIGH + MEDIUM + LOW)

**Export level**: `--export-level full`
**Size**: ~170-200KB for 590 entities (~288-339 bytes per entity)
**Token estimate**: ~43-50K tokens

**Added variables** (beyond Standard):
- **TDD Classification**: dependencies (count), test_coverage_estimate
- **LSP Type Information**: generic_parameters, definition_location.character
- **LSP Usage Analysis**: usage_locations (full array)
- **Language-Specific**: lifetimes, where_clauses, attributes, decorators, is_arrow
- **Entity Metadata**: created_at, modified_at, content_hash, additional

**Use cases**:
- Comprehensive code analysis
- IDE integration
- Custom analytics
- Debugging

---

### Preset 4: Bulk (Everything + Code)

**Export level**: `--export-level bulk`
**Size**: ~400-450KB for 590 entities (~678-763 bytes per entity)
**Token estimate**: ~100-113K tokens

**Added variables** (beyond Full):
- **current_code**: Full source code (current state)
- **future_code**: Full source code (future state)

**Use cases**:
- Code generation
- LLM-powered refactoring
- Detailed debugging
- Training data for ML models

**Warning**: This is the most expensive export - only use when you explicitly need source code!

---

## Summary Statistics

### Variable Count by Category

| Category | Total Variables | Status | HIGH | MEDIUM | LOW |
|----------|----------------|--------|------|--------|-----|
| **ACTUALLY EXTRACTED (Tree-sitter AST)** | | | | | |
| Dependency Graph | 4 | ✅ Extracted | 4 | 0 | 0 |
| Core Identity | 5 | ✅ Extracted | 5 | 0 | 0 |
| Core Identity (code fields) | 2 | ✅ Extracted (BULK) | 0 | 0 | 2 |
| Temporal State | 3 | ✅ Extracted | 3 | 0 | 0 |
| Interface Signature | 8 | ✅ Extracted | 3 | 2 | 3 |
| TDD Classification | 7 | ✅ Extracted | 1 | 4 | 2 |
| Language-Specific (Rust) | 6 | ✅ Extracted | 0 | 2 | 4 |
| Entity Metadata | 4 | ✅ Extracted | 0 | 0 | 4 |
| **Subtotal Extracted** | **39** | | **16** | **8** | **15** |
| **PLANNED BUT NOT IMPLEMENTED (LSP)** | | | | | |
| LSP Type Information | 6 | ❌ Stub (always None) | 0 | 4 | 2 |
| LSP Usage Analysis | 3 | ❌ Stub (always None) | 2 | 1 | 0 |
| LSP Semantic Tokens | 6 | ❌ Stub (always None) | 0 | 0 | 6 |
| **Subtotal Planned** | **15** | | **2** | **5** | **8** |
| **TOTAL DOCUMENTED** | **54** | | **18** | **13** | **23** |

**Key Findings:**
- ✅ **37 variables extracted** (tree-sitter AST parsing) - excludes code fields
- ✅ **39 variables extracted** (with current_code, future_code)
- ❌ **15 variables planned** (LSP metadata - infrastructure exists but stubbed)
- **Total documented**: 54 variables

**What's Actually Available:**
- Dependency graphs, ISG data, temporal state, TDD classification, entity metadata: **All extracted**
- LSP type information, usage analysis, semantic tokens: **NOT extracted** (stub returns None)

### Size Comparison

| Export Level | Variables | Status | Size (590 ent) | Tokens | Savings vs Bulk |
|--------------|-----------|--------|----------------|--------|-----------------|
| **Essential** | 16 HIGH criticality | ✅ Available now | ~20-30KB | ~5-8K | **15-22x** |
| **Standard** | 24 (HIGH + MEDIUM, no LSP) | ✅ Available now | ~60-80KB | ~15-20K | **6-8x** |
| **Full** | 39 (all extracted, no LSP) | ✅ Available now | ~100-130KB | ~25-33K | **3-4x** |
| **Full + LSP** | 54 (with LSP metadata) | ⚠️ Planned (LSP not implemented) | ~170-200KB | ~43-50K | **2-3x** |
| **Bulk** | 39 + code fields | ✅ Available now | ~400-450KB | ~100-113K | **1x (baseline)** |

**Note**: "Full + LSP" estimates require LSP implementation (currently stubbed).

---

## Recommendations

### Default Export: Essential

**Rationale**:
- Answers 95% of code navigation questions
- 15-22x smaller than with-code exports
- LLMs understand graphs better than signatures
- Contains highest-value data (relationships, identity, temporal state)

### When to Use Standard

- Type-aware refactoring
- API documentation generation
- Test planning with complexity/risk analysis
- Cross-language codebase analysis

### When to Use Full

- IDE integration
- Comprehensive code metrics
- Custom analytics dashboards
- Dead code detection with full metadata

### When to Use Bulk

- LLM-powered code generation
- Detailed debugging (need to see actual code)
- Training data for ML models
- One-off deep analysis tasks

**Warning**: Only use Bulk when explicitly needed - it's 15-22x larger!

---

## Implementation Checklist

### Phase 1: Add Export Levels to PT02
- [ ] Add `--export-level` CLI flag (essential, standard, full, bulk)
- [ ] Implement variable filtering based on level
- [ ] Update query builder to select appropriate columns
- [ ] Add graph-optimized JSON output format

### Phase 2: Update Documentation
- [ ] Update lib.rs with export level examples
- [ ] Add this dictionary to root repository
- [ ] Update Challenge01-ISGSize.md with findings
- [ ] Create migration guide for existing users

### Phase 3: Validate with Real Data
- [ ] Export parseltongue codebase at all 4 levels
- [ ] Measure actual sizes and token counts
- [ ] Test LLM reasoning quality at each level
- [ ] Document findings

---

**End of All Extracted Metadata Dictionary**
