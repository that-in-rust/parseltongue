- **User Segment**: Apple Silicon developers on multi-language codebases with Rust-first support
- **Language Support**: Tree-sitter based parsing for all supported languages, with enhanced LSP integration for Rust
- **Reliability-First Principle**:
    - Optimize for accurate 1-go fixes that feel trustworthy and increase user efficacy
    - Prefer CPU-bound static analysis (tree-sitter parsing, ISG traversals) and small, local, free subagents
    - Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible
- **Shreyas Doshi (product framing)**: Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome
- **Jeff Dean (systems framing)**: Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, tree-sitter, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates
- **User Promise**: "When I encounter a code bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. For Rust projects, this includes full LSP-enhanced validation; for other languages, core parsing and analysis is provided. Speed is a byproduct; correctness is the KPI"


## The 3 Trifecta of Parseltongue use-cases

- Analyzing a codebase to understand its structure and relationships & creating a thorough documentation of the codebase
- Fixing a bug in the codebase that
    - Does not require any domain-research
    - Does require domain-research



