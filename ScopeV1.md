- **User Segment**: Apple Silicon developers on multi-language codebases with Rust-first support
- **Language Support**: Tree-sitter based parsing for all supported languages, with enhanced LSP integration for Rust
- **Reliability-First Principle**:
    - Prefer CPU-bound static analysis (tree-sitter parsing, ISG traversals) and small, local, free subagents
    - Keep the reasoning LLM as lean and late as possible; minimize context/tokens; use deterministic transforms whenever feasible
- **Shreyas Doshi (product framing)**: Prioritize first-apply correctness over speed. Design for clarity, safety, and explicit confidence gating. Time is a secondary outcome
- **Jeff Dean (systems framing)**: Make correctness the fast path. Push work to deterministic, cacheable computations (ISG, tree-sitter, HNSW). Parallelize retrieval/validation; minimize token movement; measure token-per-fix and cache hit rates
- **User Promise**: "When I encounter a code bug, the system produces a single-pass, safe, minimal diff that compiles and (when present) passes tests before applying. For Rust projects, this includes full LSP-enhanced validation; for other languages, core parsing and analysis is provided. Speed is a byproduct; correctness is the KPI"


## The 3 Trifecta of Parseltongue use-cases

When a So when weI'm basically trying to think of the things the user wants to do when they are right in front of the codebase. I come to a codebase and it's obviously overwhelming. I want to see a top-of-the-pyramid representation of the codebase which will help me get an idea of it, get comfortable with it, and that could be many things right. That I'll do if I get comfortable. I can figure out maybe I have come to solve a bug, maybe I've come to build a feature, or maybe I've come to think of whether a feature can be built or not. I don't even have to know the answer. Maybe I've just come to get an understanding of the codebase so that I can talk to my stakeholders about what it roughly has and what it doesn't have. Maybe I've come to search what exists and what doesn't exist? Maybe I have come to see what is the quality of the codebase? Does it adhere to some good HLD, HLD, LLD guidelines? All of that info is right here in the codebase but at an aggregate level is where I want to look at it. Which is where interface signature graphs will be useful I guess.


- Analyzing a codebase to understand its structure and relationships & creating a thorough documentation of the codebase
- Fixing a bug in the codebase that
    - Does not require any domain-research
    - Does require domain-research



