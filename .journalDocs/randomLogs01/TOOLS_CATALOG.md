# CPU-Based Code Analysis Tools Catalog

**Last Updated**: 2025-11-03
**Purpose**: Reference catalog of open-source, CPU-intensive code analysis tools for parseltongue integration

---

## Table of Contents

1. [Code Property Graph (CPG) Tools](#code-property-graph-cpg-tools)
2. [AST-Based Pattern Matching](#ast-based-pattern-matching)
3. [Static Analysis Tools](#static-analysis-tools)
4. [Code Search Tools](#code-search-tools)
5. [Dependency & Graph Analysis](#dependency--graph-analysis)
6. [Code Metrics & Statistics](#code-metrics--statistics)
7. [Language-Specific Tools](#language-specific-tools)
8. [CPU vs LLM Cost Comparison](#cpu-vs-llm-cost-comparison)
9. [Integration Recommendations](#integration-recommendations-for-parseltongue)

---

## Code Property Graph (CPG) Tools

### 1. Joern ⭐ CLONED
- **Repository**: https://github.com/joernio/joern
- **Local Path**: `.ref/tool-joern/`
- **License**: Apache-2.0
- **Language**: Scala
- **Description**: Open-source code analysis platform based on code property graphs
- **Supported Languages**: C/C++, Java, JavaScript, Python, Kotlin, x86/x64 binaries, JVM bytecode, PHP, Go, Ruby, Swift, C#
- **Key Features**:
  - Multi-layer semantic representation (AST + CFG + PDG + Call Graph)
  - Scala-based query DSL (CPGQL)
  - Interactive shell for exploration
  - Taint analysis and vulnerability detection
  - Export capabilities to various formats
- **CPU Cost**: High (graph construction is compute-intensive)
- **Integration Potential**: ⭐⭐⭐⭐⭐
  - Deep semantic analysis beyond AST pattern matching
  - Security-focused vulnerability detection
  - Cross-method data flow tracking
  - Complements parseltongue's AST analysis

### 2. Fraunhofer AISEC CPG ⭐ CLONED
- **Repository**: https://github.com/Fraunhofer-AISEC/cpg
- **Local Path**: `.ref/tool-fraunhofer-cpg/`
- **License**: Apache-2.0
- **Language**: Kotlin/Java
- **Description**: Language-independent code analysis platform using CPG
- **Supported Languages**: C/C++ (C17), Java (Java 13), Go (experimental), Python (experimental), TypeScript (experimental), LLVM IR
- **Key Features**:
  - Research-focused implementation
  - Strong type system modeling
  - LLVM IR support (theoretically all LLVM languages)
  - Academic backing from German research institute
- **CPU Cost**: High
- **Integration Potential**: ⭐⭐⭐⭐
  - Alternative CPG implementation
  - Research-oriented features
  - Potential for academic collaboration

### 3. MATE (Galois)
- **Repository**: https://github.com/GaloisInc/MATE
- **Website**: https://www.galois.com/articles/mate-interactive-program-analysis-with-code-property-graphs
- **License**: Apache-2.0
- **Description**: Interactive program analysis with code property graphs for C/C++
- **Key Features**:
  - Unifies high-level and low-level analysis
  - Application-specific vulnerability detection
  - Interactive exploration interface
- **Status**: Not cloned (C/C++ specific, less general-purpose)
- **Integration Potential**: ⭐⭐

---

## AST-Based Pattern Matching

### 4. ast-grep ⭐ ALREADY PRESENT
- **Repository**: https://github.com/ast-grep/ast-grep
- **Local Path**: `.ref/ast-grep/`
- **License**: MIT
- **Language**: Rust
- **Description**: Fast code searching and refactoring using AST patterns
- **Supported Languages**: 40+ via tree-sitter
- **Key Features**:
  - Lightning-fast pattern matching
  - Code-like pattern syntax
  - YAML rule engine
  - Parallel processing
- **CPU Cost**: Low-Medium
- **Integration Potential**: ⭐⭐⭐⭐⭐ (Already integrated!)

### 5. Semgrep ⭐ CLONED
- **Repository**: https://github.com/semgrep/semgrep
- **Local Path**: `.ref/tool-semgrep/`
- **License**: LGPL 2.1 (Community Edition)
- **Language**: OCaml/Python
- **Description**: Lightweight static analysis for bug detection and security scanning
- **Supported Languages**: 30+ languages
- **Key Features**:
  - Local analysis (no code upload)
  - Large rule library (1000+ security rules)
  - CI/CD integration
  - Pattern syntax resembles target code
- **CPU Cost**: Medium
- **Integration Potential**: ⭐⭐⭐⭐
  - Security rule library
  - Validate LLM suggestions against known patterns
  - Pre-filter code before expensive LLM analysis

### 6. Comby ⭐ CLONED
- **Repository**: https://github.com/comby-tools/comby
- **Local Path**: `.ref/tool-comby/`
- **License**: Apache-2.0
- **Language**: OCaml
- **Description**: Structural code search and replace for ~every language
- **Supported Languages**: Language-agnostic
- **Key Features**:
  - Lightweight template syntax
  - Language-aware parsing
  - Simpler than regex
  - Interactive review mode
- **CPU Cost**: Low-Medium
- **Integration Potential**: ⭐⭐⭐⭐
  - Apply refactorings suggested by LLM
  - Validate transformation patterns
  - Quick structural searches

### 7. Coccinelle
- **Repository**: https://github.com/coccinelle/coccinelle
- **Website**: https://coccinelle.gitlabpages.inria.fr/website/
- **License**: GPL-2.0
- **Language**: OCaml
- **Description**: Semantic patching for C programs
- **Supported Languages**: C
- **Key Features**:
  - SmPL (Semantic Patch Language)
  - Used extensively in Linux kernel development
  - Bug detection and code transformation
- **CPU Cost**: Medium
- **Status**: Not cloned (C-specific, niche use case)
- **Integration Potential**: ⭐⭐

### 8. tree-sitter ⭐ CLONED
- **Repository**: https://github.com/tree-sitter/tree-sitter
- **Local Path**: `.ref/tool-tree-sitter/`
- **License**: MIT
- **Language**: C/Rust
- **Description**: Incremental parsing library for building syntax trees
- **Supported Languages**: 40+ language grammars
- **Key Features**:
  - Incremental parsing (fast updates)
  - Error recovery
  - S-expression query language
  - Real-time performance
- **CPU Cost**: Low
- **Integration Potential**: ⭐⭐⭐⭐⭐
  - Foundation for AST parsing
  - Can replace/augment parseltongue's parsing layer
  - Query language for pattern matching

---

## Static Analysis Tools

### 9. Clippy (Rust)
- **Repository**: https://github.com/rust-lang/rust-clippy
- **License**: MIT/Apache-2.0
- **Description**: Rust linter with 550+ lints
- **Key Features**:
  - Built into cargo
  - Catches common mistakes
  - Enforces best practices
- **CPU Cost**: Low-Medium
- **Status**: Not cloned (Rust-specific, built into toolchain)
- **Integration Potential**: ⭐⭐⭐
  - Validate Rust code analysis
  - Learn from lint patterns

### 10. ESLint (JavaScript)
- **Repository**: https://github.com/eslint/eslint
- **License**: MIT
- **Description**: Pluggable JavaScript linter
- **Key Features**:
  - Highly configurable
  - Large plugin ecosystem
  - Auto-fixing capabilities
- **CPU Cost**: Low-Medium
- **Status**: Not cloned (widely available via npm)
- **Integration Potential**: ⭐⭐⭐
  - JavaScript code quality checks
  - Complement JS analysis

### 11. Pylint (Python)
- **Repository**: https://github.com/pylint-dev/pylint
- **License**: GPL-2.0
- **Description**: Python code static checker
- **Key Features**:
  - Comprehensive Python analysis
  - Code smell detection
  - Coding standard enforcement
- **CPU Cost**: Medium
- **Status**: Not cloned (widely available via pip)
- **Integration Potential**: ⭐⭐⭐

### 12. cargo-audit (Rust Security)
- **Repository**: https://github.com/rustsec/rustsec
- **License**: Apache-2.0/MIT
- **Description**: Audit Cargo.lock for vulnerabilities
- **Key Features**:
  - Security vulnerability detection
  - RustSec Advisory Database integration
  - CI integration
- **CPU Cost**: Low
- **Status**: Not cloned (Rust-specific, available via cargo)
- **Integration Potential**: ⭐⭐
  - Security auditing for Rust projects

### 13. SonarQube Community Edition
- **Repository**: https://github.com/SonarSource/sonarqube
- **License**: LGPL-3.0
- **Description**: Continuous code quality and security platform
- **Supported Languages**: 16 languages in community edition
- **Key Features**:
  - Comprehensive quality metrics
  - Security vulnerability detection
  - Technical debt tracking
  - 6000+ language-specific rules
- **CPU Cost**: High (requires server infrastructure)
- **Status**: Not cloned (heavyweight, server-based)
- **Integration Potential**: ⭐⭐
  - Too heavyweight for parseltongue
  - Could reference rule patterns

### 14. Cppcheck (C/C++)
- **Repository**: https://github.com/danmar/cppcheck
- **License**: GPL-3.0
- **Description**: Static analysis tool for C/C++
- **Key Features**:
  - Undefined behavior detection
  - Bi-directional analysis
  - Fast analysis
- **CPU Cost**: Medium
- **Status**: Not cloned (C/C++ specific)
- **Integration Potential**: ⭐⭐

### 15. rust-analyzer (Rust IDE)
- **Repository**: https://github.com/rust-lang/rust-analyzer
- **License**: MIT/Apache-2.0
- **Description**: Rust language server with rich IDE features
- **Key Features**:
  - Goto definition, type inference
  - Symbol search, refactoring
  - Code completion
- **CPU Cost**: Medium
- **Status**: Not cloned (IDE-focused)
- **Integration Potential**: ⭐⭐⭐
  - Learn from Rust analysis techniques
  - Reference for IDE-quality analysis

---

## Code Search Tools

### 16. ripgrep (rg)
- **Repository**: https://github.com/BurntSushi/ripgrep
- **License**: MIT/Unlicense
- **Language**: Rust
- **Description**: Lightning-fast regex search tool
- **Key Features**:
  - Extremely fast
  - Respects .gitignore
  - Parallel search
  - Unicode support
- **CPU Cost**: Low
- **Status**: Not cloned (commonly pre-installed)
- **Integration Potential**: ⭐⭐⭐⭐
  - Already used by parseltongue via Grep tool
  - Fast preliminary searches

### 17. The Silver Searcher (ag)
- **Repository**: https://github.com/ggreer/the_silver_searcher
- **License**: Apache-2.0
- **Description**: Code searching tool similar to ack
- **Key Features**:
  - Fast code search
  - .gitignore support
  - Simple syntax
- **CPU Cost**: Low
- **Status**: Not cloned (superseded by ripgrep)
- **Integration Potential**: ⭐⭐

### 18. ugrep
- **Repository**: https://github.com/Genivia/ugrep
- **License**: BSD-3-Clause
- **Description**: Ultra-fast grep with advanced features
- **Key Features**:
  - Interactive query UI
  - Fuzzy search
  - Archive search
- **CPU Cost**: Low
- **Status**: Not cloned (niche use case)
- **Integration Potential**: ⭐⭐

### 19. Sourcegraph SCIP
- **Repository**: https://github.com/sourcegraph/scip
- **License**: Apache-2.0
- **Description**: Code Intelligence Protocol for code navigation
- **Key Features**:
  - Language-agnostic indexing
  - Precise code navigation
  - Better than LSIF
- **CPU Cost**: Medium
- **Status**: Not cloned (indexing protocol, not analysis tool)
- **Integration Potential**: ⭐⭐⭐
  - Reference for code indexing
  - Potential future integration

### 20. universal-ctags
- **Repository**: https://github.com/universal-ctags/ctags
- **License**: GPL-2.0
- **Description**: Generate tag files for source code
- **Key Features**:
  - Symbol indexing
  - Multi-language support
  - Code navigation support
- **CPU Cost**: Low
- **Status**: Not cloned (basic indexing, limited analysis)
- **Integration Potential**: ⭐⭐

---

## Dependency & Graph Analysis

### 21. dependency-cruiser ⭐ CLONED
- **Repository**: https://github.com/sverweij/dependency-cruiser
- **Local Path**: `.ref/tool-dependency-cruiser/`
- **License**: MIT
- **Language**: JavaScript
- **Description**: Validate and visualize JavaScript/TypeScript dependencies
- **Key Features**:
  - Dependency graph visualization
  - Custom validation rules
  - Circular dependency detection
  - HTML reports
- **CPU Cost**: Medium
- **Integration Potential**: ⭐⭐⭐⭐
  - Understand JavaScript codebase structure
  - Dependency analysis for parseltongue itself
  - Graph-based insights

### 22. madge ⭐ CLONED
- **Repository**: https://github.com/pahen/madge
- **Local Path**: `.ref/tool-madge/`
- **License**: MIT
- **Language**: JavaScript
- **Description**: Visual module dependency graph for JS/TS
- **Key Features**:
  - Simple visualization
  - Circular dependency detection
  - CommonJS/AMD/ES6 support
  - Multiple output formats
- **CPU Cost**: Low-Medium
- **Integration Potential**: ⭐⭐⭐
  - Simpler alternative to dependency-cruiser
  - Quick dependency insights

### 23. cargo-tree (Rust)
- **Built into Cargo**: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
- **Description**: Display dependency tree for Rust projects
- **Key Features**:
  - Shows transitive dependencies
  - Invert tree to find why package is included
  - Filter by dependency type
- **CPU Cost**: Low
- **Status**: Not cloned (built into cargo)
- **Integration Potential**: ⭐⭐⭐
  - Analyze Rust project dependencies
  - Understand parseltongue's dependency tree

### 24. pipdeptree (Python)
- **Repository**: https://github.com/tox-dev/pipdeptree
- **License**: MIT
- **Description**: Display Python package dependency tree
- **Key Features**:
  - Visualize pip dependencies
  - Detect conflicts
  - JSON output
- **CPU Cost**: Low
- **Status**: Not cloned (simple Python tool)
- **Integration Potential**: ⭐⭐

### 25. arkit (JavaScript)
- **Repository**: https://github.com/dyatko/arkit
- **Website**: https://arkit.pro/
- **License**: MIT
- **Description**: JavaScript architecture diagrams and dependency graphs
- **Key Features**:
  - Automated architecture diagrams
  - Component relationships
  - Multiple diagram types
- **CPU Cost**: Low-Medium
- **Status**: Not cloned (similar to madge)
- **Integration Potential**: ⭐⭐⭐

---

## Code Metrics & Statistics

### 26. scc (Sloc Cloc and Code) ⭐ CLONED
- **Repository**: https://github.com/boyter/scc
- **Local Path**: `.ref/tool-scc/`
- **License**: MIT/Unlicense
- **Language**: Go
- **Description**: Fast code counter with complexity calculations
- **Key Features**:
  - Counts lines of code by language
  - Complexity estimation
  - COCOMO estimates
  - DRYness metrics
  - Fastest on Windows/macOS
- **CPU Cost**: Low
- **Integration Potential**: ⭐⭐⭐⭐
  - Quick codebase statistics
  - Complexity metrics
  - Pre-analysis filtering (skip large files)

### 27. tokei
- **Repository**: https://github.com/XAMPPRocky/tokei
- **License**: MIT/Apache-2.0
- **Language**: Rust
- **Description**: Display code statistics
- **Key Features**:
  - Very fast (fastest on Linux)
  - Multi-line comment handling
  - JSON output
  - Millions of lines in seconds
- **CPU Cost**: Low
- **Status**: Not cloned (similar to scc)
- **Integration Potential**: ⭐⭐⭐

### 28. cloc
- **Repository**: https://github.com/AlDanial/cloc
- **License**: GPL-2.0
- **Language**: Perl
- **Description**: Count lines of code
- **Key Features**:
  - Mature, well-tested
  - Extensive language support
  - Detailed reports
- **CPU Cost**: Medium (slower than tokei/scc)
- **Status**: Not cloned (superseded by tokei/scc)
- **Integration Potential**: ⭐⭐

---

## Language-Specific Tools

### 29. Rust Tools Summary
- **clippy**: Linting (built-in)
- **cargo-audit**: Security auditing
- **cargo-tree**: Dependency analysis (built-in)
- **rust-analyzer**: IDE features
- **cargo-geiger**: Unsafe code detection

**Integration Potential**: ⭐⭐⭐⭐ (for Rust codebases)

### 30. Python Tools Summary
- **pylint**: Static analysis
- **bandit**: Security linting
- **pipdeptree**: Dependency analysis
- **mypy**: Type checking
- **radon**: Code metrics

**Integration Potential**: ⭐⭐⭐ (for Python codebases)

### 31. JavaScript/TypeScript Tools Summary
- **ESLint**: Linting
- **dependency-cruiser**: Dependency analysis (cloned)
- **madge**: Dependency visualization (cloned)
- **TypeScript**: Type checking
- **npm-audit**: Security auditing

**Integration Potential**: ⭐⭐⭐⭐ (for JS/TS codebases)

### 32. C/C++ Tools Summary
- **Cppcheck**: Static analysis
- **Joern**: CPG analysis (cloned)
- **Coccinelle**: Semantic patching
- **Clang Static Analyzer**: LLVM-based analysis

**Integration Potential**: ⭐⭐⭐ (for C/C++ codebases)

---

## CPU vs LLM Cost Comparison

### CPU-Only Tool Costs

| Tool Category | CPU Cost | Memory | Time (Large Codebase) | Electricity Cost |
|---------------|----------|--------|----------------------|------------------|
| Code Search (ripgrep) | Very Low | ~100MB | Seconds | $0.00001 |
| AST Pattern (ast-grep) | Low | ~500MB | Minutes | $0.0001 |
| Metrics (scc/tokei) | Very Low | ~50MB | Seconds | $0.00001 |
| Dependencies (madge) | Low | ~200MB | Seconds | $0.00001 |
| Static Analysis (Semgrep) | Medium | ~1GB | Minutes | $0.001 |
| CPG Construction (Joern) | High | ~4GB | Hours | $0.01 |

### LLM API Costs (for comparison)

| Operation | Model | Input Tokens | Output Tokens | Cost |
|-----------|-------|--------------|---------------|------|
| Simple query | Claude Haiku | 10K | 1K | $0.00125 |
| Code analysis | Claude Sonnet | 50K | 5K | $0.165 |
| Deep analysis | Claude Opus | 100K | 10K | $1.80 |
| Full codebase | Claude Opus | 1M | 100K | $33.00 |

### Cost Analysis

1. **Pattern Matching**: CPU tools (ast-grep, Semgrep) are ~1000x cheaper than LLM
2. **Structural Search**: Comby costs ~$0.0001 vs $0.10+ for LLM equivalent
3. **Dependency Analysis**: CPU tools are essentially free vs $0.01+ per analysis
4. **CPG Construction**: One-time $0.01 cost vs $1+ for repeated LLM analysis

### Hybrid Approach Benefits

Using CPU tools to pre-filter and focus LLM attention:

- **Before**: Analyze 1M LOC with LLM = $33.00
- **After**:
  - CPU filter to 10K LOC = $0.001
  - LLM analyze filtered code = $0.165
  - **Total**: $0.166 (200x cheaper!)

---

## Integration Recommendations for Parseltongue

### Tier 1: Immediate Integration (High Value, Low Effort)

1. **tree-sitter** (Already have, deepen integration)
   - Use for faster AST parsing
   - Leverage query language for pattern matching
   - Replace/augment existing parsers

2. **scc** (Code metrics)
   - Pre-analysis filtering (skip large files)
   - Complexity metrics guide analysis depth
   - Quick codebase overview

3. **Semgrep** (Security patterns)
   - Validate LLM suggestions against security rules
   - Pre-filter for known vulnerability patterns
   - Learn from rule library

4. **dependency-cruiser** (Dependency analysis)
   - Understand codebase structure
   - Identify analysis boundaries
   - Detect architectural issues

### Tier 2: Strategic Integration (High Value, Medium Effort)

5. **Joern** (CPG analysis)
   - Deep semantic analysis
   - Security vulnerability detection
   - Control/data flow analysis
   - Complement AST-based analysis

6. **Comby** (Refactoring)
   - Apply LLM-suggested transformations
   - Validate transformation patterns
   - Interactive refactoring

7. **madge** (Quick dependency graphs)
   - Faster than dependency-cruiser
   - Simpler API for programmatic use

### Tier 3: Research & Exploration (Future Potential)

8. **Fraunhofer CPG** (Alternative CPG)
   - Research-oriented features
   - LLVM IR support
   - Academic collaboration potential

9. **SCIP** (Code indexing)
   - Better code navigation
   - Cross-reference analysis
   - Symbol resolution

### Integration Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Parseltongue Core                     │
└─────────────────────────────────────────────────────────┘
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
       ▼                   ▼                   ▼
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│  CPU Layer   │   │  LLM Layer   │   │ Graph Layer  │
│              │   │              │   │              │
│ • ast-grep   │   │ • Claude API │   │ • Joern CPG  │
│ • Semgrep    │   │ • OpenAI     │   │ • Fraunhofer │
│ • tree-sitter│   │ • Local LLM  │   │ • Call Graph │
│ • Comby      │   │              │   │              │
└──────────────┘   └──────────────┘   └──────────────┘
       │                   │                   │
       └───────────────────┼───────────────────┘
                           ▼
                  ┌──────────────┐
                  │   Metrics    │
                  │              │
                  │ • scc        │
                  │ • tokei      │
                  │ • madge      │
                  └──────────────┘
```

### Workflow Optimization

**Current**: LLM-heavy analysis
```
Code → AST → LLM Analysis → Results
Cost: High ($0.10 - $33 per analysis)
Time: Slow (API latency)
```

**Optimized**: CPU pre-filtering + focused LLM
```
Code → Metrics (scc) → Filter large files
     → AST (tree-sitter) → Pattern match (ast-grep/Semgrep)
     → Dependency (madge) → Identify hotspots
     → LLM (focused) → Deep analysis on 5-10% of code
     → CPG (Joern) → Validate with data flow
     → Comby → Apply transformations

Cost: Low ($0.01 - $0.50 per analysis)
Time: Fast (parallel CPU operations)
Quality: Higher (multi-tool validation)
```

### Recommended Next Steps

1. **Week 1-2**: Integrate scc for code metrics
   - Add codebase size/complexity reporting
   - Filter files by complexity before LLM analysis
   - Estimate analysis costs upfront

2. **Week 3-4**: Deepen tree-sitter integration
   - Add tree-sitter query support
   - Implement faster AST pattern matching
   - Support more languages via tree-sitter grammars

3. **Week 5-6**: Add Semgrep security layer
   - Pre-screen code for known vulnerabilities
   - Validate LLM suggestions against security rules
   - Build custom rule library

4. **Week 7-8**: Integrate dependency analysis
   - Add madge for JavaScript projects
   - Implement architecture visualization
   - Detect circular dependencies

5. **Month 3**: Explore Joern CPG integration
   - Evaluate CPG construction performance
   - Test data flow analysis capabilities
   - Compare Joern vs Fraunhofer CPG

6. **Month 4+**: Build hybrid analysis pipeline
   - Combine CPU pre-filtering with LLM analysis
   - Implement cost optimization strategies
   - Measure quality improvements

### Success Metrics

Track these to measure integration success:

1. **Cost Reduction**: Target 80-90% reduction in LLM API costs
2. **Speed Improvement**: 5-10x faster analysis with CPU pre-filtering
3. **Quality Increase**: 20-30% better detection via multi-tool validation
4. **Coverage Expansion**: Support 10+ more languages via tree-sitter
5. **Developer Satisfaction**: Faster feedback, better insights

---

## Repository Summary

### Cloned Repositories (8 total)

1. `.ref/tool-joern/` - CPG analysis platform
2. `.ref/tool-semgrep/` - Security-focused static analysis
3. `.ref/tool-comby/` - Structural code search/replace
4. `.ref/tool-tree-sitter/` - Incremental parsing library
5. `.ref/tool-fraunhofer-cpg/` - Research CPG implementation
6. `.ref/tool-dependency-cruiser/` - JS dependency validation
7. `.ref/tool-scc/` - Code metrics and complexity
8. `.ref/tool-madge/` - JS dependency visualization

### Research Documentation (2 files)

1. `.ref/research/code-property-graphs-overview.md`
2. `.ref/research/structural-search-tools.md`

### Not Cloned (Rationale)

- **Language-specific tools** (clippy, ESLint, pylint): Available via package managers
- **Basic utilities** (ripgrep, ag): Commonly pre-installed
- **Heavyweight tools** (SonarQube): Too complex for integration
- **Niche tools** (Coccinelle, Cppcheck): Limited scope
- **Superseded tools** (cloc, ag): Better alternatives cloned

---

## References

### Key Resources

- Code Property Graph Spec: https://cpg.joern.io/
- Tree-sitter Query Language: https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries
- Semgrep Rule Library: https://semgrep.dev/r
- Comby Documentation: https://comby.dev/docs
- Analysis Tools Directory: https://analysis-tools.dev/

### Academic Papers

- Code Property Graphs (2014): Original CPG paper by Yamaguchi et al.
- "A Language-Independent Analysis Platform for Source Code" (2022): Fraunhofer CPG paper
- Tree-sitter: An Incremental Parsing System (2018): Tree-sitter foundation

### Community Resources

- Joern Discord: https://discord.gg/vv4MH284Hc
- Semgrep Community: https://r2c.dev/slack
- Tree-sitter Discussions: https://github.com/tree-sitter/tree-sitter/discussions

---

**Total Tools Cataloged**: 32
**Repositories Cloned**: 8
**Research Documents**: 2
**Estimated Storage**: ~2.5 GB
**Estimated Time Saved**: 100+ hours of research for future developers
