# Requirements Document

## Introduction
Parseltongue v2 represents a strategic evolution from a **technical masterpiece with a user experience gap** to a **discovery-first architectural intelligence tool**. Based on comprehensive validation with real-world codebases (Iggy message broker: 983 files, 2727 nodes, 8111 edges; Axum framework: 295 files, 1,147 nodes, 2,090 edges), we have identified that the primary constraint on adoption is not technical capability but **entity discoverability**.

The core insight: Users spend 5+ minutes discovering the right entity name to achieve a 1-microsecond query. This represents a 300,000:1 ratio of discovery time to execution time - a fundamental user experience inversion that prevents access to the tool's exceptional analytical capabilities.

**v2 Mission**: Transform Parseltongue from an **analysis-first tool that happens to need better discovery** into a **discovery-first tool that happens to have excellent analysis capabilities**.

### Layer 1: Surface Problem (What Users Report)
- "Don't know what entities exist in this codebase"
- "Can't find entities by file location"
- "Need to browse and explore before I can analyze"

### Layer 2: Systemic Problem (Root Cause)
The current architecture optimizes for **query execution speed** at the expense of **query formulation ease**. This creates a classic product-market fit paradox: exceptional technical capability with discovery friction that prevents users from accessing that capability.

### Layer 3: Strategic Problem (Business Impact)
Parseltongue v1 is a **power tool for experts** when it needs to be a **discovery tool for practitioners**. The gap between "tool capability" and "user capability" is the primary constraint on adoption and business value realization.

## Requirements: The 80/20 Focus

### P0 Requirements: The Core Constraint Removers
#### Requirement 1: Entity Discovery Infrastructure
**The One Thing**: Eliminate the entity name discovery bottleneck that prevents users from accessing Parseltongue's existing excellence.

**User Story:** As a Rust developer exploring an unfamiliar codebase, I want to see what entities exist and where they are located, so that I can formulate effective queries without needing to guess entity names.

**Why This Matters**: This is the **primary constraint on adoption**. Users spend 5+ minutes discovering entity names to achieve 1-microsecond queries. Solving this unlocks access to all existing capabilities.

**Success Criteria**:
- Entity discovery time: <30 seconds (from current 5+ minutes)
- Query success rate: 90%+ (from current ~30% for unknown entities)
- Interactive responsiveness: <100ms for entity listing queries

**Implementation Requirements**:
1. WHEN I run `parseltongue list-entities` THEN the system SHALL return all entities with their file locations and types
2. WHEN I run `parseltongue list-entities --type functions` THEN the system SHALL return only function entities with file locations
3. WHEN I run `parseltongue entities-in-file src/server/handlers.rs` THEN the system SHALL return all entities defined in that specific file
4. WHEN I run `parseltongue where-defined EntityName` THEN the system SHALL return the exact file location for immediate navigation
5. WHEN discovery queries return results THEN they SHALL be organized by entity type for easy browsing

#### Requirement 2: File Location as Entity Attributes
**The One Thing**: Embed file location directly in entity nodes as attributes, avoiding separate file nodes that would degrade performance.

**User Story:** As a developer analyzing code relationships, I want every entity to include its file location as an immediate attribute, so that I can navigate from architectural insights to code without additional graph traversals.

**Why This Matters**: Files are containers, not entities. The ISG should model semantic relationships with files as metadata. This preserves O(1) performance while enabling file-based navigation.

**Success Criteria**:
- O(1) file location access for any entity
- No performance degradation from file location queries
- Simple, direct implementation without graph complexity

**Implementation Requirements**:
1. WHEN the system creates entity nodes THEN it SHALL embed file_path as a required attribute (not a separate node)
2. WHEN I query `parseltongue where-defined EntityName` THEN the system SHALL return file_path directly from the entity node in O(1) time
3. WHEN I query `parseltongue entities-in-file src/path/file.rs` THEN the system SHALL filter nodes by file_path attribute in O(n) time with direct access
4. WHEN storing file paths THEN the system SHALL use FileId (string interning) to minimize memory overhead
5. WHEN displaying entity information THEN file location SHALL be immediately available without additional graph traversal
#### Requirement 3: Readable Impact Analysis
**The One Thing**: Fix the hash-only output that breaks analytical flow and destroys user confidence.

**User Story:** As a developer planning code changes, I want blast-radius analysis to return human-readable entity names with file context, so that I can immediately understand and act on the impact assessment.

**Why This Matters**: This is a **high-impact, low-effort fix** that dramatically improves existing successful workflows.

**Success Criteria**:
- 100% readable output (zero hash values in user-facing results)
- Immediate actionability of impact analysis
- Risk categorization for decision support

**Implementation Requirements**:
1. WHEN I run `parseltongue blast-radius EntityName` THEN the system SHALL return readable entity names instead of hash values
2. WHEN displaying blast-radius results THEN the system SHALL group impacts by relationship type (CALLS, USES, IMPLEMENTS) with file locations
3. WHEN blast-radius analysis completes THEN the system SHALL provide risk categorization (Low: 1-5, Medium: 6-20, High: 21-50, Critical: 50+)
4. WHEN displaying impact results THEN the system SHALL separate test files from production code
5. WHEN I run `parseltongue blast-radius EntityName --summary` THEN the system SHALL provide executive summary suitable for team communication

### P0 Constraints: The Non-Negotiables

#### Constraint 1: Performance Preservation
**The One Thing**: Maintain the microsecond performance that is Parseltongue's key differentiator.

**Why This Matters**: **Microsecond performance is the core competitive advantage**. Any regression would undermine the tool's unique value proposition.

**Success Criteria**:
- Existing query performance: <50μs (no regression)
- Discovery query performance: <100ms (new capability)
- Memory usage increase: <20% (efficient implementation)

**Implementation Requirements**:
1. WHEN new discovery features are implemented THEN existing query performance SHALL remain under 50μs for simple queries
2. WHEN the system stores file location data THEN memory usage SHALL increase by no more than 20% through efficient data structures
3. WHEN I run entity listing queries THEN they SHALL complete in <100ms to maintain interactive responsiveness
4. WHEN handling large codebases (100K+ LOC) THEN the system SHALL maintain sub-second ingestion and sub-millisecond query performance
5. WHEN performance is measured THEN the system SHALL include built-in benchmarking to validate performance contracts

## Success Metrics: The One Metric That Matters

### North Star Metric
**New user time-to-first-successful-analysis**: <10 minutes from installation to completing core workflow

**Why This Matters**: This single metric captures whether we've solved the discovery problem. If users can't get value quickly, all other metrics are irrelevant.

### Supporting Metrics (Only These Three)
1. **Entity discovery time**: <30 seconds (from current 5+ minutes)
2. **Query success rate**: 90%+ (from current ~30% for unknown entities)  
3. **Performance preservation**: <50μs for existing queries (no regression)

### Anti-Metrics (Don't Optimize For)
- User retention (follows from solving the core problem)
- Feature adoption rates (irrelevant if core problem unsolved)
- Advanced capability metrics (already excellent)

## Scope Control: The Discipline of No
### In Scope (P0 - Solves the Core Constraint)
- ✅ **Simple entity listing** - The primary adoption blocker solver
- ✅ **Entity type filtering** - Essential for focused discovery
- ✅ **File-centric navigation** - Essential for actionable results  
- ✅ **Readable impact analysis** - High-impact, low-effort UX fix
- ✅ **Performance preservation** - Core differentiator protection

### Deliberately Cut (Deferred to v3.0+)
- ❌ **Fuzzy search capabilities** - Complex solution to wrong problem
- ❌ **Pattern matching/glob support** - Premature optimization
- ❌ **Enhanced AI context generation** - Nice-to-have, not adoption-critical
- ❌ **Architectural pattern detection** - Advanced feature for power users
- ❌ **IDE integration foundation** - Major effort, uncertain ROI for v2
- ❌ **Production deployment features** - Premature optimization
- ❌ **Multi-language support** - Scope creep, focus on Rust excellence
- ❌ **Advanced macro expansion** - Technical complexity without clear user demand
- ❌ **Cross-crate workspace analysis** - Complex, uncertain value
- ❌ **Real-time collaboration** - Premature for current adoption stage

**Strategic Rationale**: v2 focuses exclusively on **removing the entity discovery friction** that prevents users from accessing Parseltongue's existing excellence. Everything else is distraction until this core problem is solved.

## The Implementation Philosophy

### The 80/20 Principle Applied
- 80% of user value comes from simple entity listing and browsing
- 20% of technical effort is required to solve it (no fuzzy search complexity)
- Focus ruthlessly on the 80% value, defer the 80% effort

### The Constraint Theory Application
- The primary constraint is entity discovery, not analysis capability
- Optimizing anything other than the primary constraint is waste
- Once discovery is solved, the next constraint will reveal itself

### The Jobs-to-be-Done Validation
Every requirement must answer: "Does this help users see what entities exist?" If no, it's out of scope for v2.

**The MVP Test**: Can a user run `parseltongue list-entities` and immediately see what's available to analyze? This single capability solves the core constraint.

**Core Validation**: Proves that discovery-first architectural intelligence can transform developer productivity by eliminating the entity name discovery bottleneck while maintaining the exceptional performance and accuracy that makes Parseltongue uniquely valuable.
