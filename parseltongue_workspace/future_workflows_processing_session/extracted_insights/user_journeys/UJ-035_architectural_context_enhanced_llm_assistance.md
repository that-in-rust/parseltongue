# User Journey: Architectural Context-Enhanced LLM Assistance

**ID**: UJ-035
**Source**: DTNotes03.md - Hyper-Contextual Snippet Generator
**Persona**: Individual Developer
**Workflow Type**: LLM Integration

## Current Pain Points
- LLMs provide generic code suggestions without understanding architectural context
- Code assistance often introduces patterns that violate existing architectural decisions
- Developers spend time correcting LLM suggestions that don't fit the codebase structure
- Context provided to LLMs lacks actual usage patterns and surrounding logic

## Proposed Solution
Implement a Hyper-Contextual Snippet Generator that enriches Parseltongue's debug output with actual code snippets from usage sites, providing LLMs with comprehensive architectural context including:

- How entities are actually used (arguments, error handling, surrounding logic)
- Real usage patterns from multiple call sites
- Architectural relationships and dependencies
- Context lines showing the broader code environment

## Technical Implementation
```bash
# Core workflow combining Parseltongue debug with ripgrep context extraction
./pt debug EntityName
# Extract usage sites and enrich with surrounding code context
awk + ripgrep pipeline to generate enriched context files
# Provide comprehensive context to LLM for accurate assistance
```

## Success Metrics
- **Context Quality**: 90% reduction in architecturally inappropriate LLM suggestions
- **Developer Productivity**: 40% faster code completion with architectural alignment
- **Code Quality**: 60% reduction in architectural violations in LLM-assisted code
- **Context Completeness**: Usage patterns from all call sites included in LLM context

## Integration Requirements
- Parseltongue debug command with usage site extraction
- ripgrep for fast context extraction with configurable line counts
- awk for reliable file path and line number parsing
- Markdown generation for LLM-friendly context format

## Expected Outcomes
- LLMs provide architecturally consistent code suggestions
- Developers receive context-aware assistance that respects existing patterns
- Reduced time spent correcting inappropriate LLM suggestions
- Enhanced code quality through architectural grounding of AI assistance

## Dependencies
- Parseltongue debug functionality
- ripgrep installation for performance
- Bash scripting environment
- LLM integration pipeline

## Priority
**High** - Addresses fundamental problem of LLM context quality in architectural development

## Related Insights
- Links to TI-032: LLM Context Enrichment Pipeline
- Supports ST-026: Zero-Hallucination LLM Integration
- Connects to UJ-033: Zero Hallucination LLM Context Generation (DTNote01.md)