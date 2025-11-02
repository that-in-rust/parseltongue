# Parseltongue Standard Operating Procedure (SOP)

**Purpose**: Smart query patterns for using Parseltongue without token explosion.

**Updated**: v0.8.5 - Progressive Disclosure Design

---

## THE GOLDEN RULE

**Never load code for all entities** → 500k+ tokens = context explosion

**Solution**: Use **progressive disclosure** - pick the right level for your task

| Level | Tokens | Use When |
|-------|--------|----------|
| **Level 0** | 2-5K | "What depends on what?" ← **RECOMMENDED** (essence of ISG) |
| **Level 1** | 30K | "How do I refactor this?" |
| **Level 2** | 60K | "Is this type-safe?" |
| **Level 1 + code** | 500-700K | "Show me the implementation" (rarely!) |

---

## CLI COMMAND FORMAT

All Parseltongue tools use the unified binary:

```bash
parseltongue <tool-name> [arguments]
```

### Tool 1: pt01-folder-to-cozodb-streamer

```bash
# Index current directory (default)
parseltongue pt01-folder-to-cozodb-streamer .

# Index specific directory with custom database
parseltongue pt01-folder-to-cozodb-streamer ./crates --db rocksdb:analysis.db --verbose
```

**Key points:**
- `<directory>` is positional [default: `.`]
- Processes ALL files - tree-sitter determines what it can parse
- Gracefully skips non-code files (.md, .json, .toml, etc.)

---

## TOOL 2: PT02 PROGRESSIVE DISCLOSURE (v0.8.5)

Three levels, one goal: **Give LLMs exactly what they need, nothing more**.

### PT02-Level00: Pure Edge List (RECOMMENDED)
```bash
parseltongue pt02-level00 --where-clause "ALL" --output edges.json
```
- **Tokens**: ~2-5K
- **Use case**: "What depends on what?" - Pure dependency analysis (essence of ISG)
- **Output**: Just edges (from_key, to_key, edge_type)
- **Why RECOMMENDED**: The dependency graph IS the Interface Signature Graph's core value

### PT02-Level01: Entity + ISG + Temporal
```bash
# Signatures only (CHEAP)
parseltongue pt02-level01 --include-code 0 --where-clause "ALL" --output entities.json

# With code (EXPENSIVE - only when needed!)
parseltongue pt02-level01 --include-code 1 --where-clause "future_action != null" --output changes.json
```
- **Tokens**: ~30K (signatures) or ~500-700K (with code)
- **Use case**: "How do I refactor this?" - Code understanding, planning
- **Output**: 14 fields (isgl1_key, forward_deps, reverse_deps, temporal state, etc.)

### PT02-Level02: + Type System
```bash
# Find async functions
parseltongue pt02-level02 --include-code 0 --where-clause "is_async = true" --output async.json

# Find unsafe code
parseltongue pt02-level02 --include-code 0 --where-clause "is_unsafe = true" --output unsafe.json
```
- **Tokens**: ~60K (signatures) or ~500-700K (with code)
- **Use case**: "Is this type-safe?" - Safety audits, API analysis
- **Output**: 22 fields (all Level 1 + return_type, param_types, is_async, is_unsafe, etc.)

---

## DATALOG QUERY PATTERNS (v0.8.5)

### Pattern 1: Overview Without Code (DEFAULT - SAFE!)

**When**: Phase 2 - Understanding codebase structure
**Token Cost**: ~30K tokens for 1500 entities

```bash
# Level 1: Entity + ISG + Temporal (signatures only - RECOMMENDED)
parseltongue pt02-level01 \
  --include-code 0 \
  --where-clause "ALL" \
  --output entities.json \
  --db rocksdb:analysis.db
```

**What happens**: Exports entities with ISG + temporal state, NO code (just signatures)

---

### Pattern 2: Changed Entities WITH Code

**When**: Phase 3 - Implementing changes, need code for editing
**Token Cost**: ~10k additional (only changed rows)

```bash
# Export entities with planned changes (with code)
parseltongue pt02-level01 \
  --include-code 1 \
  --where-clause "future_action != null" \
  --output changes.json \
  --db rocksdb:analysis.db
```

**Why safe**: WHERE clause limits to specific rows being modified

---

### Pattern 3: Specific Entity Inspection

**When**: Need to see code for ONE specific function
**Token Cost**: ~100 tokens

```bash
# Export specific entity with code using Datalog pattern matching
parseltongue pt02-level01 \
  --include-code 1 \
  --where-clause "isgl1_key = 'rust:fn:calculate:src_lib_rs:42-56'" \
  --output specific_entity.json \
  --db rocksdb:analysis.db
```

**Why safe**: WHERE clause = single row only

---

### Pattern 4: All Signatures (No Code)

**When**: Need complete list of all entities
**Token Cost**: ~30k for 1500 entities

```bash
# Level 1 export without code (default safe mode)
parseltongue pt02-level01 \
  --include-code 0 \
  --where-clause "ALL" \
  --output all_entities.json \
  --db rocksdb:analysis.db
```

**Why safe**: --include-code 0 excludes code content

---

### Pattern 5: Include Current_Code for Debugging (USE SPARINGLY!)

**When**: Need to debug by seeing actual current code for all entities
**Token Cost**: ~500k for 1500 entities (13x larger!)

```bash
# Include code for ALL entities (DANGEROUS - only for debugging!)
parseltongue pt02-level01 \
  --include-code 1 \
  --where-clause "ALL" \
  --output all_with_code.json \
  --db rocksdb:analysis.db
```

**Why dangerous**: --include-code 1 with "ALL" → includes Current_Code for ALL entities → massive token explosion
**When to use**: Only for debugging when signatures alone aren't enough

---

## ANTI-PATTERN (NEVER DO THIS!)

```bash
# ❌ CONTEXT EXPLOSION - 500k+ tokens
parseltongue pt02-level01 \
  --include-code 1 \
  --where-clause "ALL" \
  --output explosion.json \
  --db rocksdb:analysis.db
```

**Why fails**: --include-code 1 with "ALL" → loads Current_Code for ALL 1500 entities → token explosion

---

## ITERATIVE WORKFLOW

```bash
# Iteration 1: Overview (Pattern 1 - no code)
parseltongue pt02-level01 \
  --include-code 0 \
  --where-clause "ALL" \
  --output entities.json \
  --db rocksdb:analysis.db

# Mark changes with Tool 3 (simple interface)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() -> &'static str { \"Hello!\" }" \
  --db rocksdb:analysis.db

# Iteration 2: Review changes (Pattern 2 - with code)
parseltongue pt02-level01 \
  --include-code 1 \
  --where-clause "future_action != null" \
  --output changes.json \
  --db rocksdb:analysis.db

# Repeat until confident ≥80%
```

---

## QUERY DECISION TREE

```
Need overview of codebase?
├─ YES → Use default (Pattern 1)
└─ NO → Continue

Need to implement changes?
├─ YES → Pattern 2 (WHERE Future_Action != NULL)
└─ NO → Continue

Need to inspect ONE entity?
├─ YES → Pattern 3 (WHERE isgl1_key = '...')
└─ NO → Continue

Need all signatures?
└─ YES → Pattern 4 (EXCEPT code)
```

---

## KEY PRINCIPLES

**Two ways to avoid explosion:**

1. **Use --include-code 0** to exclude code content (signatures only):
   ```bash
   parseltongue pt02-level01 --include-code 0 --where-clause "ALL" --output entities.json
   ```

2. **Use WHERE clause** to limit rows:
   ```bash
   parseltongue pt02-level01 --include-code 1 --where-clause "future_action != null" --output changes.json
   ```

**Never**: Use `--include-code 1 --where-clause "ALL"` unless debugging

---

## QUERY BY PHASE

| Phase | Export Command | Purpose |
|-------|----------------|---------|
| Phase 2: MicroPRD | pt02-level01 --include-code 0 --where-clause "ALL" | Understand structure |
| Phase 3: Planning | pt02-level01 --include-code 0 --where-clause "ALL" | Plan changes |
| Phase 3: Implementing | pt02-level01 --include-code 1 --where-clause "future_action != null" | Write code |
| Phase 3: Inspecting | pt02-level01 --include-code 1 --where-clause "isgl1_key = '...'" | Debug specific entity |
| Phase 4: Validation | pt02-level01 --include-code 1 --where-clause "future_action != null" | Final review |

---

**Last Updated**: 2025-11-02
**Core Learning**: Use pt02-level00/01/02 progressive disclosure, NOT the old pt02-llm-cozodb-to-context-writer command.
