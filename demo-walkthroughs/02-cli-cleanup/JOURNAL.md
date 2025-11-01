# Example 2: CLI Cleanup - Using Parseltongue on Itself

## Problem Statement
Many CLI arguments exist across the 6 tools, but some may be unused. Remove unnecessary arguments.

## Approach: Use Parseltongue Context Only
**Constraint**: Don't read source files directly. Use only Parseltongue-exported context to understand code.

---

## Phase 1: Index and Understand Structure

### Step 1: Index Codebase
```bash
../target/release/parseltongue folder-to-cozodb-streamer ../crates \
  --db rocksdb:parseltongue-analysis.db
```

**Expected**: ~200-300 entities (all CLI definitions, main functions, struct fields)

---

### Step 2: Export All Entity Signatures
```bash
../target/release/parseltongue llm-cozodb-to-context-writer \
  --output . \
  --db rocksdb:parseltongue-analysis.db
# Uses default: excludes Current_Code, loads signatures only
# Generates: context_{uuid}_{timestamp}.json
```

**What to look for in generated JSON**:
- Entities with `clap::Arg` in signatures
- Struct fields defining CLI arguments
- Function signatures showing argument usage

**Expected**: Interface signatures showing CLI arg definitions and usage

---

## Phase 2: Analyze Unused Arguments

**Manual analysis of the generated JSON**:
1. List all CLI arg definitions (struct fields, Arg::new() calls)
2. Search for each arg name in function signatures
3. If arg appears ONLY in definition, never in usage → **unused candidate**

**Document findings**: Which args are unused in which tools

---

## Phase 3: Plan Removal (Create Temporal Changes)

### Step 4: Mark Unused Args for Deletion
For each unused argument found, plan the deletion:

**Example** (if we find `--verbose` is unused in Tool 2):
```bash
# This would be done via direct CozoDB updates since Tool 2
# is for LLM batch processing, not single-entity marking
```

**Create list**: Document which args to remove in which tools

---

## Phase 4: Validate and Apply

### Step 5: Validate Syntax
```bash
../target/release/parseltongue rust-preflight-code-simulator \
  --db rocksdb:parseltongue-analysis.db
```

**Expected**: All syntax valid after deletions

---

### Step 6: Generate Diff
```bash
../target/release/parseltongue llm-cozodb-to-diff-writer \
  --output ./CodeDiff.json \
  --db rocksdb:parseltongue-analysis.db
```

**Expected**: CodeDiff.json showing Delete operations for unused CLI args

---

## Key Insight: Parseltongue as Analysis Tool

By using **only exported context** (interface signatures, entity relationships), we can:
- Understand CLI argument definitions
- Trace usage through function signatures
- Identify unused arguments
- Plan deletions

**No source file reading needed** - the interface signatures tell us everything!

---

## Status
🟡 Ready to start - Phase 1: Indexing
