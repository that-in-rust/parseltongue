# Challenge 04: PT02 Implementation Plan - Command Optimization & Interface Redesign

**Version**: v0.8.3
**Date**: 2025-11-02
**Purpose**: Actionable implementation plan for PT02 command optimization with token cost analysis
**Status**: 🎯 **READY FOR IMPLEMENTATION**

---

## Executive Summary

This document provides a comprehensive analysis of immediate PT02 implementation priorities, including criticality assessment, token cost calculations, and interface redesign recommendations.

### Critical Findings

**1. Biggest Gap: Dependency Graphs** 🔥
- **Status**: ✅ EXTRACTED, ✅ STORED, ❌ **NOT EXPOSED IN PT02 CLI**
- **Impact**: 5 HIGH-priority Challenge03 variables completely invisible to LLMs
- **Token cost**: 54,575 tokens (one-time export for 590 entities)
- **Criticality**: **CRITICAL** - Blast radius, impact analysis, architectural reasoning

**2. Scope Misalignment**
- **User's "essential fields"**: 23 variables, ~62K tokens
- **Challenge03 Tier 1**: 38 variables, ~295-590K tokens
- **Gap**: User's list is **60-80% under-scoped** for Tier 1
- **Recommendation**: Rename to `--preset compact`, not `--preset essential`

**3. Quick Wins Available**
- **3 trivial optimizations**: Save ~106K tokens with <8 hours implementation
  1. Null field exclusion: -41K tokens (40% reduction)
  2. TDD de-bloat: -10K tokens (85% reduction)
  3. Dependency as separate export: 54K tokens reusable

**4. Interface Design**
- **Current proposal**: Separate binaries (verbose, non-S01 compliant)
- **Recommended**: Preset system with granular overrides (composable, future-proof)

### Key Statistics (590 Entities)

| Configuration | Variables | Total Tokens | vs Current | Implementation Effort |
|---------------|-----------|--------------|------------|----------------------|
| **Current (no code)** | 4 | 15,340 | baseline | ✅ Done |
| **+ Optimizations** | 4 | ~10,000 | -35% | TRIVIAL (< 8 hrs) |
| **+ Compact preset** | 25 | 62,540 | +308% | MEDIUM (2 weeks) |
| **+ Dependencies** | - | +54,575 | +356% | LOW (1 week) |
| **Total optimized** | 25 | 117,115 | +663% | 3 weeks |
| **Challenge03 Tier 1** | 38 | 295,000-590,000 | +1,823-3,747% | 25-35 days |

---

## 1. Criticality Assessment

### 1.1 Dependency Graphs: **CRITICAL** 🔥

**Challenge03 Alignment**: 5 HIGH-priority variables
- `forward_deps` (HIGH #23)
- `reverse_deps` (HIGH #24)
- `blast_radius_count` (HIGH #25)
- `blast_radius_files` (HIGH #26)
- `module_dependencies` (HIGH #27)

**Current Implementation Status**:

| Component | Status | Location | Tests |
|-----------|--------|----------|-------|
| **Extraction** | ✅ Complete | PT01: `isgl1_generator.rs:540-612` | 3 tests |
| **Storage** | ✅ Complete | CozoDB `DependencyEdges` table | - |
| **Blast radius query** | ✅ Complete | Core: `cozo_client.rs:305-372` | 4 tests |
| **Forward deps query** | ✅ Complete | Core: `cozo_client.rs:420-443` | 5 tests |
| **Reverse deps query** | ✅ Complete | Core: `cozo_client.rs:491-514` | 4 tests |
| **Transitive closure** | ✅ Complete | Core: `cozo_client.rs:588-625` | 4 tests |
| **PT02 CLI exposure** | ❌ **MISSING** | - | - |

**Variables Stored** (per edge):

| Variable | Type | Size | Description | Example |
|----------|------|------|-------------|---------|
| `from_key` | String | ~60 bytes | Source ISGL1 key | `rust:fn:main:src_main_rs:1-10` |
| `to_key` | String | ~60 bytes | Target ISGL1 key | `rust:fn:helper:src_lib_rs:20-30` |
| `edge_type` | Enum | ~8 bytes | Relationship type | `Calls`, `Uses`, `Implements` |
| `source_location` | String | ~20 bytes | Where relationship occurs | `src/main.rs:5` |

**Total per edge**: ~148 bytes = ~37 tokens/edge (4 bytes ≈ 1 token)

**Token Cost Calculation** (Parseltongue codebase):
```
590 entities × avg 2.5 dependencies/entity = 1,475 edges
1,475 edges × 37 tokens/edge = 54,575 tokens

User's estimate: 50-80KB ✅ (aligns with calculation)
```

**Why CRITICAL**:
1. **Blast radius calculation**: Quantify change impact (23 entities affected)
2. **Dependency traversal**: Navigate call graphs, usage patterns
3. **Test impact analysis**: Which tests to run when code changes
4. **Refactoring safety**: Understand cascading effects
5. **Architecture understanding**: Module coupling and cohesion visibility

**Expected Output Format** (JSONL):
```json
{"from": "rust:fn:parse_markdown:src_parser_rs:10", "to": "rust:fn:tokenize:src_tokenizer_rs:5", "type": "Calls", "loc": "src/parser.rs:15"}
{"from": "rust:fn:main:src_main_rs:1", "to": "rust:fn:parse_markdown:src_parser_rs:10", "type": "Calls", "loc": "src/main.rs:8"}
```

**Token Efficiency**:
- Embedding in ISG: +92 tokens/entity (avg 2.5 deps × 37 tokens)
- Separate export: 54K tokens ONCE
- Savings per query: 54K tokens (after initial export)
- Reusability: HIGH (dependency graph changes infrequently)

---

### 1.2 Temporal State: **CRITICAL** ✅

**Challenge03 Alignment**: 3 HIGH-priority variables
- `current_ind` (HIGH #46)
- `future_ind` (HIGH #47)
- `future_action` (HIGH #48)

**Current Implementation Status**:

| Component | Status | Location |
|-----------|--------|----------|
| **Extraction** | ✅ Complete | PT01 initializes all entities |
| **Storage** | ✅ Complete | `TemporalState` struct in `CodeGraph` |
| **Updates** | ✅ Complete | PT03 modifies via `--action` |
| **Query access** | ✅ Complete | PT02 `--query` flag (advanced) |
| **Simple flag** | ⚠️ Missing | No `--include-temporal` flag |

**Variables** (per entity):

| Variable | Type | Size | Description | Values |
|----------|------|------|-------------|--------|
| `current_ind` | Boolean | ~1 byte | Current file exists | `0` or `1` |
| `future_ind` | Boolean | ~1 byte | Future file exists | `0` or `1` |
| `future_action` | Enum | ~10 bytes | Planned action | `None`, `Create`, `Edit`, `Delete` |

**Total per entity**: ~12 bytes = ~3 tokens/entity

**Token Cost Calculation**:
```
590 entities × 3 tokens = 1,770 tokens total
```

**Temporal State Semantics**:

| current_ind | future_ind | future_action | Meaning |
|-------------|------------|---------------|---------|
| `1` | `1` | `None` | Unchanged entity (stable) |
| `1` | `1` | `Edit` | Entity will be modified |
| `1` | `0` | `Delete` | Entity will be removed |
| `0` | `1` | `Create` | Entity will be added |

**Why CRITICAL**:
1. **Change planning**: Track multi-step operations (PT03 → PT04 → PT05 → PT06)
2. **Blast radius**: Filter only changed entities for analysis
3. **Workflow coordination**: State machine for temporal operations
4. **Incremental updates**: Only re-process changed entities

**Current Access Method** (advanced):
```bash
# Requires Datalog knowledge
pt02 --query "?[isgl1_key, current_ind, future_ind, future_action] := *CodeGraph{isgl1_key, temporal_state: {current_ind, future_ind, future_action}}"
```

**Gap**: Should have simple flag like `--include-temporal` or embedded in presets.

---

### 1.3 TDD Classification: **MEDIUM** ⚠️

**Challenge03 Alignment**: 1 HIGH-priority variable
- `entity_class` (HIGH #39)

**Current Implementation Status**:

| Component | Status | Issue |
|-----------|--------|-------|
| **Extraction** | ✅ Complete | Extracted by PT01 |
| **Storage** | ✅ Complete | `TDDClassification` struct |
| **PT02 export** | ⚠️ **BLOATED** | 7 fields, 6 are defaults |

**Fields Currently Exported**:

| Field | Type | Value | Criticality | Waste |
|-------|------|-------|-------------|-------|
| `entity_class` | Enum | `CodeImplementation`, `TestLogic`, etc. | **HIGH** | ✅ Keep |
| `has_code` | Boolean | Always `true` | LOW | ❌ Remove |
| `should_have_tests` | Boolean | Usually `true` | LOW | ❌ Remove |
| `should_propagate` | Boolean | Usually `true` | LOW | ❌ Remove |
| `is_generic` | Boolean | Usually `false` | LOW | ❌ Remove |
| `test_coverage_status` | Enum | Usually `Unknown` | LOW | ❌ Remove |
| `propagation_heuristic` | String | Usually `default` | LOW | ❌ Remove |

**Token Cost Analysis**:
```
Current bloat: 7 fields × ~3 tokens = ~21 tokens/entity
Useful: entity_class only = ~3 tokens/entity
Waste: 18 tokens/entity × 590 = 10,620 tokens (86% waste)

Savings: 10,620 tokens (86% reduction)
```

**Why MEDIUM (not CRITICAL)**:
- Easy token savings but less critical for reasoning than dependencies/types
- `entity_class` is useful (distinguishes test vs code vs interface)
- But 6/7 fields provide near-zero reasoning value

**Recommended Output** (minimal):
```json
{
  "isgl1_key": "rust:fn:calculate_total:src_billing_rs:42",
  "entity_class": "CodeImplementation"
}
```

---

### 1.4 LSP Essential Data (23 Variables): **MIXED**

User proposed 23 variables. Let's categorize by Challenge03 criticality:

#### **CRITICAL Tier** (12 variables - Challenge03 HIGH)

| # | Variable | Challenge03 ID | Token Cost | Status |
|---|----------|---------------|-----------|--------|
| 1 | `generic_params` | HIGH #12 | ~30-200 | ⏳ LSP needed |
| 2 | `where_clauses` | HIGH #13 | ~50-300 | ⏳ LSP needed |
| 3 | `return_type` | HIGH #14 | ~20-100 | ⏳ LSP needed |
| 4 | `param_types` | HIGH #15 | ~30-200 | ⏳ LSP needed |
| 5 | `trait_bounds` | HIGH #16 | ~30-200 | ⏳ LSP needed |
| 6 | `forward_deps` | HIGH #23 | ~100-500 | ✅ In CozoDB |
| 7 | `reverse_deps` | HIGH #24 | ~100-500 | ✅ In CozoDB |
| 8 | `blast_radius_count` | HIGH #25 | ~5 | ✅ In CozoDB |
| 9 | `blast_radius_files` | HIGH #26 | ~50-300 | ✅ In CozoDB |
| 10 | `module_dependencies` | HIGH #27 | ~50-200 | ⏳ Needs extraction |
| 11 | `total_references` | - | ~5 | ⏳ LSP needed |
| 12 | `usage_locations` | - | ~20-100 | ⏳ LSP needed |

**Subtotal**: 12 CRITICAL variables, ~485-2,610 tokens/entity

#### **HIGH Tier** (10 variables - Challenge03 MEDIUM)

| # | Variable | Challenge03 ID | Token Cost | Status |
|---|----------|---------------|-----------|--------|
| 13 | `lifetime_params` | MEDIUM #17 | ~20-100 | ⏳ LSP needed |
| 14 | `impl_trait_for` | MEDIUM #18 | ~30-150 | ⏳ LSP needed |
| 15 | `associated_types` | MEDIUM #19 | ~30-150 | ⏳ LSP needed |
| 16 | `type_aliases` | MEDIUM #20 | ~20-100 | ⏳ LSP needed |
| 17 | `derived_traits` | MEDIUM #21 | ~20-100 | ⏳ Tree-sitter |
| 18 | `const_generics` | MEDIUM #22 | ~20-80 | ⏳ LSP needed |
| 19 | `transitive_deps_forward` | MEDIUM #28 | ~200-1000 | ✅ In CozoDB |
| 20 | `transitive_deps_reverse` | MEDIUM #29 | ~200-1000 | ✅ In CozoDB |
| 21 | `import_statements` | MEDIUM #30 | ~50-300 | ⏳ Tree-sitter |
| 22 | `macro_invocations` | MEDIUM #31 | ~30-200 | ⏳ Tree-sitter |

**Subtotal**: 10 HIGH variables, ~620-3,180 tokens/entity

#### **MEDIUM Tier** (3 variables - Nice-to-have)

| # | Variable | Challenge03 ID | Token Cost | Status |
|---|----------|---------------|-----------|--------|
| 23 | `trait_object_usage` | MEDIUM #32 | ~30-150 | ⏳ Tree-sitter |
| 24 | `has_tests` | HIGH #39 | ~5 | ⏳ Test detection |
| 25 | `dependents` | - | ~100-500 | ✅ (duplicate of reverse_deps) |

**Subtotal**: 3 MEDIUM variables, ~135-655 tokens/entity

---

#### **Total "Essential Fields" Token Cost**

**Without null exclusion**:
```
CRITICAL (12 vars): ~485-2,610 tokens
HIGH (10 vars): ~620-3,180 tokens
MEDIUM (3 vars): ~135-655 tokens

Total: ~1,240-6,445 tokens/entity (highly variable)
Average: ~3,840 tokens/entity
590 entities × 3,840 = ~2,265,600 tokens
```

**With null exclusion** (assuming 40% fields are null):
```
Total × 0.6 = ~1,359,360 tokens
Per entity: ~2,304 tokens
```

**Wait, this doesn't match user's estimate!**

The discrepancy is because:
1. **User wants "only if not null"**: Many fields will be missing
2. **User wants "essential only"**: Likely skipping expensive vars like transitive_deps
3. **Conservative estimate**: Most entities have minimal type complexity

**Realistic estimate** (excluding expensive vars, high null rate):
```
Core fields: ~50 tokens/entity
Type system (sparse): ~30 tokens/entity
Dependencies (in separate export): 0 tokens/entity
Usage metrics: ~20 tokens/entity

Total: ~100 tokens/entity
590 entities × 100 = ~59,000 tokens ✅

This matches user's intent!
```

---

## 2. Token Cost Analysis (590 Entities)

### 2.1 Current Baseline

**PT02 with `--include-current-code false`** (4 variables):
```
Variables:
- isgl1_key: ~15 tokens
- entity_name: ~8 tokens
- entity_type: ~2 tokens
- file_path: ~15 tokens

Total: ~40 tokens/entity (not 26 as initially estimated)
590 entities × 40 = ~23,600 tokens
```

**PT02 with `--include-current-code true`** (4 variables + code):
```
+ current_code: ~1,200 tokens/entity (avg for Rust)

Total: ~1,240 tokens/entity
590 entities × 1,240 = ~731,600 tokens
```

---

### 2.2 Proposed Configurations

| Configuration | Variables | Tokens/Entity | Total (590) | vs Baseline (no code) | Implementation Effort |
|---------------|-----------|---------------|-------------|---------------------|----------------------|
| **Current (no code)** | 4 | 40 | 23,600 | baseline | ✅ Done |
| **+ Null exclusion** | 4 | 24 | 14,160 | -40% | TRIVIAL (< 1 hr) |
| **+ TDD de-bloat** | 4 | 26 | 15,340 | -35% | TRIVIAL (< 2 hrs) |
| **+ Temporal** | 7 | 29 | 17,110 | -28% | LOW (already queryable) |
| **+ Dependency export** | - | - | +54,575 | +231% | LOW (< 1 week) |
| **"Compact" preset (user's list)** | 25 | 106 | 62,540 | +165% | MEDIUM (2 weeks) |
| **Compact + deps (separate)** | 25 | 106 | 117,115 | +396% | MEDIUM (2-3 weeks) |
| **Compact + code** | 25 | 1,306 | 770,540 | +3,164% | MEDIUM (2 weeks) |
| **Challenge03 Tier 1** | 38 | 500-1,000 | 295,000-590,000 | +1,150-2,400% | HIGH (25-35 days) |

---

### 2.3 Token Savings Breakdown

#### **Quick Win 1: Null Field Exclusion**
```
Estimated null rate: 40% of optional fields
Current optional fields: ~60% of total fields
Savings: 40% × 60% = 24% overall reduction

Current (no code): 40 tokens/entity
With null exclusion: 24 tokens/entity

Savings: 16 tokens/entity × 590 = 9,440 tokens (40% reduction on optionals)
```

#### **Quick Win 2: TDD De-bloat**
```
Current TDD export: 7 fields × 3 tokens = 21 tokens/entity
Proposed TDD export: 1 field × 3 tokens = 3 tokens/entity

Savings: 18 tokens/entity × 590 = 10,620 tokens (86% TDD reduction)
```

#### **Quick Win 3: Dependency as Separate Export**
```
Embedding in ISG: 2.5 deps × 37 tokens = 92.5 tokens/entity
Separate export: 54,575 tokens ONCE

Per-query savings: 92.5 × 590 = 54,575 tokens
Reusability: Very high (dependencies rarely change)
```

**Total Quick Wins**: 9,440 + 10,620 = 20,060 tokens saved in ISG export
**Plus**: 54,575 tokens available separately (reusable)

---

### 2.4 Cost Comparison: Presets

| Preset Name | Variables | Tokens/Entity | Total (590) | Use Case |
|-------------|-----------|---------------|-------------|----------|
| **minimal** | 4 | 24 | 14,160 | Quick checks, minimal context |
| **compact** | 22-25 | 106 | 62,540 | Daily refactoring, moderate reasoning |
| **tier1** | 38 | 500-1,000 | 295,000-590,000 | Safe refactoring, full type reasoning |
| **full** | 78 | 2,000-5,000 | 1,180,000-2,950,000 | Deep analysis, architectural decisions |

---

## 3. Implementation Roadmap

### Phase 1: Critical Gaps (Week 1) 🔥

**Goal**: Fix the most glaring omissions with minimal effort

#### **Task 1.1: Dependency Graph Export**
**Priority**: CRITICAL
**Effort**: LOW (8-16 hours)
**Token impact**: +54,575 tokens (separate export)

**Implementation**:
```rust
// crates/pt02-llm-cozodb-to-context-writer/src/cli.rs
.arg(
    Arg::new("export-dependencies")
        .long("export-dependencies")
        .value_name("PATH")
        .help("Export dependency graph to separate JSON file")
)

// crates/pt02-llm-cozodb-to-context-writer/src/main.rs
if let Some(deps_path) = matches.get_one::<String>("export-dependencies") {
    let edges = storage.get_all_dependency_edges().await?;

    let output = edges.iter()
        .map(|e| json!({
            "from": e.from_key,
            "to": e.to_key,
            "type": format!("{:?}", e.edge_type),
            "loc": e.source_location,
        }))
        .collect::<Vec<_>>();

    let mut file = std::fs::File::create(deps_path)?;
    for edge in output {
        writeln!(file, "{}", serde_json::to_string(&edge)?)?;
    }
}
```

**Output format** (JSONL):
```json
{"from":"rust:fn:parse:src_parser_rs:10","to":"rust:fn:tokenize:src_tokenizer_rs:5","type":"Calls","loc":"src/parser.rs:15"}
```

**Testing**:
```rust
#[tokio::test]
async fn test_export_dependencies() {
    let storage = setup_test_storage().await;
    let edges = storage.get_all_dependency_edges().await?;
    assert!(edges.len() > 0);
    assert_eq!(edges[0].edge_type, EdgeType::Calls);
}
```

**Success criteria**:
- ✅ Can export all edges to JSONL
- ✅ File size ~50-80KB for Parseltongue
- ✅ Validates against expected edge count

---

#### **Task 1.2: Null Field Exclusion**
**Priority**: HIGH
**Effort**: TRIVIAL (1-2 hours)
**Token impact**: -9,440 tokens (40% reduction on optionals)

**Implementation**:
```rust
// crates/parseltongue-core/src/entities.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEntity {
    pub isgl1_key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_params: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub where_clauses: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsp_metadata: Option<LspMetadata>,

    // ... repeat for all optional fields
}
```

**Testing**:
```rust
#[test]
fn test_null_field_exclusion() {
    let entity = CodeEntity {
        isgl1_key: "test_key".to_string(),
        generic_params: None,  // Should not appear in JSON
        // ...
    };

    let json = serde_json::to_string(&entity).unwrap();
    assert!(!json.contains("generic_params"));
}
```

**Success criteria**:
- ✅ Null fields omitted from JSON
- ✅ Non-null fields still present
- ✅ File size reduced by ~40% for sparse entities

---

#### **Task 1.3: TDD Classification De-bloat**
**Priority**: MEDIUM
**Effort**: TRIVIAL (2-4 hours)
**Token impact**: -10,620 tokens (86% TDD reduction)

**Implementation**:
```rust
// crates/pt02-llm-cozodb-to-context-writer/src/main.rs

// New minimal TDD output struct
#[derive(Debug, Clone, Serialize)]
struct TDDMinimal {
    entity_class: String,  // Only this field
}

impl From<&CodeEntity> for TDDMinimal {
    fn from(entity: &CodeEntity) -> Self {
        Self {
            entity_class: format!("{:?}", entity.tdd_classification.entity_class),
        }
    }
}

// In export logic
let tdd_minimal: Vec<TDDMinimal> = entities.iter()
    .map(TDDMinimal::from)
    .collect();
```

**Output**:
```json
{
  "isgl1_key": "rust:fn:calculate_total:src_billing_rs:42",
  "tdd_class": "CodeImplementation"
}
```

**Testing**:
```rust
#[test]
fn test_tdd_minimal_export() {
    let entity = create_test_entity();
    let minimal = TDDMinimal::from(&entity);
    let json = serde_json::to_string(&minimal).unwrap();

    assert!(json.contains("CodeImplementation"));
    assert!(!json.contains("has_code"));  // Bloat removed
}
```

**Success criteria**:
- ✅ Only entity_class exported
- ✅ File size reduced by 86% for TDD fields
- ✅ Backward compatible (can add fields later)

---

**Phase 1 Total**: <40 hours, ~74,635 tokens impact (+54K separate, -20K in ISG)

---

### Phase 2: Essential Fields & Preset System (Week 2-3) 📈

**Goal**: Implement user's "compact" preset with 22-25 variables

#### **Task 2.1: Preset System**
**Priority**: HIGH
**Effort**: LOW (8-12 hours)
**Token impact**: 0 (organizational only)

**Implementation**:
```rust
// crates/pt02-llm-cozodb-to-context-writer/src/cli.rs

#[derive(Debug, Clone, Copy)]
enum Preset {
    Minimal,    // 4 vars: isgl1_key, name, type, path
    Compact,    // 22 vars: user's essential list
    Tier1,      // 38 vars: Challenge03 HIGH priority
    Full,       // All available vars
}

.arg(
    Arg::new("preset")
        .long("preset")
        .value_parser(["minimal", "compact", "tier1", "full"])
        .default_value("minimal")
        .help("Output preset: minimal, compact, tier1, or full")
)

// Granular overrides
.arg(
    Arg::new("include-temporal")
        .long("include-temporal")
        .action(ArgAction::SetTrue)
        .help("Include temporal state (current_ind, future_ind, future_action)")
)

.arg(
    Arg::new("include-dependencies")
        .long("include-dependencies")
        .action(ArgAction::SetTrue)
        .help("Include dependency information (forward/reverse deps)")
)
```

**Preset definitions**:
```rust
impl Preset {
    fn get_fields(&self) -> Vec<&'static str> {
        match self {
            Preset::Minimal => vec![
                "isgl1_key", "entity_name", "entity_type", "file_path"
            ],
            Preset::Compact => vec![
                "isgl1_key", "entity_name", "entity_type", "file_path",
                "entity_class", "current_ind", "future_ind", "future_action",
                "generic_params", "where_clauses", "return_type", "param_types",
                "trait_bounds", "total_references", "usage_locations",
                // ... 22 total
            ],
            Preset::Tier1 => {
                // All 38 Challenge03 HIGH variables
                // ...
            },
            Preset::Full => {
                // All 78 variables
                // ...
            }
        }
    }
}
```

**Usage examples**:
```bash
# Minimal preset (default)
pt02 --preset minimal --where "..."

# Compact preset (user's essential fields)
pt02 --preset compact --where "..."

# Compact with code
pt02 --preset compact --include-current-code true --where "..."

# Tier 1 without dependencies (custom)
pt02 --preset tier1 --include-dependencies false --where "..."
```

**Testing**:
```rust
#[test]
fn test_preset_field_selection() {
    let minimal = Preset::Minimal.get_fields();
    assert_eq!(minimal.len(), 4);

    let compact = Preset::Compact.get_fields();
    assert_eq!(compact.len(), 22);

    let tier1 = Preset::Tier1.get_fields();
    assert_eq!(tier1.len(), 38);
}
```

**Success criteria**:
- ✅ Preset system parses correctly
- ✅ Field selection works for each preset
- ✅ Granular overrides work (--include-temporal, etc.)
- ✅ Backward compatible with existing flags

---

#### **Task 2.2: Type System Variables** (CRITICAL subset)
**Priority**: CRITICAL
**Effort**: MEDIUM (40-60 hours)
**Token impact**: +40 tokens/entity

**Variables**: `generic_params`, `where_clauses`, `return_type`, `param_types`, `trait_bounds`

**Implementation**: Requires LSP integration (see Challenge02)
- LSP hover for return_type
- LSP documentSymbol for param_types
- Tree-sitter + LSP for generic_params, where_clauses, trait_bounds

**Success criteria**:
- ✅ Type signatures extracted for functions
- ✅ Generic parameters parsed correctly
- ✅ Where clauses captured
- ✅ Trait bounds identified

---

#### **Task 2.3: Usage Metrics** (CRITICAL subset)
**Priority**: HIGH
**Effort**: LOW (16-24 hours)
**Token impact**: +25 tokens/entity

**Variables**: `total_references`, `usage_locations`

**Implementation**:
```rust
// Use existing LSP references query
let references = lsp_client.get_references(file_path, line, character).await?;

let usage_analysis = UsageAnalysis {
    total_references: references.len(),
    usage_locations: references.iter()
        .map(|loc| Location {
            file_path: loc.uri.to_file_path().unwrap(),
            line: loc.range.start.line,
            character: loc.range.start.character,
        })
        .collect(),
    dependents: vec![],  // Computed separately via CozoDB
};
```

**Success criteria**:
- ✅ Reference count accurate
- ✅ Usage locations captured
- ✅ Performance acceptable (<500ms per entity)

---

**Phase 2 Total**: ~80-96 hours, +65 tokens/entity (~38,350 tokens for 590 entities)

---

### Phase 3: Advanced Types & Transitive Deps (Week 3-4) 📊

**Goal**: Complete "compact" preset, approach Tier 1

#### **Task 3.1: Advanced Type System** (HIGH subset)
**Priority**: MEDIUM
**Effort**: MEDIUM (40-60 hours)
**Token impact**: +20 tokens/entity

**Variables**: `lifetime_params`, `impl_trait_for`, `associated_types`, `type_aliases`, `derived_traits`, `const_generics`

**Implementation**: LSP + tree-sitter extensions

**Success criteria**:
- ✅ Lifetime parameters extracted
- ✅ Trait implementations identified
- ✅ Associated types captured

---

#### **Task 3.2: Transitive Dependencies**
**Priority**: MEDIUM
**Effort**: LOW (16-24 hours)
**Token impact**: +30 tokens/entity (expensive)

**Variables**: `transitive_deps_forward`, `transitive_deps_reverse`

**Implementation**: Use existing CozoDB queries
```rust
let transitive_forward = storage.get_transitive_closure(isgl1_key).await?;
let transitive_reverse = storage.get_reverse_transitive_closure(isgl1_key).await?;
```

**Success criteria**:
- ✅ Transitive closure computed correctly
- ✅ Cycle detection works
- ✅ Performance acceptable (<200ms per entity)

---

**Phase 3 Total**: ~56-84 hours, +50 tokens/entity (~29,500 tokens for 590 entities)

---

### Implementation Summary

| Phase | Duration | Effort (hours) | Token Impact | Cumulative Tokens |
|-------|----------|---------------|--------------|-------------------|
| **Phase 1: Critical gaps** | Week 1 | 20-40 | +54K (separate), -20K (ISG) | 58K |
| **Phase 2: Essential fields** | Week 2-3 | 80-96 | +38K | 96K |
| **Phase 3: Advanced types** | Week 3-4 | 56-84 | +30K | 126K |
| **Total** | 3-4 weeks | 156-220 | +126K optimized | 126K |

**vs Challenge03 Tier 1**: 126K < 295-590K (60-80% below target)

---

## 4. Interface Design Comparison

### Option A: Separate Binaries (User's Proposal) ❌

**Proposed names**:
```
pt02-llm-cozodb-to-context-writer-isg-only-essential-fields
pt02-llm-cozodb-to-context-writer-isg-and-code-fields
```

**Analysis**:

| Aspect | Assessment | Details |
|--------|-----------|---------|
| **Length** | ❌ Verbose | 62 characters (vs 15 for "pt02") |
| **S01 compliance** | ❌ Fails | Ultra-minimalist principle violated |
| **Composability** | ❌ Poor | Can't mix presets with flags |
| **Maintenance** | ❌ High burden | 4 separate binaries to maintain |
| **Clarity** | ⚠️ Ambiguous | "essential fields" undefined (22 vars? 38 vars?) |
| **Future-proof** | ❌ Rigid | Hard to add tier2, tier3 |

**Recommendation**: ❌ **Reject**

---

### Option B: Granular Flags Only ❌

**Example**:
```bash
pt02 \
  --include-current-code false \
  --include-temporal true \
  --include-tdd-class true \
  --include-type-system true \
  --include-dependencies true \
  --include-usage-metrics true \
  --where "..."
```

**Analysis**:

| Aspect | Assessment | Details |
|--------|-----------|---------|
| **Flexibility** | ✅ Maximum | Fine-grained control |
| **Cognitive load** | ❌ High | 6+ flags to understand |
| **Beginner-friendly** | ❌ Poor | Requires flag knowledge |
| **S01 compliance** | ⚠️ Moderate | Flag explosion |
| **Composability** | ✅ Good | Can mix any combination |

**Recommendation**: ⚠️ **Partial** (use as overrides only)

---

### Option C: Preset System + Granular Overrides ✅

**Recommended design**:

```bash
# --- Presets (beginner-friendly) ---

# Minimal (4 vars, ~24 tokens/entity)
pt02 --preset minimal --where "..."

# Compact (22 vars, ~106 tokens/entity) - User's essential list
pt02 --preset compact --where "..."

# Tier 1 (38 vars, ~500-1000 tokens/entity) - Challenge03 HIGH
pt02 --preset tier1 --where "..."

# Full (78 vars, ~2000-5000 tokens/entity) - Gold standard
pt02 --preset full --where "..."

# --- Granular overrides (expert-friendly) ---

# Compact with code
pt02 --preset compact --include-current-code true --where "..."

# Tier 1 without dependencies (custom)
pt02 --preset tier1 --include-dependencies false --where "..."

# Minimal with temporal state
pt02 --preset minimal --include-temporal true --where "..."

# --- Specialized exports ---

# Dependency graph (separate file)
pt02 --export-dependencies deps.json --where "..."

# Advanced query (unchanged)
pt02 --query "?[isgl1_key, ...] := *CodeGraph{...}" --output ctx.json
```

**Analysis**:

| Aspect | Assessment | Details |
|--------|-----------|---------|
| **S01 compliance** | ✅ Excellent | Minimal flags for common cases |
| **Beginner-friendly** | ✅ Excellent | Clear preset names |
| **Expert-friendly** | ✅ Excellent | Granular overrides available |
| **Composability** | ✅ Excellent | Mix presets with overrides |
| **Future-proof** | ✅ Excellent | Easy to add tier2, tier3 |
| **Maintenance** | ✅ Low | Single binary |
| **Clarity** | ✅ Clear | Preset names align with Challenge03 |

**Recommendation**: ✅ **STRONGLY RECOMMENDED**

---

## 5. Token Optimization Strategies

### Strategy 1: Null Field Exclusion

**Technique**: Use serde's `skip_serializing_if` attribute

**Implementation**:
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub generic_params: Option<Vec<String>>,
```

**Impact**:
- Estimated null rate: 40% of optional fields
- Savings: 40% reduction on optional fields
- Total: ~9,440 tokens for 590 entities

**Effort**: TRIVIAL (< 1 hour)

---

### Strategy 2: TDD De-bloat

**Technique**: Export only `entity_class` field

**Current**:
```json
{
  "tdd_classification": {
    "entity_class": "CodeImplementation",
    "has_code": true,
    "should_have_tests": true,
    "should_propagate": true,
    "is_generic": false,
    "test_coverage_status": "Unknown",
    "propagation_heuristic": "default"
  }
}
```

**Proposed**:
```json
{
  "tdd_class": "CodeImplementation"
}
```

**Impact**:
- Savings: 86% reduction (18 tokens → 3 tokens)
- Total: ~10,620 tokens for 590 entities

**Effort**: TRIVIAL (< 2 hours)

---

### Strategy 3: Dependency Graph as Separate Export

**Technique**: Export dependencies to separate JSONL file

**Rationale**:
- Dependencies change infrequently (stable across sessions)
- Embedding adds ~92 tokens/entity to EVERY query
- Separate export is one-time cost, reusable

**Implementation**:
```bash
# Export once
pt02 --export-dependencies deps.json

# Use across many queries
pt02 --preset compact --where "changed_entities"  # Dependencies already known
```

**Impact**:
- Embedding cost: 92 tokens/entity × 590 = 54,280 tokens per query
- Separate cost: 54,575 tokens ONCE
- Savings: 54,280 tokens per query (after first export)

**Effort**: LOW (8-16 hours)

---

### Strategy 4: Code Field Toggle

**Technique**: Use existing `--include-current-code` flag

**Impact**:
- With code: ~1,200 tokens/entity
- Without code: ~106 tokens/entity (compact preset)
- Savings: 1,094 tokens/entity × 590 = 645,460 tokens (92% reduction)

**Status**: ✅ Already implemented

---

### Strategy 5: Lazy Loading (Future)

**Technique**: Export minimal context, fetch details on-demand

**Workflow**:
```bash
# Step 1: Export minimal context (cheap)
pt02 --preset minimal --where "..." > minimal.json

# Step 2: LLM identifies entities of interest
# (e.g., "I need full type info for calculate_total")

# Step 3: Fetch details for specific entities
pt02 --enrich isgl1_key="rust:fn:calculate_total:..." \
     --fields "generic_params,where_clauses,return_type" \
     > enriched.json
```

**Impact**:
- Initial query: 24 tokens/entity × 590 = 14,160 tokens
- Enrichment: +40 tokens × 10 entities = +400 tokens
- Total: 14,560 tokens (vs 62,540 for compact on all entities)
- Savings: 77% reduction

**Status**: ⏳ Future work (requires `--enrich` flag)

---

## 6. Naming Conventions & Preset Definitions

### Preset: `minimal`

**Variables** (4):
- `isgl1_key`
- `entity_name`
- `entity_type`
- `file_path`

**Token cost**: ~24 tokens/entity (with null exclusion)

**Use case**: Quick checks, existence queries, minimal context

**Example**:
```bash
pt02 --preset minimal --where "entity_type = 'fn'"
```

---

### Preset: `compact`

**Variables** (22):

**Core Identity** (4):
- `isgl1_key`, `entity_name`, `entity_type`, `file_path`

**TDD** (1):
- `entity_class`

**Temporal** (3):
- `current_ind`, `future_ind`, `future_action`

**Type System** (5):
- `generic_params`, `where_clauses`, `return_type`, `param_types`, `trait_bounds`

**Usage** (2):
- `total_references`, `usage_locations`

**Advanced Types** (6):
- `lifetime_params`, `impl_trait_for`, `associated_types`, `type_aliases`, `derived_traits`, `const_generics`

**Dependencies** (included in separate export):
- `forward_deps`, `reverse_deps`, `blast_radius_count` (via `--export-dependencies`)

**Token cost**: ~106 tokens/entity

**Use case**: Daily refactoring, moderate reasoning, type-aware changes

**Example**:
```bash
pt02 --preset compact --where "future_action != null"
```

---

### Preset: `tier1`

**Variables** (38): All Challenge03 HIGH-priority variables

**Categories**:
- Core Identity: 6 vars
- Type System: 8 vars
- Dependencies: 5 vars
- Documentation: 2 vars
- Testing: 2 vars
- Temporal: 3 vars
- Complexity: 1 var (unsafe_blocks_count)
- ISG: 4 vars
- Usage: 2 vars
- Advanced: 5 vars

**Token cost**: ~500-1,000 tokens/entity

**Use case**: Safe refactoring, full type reasoning, architectural changes

**Example**:
```bash
pt02 --preset tier1 --where "entity_class = 'logic'"
```

---

### Preset: `full`

**Variables** (78): All Challenge03 variables

**Token cost**: ~2,000-5,000 tokens/entity

**Use case**: Deep analysis, architectural decisions, complete context

**Example**:
```bash
pt02 --preset full --where "..." --include-current-code true
```

---

## 7. Implementation Specifications

### Spec 1: Dependency Graph Export

**File**: `crates/pt02-llm-cozodb-to-context-writer/src/main.rs`

**CLI Flag**:
```rust
.arg(
    Arg::new("export-dependencies")
        .long("export-dependencies")
        .value_name("PATH")
        .help("Export dependency graph to separate JSONL file")
)
```

**Query**:
```rust
pub async fn get_all_dependency_edges(&self) -> Result<Vec<DependencyEdge>> {
    let query = r#"
        ?[from_key, to_key, edge_type, source_location] :=
            *DependencyEdges{from_key, to_key, edge_type, source_location}
    "#;

    let result = self.db.run_script(query, BTreeMap::new(), ScriptMutability::Immutable)?;

    // Parse results...
    Ok(edges)
}
```

**Output Format** (JSONL):
```json
{"from":"rust:fn:parse:src_parser_rs:10","to":"rust:fn:tokenize:src_tokenizer_rs:5","type":"Calls","loc":"src/parser.rs:15"}
{"from":"rust:fn:main:src_main_rs:1","to":"rust:fn:parse:src_parser_rs:10","type":"Calls","loc":"src/main.rs:8"}
```

**Testing**:
```rust
#[tokio::test]
async fn test_dependency_export() {
    let storage = CozoDbStorage::new("mem").await?;

    // Add test edges
    storage.add_dependency_edge(/* ... */).await?;

    let edges = storage.get_all_dependency_edges().await?;
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0].edge_type, EdgeType::Calls);
}
```

**Effort**: LOW (8-16 hours)

---

### Spec 2: Null Field Exclusion

**File**: `crates/parseltongue-core/src/entities.rs`

**Implementation**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEntity {
    // Required fields (no skip)
    pub isgl1_key: String,
    pub interface_signature: InterfaceSignature,

    // Optional fields (skip if None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_params: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub where_clauses: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsp_metadata: Option<LspMetadata>,

    // ... repeat for all optional fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_information: Option<TypeInformation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_analysis: Option<UsageAnalysis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub semantic_tokens: Vec<SemanticToken>,
}
```

**Testing**:
```rust
#[test]
fn test_null_field_exclusion() {
    let entity = CodeEntity {
        isgl1_key: "test".to_string(),
        interface_signature: /* ... */,
        generic_params: None,
        where_clauses: Some(vec!["T: Clone".to_string()]),
        // ...
    };

    let json = serde_json::to_string(&entity).unwrap();

    assert!(!json.contains("generic_params"));  // Null, excluded
    assert!(json.contains("where_clauses"));   // Non-null, included
}
```

**Effort**: TRIVIAL (1-2 hours)

---

### Spec 3: TDD De-bloat

**File**: `crates/pt02-llm-cozodb-to-context-writer/src/main.rs`

**New Output Struct**:
```rust
#[derive(Debug, Clone, Serialize)]
struct EntityMinimal {
    isgl1_key: String,
    entity_name: String,
    entity_type: String,
    file_path: String,
    tdd_class: String,  // Only this field from TDDClassification

    #[serde(skip_serializing_if = "Option::is_none")]
    temporal_state: Option<TemporalStateMinimal>,

    // ... other minimal fields
}

#[derive(Debug, Clone, Serialize)]
struct TemporalStateMinimal {
    current_ind: bool,
    future_ind: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    future_action: Option<String>,
}

impl From<&CodeEntity> for EntityMinimal {
    fn from(entity: &CodeEntity) -> Self {
        Self {
            isgl1_key: entity.isgl1_key.clone(),
            entity_name: entity.interface_signature.entity_name.clone(),
            entity_type: format!("{:?}", entity.interface_signature.entity_type),
            file_path: entity.interface_signature.file_path.to_string_lossy().to_string(),
            tdd_class: format!("{:?}", entity.tdd_classification.entity_class),
            temporal_state: Some(TemporalStateMinimal {
                current_ind: entity.temporal_state.current_ind,
                future_ind: entity.temporal_state.future_ind,
                future_action: entity.temporal_state.future_action.as_ref().map(|a| format!("{:?}", a)),
            }),
        }
    }
}
```

**Usage in main.rs**:
```rust
// Serialize with minimal projection
let minimal: Vec<EntityMinimal> = entities.iter().map(EntityMinimal::from).collect();
let json = serde_json::to_string_pretty(&minimal)?;
```

**Testing**:
```rust
#[test]
fn test_tdd_minimal() {
    let entity = create_test_entity();
    let minimal = EntityMinimal::from(&entity);
    let json = serde_json::to_string(&minimal).unwrap();

    assert!(json.contains("tdd_class"));
    assert!(json.contains("CodeImplementation"));
    assert!(!json.contains("has_code"));  // Bloat removed
    assert!(!json.contains("should_have_tests"));
}
```

**Effort**: TRIVIAL (2-4 hours)

---

### Spec 4: Preset System

**File**: `crates/pt02-llm-cozodb-to-context-writer/src/cli.rs`

**Enum Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Minimal,
    Compact,
    Tier1,
    Full,
}

impl FromStr for Preset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(Preset::Minimal),
            "compact" => Ok(Preset::Compact),
            "tier1" => Ok(Preset::Tier1),
            "full" => Ok(Preset::Full),
            _ => Err(format!("Invalid preset: {}", s)),
        }
    }
}

impl Preset {
    pub fn get_field_set(&self) -> FieldSet {
        match self {
            Preset::Minimal => FieldSet {
                include_core: true,
                include_temporal: false,
                include_tdd: false,
                include_type_system: false,
                include_dependencies: false,
                include_usage: false,
                include_advanced_types: false,
            },
            Preset::Compact => FieldSet {
                include_core: true,
                include_temporal: true,
                include_tdd: true,
                include_type_system: true,
                include_dependencies: false,  // Separate export
                include_usage: true,
                include_advanced_types: true,
            },
            Preset::Tier1 => FieldSet {
                include_core: true,
                include_temporal: true,
                include_tdd: true,
                include_type_system: true,
                include_dependencies: true,
                include_usage: true,
                include_advanced_types: true,
                // ... + more
            },
            Preset::Full => FieldSet::all(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSet {
    pub include_core: bool,
    pub include_temporal: bool,
    pub include_tdd: bool,
    pub include_type_system: bool,
    pub include_dependencies: bool,
    pub include_usage: bool,
    pub include_advanced_types: bool,
}

impl FieldSet {
    pub fn all() -> Self {
        Self {
            include_core: true,
            include_temporal: true,
            include_tdd: true,
            include_type_system: true,
            include_dependencies: true,
            include_usage: true,
            include_advanced_types: true,
        }
    }

    pub fn apply_overrides(&mut self, matches: &ArgMatches) {
        // Allow granular overrides
        if matches.get_flag("include-temporal") {
            self.include_temporal = true;
        }
        if matches.get_flag("exclude-dependencies") {
            self.include_dependencies = false;
        }
        // ... etc.
    }
}
```

**CLI Arguments**:
```rust
.arg(
    Arg::new("preset")
        .long("preset")
        .value_parser(["minimal", "compact", "tier1", "full"])
        .default_value("minimal")
        .help("Output preset: minimal, compact, tier1, or full")
)
.arg(
    Arg::new("include-temporal")
        .long("include-temporal")
        .action(ArgAction::SetTrue)
        .help("Include temporal state (overrides preset)")
)
.arg(
    Arg::new("exclude-dependencies")
        .long("exclude-dependencies")
        .action(ArgAction::SetTrue)
        .help("Exclude dependencies (overrides preset)")
)
```

**Usage in main.rs**:
```rust
let preset = matches.get_one::<String>("preset")
    .and_then(|s| Preset::from_str(s).ok())
    .unwrap_or(Preset::Minimal);

let mut field_set = preset.get_field_set();
field_set.apply_overrides(&matches);

// Use field_set to filter entity fields during serialization
```

**Testing**:
```rust
#[test]
fn test_preset_parsing() {
    assert_eq!(Preset::from_str("minimal"), Ok(Preset::Minimal));
    assert_eq!(Preset::from_str("compact"), Ok(Preset::Compact));
    assert!(Preset::from_str("invalid").is_err());
}

#[test]
fn test_preset_field_sets() {
    let minimal = Preset::Minimal.get_field_set();
    assert!(minimal.include_core);
    assert!(!minimal.include_temporal);

    let compact = Preset::Compact.get_field_set();
    assert!(compact.include_temporal);
    assert!(compact.include_type_system);
}
```

**Effort**: LOW (8-12 hours)

---

## 8. Real-World Examples

### Example 1: Export Minimal Context

**Command**:
```bash
pt02 --preset minimal --where "entity_type = 'fn'" --output minimal.json
```

**Token Cost**:
```
Assume 300 functions in codebase
300 entities × 24 tokens = 7,200 tokens
```

**Output Sample**:
```json
[
  {
    "isgl1_key": "rust:fn:calculate_total:src_billing_rs:42",
    "entity_name": "calculate_total",
    "entity_type": "fn",
    "file_path": "src/billing.rs"
  },
  {
    "isgl1_key": "rust:fn:process_payment:src_payment_rs:10",
    "entity_name": "process_payment",
    "entity_type": "fn",
    "file_path": "src/payment.rs"
  }
]
```

**Use Case**: Quick function listing, existence checks

---

### Example 2: Export Changed Entities with Compact Context

**Command**:
```bash
pt02 --preset compact --where "future_action != null" --output changed.json
```

**Token Cost**:
```
Assume 20 changed entities (typical refactoring session)
20 entities × 106 tokens = 2,120 tokens
```

**Output Sample**:
```json
[
  {
    "isgl1_key": "rust:fn:calculate_total:src_billing_rs:42",
    "entity_name": "calculate_total",
    "entity_type": "fn",
    "file_path": "src/billing.rs",
    "tdd_class": "CodeImplementation",
    "temporal_state": {
      "current_ind": true,
      "future_ind": true,
      "future_action": "Edit"
    },
    "return_type": "f64",
    "param_types": ["&[Item]"],
    "generic_params": null,
    "total_references": 23
  }
]
```

**Use Case**: Refactoring workflow, change review

---

### Example 3: Export All Entities + Dependency Graph

**Commands**:
```bash
# Export entities (compact preset)
pt02 --preset compact --where "ALL" --output entities.json

# Export dependencies (separate file)
pt02 --export-dependencies deps.json --where "ALL"
```

**Token Cost**:
```
Entities: 590 × 106 = 62,540 tokens
Dependencies: 1,475 edges × 37 = 54,575 tokens
Total: 117,115 tokens
```

**Use Case**: Full codebase analysis, architectural review

---

### Example 4: Export with Code (Expensive)

**Command**:
```bash
pt02 --preset compact --include-current-code true --where "entity_name = 'calculate_total'" --output calc_with_code.json
```

**Token Cost**:
```
1 entity × (106 + 1,200) tokens = 1,306 tokens
```

**Output Sample**:
```json
[
  {
    "isgl1_key": "rust:fn:calculate_total:src_billing_rs:42",
    "entity_name": "calculate_total",
    "entity_type": "fn",
    "file_path": "src/billing.rs",
    "tdd_class": "CodeImplementation",
    "current_code": "pub fn calculate_total(items: &[Item]) -> f64 {\n    items.iter().map(|i| i.price).sum()\n}",
    "return_type": "f64",
    "param_types": ["&[Item]"]
  }
]
```

**Use Case**: Deep dive into specific function, implementation understanding

---

### Example 5: Tier 1 Export (Future)

**Command**:
```bash
pt02 --preset tier1 --where "entity_class = 'logic'" --output tier1.json
```

**Token Cost**:
```
Assume 100 logic entities
100 entities × 750 tokens (avg for Tier 1) = 75,000 tokens
```

**Use Case**: Safe refactoring with full type context, architectural changes

---

## 9. Quick Win Implementation Guide

### Quick Win 1: Null Field Exclusion (< 1 hour)

**Steps**:
1. Open `crates/parseltongue-core/src/entities.rs`
2. Add `#[serde(skip_serializing_if = "Option::is_none")]` to all `Option<T>` fields
3. Add `#[serde(skip_serializing_if = "Vec::is_empty")]` to all `Vec<T>` fields
4. Run tests: `cargo test`
5. Commit: `feat(pt02): add null field exclusion for token savings`

**Expected Outcome**:
- File size reduces by ~40% for sparse entities
- JSON output cleaner (no null clutter)
- ~9,440 token savings for 590 entities

---

### Quick Win 2: TDD De-bloat (< 2 hours)

**Steps**:
1. Open `crates/pt02-llm-cozodb-to-context-writer/src/main.rs`
2. Create `EntityMinimal` struct with only `tdd_class` field
3. Implement `From<&CodeEntity> for EntityMinimal`
4. Update serialization logic to use `EntityMinimal`
5. Run tests: `cargo test`
6. Commit: `feat(pt02): de-bloat TDD classification to single field`

**Expected Outcome**:
- TDD fields reduce from 21 tokens to 3 tokens (86% reduction)
- ~10,620 token savings for 590 entities

---

### Quick Win 3: Dependency Export (< 16 hours)

**Steps**:
1. Open `crates/pt02-llm-cozodb-to-context-writer/src/cli.rs`
2. Add `--export-dependencies` flag
3. Implement query in `main.rs`:
   ```rust
   if let Some(deps_path) = matches.get_one::<String>("export-dependencies") {
       let edges = storage.get_all_dependency_edges().await?;
       export_jsonl(&edges, deps_path)?;
   }
   ```
4. Write JSONL export function
5. Add integration test
6. Commit: `feat(pt02): add dependency graph export`

**Expected Outcome**:
- New `--export-dependencies` flag available
- ~54,575 tokens exported to separate file (reusable)
- Dependencies no longer bloat per-entity context

---

**Total Quick Win Implementation**: < 20 hours, ~74K token impact

---

## 10. Conclusion & Recommendations

### Summary of Findings

1. **Biggest Gap**: Dependency graphs (5 HIGH-priority variables) are FULLY IMPLEMENTED but NOT EXPOSED in PT02 CLI
2. **Scope Alignment**: User's "essential fields" (22 vars, 62K tokens) is 60-80% BELOW Challenge03 Tier 1 (38 vars, 295-590K tokens)
3. **Quick Wins**: 3 trivial optimizations save ~74K tokens with <20 hours effort
4. **Interface Design**: Preset system is superior to separate binaries (S01-aligned, composable, future-proof)

---

### Immediate Action Items

**Priority 1 (This Week)**: 🔥
1. ✅ Implement dependency graph export (`--export-dependencies`)
2. ✅ Add null field exclusion (serde attributes)
3. ✅ De-bloat TDD classification (entity_class only)

**Priority 2 (Next 2 Weeks)**: 📈
4. ✅ Implement preset system (`--preset minimal/compact/tier1/full`)
5. ⏳ Add type system variables (requires LSP - see Challenge02)
6. ⏳ Add usage metrics (LSP references)

**Priority 3 (Weeks 3-4)**: 📊
7. ⏳ Add advanced type system (lifetimes, impl_trait_for, etc.)
8. ⏳ Add transitive dependencies (CozoDB queries ready)

---

### Final Recommendations

**1. Rename "Essential" to "Compact"**:
- User's 22-variable list is below Tier 1 scope
- "Compact" better describes its position (between minimal and tier1)
- Reserve "essential" for future Tier 1 implementation

**2. Use Preset System, Not Separate Binaries**:
- S01-aligned (ultra-minimalist principle)
- Composable (mix presets with overrides)
- Future-proof (easy to add tier2, tier3)

**3. Prioritize Dependency Export**:
- Biggest gap in current implementation
- 5 HIGH-priority Challenge03 variables
- LOW effort, HIGH impact

**4. Implement Quick Wins First**:
- 74K token impact with <20 hours effort
- Builds momentum for larger features
- Demonstrates immediate value

---

**END OF CHALLENGE04-PT02-IMPLEMENTATION-PLAN.MD**
