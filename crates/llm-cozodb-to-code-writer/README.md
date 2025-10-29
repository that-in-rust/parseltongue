# parseltongue-05: LLM-cozoDB-to-code-writer

**Tool 5 in the Parseltongue 6-tool pipeline**

## Purpose

Ultra-minimalist file writer that reads Future_Code from CozoDB and writes it directly to files.

## Ultra-Minimalist Design Principles

- **NO BACKUPS**: No .bak, .backup, .old, or ~ files created
- **NO CONFIGURATION**: Single reliable write operation, hardcoded behavior
- **NO SAFETY LEVELS**: Direct writes only, no rollback mechanisms
- **NO COMPLEXITY**: One file = one operation = succeed or fail clearly

## Operations

1. **Create**: Write new file from Future_Code
2. **Edit**: Overwrite existing file with Future_Code (no backup)
3. **Delete**: Remove file permanently (no trash/recycle)

## Usage

```bash
# Write all pending changes from CozoDB
parseltongue-05 --database ./parseltongue.db --root ./my-project

# Dry-run mode (show what would be written)
parseltongue-05 --database ./parseltongue.db --root ./my-project --dry-run
```

## Integration in Pipeline

**Position**: After validation (Tool 4) → Before state reset (Tool 6)

**Input**: CozoDB entities with Future_Action (Create/Edit/Delete)
**Output**: Files written to disk
**Validation**: Assumes Tool 4 pre-validated all Future_Code

## Philosophy

> "Simplicity is the ultimate sophistication" - Leonardo da Vinci

Tool 5 embodies the ultra-minimalist philosophy: one purpose, no options, maximum reliability through simplicity. The entire CozoDB database serves as the backup - Tool 6 can always reset state by re-ingesting from source files.

## Error Handling

Fail-fast with clear error messages:
- File exists when creating → Error
- File not found when editing/deleting → Error
- Permission denied → Error
- Disk full → Error

No automatic retries, no fallback options. Fix the root cause and re-run.

## License

MIT OR Apache-2.0
