# Parseltongue Commands Index

**Quick Reference for All CLI Commands**  
**Document**: `C01-commands-20251031.md`  
**Version**: 1.0  
**Updated**: 2025-10-31

## Quick Start

All tools are run via `cargo run --package <tool-name> -- [OPTIONS]`

## Tools at a Glance

| # | Tool | Binary | Purpose | Status |
|---|------|--------|---------|--------|
| 1 | folder-to-cozodb-streamer | `parseltongue-01` | Index codebase + dependencies → CozoDB | ✅ Complete (Phase 2: Dependency extraction) |
| 2 | llm-to-cozodb-writer | `parseltongue-02` | LLM changes → CozoDB temporal | ✅ Complete |
| 3 | llm-cozodb-to-context-writer | `parseltongue-03` | CozoDB → CodeGraphContext.json + real deps | ✅ Complete (Phase 4.1: Real dependency integration) |
| 4 | rust-preflight-code-simulator | (binary) | Validate syntax | ✅ Complete |
| 5 | llm-cozodb-to-diff-writer | (binary) | CozoDB → CodeDiff.json | ✅ Complete |
| 6 | cozodb-make-future-code-current | (binary) | Reset state & re-index | ✅ Complete |

## Most Common Commands

### Tool 1: Index Project
```bash
cargo run --package folder-to-cozodb-streamer -- --dir ./src --output-db sqlite:parseltongue.db --verbose
```

### Tool 2: Generate LLM Changes (Dry-Run)
```bash
cargo run --package llm-to-cozodb-writer -- --db parseltongue.db --api-key sk-... --dry-run --verbose
```

### Tool 3: Generate Context
```bash
cargo run --package llm-cozodb-to-context-writer -- --db parseltongue.db --api-key sk-... --output ./contexts --verbose
```

### Tool 4: Validate Syntax
```bash
cargo run --package rust-preflight-code-simulator -- --database parseltongue.db --verbose
```

### Tool 5: Generate Diff
```bash
cargo run --package llm-cozodb-to-diff-writer -- --database parseltongue.db --output ./CodeDiff.json
```

### Tool 6: Reset State
```bash
cargo run --package cozodb-make-future-code-current -- --database parseltongue.db --project-path . --verbose
```

## Environment Variables

```bash
export OPENAI_API_KEY="sk-..."  # For Tools 2 & 3
```

## Key Flags (All Tools)

| Flag | Purpose |
|------|---------|
| `--verbose` / `-v` | Detailed output |
| `--quiet` / `-q` | Suppress output |
| `--dry-run` / `-d` | Test without changes (Tool 2, 3) |
| `--help` | Show help text |
| `--version` | Show version |

## Default Behaviors

- **Database**: `parseltongue.db` (Tools 2, 3, 5) or `mem` (Tool 1, 4)
- **Output Directory**: `./contexts` (Tool 3)
- **Model**: `gpt-4` (Tools 2, 3)
- **Temperature**: `0.7` (Tool 2), `0.3` (Tool 3)

## Performance Targets

- Tool 1: <30s for 50k LOC
- Tool 3: <500ms context generation
- Tool 4: O(n) validation
- Tool 5: <100ms diff generation

### Dependency Query Performance (Phase 3)

- Blast Radius (5 hops, 10k nodes): <50ms (validated: 8ms in release mode)
- Forward Dependencies (1 hop, 10k nodes): <20ms (validated: 12ms in release mode)
- Transitive Closure (1k nodes): <50ms (validated: 12ms in release mode)
- All queries use CozoDB recursive Datalog with fixed-point semantics

## Exit Codes

- `0`: Success
- `1`: Error (check stderr for details)

## Documentation

**Full Reference**: See `C01-commands-20251031.md` for:
- Complete argument list with types and defaults
- All usage examples
- Input/output specifications
- Error handling details
- Performance characteristics
- Optimization options

## Architecture

Pipeline execution order:
```
Tool 1: Index + Dependencies → CozoDB (Phase 2: Dependency extraction)
  ↓
Tool 2: LLM reasoning → Temporal updates
  ↓
Tool 3: Context generation → JSON (Phase 4.1: Real dependency relationships)
  │        (Dependency queries: blast radius, forward/reverse deps, transitive closure)
  ↓
Tool 4: Validation → Syntax check
  ↓
Tool 5: Diff generation → JSON
  ↓
Tool 6: State reset → Clean database
```

## Getting Help

```bash
# Show help for any tool
cargo run --package <tool-name> -- --help

# Examples
cargo run --package folder-to-cozodb-streamer -- --help
cargo run --package llm-to-cozodb-writer -- --help
cargo run --package llm-cozodb-to-context-writer -- --help
cargo run --package rust-preflight-code-simulator -- --help
cargo run --package llm-cozodb-to-diff-writer -- --help
cargo run --package cozodb-make-future-code-current -- --help
```

---

For detailed information, see the complete command reference: `C01-commands-20251031.md`
