---
inclusion: always
---

# Parseltongue Requirements Focus - Steering Rules

## Core Constraints (Non-Negotiable)

### 1. Rust-Only Focus
- **MUST**: All features, patterns, and optimizations are Rust-specific
- **MUST**: Use Rust-native tools: `syn` crate, `notify` crate, `sqlx`, `tokio`
- **MUST**: Ignore non-Rust files (.js, .py, .java, etc.) - focus exclusively on .rs files
- **MUST**: Leverage Rust's type system, ownership model, and trait system for architectural intelligence

### 2. High-Speed Updates (Critical Performance Constraint)
- **MUST**: Maintain <12ms total pipeline latency from .rs file save to query readiness
- **MUST**: Achieve sub-millisecond query response times (<500μs for simple traversals)
- **MUST**: Use deterministic SigHash-based O(1) lookups throughout the system
- **MUST**: Optimize for real-time development workflow without interruption

### 3. LLM-Terminal Integration
- **MUST**: Provide deterministic, zero-hallucination architectural context for AI tools
- **MUST**: Support variety of query types accessible from terminal during active development
- **MUST**: Generate compressed ISG context (95%+ token reduction) for LLM consumption
- **MUST**: Enable instant architectural queries during coding sessions

## Requirements Refinement Guidelines

### What to Include
- **Rust-specific patterns**: newtype patterns, trait objects, async/await, ownership transfer
- **Performance specifications**: Exact latency targets, memory footprint limits, throughput requirements
- **Architectural intelligence**: Graph relationships, dependency analysis, constraint validation
- **Developer workflow integration**: CLI commands, real-time monitoring, incremental updates
- **Enterprise scalability**: Support for 100K-500K LOC Rust codebases

### What to Exclude
- **Multi-language support**: No JavaScript, Python, Java, etc. - Rust only
- **Complex coordination**: No Redis, message queues, microservice orchestration
- **Probabilistic methods**: No vector embeddings, fuzzy matching, or ML-based analysis
- **Non-architectural features**: No code formatting, linting, or style checking

### Requirements Quality Standards
- **Specific and Measurable**: Include exact performance targets (ms, μs, MB)
- **Rust-Focused**: Reference specific Rust crates, patterns, and idioms
- **EARS Format**: Use "WHEN...THEN...SHALL" structure for acceptance criteria
- **Testable**: Each criterion must be objectively verifiable
- **Complete**: Cover all aspects of the user story without gaps

## Decision Framework

### When Adding New Requirements
1. **Does it align with Rust-only focus?** If not, exclude
2. **Does it support <12ms update latency?** If not, optimize or exclude
3. **Does it enhance LLM-terminal integration?** If not, deprioritize
4. **Is it architecturally focused?** Prefer structural over syntactic features
5. **Is it deterministic?** Prefer exact over probabilistic approaches

### When Refining Existing Requirements
1. **Add Rust-specific technical details** (crate names, data structures, patterns)
2. **Include precise performance targets** (latency, memory, throughput)
3. **Specify error handling patterns** (Result<T,E>, graceful degradation)
4. **Add constraint validation** (architectural rules, pattern enforcement)
5. **Enhance LLM integration** (context compression, structured output)

## Current Focus Areas

### Priority 1: Core Performance Requirements
- File system monitoring with `notify` crate
- SigHash-based graph operations with deterministic performance
- SQLite integration with WAL mode and optimized indexes
- Memory management with Arc<RwLock<T>> and DashMap patterns

### Priority 2: Rust Pattern Recognition
- Complex generic constraints and where clauses
- Trait object patterns and dynamic dispatch
- Async/await patterns and Future types
- Ownership and borrowing pattern validation

### Priority 3: Architectural Intelligence
- Graph schema with 7 node types and 9 relationship types
- Advanced queries: blast-radius, cycle detection, implementation discovery
- Constraint validation and architectural rule enforcement
- Multi-source merging with deterministic conflict resolution

### Priority 4: Developer Experience
- CLI interface with comprehensive command set
- Code dump processing for unfamiliar codebases
- Error handling and system resilience
- Real-time feedback and progress reporting

## Success Metrics

### Technical Metrics
- **Update Latency**: <12ms from file save to query readiness
- **Query Performance**: <500μs for simple graph traversals
- **Memory Efficiency**: <25MB for 100K LOC Rust codebase
- **Compression Ratio**: >95% from raw code to architectural essence

### Quality Metrics
- **Pattern Coverage**: 85-90% with pure `syn` parsing, 95-98% with selective compiler assistance
- **Accuracy**: Zero false positives in dependency analysis
- **Reliability**: Graceful handling of parsing errors and system failures
- **Scalability**: Support for enterprise codebases up to 500K LOC

## Anti-Patterns to Avoid

### Technical Anti-Patterns
- **Blocking operations** in async contexts
- **Probabilistic analysis** instead of deterministic graph traversal
- **Multi-language complexity** that dilutes Rust focus
- **Performance compromises** that break <12ms update target

### Requirements Anti-Patterns
- **Vague acceptance criteria** without measurable targets
- **Generic patterns** that could apply to any language
- **Missing error handling** specifications
- **Incomplete performance requirements**

This steering document ensures all requirements refinement stays focused on our core mission: building the world's fastest, most accurate Rust-specific architectural intelligence system.