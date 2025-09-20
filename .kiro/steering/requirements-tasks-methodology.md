# Requirements-Tasks Methodology - Steering Rules

## Purpose

This document provides steering guidance for managing the requirements-tasks.md file, which serves as the central coordination hub for systematic document analysis, requirements refinement, and MVP development tracking.

## Core Methodology

### Document Analysis Protocol

#### Phase 1: Systematic Reference Analysis
- **Objective**: Extract MVP-relevant concepts from ALL reference documents
- **Strategy**: Chunked reading (1000 lines at a time) with MVP filtering
- **Focus**: Only concepts supporting Rust-only, <12ms, LLM-terminal constraints
- **Output**: Architecture concepts documented in architecture-backlog.md

#### Analysis Workflow
1. **File Discovery**: `wc -l filename` to get total lines
2. **Task Breakdown**: Create subtasks for every 1000 lines (MAXIMUM chunk size)
3. **Chunked Reading**: Read exactly 1000 lines at a time for comprehensive coverage
4. **MVP Filtering**: Extract only concepts supporting core constraints
5. **Backlog Management**: Move advanced concepts to appropriate version backlogs
6. **Progress Tracking**: Update requirements-tasks.md with findings after each chunk

#### Document Categories
- **_refDocs**: Technical architecture, implementation details, performance specs
- **_refIdioms**: Rust patterns, TDD methodologies, advanced architectural concepts
- **Priority Files**: Focus on high-impact documents first (Sig-Graph-Ideas.md, etc.)

### Task Structure Requirements

#### Task Hierarchy
- **Phase-based organization**: Requirements Analysis → Design → Implementation Planning
- **Numbered subtasks**: Use decimal notation (1.1, 1.2, 2.1) for clear hierarchy
- **Completion tracking**: ✅ COMPLETED, 🟡 IN PROGRESS, 🔴 NOT STARTED

#### Task Quality Standards
- **Actionable**: Each task must be executable with clear deliverables
- **Measurable**: Include specific completion criteria and success metrics
- **Traceable**: Reference specific requirements and architectural decisions
- **Incremental**: Build on previous tasks without big complexity jumps

### MVP Discipline Framework

#### What MVP 1.0 IS
- ✅ **Rust-only focus**: Parse .rs files exclusively with `syn` crate
- ✅ **High-speed updates**: <12ms from file save to query readiness
- ✅ **LLM-terminal integration**: Deterministic architectural context generation
- ✅ **SQLite storage**: Proven, simple, meets performance requirements
- ✅ **Core queries**: who-implements, blast-radius, find-cycles, generate-context
- ✅ **Essential patterns**: 80% coverage with pure `syn` parsing

#### What MVP 1.0 IS NOT
- ❌ **Multi-language support**: No JavaScript, Python, Java, etc.
- ❌ **Advanced graph databases**: No MemGraph, SurrealDB, TigerGraph
- ❌ **Complex coordination**: No Redis, message queues, microservices
- ❌ **ML/AI features**: No vector embeddings, fuzzy matching, probabilistic analysis
- ❌ **Enterprise features**: No distributed analysis, advanced security, complex workflows

#### Backlog Decision Framework
**Move to backlog if**:
1. Doesn't directly support Rust-only constraint
2. Would compromise <12ms update performance
3. Adds complexity without clear LLM-terminal value
4. Requires technologies beyond SQLite + Rust ecosystem
5. Serves enterprise needs beyond MVP scope

### Progress Tracking Standards

#### Status Indicators
- **Requirements Document**: ✅ COMPLETE - 18 MVP requirements with EARS format
- **Document Analysis**: Track completion percentage (X/Y documents analyzed)
- **Quality Assurance**: Verification of requirements quality standards
- **Design Document**: Technical architecture and API specifications
- **Implementation Planning**: Detailed, actionable implementation tasks

#### Success Metrics Dashboard
- **Requirements Phase**: Requirements complete, analysis progress, QA status
- **Design Phase**: Architecture design, API specifications, technical decisions
- **Implementation Phase**: Task breakdown, development progress, testing status
- **Overall MVP Progress**: Percentage complete across all phases

### Risk Management Protocol

#### High-Risk Areas for MVP
1. **Performance Targets**: <12ms may be challenging with complex parsing
2. **Memory Management**: Keeping large graphs under memory limits
3. **Rust Complexity**: Handling edge cases in type system
4. **Concurrency**: Thread-safe updates without performance loss

#### Mitigation Strategies
1. **Performance**: Profile early, optimize incrementally, use benchmarks
2. **Memory**: Efficient data structures, lazy loading, compression
3. **Complexity**: 80/20 rule - handle common cases first, edge cases later
4. **Concurrency**: Simple patterns (Arc<RwLock<T>>), avoid complex coordination

### Session Continuity Requirements

#### Context Persistence
- **requirements-tasks.md**: Primary task tracking and progress coordination
- **SESSION_CONTEXT.md**: Universal session context and recovery information
- **architecture-backlog.md**: Technical concepts and implementation details
- **Git History**: Complete development timeline and decisions

#### Recovery Protocol
- **Session Start**: Verify live status and priority tasks from SESSION_CONTEXT.md
- **Progress Check**: Review requirements-tasks.md for current completion status
- **Architecture Reference**: Check architecture-backlog.md for technical decisions
- **Requirements Status**: Validate requirements.md completeness and quality

### Quality Gates


#### Completion Criteria
- **Document Analysis**: 100% of reference documents analyzed with MVP concepts extracted
- **Requirements Quality**: All 18 requirements meet EARS format and performance standards
- **Design Completeness**: Technical architecture, API specs, and data models defined
- **Implementation Readiness**: Detailed tasks with clear deliverables and success criteria

## Enforcement Rules

### MUST Requirements
- **Follow MVP constraints**: Every decision must align with Rust-only, <12ms, LLM-terminal
- **Maintain task hierarchy**: Use proper numbering and completion tracking
- **Document all findings**: Extract concepts to architecture-backlog.md
- **Track progress accurately**: Update completion percentages and status indicators

### SHOULD Guidelines
- **Prioritize high-impact documents**: Focus on architecture and implementation details first
- **Use aggressive backlog management**: Move non-MVP features to future versions immediately
- **Maintain session continuity**: Update SESSION_CONTEXT.md with major milestones
- **Validate against steering rules**: Check decisions against parseltongue-requirements-focus.md

### MUST NOT Anti-Patterns
- **Skip document analysis**: All reference documents must be systematically analyzed
- **Compromise MVP scope**: No feature creep beyond core constraints
- **Break task hierarchy**: Maintain clear phase-based progression
- **Ignore quality gates**: Each phase must meet completion criteria before proceeding

This methodology ensures systematic, disciplined progression through the requirements-tasks workflow while maintaining laser focus on MVP 1.0 delivery.