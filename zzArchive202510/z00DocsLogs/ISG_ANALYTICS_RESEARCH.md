# ISG Analytics & Visualization Research
## Comprehensive Analysis for pt07-cozodb-code-as-visuals

**Date**: 2025-11-01
**Version**: 1.0
**Target**: CLI tool for actionable codebase insights from ISG data

---

## Executive Summary

This document provides comprehensive research on analytics and visualizations that can be created from Interface Signature Graph (ISG) data stored in CozoDB. The goal is to design **pt07-cozodb-code-as-visuals** - a CLI tool that helps developers make decisions through actionable insights, not just pretty charts.

### Key Findings

1. **Rich Data Available**: ISG entities contain 8+ dimensions for analysis (temporal state, complexity, dependencies, entity types, TDD classification, visibility, risk levels)
2. **Graph-First Analytics**: CozoDB's Datalog enables powerful dependency analysis (blast radius, transitive closure, reverse dependencies)
3. **Terminal-First Design**: Rust ecosystem has mature libraries for ASCII tables, sparklines, and text-based charts
4. **Actionable Over Aesthetic**: Focus on metrics that answer "what should I work on?" and "where are the risks?"

---

## Part 1: Analytics Taxonomy

### 1.1 Entity Distribution Analytics

**What to Measure**: Breakdown of codebase composition by entity types

**Dimensions**:
- Entity Type (Function, Struct, Trait, Enum, Module, ImplBlock)
- Entity Class (CodeImplementation vs TestImplementation)
- Visibility (Public, Private, Crate, Module)
- Complexity Level (Simple, Moderate, Complex)
- Testability Level (High, Medium, Low)
- Risk Level (Low, Medium, High)

**Example Queries**:

```datalog
# Count entities by type
?[entity_type, count] := *CodeGraph{entity_type},
                         count = count(entity_type)
:order count desc

# Distribution by complexity
?[complexity, count] := *CodeGraph{TDD_Classification},
                        complexity = json_extract(TDD_Classification, '$.complexity'),
                        count = count(complexity)

# Public vs Private breakdown
?[visibility, count] := *CodeGraph{interface_signature},
                        visibility = json_extract(interface_signature, '$.visibility'),
                        count = count(visibility)
```

**Actionable Insights**:
- High ratio of Complex entities → code health issue
- Low TestImplementation count → coverage gap
- High Private/Low Public ratio → potential over-engineering

---

### 1.2 Temporal State Analytics

**What to Measure**: Change activity and pending modifications

**Dimensions**:
- Temporal Action (Create, Edit, Delete, Unchanged)
- Change Count by File
- Change Count by Entity Type
- Change Velocity (entities modified per session)

**Example Queries**:

```datalog
# All pending changes
?[ISGL1_key, Future_Action, file_path] :=
    *CodeGraph{ISGL1_key, Future_Action, file_path},
    Future_Action != null
:order file_path

# Change distribution by action type
?[action, count] := *CodeGraph{Future_Action},
                    Future_Action != null,
                    count = count(action)

# Files with most changes
?[file_path, change_count] :=
    *CodeGraph{file_path, Future_Action},
    Future_Action != null,
    change_count = count(file_path)
:order change_count desc
```

**Actionable Insights**:
- Files with 5+ pending changes → high-risk refactor zone
- Many Deletes → architectural shift in progress
- Many Creates in one file → new feature development

---

### 1.3 Complexity & Risk Analytics

**What to Measure**: Code health indicators and technical debt

**Dimensions**:
- Complexity Distribution (Simple vs Moderate vs Complex)
- Risk-Complexity Matrix (High Risk + Complex = danger zone)
- Testability Gaps (Low testability entities)
- Critical Path Entities (critical_path=true)

**Example Queries**:

```datalog
# High-risk complex entities (DANGER ZONE)
?[ISGL1_key, name, complexity, risk] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    complexity = json_extract(TDD_Classification, '$.complexity'),
    risk = json_extract(TDD_Classification, '$.change_risk'),
    complexity == "Complex",
    risk == "High",
    name = json_extract(interface_signature, '$.name')

# Entities on critical path
?[ISGL1_key, name, testability] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    critical = json_extract(TDD_Classification, '$.critical_path'),
    critical == true,
    name = json_extract(interface_signature, '$.name'),
    testability = json_extract(TDD_Classification, '$.testability')

# Low testability code (technical debt)
?[ISGL1_key, name, testability, complexity] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    testability = json_extract(TDD_Classification, '$.testability'),
    testability == "Low",
    complexity = json_extract(TDD_Classification, '$.complexity'),
    name = json_extract(interface_signature, '$.name')
```

**Actionable Insights**:
- Complex + High Risk + Low Testability → immediate refactor candidate
- Critical path entities with Low testability → testing priority
- Growing count of Complex entities → architecture review needed

---

### 1.4 Test Coverage Analytics

**What to Measure**: Testing gaps and coverage estimates

**Dimensions**:
- Test/Code Ratio (TestImplementation count / CodeImplementation count)
- Test Coverage Estimate (average of test_coverage_estimate field)
- Untested Public APIs (Public functions with 0% coverage)
- Test Organization (test file locations)

**Example Queries**:

```datalog
# Test vs Code ratio
?[entity_class, count] :=
    *CodeGraph{TDD_Classification},
    entity_class = json_extract(TDD_Classification, '$.entity_class'),
    count = count(entity_class)

# Average test coverage
?[avg_coverage] :=
    *CodeGraph{TDD_Classification},
    coverage = json_extract(TDD_Classification, '$.test_coverage_estimate'),
    avg_coverage = mean(coverage)

# Public APIs with zero coverage
?[ISGL1_key, name] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    visibility = json_extract(interface_signature, '$.visibility'),
    visibility == "Public",
    coverage = json_extract(TDD_Classification, '$.test_coverage_estimate'),
    coverage == 0.0,
    name = json_extract(interface_signature, '$.name')
```

**Actionable Insights**:
- Test ratio < 0.3 → insufficient test coverage
- Public APIs with 0% coverage → integration risk
- High coverage variance → inconsistent testing practices

---

### 1.5 Dependency & Graph Analytics

**What to Measure**: Code coupling and architectural insights

**Dimensions**:
- Dependency Count Distribution
- Blast Radius (entities affected by changes)
- Dependency Depth (transitive closure size)
- Coupling Metrics (fan-in, fan-out)
- Circular Dependencies

**Example Queries**:

```datalog
# High-dependency entities (coupling hotspots)
?[ISGL1_key, name, dep_count] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    dep_count = json_extract(TDD_Classification, '$.dependencies'),
    dep_count > 5,
    name = json_extract(interface_signature, '$.name')
:order dep_count desc

# Average dependencies by entity type
?[entity_type, avg_deps] :=
    *CodeGraph{entity_type, TDD_Classification},
    deps = json_extract(TDD_Classification, '$.dependencies'),
    avg_deps = mean(deps)

# Entities with zero dependencies (leaf nodes)
?[ISGL1_key, name] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    deps = json_extract(TDD_Classification, '$.dependencies'),
    deps == 0,
    name = json_extract(interface_signature, '$.name')
```

**Graph-Specific Queries** (using DependencyEdges table):

```datalog
# Most-called functions (high fan-in)
?[to_key, call_count] :=
    *DependencyEdges{to_key, edge_type},
    edge_type == "Calls",
    call_count = count(to_key)
:order call_count desc
:limit 20

# Functions that call many others (high fan-out)
?[from_key, call_count] :=
    *DependencyEdges{from_key, edge_type},
    edge_type == "Calls",
    call_count = count(from_key)
:order call_count desc
:limit 20

# Circular dependency detection (simplified)
?[key1, key2] :=
    *DependencyEdges{from_key: key1, to_key: key2},
    *DependencyEdges{from_key: key2, to_key: key1},
    key1 < key2  # Avoid duplicates
```

**Actionable Insights**:
- Entities with 10+ dependencies → refactoring candidate
- High fan-in functions → core utilities (test thoroughly!)
- Circular dependencies → architectural smell
- Deep transitive closures → tight coupling

---

### 1.6 File & Module Analytics

**What to Measure**: Code organization quality

**Dimensions**:
- Entities per File
- Lines of Code per Entity (line_range span)
- Module Depth (module_path length)
- File Hotspots (files with most activity)

**Example Queries**:

```datalog
# Files with most entities (potential god files)
?[file_path, entity_count] :=
    *CodeGraph{file_path},
    entity_count = count(file_path)
:order entity_count desc
:limit 20

# Average entities per file
?[avg_entities_per_file] :=
    *CodeGraph{file_path},
    entity_count = count(file_path),
    avg_entities_per_file = mean(entity_count)

# Large entities (> 50 lines)
?[ISGL1_key, name, line_span] :=
    *CodeGraph{ISGL1_key, interface_signature},
    line_range = json_extract(interface_signature, '$.line_range'),
    start = json_extract(line_range, '$.start'),
    end = json_extract(line_range, '$.end'),
    line_span = end - start + 1,
    line_span > 50,
    name = json_extract(interface_signature, '$.name')
:order line_span desc
```

**Actionable Insights**:
- Files with 20+ entities → split candidate
- Functions > 100 lines → complexity smell
- Deep module paths (5+ levels) → over-organization

---

### 1.7 Language-Specific Analytics

**What to Measure**: Language-specific patterns (Rust focus)

**Dimensions**:
- Trait Implementations
- Generic Usage
- Lifetime Complexity
- Macro Usage
- Visibility Patterns

**Example Queries**:

```datalog
# Rust: Entities with generics
?[ISGL1_key, name, generic_count] :=
    *CodeGraph{ISGL1_key, interface_signature, language},
    language == "rust",
    lang_specific = json_extract(interface_signature, '$.language_specific'),
    generics = json_extract(lang_specific, '$.generics'),
    generic_count = length(generics),
    generic_count > 0,
    name = json_extract(interface_signature, '$.name')

# Rust: Trait implementations
?[ISGL1_key, trait_name, for_type] :=
    *CodeGraph{ISGL1_key, interface_signature, entity_type},
    entity_type == "impl",
    lang_specific = json_extract(interface_signature, '$.language_specific'),
    trait_impl = json_extract(lang_specific, '$.trait_impl'),
    trait_impl != null,
    trait_name = json_extract(trait_impl, '$.trait_name'),
    for_type = json_extract(trait_impl, '$.for_type')

# Rust: Entities with lifetimes
?[ISGL1_key, name, lifetime_count] :=
    *CodeGraph{ISGL1_key, interface_signature, language},
    language == "rust",
    lang_specific = json_extract(interface_signature, '$.language_specific'),
    lifetimes = json_extract(lang_specific, '$.lifetimes'),
    lifetime_count = length(lifetimes),
    lifetime_count > 0,
    name = json_extract(interface_signature, '$.name')
```

**Actionable Insights**:
- High generic/lifetime usage → Rust complexity indicator
- Trait implementations → extensibility patterns
- Macro-heavy code → debugging challenges

---

## Part 2: Visualization Approaches

### 2.1 Text-Based Tables (Primary Format)

**Best Use**: Structured data with multiple columns

**Rust Libraries**:
- `comfy-table` (most feature-rich, 4.4k stars)
- `tabled` (flexible, 2.1k stars)
- `cli-table` (simple, clean API)

**Example Visualizations**:

```
┌──────────────┬────────┬───────────┬──────────┐
│ Entity Type  │ Count  │ Percent   │ Avg LOC  │
├──────────────┼────────┼───────────┼──────────┤
│ Function     │ 423    │ 64.0%     │ 12       │
│ Struct       │ 156    │ 23.6%     │ 8        │
│ Trait        │ 45     │ 6.8%      │ 15       │
│ Enum         │ 22     │ 3.3%      │ 6        │
│ Impl         │ 15     │ 2.3%      │ 25       │
└──────────────┴────────┴───────────┴──────────┘

COMPLEXITY DISTRIBUTION
┌──────────┬────────┬──────────────────────┐
│ Level    │ Count  │ Bar                  │
├──────────┼────────┼──────────────────────┤
│ Simple   │ 523    │ ████████████████████ │
│ Moderate │ 112    │ ████                 │
│ Complex  │ 26     │ █                    │
└──────────┴────────┴──────────────────────┘

RISK MATRIX (High-Risk Complex Entities)
┌──────────────────────────┬────────────┬────────┬──────────┐
│ Entity                   │ Complexity │ Risk   │ Test Cov │
├──────────────────────────┼────────────┼────────┼──────────┤
│ calculate_blast_radius   │ Complex    │ High   │ 45%      │
│ parse_temporal_state     │ Complex    │ High   │ 30%      │
│ validate_dependencies    │ Complex    │ Medium │ 65%      │
└──────────────────────────┴────────────┴────────┴──────────┘
```

**Implementation Pattern**:
```rust
use comfy_table::{Table, Cell, Color};

fn render_entity_distribution(entities: &[Entity]) -> Table {
    let mut table = Table::new();
    table.set_header(vec!["Entity Type", "Count", "Percent", "Avg LOC"]);

    let stats = calculate_stats(entities);
    for (entity_type, stat) in stats {
        table.add_row(vec![
            Cell::new(entity_type),
            Cell::new(stat.count),
            Cell::new(format!("{:.1}%", stat.percent)),
            Cell::new(stat.avg_loc)
        ]);
    }

    table
}
```

---

### 2.2 ASCII Bar Charts & Sparklines

**Best Use**: Trends, distributions, quick visual comparisons

**Rust Libraries**:
- `textplots` (terminal plotting with Braille canvas)
- `rasciigraph` (simple ASCII graphs)
- Custom unicode blocks (█▓▒░)

**Example Visualizations**:

```
COMPLEXITY TREND (Last 30 Days)
Simple   ████████████████████████████████████ 523
Moderate ████████ 112
Complex  ██ 26

DEPENDENCIES DISTRIBUTION
0 deps   ████████████████████ 234 entities
1-2      ████████████ 156
3-5      ██████ 78
6-10     ███ 45
10+      █ 12

TEST COVERAGE SPARKLINE (by module)
core     ▁▂▃▅▇█████ 85%
utils    ▁▁▂▃▄▅▆▇ 68%
storage  ▁▁▁▂▃▄ 45%
cli      ▁▁▃▅ 32%
```

**Implementation Pattern**:
```rust
fn render_bar_chart(data: &[(String, usize)], max_width: usize) -> String {
    let max_val = data.iter().map(|(_, v)| v).max().unwrap_or(&1);

    data.iter().map(|(label, value)| {
        let bar_len = (value * max_width) / max_val;
        let bar = "█".repeat(bar_len);
        format!("{:<15} {} {}", label, bar, value)
    }).collect::<Vec<_>>().join("\n")
}
```

---

### 2.3 Unicode Box Drawing & Trees

**Best Use**: Hierarchies, dependency trees, module structure

**Rust Libraries**:
- `ptree` (pretty tree printing)
- Custom unicode box chars (─│┌┐└┘├┤┬┴┼)

**Example Visualizations**:

```
MODULE HIERARCHY
src/
├── entities/
│   ├── code_entity.rs (12 entities)
│   ├── temporal.rs (8 entities)
│   └── tdd.rs (6 entities)
├── storage/
│   ├── cozo_client.rs (25 entities)
│   └── mod.rs (3 entities)
└── cli/
    ├── main.rs (18 entities)
    └── args.rs (9 entities)

DEPENDENCY TREE (blast_radius from "process_entity")
process_entity
├── validate_entity
│   ├── check_syntax
│   └── check_types
├── store_entity
│   └── serialize_json
└── emit_event
    └── log_change
```

**Implementation Pattern**:
```rust
use ptree::{TreeBuilder, Style};

fn render_dependency_tree(root: &str, deps: &DependencyGraph) -> String {
    let mut builder = TreeBuilder::new(root.to_string());

    for dep in deps.get_children(root) {
        add_node_recursive(&mut builder, dep, deps);
    }

    let tree = builder.build();
    format!("{}", tree)
}
```

---

### 2.4 Color Coding (Optional Enhancement)

**Best Use**: Quick visual distinction of risk/complexity levels

**Rust Libraries**:
- `colored` (simple color support)
- `owo-colors` (modern, zero-alloc)
- `termion` (full terminal control)

**Color Scheme**:
```
Risk Levels:
  Low    → Green  (✓)
  Medium → Yellow (⚠)
  High   → Red    (✗)

Complexity:
  Simple   → Dim/Gray
  Moderate → White
  Complex  → Bold/Bright

Entity Types:
  Functions → Cyan
  Structs   → Blue
  Traits    → Magenta
```

**Example**:
```
RISK HOTSPOTS
[✗] calculate_blast_radius   (Complex, High Risk, 45% coverage)
[✗] parse_temporal_state     (Complex, High Risk, 30% coverage)
[⚠] validate_dependencies    (Complex, Medium Risk, 65% coverage)
[✓] format_output            (Simple, Low Risk, 90% coverage)
```

---

### 2.5 Summary Cards (Dashboard Style)

**Best Use**: At-a-glance overview of codebase health

**Example Visualization**:

```
╔═══════════════════════════════════════════════════════╗
║           CODEBASE HEALTH DASHBOARD                   ║
╠═══════════════════════════════════════════════════════╣
║                                                       ║
║  Total Entities:    661                               ║
║  Files Indexed:     63                                ║
║  Languages:         Rust (100%)                       ║
║                                                       ║
║  ┌─────────────────────────────────────────┐          ║
║  │ COMPOSITION                             │          ║
║  │  Functions:  423 (64%)                  │          ║
║  │  Structs:    156 (24%)                  │          ║
║  │  Traits:      45 (7%)                   │          ║
║  │  Other:       37 (5%)                   │          ║
║  └─────────────────────────────────────────┘          ║
║                                                       ║
║  ┌─────────────────────────────────────────┐          ║
║  │ CODE HEALTH                             │          ║
║  │  Test Coverage:    68%                  │          ║
║  │  Avg Complexity:   Simple               │          ║
║  │  High Risk Items:  12                   │          ║
║  │  Public APIs:      234                  │          ║
║  └─────────────────────────────────────────┘          ║
║                                                       ║
║  ┌─────────────────────────────────────────┐          ║
║  │ PENDING CHANGES                         │          ║
║  │  Creates:  0                            │          ║
║  │  Edits:    0                            │          ║
║  │  Deletes:  0                            │          ║
║  └─────────────────────────────────────────┘          ║
║                                                       ║
╚═══════════════════════════════════════════════════════╝
```

---

### 2.6 Comparison to Other CLI Tools

**tokei** style (code statistics):
```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Rust                   63        17721        14234         1823         1664
===============================================================================
```

**cargo-tree** style (dependencies):
```
parseltongue v0.8.1
├── clap v4.0
│   ├── clap_derive v4.0
│   └── anyhow v1.0
├── cozo v0.7
│   └── serde v1.0
└── tree-sitter v0.20
```

**cargo-bloat** style (size analysis):
```
 File  .text     Size Crate
 0.5%   6.1%   1.2KiB parseltongue
 0.4%   4.8%   1.0KiB cozo
 0.3%   3.2%   650.B  serde_json
```

**Key Lessons**:
- **Columnar alignment** for scannability
- **Percentage + absolute values** for context
- **Sorting** by most important metric
- **Limited color** (optional, not required)
- **Summary statistics** at top/bottom

---

## Part 3: Practical Use Cases

### 3.1 "At a Glance" Codebase Health

**User Question**: "Is this codebase well-maintained?"

**What to Show**:
```
CODEBASE HEALTH SCORE: B+ (78/100)

✓ Test Coverage:        68% (target: 70%)
✓ Avg Complexity:       Simple (good)
⚠ High-Risk Entities:   12 (review recommended)
✗ Public API Coverage:  45% (target: 80%)
✓ Documentation:        85% of public APIs
⚠ Complex Functions:    26 (8 need refactoring)

TOP PRIORITIES:
1. Add tests for 12 high-risk entities
2. Document 23 public APIs
3. Refactor 8 complex functions
```

**Implementation**:
- Single aggregation query for each metric
- Thresholds configurable (default values provided)
- Color-coded pass/fail indicators
- Actionable next steps

---

### 3.2 Complexity Hotspots

**User Question**: "Where should I focus refactoring efforts?"

**What to Show**:
```
COMPLEXITY HOTSPOTS (Top 10)

┌────┬──────────────────────────┬────────────┬──────┬──────────┬─────────┐
│ #  │ Entity                   │ Complexity │ Risk │ Coverage │ Actions │
├────┼──────────────────────────┼────────────┼──────┼──────────┼─────────┤
│ 1  │ calculate_blast_radius   │ Complex    │ High │ 45%      │ ✗ TEST  │
│ 2  │ parse_temporal_state     │ Complex    │ High │ 30%      │ ✗✗ CRIT │
│ 3  │ validate_entity          │ Complex    │ Med  │ 65%      │ ⚠ DOC   │
│ 4  │ serialize_graph          │ Complex    │ Med  │ 80%      │ ✓ OK    │
│ 5  │ apply_diff               │ Moderate   │ High │ 20%      │ ✗✗ CRIT │
│ 6  │ merge_contexts           │ Moderate   │ Med  │ 55%      │ ⚠ TEST  │
│ 7  │ extract_signature        │ Moderate   │ Med  │ 70%      │ ✓ OK    │
│ 8  │ detect_dependencies      │ Moderate   │ Low  │ 90%      │ ✓ OK    │
│ 9  │ format_diff_output       │ Simple     │ Med  │ 40%      │ ⚠ TEST  │
│ 10 │ init_database            │ Simple     │ High │ 85%      │ ⚠ DOC   │
└────┴──────────────────────────┴────────────┴──────┴──────────┴─────────┘

Legend:
  ✗✗ CRITICAL - Complex + High Risk + Low Coverage (<50%)
  ✗ TEST     - Needs test coverage boost
  ⚠ DOC      - Needs documentation
  ✓ OK       - Acceptable state
```

**Implementation**:
- Query: High complexity OR high risk entities
- Sort by risk × complexity score
- Action recommendation algorithm
- Limit to top 10 for actionability

---

### 3.3 Test Coverage Gaps

**User Question**: "What needs testing?"

**What to Show**:
```
TEST COVERAGE GAPS

Overall Coverage: 68% (451/661 entities)

CRITICAL GAPS (Public APIs with 0% coverage)
┌─────────────────────────┬──────────┬────────────┬──────┐
│ Entity                  │ Type     │ Visibility │ Risk │
├─────────────────────────┼──────────┼────────────┼──────┤
│ export_to_json          │ Function │ Public     │ High │
│ validate_syntax         │ Function │ Public     │ Med  │
│ merge_entities          │ Function │ Public     │ Med  │
│ EntityBuilder           │ Struct   │ Public     │ Low  │
└─────────────────────────┴──────────┴────────────┴──────┘

MODULE-LEVEL COVERAGE
┌──────────────┬──────────┬───────────┐
│ Module       │ Coverage │ Gap       │
├──────────────┼──────────┼───────────┤
│ core         │ 85%      │ ██        │
│ storage      │ 72%      │ █████     │
│ cli          │ 45%      │ ███████████│
│ utils        │ 90%      │ █         │
└──────────────┴──────────┴───────────┘

PRIORITY TESTING TARGETS:
1. cli module (45% → target 70%)
2. export_to_json function (0% → 80%)
3. validate_syntax function (0% → 80%)
```

**Implementation**:
- Query: entities with coverage < threshold
- Prioritize by visibility × risk
- Module-level aggregation
- Visual gap representation

---

### 3.4 Change Risk Assessment

**User Question**: "What's the blast radius of changing function X?"

**What to Show**:
```
BLAST RADIUS ANALYSIS: process_entity()

Direct Impact (1 hop):
  → 8 functions call this directly
  → 3 are on critical path
  → 2 are high-risk entities

Transitive Impact (5 hops):
  → 47 total entities affected
  → 12 public APIs impacted
  → 6 files need review

IMPACT TREE (depth=3)
process_entity
├── [HIGH RISK] validate_entity (called by 12 others)
│   ├── [CRITICAL] check_syntax
│   └── check_types
├── store_entity (called by 5 others)
│   └── serialize_json
└── emit_event
    └── log_change

RECOMMENDATION:
⚠ MEDIUM RISK - 47 entities affected
  → Add integration tests before modifying
  → Review 3 critical path callers
  → Update documentation for 12 public APIs
```

**Implementation**:
- Use `calculate_blast_radius(key, 5)` query
- Mark critical path entities
- Highlight high-risk downstream dependencies
- Risk score = affected_count × risk_multiplier

---

### 3.5 Module Organization Quality

**User Question**: "Is our code well-organized?"

**What to Show**:
```
MODULE ORGANIZATION ANALYSIS

FILE HEALTH
┌──────────────────────────────┬──────────┬─────────┬────────┐
│ File                         │ Entities │ LOC/Ent │ Status │
├──────────────────────────────┼──────────┼─────────┼────────┤
│ src/storage/cozo_client.rs   │ 25       │ 48      │ ⚠ LARGE│
│ src/entities/code_entity.rs  │ 18       │ 35      │ ⚠ LARGE│
│ src/cli/main.rs              │ 12       │ 28      │ ✓ OK   │
│ src/temporal.rs              │ 8        │ 22      │ ✓ OK   │
└──────────────────────────────┴──────────┴─────────┴────────┘

LARGE FUNCTIONS (>100 lines)
  → row_to_entity (125 lines) in cozo_client.rs
  → parse_interface_signature (110 lines) in entities.rs

MODULE DEPTH DISTRIBUTION
  1 level: ███████████████ 45 files
  2 level: █████████ 18 files
  3+ level: ██ 5 files (over-nested?)

RECOMMENDATIONS:
1. Split cozo_client.rs (25 entities → target <15)
2. Extract row_to_entity into separate module
3. Review 3-level nesting in tests/integration/
```

**Implementation**:
- Aggregate entities per file
- Calculate LOC per entity (line_range.span)
- Flag large files (>15 entities) and large functions (>100 lines)
- Module depth from file paths

---

### 3.6 Dependency Health

**User Question**: "Are our dependencies reasonable?"

**What to Show**:
```
DEPENDENCY HEALTH

COUPLING METRICS
  Avg Dependencies:  3.2 per entity
  Max Dependencies:  12 (calculate_blast_radius)
  Zero Dependencies: 234 entities (35%)

HIGH-COUPLING ENTITIES (>8 dependencies)
┌──────────────────────────┬──────┬──────────┐
│ Entity                   │ Deps │ Risk     │
├──────────────────────────┼──────┼──────────┤
│ calculate_blast_radius   │ 12   │ High     │
│ parse_entity             │ 10   │ Medium   │
│ validate_all             │ 9    │ High     │
└──────────────────────────┴──────┴──────────┘

FAN-IN ANALYSIS (Most-called functions)
┌──────────────────────────┬─────────┐
│ Function                 │ Callers │
├──────────────────────────┼─────────┤
│ log_error                │ 47      │
│ serialize_entity         │ 23      │
│ validate_key             │ 18      │
└──────────────────────────┴─────────┘

FAN-OUT ANALYSIS (Functions that call many others)
┌──────────────────────────┬────────┐
│ Function                 │ Calls  │
├──────────────────────────┼────────┤
│ process_workflow         │ 15     │
│ run_pipeline             │ 12     │
│ init_system              │ 10     │
└──────────────────────────┴────────┘

STATUS: ✓ HEALTHY
  → Reasonable average coupling (3.2)
  → Only 3 high-coupling entities
  → No circular dependencies detected
```

**Implementation**:
- Aggregate `dependencies` field from TDD classification
- Query DependencyEdges for fan-in/fan-out
- Circular dependency detection via graph query
- Thresholds: >8 deps = high coupling

---

## Part 4: Tool Design Proposal (pt07-cozodb-code-as-visuals)

### 4.1 Command Structure

```bash
# Default: Dashboard overview
parseltongue pt07-cozodb-code-as-visuals --db rocksdb:analysis.db

# Specific reports
parseltongue pt07-cozodb-code-as-visuals --report health --db rocksdb:analysis.db
parseltongue pt07-cozodb-code-as-visuals --report complexity --db rocksdb:analysis.db
parseltongue pt07-cozodb-code-as-visuals --report coverage --db rocksdb:analysis.db
parseltongue pt07-cozodb-code-as-visuals --report dependencies --db rocksdb:analysis.db
parseltongue pt07-cozodb-code-as-visuals --report changes --db rocksdb:analysis.db
parseltongue pt07-cozodb-code-as-visuals --report blast-radius --entity <ISGL1_key> --db rocksdb:analysis.db

# Output formats
parseltongue pt07-cozodb-code-as-visuals --format table  # Default
parseltongue pt07-cozodb-code-as-visuals --format json   # Machine-readable
parseltongue pt07-cozodb-code-as-visuals --format csv    # Spreadsheet-friendly

# Filtering
parseltongue pt07-cozodb-code-as-visuals --report complexity --filter "risk=High"
parseltongue pt07-cozodb-code-as-visuals --report coverage --filter "visibility=Public"
parseltongue pt07-cozodb-code-as-visuals --report entities --filter "entity_type=Function"

# Custom thresholds
parseltongue pt07-cozodb-code-as-visuals --report health \
  --threshold coverage=80 \
  --threshold max_complexity=10
```

---

### 4.2 CLI Arguments

```rust
#[derive(Parser)]
#[command(name = "pt07-cozodb-code-as-visuals")]
#[command(about = "Generate analytics and visualizations from ISG data")]
struct Cli {
    /// Database path
    #[arg(long, default_value = "parseltongue.db")]
    db: String,

    /// Report type to generate
    #[arg(long, value_enum, default_value = "dashboard")]
    report: ReportType,

    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    format: OutputFormat,

    /// Filter expression (e.g., "risk=High,complexity=Complex")
    #[arg(long)]
    filter: Option<String>,

    /// Entity key for blast-radius report
    #[arg(long)]
    entity: Option<String>,

    /// Custom thresholds (e.g., "coverage=80,max_deps=10")
    #[arg(long)]
    threshold: Vec<String>,

    /// Limit results (default: 20)
    #[arg(long, default_value = "20")]
    limit: usize,

    /// Sort by column
    #[arg(long)]
    sort: Option<String>,

    /// Enable color output
    #[arg(long, default_value = "true")]
    color: bool,

    /// Verbose output
    #[arg(long, short)]
    verbose: bool,
}

#[derive(Clone, ValueEnum)]
enum ReportType {
    Dashboard,      // Overview summary
    Health,         // Codebase health metrics
    Complexity,     // Complexity hotspots
    Coverage,       // Test coverage gaps
    Dependencies,   // Dependency analysis
    Changes,        // Pending changes
    BlastRadius,    // Impact analysis (requires --entity)
    Entities,       // Entity listing (filterable)
    Modules,        // Module organization
    Language,       // Language-specific analysis
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Table,  // Terminal tables (default)
    Json,   // JSON output
    Csv,    // CSV output
}
```

---

### 4.3 Default Visualizations (Dashboard)

When run with no arguments, show a comprehensive dashboard:

```
╔═══════════════════════════════════════════════════════════════╗
║                   PARSELTONGUE ANALYTICS                      ║
║                   Database: rocksdb:test.db                   ║
╠═══════════════════════════════════════════════════════════════╣

📊 CODEBASE OVERVIEW
  Total Entities:   661
  Files:            63
  Languages:        Rust (100%)
  Last Indexed:     2025-11-01 09:02:29 UTC

─────────────────────────────────────────────────────────────────

📈 COMPOSITION
  Functions:   423 (64.0%)  ████████████████████████████████
  Structs:     156 (23.6%)  ████████████
  Traits:       45 (6.8%)   ███
  Enums:        22 (3.3%)   █
  Impl Blocks:  15 (2.3%)   █

─────────────────────────────────────────────────────────────────

🎯 HEALTH SCORE: B+ (78/100)

  ✓ Test Coverage:       68%  (target: 70%)
  ✓ Avg Complexity:      Simple
  ⚠ High-Risk Entities:  12  (review recommended)
  ✗ Public API Coverage: 45%  (target: 80%)
  ✓ Documentation:       85%

─────────────────────────────────────────────────────────────────

⚠️  TOP PRIORITIES

  1. Add tests for 12 high-risk entities
  2. Document 23 public APIs without coverage
  3. Refactor 8 complex functions (>100 LOC)

─────────────────────────────────────────────────────────────────

📝 PENDING CHANGES

  Creates:   0
  Edits:     0
  Deletes:   0

  Status: Clean - No pending changes

─────────────────────────────────────────────────────────────────

💡 QUICK COMMANDS

  Complexity Hotspots:
    parseltongue pt07-cozodb-code-as-visuals --report complexity

  Coverage Gaps:
    parseltongue pt07-cozodb-code-as-visuals --report coverage

  Dependency Health:
    parseltongue pt07-cozodb-code-as-visuals --report dependencies

╚═══════════════════════════════════════════════════════════════╝
```

---

### 4.4 Report-Specific Outputs

#### Complexity Report
```bash
parseltongue pt07-cozodb-code-as-visuals --report complexity
```

Output:
```
COMPLEXITY HOTSPOTS

┌────┬──────────────────────────┬────────────┬──────┬──────────┬─────────┐
│ #  │ Entity                   │ Complexity │ Risk │ Coverage │ Action  │
├────┼──────────────────────────┼────────────┼──────┼──────────┼─────────┤
│ 1  │ calculate_blast_radius   │ Complex    │ High │ 45%      │ ✗ TEST  │
│ 2  │ parse_temporal_state     │ Complex    │ High │ 30%      │ ✗✗ CRIT │
│ 3  │ validate_entity          │ Complex    │ Med  │ 65%      │ ⚠ DOC   │
│ 4  │ serialize_graph          │ Complex    │ Med  │ 80%      │ ✓ OK    │
│ 5  │ apply_diff               │ Moderate   │ High │ 20%      │ ✗✗ CRIT │
└────┴──────────────────────────┴────────────┴──────┴──────────┴─────────┘

COMPLEXITY DISTRIBUTION
  Simple:    523 (79%)  ████████████████████████████████████
  Moderate:  112 (17%)  ███████
  Complex:    26 (4%)   █

RECOMMENDATIONS:
  → Review 2 CRITICAL entities (Complex + High Risk + Low Coverage)
  → Add tests for entities marked ✗ TEST
  → Document entities marked ⚠ DOC
```

#### Coverage Report
```bash
parseltongue pt07-cozodb-code-as-visuals --report coverage --filter "visibility=Public"
```

Output:
```
TEST COVERAGE GAPS (Public APIs Only)

Overall Public API Coverage: 45% (105/234)

CRITICAL GAPS (0% Coverage)
┌─────────────────────────┬──────────┬──────┬──────────┐
│ Entity                  │ Type     │ Risk │ File     │
├─────────────────────────┼──────────┼──────┼──────────┤
│ export_to_json          │ Function │ High │ cli.rs   │
│ validate_syntax         │ Function │ Med  │ valid.rs │
│ EntityBuilder::new      │ Method   │ Low  │ entity.rs│
└─────────────────────────┴──────────┴──────┴──────────┘

MODULE COVERAGE
┌──────────────┬──────────┬───────────────────────────┐
│ Module       │ Coverage │ Gap                       │
├──────────────┼──────────┼───────────────────────────┤
│ core         │ 85%      │ ███                       │
│ storage      │ 72%      │ ██████                    │
│ cli          │ 45%      │ ██████████████            │
│ utils        │ 90%      │ ██                        │
└──────────────┴──────────┴───────────────────────────┘

NEXT STEPS:
  1. Add tests for export_to_json (High Risk)
  2. Add tests for validate_syntax (Medium Risk)
  3. Boost cli module from 45% → 70%
```

#### Blast Radius Report
```bash
parseltongue pt07-cozodb-code-as-visuals --report blast-radius \
  --entity "rust:fn:process_entity:src_core_rs:42-68"
```

Output:
```
BLAST RADIUS: process_entity()

Impact Summary:
  Direct callers:     8 entities
  Transitive impact:  47 entities (within 5 hops)
  Critical path:      3 entities
  Public APIs:        12 entities
  Files affected:     6

Impact Tree (3 levels):
process_entity
├── [HIGH RISK] validate_entity (12 callers)
│   ├── [CRITICAL] check_syntax
│   └── check_types
├── store_entity (5 callers)
│   └── serialize_json
└── emit_event
    └── log_change

Risk Assessment: ⚠ MEDIUM
  → 47 entities affected (threshold: 50)
  → 3 critical path dependencies
  → Recommendation: Add integration tests before modifying
```

---

### 4.5 Prioritized Feature List

#### MUST-HAVE (MVP)
1. **Dashboard Report** - Overview of codebase health
2. **Complexity Report** - Top complexity hotspots with actionable recommendations
3. **Coverage Report** - Test coverage gaps prioritized by risk
4. **Entity Listing** - Filterable table of all entities
5. **Table Output Format** - ASCII tables with comfy-table
6. **Basic Filtering** - By entity_type, risk, complexity, visibility

#### SHOULD-HAVE (v1.0)
7. **Blast Radius Report** - Impact analysis for specific entities
8. **Dependencies Report** - Coupling metrics and fan-in/fan-out
9. **Changes Report** - Summary of pending temporal changes
10. **JSON Output Format** - For programmatic consumption
11. **Color Support** - Risk/complexity color coding
12. **Custom Thresholds** - User-defined health score parameters

#### NICE-TO-HAVE (v2.0)
13. **Modules Report** - File organization quality analysis
14. **Language Report** - Rust-specific analytics (generics, lifetimes)
15. **Trend Analysis** - Compare snapshots over time
16. **CSV Output Format** - For spreadsheet analysis
17. **Bar Charts** - ASCII visualization of distributions
18. **Interactive Mode** - Navigate reports with keyboard

---

### 4.6 Implementation Phases

#### Phase 1: Core Infrastructure (Week 1)
- CLI argument parsing with clap
- Database connection (reuse parseltongue-core)
- Query execution framework
- Table rendering with comfy-table
- Basic filtering logic

**Deliverable**: `parseltongue pt07-cozodb-code-as-visuals --report entities` works

#### Phase 2: Essential Reports (Week 2)
- Dashboard report
- Complexity report
- Coverage report
- Health score calculation

**Deliverable**: 3 core reports with actionable insights

#### Phase 3: Advanced Analytics (Week 3)
- Blast radius report (using existing CozoDB graph queries)
- Dependencies report
- Changes report
- Custom threshold support

**Deliverable**: Full analytics suite

#### Phase 4: Polish & Export (Week 4)
- JSON/CSV output formats
- Color support (optional)
- Documentation
- Integration tests
- Performance optimization

**Deliverable**: Production-ready v1.0

---

### 4.7 Example Usage Patterns

#### Morning Standup Check
```bash
# Quick health check before starting work
parseltongue pt07-cozodb-code-as-visuals --db rocksdb:project.db

# Output: Dashboard showing current codebase state
```

#### Pre-Refactor Analysis
```bash
# Before modifying a function, check impact
parseltongue pt07-cozodb-code-as-visuals \
  --report blast-radius \
  --entity "rust:fn:calculate_tax:src_billing_rs:120-145" \
  --db rocksdb:project.db

# Output: List of all affected entities and risk assessment
```

#### Code Review Prep
```bash
# Generate coverage report for review discussion
parseltongue pt07-cozodb-code-as-visuals \
  --report coverage \
  --filter "visibility=Public" \
  --format csv \
  --db rocksdb:project.db > coverage_gaps.csv

# Output: CSV for sharing with team
```

#### Continuous Integration
```bash
# Check if health score meets threshold
parseltongue pt07-cozodb-code-as-visuals \
  --report health \
  --threshold coverage=80 \
  --threshold max_complexity=10 \
  --format json \
  --db rocksdb:project.db | jq '.health_score'

# Output: JSON for CI pipeline to parse
```

---

## Part 5: Technical Implementation Notes

### 5.1 Query Optimization Patterns

**Avoid**: Running separate queries for each metric
```rust
// BAD: N queries
for entity_type in ["Function", "Struct", "Trait"] {
    let count = query(&format!("SELECT COUNT(*) WHERE entity_type = '{}'", entity_type));
}
```

**Prefer**: Single aggregation query
```rust
// GOOD: 1 query
let query = "?[entity_type, count] := *CodeGraph{entity_type}, count = count(entity_type)";
let results = db.run_script(query);
```

### 5.2 Rendering Performance

For large codebases (1000+ entities):
- **Limit results** to top N (default: 20)
- **Paginate** if needed (--page flag)
- **Stream output** instead of buffering entire table
- **Cache** repeated queries (e.g., total entity count)

### 5.3 Error Handling

```rust
// Graceful degradation
let coverage = match calculate_coverage(&db) {
    Ok(val) => format!("{:.1}%", val),
    Err(_) => "N/A".to_string(),  // Don't fail entire report
};
```

### 5.4 Testing Strategy

**Unit Tests**:
- Query construction
- Filtering logic
- Threshold calculations
- Table rendering

**Integration Tests**:
- Full report generation with sample database
- JSON/CSV output validation
- Filter combinations

**Snapshot Tests**:
- Golden files for report outputs
- Ensure formatting consistency

### 5.5 Extensibility Points

```rust
// Trait for pluggable reports
pub trait Report {
    fn name(&self) -> &str;
    fn generate(&self, db: &CozoDbStorage, options: &ReportOptions) -> Result<String>;
    fn supports_filter(&self, filter_key: &str) -> bool;
}

// Users can add custom reports
struct CustomReport;
impl Report for CustomReport {
    fn name(&self) -> &str { "custom" }
    fn generate(&self, db: &CozoDbStorage, options: &ReportOptions) -> Result<String> {
        // Custom logic
    }
    fn supports_filter(&self, filter_key: &str) -> bool { false }
}
```

---

## Part 6: Comparison to Existing Tools

### tokei - Code Statistics
**What it does well**:
- Fast file/line counting
- Multi-language support
- Clean columnar output
- JSON export

**What we add**:
- **Semantic analysis** (not just lines, but entity types)
- **Code health metrics** (complexity, risk, testability)
- **Dependency analysis** (blast radius, coupling)
- **Temporal tracking** (pending changes)

### cargo-tree - Dependency Visualization
**What it does well**:
- Tree visualization of crate dependencies
- Duplicate detection
- Clear hierarchical structure

**What we add**:
- **Code-level dependencies** (function calls, not crates)
- **Impact analysis** (who depends on this function?)
- **Risk assessment** (change impact prediction)

### cargo-bloat - Binary Size Analysis
**What it does well**:
- Function-level size breakdown
- Actionable size reduction targets
- Percentage-based reporting

**What we add**:
- **Code complexity** (not binary size)
- **Test coverage** integration
- **Multi-dimensional analysis** (risk + complexity + coverage)

### Key Differentiators
1. **Graph-Native**: Dependency analysis via CozoDB Datalog
2. **LLM-Oriented**: Metrics designed for code modification decisions
3. **Temporal-Aware**: Track pending changes and their impact
4. **Multi-Dimensional**: Combine complexity, risk, coverage, dependencies
5. **Actionable**: Every report ends with "what should I do next?"

---

## Part 7: Research Conclusions

### What Works Best for Terminal Analytics

1. **Tables are King**: Columnar data with clear headers beats everything
2. **Limit Results**: Show top 10-20, not all 661 entities
3. **Actionable Recommendations**: End every report with next steps
4. **Multi-Format Support**: Terminal (default), JSON (CI), CSV (sharing)
5. **Progressive Disclosure**: Dashboard → Detailed Reports → Filtered Views
6. **Color is Optional**: Design works without color, enhanced with it
7. **Unicode is Safe**: Box drawing chars work everywhere modern (2025)

### What to Avoid

1. **ASCII Art Overkill**: Cute but not functional
2. **Too Much Color**: Reduces scannability
3. **Deep Nesting**: Keep trees to 3 levels max
4. **Wall of Text**: Use whitespace, headers, separators
5. **Raw Numbers**: Always include context (percentage, threshold, comparison)
6. **Generic Advice**: "Improve code quality" → "Add tests for export_to_json"

### Core Metrics That Matter

**For Daily Development**:
1. Health Score (single number)
2. Complexity Hotspots (top 10)
3. Coverage Gaps (public APIs with 0%)
4. Pending Changes (temporal state summary)

**For Refactoring**:
5. Blast Radius (specific entity)
6. Dependency Coupling (high fan-in/fan-out)
7. File Organization (entities per file)

**For Code Review**:
8. Risk Assessment (change impact)
9. Test Coverage (by module)
10. Documentation Coverage (public APIs)

---

## Appendix A: CozoDB Query Cookbook

### A.1 Basic Aggregations

```datalog
# Total entity count
?[count] := *CodeGraph{ISGL1_key}, count = count(ISGL1_key)

# Count by entity type
?[entity_type, count] := *CodeGraph{entity_type}, count = count(entity_type)

# Count by complexity level
?[complexity, count] :=
    *CodeGraph{TDD_Classification},
    complexity = json_extract(TDD_Classification, '$.complexity'),
    count = count(complexity)
```

### A.2 Filtering & Sorting

```datalog
# High-risk entities
?[ISGL1_key, risk] :=
    *CodeGraph{ISGL1_key, TDD_Classification},
    risk = json_extract(TDD_Classification, '$.change_risk'),
    risk == "High"

# Sorted by dependencies
?[ISGL1_key, deps] :=
    *CodeGraph{ISGL1_key, TDD_Classification},
    deps = json_extract(TDD_Classification, '$.dependencies')
:order deps desc
:limit 20
```

### A.3 Multi-Condition Queries

```datalog
# Complex AND High Risk AND Low Coverage
?[ISGL1_key, name] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification},
    complexity = json_extract(TDD_Classification, '$.complexity'),
    complexity == "Complex",
    risk = json_extract(TDD_Classification, '$.change_risk'),
    risk == "High",
    coverage = json_extract(TDD_Classification, '$.test_coverage_estimate'),
    coverage < 0.5,
    name = json_extract(interface_signature, '$.name')
```

### A.4 JSON Field Extraction

```datalog
# Extract nested JSON fields
?[name, visibility, entity_type] :=
    *CodeGraph{interface_signature},
    name = json_extract(interface_signature, '$.name'),
    visibility = json_extract(interface_signature, '$.visibility'),
    entity_type = json_extract(interface_signature, '$.entity_type')
```

### A.5 Temporal State Queries

```datalog
# All pending changes
?[ISGL1_key, action] :=
    *CodeGraph{ISGL1_key, Future_Action},
    Future_Action != null,
    action = Future_Action

# Entities to be created
?[ISGL1_key] :=
    *CodeGraph{ISGL1_key, current_ind, future_ind, Future_Action},
    current_ind == false,
    future_ind == true,
    Future_Action == "Create"
```

### A.6 Graph Queries (DependencyEdges)

```datalog
# Find all callers of a function
?[from_key] :=
    *DependencyEdges{from_key, to_key, edge_type},
    to_key == $target_key,
    edge_type == "Calls"

# Find all functions this calls
?[to_key] :=
    *DependencyEdges{from_key, to_key, edge_type},
    from_key == $target_key,
    edge_type == "Calls"

# Fan-in (most-called)
?[to_key, call_count] :=
    *DependencyEdges{to_key, edge_type},
    edge_type == "Calls",
    call_count = count(to_key)
:order call_count desc
:limit 20
```

---

## Appendix B: Recommended Libraries

### Terminal Tables
- **comfy-table** (primary choice)
  - Rich feature set (borders, colors, alignment)
  - 4.4k stars, actively maintained
  - Example: `let mut table = Table::new(); table.set_header(...);`

- **tabled** (alternative)
  - Derive macros for structs
  - Flexible styling
  - Example: `println!("{}", Table::new(entities));`

### Terminal Charts
- **textplots** (ASCII/Braille plotting)
  - Line charts, histograms
  - Example: `Chart::new(120, 60, 0.0, 100.0).lineplot(&data);`

- **rasciigraph** (simple bar charts)
  - Minimal, focused
  - Example: `plot(data, Config::default());`

### Colors (Optional)
- **owo-colors** (modern, zero-alloc)
  - Example: `println!("{}", "High Risk".red());`

- **colored** (simple API)
  - Example: `println!("{}", "✓ OK".green());`

### Tree Rendering
- **ptree** (hierarchical trees)
  - Example: `TreeBuilder::new("root").build();`

---

## Appendix C: Health Score Algorithm

```rust
pub fn calculate_health_score(stats: &CodebaseStats) -> u8 {
    let mut score = 100u8;

    // Test coverage (max -30 points)
    if stats.test_coverage < 0.8 {
        score -= ((0.8 - stats.test_coverage) * 30.0) as u8;
    }

    // Complexity ratio (max -20 points)
    let complex_ratio = stats.complex_entities as f64 / stats.total_entities as f64;
    if complex_ratio > 0.1 {
        score -= ((complex_ratio - 0.1) * 200.0) as u8;
    }

    // High-risk entities (max -20 points)
    let risk_ratio = stats.high_risk_entities as f64 / stats.total_entities as f64;
    if risk_ratio > 0.05 {
        score -= ((risk_ratio - 0.05) * 400.0) as u8;
    }

    // Public API documentation (max -15 points)
    if stats.public_api_doc_coverage < 0.9 {
        score -= ((0.9 - stats.public_api_doc_coverage) * 15.0) as u8;
    }

    // Coupling (max -15 points)
    if stats.avg_dependencies > 5.0 {
        score -= ((stats.avg_dependencies - 5.0) * 3.0) as u8;
    }

    score.max(0)  // Floor at 0
}

pub fn score_to_grade(score: u8) -> &'static str {
    match score {
        90..=100 => "A+ (Excellent)",
        80..=89  => "A (Very Good)",
        70..=79  => "B+ (Good)",
        60..=69  => "B (Acceptable)",
        50..=59  => "C (Needs Work)",
        _        => "D (Critical Issues)",
    }
}
```

---

## Appendix D: Sample JSON Output

```json
{
  "report_type": "dashboard",
  "timestamp": "2025-11-01T10:30:00Z",
  "database": "rocksdb:test.db",
  "summary": {
    "total_entities": 661,
    "total_files": 63,
    "languages": ["Rust"],
    "last_indexed": "2025-11-01T09:02:29Z"
  },
  "composition": {
    "functions": {"count": 423, "percent": 64.0},
    "structs": {"count": 156, "percent": 23.6},
    "traits": {"count": 45, "percent": 6.8},
    "enums": {"count": 22, "percent": 3.3},
    "impls": {"count": 15, "percent": 2.3}
  },
  "health_score": {
    "score": 78,
    "grade": "B+",
    "metrics": {
      "test_coverage": 68.0,
      "avg_complexity": "Simple",
      "high_risk_count": 12,
      "public_api_coverage": 45.0,
      "documentation_coverage": 85.0
    }
  },
  "priorities": [
    "Add tests for 12 high-risk entities",
    "Document 23 public APIs without coverage",
    "Refactor 8 complex functions (>100 LOC)"
  ],
  "pending_changes": {
    "creates": 0,
    "edits": 0,
    "deletes": 0,
    "status": "clean"
  }
}
```

---

## Recommended Next Steps

1. **Validate with Users**: Show this research to 2-3 developers, get feedback on which reports are most valuable
2. **Prototype Dashboard**: Build the dashboard report first (highest value)
3. **Iterate on Queries**: Test CozoDB queries on real 1000+ entity databases for performance
4. **Design Review**: Review table formatting and output examples with team
5. **Implementation Sprint**: Start Phase 1 (Core Infrastructure) once design approved

---

**End of Research Document**
**Total Pages**: 25
**Query Examples**: 40+
**Report Types**: 10
**Visualization Examples**: 15+
