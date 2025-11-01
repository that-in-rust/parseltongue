# Parseltongue Standard Operating Procedure (SOP)

**Purpose**: Critical principles for using Parseltongue tools effectively without context pollution or workflow errors.

---

## 1. CONTEXT MANAGEMENT: The Golden Rule

**❌ NEVER DO THIS:**
```bash
# This will explode your context (500k+ tokens for 1500 entities)
parseltongue llm-cozodb-to-context-writer \
  --filter all \
  --db rocksdb:analysis.db
```

**✅ ALWAYS DO THIS:**
```bash
# Phase 2: Interface signatures only (37.5k tokens for 1500 entities)
parseltongue llm-cozodb-to-context-writer \
  --filter current \
  --db rocksdb:analysis.db

# Phase 3: Only changing rows with code
parseltongue llm-cozodb-to-context-writer \
  --filter changed \
  --db rocksdb:analysis.db
```

**Why**: `current_code` column contains full source code. Loading it for all entities = catastrophic context bloat. Interface signatures give you enough to reason about changes.

**Exception**: Only load `current_code`/`future_code` for rows actually being modified (filter=changed).

---

## 2. ITERATIVE WORKFLOW: Read-Edit-Read-Edit

**Pattern**:
1. **READ** context (Tool 3: `--filter changed`)
2. **EDIT** temporal state (Tool 2: propose changes)
3. **READ** again (Tool 3: verify changes)
4. **REPEAT** until confidence ≥80%

**Why**: Single-pass rarely gets complex changes right. The database IS your working memory.

**Anti-pattern**: Rushing to Tool 4 validation before thoroughly reasoning through changes in multiple read-edit cycles.

---

## 3. CONFIDENCE GATING: Don't Rush Validation

**Rule**: Only move to Tool 4 (validation) when LLM confidence ≥80% after multiple iterations.

**Why**: Pre-flight validation (Tool 4) is cheap (<20ms), but cargo build/test failures send you back to Step A01. Better to iterate in temporal space first.

**Signs you're ready**:
- Multiple read-edit cycles completed
- All edge cases considered
- Test coverage looks comprehensive

**Signs you're NOT ready**:
- First iteration
- Still discovering affected code
- Uncertain about dependencies

---

## 4. VALIDATION SCOPE: Understand What Each Tool Does

**Tool 4 (rust-preflight-code-simulator)**:
- ✓ Syntax validation (tree-sitter)
- ✓ Fast (<20ms)
- ✗ Does NOT validate types/imports/lifetimes
- ✗ Does NOT run tests

**Cargo build/test (after Tool 5 applies changes)**:
- ✓ Type checking
- ✓ Import resolution
- ✓ Lifetime validation
- ✓ Test execution

**Why**: Tool 4 is a pre-flight check. Real validation happens with cargo. Don't expect Tool 4 to catch everything.

---

## 5. FILTER STRATEGY: Use the Right Filter for Each Phase

| Phase | Filter | What It Loads | Token Cost |
|-------|--------|---------------|------------|
| Phase 2: MicroPRD | `current` | Signatures only, NO code | ~37.5k for 1500 entities |
| Phase 3: Iteration | `changed` | Code ONLY for changing rows | ~10k additional |
| Phase 4: Validation | `changed` | Same as Phase 3 | Same |
| Phase 5: Review | `changed` | Final state before commit | Same |

**Why**: Each phase needs different granularity. Loading everything always = context explosion.

---

## Common Mistakes Summary

1. **Loading `current_code` for all entities** → 500k+ tokens, context overflow
2. **Skipping read-edit iterations** → Rushing to validation, more failures
3. **Expecting Tool 4 to catch type errors** → It's syntax-only, cargo catches types
4. **Using `--filter all` in Phase 3** → Loads unchanged code unnecessarily
5. **Not documenting confidence level** → Hard to know when to proceed

---

## Quick Reference: Which Tool When?

```
Tool 1: Once per project (indexing)
Tool 3: Multiple times (read context, --filter current then --filter changed)
Tool 2: Multiple times (edit temporal state, iterate until confident)
Tool 4: Once when confident (pre-flight syntax check)
Tool 5: Once (generate diff)
Tool 6: Once after user approval (reset state)
```

---

**Last Updated**: 2025-11-01
**Based On**: P00.md, P01PRDL1Minimal.md learnings on context optimization
