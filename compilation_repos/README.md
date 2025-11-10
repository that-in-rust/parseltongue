# Compilation Research Repository

## Research Question

**Can we compile code directly from CozoDB ingestion?**

This directory contains shallow clones of key Rust compiler-related repositories to explore the feasibility of moving parseltongue from a pure analysis tool to one capable of compilation.

## Current State → Potential Future

### Current: Analysis Only
- parseltongue ingests codebases into CozoDB
- We analyze structure, dependencies, patterns
- Output: context, insights, visualizations

### Research Target: Analysis + Compilation
- Same ingestion pipeline
- **NEW**: Attempt to compile code from CozoDB representation
- Questions to explore:
  - At what point does compilation trigger?
  - Can we reconstruct compilable code from CozoDB?
  - What's the minimum viable representation needed?
  - Could we enable "what-if" compilation scenarios?

## Cloned Repositories

### Core Compiler Infrastructure

1. **rust/** - The Rust compiler (rustc) and standard library
   - The full compilation pipeline
   - How AST → HIR → MIR → LLVM IR works
   - Entry point for understanding compilation stages

2. **cargo/** - Rust's package manager
   - Workspace resolution
   - Dependency graph compilation order
   - Build orchestration

3. **rust-analyzer/** - LSP server with incremental compilation
   - On-the-fly compilation without full builds
   - Salsa-based incremental computation
   - Query-based compilation model (similar to our CozoDB approach!)

### Parsing & Code Generation

4. **syn/** - Most popular Rust parsing library (by dtolnay)
   - How to parse Rust into AST
   - Span information preservation
   - Error recovery strategies

5. **quote/** - Rust code generation (by dtolnay)
   - AST → source code
   - Macro hygiene
   - Reverse of parsing: generation

### Specialized Analysis

6. **rustfmt/** - Code formatter
   - Parsing → reformatting → output
   - Preserving semantics while changing syntax
   - Round-trip parsing

7. **chalk/** - Trait solver
   - Type checking logic
   - Proof search
   - Could we do minimal type checking from CozoDB?

8. **miri/** - Rust interpreter
   - Executing MIR (mid-level IR) directly
   - No LLVM needed for interpretation
   - Could we interpret from CozoDB representation?

## Key Questions to Investigate

### 1. **Compilation Trigger Points**
- When does rustc decide to compile?
- What's the minimum information needed to start?
- How does rust-analyzer do incremental compilation?

### 2. **CozoDB → Compilable Code Path**
- Can we serialize CozoDB entities back to valid Rust?
- What metadata must we preserve during ingestion?
- Is round-tripping feasible?

### 3. **Incremental Compilation Model**
- rust-analyzer uses Salsa (query-based)
- We use CozoDB (graph-based)
- Can we map between these models?

### 4. **Minimal Viable Compilation**
- Could we compile single functions from CozoDB?
- What about type-checking without full compilation?
- Can we validate without full compilation?

## Research Methodology

### Phase 1: Study (Current)
- Clone all repos (✓ DONE - shallow clones only)
- Read entry points and main compilation flows
- Document trigger points

### Phase 2: Experimentation
- Try parsing simple Rust with syn
- Store parsed AST in CozoDB
- Attempt to regenerate source with quote
- Compare: original vs. regenerated

### Phase 3: Integration Design
- Design pt08-compiler-from-cozodb crate
- Define minimal compilation interface
- Identify CozoDB schema requirements

### Phase 4: Prototype
- Build proof-of-concept
- Compile simple function from CozoDB
- Measure feasibility and performance

## Ultrathink Integration

The user asked: "**at what point do they trigger ultrathink**"

This refers to:
- When does deep analysis/exploration begin?
- For rust-analyzer: on file change, query invalidation
- For rustc: on explicit build command
- For parseltongue: when we invoke pt01-folder-to-cozodb-streamer

**Key insight**: If we enable compilation from CozoDB, we could trigger "ultrathink compilation" at query time - compile code patterns that match specific queries!

Example workflow:
```
1. User queries: "Show me all error handling patterns"
2. CozoDB returns matching code entities
3. NEW: "Compile these patterns to verify they're valid"
4. Run miniature compilation on query results
5. Return: valid patterns vs. invalid patterns
```

## Next Steps

1. **Study rust-analyzer's salsa integration**
   - Most similar to our approach
   - Query-based incremental model
   - See: `rust-analyzer/crates/salsa/`

2. **Examine syn's parsing entry points**
   - See: `syn/src/lib.rs`
   - Understand: `syn::parse_file()`
   - Learn: span preservation

3. **Explore miri's interpretation model**
   - Could we interpret from CozoDB without LLVM?
   - See: `miri/src/`

4. **Read cargo's build graph construction**
   - How does it order compilation?
   - See: `cargo/src/cargo/core/compiler/`

## Directory Structure

```
compilation_repos/
├── README.md (this file)
├── rust/          # Main Rust compiler
├── cargo/         # Package manager & build system
├── rust-analyzer/ # Incremental LSP compiler
├── syn/           # Parsing library
├── quote/         # Code generation
├── rustfmt/       # Formatter (parse → regenerate)
├── chalk/         # Trait solver
└── miri/          # MIR interpreter
```

## Notes

- All clones are **shallow** (`--depth 1`) to save space
- Focus on understanding concepts, not full implementation
- This is exploratory research - may prove infeasible
- Document findings in this README as we learn

---

**Research started**: 2025-11-10
**Status**: Phase 1 - Cloning complete, study beginning
**Next action**: Investigate rust-analyzer's incremental compilation model
