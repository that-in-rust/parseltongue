# pt07-cozodb-code-as-visuals: Visual Mockups & Examples

**Companion Document to ISG_ANALYTICS_RESEARCH.md**

This document provides concrete visual examples of what `pt07-cozodb-code-as-visuals` output should look like. All examples use actual data structure from Parseltongue ISG.

---

## Table of Contents

1. [Dashboard Report](#1-dashboard-report)
2. [Complexity Report](#2-complexity-report)
3. [Coverage Report](#3-coverage-report)
4. [Blast Radius Report](#4-blast-radius-report)
5. [Dependencies Report](#5-dependencies-report)
6. [Changes Report](#6-changes-report)
7. [Entities Report](#7-entities-report)
8. [Modules Report](#8-modules-report)
9. [Quick Reference Guide](#9-quick-reference-guide)

---

## 1. Dashboard Report

**Command**: `parseltongue pt07-cozodb-code-as-visuals --db rocksdb:test.db`

**Output**:

```
╔═══════════════════════════════════════════════════════════════════════╗
║                     PARSELTONGUE CODE ANALYTICS                       ║
║                                                                       ║
║  Database: rocksdb:test.db                                            ║
║  Indexed:  2025-11-01 09:02:29 UTC                                    ║
╠═══════════════════════════════════════════════════════════════════════╣

📊 CODEBASE SNAPSHOT

  Total Entities:  661
  Files Analyzed:  63 Rust files
  Total LOC:       17,721 lines
  Database Size:   4.2 KB

─────────────────────────────────────────────────────────────────────────

📈 COMPOSITION BREAKDOWN

  Entity Type       Count    Percent   Avg LOC
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Function          423      64.0%     12
  Struct            156      23.6%     8
  Trait             45       6.8%      15
  Enum              22       3.3%      6
  Impl Block        15       2.3%      25
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Visibility Distribution:
    Public:   234 (35%)  ████████████████
    Private:  312 (47%)  █████████████████████
    Crate:    115 (18%)  ████████

─────────────────────────────────────────────────────────────────────────

🎯 HEALTH SCORE: B+ (78/100)

  Metric                    Value    Target   Status
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Test Coverage             68%      ≥70%     ⚠  Near
  Avg Complexity            Simple   Simple   ✓  Good
  High-Risk Entities        12       ≤10      ⚠  Review
  Public API Coverage       45%      ≥80%     ✗  Low
  Documentation Coverage    85%      ≥80%     ✓  Good
  Avg Dependencies          3.2      ≤5.0     ✓  Good
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

─────────────────────────────────────────────────────────────────────────

⚡ COMPLEXITY DISTRIBUTION

  Level      Count    Percent   Bar
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Simple     523      79%       ████████████████████████████████
  Moderate   112      17%       ███████
  Complex    26       4%        ██
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

─────────────────────────────────────────────────────────────────────────

🧪 TEST COVERAGE ANALYSIS

  Category           Entities  Coverage
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Code               573       65%
  Tests              88        100%
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Coverage by Risk:
    Low Risk:     90%  ████████████████████████
    Medium Risk:  72%  ██████████████████
    High Risk:    42%  ██████████  ⚠ CRITICAL GAP

─────────────────────────────────────────────────────────────────────────

📝 PENDING CHANGES

  Action     Count   Files Affected
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Create     0       -
  Edit       0       -
  Delete     0       -
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Status: ✓ CLEAN - No pending temporal changes

─────────────────────────────────────────────────────────────────────────

⚠️  TOP 3 PRIORITIES

  1. CRITICAL: Add tests for 12 high-risk entities (42% coverage)
     → Focus on: calculate_blast_radius, parse_temporal_state, apply_diff

  2. IMPORTANT: Document 23 public APIs (missing coverage)
     → Priority: Functions in cli/ and storage/ modules

  3. REVIEW: Refactor 8 complex functions (>100 LOC)
     → Candidates: row_to_entity, parse_interface_signature

─────────────────────────────────────────────────────────────────────────

💡 QUICK ACTIONS

  View complexity hotspots:
    parseltongue pt07-cozodb-code-as-visuals --report complexity

  Analyze test coverage gaps:
    parseltongue pt07-cozodb-code-as-visuals --report coverage

  Check dependency health:
    parseltongue pt07-cozodb-code-as-visuals --report dependencies

  Assess change impact:
    parseltongue pt07-cozodb-code-as-visuals --report blast-radius \
      --entity "rust:fn:YOUR_FUNCTION:..."

╚═══════════════════════════════════════════════════════════════════════╝

Report generated in 42ms
```

---

## 2. Complexity Report

**Command**: `parseltongue pt07-cozodb-code-as-visuals --report complexity --db rocksdb:test.db`

**Output**:

```
COMPLEXITY HOTSPOTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Top 20 entities by complexity/risk score (showing all CRITICAL and HIGH)

┌────┬─────────────────────────────┬────────────┬──────────┬──────────┬─────────┬──────────────────┐
│ #  │ Entity                      │ Complexity │ Risk     │ Coverage │ Action  │ Location         │
├────┼─────────────────────────────┼────────────┼──────────┼──────────┼─────────┼──────────────────┤
│ 1  │ calculate_blast_radius      │ Complex    │ High     │ 45%      │ ✗✗ CRIT │ cozo_client.rs   │
│ 2  │ parse_temporal_state        │ Complex    │ High     │ 30%      │ ✗✗ CRIT │ temporal.rs      │
│ 3  │ row_to_entity               │ Complex    │ High     │ 50%      │ ✗ TEST  │ cozo_client.rs   │
│ 4  │ apply_diff                  │ Complex    │ High     │ 35%      │ ✗✗ CRIT │ diff_writer.rs   │
│ 5  │ validate_entity             │ Complex    │ Medium   │ 65%      │ ⚠ TEST  │ validator.rs     │
│ 6  │ serialize_graph             │ Complex    │ Medium   │ 80%      │ ✓ OK    │ storage.rs       │
│ 7  │ parse_interface_signature   │ Complex    │ Medium   │ 70%      │ ⚠ DOC   │ entities.rs      │
│ 8  │ extract_dependencies        │ Complex    │ Low      │ 85%      │ ✓ OK    │ analyzer.rs      │
│    │                             │            │          │          │         │                  │
│ 9  │ merge_contexts              │ Moderate   │ High     │ 55%      │ ⚠ TEST  │ context.rs       │
│ 10 │ build_entity_tree           │ Moderate   │ High     │ 48%      │ ✗ TEST  │ tree_builder.rs  │
│ 11 │ resolve_references          │ Moderate   │ Medium   │ 72%      │ ⚠ DOC   │ resolver.rs      │
│ 12 │ format_diff_output          │ Moderate   │ Medium   │ 90%      │ ✓ OK    │ formatter.rs     │
│ 13 │ detect_circular_deps        │ Moderate   │ Medium   │ 60%      │ ⚠ TEST  │ analyzer.rs      │
│ 14 │ init_database               │ Moderate   │ Low      │ 95%      │ ✓ OK    │ db_init.rs       │
│ 15 │ handle_temporal_change      │ Moderate   │ Low      │ 78%      │ ✓ OK    │ temporal.rs      │
│    │                             │            │          │          │         │                  │
│ 16 │ log_event                   │ Simple     │ High     │ 40%      │ ⚠ TEST  │ logger.rs        │
│ 17 │ validate_key_format         │ Simple     │ Medium   │ 85%      │ ✓ OK    │ validation.rs    │
│ 18 │ format_error_message        │ Simple     │ Low      │ 92%      │ ✓ OK    │ error.rs         │
│ 19 │ sanitize_path               │ Simple     │ Low      │ 88%      │ ✓ OK    │ utils.rs         │
│ 20 │ get_timestamp               │ Simple     │ Low      │ 100%     │ ✓ OK    │ utils.rs         │
└────┴─────────────────────────────┴────────────┴──────────┴──────────┴─────────┴──────────────────┘

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

COMPLEXITY DISTRIBUTION

  Level      Count    Percent   Distribution
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Simple     523      79.1%     ████████████████████████████████████
  Moderate   112      16.9%     ████████
  Complex    26       3.9%      ██
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RISK-COMPLEXITY MATRIX

                  │ Low Risk │ Medium Risk │ High Risk │
  ────────────────┼──────────┼─────────────┼───────────┤
  Complex         │    2     │      4      │     4     │  ← Focus here
  Moderate        │   23     │     12      │     3     │
  Simple          │  412     │     78      │     5     │
  ────────────────┴──────────┴─────────────┴───────────┘

  DANGER ZONE: 4 entities (Complex + High Risk)
  → Immediate action required

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ACTION LEGEND

  ✗✗ CRITICAL - Complex/Moderate + High Risk + Coverage <50%
               → Add comprehensive tests immediately

  ✗  TEST     - Coverage <70%, testability possible
               → Boost test coverage

  ⚠  TEST     - Coverage 50-70%, needs improvement
               → Add edge case tests

  ⚠  DOC      - Missing or incomplete documentation
               → Document public API and behavior

  ✓  OK       - Meets quality thresholds
               → No immediate action needed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS

  Immediate (This Sprint):
    1. Add tests for calculate_blast_radius (Complex, High, 45%)
    2. Add tests for parse_temporal_state (Complex, High, 30%)
    3. Add tests for apply_diff (Complex, High, 35%)
    4. Review row_to_entity for refactoring (125 LOC, Complex)

  Short-term (Next Sprint):
    5. Boost coverage for validate_entity (65% → 80%)
    6. Document parse_interface_signature
    7. Add tests for merge_contexts (Moderate, High, 55%)

  Long-term (Tech Debt):
    8. Consider splitting large files with 20+ entities
    9. Review all High Risk entities for architecture improvements
    10. Establish coverage threshold CI check (70% minimum)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 28ms
Query: ?[ISGL1_key, name, complexity, risk, coverage] := ...
```

---

## 3. Coverage Report

**Command**: `parseltongue pt07-cozodb-code-as-visuals --report coverage --filter "visibility=Public" --db rocksdb:test.db`

**Output**:

```
TEST COVERAGE ANALYSIS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Filter: visibility=Public (234 entities)
Overall Public API Coverage: 45.3% (106/234 entities)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

CRITICAL GAPS (Public APIs with 0% Coverage)

┌─────────────────────────────┬──────────┬──────────┬────────────┬──────────────────┐
│ Entity                      │ Type     │ Risk     │ Complexity │ Location         │
├─────────────────────────────┼──────────┼──────────┼────────────┼──────────────────┤
│ export_to_json              │ Function │ High     │ Moderate   │ export.rs:42     │
│ validate_syntax             │ Function │ High     │ Complex    │ validator.rs:15  │
│ merge_entities              │ Function │ Medium   │ Moderate   │ merger.rs:88     │
│ create_isgl1_key            │ Function │ Medium   │ Simple     │ keys.rs:12       │
│ EntityBuilder::new          │ Method   │ Low      │ Simple     │ builder.rs:23    │
│ format_temporal_state       │ Function │ Low      │ Simple     │ temporal.rs:156  │
└─────────────────────────────┴──────────┴──────────┴────────────┴──────────────────┘

6 public APIs have ZERO test coverage (2.6% of public APIs)

PRIORITY: Add tests for High-risk entities first (export_to_json, validate_syntax)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MODULE-LEVEL COVERAGE

┌────────────────────┬───────────┬──────────┬───────────────────────────────┐
│ Module             │ Entities  │ Coverage │ Coverage Bar                  │
├────────────────────┼───────────┼──────────┼───────────────────────────────┤
│ utils              │ 23        │ 90%      │ ███████████████████████████   │
│ core               │ 67        │ 85%      │ ██████████████████████████    │
│ storage            │ 45        │ 72%      │ ████████████████████          │
│ entities           │ 52        │ 68%      │ ███████████████████           │
│ temporal           │ 18        │ 65%      │ ██████████████████            │
│ cli                │ 29        │ 45%      │ ████████████                  │
└────────────────────┴───────────┴──────────┴───────────────────────────────┘

LOWEST COVERAGE: cli module (45%) - 16 entities need tests

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

COVERAGE BY RISK LEVEL

  Risk Level   Avg Coverage   Entities   Status
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Low          88%            178        ✓ Good
  Medium       68%            46         ⚠ Below target
  High         42%            10         ✗ CRITICAL
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ⚠ WARNING: High-risk entities have dangerously low coverage

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

COVERAGE DISTRIBUTION

  Range      Count    Percent   Distribution
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  90-100%    52       22%       ███████████
  70-89%     54       23%       ███████████
  50-69%     38       16%       ████████
  30-49%     28       12%       ██████
  10-29%     18       8%        ████
  0-9%       44       19%       █████████
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  44 entities (19%) have almost no coverage (<10%)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS

  🎯 Short-term Goals (1-2 weeks):
     → Boost high-risk coverage from 42% to 70% (8 entities)
     → Focus on: export_to_json, validate_syntax
     → Target: Add 35 new tests

  📊 Medium-term Goals (1 month):
     → Bring cli module from 45% to 70%
     → Reduce 0% coverage entities from 6 to 0
     → Target: Overall public API coverage 65%

  🏆 Long-term Goals (Quarter):
     → Establish 80% coverage threshold for public APIs
     → Integrate coverage CI check in pipeline
     → Maintain coverage velocity (no regressions)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 QUICK ACTIONS

  Generate test stubs for uncovered APIs:
    parseltongue pt07-cozodb-code-as-visuals --report coverage \
      --filter "coverage=0" --format json | jq '.entities[].name'

  View coverage for specific module:
    parseltongue pt07-cozodb-code-as-visuals --report coverage \
      --filter "module=cli"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 19ms
```

---

## 4. Blast Radius Report

**Command**:
```bash
parseltongue pt07-cozodb-code-as-visuals --report blast-radius \
  --entity "rust:fn:process_entity:src_core_rs:42-68" \
  --db rocksdb:test.db
```

**Output**:

```
BLAST RADIUS ANALYSIS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Target Entity: process_entity()
Location:      src/core.rs:42-68
Type:          Function (Public)
Complexity:    Moderate
Risk:          High

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

IMPACT SUMMARY

  Metric                Value
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Direct callers        8 entities
  Transitive impact     47 entities (within 5 hops)
  Critical path         3 entities
  Public APIs affected  12 entities
  Files to review       6 files
  Estimated LOC impact  ~1,240 lines
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DIRECT CALLERS (1 hop)

┌─────────────────────────┬──────────┬──────────┬─────────────┬──────────────────┐
│ Caller                  │ Type     │ Risk     │ Critical?   │ Location         │
├─────────────────────────┼──────────┼──────────┼─────────────┼──────────────────┤
│ run_pipeline            │ Function │ High     │ ✓ YES       │ pipeline.rs:15   │
│ handle_workflow         │ Function │ High     │ ✓ YES       │ workflow.rs:88   │
│ batch_process           │ Function │ Medium   │ NO          │ batch.rs:42      │
│ validate_and_process    │ Function │ High     │ ✓ YES       │ validator.rs:156 │
│ retry_failed            │ Function │ Medium   │ NO          │ retry.rs:23      │
│ log_process_event       │ Function │ Low      │ NO          │ logger.rs:67     │
│ test_process_entity     │ Test     │ -        │ NO          │ tests/core.rs    │
│ benchmark_processing    │ Bench    │ -        │ NO          │ benches/core.rs  │
└─────────────────────────┴──────────┴──────────┴─────────────┴──────────────────┘

⚠ ALERT: 3 critical path entities depend on this function

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DEPENDENCY TREE (depth=3)

process_entity
├── [HIGH RISK] validate_entity (called by 12 others)
│   ├── [CRITICAL PATH] check_syntax
│   │   └── parse_tree_sitter
│   ├── check_types
│   │   └── resolve_type_info
│   └── check_temporal_state
│       └── validate_temporal_indicators
├── store_entity (called by 5 others)
│   ├── serialize_to_json
│   │   └── escape_special_chars
│   └── write_to_database
│       └── cozo_put_query
├── emit_event
│   └── log_change
│       └── format_log_message
└── update_metrics
    ├── increment_counter
    └── record_latency

Total reachable entities: 18 (within 3 hops)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

TRANSITIVE IMPACT (5 hops, top 20 by distance)

┌──────┬──────────────────────────┬──────────┬──────────┬──────────────────┐
│ Dist │ Entity                   │ Type     │ Risk     │ Location         │
├──────┼──────────────────────────┼──────────┼──────────┼──────────────────┤
│  1   │ validate_entity          │ Function │ High     │ validator.rs     │
│  1   │ store_entity             │ Function │ Medium   │ storage.rs       │
│  1   │ emit_event               │ Function │ Low      │ events.rs        │
│  1   │ update_metrics           │ Function │ Low      │ metrics.rs       │
│      │                          │          │          │                  │
│  2   │ check_syntax             │ Function │ High     │ syntax.rs        │
│  2   │ check_types              │ Function │ Medium   │ types.rs         │
│  2   │ check_temporal_state     │ Function │ High     │ temporal.rs      │
│  2   │ serialize_to_json        │ Function │ Medium   │ serializer.rs    │
│  2   │ write_to_database        │ Function │ High     │ db_writer.rs     │
│      │                          │          │          │                  │
│  3   │ parse_tree_sitter        │ Function │ Medium   │ parser.rs        │
│  3   │ resolve_type_info        │ Function │ Medium   │ resolver.rs      │
│  3   │ validate_temporal_ind... │ Function │ High     │ temporal.rs      │
│  3   │ cozo_put_query           │ Function │ Medium   │ cozo.rs          │
│      │                          │          │          │                  │
│  4   │ execute_datalog          │ Function │ High     │ cozo.rs          │
│  4   │ parse_temporal_action    │ Function │ Medium   │ temporal.rs      │
│  4   │ construct_isgl1_key      │ Function │ Low      │ keys.rs          │
│      │                          │          │          │                  │
│  5   │ sanitize_key_component   │ Function │ Low      │ utils.rs         │
│  5   │ hash_entity_content      │ Function │ Low      │ hashing.rs       │
│  5   │ format_line_range        │ Function │ Low      │ formatting.rs    │
│  5   │ db_connection_pool       │ Struct   │ High     │ db.rs            │
└──────┴──────────────────────────┴──────────┴──────────┴──────────────────┘

... 27 more entities (use --limit to see all)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FILES REQUIRING REVIEW

┌────────────────────────┬───────────────────┬──────────────┐
│ File                   │ Affected Entities │ Est. LOC     │
├────────────────────────┼───────────────────┼──────────────┤
│ src/storage.rs         │ 8                 │ 340          │
│ src/validator.rs       │ 7                 │ 280          │
│ src/temporal.rs        │ 6                 │ 220          │
│ src/cozo.rs            │ 5                 │ 200          │
│ src/serializer.rs      │ 4                 │ 150          │
│ src/utils.rs           │ 3                 │ 50           │
└────────────────────────┴───────────────────┴──────────────┘

Total: 6 files, ~1,240 lines of code to review

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RISK ASSESSMENT

  Overall Risk Level: ⚠ MEDIUM-HIGH

  Factors:
    ✗ 47 entities affected (threshold: 50 entities)
    ✗ 3 critical path dependencies
    ✗ 8 high-risk downstream entities
    ⚠ 12 public APIs may need updates
    ✓ Well-tested (78% coverage)

  Risk Score: 7.2/10

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS

  Before Modifying:
    1. ✓ Review all 3 critical path callers
    2. ✓ Add integration tests for run_pipeline + handle_workflow
    3. ✓ Document signature changes in CHANGELOG
    4. ✗ Coordinate with team (affects 6 files)

  During Modification:
    5. Run full test suite (not just unit tests)
    6. Update documentation for 12 public API callers
    7. Consider feature flag for gradual rollout

  After Modification:
    8. Verify all 8 direct callers still work correctly
    9. Run performance benchmarks (benchmark_processing exists)
    10. Monitor production metrics (update_metrics instrumented)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡 NEXT STEPS

  View affected entities in detail:
    parseltongue pt07-cozodb-code-as-visuals --report entities \
      --filter "depends_on=process_entity"

  Export blast radius for documentation:
    parseltongue pt07-cozodb-code-as-visuals --report blast-radius \
      --entity "rust:fn:process_entity:..." --format json > impact.json

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 15ms (query: 8ms, render: 7ms)
```

---

## 5. Dependencies Report

**Command**: `parseltongue pt07-cozodb-code-as-visuals --report dependencies --db rocksdb:test.db`

**Output**:

```
DEPENDENCY HEALTH ANALYSIS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

COUPLING METRICS

  Metric                  Value    Status
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Avg dependencies        3.2      ✓ Good (target: ≤5)
  Max dependencies        12       ⚠ Review (threshold: 10)
  Median dependencies     2
  Zero dependencies       234      (35% of entities)
  High coupling (>8)      8        (1.2% of entities)
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Overall: ✓ HEALTHY COUPLING

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DEPENDENCY DISTRIBUTION

  Range      Count    Percent   Distribution
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  0 deps     234      35%       ██████████████████
  1-2        198      30%       ███████████████
  3-5        156      24%       ████████████
  6-8        58       9%        ████
  9-12       15       2%        █
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

HIGH-COUPLING ENTITIES (>8 dependencies)

┌────┬──────────────────────────┬──────┬──────────┬────────────┬──────────────────┐
│ #  │ Entity                   │ Deps │ Risk     │ Complexity │ Location         │
├────┼──────────────────────────┼──────┼──────────┼────────────┼──────────────────┤
│ 1  │ calculate_blast_radius   │ 12   │ High     │ Complex    │ cozo_client.rs   │
│ 2  │ parse_entity             │ 10   │ Medium   │ Complex    │ parser.rs        │
│ 3  │ validate_all             │ 9    │ High     │ Moderate   │ validator.rs     │
│ 4  │ serialize_graph          │ 9    │ Medium   │ Complex    │ storage.rs       │
│ 5  │ build_dependency_tree    │ 9    │ Medium   │ Moderate   │ tree.rs          │
│ 6  │ merge_contexts           │ 9    │ High     │ Moderate   │ context.rs       │
│ 7  │ run_full_pipeline        │ 9    │ High     │ Moderate   │ pipeline.rs      │
│ 8  │ init_system              │ 9    │ Low      │ Moderate   │ init.rs          │
└────┴──────────────────────────┴──────┴──────────┴────────────┴──────────────────┘

⚠ RECOMMENDATION: Review top 3 for refactoring opportunities

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FAN-IN ANALYSIS (Most-Called Functions)

Top 20 entities by number of callers:

┌────┬──────────────────────────┬─────────┬──────────┬──────────────────┐
│ #  │ Function                 │ Callers │ Type     │ Location         │
├────┼──────────────────────────┼─────────┼──────────┼──────────────────┤
│ 1  │ log_error                │ 47      │ Function │ logger.rs        │
│ 2  │ serialize_entity         │ 23      │ Function │ serializer.rs    │
│ 3  │ validate_key             │ 18      │ Function │ validation.rs    │
│ 4  │ get_timestamp            │ 16      │ Function │ utils.rs         │
│ 5  │ format_error_message     │ 15      │ Function │ error.rs         │
│ 6  │ db_execute_query         │ 14      │ Function │ db.rs            │
│ 7  │ sanitize_path            │ 12      │ Function │ utils.rs         │
│ 8  │ construct_isgl1_key      │ 11      │ Function │ keys.rs          │
│ 9  │ parse_json_field         │ 10      │ Function │ json_utils.rs    │
│ 10 │ check_temporal_state     │ 9       │ Function │ temporal.rs      │
│ 11 │ extract_line_range       │ 8       │ Function │ parser.rs        │
│ 12 │ resolve_file_path        │ 8       │ Function │ path_utils.rs    │
│ 13 │ hash_content             │ 7       │ Function │ hashing.rs       │
│ 14 │ validate_json            │ 7       │ Function │ json_utils.rs    │
│ 15 │ get_entity_type          │ 6       │ Function │ entities.rs      │
│ 16 │ escape_special_chars     │ 6       │ Function │ string_utils.rs  │
│ 17 │ init_logger              │ 5       │ Function │ logger.rs        │
│ 18 │ format_timestamp         │ 5       │ Function │ utils.rs         │
│ 19 │ calculate_hash           │ 5       │ Function │ hashing.rs       │
│ 20 │ read_file_content        │ 4       │ Function │ io.rs            │
└────┴──────────────────────────┴─────────┴──────────┴──────────────────┘

💡 TIP: High fan-in = core utilities → Test thoroughly!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FAN-OUT ANALYSIS (Functions That Call Many Others)

Top 20 entities by number of outgoing calls:

┌────┬──────────────────────────┬────────┬──────────┬──────────────────┐
│ #  │ Function                 │ Calls  │ Type     │ Location         │
├────┼──────────────────────────┼────────┼──────────┼──────────────────┤
│ 1  │ process_workflow         │ 15     │ Function │ workflow.rs      │
│ 2  │ run_pipeline             │ 12     │ Function │ pipeline.rs      │
│ 3  │ calculate_blast_radius   │ 12     │ Function │ cozo_client.rs   │
│ 4  │ init_system              │ 10     │ Function │ init.rs          │
│ 5  │ handle_entity_change     │ 9      │ Function │ handler.rs       │
│ 6  │ validate_all             │ 9      │ Function │ validator.rs     │
│ 7  │ parse_entity             │ 8      │ Function │ parser.rs        │
│ 8  │ build_context            │ 8      │ Function │ context.rs       │
│ 9  │ serialize_graph          │ 7      │ Function │ storage.rs       │
│ 10 │ apply_temporal_changes   │ 7      │ Function │ temporal.rs      │
│ 11 │ generate_diff            │ 6      │ Function │ diff_gen.rs      │
│ 12 │ merge_entities           │ 6      │ Function │ merger.rs        │
│ 13 │ format_output            │ 5      │ Function │ formatter.rs     │
│ 14 │ construct_response       │ 5      │ Function │ response.rs      │
│ 15 │ load_entities            │ 5      │ Function │ loader.rs        │
│ 16 │ validate_schema          │ 4      │ Function │ schema.rs        │
│ 17 │ execute_query            │ 4      │ Function │ query.rs         │
│ 18 │ render_table             │ 4      │ Function │ render.rs        │
│ 19 │ extract_metadata         │ 3      │ Function │ metadata.rs      │
│ 20 │ create_entity            │ 3      │ Function │ creator.rs       │
└────┴──────────────────────────┴────────┴──────────┴──────────────────┘

💡 TIP: High fan-out = orchestrators → Monitor complexity!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

CIRCULAR DEPENDENCIES

✓ No circular dependencies detected

  Scanned: 661 entities
  Checked: 2,134 edges
  Status:  CLEAN

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DEPENDENCY HEALTH BY MODULE

┌────────────────────┬───────────┬──────────┬───────────┐
│ Module             │ Avg Deps  │ Max Deps │ Status    │
├────────────────────┼───────────┼──────────┼───────────┤
│ storage            │ 4.2       │ 12       │ ⚠ Review  │
│ validator          │ 3.8       │ 9        │ ✓ OK      │
│ parser             │ 3.5       │ 10       │ ✓ OK      │
│ temporal           │ 2.9       │ 7        │ ✓ OK      │
│ utils              │ 1.2       │ 3        │ ✓ Good    │
│ entities           │ 2.1       │ 5        │ ✓ Good    │
│ cli                │ 2.4       │ 6        │ ✓ OK      │
└────────────────────┴───────────┴──────────┴───────────┘

⚠ storage module has highest coupling (review calculate_blast_radius)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS

  Immediate:
    1. Review calculate_blast_radius (12 deps) - consider extracting helpers
    2. Ensure log_error (47 callers) has 100% coverage
    3. Document serialize_entity (23 callers) - critical utility

  Architectural:
    4. Extract common utilities from high fan-in functions
    5. Consider dependency injection for process_workflow (15 calls)
    6. Review storage module coupling (avg 4.2, max 12)

  Monitoring:
    7. Set CI alert for new entities with >10 dependencies
    8. Track fan-in/fan-out trends over time
    9. Periodic circular dependency checks in CI

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 34ms
```

---

## 6. Changes Report

**Command**: `parseltongue pt07-cozodb-code-as-visuals --report changes --db rocksdb:test.db`

**Output** (when changes exist):

```
PENDING TEMPORAL CHANGES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Status: 14 pending changes across 7 files

Last Modified: 2025-11-01 14:32:18 UTC

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SUMMARY BY ACTION

  Action     Count   Entities
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Create     3       new_feature, helper_fn, Config
  Edit       9       process_entity, validate, ...
  Delete     2       deprecated_fn, OldStruct
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

CHANGES BY FILE

┌────────────────────────────┬────────┬──────────┬──────────┬──────────┐
│ File                       │ Create │ Edit     │ Delete   │ Total    │
├────────────────────────────┼────────┼──────────┼──────────┼──────────┤
│ src/features/new_feature.rs│ 2      │ 1        │ 0        │ 3        │
│ src/core.rs                │ 0      │ 3        │ 1        │ 4        │
│ src/validator.rs           │ 1      │ 2        │ 0        │ 3        │
│ src/utils.rs               │ 0      │ 2        │ 0        │ 2        │
│ src/config.rs              │ 0      │ 1        │ 0        │ 1        │
│ src/deprecated.rs          │ 0      │ 0        │ 1        │ 1        │
└────────────────────────────┴────────┴──────────┴──────────┴──────────┘

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DETAILED CHANGES

CREATE (3 entities)

┌────────────────────────────┬──────────┬──────────────────────────┐
│ Entity                     │ Type     │ Location                 │
├────────────────────────────┼──────────┼──────────────────────────┤
│ new_feature                │ Function │ features/new_feature.rs  │
│ helper_fn                  │ Function │ features/new_feature.rs  │
│ Config                     │ Struct   │ validator.rs             │
└────────────────────────────┴──────────┴──────────────────────────┘

─────────────────────────────────────────────────────────────────────────

EDIT (9 entities)

┌────────────────────────────┬──────────┬──────────────────────────┬──────────┐
│ Entity                     │ Type     │ Location                 │ Risk     │
├────────────────────────────┼──────────┼──────────────────────────┼──────────┤
│ process_entity             │ Function │ core.rs:42               │ High     │
│ validate                   │ Function │ validator.rs:15          │ Medium   │
│ check_syntax               │ Function │ validator.rs:88          │ High     │
│ sanitize_input             │ Function │ utils.rs:23              │ Low      │
│ format_output              │ Function │ utils.rs:67              │ Low      │
│ get_config_value           │ Function │ config.rs:12             │ Medium   │
│ FeatureBuilder             │ Struct   │ features/new_feature.rs  │ Low      │
│ run_workflow               │ Function │ core.rs:156              │ High     │
│ emit_metrics               │ Function │ core.rs:201              │ Low      │
└────────────────────────────┴──────────┴──────────────────────────┴──────────┘

⚠ 3 High-Risk edits - review carefully before applying

─────────────────────────────────────────────────────────────────────────

DELETE (2 entities)

┌────────────────────────────┬──────────┬──────────────────────────┬───────────┐
│ Entity                     │ Type     │ Location                 │ Callers   │
├────────────────────────────┼──────────┼──────────────────────────┼───────────┤
│ deprecated_fn              │ Function │ deprecated.rs:8          │ 0 (safe)  │
│ OldStruct                  │ Struct   │ core.rs:234              │ 2 ⚠       │
└────────────────────────────┴──────────┴──────────────────────────┴───────────┘

⚠ WARNING: OldStruct still has 2 callers - deletion may break code!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

VALIDATION STATUS

  ✓ Syntax validation:  14/14 entities passed
  ✓ Type checking:      14/14 entities passed
  ⚠ Dependency check:   1 warning (OldStruct has callers)

  Status: SAFE TO APPLY (with caution on OldStruct deletion)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

BLAST RADIUS ANALYSIS

  Total entities affected:  47 (via edits to high-risk functions)
  Files requiring review:   12
  Estimated LOC impact:     ~1,850 lines

  High-impact changes:
    → process_entity edit affects 18 downstream entities
    → check_syntax edit affects 12 downstream entities
    → run_workflow edit affects 17 downstream entities

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

NEXT STEPS

  1. Review changes:
     parseltongue pt02-llm-cozodb-to-context-writer \
       --filter changed --db rocksdb:test.db

  2. Validate all changes:
     parseltongue pt04-syntax-preflight-validator --db rocksdb:test.db

  3. Generate diff:
     parseltongue pt05-llm-cozodb-to-diff-writer \
       --output CodeDiff.json --db rocksdb:test.db

  4. Check OldStruct callers before deleting:
     parseltongue pt07-cozodb-code-as-visuals --report blast-radius \
       --entity "rust:struct:OldStruct:..."

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 11ms
```

**Output** (when no changes):

```
PENDING TEMPORAL CHANGES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Status: ✓ CLEAN - No pending changes

  All entities have temporal state (1,1,null) - unchanged

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Database in sync with codebase. No actions required.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 3ms
```

---

## 7. Entities Report (Filterable)

**Command**: `parseltongue pt07-cozodb-code-as-visuals --report entities --filter "entity_type=Function,risk=High" --limit 10`

**Output**:

```
ENTITY LISTING
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Filters: entity_type=Function, risk=High
Results: 10 of 18 total matches (use --limit to see more)

┌────┬─────────────────────────────┬────────────┬────────────┬──────────┬──────────┬──────────────────┐
│ #  │ Name                        │ Visibility │ Complexity │ Coverage │ Deps     │ Location         │
├────┼─────────────────────────────┼────────────┼────────────┼──────────┼──────────┼──────────────────┤
│ 1  │ calculate_blast_radius      │ Public     │ Complex    │ 45%      │ 12       │ cozo_client.rs   │
│ 2  │ parse_temporal_state        │ Public     │ Complex    │ 30%      │ 6        │ temporal.rs      │
│ 3  │ apply_diff                  │ Public     │ Complex    │ 35%      │ 8        │ diff_writer.rs   │
│ 4  │ validate_entity             │ Public     │ Complex    │ 65%      │ 7        │ validator.rs     │
│ 5  │ merge_contexts              │ Public     │ Moderate   │ 55%      │ 9        │ context.rs       │
│ 6  │ export_to_json              │ Public     │ Moderate   │ 0%       │ 5        │ export.rs        │
│ 7  │ validate_syntax             │ Public     │ Complex    │ 0%       │ 4        │ validator.rs     │
│ 8  │ process_entity              │ Public     │ Moderate   │ 78%      │ 4        │ core.rs          │
│ 9  │ run_pipeline                │ Public     │ Moderate   │ 82%      │ 12       │ pipeline.rs      │
│ 10 │ handle_workflow             │ Public     │ Moderate   │ 75%      │ 8        │ workflow.rs      │
└────┴─────────────────────────────┴────────────┴────────────┴──────────┴──────────┴──────────────────┘

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

8 more entities match (use --limit 20 to see all)

Report generated in 5ms
```

---

## 8. Modules Report

**Command**: `parseltongue pt07-cozodb-code-as-visuals --report modules --db rocksdb:test.db`

**Output**:

```
MODULE ORGANIZATION ANALYSIS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FILE HEALTH OVERVIEW

┌──────────────────────────────┬──────────┬─────────┬─────────────┬────────┐
│ File                         │ Entities │ LOC/Ent │ Avg Cplx    │ Status │
├──────────────────────────────┼──────────┼─────────┼─────────────┼────────┤
│ src/storage/cozo_client.rs   │ 25       │ 48      │ Moderate    │ ⚠ BIG  │
│ src/entities/code_entity.rs  │ 18       │ 35      │ Simple      │ ⚠ BIG  │
│ src/cli/main.rs              │ 12       │ 28      │ Simple      │ ✓ OK   │
│ src/temporal.rs              │ 8        │ 22      │ Simple      │ ✓ OK   │
│ src/validator.rs             │ 7        │ 31      │ Moderate    │ ✓ OK   │
│ src/parser.rs                │ 6        │ 26      │ Moderate    │ ✓ OK   │
│ src/utils.rs                 │ 5        │ 18      │ Simple      │ ✓ GOOD │
│ src/config.rs                │ 3        │ 12      │ Simple      │ ✓ GOOD │
│ ... 55 more files            │          │         │             │        │
└──────────────────────────────┴──────────┴─────────┴─────────────┴────────┘

⚠ 2 files exceed recommended entity count (15 entities)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

LARGE FUNCTIONS (>100 LOC)

┌──────────────────────────┬───────┬──────────────────────────┐
│ Function                 │ LOC   │ Location                 │
├──────────────────────────┼───────┼──────────────────────────┤
│ row_to_entity            │ 125   │ cozo_client.rs:980       │
│ parse_interface_sig...   │ 110   │ entities.rs:420          │
│ calculate_blast_radius   │ 105   │ cozo_client.rs:305       │
└──────────────────────────┴───────┴──────────────────────────┘

💡 Consider extracting helper functions

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MODULE DEPTH DISTRIBUTION

  Depth    Files    Percent   Distribution
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  1 level  45       71%       ████████████████████████████
  2 level  15       24%       █████████
  3 level  3        5%        ██
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Deepest: src/tests/integration/helpers/mod.rs (4 levels)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MODULE TREE (Top-Level)

src/
├── storage/ (28 entities, avg cplx: Moderate)
│   ├── cozo_client.rs (25)
│   └── mod.rs (3)
├── entities/ (45 entities, avg cplx: Simple)
│   ├── code_entity.rs (18)
│   ├── temporal.rs (12)
│   ├── tdd.rs (8)
│   └── mod.rs (7)
├── cli/ (23 entities, avg cplx: Simple)
│   ├── main.rs (12)
│   ├── args.rs (9)
│   └── mod.rs (2)
├── validator.rs (7 entities)
├── parser.rs (6 entities)
├── utils.rs (5 entities)
└── config.rs (3 entities)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS

  File Organization:
    1. Split cozo_client.rs (25 entities → target 15)
       → Extract query builders to separate module
       → Extract row conversion to serialization module

    2. Consider splitting code_entity.rs (18 entities)
       → Extract builder pattern to separate file

  Function Refactoring:
    3. Refactor row_to_entity (125 LOC)
       → Extract field parsers
       → Use builder pattern

    4. Simplify parse_interface_signature (110 LOC)
       → Extract language-specific parsing

  Module Structure:
    5. Flatten 3-level nesting in tests/
    6. Consider grouping validator.rs, parser.rs into analysis/ module

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report generated in 14ms
```

---

## 9. Quick Reference Guide

### All Report Types

```bash
# Dashboard (default - comprehensive overview)
parseltongue pt07-cozodb-code-as-visuals

# Complexity hotspots
parseltongue pt07-cozodb-code-as-visuals --report complexity

# Test coverage gaps
parseltongue pt07-cozodb-code-as-visuals --report coverage

# Dependency health
parseltongue pt07-cozodb-code-as-visuals --report dependencies

# Pending changes
parseltongue pt07-cozodb-code-as-visuals --report changes

# Blast radius (impact analysis)
parseltongue pt07-cozodb-code-as-visuals --report blast-radius \
  --entity "rust:fn:YOUR_FUNCTION:..."

# Entity listing (filterable)
parseltongue pt07-cozodb-code-as-visuals --report entities

# Module organization
parseltongue pt07-cozodb-code-as-visuals --report modules
```

### Common Filters

```bash
# High-risk functions only
--filter "entity_type=Function,risk=High"

# Public APIs
--filter "visibility=Public"

# Complex entities
--filter "complexity=Complex"

# Zero coverage
--filter "coverage=0"

# Multiple filters
--filter "entity_type=Function,risk=High,coverage<50"
```

### Output Formats

```bash
# Terminal table (default)
--format table

# JSON for CI/tooling
--format json

# CSV for spreadsheets
--format csv
```

### Useful Combinations

```bash
# Critical testing priorities
parseltongue pt07-cozodb-code-as-visuals --report coverage \
  --filter "visibility=Public,risk=High,coverage=0" \
  --limit 10

# Refactoring candidates
parseltongue pt07-cozodb-code-as-visuals --report complexity \
  --filter "complexity=Complex,dependencies>8" \
  --limit 20

# Export all high-risk entities for review
parseltongue pt07-cozodb-code-as-visuals --report entities \
  --filter "risk=High" \
  --format json > high_risk.json
```

---

## Design Notes

### Typography & Symbols Used

- **Box Drawing**: `─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼` (for tables)
- **Double Lines**: `═ ║ ╔ ╗ ╚ ╝ ╠ ╣` (for headers/emphasis)
- **Bars**: `█ ▓ ▒ ░` (for progress/distribution)
- **Bullets**: `• ⚠ ✓ ✗ ⚡ 📊 📈 🎯 💡 🧪 📝` (status indicators)
- **Separators**: `━` (section breaks)

### Color Strategy (Optional)

When `--color` enabled:
- **Green** (`✓`): Good/passing metrics
- **Yellow** (`⚠`): Warning/needs attention
- **Red** (`✗`): Critical/failing
- **Cyan**: Entity names, emphasis
- **Dim/Gray**: Less important data

### Layout Principles

1. **Header Section**: Report title, database info, timestamp
2. **Summary Section**: Key metrics at-a-glance
3. **Detail Tables**: Sorted by importance
4. **Distribution Charts**: Visual context
5. **Recommendations**: Actionable next steps
6. **Footer**: Quick commands, generation time

---

**End of Visual Mockups Document**
