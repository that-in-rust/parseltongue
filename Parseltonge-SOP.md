# Parseltonge Standard Operating Procedure (SOP)

**Purpose**: SQL query patterns for using Parseltongue tools without context pollution.

---

## THE GOLDEN RULE

**Never load `Current_Code` for all entities** → 500k+ tokens = context explosion

**Solution**: Use SQL `EXCEPT` clause OR `WHERE` clause to limit rows

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

## TOOL 2 BEHAVIOR

Tool 2 (`pt02-llm-cozodb-to-context-writer`):
- **Purpose**: Extract entities from CozoDB → JSON file for LLM to read
- **Input**: `--db` (database path), `--query` (optional SQL), `--output` (directory), `--include-current-code` (0 or 1)
- **Output**: Writes JSON file: `context_{id}_{timestamp}.json`
- **Default query**: `SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph WHERE current_ind=1`
- **Token optimization**: Use `--include-current-code 0` (default) to exclude Current_Code, saves ~500k tokens
- **Workflow**: Database → JSON file → **LLM reads JSON** to understand codebase

---

## SQL QUERY PATTERNS

### Pattern 1: Overview Without Code (DEFAULT - SAFE!)

**When**: Phase 2 - Understanding codebase structure
**Token Cost**: ~37.5k for 1500 entities

```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:analysis.db
# Generates: ./contexts/context_{uuid}_{timestamp}.json
# Default: --include-current-code 0 (excludes Current_Code)
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
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL" \
  --output ./contexts \
  --db rocksdb:analysis.db
```

**Why safe**: WHERE clause limits to specific rows being modified

---

### Pattern 3: Specific Entity Inspection

**When**: Need to see code for ONE specific function
**Token Cost**: ~100 tokens

```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE isgl1_key = 'rust:fn:calculate:src_lib_rs:42-56'" \
  --output ./contexts \
  --db rocksdb:analysis.db
```

**Why safe**: WHERE clause = single row only

---

### Pattern 4: All Signatures (No Code)

**When**: Need complete list of all entities
**Token Cost**: ~50k for 1500 entities

```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph" \
  --output ./contexts \
  --db rocksdb:analysis.db
```

**Why safe**: EXCEPT removes code columns

---

### Pattern 5: Include Current_Code for Debugging (USE SPARINGLY!)

**When**: Need to debug by seeing actual current code for all entities
**Token Cost**: ~500k for 1500 entities (13x larger!)

```bash
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:analysis.db \
  --include-current-code 1
# Uses modified query: SELECT * EXCEPT (Future_Code) FROM CodeGraph WHERE current_ind=1
```

**Why dangerous**: Includes Current_Code for ALL entities → massive token explosion
**When to use**: Only for debugging when signatures alone aren't enough

---

## ANTI-PATTERN (NEVER DO THIS!)

```bash
# ❌ CONTEXT EXPLOSION - 500k+ tokens
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph" \
  --output ./contexts \
  --db rocksdb:analysis.db
```

**Why fails**: No EXCEPT, no WHERE → loads Current_Code for ALL 1500 entities

---

## ITERATIVE WORKFLOW

```bash
# Iteration 1: Overview (Pattern 1 - no code)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./contexts \
  --db rocksdb:analysis.db

# Mark changes with Tool 3 (simple interface)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action edit \
  --future-code "pub fn hello() -> &'static str { \"Hello!\" }" \
  --db rocksdb:analysis.db

# Iteration 2: Review changes (Pattern 2 - with code)
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL" \
  --output ./contexts \
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
