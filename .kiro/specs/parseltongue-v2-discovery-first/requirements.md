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

#### Core Discovery Primitives (Parseltongue Binary)
- ✅ **Simple entity listing** - The primary adoption blocker solver
- ✅ **Entity type filtering** - Essential for focused discovery
- ✅ **File-centric navigation** - Essential for actionable results  
- ✅ **Readable impact analysis** - High-impact, low-effort UX fix
- ✅ **JSON output support** - Enable tooling integration
- ✅ **Performance preservation** - Core differentiator protection

#### Workflow Orchestration Layer (Shell Script Toolkit)
- ✅ **Complete JTBD workflows** - The real user value delivery
- ✅ **Workspace state management** - Persistent analysis sessions
- ✅ **Repository-specific intelligence** - Route extraction, domain guidance
- ✅ **Output integration** - PR-ready summaries, visualization generation
- ✅ **Developer journey orchestration** - Onboard → Feature → Debug → Refactor → Review

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

## Jobs-to-be-Done: Complete User Journey Workflows

### The Shreyas Doshi JTBD Framework Applied

**Core Insight**: Users don't hire Parseltongue to run individual commands. They hire it to **complete developer workflows faster**. Individual commands are just building blocks - the real value is in **workflow orchestration**.

### Primary JTBD Workflows (The Real Constraints)

#### JTBD 1: "Help me understand this unfamiliar codebase quickly"
**Current Pain**: 30+ minutes jumping between files, guessing entry points, building mental model
**Desired Outcome**: 10-15 minutes to architectural understanding + key routes + visual map

**Workflow Requirements**:
1. WHEN I run `pt onboard` THEN the system SHALL execute: ingest → overview → routes → key contexts
2. WHEN onboarding completes THEN I SHALL have: architecture.html, route table, 3-5 key entity contexts
3. WHEN I need orientation THEN I SHALL get repo-specific guidance (not generic entity lists)

#### JTBD 2: "Help me start a feature without breaking things"
**Current Pain**: Unknown side effects, unclear where to plug in, fear of hidden dependencies
**Desired Outcome**: Scoped change list + impact assessment + test guidance in <5 minutes

**Workflow Requirements**:
1. WHEN I run `pt feature-start --entities "MessageService,RoomService"` THEN the system SHALL show impact scope
2. WHEN planning changes THEN I SHALL get blast radius + caller counts + file locations
3. WHEN impact is high THEN I SHALL get explicit test recommendations

#### JTBD 3: "Help me fix this bug without creating new ones"
**Current Pain**: "Who calls this?" and "Who depends on this?" takes 15+ minutes of grep/search
**Desired Outcome**: Call traces + usage sites + minimal change scope in <3 minutes

**Workflow Requirements**:
1. WHEN I run `pt debug --function "create_message_with_deduplication"` THEN I SHALL get caller trace
2. WHEN debugging THEN I SHALL get usage sites ranked by likelihood of relevance
3. WHEN I find the issue THEN I SHALL get minimal change scope guidance

#### JTBD 4: "Help me refactor safely with confidence"
**Current Pain**: Hidden dependencies, brittle interfaces, unclear blast radius
**Desired Outcome**: Quantified risk assessment + change checklist + reviewer guidance

**Workflow Requirements**:
1. WHEN I run `pt refactor-check --target "MessageService"` THEN I SHALL get risk categorization
2. WHEN risk is high THEN I SHALL get specific test requirements and reviewer suggestions
3. WHEN refactoring THEN I SHALL get before/after impact comparison

#### JTBD 5: "Help me review PRs with architectural context"
**Current Pain**: Vague PR descriptions, missing risk markers, unclear architectural impact
**Desired Outcome**: Impact summary + architectural context + review checklist

**Workflow Requirements**:
1. WHEN I run `pt pr-review --diff HEAD~1..HEAD` THEN I SHALL get impact analysis of changes
2. WHEN reviewing THEN I SHALL get architectural context for touched entities
3. WHEN impact is significant THEN I SHALL get specific review focus areas

### Workflow Integration Requirements

#### Requirement 4: Workflow Orchestration Layer
**The One Thing**: Provide complete user journey workflows, not just individual commands.

**User Story**: As a developer completing common tasks, I want orchestrated workflows that combine discovery + analysis + guidance, so that I can complete entire jobs-to-be-done without manual command chaining.

**Why This Matters**: Individual commands solve 20% of the problem. Workflow orchestration solves the remaining 80% - how commands work together to complete real developer tasks.

**Success Criteria**:
- Complete JTBD workflows: <10 minutes (from current 30+ minutes)
- Workflow success rate: 95%+ (users complete intended task)
- Context preservation: State maintained across workflow steps

**Implementation Requirements**:
1. WHEN I run `pt onboard` THEN the system SHALL execute discovery → overview → context generation as single workflow
2. WHEN workflows generate outputs THEN they SHALL be stored in persistent workspace for reuse
3. WHEN workflows complete THEN they SHALL provide next-step guidance for common follow-up tasks
4. WHEN I run workflow commands THEN they SHALL reuse existing analysis when possible (no redundant work)
5. WHEN workflows fail THEN they SHALL provide clear recovery steps and partial results

#### Requirement 5: Machine-Readable Output Integration
**The One Thing**: Enable downstream tooling integration and workflow automation.

**User Story**: As a developer using Parseltongue in larger workflows, I want machine-readable outputs that integrate with PRs, documentation, and other tools, so that architectural insights become part of my standard development process.

**Why This Matters**: Discovery is only valuable if it integrates into existing developer workflows. JSON output enables automation, PR integration, and tooling composition.

**Success Criteria**:
- All commands support `--json` output format
- JSON schemas are stable and documented
- Integration with common developer tools (git hooks, PR templates, CI/CD)

**Implementation Requirements**:
1. WHEN I run any discovery command with `--json` THEN the system SHALL output structured JSON
2. WHEN generating JSON THEN the system SHALL include metadata (timestamps, confidence scores, file paths)
3. WHEN integrating with tools THEN JSON SHALL include actionable file paths and line numbers
4. WHEN workflows complete THEN they SHALL generate summary JSON suitable for PR descriptions

#### Requirement 6: Workspace State Management
**The One Thing**: Maintain persistent analysis state across discovery sessions.

**User Story**: As a developer working on multiple tasks, I want my discovery work to persist across sessions, so that I don't repeat expensive analysis and can build on previous insights.

**Why This Matters**: Discovery is iterative. Users build understanding over time, not in single commands. State persistence enables progressive discovery and workflow continuity.

**Success Criteria**:
- Analysis persists across sessions in `./parseltongue_workspace/`
- Latest analysis automatically reused when valid
- Clear workspace management (timestamps, cleanup, versioning)

**Implementation Requirements**:
1. WHEN I run discovery commands THEN the system SHALL store results in `./parseltongue_workspace/analysis_TIMESTAMP/`
2. WHEN analysis exists THEN the system SHALL reuse it unless `--force-refresh` specified
3. WHEN workspace grows large THEN the system SHALL provide cleanup commands for old analysis
4. WHEN switching between projects THEN workspaces SHALL remain isolated and not interfere
5. WHEN analysis is stale THEN the system SHALL detect and offer refresh options

### Workflow Success Metrics

#### North Star Metric (Updated)
**Developer task completion time**: <10 minutes for common JTBD workflows (from current 30+ minutes)

#### Supporting Workflow Metrics
1. **Onboarding time**: <15 minutes from clone to architectural understanding
2. **Feature planning time**: <5 minutes from idea to scoped change list  
3. **Bug investigation time**: <3 minutes from symptom to root cause candidates
4. **Refactor confidence**: 95%+ of refactors complete without introducing bugs
5. **PR review efficiency**: <5 minutes to architectural impact assessment

### The Jobs-to-be-Done Validation (Updated)
Every requirement must answer: "Does this help users complete entire developer workflows faster?" Individual commands are building blocks - workflows are the real product.

**The Workflow Test**: Can a user run `pt onboard` and complete architectural understanding in <15 minutes? Can they run `pt feature-start` and get scoped change guidance in <5 minutes? These complete workflows solve the real constraints.

**Core Validation**: Proves that discovery-first architectural intelligence can transform developer productivity by eliminating the entity name discovery bottleneck while maintaining the exceptional performance and accuracy that makes Parseltongue uniquely valuable.
