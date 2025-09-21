# Requirements Document

## Introduction

Parseltongue AIM Daemon is a **Rust-only** development tool that transforms code analysis from probabilistic text searches to deterministic, graph-based architectural navigation. The system creates Interface Signature Graphs (ISG) exclusively from Rust codebases, enabling sub-millisecond queries, real-time architectural awareness, and zero-hallucination LLM context generation.

**MVP v1.0 Focus**: Essential functionality to start using the daemon immediately for both code dumps and live codebases.

**Core MVP Constraints:**
- **Rust-Only Focus**: Exclusively designed for Rust codebases using `syn` crate for high-fidelity parsing
- **High-Speed Updates**: Interface graph updates must complete in <12ms for real-time development workflow
- **LLM-Terminal Integration**: Optimized for LLMs querying from terminal during active development sessions
- **Immediate Usability**: Can be used productively from day one with minimal configuration

## MVP v1.0 Requirements

### REQ-MVP-001.0: Code Dump Ingestion and Processing

**User Story:** As a Rust developer analyzing unfamiliar Rust codebases, I want to ingest Rust code dumps and extract architectural intelligence deterministically, so that I can understand complex Rust systems in seconds rather than hours.

#### Acceptance Criteria

1. WHEN I run `parseltongue ingest <file>` THEN the system SHALL parse separated dump format with FILE: markers and extract all Rust interface signatures using `syn` crate
2. WHEN processing a 2.1MB Rust code dump THEN the system SHALL complete ISG construction in less than 5 seconds
3. WHEN building the Interface Signature Graph THEN the system SHALL create nodes for Rust Function, Struct, and Trait entities with basic relationships
4. WHEN ISG construction completes THEN the system SHALL display basic status: "✓ Processed X files → Y nodes"
5. WHEN encountering parse errors THEN the system SHALL log the error and continue processing other files
6. WHEN ingestion completes THEN the system SHALL be ready for immediate queries

### REQ-MVP-002.0: Live Codebase Monitoring

**User Story:** As a Rust developer working on live Rust codebases, I want real-time architectural monitoring so that I can query the daemon immediately after making file changes.

#### Acceptance Criteria

1. WHEN I run `parseltongue daemon --watch <directory>` THEN the system SHALL start monitoring all .rs files recursively using the `notify` crate
2. WHEN I save a Rust file THEN the system SHALL detect the change and update the ISG within 12ms (CRITICAL PERFORMANCE CONSTRAINT)
3. WHEN the daemon is running THEN I can query it immediately with `parseltongue query <type> <target>` and get current results
4. WHEN I stop the daemon with Ctrl+C THEN it SHALL shut down gracefully and save state
5. WHEN monitoring starts THEN the system SHALL display "🐍 Watching <directory> for .rs files"
6. WHEN files are updated THEN the system SHALL show basic status: "✓ Updated <file> → <node_count> nodes"

### REQ-MVP-003.0: Essential Graph Queries

**User Story:** As a Rust developer needing dependency analysis, I want basic graph-based queries that return factual results, so that I can make confident architectural decisions.

#### Acceptance Criteria

1. WHEN I run `parseltongue query what-implements <trait>` THEN the system SHALL return all implementing structs/functions in sub-millisecond time
2. WHEN I run `parseltongue query blast-radius <entity>` THEN the system SHALL show all functions and modules affected by changes to that entity
3. WHEN I run `parseltongue query find-cycles` THEN the system SHALL detect and report circular dependencies in the codebase
4. WHEN executing any query THEN the system SHALL respond in less than 1ms for simple graph traversals
5. WHEN returning query results THEN the system SHALL provide clear, human-readable output by default
6. WHEN I add `--format json` THEN the system SHALL return machine-readable JSON for LLM consumption

### REQ-MVP-004.0: LLM Context Generation

**User Story:** As a developer using LLMs for code assistance, I want compressed architectural context that eliminates hallucination, so that AI tools receive factual architectural information.

#### Acceptance Criteria

1. WHEN I run `parseltongue generate-context <entity>` THEN the system SHALL extract relevant ISG slice for that entity and its immediate dependencies
2. WHEN generating context THEN the system SHALL include function signatures, trait constraints, and basic dependency relationships
3. WHEN formatting for LLMs THEN the system SHALL structure output with clear sections for signatures, dependencies, and relationships
4. WHEN providing context THEN the system SHALL include upstream callers and downstream dependencies within 2 hops
5. WHEN I add `--format json` THEN the system SHALL return structured JSON suitable for LLM consumption
6. WHEN context is generated THEN the system SHALL ensure deterministic, reproducible results for the same entity

### REQ-MVP-005.0: Essential CLI Interface

**User Story:** As a Rust developer working from terminal, I want a simple CLI interface for essential operations, so that I can start using the daemon immediately.

#### Acceptance Criteria

1. WHEN I run `parseltongue ingest <file>` THEN the system SHALL process code dumps and build the ISG
2. WHEN I run `parseltongue daemon --watch <directory>` THEN the system SHALL start monitoring live files
3. WHEN I run `parseltongue query <type> <target>` THEN the system SHALL support what-implements, blast-radius, and find-cycles queries
4. WHEN I run `parseltongue generate-context <entity>` THEN the system SHALL output LLM-ready context
5. WHEN any command fails THEN the system SHALL show clear error message and suggested fix
6. WHEN I run `parseltongue --help` THEN the system SHALL show usage for all commands

### REQ-MVP-006.0: In-Memory Performance and Persistence

**User Story:** As a developer working with typical Rust projects, I want the daemon to handle common codebases with sub-millisecond query performance using in-memory architecture, so that it meets the <12ms update and <1ms query constraints.

#### Acceptance Criteria

1. WHEN processing up to 100K lines of Rust code THEN the system SHALL maintain memory usage under 25MB using OptimizedISG architecture
2. WHEN handling queries THEN the system SHALL maintain sub-millisecond response times using Arc<RwLock<ISGState>> for thread-safe access
3. WHEN persisting data THEN the system SHALL use high-performance, asynchronous snapshotting (rkyv serialization) of the in-memory graph
4. WHEN the daemon restarts THEN the system SHALL reload the ISG from snapshot within 500ms
5. WHEN memory usage grows THEN the system SHALL use efficient data structures (FxHashMap, Arc<str> interning)
6. WHEN concurrent access occurs THEN the system SHALL use single RwLock for atomic consistency

### REQ-MVP-007.0: Essential Error Handling

**User Story:** As a Rust developer using the daemon daily, I want clear error messages and graceful failure handling, so that temporary issues don't disrupt my development workflow.

#### Acceptance Criteria

1. WHEN Rust file parsing fails THEN the system SHALL log the error details and continue processing other files
2. WHEN file monitoring fails THEN the system SHALL attempt to restart monitoring after a brief delay
3. WHEN storage operations fail THEN the system SHALL retry up to 3 times before reporting failure
4. WHEN any command fails THEN the system SHALL show clear error message with suggested fix
5. WHEN unrecoverable errors occur THEN the system SHALL shut down gracefully with diagnostic information
6. WHEN the daemon crashes THEN it SHALL be able to restart and reload state from storage

## MVP v1.0 Scope Summary

The above 7 requirements represent the **complete MVP v1.0 scope** for Parseltongue AIM Daemon. This focused scope ensures you can start using the daemon immediately for both code dumps and live codebases.

### MVP v1.0 Success Criteria (Revised - Technically Aligned)

1. **Code Dump Analysis**: Process separated dump format and build ISG in <5 seconds
2. **Live File Monitoring**: Watch .rs files and update ISG in <12ms using OptimizedISG
3. **Essential Queries**: Support what-implements, blast-radius, find-cycles in <1ms
4. **LLM Context**: Generate compressed architectural context via CLI
5. **Simple CLI**: 4 core commands with --format json support
6. **In-Memory Performance**: OptimizedISG with rkyv snapshotting, <25MB for 100K LOC
7. **Error Handling**: Clear messages and graceful failure recovery

**Core Validation**: Proves deterministic, sub-millisecond architectural intelligence on live Rust codebases using structural ISG analysis.

# Marketing due-diligence

The README.md should be written using the minto pyramid principle, this means that essence is at the top, and details are revealed layer by layer