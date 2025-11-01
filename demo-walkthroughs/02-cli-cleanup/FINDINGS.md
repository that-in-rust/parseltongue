# CLI Cleanup Findings - Parseltongue v0.7.0

## Executive Summary

The Parseltongue unified binary (`parseltongue`) defines simplified CLIs in `crates/parseltongue/src/main.rs` that override the individual crate CLIs. Many CLI arguments defined in individual crate `cli.rs` files are **unused** when tools are called via the unified binary.

---

## Architecture Discovery

### Two CLI Layers

1. **Unified Binary CLIs** (`crates/parseltongue/src/main.rs`):
   - Simple, minimal argument sets
   - This is what users actually interact with
   - Hardcoded in `run_{tool_name}()` functions

2. **Individual Crate CLIs** (`crates/*/src/cli.rs`):
   - Extensive argument definitions
   - Includes LLM integration arguments, optimization flags, etc.
   - **NOT used when called via unified binary**

---

## Actual CLI Arguments (Unified Binary)

### Tool 1: folder-to-cozodb-streamer
```
<directory>   (positional, required)
--db          Database file path [default: parseltongue.db]
--verbose     Enable verbose output
--quiet       Suppress output
```

### Tool 2: llm-to-cozodb-writer
```
--entity         ISGL1 key
--action         create, edit, delete
--future-code    Future code content
--db             Database file path [default: parseltongue.db]
```

### Tool 3: llm-cozodb-to-context-writer
```
--output    Output JSON file (required)
--db        Database file path [default: parseltongue.db]
--filter    all, changed, or current [default: all]
```

### Tool 4: rust-preflight-code-simulator
```
--db        Database file path [default: parseltongue.db]
--verbose   Show detailed errors
```

### Tool 5: llm-cozodb-to-diff-writer
```
--output    Output JSON file (required)
--db        Database file path [default: parseltongue.db]
```

### Tool 6: cozodb-make-future-code-current
```
--project   Project root directory (required)
--db        Database file path [default: parseltongue.db]
```

---

## Unused Arguments in Individual Crate CLIs

### Tool 1 Crate (folder-to-cozodb-streamer/src/cli.rs)
**Defined but unused in unified binary:**
- `--output-db` (alias for --db)
- `--parsing-library`
- `--chunking`
- `--max-size`
- `--include`
- `--exclude`

**Status:** `--verbose` and `--quiet` ARE used

### Tool 2 Crate (llm-to-cozodb-writer/src/cli.rs)
**Defined but unused in unified binary:**
- `--endpoint`
- `--api-key`
- `--model`
- `--max-tokens`
- `--temperature`
- `--query`
- `--batch-size`
- `--dry-run`
- `--verbose`
- `--quiet`

**Status:** Unified binary uses simplified single-entity API instead

### Tool 3 Crate (llm-cozodb-to-context-writer/src/cli.rs)
**Defined but unused in unified binary:**
- `--endpoint`
- `--api-key`
- `--model`
- `--max-tokens`
- `--temperature`
- `--query` (replaced by `--filter`)
- `--max-context-tokens`
- `--relevance-threshold`
- `--context-id`
- `--focus-areas`
- `--optimization-goals`
- `--dry-run`
- `--verbose`
- `--quiet`

**Status:** Tool 3 has LLM optimization infrastructure but unified binary uses simple JSON export

---

## Key Findings

### 1. Architectural Mismatch
The individual crates were designed with rich CLIs for standalone use, but the unified binary implements simplified wrappers that ignore most arguments.

### 2. LLM Integration Arguments
Tool 2 and Tool 3 crate CLIs define extensive LLM API arguments:
- `--endpoint`, `--api-key`, `--model`, `--max-tokens`, `--temperature`

These exist in the crate code but are **not wired up in the unified binary**.

### 3. Filter vs Query
Tool 3 demonstrates the discrepancy:
- **Crate CLI**: Uses `--query` with SQL strings (flexible)
- **Unified binary**: Uses `--filter` with enum values: `all`, `changed`, `current` (simplified)

The README.md and SOP documentation were using `--query` (from crate) but users interact with `--filter` (from unified binary).

### 4. Output Argument Behavior
Tool 3's `--output`:
- **Crate definition**: Takes directory, generates `context_{uuid}_{timestamp}.json`
- **Unified binary**: Takes file path, creates that exact file

Unified binary is simpler but less flexible.

---

## Documentation Impact

### Files Updated in This Demo

1. **README.md**: Fixed to use `--filter` instead of `--query`
2. **Parseltonge-SOP.md**: Updated to show `--query` only works with crate binary, not unified
3. **demo-walkthroughs/01-greeter-bug-fix/JOURNAL.md**: Fixed command examples

### Remaining Documentation Issues

Current SOP shows SQL query patterns like:
```bash
parseltongue llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph WHERE Future_Action IS NOT NULL"
```

But the unified binary doesn't support `--query` - only `--filter all|changed|current`.

**Recommendation**: Update SOP to document both:
- Unified binary usage (what users actually use)
- Crate binary usage (for advanced users)

---

## Recommendations

### Option 1: Remove Unused Arguments (Ultra-Minimalist)
Delete all unused CLI arguments from individual crate `cli.rs` files to match unified binary's minimal API.

**Pros:**
- Cleaner codebase
- No confusion about what arguments exist
- Aligns with ultra-minimalist philosophy

**Cons:**
- Loses flexibility if standalone crate binaries are ever needed
- Breaks backward compatibility if anyone uses crate binaries

### Option 2: Document the Dual-CLI Architecture
Keep both CLIs but clearly document:
- Unified binary = production CLI (minimal)
- Individual crates = development/testing CLI (full-featured)

**Pros:**
- Preserves flexibility
- Allows testing individual tools with rich options

**Cons:**
- Confusion about which CLI to document
- Maintenance burden (two CLIs per tool)

### Option 3: Wire Up Unused Arguments (Future Work)
Extend unified binary to support the rich CLI options defined in crates.

**Pros:**
- Users get full feature set
- No wasted code

**Cons:**
- Goes against ultra-minimalist principle
- Increases complexity

---

## Next Steps

1. **Decide on approach:** Remove, document, or wire up?
2. **Update all documentation** to reflect chosen approach
3. **Run this analysis on Tools 4-6** when implemented
4. **Update Parseltonge-SOP.md** with CLI decision

---

## Methodology Notes

This analysis was performed using Parseltongue on itself:

1. **Indexed** codebase with Tool 1 (660 entities from 62 files)
2. **Exported** entity signatures with Tool 3 to `step2-all-entities.json`
3. **Analyzed** JSON to extract:
   - CLI argument definitions in `build_cli()` functions
   - Argument usage in implementation code
4. **Tested** actual unified binary CLI with `--help`
5. **Compared** crate definitions vs unified binary behavior

**Key Learning:** Context JSON contains all code entities, eliminating need to read source files directly. Pure data-driven analysis!

---

**Analysis Date:** 2025-11-01
**Parseltongue Version:** v0.7.0
**Database:** `parseltongue-analysis.db` (660 entities)
