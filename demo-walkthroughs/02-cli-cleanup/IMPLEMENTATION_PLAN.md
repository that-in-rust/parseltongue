# CLI Cleanup Implementation Plan

**Philosophy**: Ultra-minimalist (S01) - Remove complexity, keep only what's used

---

## Changes to Make

### Tool 1: folder-to-cozodb-streamer

**Keep (used by unified binary):**
- `<directory>` (positional)
- `--db`
- `--verbose`
- `--quiet`

**Remove (unused):**
- `--output-db` (alias, redundant)
- `--parsing-library` (not wired up)
- `--chunking` (not wired up)
- `--max-size` (not wired up)
- `--include` (not wired up)
- `--exclude` (not wired up)

**File**: `crates/folder-to-cozodb-streamer/src/cli.rs`

---

### Tool 2: llm-to-cozodb-writer

**Keep (used by unified binary):**
- `--entity`
- `--action`
- `--future-code`
- `--db`

**Remove (unused):**
- `--endpoint` (LLM not used in unified binary)
- `--api-key` (LLM not used)
- `--model` (LLM not used)
- `--max-tokens` (LLM not used)
- `--temperature` (LLM not used)
- `--query` (not used in unified binary)
- `--batch-size` (not used)
- `--dry-run` (not used)
- `--verbose` (not used)
- `--quiet` (not used)

**File**: `crates/llm-to-cozodb-writer/src/cli.rs`

---

### Tool 3: llm-cozodb-to-context-writer

**Keep (used by unified binary):**
- `--output`
- `--db`
- `--filter` (NOTE: unified binary uses this, not `--query`)

**Remove (unused):**
- `--endpoint` (Tool 3 doesn't call LLM!)
- `--api-key` (Tool 3 doesn't call LLM!)
- `--model` (Tool 3 doesn't call LLM!)
- `--max-tokens` (not used)
- `--temperature` (not used)
- `--query` (replaced by --filter in unified binary)
- `--max-context-tokens` (not used)
- `--relevance-threshold` (not used)
- `--context-id` (auto-generated)
- `--focus-areas` (not used)
- `--optimization-goals` (not used)
- `--dry-run` (not used)
- `--verbose` (not used)
- `--quiet` (not used)

**CRITICAL**: Tool 3 should NOT have LLM infrastructure. It exports JSON, LLM reads JSON separately.

**Files**:
- `crates/llm-cozodb-to-context-writer/src/cli.rs`
- `crates/llm-cozodb-to-context-writer/src/llm_client.rs` (entire file can be removed?)
- `crates/llm-cozodb-to-context-writer/src/lib.rs` (check LLM dependencies)

---

## Architecture Documentation

Add to top of each crate's `src/lib.rs`:

```rust
//! # CLI Architecture
//!
//! This crate has two CLI modes:
//!
//! 1. **Unified Binary** (production): Defined in `parseltongue/src/main.rs`
//!    - Minimal arguments
//!    - Used when calling `parseltongue {tool-name}`
//!
//! 2. **Standalone Binary** (deprecated): Defined in this crate's `cli.rs`
//!    - Ultra-minimalist: Only arguments actually used by unified binary
//!    - Standalone binaries NOT recommended for production use
//!
//! ## Unified Binary Arguments
//!
//! [List actual arguments here]
//!
//! ## Philosophy
//!
//! Following S01 ultra-minimalist principles:
//! - NO unused arguments
//! - NO configuration complexity
//! - Single reliable operations only
```

---

## README.md Updates

Add new section:

```markdown
## CLI Architecture

Parseltongue uses a **unified binary** design:

```bash
parseltongue <tool-name> [options]
```

All CLI arguments are defined in `crates/parseltongue/src/main.rs` for consistency.

### Why Unified Binary?

1. **Single source of truth** for all commands
2. **Self-documenting** - tool names match crate architecture
3. **Minimal** - only arguments actually used
4. **Consistent** - same `--db` flag across all tools

### Individual Crate Binaries (Not Recommended)

Each crate can be built as a standalone binary, but this is NOT recommended:

```bash
cargo run --package folder-to-cozodb-streamer -- [options]
```

The unified binary should always be used instead.
```

---

## Parseltonge-SOP.md Updates

Update to clarify:

```markdown
## IMPORTANT: CLI Architecture

All commands use the **unified binary**:

```bash
parseltongue {tool-name} [options]
```

NOT the individual crate binaries. This ensures:
- Consistent argument names across tools
- No unused/confusing options
- Ultra-minimalist design (S01 principle)
```

---

## Implementation Steps

### 1. Backup and Branch
```bash
git checkout -b cli-cleanup
```

### 2. Remove Unused Arguments (TDD: Check tests first)
```bash
cargo test --workspace  # Baseline
```

Then edit each `cli.rs`:
- Remove unused `Arg::new()` definitions
- Remove unused config struct fields
- Remove unused parsing logic

### 3. Add Documentation
- Add header comments to each crate's `lib.rs`
- Update README.md
- Update Parseltonge-SOP.md

### 4. Test
```bash
cargo test --workspace
cargo build --release
./target/release/parseltongue folder-to-cozodb-streamer --help
./target/release/parseltongue llm-to-cozodb-writer --help
./target/release/parseltongue llm-cozodb-to-context-writer --help
```

### 5. Document in Demo
Update `demo-walkthroughs/02-cli-cleanup/README.md` with results

---

## Expected Impact

### Code Removed
- ~200 lines from Tool 1 cli.rs
- ~400 lines from Tool 2 cli.rs
- ~500 lines from Tool 3 cli.rs + llm_client.rs

### Clarity Gained
- No confusion about `--query` vs `--filter`
- No LLM arguments in tools that don't call LLMs
- Clear documentation of unified binary architecture

### S01 Alignment
- ✅ NO configuration complexity
- ✅ Single reliable operations
- ✅ Ultra-minimalist (10 users, simple)
- ✅ NO multiple safety levels

---

**Next**: Execute this plan step by step with TDD discipline (test before/after each change)
