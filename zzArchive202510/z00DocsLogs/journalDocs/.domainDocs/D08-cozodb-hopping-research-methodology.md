# D08: CozoDB Graph Hopping Research Methodology

**Status**: Research Protocol
**Date**: 2025-10-31
**Purpose**: Systematic discovery of production-proven CozoDB recursive query patterns
**Related**: D07-dependency-tracking-gap-analysis.md
**Research Protocol**: Multi-Layered Anthropological Software Archaeology

---

## Executive Summary

This document outlines the systematic research methodology for discovering, evaluating, and acquiring best-in-market CozoDB implementations demonstrating advanced graph hopping and dependency traversal patterns. The research directly addresses the critical dependency tracking gap identified in D07.

**Research Goal**: Find and extract proven Datalog patterns for implementing function-to-function dependency tracking in CozoDB.

**Success Criteria**:
- ✅ 3-5 high-quality reference implementations acquired
- ✅ Concrete recursive query patterns extracted
- ✅ Production-tested approaches documented
- ✅ Implementation roadmap validated or refined

---

## Problem Context (from D07)

### What We Lost
The previous Interface Signature Graph (ISG) implementation provided:
- Sub-millisecond dependency queries (<50μs for 1-hop)
- Blast radius calculation (<1ms)
- Path finding (<100μs)
- Trait implementor lookup (<500μs)

### What We Need to Restore
CozoDB-native patterns for:
1. **1-hop queries**: "What functions does X call?" / "Who calls X?"
2. **N-hop traversal**: Multi-level dependency chains with distance tracking
3. **Blast radius**: Bounded recursive queries (stop at depth N)
4. **Transitive closure**: Full reachability analysis
5. **Shortest path**: BFS-style path finding in Datalog
6. **Reverse indexing**: Efficient "find all callers" queries

### Current Schema Constraint
Must work with our CodeGraph + DependencyEdges architecture:
```datalog
:create CodeGraph {
    ISGL1_key: String => /* node data */
}

:create DependencyEdges {
    from_isgl1_key: String,
    to_isgl1_key: String =>
    edge_type: String,  # "Calls", "Uses", "Implements"
    source_location: String?
}
```

---

## Research Philosophy: Multi-Layered Software Archaeology

Instead of simply searching for "popular CozoDB repos", we employ a **systematic anthropological approach** that excavates the CozoDB ecosystem in layers:

### Layer 1: Official Canonical Sources
- CozoDB core repository (reference implementations)
- Official documentation and examples
- Tutorial materials and test suites

### Layer 2: Production Implementations
- Real-world projects actively using CozoDB
- Production code with battle-tested patterns
- Active maintenance signals (commits in last 6 months)

### Layer 3: Academic & Research
- Papers citing CozoDB with reference implementations
- Research prototypes demonstrating novel patterns
- Cutting-edge techniques not yet in mainstream

### Layer 4: Cross-Pollination
- Similar Datalog engines (Souffle, Datafun) with portable patterns
- Graph databases (Datomic, Neo4j) with analogous query structures
- Code analysis tools using recursive graph queries

**Rationale**: This layered approach ensures we don't miss hidden gems—repos with low stars but high technical quality, or patterns proven in related domains.

---

## Evaluation Framework

### Primary Evaluation Criteria

Every candidate repository must be assessed against these dimensions:

#### 1. Relevance Score (0-10)
**Question**: "Does this repo demonstrate the patterns we need?"

| Score | Criteria |
|-------|----------|
| 9-10 | Multiple recursive query patterns, directly applicable to our use case |
| 7-8 | Some relevant patterns, requires adaptation |
| 5-6 | General CozoDB usage, few specific hopping examples |
| 3-4 | Mentions CozoDB but limited query examples |
| 0-2 | Irrelevant or no actual CozoDB Datalog code |

**Minimum threshold**: 6/10

#### 2. Quality Score (0-10)
**Question**: "Can we trust and learn from this code?"

Quality signals:
- ✅ Tests present (+2 points)
- ✅ Documentation (+2 points)
- ✅ Production usage indicators (+2 points)
- ✅ Active maintenance (+2 points)
- ✅ Code clarity and comments (+2 points)

**Minimum threshold**: 5/10

#### 3. Pattern Coverage Checklist

For each repo, mark which patterns are present:

```
[ ] 1-hop forward query (X calls what?)
[ ] 1-hop reverse query (who calls X?)
[ ] N-hop traversal with distance tracking
[ ] Transitive closure (full reachability)
[ ] Bounded recursion (stop at depth N)
[ ] Shortest path (BFS in Datalog)
[ ] Cycle detection
[ ] Topological sorting
[ ] Graph aggregations (count descendants, etc.)
[ ] Performance optimizations (indices, query hints)
```

**Minimum threshold**: 2+ patterns present

#### 4. Size & Complexity Assessment
**Question**: "Is this manageable to analyze?"

| Category | Lines of Code | Verdict |
|----------|---------------|---------|
| Ideal | < 5k LOC | Easy to extract patterns |
| Good | 5k - 20k LOC | Manageable with focused analysis |
| Acceptable | 20k - 50k LOC | Requires selective file analysis |
| Challenging | > 50k LOC | Only if extremely high relevance/quality |

#### 5. Recency & Activity
**Question**: "Is this current with CozoDB capabilities?"

| Status | Last Commit | Verdict |
|--------|-------------|---------|
| Active | < 3 months | Current with latest CozoDB features |
| Maintained | 3-12 months | Likely still relevant |
| Stable | 1-2 years | Check if patterns still work |
| Archived | > 2 years | Verify compatibility before using |

### Composite Decision Matrix

A repository is **APPROVED FOR CLONING** if:
```
(Relevance >= 6) AND (Quality >= 5) AND (Pattern_Count >= 2)
AND (Size is manageable OR Relevance >= 9)
```

---

## Research Execution Plan

### Phase 1: Inventory Existing Resources (Day 1 - 1 hour)

**Objective**: Understand what we already have before searching externally.

**Tasks**:
1. Examine `/Users/amuldotexe/Projects/parseltongue/.doNotCommit/.refGithubRepo/cozo/`
   - List all example files
   - Identify graph-related queries
   - Note any hopping patterns already present
   - Document what's missing

2. Check other existing repos in `.refGithubRepo/` for incidental CozoDB usage

3. Create baseline inventory:
   ```markdown
   ## Already Available
   - cozo/examples/[file]: [patterns present]
   - [other-repo]/[path]: [incidental patterns]

   ## Gaps to Fill
   - [pattern not found in existing repos]
   ```

**Deliverable**: `INVENTORY.md` in `.refGithubRepo/`

---

### Phase 2: Official CozoDB Sources (Day 1 - 2 hours)

**Objective**: Extract all canonical patterns from CozoDB itself.

#### Task 2.1: CozoDB Main Repository Deep Dive
If not fully cloned, ensure we have:
- `cozodb/cozo` - main repository
- Complete clone of: `examples/`, `tests/`, `docs/`, `cozo-core/src/runtime/` (query engine)

#### Task 2.2: Catalog Official Examples
Create structured catalog:
```markdown
## CozoDB Official Examples - Graph Patterns

### File: examples/[name].cozo
**Pattern Type**: Transitive Closure
**Code**:
```datalog
?[...] := ...
```

**Explanation**: [what it does]
**Applicable To**: [our use case mapping]
**Performance Notes**: [if any]
```

#### Task 2.3: Test Suite Mining
Search CozoDB test files for:
- `tests/` directory
- Grep for: "recursive", "transitive", "closure", "path", "reachable"
- Extract query patterns from test cases

**Key Files to Examine**:
- `cozo/tests/queries/` (if exists)
- `cozo/cozo-core/src/runtime/transactional.rs` (test modules)

#### Task 2.4: Documentation Scraping
- Official docs: https://docs.cozodb.org
- Tutorial sections on recursive queries
- Any "cookbook" or "recipes" sections
- Blog posts on the CozoDB site

**Deliverable**: `OFFICIAL-PATTERNS.md` cataloging all canonical examples

---

### Phase 3: GitHub Production Repository Search (Day 1-2 - 4 hours)

**Objective**: Find real-world usage of CozoDB in production systems.

#### Search Strategy

Execute these GitHub searches (via web search tool):

##### Search Query 1: Recursive Patterns
```
"CozoDB" AND ("recursive" OR "transitive closure" OR "?[")
language:Rust OR language:Python
stars:>5
pushed:>2024-01-01
```

##### Search Query 2: Dependency Analysis
```
"CozoDB" AND ("dependency graph" OR "call graph" OR "hopping")
language:Rust
pushed:>2024-01-01
```

##### Search Query 3: Dependency Declaration Search
```
filename:Cargo.toml "cozo"
# Then check repos for actual query usage
```

```
filename:requirements.txt "pycozo"
# Python projects using CozoDB
```

##### Search Query 4: Code Search
```
"?[" AND "cozoscript" extension:cozo
# Direct Datalog query files
```

##### Search Query 5: Issue/Discussion Mining
Search CozoDB repository issues/discussions for:
- "recursive query example"
- "transitive closure"
- "graph traversal"
- "shortest path"
- Users asking "how do I..." questions (often get example answers)

#### Evaluation Process

For each candidate repository found:

1. **Quick Scan** (2 minutes):
   - README: Does it mention graph/dependency/traversal?
   - File list: Any `.cozo` files or obvious query modules?
   - Recent activity: Commits in last 6 months?

2. **Deep Evaluation** (10 minutes if quick scan passes):
   - Search codebase for `?[` (Datalog query syntax)
   - Count query patterns matching our needs
   - Assess code quality (tests, docs, structure)
   - Apply evaluation framework scores

3. **Decision**: CLONE / SKIP / BOOKMARK
   - **CLONE**: Meets thresholds, immediate value
   - **SKIP**: Doesn't meet criteria, document why
   - **BOOKMARK**: Interesting but lower priority

**Deliverable**: `GITHUB-EVALUATION.md` with scored repositories

---

### Phase 4: Academic & Research Sources (Day 2 - 2 hours)

**Objective**: Find cutting-edge patterns from research community.

#### Research Venues to Search

##### Academic Search Engines
1. **Google Scholar**:
   - Search: `"CozoDB" OR "Cozo" database`
   - Search: `Datalog "recursive query" "graph traversal"`
   - Look for: Papers with "code available" or GitHub links

2. **arXiv.org**:
   - Category: cs.DB (Databases), cs.SE (Software Engineering)
   - Search: `Datalog transitive closure`

3. **Conference Proceedings**:
   - SIGMOD, VLDB, ICDE (database conferences)
   - PLDI, POPL (programming language conferences with Datalog research)

##### Code Repositories from Papers
- Papers with Artifacts (reproducibility packages)
- Supplementary material repositories
- Benchmark implementations

#### Specific Pattern Hunt

Look for research on:
- **Seminaive evaluation** (optimized recursive query evaluation)
- **Magic sets transformation** (query optimization for Datalog)
- **Incremental view maintenance** (efficient updates to recursive queries)
- **Graph query optimization**

**Deliverable**: `ACADEMIC-SOURCES.md` with papers and associated code

---

### Phase 5: Cross-Pollination Sources (Day 2 - 2 hours)

**Objective**: Find analogous patterns in related systems that can be ported to CozoDB.

#### Related Datalog Systems

##### Souffle Datalog
- Website: https://souffle-lang.github.io/
- Examples repository
- Tutorials on transitive closure
- Performance optimization techniques

**Key Question**: "Can this Souffle pattern be translated to CozoDB Datalog?"

##### Datafun
- Functional Datalog language
- Academic research language with novel patterns

##### Flix (Datalog + Functional Programming)
- Modern Datalog implementation
- Check example programs

#### Related Graph Databases (Pattern Inspiration)

##### Datomic (Datalog-based)
- Query examples using Datalog
- Recursive rule patterns
- Translate Datomic syntax → CozoDB syntax

##### TerminusDB (Graph + Logic)
- WOQL query language examples
- Graph traversal patterns

#### Code Analysis Tools Using Datalog

##### CodeQL (GitHub)
- Uses Datalog for code analysis
- Dependency graph queries
- Call graph traversal examples
- **Highly relevant**: Security analysis often needs "find all callers" patterns

##### Semgrep
- Static analysis tool
- May have graph query patterns

##### Joern (Code Property Graphs)
- Uses Datalog-like queries
- Call graph analysis

**Deliverable**: `CROSS-POLLINATION.md` with portable patterns

---

### Phase 6: Community & Discussion Mining (Day 2 - 1 hour)

**Objective**: Find patterns shared in community discussions.

#### Sources to Check

1. **CozoDB GitHub Discussions**
   - Search for: graph, recursive, transitive, path
   - Look for user-contributed patterns

2. **Stack Overflow**
   - Tag: `cozodb` (if exists)
   - Search: `CozoDB recursive query`
   - Look for answered questions with example code

3. **Reddit**
   - r/programming, r/Database, r/rust
   - Search: `CozoDB`
   - Look for: "Show HN" posts, tutorials, discussions

4. **Hacker News**
   - Search: `CozoDB`
   - Check comment threads for code examples

5. **Discord/Slack/Zulip**
   - CozoDB community channels (if accessible)
   - Search history for query examples

6. **Twitter/X**
   - Search: `CozoDB example` or `@cozodb`
   - Developers sometimes share code snippets

**Deliverable**: `COMMUNITY-PATTERNS.md` with extracted examples

---

## Repository Cloning Protocol

### Directory Structure

Organize cloned repositories by source:
```
.refGithubRepo/
├── cozo/                          # Official (already exists)
├── cozodb-production/
│   ├── repo-name-1/
│   │   └── MANIFEST.md
│   ├── repo-name-2/
│   └── ...
├── cozodb-research/
│   └── paper-implementation-1/
├── datalog-reference/
│   ├── souffle-examples/
│   └── datomic-patterns/
└── RESEARCH-MANIFEST.md           # Master index
```

### Manifest Format

For each cloned repository, create `MANIFEST.md`:

```markdown
# [Repository Name]

**Source**: [GitHub URL / Paper URL / etc.]
**Cloned**: [Date]
**Evaluation Scores**:
- Relevance: X/10
- Quality: Y/10
- Pattern Count: Z

## Purpose
[Why we cloned this - specific patterns or techniques]

## Key Files for Analysis
- `path/to/file1.cozo`: [Demonstrates transitive closure]
- `path/to/file2.rs`: [Shows integration with Rust]
- `path/to/tests.rs`: [Performance benchmarks]

## Pattern Coverage
- [x] 1-hop forward query
- [x] Transitive closure
- [ ] Shortest path
- [x] Bounded recursion
(etc.)

## Extraction Priority
**HIGH / MEDIUM / LOW**

## Notes
[Any special considerations, compatibility issues, or insights]

## Quick Reference
**Most Important Query**:
```datalog
[paste the single most valuable query from this repo]
```

**Use Case**: [How it solves our problem]
```

### Cloning Commands

Use consistent naming:
```bash
cd /Users/amuldotexe/Projects/parseltongue/.doNotCommit/.refGithubRepo/cozodb-production/

# Clone with descriptive name
git clone https://github.com/org/repo.git org-repo-purpose

# Immediately create manifest
cat > org-repo-purpose/MANIFEST.md << 'EOF'
[Fill in template]
EOF
```

---

## Pattern Extraction Framework

### Target Pattern Types

For each pattern found, document using this structure:

#### Pattern Template

```markdown
### Pattern: [Name]
**Category**: [Transitive Closure / N-Hop / Shortest Path / etc.]
**Complexity**: [Simple / Medium / Complex]
**Source**: [repo/file]

#### Problem Solved
[What dependency tracking problem does this solve?]

#### CozoDB Datalog Query
```datalog
[Complete query that can be copy-pasted]
```

#### Explanation
[Line-by-line breakdown of how it works]

#### Performance Characteristics
- **Time Complexity**: [O(?) if known]
- **Space Complexity**: [memory usage notes]
- **Index Requirements**: [what indices help performance]
- **Benchmark**: [any available numbers]

#### Adaptations for Parseltongue
[How to adapt this to our CodeGraph + DependencyEdges schema]

**Before** (generic pattern):
```datalog
?[from, to] := *Edge[from, to]
```

**After** (our schema):
```datalog
?[from_key, to_key] :=
    *DependencyEdges{from_isgl1_key: from_key,
                     to_isgl1_key: to_key,
                     edge_type: "Calls"}
```

#### Limitations
[Any known issues, edge cases, or scenarios where this doesn't work]

#### Related Patterns
[Links to similar patterns or variations]
```

### Pattern Categories to Catalog

1. **Direct 1-Hop Queries**
   - Forward: "what does X call?"
   - Reverse: "who calls X?"
   - Filtered: "what tests call X?"

2. **Multi-Hop Traversal**
   - Fixed N-hop (exactly 2 hops away)
   - Bounded N-hop (up to N hops)
   - Unbounded (transitive closure)
   - With distance tracking

3. **Path Finding**
   - Any path (DFS-style)
   - Shortest path (BFS-style)
   - All paths
   - Cycle detection

4. **Aggregations**
   - Count descendants
   - Depth calculations
   - Fan-in/fan-out metrics

5. **Optimizations**
   - Index usage patterns
   - Query hints
   - Incremental updates
   - Caching strategies

---

## Success Metrics

### Quantitative Goals

- [ ] **Repositories**: 3-5 high-quality repos cloned
- [ ] **Patterns**: 10+ distinct query patterns extracted
- [ ] **Coverage**: All 6 critical pattern types found (1-hop, N-hop, transitive, shortest path, bounded, reverse)
- [ ] **Quality**: Average quality score ≥ 7/10 for cloned repos
- [ ] **Recency**: 50%+ of repos active in last 6 months

### Qualitative Goals

- [ ] **Confidence**: Can implement CozoDB-native dependency tracking
- [ ] **Validation**: Confirm/refine our D07 implementation approach
- [ ] **Novel Insights**: Discover optimization techniques not considered in D07
- [ ] **Risk Mitigation**: Identify performance gotchas or limitations
- [ ] **Alternatives**: If CozoDB patterns insufficient, identify fallback approaches

### Deliverables Checklist

Research Phase (General-Purpose Agent):
- [ ] `INVENTORY.md` - What we already have
- [ ] `OFFICIAL-PATTERNS.md` - CozoDB canonical examples
- [ ] `GITHUB-EVALUATION.md` - Scored production repos
- [ ] `ACADEMIC-SOURCES.md` - Research papers and code
- [ ] `CROSS-POLLINATION.md` - Portable patterns from related systems
- [ ] `COMMUNITY-PATTERNS.md` - Community-shared examples
- [ ] `RESEARCH-MANIFEST.md` - Master index of all findings

Analysis Phase (Explore Agent):
- [ ] Pattern extraction for each category
- [ ] Performance analysis
- [ ] Adaptation guidelines for Parseltongue
- [ ] Comparative evaluation

Synthesis Phase (Final Document):
- [ ] `D09-cozodb-hopping-patterns-compendium.md` - Comprehensive findings
- [ ] Implementation recommendations
- [ ] Revised roadmap (if needed based on findings)

---

## Risk Factors & Mitigation

### Risk 1: Insufficient Examples Found
**Scenario**: CozoDB usage is too niche, few public examples exist

**Mitigation**:
1. Broaden to Datalog examples (Souffle, Datomic) and translate
2. Consult CozoDB documentation directly for "official" patterns
3. Experiment with constructing patterns from primitives
4. Engage with CozoDB community directly (file issue asking for examples)

### Risk 2: Patterns Don't Scale
**Scenario**: Examples work for toy datasets but not 50k+ node graphs

**Mitigation**:
1. Look specifically for performance benchmarks
2. Extract optimization techniques (indices, query hints)
3. Plan to benchmark early with realistic data
4. Consider hybrid approach (Option B from D07) if pure CozoDB insufficient

### Risk 3: Incompatible CozoDB Versions
**Scenario**: Examples use older CozoDB syntax that's changed

**Mitigation**:
1. Note CozoDB version in manifest
2. Check official docs for migration guides
3. Test patterns against current CozoDB version
4. Document required adaptations

### Risk 4: Analysis Paralysis
**Scenario**: Too many repos to analyze, research takes too long

**Mitigation**:
1. Strict adherence to evaluation framework (reject low scores quickly)
2. Set time limits: 2 minutes quick scan, 10 minutes deep eval
3. Focus on top 3-5 repos, bookmark rest for later
4. Use Explore agent efficiently (targeted file analysis)

### Risk 5: Pattern Extraction Bottleneck
**Scenario**: Explore agent can't extract patterns effectively

**Mitigation**:
1. Provide clear file prioritization to agent
2. Focus on `.cozo` files and test files first
3. Use pattern templates for structured extraction
4. Manual review of critical patterns if needed

---

## Timeline & Resource Allocation

### Day 1: Discovery & Acquisition (6-8 hours)
- **Morning** (3h): Phase 1 (Inventory) + Phase 2 (Official sources)
- **Afternoon** (3-4h): Phase 3 (GitHub search) + Phase 4 (Academic)
- **Evening** (1-2h): Phase 5 (Cross-pollination) + Phase 6 (Community)

**Output**: All repos cloned, manifests created, research manifest compiled

### Day 2: Analysis & Extraction (6-8 hours)
- **Morning** (4h): Deploy Explore agent on top 3 priority repos
- **Afternoon** (2-3h): Pattern extraction and categorization
- **Evening** (1-2h): Performance analysis and adaptation notes

**Output**: Pattern compendium with extracted queries

### Day 3: Synthesis & Documentation (4-6 hours)
- **Morning** (3h): Write D09 comprehensive research document
- **Afternoon** (2h): Update D07 implementation roadmap if needed
- **Evening** (1h): Final review and commit

**Output**: Complete D09 document, revised implementation plan

---

## Next Steps After This Document

1. **Commit D08** (this methodology document)
2. **Execute Phase 1-6** via general-purpose agent
3. **Execute Analysis** via Explore agent
4. **Synthesize D09** final research document
5. **Validate D07 approach** or refine based on findings
6. **Begin implementation** with high confidence

---

## Appendix A: Query Pattern Reference

### Minimal Example: Transitive Closure in Datalog

```datalog
# Base case: direct edges
?[ancestor, descendant] := *Edge[ancestor, descendant]

# Recursive case: transitive edges
?[ancestor, descendant] :=
    *Edge[ancestor, intermediate],
    ?[intermediate, descendant]
```

**Explanation**:
- Line 2: Start with direct edges from Edge table
- Line 5-7: For each edge A→B, and each path B→C, create path A→C
- Datalog engine handles fixpoint iteration automatically

### Adaptation to Our Schema

```datalog
# Direct function calls
?[caller, callee] :=
    *DependencyEdges{
        from_isgl1_key: caller,
        to_isgl1_key: callee,
        edge_type: "Calls"
    }

# Transitive calls (A calls B, B calls C → A transitively calls C)
?[caller, callee] :=
    *DependencyEdges{
        from_isgl1_key: caller,
        to_isgl1_key: intermediate,
        edge_type: "Calls"
    },
    ?[intermediate, callee]
```

---

## Appendix B: Evaluation Scorecard Template

```markdown
## Repository Evaluation: [Name]

**URL**: [url]
**Date Evaluated**: [date]

### Quick Facts
- **Stars**: X
- **Last Commit**: [date]
- **Primary Language**: [language]
- **CozoDB Version**: [version]

### Relevance Score: _/10
- [ ] Contains CozoDB Datalog queries (+2)
- [ ] Demonstrates recursive patterns (+3)
- [ ] Graph/dependency analysis focus (+3)
- [ ] Directly applicable to our problem (+2)

**Total**: _/10
**Notes**: [explain score]

### Quality Score: _/10
- [ ] Tests present (+2)
- [ ] Documentation (+2)
- [ ] Production usage (+2)
- [ ] Active maintenance (+2)
- [ ] Code clarity (+2)

**Total**: _/10
**Notes**: [explain score]

### Pattern Coverage
- [ ] 1-hop forward query
- [ ] 1-hop reverse query
- [ ] N-hop traversal
- [ ] Transitive closure
- [ ] Bounded recursion
- [ ] Shortest path
- [ ] Cycle detection
- [ ] Topological sort
- [ ] Aggregations
- [ ] Optimizations

**Count**: _/10

### Size Assessment
- **Total LOC**: [estimate]
- **CozoDB Query LOC**: [estimate]
- **Verdict**: Ideal / Good / Acceptable / Challenging

### Decision
**CLONE / SKIP / BOOKMARK**

**Reasoning**: [brief explanation]

**If CLONE, Priority**: HIGH / MEDIUM / LOW

**Key Files to Analyze**:
1. [file path] - [reason]
2. [file path] - [reason]
```

---

## Appendix C: Research Keywords & Terms

Use these terms when searching:

**Datalog Specific**:
- transitive closure
- recursive query
- stratified negation
- seminaive evaluation
- fixpoint iteration
- magic sets
- subsumption
- tabling / memoization

**Graph Algorithms**:
- graph traversal
- reachability analysis
- shortest path
- all-pairs shortest path
- strongly connected components
- topological sort
- cycle detection
- graph diameter

**Dependency Analysis**:
- call graph
- dependency graph
- control flow graph
- data flow analysis
- def-use chains
- reaching definitions
- dominator tree
- control dependencies

**Performance**:
- query optimization
- index selection
- join ordering
- predicate pushdown
- materialized views
- incremental maintenance

**Code Analysis**:
- static analysis
- program analysis
- pointer analysis
- alias analysis
- taint analysis
- security analysis

---

**End of D08 Research Methodology**

*This document serves as the definitive protocol for CozoDB hopping pattern research. All research activities should follow this framework to ensure systematic, rigorous, and comprehensive discovery of applicable patterns.*
