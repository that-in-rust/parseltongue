# Parseltonge Standard Operating Procedure (SOP)

**Purpose**: SQL query patterns for using Parseltongue tools without context pollution.

---

## THE GOLDEN RULE

**Never load `Current_Code` for all entities** → 500k+ tokens = context explosion

**Solution**: Use SQL `EXCEPT` clause OR `WHERE` clause to limit rows

---

## SQL QUERY PATTERNS

### Pattern 1: Overview Without Code (DEFAULT - SAFE!)

**When**: Phase 2 - Understanding codebase structure
**Token Cost**: ~37.5k for 1500 entities

```bash
parseltongue llm-cozodb-to-context-writer \
  --output overview.json \
  --db rocksdb:analysis.db
```

**What happens**: Uses default query (already excludes code!)
```sql
SELECT * EXCEPT (Current_Code, Future_Code)
FROM CodeGraph
WHERE current_ind=1
```

---

### Pattern 2: Changed Entities WITH Code

**When**: Phase 3 - Implementing changes, need code for editing
**Token Cost**: ~10k additional (only changed rows)

```bash
parseltongue llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL" \
  --output changes.json \
  --db rocksdb:analysis.db
```

**Why safe**: WHERE clause limits to specific rows being modified

---

### Pattern 3: Specific Entity Inspection

**When**: Need to see code for ONE specific function
**Token Cost**: ~100 tokens

```bash
parseltongue llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE isgl1_key = 'rust:fn:calculate:src_lib_rs:42-56'" \
  --output single.json \
  --db rocksdb:analysis.db
```

**Why safe**: WHERE clause = single row only

---

### Pattern 4: All Signatures (No Code)

**When**: Need complete list of all entities
**Token Cost**: ~50k for 1500 entities

```bash
parseltongue llm-cozodb-to-context-writer \
  --query "SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph" \
  --output all-sigs.json \
  --db rocksdb:analysis.db
```

**Why safe**: EXCEPT removes code columns

---

## ANTI-PATTERN (NEVER DO THIS!)

```bash
# ❌ CONTEXT EXPLOSION - 500k+ tokens
parseltongue llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph" \
  --output explosion.json \
  --db rocksdb:analysis.db
```

**Why fails**: No EXCEPT, no WHERE → loads Current_Code for ALL 1500 entities

---

## ITERATIVE WORKFLOW

```bash
# Iteration 1: Overview (Pattern 1 - no code)
parseltongue llm-cozodb-to-context-writer \
  --output iter1-overview.json \
  --db rocksdb:analysis.db

# Mark changes with Tool 2 (query selects entities to process)
parseltongue llm-to-cozodb-writer \
  --query "SELECT * FROM CodeGraph WHERE current_ind=1 LIMIT 10" \
  --db rocksdb:analysis.db

# Iteration 2: Review changes (Pattern 2 - with code)
parseltongue llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL" \
  --output iter2-changes.json \
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

1. **EXCEPT clause** removes code columns:
   ```sql
   SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph
   ```

2. **WHERE clause** limits rows:
   ```sql
   SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL
   ```

**Never**: `SELECT * FROM CodeGraph` without EXCEPT or WHERE

---

## QUERY BY PHASE

| Phase | Query Pattern | Purpose |
|-------|---------------|---------|
| Phase 2: MicroPRD | Pattern 1 (default) | Understand structure |
| Phase 3: Planning | Pattern 1 (default) | Plan changes |
| Phase 3: Implementing | Pattern 2 (WHERE Future_Action != NULL) | Write code |
| Phase 3: Inspecting | Pattern 3 (WHERE isgl1_key) | Debug specific entity |
| Phase 4: Validation | Pattern 2 (WHERE Future_Action != NULL) | Final review |

---

**Last Updated**: 2025-11-01
**Core Learning**: Use `--query` with SQL, not fictional `--filter` argument.
