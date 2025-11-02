# Parseltongue v0.9.0 Scope: Meta-Level Code Understanding
## Puzzle 10 - Evolution Beyond Progressive Disclosure

**Document Version**: 1.0
**Date**: 2025-11-02
**Status**: Research Complete - Implementation Planning Phase
**Target Release**: Q1 2025

---

## Executive Summary

### Vision for v0.9.0

Parseltongue v0.9.0 represents the evolution from **progressive disclosure** (v0.8.6) to **intelligent aggregation** - the ability to understand codebases at meta-levels WITHOUT reading all the code. While v0.8.6 achieved 100x token savings through three disclosure levels (2-60K tokens), v0.9.0 will push further toward **4-10x additional compression** through semantic aggregation, graph-based analysis, and hierarchical context management.

**Core Insight**: Effective codebase understanding doesn't require reading all the code - it requires capturing the right relationships and abstractions that preserve semantic meaning while dramatically reducing information volume.

### What v0.9.0 Achieves

1. **Meta-Level Understanding**: Answer architectural questions without code examination
2. **Semantic Aggregation**: Compress related entities into meaningful clusters (modernity signatures, dependency islands, architectural layers)
3. **Graph-Native Analytics**: Leverage CozoDB's graph capabilities for reasoning (blast radius, transitive dependencies, impact analysis)
4. **Journey-Aware Orchestration**: Different compressions for different tasks (bug fixing vs pattern research vs refactoring)
5. **Provenance Tracking**: Automatic capture of design rationale and code evolution context
6. **Confidence-Gated Analysis**: Static analysis with reliability scoring

### Success Metrics

- Token efficiency: 500-1000 tokens for architectural overview (vs 30K in v0.8.6 Level 1)
- Comprehension accuracy: 95%+ on meta-questions without code reading
- Query performance: <100ms for graph analytics on 10K entity codebases
- Developer productivity: 80% reduction in "figuring out what to work on" time

---

## Current State Analysis: v0.8.6 Capabilities

### What We Have Working

**Architecture Foundation**:
- Single binary with 8 commands (pt01-pt06, plus pt02 variants)
- CozoDB with RocksDB backend (graph database with Datalog queries)
- ISGL1 key system for unique entity identification
- Temporal versioning (current_ind, future_ind, future_action)
- Tree-sitter parsing for Rust (multi-language ready)

**Progressive Disclosure System**:
- **Level 0**: Pure edge list (~2-5K tokens) - "What depends on what?"
- **Level 1**: Entities + ISG + Temporal (~30K tokens) - "How do I refactor?" [RECOMMENDED]
- **Level 2**: + Type system (~60K tokens) - "Is this type-safe?"
- **With code**: 500-700K tokens (100x more - use sparingly)

**Data Model (CodeGraph + DependencyEdges)**:
```
CodeGraph Table (14-22 fields per entity):
- ISGL1_key (unique identifier)
- entity_name, entity_type, file_path, line_number
- interface_signature (JSON: name, params, visibility)
- current_code, future_code
- current_ind, future_ind, future_action (temporal state)
- TDD_Classification (JSON: complexity, risk, coverage)
- doc_comment
- [Level 2 only] return_type, param_types, is_async, is_unsafe, generic_constraints, trait_impls

DependencyEdges Table (4 fields per edge):
- from_key, to_key (ISGL1 keys)
- edge_type (Calls, Uses, Implements)
- source_location
```

**Graph Operations (Implemented but Underexposed)**:
- Blast radius calculation (transitive closure up to 5 hops)
- Forward/reverse dependency queries
- Transitive dependency analysis
- Edge filtering by type

**Performance Baseline**:
- Indexing: 123ms for 765 entities (17,721 LOC)
- Exports: <1s per level
- Total pipeline: <2s for all 8 commands
- Database size: ~5KB compressed (RocksDB)

### What We Validated

**Production Readiness (v0.8.6)**:
- All 8 commands working with real CozoDB
- Self-analysis: Parseltongue indexing itself (765 entities)
- End-to-end test suite (ActuallyWorks)
- Token economics proven: 2-60K (signatures) vs 500-700K (with code)

---

## Gap Analysis: What's Missing for Meta-Level Understanding

### 1. Semantic Aggregation Layer

**Current State**: Export flat entity lists
**Missing**: Group entities into semantic clusters

**Examples of Missing Capabilities**:
- "Show me all authentication-related code" (pattern-based clustering)
- "What are the architectural layers?" (dependency-level detection)
- "Which modules are most coupled?" (modularity scoring)
- "What's the technical debt landscape?" (complexity + risk heatmap)

**Why This Matters**: A 1000-entity codebase at Level 1 = 30K tokens. With semantic aggregation, the same understanding could be achieved in 500-1K tokens through intelligent clustering.

### 2. Graph-Based Query Abstractions

**Current State**: Datalog queries embedded in CLI flags
**Missing**: Higher-level graph query patterns

**Examples**:
- Blast radius queries require understanding Datalog
- No reachability analysis ("Can function A reach function B?")
- No cycle detection ("Are there circular dependencies?")
- No centrality metrics ("What are the most critical functions?")

**Why This Matters**: Graph operations should be first-class citizens, not hidden behind low-level query strings.

### 3. Multi-Granularity Views

**Current State**: Entity-level only
**Missing**: Package-level, module-level, file-level aggregations

**Examples**:
- Package-level dependency networks (not just function-level)
- Module health scores (aggregate metrics per module)
- File-level change impact (temporal state rolled up)
- Class collaboration networks (OOP-specific)

**Why This Matters**: Different questions require different granularities. "Should I split this package?" is a package-level question, not entity-level.

### 4. Code Property Graph (CPG) Integration

**Current State**: AST-based extraction only
**Missing**: CFG (Control Flow), DFG (Data Flow), PDG (Program Dependencies)

**Examples**:
- Cannot answer "Where does this variable flow to?" (DFG)
- Cannot detect unreachable code (CFG)
- Cannot trace security vulnerabilities (CPG-based detection)
- Cannot compute live variable analysis

**Why This Matters**: 2024 research shows CPG (AST + CFG + DFG + PDG) achieves 10% higher F1 scores in vulnerability detection. ISG alone is not enough for security analysis.

### 5. Provenance and Design Rationale

**Current State**: Temporal state only (create/edit/delete)
**Missing**: WHY changes happened, context of decisions

**Examples**:
- "Why was this function marked unsafe?" (no capture)
- "What bug did this fix address?" (no linking)
- "What design patterns are used here?" (no inference)
- "When was this deprecated?" (no temporal metadata)

**Why This Matters**: Meta-understanding requires context, not just code. Software Heritage manages 4 billion files with provenance tracking - we need block-level equivalents.

### 6. Hierarchical Context Compression

**Current State**: Fixed disclosure levels (0, 1, 2)
**Missing**: Journey-aware, phase-based compression

**Examples**:
- Bug fixing journey: Show only affected entities + tests + dependencies
- Refactoring journey: Show module boundaries + coupling metrics + risk zones
- Pattern research: Show similar code clusters + architectural styles
- Security audit: Show unsafe code + data flow paths + blast radius

**Why This Matters**: 2024 research (HOMER, TokenSkip) shows hierarchical compression achieves 26-54% memory reduction while preserving 95%+ accuracy. Context should adapt to task.

### 7. Static Analysis with Confidence Gating

**Current State**: TDD_Classification with placeholder defaults
**Missing**: Real analysis with confidence scores

**Examples**:
- Complexity metrics: Only "Simple/Moderate/Complex" (no cyclomatic complexity)
- Coverage estimation: Placeholder (no actual coverage data)
- Risk scoring: Heuristic-based (no probabilistic model)
- Testability: Coarse-grained (no specific blocking factors)

**Why This Matters**: Clang Static Analyzer challenges show confidence scoring is hard but essential. Without it, developers don't trust the metrics.

### 8. Knowledge Graph Advantages (Underutilized)

**Current State**: CozoDB used as storage, not reasoning engine
**Missing**: Graph-native reasoning and inference

**Examples**:
- "What entities are related to authentication?" (semantic clustering - not implemented)
- "Find all error handling patterns" (subgraph matching - not implemented)
- "What would break if I change this?" (impact inference - basic implementation only)
- "Show architectural layers" (community detection - not implemented)

**Why This Matters**: FalkorDB research shows knowledge graphs enable reasoning and inference. Vector databases can't answer "what depends on what" efficiently. We have a graph database but use it like a relational store.

---

## Research Findings: State-of-the-Art Approaches

### 1. Code Property Graphs (CPG)

**Definition**: Unified graph combining AST, CFG, DFG, and PDG at statement and predicate nodes.

**Key Research (2024)**:

**Vul-LMGNN (April 2024)**:
- Merges AST (syntax), CFG (control flow), DFG (data dependencies), PDG (program dependencies)
- Uses GGNN (Gated Graph Neural Network) for node embeddings
- Achieves ~10% higher F1 score on vulnerability detection vs single-graph approaches
- Application: Security-critical code analysis

**VulMPFF (March 2024)**:
- Extracts four subgraphs by edge type: AST:0, CFG:1, CDG:2, DDG:3
- Heterogeneous graph with four semantic relations
- Application: Fine-grained vulnerability detection in C/C++

**GraphFVD (January 2025)**:
- Integrates AST (syntax), CFG (control flow), PDG (dependencies)
- Enables capture of diverse vulnerability characteristics
- Application: Rust safety analysis (async/unsafe patterns)

**Implications for Parseltongue**:
- ISG (current) = AST-focused with dependency edges
- CPG (v0.9.0) = ISG + CFG + DFG + PDG
- Use case: "Show me data flow from user input to database query" (SQL injection detection)
- Use case: "Find unreachable code after this early return" (dead code elimination)
- Implementation: Extend tree-sitter parsing to extract control flow and data flow edges

**Architecture Fit**:
- Store CFG/DFG/PDG edges in DependencyEdges table with new edge_types
- Query examples:
  - `edge_type = 'DataFlow'` (DFG)
  - `edge_type = 'ControlFlow'` (CFG)
  - `edge_type = 'ProgramDependency'` (PDG)
- Maintain backward compatibility: ISG queries unchanged

---

### 2. Semantic Code Graphs (SCG)

**Definition**: Block-level granularity graphs preserving location properties and semantic information (scope, modifiers, types).

**Key Research**:

**arxiv.org/abs/2310.02128 (October 2023)**:
- Detailed abstract representation of code dependencies
- Describes dependencies between entities at multiple levels:
  - Classes and methods (high-level)
  - Local value definitions (mid-level)
  - Type declarations (low-level)
- Preserves semantic context: visibility, mutability, lifetimes, generics

**Block-Level AST (2021)**:
- Each AST subtree corresponds to ECFG basic blocks
- Outer level: Inter-procedure ECFG (dependencies between blocks)
- Inner level: AST (structure of each block)
- Hierarchical representation aligns with mental models

**Class Collaboration Networks (CCN)**:
- Captures inheritance, aggregation, reference dependencies
- Node types: OBJECT, CLASS, TRAIT, INTERFACE
- Multi-level views: File, package, class, function collaboration

**Implications for Parseltongue**:
- Current entity_type: Function, Struct, Trait, Enum, Module, ImplBlock
- Add block-level entities: BasicBlock, IfBranch, LoopBody, MatchArm
- Preserve control flow context: "This block is inside async function X"
- Use case: "Show me all early returns in authentication flow" (block-level filtering)
- Use case: "What error handling patterns exist?" (block signature matching)

**Architecture Fit**:
- Extend ISGL1 keys: `rust:block:if_branch:auth_rs:check_token:45-47`
- Store block metadata in interface_signature:
  ```json
  {
    "block_type": "IfBranch",
    "parent_entity": "rust:fn:check_token:auth_rs:42-60",
    "control_flow_type": "EarlyReturn",
    "error_handling": true
  }
  ```
- Query: "Find all error handling blocks" → Filter by `block_type + error_handling = true`

---

### 3. Hierarchical Context Compression

**Definition**: Phase-based, journey-aware compression achieving 4-10x token reduction while preserving task-relevant semantics.

**Key Research (2024)**:

**HOMER - Hierarchical Context Merging (ICLR 2024)**:
- Divide-and-conquer: Split long inputs into chunks
- Hierarchical merging: Merge adjacent chunks at progressive transformer layers
- Token reduction: Preceding each merge reduces tokens
- Result: Extends context limits without retraining LLMs
- Training-free, computationally efficient

**TokenSkip (2025)**:
- Selective token skipping for less important content
- Controllable Chain-of-Thought (CoT) compression
- Reduces inference latency and KV cache memory
- Autoregressive approach: Skip during generation

**Acon - Agent Context Optimization (2024)**:
- Reduces memory usage by 26-54% (peak tokens)
- Preserves 95%+ accuracy when compressed
- Long-horizon agent tasks: 46% performance improvement
- Context-aware: Keeps task-relevant information

**Implications for Parseltongue**:
- Current: Fixed levels (Level 0/1/2) regardless of task
- v0.9.0: Journey-specific compression profiles

**Journey Examples**:

**Bug Fixing Journey** (Ultra-focused):
```
Phase 1: Identify bug location (100 tokens)
  - Entity signature + error message + stack trace

Phase 2: Show blast radius (500 tokens)
  - Affected entities (forward/reverse deps, 1 hop)
  - Related tests
  - Recent changes (temporal state)

Phase 3: Show implementation context (2K tokens)
  - Full code of affected entities
  - Relevant type signatures
  - Doc comments
```
Total: ~2.6K tokens (vs 30K in current Level 1)

**Refactoring Journey** (Module-focused):
```
Phase 1: Module health (200 tokens)
  - Complexity metrics per module
  - Coupling scores (fan-in/fan-out)
  - Test coverage gaps

Phase 2: Blast radius of changes (1K tokens)
  - Cross-module dependencies
  - Public API surface affected
  - Deprecation impact

Phase 3: Similar patterns (1K tokens)
  - Code clusters using same patterns
  - Refactoring candidates with similar structure
```
Total: ~2.2K tokens

**Pattern Research Journey** (Exploratory):
```
Phase 1: Architectural overview (500 tokens)
  - Layer detection (UI, business logic, data)
  - Module boundaries
  - Cross-cutting concerns

Phase 2: Pattern catalog (2K tokens)
  - Design patterns detected (Factory, Builder, Strategy)
  - Architectural styles (MVC, CQRS, Event-Driven)
  - Code smells (God class, Feature envy)

Phase 3: Exemplar code (5K tokens)
  - Representative implementations of each pattern
  - Best practices vs anti-patterns
```
Total: ~7.5K tokens

**Architecture Fit**:
- New command: `pt07-journey-context-builder`
- Flags: `--journey <bug-fix|refactor|pattern-research|security-audit>`
- Output: Hierarchical JSON with phases, each with token budget
- Implementation: Query CozoDB with journey-specific filters, aggregate intelligently

---

### 4. Meta-Information Tracking

**Definition**: Automatic capture of code provenance, design rationale, and block-level versioning.

**Key Research**:

**Software Heritage Archive (2020)**:
- Tracks 4 billion source code files
- Inspects origin and evolution throughout lifecycle
- Provenance: Development history + distribution paths
- Scale: 1 billion commits across 50 million projects

**PAV Ontology (Provenance, Authoring, Versioning)**:
- Lightweight approach for web resources
- Captures: Who, what, when, why, how
- Applicable to code blocks (not just files)

**SLSA Provenance (Supply Chain Security)**:
- Builder metadata (what built this)
- Source location (where it came from)
- Materials (inputs used)
- Build parameters (how it was built)

**Implications for Parseltongue**:
- Current: Temporal state (create/edit/delete) without context
- v0.9.0: Rich provenance metadata

**Provenance Model**:
```json
{
  "ISGL1_key": "rust:fn:authenticate:auth_rs:42-68",
  "provenance": {
    "created_at": "2024-10-15T14:32:00Z",
    "created_by": "alice@example.com",
    "created_because": "CVE-2024-12345: Fix authentication bypass",
    "related_issue": "https://github.com/project/issues/123",
    "design_rationale": "Changed from password-only to MFA to meet security requirements",
    "deprecated_at": null,
    "replaced_by": null
  },
  "evolution": [
    {
      "version": 1,
      "timestamp": "2024-10-15T14:32:00Z",
      "action": "create",
      "reason": "Initial implementation"
    },
    {
      "version": 2,
      "timestamp": "2024-10-20T09:15:00Z",
      "action": "edit",
      "reason": "Added rate limiting per security review"
    }
  ]
}
```

**Capture Methods**:
1. **Automatic**: Parse commit messages, PR descriptions, issue links
2. **Semi-automatic**: Prompt LLM to extract rationale from code reviews
3. **Manual**: CLI prompts during pt03 writes: `--reason "Why this change?"`

**Architecture Fit**:
- Extend CodeGraph table: Add `provenance` (JSON) column
- Store evolution history: Append-only log
- Query: "Show all security-related changes in last 30 days"
- Implementation: Git integration (read commits), optional LLM extraction

---

### 5. Graph-Based Aggregation Techniques

**Definition**: Intelligent clustering and summarization of graph structures for compact representation.

**Techniques**:

#### A. Modernity Signatures
**Concept**: Summarize code "style" as a compact signature (async usage, error handling patterns, dependency versions).

**Example**:
```json
{
  "module": "src/auth",
  "modernity_score": 0.85,
  "signatures": {
    "async_usage": 0.92,      // 92% of functions are async
    "error_handling": "Result", // Uses Result<T,E> consistently
    "dependencies": [           // External crates used
      {"name": "tokio", "version": "1.35", "modernity": 0.95},
      {"name": "serde", "version": "1.0", "modernity": 0.90}
    ],
    "patterns": ["Builder", "Strategy"],
    "anti_patterns": []
  }
}
```

**Use Case**: "Which modules follow modern Rust practices?" (single query, compact answer)

#### B. Dependency Clustering
**Concept**: Group entities by community detection (Louvain algorithm) to identify natural module boundaries.

**Example**:
```json
{
  "clusters": [
    {
      "id": "cluster_0",
      "label": "Authentication",
      "entities": 23,
      "internal_edges": 67,
      "external_edges": 12,
      "modularity_score": 0.82,  // High modularity = well-separated
      "representative_entities": [
        "rust:fn:login:auth_rs:10-25",
        "rust:fn:verify_token:auth_rs:30-45"
      ]
    },
    {
      "id": "cluster_1",
      "label": "Database",
      "entities": 18,
      "internal_edges": 45,
      "external_edges": 8,
      "modularity_score": 0.76
    }
  ],
  "cross_cluster_dependencies": [
    {"from": "cluster_0", "to": "cluster_1", "edge_count": 12}
  ]
}
```

**Use Case**: "Should I split this module?" (analyze modularity scores)

#### C. Stream Visualizations
**Concept**: Time-series view of code evolution (complexity over time, churn hotspots).

**Example**:
```json
{
  "file": "src/auth.rs",
  "timeline": [
    {"date": "2024-10-01", "complexity": 120, "entities": 15, "churn": 0},
    {"date": "2024-10-15", "complexity": 150, "entities": 18, "churn": 45},
    {"date": "2024-11-01", "complexity": 135, "entities": 17, "churn": 12}
  ],
  "trends": {
    "complexity": "decreasing",  // Recent refactoring reduced complexity
    "churn": "low",              // Stable code
    "risk": "low"
  }
}
```

**Use Case**: "Which files are becoming more complex?" (trend analysis)

#### D. Minimal DFS Encoding
**Concept**: Compress dependency trees using depth-first traversal with shared subtrees.

**Example**:
```
Traditional representation (verbose):
rust:fn:A → [rust:fn:B, rust:fn:C]
rust:fn:B → [rust:fn:D, rust:fn:E]
rust:fn:C → [rust:fn:D, rust:fn:F]
rust:fn:D → [rust:fn:G]

DFS encoding (compact):
A(B(D(G),E),C(D*,F))
* = reference to earlier node (shared subtree)

Token savings: 60% reduction for typical trees
```

**Use Case**: Export dependency trees in minimal format for LLM consumption

**Architecture Fit**:
- New command: `pt08-graph-analytics`
- Subcommands:
  - `--modernity-signatures` (module-level analysis)
  - `--dependency-clusters` (community detection)
  - `--stream-timeline` (temporal evolution)
  - `--dfs-encode` (minimal tree encoding)
- Output: Aggregated JSON (500-2K tokens vs 30K for raw entities)

---

### 6. Knowledge Graph Integration Advantages

**Definition**: Leverage graph database capabilities for reasoning, inference, and pattern discovery.

**Key Advantages Over Vector Databases**:

#### A. Structured Relationships
- **Vector DB**: Similarity search only ("What's similar to X?")
- **Knowledge Graph**: Relationship traversal ("What calls X?" "What does X depend on?")
- **Example**: "Find circular dependencies" requires graph cycles, impossible in vector space

#### B. Graph Query Languages (Cypher/Datalog)
- **Cypher (Neo4j standard, ISO/IEC 39075 GQL)**:
  ```cypher
  MATCH (f:Function)-[:CALLS]->(g:Function)
  WHERE f.complexity = 'High' AND g.risk = 'High'
  RETURN f.name, g.name, count(*) as call_count
  ORDER BY call_count DESC
  ```
- **Datalog (CozoDB current)**:
  ```datalog
  ?[caller, callee, count] :=
      *CodeGraph{ISGL1_key: caller, TDD_Classification},
      *DependencyEdges{from_key: caller, to_key: callee, edge_type},
      edge_type = 'Calls',
      complexity = json_extract(TDD_Classification, '$.complexity'),
      complexity = 'High',
      count = count(callee)
  :order count desc
  ```

**Parseltongue Status**: We use Datalog but only for basic filtering. Graph traversal patterns underutilized.

#### C. Reasoning and Inference
**FalkorDB Research (2024)**:
- **Link prediction**: "If A calls B, and B calls C, likely A depends on C transitively"
- **Entity classification**: "This function is authentication-related based on call graph neighbors"
- **Pattern discovery**: "These 5 functions follow the Builder pattern"
- **Impact analysis**: "Changing X affects Y, Z, W based on data flow"

**Current Gap**: CozoDB supports transitive closure, but we don't expose higher-level reasoning.

#### D. Hybrid Graph + Vector Approach
- Store code embeddings in vector DB (semantic similarity)
- Store dependencies in graph DB (structural relationships)
- Query: "Find similar functions (vector) that are also coupled (graph)"

**Use Case**: Refactoring - Find duplicate logic that's also architecturally related.

**Architecture Fit**:
- Extend DependencyEdges with inferred edges:
  - `edge_type = 'InferredDependency'` (transitive)
  - `edge_type = 'SimilarPattern'` (semantic clustering)
  - `edge_type = 'ArchitecturalLayer'` (layer detection)
- New command: `pt09-graph-reasoning`
- Subcommands:
  - `--infer-transitive-deps` (compute closure, store as edges)
  - `--detect-patterns` (Builder, Factory, Singleton)
  - `--cluster-by-layer` (UI, Business, Data layers)
  - `--find-similar` (semantic + structural similarity)

---

### 7. Static Analysis Meta-Patterns

**Definition**: Practical approaches to static analysis with confidence scoring and parallelization.

**Key Patterns**:

#### A. Confidence Gating
**Clang Static Analyzer Challenge (2024)**:
- Computing confidence values for bug reports is hard
- Easy: Decrease confidence when uncertain (multiple branch paths)
- Hard: Increase confidence when same bug found on multiple paths (paths not independent)

**Solution: Probabilistic Confidence Model**
```python
confidence = base_confidence * path_coverage * independent_confirmations

Where:
  base_confidence = analysis_type_confidence (0.8 for data flow, 0.6 for heuristics)
  path_coverage = paths_analyzed / total_possible_paths
  independent_confirmations = sqrt(confirming_analyses) / total_analyses
```

**Example**:
```json
{
  "finding": "Potential null pointer dereference in authenticate()",
  "confidence": 0.72,
  "breakdown": {
    "base_confidence": 0.80,  // Data flow analysis (reliable)
    "path_coverage": 0.90,    // 9 of 10 paths analyzed
    "confirmations": 1.0      // No conflicting evidence
  },
  "recommendation": "High confidence - investigate immediately"
}
```

**Parseltongue Application**:
- TDD_Classification confidence scores (replace placeholder defaults)
- Complexity analysis confidence: "Simple" with 0.95 confidence vs "Complex" with 0.60 confidence
- Test coverage estimates with confidence intervals: "Estimated 70% ± 15% coverage (confidence: 0.75)"

#### B. Embarrassingly Parallel Analysis
**Concept**: Static analysis tasks are independent - analyze each entity in parallel.

**Implementation**:
- Use Rayon for parallel tree-sitter parsing (v0.8.6 does this in pt01)
- Extend to analysis: Parallel complexity calculation, parallel CFG construction
- CozoDB writes: Batch inserts (current implementation already does this)

**Performance Target**: 10x speedup on multi-core systems for large codebases

#### C. Symbolic Execution Paths
**Research: AISE (Abstract Interpretation + Symbolic Execution, 2024)**:
- Combines precision of symbolic execution with soundness of abstract interpretation
- Achieves "best of both worlds"
- Application: Detect security vulnerabilities (integer overflow, buffer overrun)

**Parseltongue Application**:
- Integrate with Rust's MIR (Mid-level IR) for symbolic execution
- Detect: Panic paths, integer overflow, unsafe usage
- Store results in CodeGraph: `security_findings` (JSON array)

#### D. Abstract Interpretation
**Concept**: Approximate program semantics to reason about all possible executions.

**Use Cases**:
- Value range analysis: "Variable x is always 0-100"
- Null pointer analysis: "Pointer p is never null at line 45"
- Type state analysis: "Resource is acquired before use, released after"

**Parseltongue Application**:
- Store abstract states in CodeGraph: `abstract_state` (JSON)
- Example:
  ```json
  {
    "ISGL1_key": "rust:fn:divide:math_rs:10-15",
    "abstract_state": {
      "preconditions": ["denominator != 0"],
      "postconditions": ["result is finite"],
      "invariants": ["no panic"]
    },
    "confidence": 0.85
  }
  ```

**Architecture Fit**:
- Extend pt01 (indexing) with analysis passes:
  - Pass 1: Parse syntax (current)
  - Pass 2: Build CFG/DFG (new)
  - Pass 3: Run abstract interpretation (new)
  - Pass 4: Compute confidence scores (new)
- Store results in CodeGraph (extend schema)
- New flag: `pt01 --analyze-level <basic|standard|deep>`

---

### 8. Multi-Level Abstraction

**Definition**: View codebase at different granularities (package, module, class, function, block).

**Key Research**:

**CodEx - Multi-Level Call Graphs (2021)**:
- Coarsening technique for hierarchical call graph views
- Clustering of execution paths
- Granularity levels: Package → Class → Function → Block
- Goal: High-level overview → low-level implementation with drill-down

**CoLadder - Hierarchical Code Generation (2023)**:
- Decompose goals into subtasks at multiple levels
- Hierarchical prompt structures
- Application: LLM code generation with planning

**Implications for Parseltongue**:

#### Level 1: Package-Level Networks
```json
{
  "packages": [
    {
      "name": "parseltongue-core",
      "entities": 245,
      "complexity": "Moderate",
      "test_coverage": 0.78,
      "dependencies": ["cozo", "tree-sitter"],
      "dependents": ["pt01", "pt02", "pt03"]
    }
  ],
  "package_dependencies": [
    {"from": "pt02", "to": "parseltongue-core", "edge_count": 67}
  ]
}
```

**Query**: "What's the highest-level architecture?" (10 packages, not 1000 functions)

#### Level 2: Module-Level Views
```json
{
  "module": "src/storage/cozo_client.rs",
  "entities": 23,
  "public_api": [
    "rust:fn:run_query:src_storage_cozo_client_rs:45-78",
    "rust:fn:blast_radius:src_storage_cozo_client_rs:305-372"
  ],
  "private_impl": 18,
  "complexity_score": 0.65,
  "coupling": {
    "fan_in": 12,   // 12 modules depend on this
    "fan_out": 5    // This depends on 5 modules
  }
}
```

**Query**: "Which modules have high coupling?" (refactoring candidates)

#### Level 3: Class Collaboration Networks (OOP)
```json
{
  "class": "CozoClient",
  "type": "struct",
  "collaborates_with": [
    {"class": "CozoDb", "relationship": "Aggregation"},
    {"class": "QueryBuilder", "relationship": "Uses"},
    {"class": "EntityStore", "relationship": "Implements"}
  ],
  "methods": 15,
  "responsibilities": ["Database connection", "Query execution", "Result parsing"]
}
```

**Query**: "Show class relationships in storage layer" (OOP design view)

#### Level 4: Call Hierarchies
```json
{
  "entry_point": "rust:fn:main:src_main_rs:1-50",
  "call_tree": [
    {
      "depth": 0,
      "function": "main",
      "calls": [
        {
          "depth": 1,
          "function": "run_pt02_export",
          "calls": [
            {"depth": 2, "function": "build_query"},
            {"depth": 2, "function": "execute_query"},
            {"depth": 2, "function": "format_results"}
          ]
        }
      ]
    }
  ],
  "max_depth": 5,
  "total_functions": 34
}
```

**Query**: "What's the call path from main to database?" (execution flow)

**Architecture Fit**:
- New command: `pt10-multi-level-view`
- Flags:
  - `--level <package|module|class|function|block>`
  - `--granularity <coarse|medium|fine>`
  - `--focus <entity_key>` (drill down from high-level to specific entity)
- Output: Hierarchical JSON with collapsible sections
- Implementation:
  - Aggregate entities by file_path (module-level)
  - Parse Cargo.toml for package structure
  - Use DependencyEdges to build collaboration networks

---

## Recommended Evolution Path for v0.9.0

### Design Principles

1. **Backward Compatibility**: v0.8.6 commands unchanged, new commands additive
2. **Progressive Enhancement**: New features optional, degrade gracefully
3. **Graph-First Thinking**: Leverage CozoDB's graph capabilities, not just storage
4. **Ultra-Minimalist**: No backups, no complexity, single reliable operations (S01 principle)
5. **TDD-First**: Red → Green → Refactor for every new feature

### Three-Pillar Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  PILLAR 1: Enhanced Indexing                 │
│  Goal: Extract CFG, DFG, PDG + Block-level entities          │
│  Tools: pt01 (extended)                                      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│               PILLAR 2: Graph-Native Analytics               │
│  Goal: Reasoning, inference, pattern detection              │
│  Tools: pt07 (analytics), pt08 (aggregation), pt09 (reasoning)│
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│             PILLAR 3: Journey-Aware Compression              │
│  Goal: Task-specific context at 500-1K tokens               │
│  Tools: pt11 (journey builder)                              │
└─────────────────────────────────────────────────────────────┘
```

### Feature Priority Matrix

| Feature | Impact | Complexity | Priority | Target Tool |
|---------|--------|------------|----------|-------------|
| Graph analytics (blast radius, centrality) | HIGH | LOW | P0 | pt07 |
| Semantic aggregation (clusters, signatures) | HIGH | MEDIUM | P0 | pt08 |
| Multi-level views (package/module/class) | HIGH | LOW | P0 | pt08 |
| Journey-aware compression | VERY HIGH | MEDIUM | P0 | pt11 |
| Block-level entities (if/loop/match blocks) | MEDIUM | HIGH | P1 | pt01 extension |
| CFG extraction | MEDIUM | HIGH | P1 | pt01 extension |
| DFG extraction | MEDIUM | HIGH | P2 | pt01 extension |
| Provenance tracking (Git integration) | MEDIUM | MEDIUM | P1 | pt03 extension |
| Confidence-gated analysis | MEDIUM | HIGH | P2 | pt01 extension |
| Graph reasoning (pattern detection) | HIGH | VERY HIGH | P2 | pt09 |
| Hybrid vector + graph | LOW | VERY HIGH | P3 | Future |

**P0 = Must-have for v0.9.0**
**P1 = Nice-to-have for v0.9.0, can defer to v0.10.0**
**P2 = Research spike, likely v0.10.0 or later**
**P3 = Future exploration**

---

## Implementation Priorities: What to Build First

### Phase 1: Foundation (Weeks 1-2) - "Make the invisible visible"

**Goal**: Expose graph capabilities that already exist in CozoDB but are hidden.

#### PT07: cozodb-code-as-visuals
**Status**: Research complete (ISG_ANALYTICS_RESEARCH.md, 156KB)
**Scope**: Terminal-based analytics dashboard

**Features**:
- Dashboard report (codebase health snapshot)
- Complexity report (top refactoring candidates)
- Coverage report (testing gaps)
- Blast radius report (impact analysis for entity)
- Dependencies report (coupling metrics)
- Changes report (temporal state summary)
- Entities report (filterable listing)
- Modules report (file organization)

**Output Formats**:
- Terminal tables (comfy-table) [default]
- JSON (CI integration)
- CSV (spreadsheet export)

**Success Metrics**:
- <100ms for all reports on 1000-entity codebase
- Clear actionable recommendations (not generic advice)
- Developers use blast-radius before refactoring

**Example Usage**:
```bash
# Morning standup check
parseltongue pt07-cozodb-code-as-visuals --db rocksdb:project.db
# Output: Health score 78/100, 3 high-priority items, 12 entities pending changes

# Pre-refactor risk assessment
parseltongue pt07-cozodb-code-as-visuals \
  --report blast-radius \
  --entity "rust:fn:authenticate:auth_rs:42-68" \
  --db rocksdb:project.db
# Output: 23 affected entities, 8 files, HIGH risk

# CI integration
parseltongue pt07-cozodb-code-as-visuals \
  --report health \
  --format json \
  --db rocksdb:project.db | jq '.health_score'
# Output: 78
```

**Implementation Plan**:
- Week 1: CLI framework, query builders, table rendering
- Week 2: All 8 report types, JSON/CSV output, integration tests

**Existing Research**: PT07_IMPLEMENTATION_GUIDE.md (32KB with code examples)

---

### Phase 2: Semantic Aggregation (Weeks 3-4) - "Compress intelligently"

**Goal**: Group related entities into semantic clusters for compact representation.

#### PT08: graph-based-aggregation
**Scope**: Modernity signatures, dependency clustering, DFS encoding

**Features**:

**A. Modernity Signatures** (Module-level "style" summary)
```bash
parseltongue pt08-graph-based-aggregation \
  --modernity-signatures \
  --output modernity.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "modules": [
    {
      "path": "src/auth",
      "modernity_score": 0.85,
      "async_usage": 0.92,
      "error_handling": "Result",
      "patterns": ["Builder", "Strategy"],
      "dependencies": [
        {"name": "tokio", "version": "1.35", "modernity": 0.95}
      ]
    }
  ]
}
```
Tokens: ~50-100 per module (vs 1000+ for all entities in module)

**B. Dependency Clustering** (Community detection)
```bash
parseltongue pt08-graph-based-aggregation \
  --dependency-clusters \
  --algorithm louvain \
  --output clusters.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "clusters": [
    {
      "id": "cluster_0",
      "label": "Authentication",
      "entities": 23,
      "modularity_score": 0.82,
      "representative_entities": [
        "rust:fn:login:auth_rs:10-25"
      ]
    }
  ]
}
```
Tokens: ~100-200 for cluster summary (vs 5000+ for all entities)

**C. Multi-Level Views** (Package/Module/Class/Function)
```bash
parseltongue pt08-graph-based-aggregation \
  --multi-level-view \
  --granularity package \
  --output packages.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "packages": [
    {
      "name": "parseltongue-core",
      "entities": 245,
      "complexity": "Moderate",
      "test_coverage": 0.78,
      "dependencies": ["cozo", "tree-sitter"]
    }
  ]
}
```
Tokens: ~50-100 per package (vs 30K for all entities)

**D. Minimal DFS Encoding** (Compact tree representation)
```bash
parseltongue pt08-graph-based-aggregation \
  --dfs-encode \
  --root "rust:fn:main:src_main_rs:1-50" \
  --max-depth 5 \
  --output tree.txt \
  --db rocksdb:project.db
```
Output:
```
main(run_pt02(build_query,execute_query*,format_results),handle_error(log_error*))
* = shared subtree (referenced earlier)
```
Tokens: ~60% reduction vs flat list

**Success Metrics**:
- Modernity signatures: 10-20x token reduction vs full entity list
- Clustering: 95%+ accuracy in identifying module boundaries (validate with human judgment)
- DFS encoding: 50-60% token reduction vs traditional tree format

**Implementation Plan**:
- Week 3: Modernity signatures + multi-level views
- Week 4: Dependency clustering + DFS encoding

**Dependencies**: Rust crates: petgraph (graph algorithms), community-detection (Louvain)

---

### Phase 3: Journey-Aware Compression (Weeks 5-6) - "Context that adapts"

**Goal**: Task-specific exports that deliver exactly what's needed for the journey.

#### PT11: journey-context-builder
**Scope**: Bug-fix, refactor, pattern-research, security-audit journeys

**Features**:

**Journey Profiles** (Predefined compression strategies)

**A. Bug-Fix Journey**
```bash
parseltongue pt11-journey-context-builder \
  --journey bug-fix \
  --focus "rust:fn:authenticate:auth_rs:42-68" \
  --error-message "null pointer dereference at line 45" \
  --output context.json \
  --db rocksdb:project.db
```
Output structure:
```json
{
  "journey": "bug-fix",
  "token_budget": 2600,
  "phases": [
    {
      "phase": 1,
      "label": "Bug Location",
      "tokens": 100,
      "content": {
        "entity": "rust:fn:authenticate:auth_rs:42-68",
        "signature": "pub fn authenticate(token: &str) -> Result<User, Error>",
        "error_line": 45,
        "error_message": "null pointer dereference"
      }
    },
    {
      "phase": 2,
      "label": "Blast Radius",
      "tokens": 500,
      "content": {
        "affected_entities": [...],  // Forward/reverse deps (1 hop)
        "related_tests": [...],
        "recent_changes": [...]      // Temporal state
      }
    },
    {
      "phase": 3,
      "label": "Implementation Context",
      "tokens": 2000,
      "content": {
        "full_code": "...",
        "type_signatures": [...],
        "doc_comments": [...]
      }
    }
  ]
}
```
Total: 2.6K tokens (vs 30K in Level 1)

**B. Refactoring Journey**
```bash
parseltongue pt11-journey-context-builder \
  --journey refactor \
  --focus-module "src/auth" \
  --output context.json \
  --db rocksdb:project.db
```
Output structure:
```json
{
  "journey": "refactor",
  "token_budget": 2200,
  "phases": [
    {
      "phase": 1,
      "label": "Module Health",
      "tokens": 200,
      "content": {
        "complexity_metrics": {...},
        "coupling_scores": {...},
        "test_coverage_gaps": [...]
      }
    },
    {
      "phase": 2,
      "label": "Blast Radius",
      "tokens": 1000,
      "content": {
        "cross_module_dependencies": [...],
        "public_api_surface": [...],
        "deprecation_impact": [...]
      }
    },
    {
      "phase": 3,
      "label": "Similar Patterns",
      "tokens": 1000,
      "content": {
        "code_clusters": [...],
        "refactoring_candidates": [...]
      }
    }
  ]
}
```
Total: 2.2K tokens

**C. Pattern Research Journey**
```bash
parseltongue pt11-journey-context-builder \
  --journey pattern-research \
  --output context.json \
  --db rocksdb:project.db
```
Output structure:
```json
{
  "journey": "pattern-research",
  "token_budget": 7500,
  "phases": [
    {
      "phase": 1,
      "label": "Architectural Overview",
      "tokens": 500,
      "content": {
        "layers": ["UI", "Business Logic", "Data"],
        "module_boundaries": [...],
        "cross_cutting_concerns": [...]
      }
    },
    {
      "phase": 2,
      "label": "Pattern Catalog",
      "tokens": 2000,
      "content": {
        "design_patterns": ["Builder", "Strategy", "Factory"],
        "architectural_styles": ["MVC", "Event-Driven"],
        "code_smells": ["God Class"]
      }
    },
    {
      "phase": 3,
      "label": "Exemplar Code",
      "tokens": 5000,
      "content": {
        "representative_implementations": [...],
        "best_practices": [...],
        "anti_patterns": [...]
      }
    }
  ]
}
```
Total: 7.5K tokens

**D. Security Audit Journey**
```bash
parseltongue pt11-journey-context-builder \
  --journey security-audit \
  --output context.json \
  --db rocksdb:project.db
```
Output structure:
```json
{
  "journey": "security-audit",
  "token_budget": 3000,
  "phases": [
    {
      "phase": 1,
      "label": "Risk Surface",
      "tokens": 500,
      "content": {
        "unsafe_code": [...],
        "public_apis": [...],
        "input_handlers": [...]
      }
    },
    {
      "phase": 2,
      "label": "Data Flow Paths",
      "tokens": 1500,
      "content": {
        "user_input_flows": [...],
        "sensitive_data_access": [...],
        "error_handling": [...]
      }
    },
    {
      "phase": 3,
      "label": "Vulnerability Analysis",
      "tokens": 1000,
      "content": {
        "potential_issues": [...],
        "confidence_scores": [...],
        "remediation_suggestions": [...]
      }
    }
  ]
}
```
Total: 3K tokens

**Success Metrics**:
- Bug-fix journey: 2-3K tokens (vs 30K baseline) = 10x reduction
- Refactor journey: 2-3K tokens = 10x reduction
- Pattern research: 7-8K tokens (vs 30K baseline) = 4x reduction
- Security audit: 3-4K tokens = 8x reduction
- Accuracy: 95%+ on meta-questions without code reading (validated by developer testing)

**Implementation Plan**:
- Week 5: Journey profiles (bug-fix, refactor) + phase builders
- Week 6: Pattern research + security audit journeys + integration tests

**Dependencies**: PT07 (analytics), PT08 (aggregation)

---

### Phase 4: Enhanced Indexing (Weeks 7-10) - "Deeper understanding"

**Goal**: Extract CFG, DFG, provenance metadata during indexing.

#### PT01 Extensions
**Scope**: Multi-pass analysis with confidence scoring

**New Analysis Passes**:

**Pass 1: Syntax Parsing** (Current - no changes)
- Tree-sitter parsing
- Entity extraction (functions, structs, traits)
- ISGL1 key generation
- Dependency edge extraction (Calls, Uses, Implements)

**Pass 2: Control Flow Graph (CFG)** (NEW)
- Extract basic blocks
- Identify control flow edges (sequential, conditional, loop)
- Store as DependencyEdges with `edge_type = 'ControlFlow'`
- Example: `rust:fn:main:1-50` → `rust:block:if_branch:main:10-15` (control flow)

**Pass 3: Data Flow Graph (DFG)** (NEW)
- Track variable definitions and uses
- Identify data dependencies
- Store as DependencyEdges with `edge_type = 'DataFlow'`
- Example: `rust:fn:calculate:10-20` → `rust:fn:validate:25-30` (variable `result` flows)

**Pass 4: Provenance Metadata** (NEW)
- Git integration: Parse commit messages, PR descriptions
- Extract design rationale (optional LLM assist)
- Store in CodeGraph: `provenance` (JSON column)

**Pass 5: Confidence Scoring** (NEW)
- Compute confidence for TDD_Classification fields
- Store alongside metrics: `complexity_confidence`, `coverage_confidence`

**New CLI Flags**:
```bash
parseltongue pt01-folder-to-cozodb-streamer ./src \
  --analyze-level <basic|standard|deep> \
  --extract-cfg \
  --extract-dfg \
  --extract-provenance \
  --git-repo .git \
  --db rocksdb:project.db
```

**Analyze Levels**:
- `basic`: Syntax only (current behavior, fast)
- `standard`: + CFG + provenance (moderate speed)
- `deep`: + DFG + confidence scoring (slower, more accurate)

**Success Metrics**:
- CFG extraction: 100% coverage for Rust control flow constructs
- DFG extraction: 90%+ accuracy on variable flow tracking
- Provenance: 80%+ commit messages successfully parsed
- Performance: <2x slowdown for `standard` level vs `basic`

**Implementation Plan**:
- Week 7: CFG extraction (control flow edges)
- Week 8: DFG extraction (data flow edges)
- Week 9: Provenance metadata (Git integration)
- Week 10: Confidence scoring + integration tests

**Dependencies**: Rust crates: git2 (Git operations), petgraph (CFG/DFG construction)

---

### Phase 5: Graph Reasoning (Weeks 11-12) - "Intelligent inference"

**Goal**: Pattern detection, semantic clustering, impact inference.

#### PT09: graph-reasoning
**Scope**: High-level graph queries with inference

**Features**:

**A. Pattern Detection** (Identify design patterns)
```bash
parseltongue pt09-graph-reasoning \
  --detect-patterns \
  --pattern-types "Builder,Factory,Singleton,Strategy" \
  --output patterns.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "patterns": [
    {
      "type": "Builder",
      "confidence": 0.87,
      "entities": [
        "rust:struct:ConfigBuilder:config_rs:10-50",
        "rust:fn:with_timeout:config_rs:25-30",
        "rust:fn:build:config_rs:45-50"
      ],
      "rationale": "Fluent API with chained setters and terminal build() method"
    }
  ]
}
```

**B. Semantic Clustering** (Group related entities)
```bash
parseltongue pt09-graph-reasoning \
  --cluster-by-semantic \
  --similarity-threshold 0.75 \
  --output clusters.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "clusters": [
    {
      "label": "Authentication",
      "entities": 23,
      "keywords": ["token", "auth", "login", "verify"],
      "confidence": 0.82
    }
  ]
}
```

**C. Impact Inference** (What breaks if X changes?)
```bash
parseltongue pt09-graph-reasoning \
  --infer-impact \
  --entity "rust:fn:authenticate:auth_rs:42-68" \
  --include-transitive \
  --output impact.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "changed_entity": "rust:fn:authenticate:auth_rs:42-68",
  "impact": {
    "direct_dependents": 12,
    "transitive_dependents": 45,
    "affected_tests": 8,
    "risk_level": "HIGH",
    "confidence": 0.78,
    "recommendations": [
      "Update API documentation (signature changed)",
      "Run integration test suite (auth_tests)",
      "Review 3 high-coupling dependents"
    ]
  }
}
```

**D. Architectural Layer Detection** (UI, Business, Data layers)
```bash
parseltongue pt09-graph-reasoning \
  --detect-layers \
  --output layers.json \
  --db rocksdb:project.db
```
Output:
```json
{
  "layers": [
    {
      "name": "UI Layer",
      "entities": 34,
      "confidence": 0.85,
      "characteristics": ["No database access", "High fan-in from business layer"]
    },
    {
      "name": "Business Logic",
      "entities": 67,
      "confidence": 0.92,
      "characteristics": ["Orchestrates data layer", "Core domain logic"]
    },
    {
      "name": "Data Layer",
      "entities": 23,
      "confidence": 0.88,
      "characteristics": ["Database queries", "No UI dependencies"]
    }
  ],
  "violations": [
    {
      "issue": "UI layer directly calls data layer",
      "entities": ["rust:fn:render_user:ui_rs:10-25", "rust:fn:query_users:db_rs:50-60"],
      "severity": "MEDIUM"
    }
  ]
}
```

**Success Metrics**:
- Pattern detection: 80%+ precision on common patterns (Builder, Factory, Strategy)
- Semantic clustering: 90%+ agreement with human-labeled clusters
- Impact inference: 95%+ accuracy on blast radius prediction
- Layer detection: 85%+ accuracy on architectural layer classification

**Implementation Plan**:
- Week 11: Pattern detection + semantic clustering
- Week 12: Impact inference + layer detection + integration tests

**Dependencies**: Rust crates: petgraph (graph algorithms), rust-bert (optional: embeddings for semantic similarity)

---

## Technical Architecture: Integration with CozoDB/ISGL1

### Schema Extensions

**Current Schema (v0.8.6)**:
```
CodeGraph {
  ISGL1_key: String (PK),
  entity_name: String,
  entity_type: String,
  file_path: String,
  line_number: String,
  interface_signature: JSON,
  current_code: String?,
  future_code: String?,
  current_ind: Bool,
  future_ind: Bool,
  future_action: String?,
  TDD_Classification: JSON,
  doc_comment: String?,
  // Level 2 only:
  return_type: String?,
  param_types: JSON?,
  is_async: Bool?,
  is_unsafe: Bool?,
  generic_constraints: JSON?,
  trait_impls: JSON?
}

DependencyEdges {
  from_key: String (FK → CodeGraph.ISGL1_key),
  to_key: String (FK → CodeGraph.ISGL1_key),
  edge_type: String,  // "Calls", "Uses", "Implements"
  source_location: String?
}
```

**v0.9.0 Extensions**:

**CodeGraph: New Columns**
```
provenance: JSON? = {
  created_at: Timestamp,
  created_by: String,
  created_because: String,  // Design rationale
  related_issue: URL?,
  deprecated_at: Timestamp?,
  evolution: [
    {version: Int, timestamp: Timestamp, action: String, reason: String}
  ]
}

abstract_state: JSON? = {
  preconditions: [String],
  postconditions: [String],
  invariants: [String],
  confidence: Float
}

security_findings: JSON? = [
  {
    type: String,  // "null_pointer", "buffer_overflow", "integer_overflow"
    severity: String,  // "LOW", "MEDIUM", "HIGH", "CRITICAL"
    confidence: Float,
    description: String
  }
]

analysis_metadata: JSON? = {
  complexity_confidence: Float,
  coverage_confidence: Float,
  risk_confidence: Float,
  analyzed_at: Timestamp,
  analyze_level: String  // "basic", "standard", "deep"
}
```

**DependencyEdges: New Edge Types**
```
edge_type values (extended):
  // Existing:
  - "Calls"
  - "Uses"
  - "Implements"

  // New (v0.9.0):
  - "ControlFlow"         // CFG edges
  - "DataFlow"            // DFG edges
  - "ProgramDependency"   // PDG edges
  - "InferredDependency"  // Transitive closure (computed)
  - "SimilarPattern"      // Semantic similarity (computed)
  - "ArchitecturalLayer"  // Layer membership (computed)
```

**New Table: AggregatedViews** (Computed, cached)
```
AggregatedViews {
  view_id: String (PK),      // "package:parseltongue-core", "module:src_auth"
  view_type: String,         // "package", "module", "cluster", "layer"
  label: String,             // Human-readable name
  entities: [String],        // Array of ISGL1_keys
  metrics: JSON,             // Aggregated stats
  computed_at: Timestamp
}
```

**New Table: JourneyProfiles** (Journey definitions)
```
JourneyProfiles {
  journey_id: String (PK),   // "bug-fix", "refactor", "pattern-research"
  phases: JSON,              // Phase definitions with token budgets
  filters: JSON,             // CozoDB query filters per phase
  created_at: Timestamp
}
```

### Backward Compatibility Strategy

**Principle**: v0.8.6 commands must work identically with v0.9.0 database.

**Implementation**:
1. **Schema Additions Only**: New columns nullable, default NULL
2. **Edge Type Filtering**: Old queries filter by known edge types automatically
3. **New Commands Only**: pt07-pt11 are new, pt01-pt06 unchanged
4. **Graceful Degradation**: If new columns missing, PT07 shows "N/A" for metrics

**Testing**: Run full v0.8.6 test suite against v0.9.0 codebase (must pass 100%)

### Query Performance Optimization

**Targets**:
- Blast radius (5 hops): <50ms on 10K entity graph
- Clustering (Louvain): <100ms on 10K entity graph
- Pattern detection: <200ms on 1K entity subset
- Journey context building: <150ms end-to-end

**Techniques**:
1. **Indexes**: Add indexes on `entity_type`, `file_path`, `future_action`
2. **Materialized Views**: Cache aggregated metrics in `AggregatedViews` table
3. **Query Plan Analysis**: Use CozoDB's explain to optimize Datalog queries
4. **Lazy Rendering**: Compute heavy analytics on-demand, not during indexing
5. **Parallel Queries**: Use Rayon for embarrassingly parallel analyses

### Data Flow (v0.9.0 Extended)

```
┌─────────────────────────────────────────────────────────────────┐
│ Step 1: Enhanced Ingest (pt01 --analyze-level deep)             │
│ Input: Codebase                                                 │
│ Output: CodeGraph (entities) + DependencyEdges (AST+CFG+DFG)   │
│ Performance: 200-500ms for 1K entities (2x slower than v0.8.6)  │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2a: Progressive Disclosure (pt02-level01) [UNCHANGED]      │
│ Output: entities.json (~30K tokens)                             │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2b: Graph Analytics (pt07) [NEW]                           │
│ Output: Dashboard, complexity report, blast radius              │
│ Performance: <100ms                                             │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2c: Semantic Aggregation (pt08) [NEW]                      │
│ Output: Clusters, signatures, multi-level views                 │
│ Tokens: 500-2K (vs 30K in pt02)                                 │
│ Performance: <150ms                                             │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2d: Journey Context (pt11) [NEW]                           │
│ Output: Task-specific JSON (bug-fix: 2.6K tokens)               │
│ Performance: <150ms                                             │
└─────────────────────────────────────────────────────────────────┘
                              ↓
         (LLM analyzes context, decides on changes)
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 3-6: Unchanged (pt03, pt04, pt05, pt06)                    │
│ Edit → Validate → Diff → Reset                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Success Metrics: How to Measure v0.9.0 Effectiveness

### Quantitative Metrics

#### 1. Token Efficiency
**Baseline (v0.8.6 Level 1)**: 30K tokens for codebase overview
**Target (v0.9.0)**:
- Journey context (bug-fix): 2-3K tokens = **10x reduction**
- Journey context (refactor): 2-3K tokens = **10x reduction**
- Journey context (pattern-research): 7-8K tokens = **4x reduction**
- Semantic aggregation (clusters): 500-1K tokens = **30-60x reduction**
- Multi-level view (packages): 500-1K tokens = **30-60x reduction**

**Measurement**: Compare token counts before/after for identical comprehension tasks.

#### 2. Query Performance
**Targets**:
- PT07 dashboard report: <100ms on 1000-entity codebase
- PT08 clustering: <150ms on 1000-entity codebase
- PT09 pattern detection: <200ms on 1000-entity codebase
- PT11 journey building: <150ms end-to-end
- PT01 deep analysis: <500ms for 1000 entities (vs 250ms basic)

**Measurement**: Benchmark suite with varying codebase sizes (100, 500, 1000, 5000 entities).

#### 3. Comprehension Accuracy
**Test Suite**: 50 meta-questions across 5 categories
- Architectural: "What are the main modules?" (10 questions)
- Dependencies: "What depends on X?" (10 questions)
- Complexity: "Where is technical debt?" (10 questions)
- Security: "Where is unsafe code?" (10 questions)
- Temporal: "What changed recently?" (10 questions)

**Baseline (human expert)**: 100% correct answers
**Target (v0.9.0)**: 95%+ correct answers without reading code

**Measurement**: Developer study with 5 participants, compare answers against ground truth.

### Qualitative Metrics

#### 4. Developer Productivity
**Baseline**: Time spent "figuring out what to work on"
**Measurement**: Developer survey before/after v0.9.0 adoption
- Question: "How long do you spend understanding codebase before starting work?"
- Baseline average: 30 minutes per task
- Target: 6 minutes per task (80% reduction)

#### 5. Tool Adoption
**Indicators**:
- % of developers using PT07 dashboard daily (target: 70%+)
- % of refactorings preceded by PT07 blast-radius check (target: 80%+)
- % of bug fixes using PT11 journey context (target: 60%+)

**Measurement**: Telemetry (opt-in), developer surveys

#### 6. Decision Confidence
**Survey Questions**:
- "I feel confident in my refactoring decisions" (1-5 scale)
- "I understand the impact of my changes" (1-5 scale)
- "I can quickly identify technical debt" (1-5 scale)

**Baseline**: Average 2.5/5
**Target**: Average 4.0/5 (60% improvement)

### Regression Testing

**Requirement**: All v0.8.6 test suites must pass with v0.9.0 codebase.

**Test Suites**:
- PT01 indexing tests (31/31 passing)
- PT02 export tests (31/31 passing)
- PT03 write tests (passing)
- PT04 validation tests (passing)
- PT05 diff tests (passing)
- PT06 reset tests (passing)
- ActuallyWorks end-to-end suite (passing)

**Target**: 100% pass rate (zero regressions)

---

## Implementation Roadmap

### Timeline Overview

```
┌─────────────┬─────────────┬─────────────┬─────────────┐
│  Weeks 1-2  │  Weeks 3-4  │  Weeks 5-6  │  Weeks 7-10 │
│   PT07      │   PT08      │   PT11      │  PT01 Ext   │
│ Analytics   │ Aggregation │  Journeys   │   CFG/DFG   │
└─────────────┴─────────────┴─────────────┴─────────────┘
                                          │  Weeks 11-12│
                                          │    PT09     │
                                          │  Reasoning  │
                                          └─────────────┘
```

### Milestones

**M1: Foundation Complete (Week 2)**
- PT07 with 8 report types working
- Terminal tables, JSON/CSV output
- Integration tests passing
- Demo: "Morning standup dashboard"

**M2: Semantic Aggregation Complete (Week 4)**
- PT08 with modernity signatures, clustering, multi-level views
- Token reduction validated (30-60x)
- Demo: "Package-level architecture overview"

**M3: Journey-Aware Compression Complete (Week 6)**
- PT11 with 4 journey types
- Token budgets validated (10x reduction for bug-fix)
- Demo: "Bug-fix journey from error to fix"

**M4: Enhanced Indexing Complete (Week 10)**
- PT01 with CFG, DFG, provenance extraction
- Confidence scoring implemented
- Performance validated (<2x slowdown)
- Demo: "Data flow tracing"

**M5: Graph Reasoning Complete (Week 12)**
- PT09 with pattern detection, impact inference, layer detection
- Accuracy validated (80-95% on benchmarks)
- Demo: "Detect Builder pattern across codebase"

**M6: v0.9.0 Release Candidate (Week 13)**
- All test suites passing (including v0.8.6 regression tests)
- Documentation complete
- Performance benchmarks met
- Developer study complete (5 participants)

**M7: v0.9.0 Production Release (Week 14)**
- Release notes published
- Binary artifacts built (macOS, Linux)
- GitHub release created
- Blog post: "From Progressive Disclosure to Intelligent Aggregation"

### Risk Mitigation

**Risk 1: Performance degradation**
- Mitigation: Benchmark continuously, optimize hot paths, add caching
- Fallback: Provide `--fast` flag that skips heavy analyses

**Risk 2: Accuracy below target (95%)**
- Mitigation: Developer study early (week 8), iterate on algorithms
- Fallback: Ship with confidence scores, let users decide trust level

**Risk 3: Schema migration complexity**
- Mitigation: Backward compatibility tested rigorously
- Fallback: Provide migration tool: `parseltongue migrate --from v0.8.6`

**Risk 4: Scope creep**
- Mitigation: Priority matrix (P0/P1/P2), cut P2 features if needed
- Fallback: Defer PT09 (reasoning) to v0.10.0 if timeline slips

---

## References

### Research Papers

**Code Property Graphs**:
1. Vul-LMGNN: "Source Code Vulnerability Detection: Combining Code Language Models and Code Property Graphs" (April 2024) - arxiv.org/html/2404.14719v1
2. VulMPFF: "A Vulnerability Detection Method for Fusing Code Features in Multiple Perspectives" (March 2024) - IET Information Security
3. GraphFVD: "Property graph-based fine-grained vulnerability detection" (January 2025) - ScienceDirect
4. AISE: "Synergizing Abstract Interpretation and Symbolic Execution" (2024) - SpringerLink

**Semantic Code Graphs**:
5. Semantic Code Graph: "An information model to facilitate software comprehension" (October 2023) - arxiv.org/abs/2310.02128
6. "Precise Learning of Source Code Contextual Semantics via Hierarchical Dependence Structure" (2021) - ScienceDirect

**Hierarchical Context Compression**:
7. HOMER: "Hierarchical Context Merging: Better Long Context Understanding for Pre-trained LLMs" (ICLR 2024) - OpenReview
8. TokenSkip: "Controllable Chain-of-Thought Compression in LLMs" (2025) - arxiv.org/abs/2502.12067
9. Acon: "Optimizing Context Compression for Long-horizon LLM Agents" (2024) - arxiv.org/html/2510.00615

**Knowledge Graphs**:
10. FalkorDB: "Code Graph: From Visualization to Integration" (2024) - falkordb.com/blog/code-graph
11. FalkorDB: "GraphRAG & Knowledge Graphs for LLMs" (2024) - falkordb.com/blog
12. "Knowledge Graph vs Vector Database for RAG" (2024) - Neo4j blog

**Static Analysis**:
13. "Sound Symbolic Execution via Abstract Interpretation" (2023) - arxiv.org/abs/2301.07783
14. "Scaling Symbolic Execution to Large Software Systems" (2024) - arxiv.org/html/2408.01909v1

**Multi-Level Abstraction**:
15. CodEx: "Facilitating program comprehension with call graph multilevel hierarchical abstractions" (2021) - ScienceDirect
16. CoLadder: "Supporting Programmers with Hierarchical Code Generation in Multi-Level Abstraction" (2023) - arxiv.org/abs/2310.08699

**Provenance Tracking**:
17. Software Heritage: "Software provenance tracking at the scale of public source code" (2020) - Springer
18. PAV Ontology: "Provenance, authoring and versioning" (2013) - Journal of Biomedical Semantics
19. SLSA: "Supply-chain Levels for Software Artifacts - Provenance" (2024) - slsa.dev/spec

### Internal Documentation

**Parseltongue v0.8.6**:
- README.md: Project overview, 8 commands, progressive disclosure
- PRDv2.md: Product requirements, command specifications
- ISG_ANALYTICS_RESEARCH.md: PT07 research (57KB, 40+ queries)
- PT07_VISUAL_MOCKUPS.md: Report mockups (67KB, 8 report types)
- PT07_IMPLEMENTATION_GUIDE.md: Code examples (32KB)
- TEST-RESULTS.md: v0.8.6 validation (765 entities, all commands working)

**Steering Documents**:
- S01: Design principles (TDD-first, ultra-minimalist, dependency injection)

### Related Tools

**Comparison Analysis**:
- tokei: Line counting (no complexity, no risk)
- cargo-tree: Crate dependencies (not code-level)
- cargo-bloat: Binary size (not code quality)
- rust-analyzer: IDE features (not meta-analysis)
- **Parseltongue v0.9.0**: Meta-level understanding, graph reasoning, journey-aware compression

---

## Appendix A: Example Query Patterns

### A.1 Graph Analytics Queries (PT07)

**Health Score Calculation**:
```datalog
# Aggregate metrics for health score
?[avg_complexity, coverage, high_risk_count, doc_coverage] :=
    *CodeGraph{TDD_Classification, doc_comment},
    complexity = json_extract(TDD_Classification, '$.complexity'),
    risk = json_extract(TDD_Classification, '$.change_risk'),
    coverage = json_extract(TDD_Classification, '$.test_coverage_estimate'),
    avg_complexity = mean(complexity),
    doc_coverage = count_if(doc_comment != null) / count(*),
    high_risk_count = count_if(risk = 'High')

# Health score formula:
# score = (0.3 * coverage_score) +
#         (0.3 * complexity_score) +
#         (0.2 * doc_score) +
#         (0.2 * risk_score)
```

**Blast Radius (5 hops)**:
```datalog
# Transitive closure with max depth 5
affected[to_key] := *DependencyEdges{from_key, to_key},
                    from_key = $entity_key
affected[next] := affected[current],
                  *DependencyEdges{from_key: current, to_key: next},
                  depth <= 5

?[affected_key, entity_name, file_path] :=
    affected[affected_key],
    *CodeGraph{ISGL1_key: affected_key, entity_name, file_path}
```

### A.2 Semantic Aggregation Queries (PT08)

**Modernity Signatures**:
```datalog
# Module-level async usage
?[module, async_ratio] :=
    *CodeGraph{file_path, is_async},
    module = split(file_path, '/')[0],  # First path segment
    async_count = count_if(is_async = true),
    total_count = count(*),
    async_ratio = async_count / total_count
:group module

# Dependency modernity (external crates)
?[crate_name, latest_version, codebase_version, outdated] :=
    *Dependencies{crate_name, version: codebase_version},
    latest_version = fetch_latest_from_crates_io(crate_name),
    outdated = (codebase_version < latest_version)
```

**Dependency Clustering (Louvain preprocessing)**:
```datalog
# Extract graph structure for clustering algorithm
?[from_key, to_key, weight] :=
    *DependencyEdges{from_key, to_key, edge_type},
    weight = case edge_type
             when 'Calls' then 1.0
             when 'Uses' then 0.8
             when 'Implements' then 1.2
             else 0.5
:export graph_edges.json

# Post-clustering: Label clusters
?[cluster_id, label, entities] :=
    *ClusterAssignments{entity_key, cluster_id},
    entities = collect(entity_key),
    # LLM labels cluster based on entity names
    label = llm_label_cluster(entities)
```

### A.3 Journey-Aware Queries (PT11)

**Bug-Fix Journey - Phase 2 (Blast Radius)**:
```datalog
# Affected entities (1 hop forward + reverse)
affected[to_key] := *DependencyEdges{from_key, to_key},
                    from_key = $bug_entity_key
affecting[from_key] := *DependencyEdges{from_key, to_key},
                       to_key = $bug_entity_key

?[entity_key, entity_name, entity_type, file_path, direction] :=
    affected[entity_key],
    *CodeGraph{ISGL1_key: entity_key, entity_name, entity_type, file_path},
    direction = 'forward'
UNION
?[entity_key, entity_name, entity_type, file_path, direction] :=
    affecting[entity_key],
    *CodeGraph{ISGL1_key: entity_key, entity_name, entity_type, file_path},
    direction = 'reverse'

# Related tests
?[test_key, test_name] :=
    affected[entity_key],
    *CodeGraph{ISGL1_key: test_key, entity_name: test_name, TDD_Classification},
    entity_class = json_extract(TDD_Classification, '$.entity_class'),
    entity_class = 'TestImplementation',
    test_name ~ entity_key  # Test name contains entity name

# Recent changes (temporal state)
?[changed_key, future_action, future_code] :=
    affected[changed_key],
    *CodeGraph{ISGL1_key: changed_key, future_action, future_code},
    future_action != null
```

**Refactoring Journey - Phase 1 (Module Health)**:
```datalog
# Complexity metrics per module
?[module, avg_complexity, max_complexity, complex_count] :=
    *CodeGraph{file_path, TDD_Classification},
    module = split(file_path, '/')[0],
    complexity = json_extract(TDD_Classification, '$.complexity'),
    avg_complexity = mean(complexity),
    max_complexity = max(complexity),
    complex_count = count_if(complexity = 'Complex')
:group module

# Coupling scores
?[module, fan_in, fan_out, coupling_score] :=
    module_entities[module, entity_key],
    fan_in = count(*DependencyEdges{to_key: entity_key}),
    fan_out = count(*DependencyEdges{from_key: entity_key}),
    coupling_score = fan_in + fan_out
:aggregate by module

# Test coverage gaps
?[module, total_entities, tested_entities, coverage_ratio] :=
    *CodeGraph{file_path, TDD_Classification},
    module = split(file_path, '/')[0],
    entity_class = json_extract(TDD_Classification, '$.entity_class'),
    total_entities = count_if(entity_class = 'CodeImplementation'),
    coverage = json_extract(TDD_Classification, '$.test_coverage_estimate'),
    tested_entities = count_if(coverage > 0.5),
    coverage_ratio = tested_entities / total_entities
:group module
```

---

## Appendix B: Tool Comparison Matrix

| Feature | v0.8.6 | v0.9.0 PT07 | v0.9.0 PT08 | v0.9.0 PT11 |
|---------|--------|-------------|-------------|-------------|
| **Token Efficiency** | 30K (Level 1) | 30K (same) | 500-1K | 2-8K (journey-specific) |
| **Blast Radius** | Manual Datalog | 1-click report | Cluster-aware | Journey-integrated |
| **Health Score** | No | Yes (0-100) | Yes + trends | Journey-specific health |
| **Complexity Analysis** | Placeholder | Real metrics | Module-level | Hotspot identification |
| **Test Coverage** | Placeholder | Real gaps | Module coverage | Journey-relevant tests |
| **Dependency Analysis** | Edge list | Coupling metrics | Community detection | Impact-aware |
| **Architectural View** | No | No | Multi-level (pkg/mod/class) | Layer-aware |
| **Provenance** | Temporal only | No | No | Design rationale |
| **Pattern Detection** | No | No | Modernity signatures | Pattern-aware journeys |
| **CFG/DFG** | No | No | No | Yes (PT01 extension) |
| **Semantic Clustering** | No | No | Yes (Louvain) | Journey-specific clusters |
| **Output Formats** | JSON | JSON/CSV/Terminal | JSON | JSON (hierarchical) |
| **Query Performance** | <1s | <100ms | <150ms | <150ms |

---

## Appendix C: Glossary

**Abstract Interpretation**: Approximate program semantics to reason about all possible executions without running code.

**Blast Radius**: Set of entities affected by changes to a specific entity (transitive dependencies).

**CFG (Control Flow Graph)**: Graph representing all paths that might be traversed during program execution.

**CPG (Code Property Graph)**: Unified graph combining AST, CFG, DFG, and PDG for comprehensive code analysis.

**Datalog**: Declarative query language used by CozoDB for graph traversal and pattern matching.

**DFG (Data Flow Graph)**: Graph representing flow of data through variable definitions and uses.

**ISGL1**: Interface Signature Graph Level 1 - Parseltongue's unique identifier format for code entities.

**Journey-Aware Compression**: Task-specific context building that adapts information density to the developer's current goal.

**Knowledge Graph**: Graph database that captures structured relationships and supports reasoning/inference.

**Meta-Level Understanding**: Comprehension of codebase architecture without reading actual implementation code.

**Modernity Signature**: Compact summary of code "style" (async usage, error handling patterns, dependency versions).

**PDG (Program Dependence Graph)**: Graph combining control and data dependencies.

**Progressive Disclosure**: Parseltongue's v0.8.6 approach of offering information at increasing detail levels (Level 0/1/2).

**Provenance**: Origin and evolution history of code, including design rationale and decision context.

**Semantic Aggregation**: Intelligent grouping of related entities based on semantic similarity or structural properties.

**Temporal State**: Parseltongue's system for tracking pending changes (current_ind, future_ind, future_action).

---

**Document Status**: FINAL
**Review Date**: 2025-11-02
**Approved for Implementation**: Yes
**Next Steps**: Begin Phase 1 (PT07 implementation) - Week 1

---

*This document represents the comprehensive research and planning for Parseltongue v0.9.0. All research findings have been validated through 2024 academic papers and industry practices. Implementation will follow TDD-first principles with continuous validation against success metrics.*
