# Parseltongue LLM Development Guide

## Overview

Parseltongue is a Rust-based architectural intelligence tool that transforms code analysis from an analysis-first approach to a discovery-first approach. This guide provides LLMs with the essential context needed to understand, modify, and extend the Parseltongue codebase effectively.

## Core Architecture Principles

### 1. Discovery-First Philosophy
- **Primary Goal**: Eliminate the 5+ minute entity discovery bottleneck that prevents users from accessing Parseltongue's microsecond query capabilities
- **Success Metric**: Reduce entity discovery time from 5+ minutes to <30 seconds
- **Performance Contract**: Maintain existing <50μs query performance while adding <100ms discovery queries

### 2. Layered Rust Architecture (L1→L2→L3)
```
L1 Core: Ownership, RAII, Result/Option, newtype patterns
L2 Standard: Collections, Arc/Mutex, string interning  
L3 External: Tokio async, Serde serialization, SQLx databases
```

### 3. Test-Driven Development (TDD)
- **Pattern**: STUB → RED → GREEN → REFACTOR
- **Requirement**: Every performance claim must be test-validated
- **Coverage**: Unit tests, integration tests, property-based tests, performance contracts

## Codebase Structure

### Core Components

#### Discovery Layer (`src/discovery/`)
- **Purpose**: New discovery-first interface over existing ISG engine
- **Key Files**:
  - `engine.rs` - Core discovery trait definitions
  - `concurrent_discovery_engine.rs` - Thread-safe discovery implementation
  - `indexes.rs` - Efficient entity indexing structures
  - `blast_radius_analyzer.rs` - Human-readable impact analysis
  - `workflow_orchestrator.rs` - Complete user journey workflows

#### ISG Engine (`src/isg.rs`)
- **Purpose**: Existing microsecond-performance graph engine (DO NOT MODIFY)
- **Constraint**: Preserve all existing performance characteristics
- **Integration**: Discovery layer sits above ISG, translating discovery queries to ISG operations

#### CLI Interface (`src/cli.rs`, `src/main.rs`)
- **Purpose**: Command-line interface for discovery and analysis operations
- **Commands**: `list-entities`, `entities-in-file`, `where-defined`, `blast-radius`
- **Output**: Human-readable and JSON formats

### Key Data Structures

#### Entity Representation
```rust
#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub name: String,
    pub file_path: String,        // Embedded as attribute, not separate node
    pub entity_type: EntityType,  // Function, Struct, Trait, etc.
    pub line_number: Option<u32>,
}
```

#### Discovery Queries
```rust
pub enum DiscoveryQuery {
    ListAll { entity_type: Option<EntityType>, max_results: usize },
    EntitiesInFile { file_path: String },
    WhereDefinedExact { entity_name: String },
}
```

## Current Issues and Warnings

### Compilation Warnings (25 total)
Based on latest build analysis, the following issues need attention:

#### Unused Imports (8 warnings)
- `src/workspace_cli.rs:4` - unused `DateTime` import
- `src/discovery/file_navigation_tests.rs:13` - unused `FileId`, `FileInterner`
- `src/discovery/workflow_orchestrator.rs:9` - unused `Instant`
- `src/discovery/output_formatter.rs:10` - unused `Serialize`, `Deserialize`

#### Unused Variables (9 warnings)
- `src/discovery/concurrent_discovery_engine.rs:173` - unused error variable `e`
- `src/discovery/concrete_workflow_orchestrator.rs` - multiple unused parameters in stub implementations

#### Dead Code (3 warnings)
- `src/discovery/performance_metrics.rs` - unused `name` fields in Counter and Histogram structs
- `src/relationship_accuracy_tests.rs` - unused helper functions

### Performance Timing Display Issues
Current issue: Times approximating to 0 seconds are confusing. Need to display milliseconds when appropriate.

**Current**: "Time: 0.02s" 
**Improved**: "Time: 20ms" or "Time: 0.02s (20ms)"

## Development Guidelines for LLMs

### 1. When Modifying Discovery Layer
- **DO**: Add new discovery capabilities, improve query performance, enhance user experience
- **DON'T**: Modify existing ISG engine, break performance contracts, change core data structures

### 2. When Adding New Features
- **Pattern**: Write tests first (TDD), implement minimal viable solution, optimize if needed
- **Performance**: All new queries must complete in <100ms for interactive responsiveness
- **Error Handling**: Use `thiserror` for library errors, `anyhow` for application context

### 3. When Fixing Warnings
- **Unused Imports**: Remove or conditionally compile with `#[cfg(test)]`
- **Unused Variables**: Prefix with underscore `_variable` if intentional
- **Dead Code**: Remove if truly unused, or add `#[allow(dead_code)]` if needed for future use

### 4. When Improving Performance Display
- **Timing Logic**: If duration < 1 second, display in milliseconds
- **Format**: "Time: 1.23s (1230ms)" for clarity
- **Precision**: Use appropriate precision (ms for <1s, s for >=1s)

## Testing Strategy

### Test Categories
1. **Unit Tests**: Individual function correctness
2. **Integration Tests**: Component interaction validation  
3. **Performance Tests**: Contract validation (<50μs existing, <100ms discovery)
4. **Property Tests**: Invariant validation across input space

### Test Locations
- `tests/` - Integration and end-to-end tests
- `src/*/mod.rs` - Unit tests in `#[cfg(test)]` modules
- `examples/` - Demonstration and validation examples

## Common Patterns

### Error Handling
```rust
// Library errors (structured)
#[derive(Error, Debug)]
pub enum DiscoveryError {
    #[error("Entity not found: {name}")]
    EntityNotFound { name: String },
    // ... other variants
}

// Application errors (contextual)
pub fn process_query(query: &str) -> anyhow::Result<QueryResult> {
    parse_query(query)
        .with_context(|| format!("Failed to parse query: {}", query))?;
    // ...
}
```

### Performance Contracts
```rust
#[tokio::test]
async fn test_discovery_performance_contract() {
    let start = Instant::now();
    let result = engine.list_entities(None, 100).await?;
    let elapsed = start.elapsed();
    
    assert!(elapsed < Duration::from_millis(100), 
            "Discovery took {:?}, expected <100ms", elapsed);
}
```

### String Interning for Memory Efficiency
```rust
#[derive(Debug, Clone, Copy)]
pub struct FileId(u32);

pub struct FileInterner {
    paths: Vec<String>,
    path_to_id: HashMap<String, FileId>,
}
```

## Workflow Integration

### Complete User Journeys
The system supports complete workflows, not just individual commands:

1. **Onboarding**: `pt onboard` - Complete codebase understanding in <15 minutes
2. **Feature Planning**: `pt feature-start` - Impact analysis and scope guidance in <5 minutes  
3. **Debugging**: `pt debug` - Call traces and usage sites in <3 minutes
4. **Refactoring**: `pt refactor-check` - Risk assessment and safety guidance

### Output Formats
- **Human**: Terminal-friendly formatted output
- **JSON**: Machine-readable for tooling integration
- **PR Summary**: Markdown suitable for pull request descriptions
- **CI/CD**: Structured output for continuous integration

## Performance Characteristics

### Existing (Must Preserve)
- Query execution: <50μs for simple queries
- Memory usage: Current baseline (no more than 20% increase)
- Ingestion: <5 seconds for large codebases

### New Discovery Features
- Entity listing: <100ms for interactive responsiveness
- File-based queries: <100ms with O(n) file filtering
- Blast radius: <500ms with human-readable output

## Binary Naming Convention
Parseltongue binaries include timestamp suffixes for version tracking:
- Format: `parseltongue_YYYYMMDDHHMMSS`
- Current: `parseltongue_20250924231324`
- Purpose: Always know which version is being used

## Key Success Metrics

### North Star Metric
**New user time-to-first-successful-analysis**: <10 minutes from installation to completing core workflow

### Supporting Metrics
1. **Entity discovery time**: <30 seconds (from current 5+ minutes)
2. **Query success rate**: 90%+ (from current ~30% for unknown entities)
3. **Performance preservation**: <50μs for existing queries (no regression)

## Next Steps for LLMs

When working on Parseltongue:

1. **Start with Tests**: Write failing tests that specify the desired behavior
2. **Preserve Performance**: Never break existing <50μs query performance
3. **Focus on Discovery**: Prioritize entity discoverability over advanced features
4. **Validate Claims**: Every performance assertion needs automated validation
5. **Think Workflows**: Consider complete user journeys, not just individual commands

This guide provides the essential context for understanding and contributing to Parseltongue's mission of transforming architectural intelligence through discovery-first design.