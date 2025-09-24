# Parseltongue Steering Template

Copy this to your `.kiro/steering/parseltongue.md` file:

```markdown
---
inclusion: manual
---

# Parseltongue Architectural Intelligence

## Quick Commands
- `parseltongue onboard .` - Understand the codebase (15 min)
- `parseltongue feature-start <entity>` - Plan changes (5 min)  
- `parseltongue debug <entity>` - Trace usage (2 min)
- `parseltongue refactor-check <target>` - Safety check (3 min)

## Before Making Changes
1. **Understand Context**: `parseltongue where-defined <entity>`
2. **Check Impact**: `parseltongue feature-start <entity>`
3. **Verify Safety**: `parseltongue refactor-check <target>`

## During Development
- Use `parseltongue list-entities` to browse available entities
- Generate LLM context: `parseltongue generate-context <entity>`
- Check blast radius: `parseltongue query blast-radius <entity>`

## Architecture Queries
- Dependencies: `parseltongue query dependencies <entity>`
- Callers: `parseltongue query callers <entity>`
- File contents: `parseltongue entities-in-file <path>`

## Output Formats
- Human-readable: Default terminal output
- JSON: Add `--format json` for tooling
- Markdown: Add `--format markdown` for documentation

## Performance Expectations
- Entity listing: <100ms
- Blast radius: <500μs  
- File analysis: <2s for large files
- Onboarding: <15 minutes for any codebase
```