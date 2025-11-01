- **User Segment**: Apple Silicon developers on multi-language codebases with Rust-first support
- **Language Support**: Tree-sitter based parsing for all supported languages, with enhanced LSP integration for Rust
- **Reliability-First Principle**:
    - Prefer CPU-bound static analysis (tree-sitter parsing, ISG traversals) and small, local, free subagents
    - Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible
- **Shreyas Doshi (product framing)**: Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome
- **Jeff Dean (systems framing)**: Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, tree-sitter, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates
- **User Promise**: "When I encounter a code bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. For Rust projects, this includes full LSP-enhanced validation; for other languages, core parsing and analysis is provided. Speed is a byproduct; correctness is the KPI"


## Three Ways Developers Use Codebases

Developers approaching a codebase need different things at different times. The common thread: getting reliable answers without reading thousands of lines of code.

### Understanding a new codebase

You open a codebase and need to answer basic questions:
- What exists here? Authentication? Payment processing? Background jobs?
- How is it structured? What are the main modules and how do they connect?
- Is this code maintained? Does it follow consistent patterns?
- Can I explain this to stakeholders? What capabilities does it have?

The information is in the codebase, but you need it at an aggregate level. ISG provides this: a structured representation of interfaces, types, and relationships that fits in small context. An LLM can reason over the ISG to answer questions about architecture without seeing every implementation detail.

What this looks like:
- Ask "where does authentication happen" and get specific module references
- Query "what calls this function" and see the actual call graph
- Request "does this follow standard patterns" and get structural analysis

The result: You understand a 50K line codebase in the time it used to take to understand one module.

### Fixing bugs

Two types of bugs need different approaches:

**Syntax and logic bugs**: Type errors, off-by-one, null checks, incorrect conditionals. These don't need domain knowledge, just careful static analysis. Tree-sitter + ISG can narrow the problem space before the LLM sees any code. The LLM reasons over a small, relevant context.

**Domain bugs**: "Is this interest calculation correct for compound APR?" or "Does this retry logic match our SLA requirements?" These need domain understanding. ISG helps locate where the logic lives, then domain research informs the fix.

In both cases, ISG keeps context small and focused. You don't send 10K lines to the LLM. You send the ISG subgraph for the relevant functions and their dependencies.

### Modifying code safely

The goal: produce a diff that compiles, passes tests, and doesn't break hidden dependencies. Do this in one pass.

How ISG helps:
- Before changing a function, query ISG for all callers
- Validate that type changes are compatible with usage sites
- Check that interface modifications propagate correctly
- Confirm that the change is minimal (only necessary files touched)

Tree-sitter parsing ensures syntax correctness. ISG tracking ensures relationship correctness.

The LLM receives a small context: the target code + ISG-derived impact summary. It reasons about the change with full structural awareness but minimal token usage.

## Why Small Context Matters

ISG is a structured view of codebase relationships. When you query it, you get:
- Direct answers (function signatures, type definitions, call graphs)
- Relationship data (what imports what, what implements what)
- Scope boundaries (module hierarchies, dependency trees)

This means an LLM can reason about a large codebase using only the ISG subset relevant to the question. You get reliable understanding without large context windows.

Not about speed or token costs. About having the right structure so understanding is possible at small context.



