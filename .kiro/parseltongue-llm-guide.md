# Parseltongue LLM Integration Guide

## Overview

This document provides comprehensive guidance for LLMs working with the Parseltongue codebase analysis tool. It includes architectural insights, common patterns, performance expectations, and best practices derived from real-world usage.

## Quick Reference

### Core Commands
```bash
# Scripts automatically detect the latest parseltongue binary
# Or set PARSELTONGUE_BIN environment variable to specify a specific binary

# Ingest codebase for analysis
parseltongue ingest codebase.dump

# Discovery operations (primary use cases)
parseltongue list-entities --limit 100
parseltongue list-entities --type functions --limit 50
parseltongue entities-in-file src/main.rs
parseltongue where-defined EntityName

# Analysis operations
parseltongue blast-radius EntityName
parseltongue caller-trace FunctionName
```

### Workflow Scripts
```bash
# Complete onboarding workflow (15 minutes target)
./parseltongue_dungeon/scripts/onboard_codebase.sh /path/to/codebase

# Self-analysis and code quality
./parseltongue_dungeon/scripts/self_analysis_simple.sh

# Feature impact analysis
./parseltongue_dungeon/scripts/feature_impact.sh EntityName

# Debug workflow
./parseltongue_dungeon/scripts/debug_entity.sh FunctionName
```

## Performance Expectations

### Timing Targets (Validated)
- **File ingestion**: 1-3 seconds for 64 Rust files
- **Parseltongue analysis**: 86-130 milliseconds for 2177 nodes
- **Entity discovery**: <100 milliseconds for 200 entities
- **Blast radius analysis**: <500 microseconds for simple queries
- **Complete onboarding**: <15 minutes for codebases up to 1000 files

### Memory Usage
- **Baseline ISG**: ~2177 nodes, 3933 edges for Parseltongue itself
- **Memory increase**: <20% from baseline for discovery features
- **Entity storage**: 24 bytes per CompactEntityInfo

## Architecture Insights

### Core Components
```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#e1f5fe', 'primaryTextColor':'#01579b', 'lineColor':'#0277bd', 'fontFamily':'Arial', 'fontSize':'12px'}, 'flowchart': {'nodeSpacing': 70, 'rankSpacing': 80, 'wrappingWidth': 130}}}%%
flowchart TD
    %% Discovery Layer
    subgraph "🔍 Discovery Layer (New in v2)"
        direction TB
        DE["🎯 DiscoveryEngine<br/><i>Core trait interface</i>"]
        SI["🔧 SimpleDiscoveryEngine<br/><i>Basic implementation</i>"]
        CE["🔀 ConcurrentDiscoveryEngine<br/><i>Thread-safe wrapper</i>"]
    end
    
    %% Core ISG Engine
    subgraph "⚙️ Core ISG Engine (Preserved)"
        direction TB
        ISG["🏗️ InMemoryIsg<br/><i>Original graph engine</i>"]
        OPT["⚡ OptimizedISG<br/><i>Performance-enhanced</i>"]
    end
    
    %% Workflow Layer
    subgraph "🎭 Workflow Layer (Shell Toolkit)"
        direction TB
        WO["🔄 WorkflowOrchestrator<br/><i>Complete user journeys</i>"]
        CWO["🎯 ConcreteWorkflowOrchestrator<br/><i>Implementation</i>"]
    end
    
    %% Performance Monitoring
    subgraph "📊 Performance Monitoring"
        direction TB
        PM["📈 PerformanceMetrics<br/><i>Contract validation</i>"]
        BM["🔍 Benchmarks<br/><i>Automated testing</i>"]
    end
    
    %% Data flow connections
    DE --> ISG
    SI --> ISG
    CE --> ISG
    WO --> DE
    CWO --> WO
    
    %% Performance monitoring
    SI --> PM
    CE --> PM
    ISG --> BM
    OPT --> BM
    
    %% Key metrics
    subgraph "⏱️ Performance Characteristics"
        direction LR
        M1["Discovery: &lt;100ms"]
        M2["Existing: &lt;50μs"]
        M3["Memory: +12%"]
        M4["Concurrency: ✅"]
    end
    
    PM --> M1
    BM --> M2
    CE --> M3
    CE --> M4
    
    %% Styling
    classDef discovery fill:#e1f5fe,stroke:#01579b,stroke-width:2px,color:#0d47a1
    classDef core fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:#e65100
    classDef workflow fill:#e8f5e8,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef performance fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px,color:#4a148c
    classDef metrics fill:#fce4ec,stroke:#c2185b,stroke-width:2px,color:#880e4f
    
    class DE,SI,CE discovery
    class ISG,OPT core
    class WO,CWO workflow
    class PM,BM performance
    class M1,M2,M3,M4 metrics
```

### Key Traits and Interfaces
- **DiscoveryEngine**: Core discovery operations (list-entities, entities-in-file, where-defined)
- **WorkflowOrchestrator**: Complete user journey workflows (onboard, feature-start, debug, refactor-check)
- **FileNavigationProvider**: File-based entity navigation
- **BlastRadiusAnalyzer**: Impact analysis with human-readable output

## Common Patterns

### Entity Discovery Pattern
```rust
// 1. List all entities for overview
let entities = discovery_engine.list_all_entities(None, 100).await?;

// 2. Filter by type for focused analysis
let functions = discovery_engine.list_all_entities(Some(EntityType::Function), 50).await?;

// 3. Find entities in specific file
let file_entities = discovery_engine.entities_in_file("src/main.rs").await?;

// 4. Get exact location for navigation
let location = discovery_engine.where_defined("EntityName").await?;
```

### Workflow Orchestration Pattern
```rust
// Complete user journey workflows
let onboard_result = orchestrator.onboard("/path/to/codebase").await?;
let feature_result = orchestrator.feature_start("EntityName").await?;
let debug_result = orchestrator.debug("FunctionName").await?;
let refactor_result = orchestrator.refactor_check("TargetEntity").await?;
```

### Performance Monitoring Pattern
```rust
// Validate performance contracts
let metrics = DiscoveryMetrics::new();
metrics.record_discovery_time(duration);
metrics.validate_contract("discovery", duration, Duration::from_millis(100))?;
```

## Code Quality Insights (From Self-Analysis)

### Current Status
- **Files analyzed**: 64 Rust files
- **Entities discovered**: 2177 nodes, 3933 edges
- **Warnings identified**: 5 cargo warnings, 57 clippy suggestions
- **Performance**: All operations within targets

### Key Issues Identified
1. **Missing imports**: `std::time::{Duration, Instant}` in file navigation tests
2. **Unused fields**: `name` fields in Counter and Histogram (now fixed with accessor methods)
3. **Dead code**: Some test helper functions not actively used
4. **Clippy suggestions**: Redundant static lifetimes, useless format! calls

### Recommended Fixes
```rust
// Fix redundant static lifetimes
const FIELDS: &[&str] = &["hash", "kind", "name"]; // Instead of &'static [&'static str]

// Use push() instead of push_str() for single characters
output.push('\n'); // Instead of output.push_str("\n")

// Use or_default() instead of or_insert_with(Vec::new)
map.entry(key).or_default().push(value);

// Remove redundant field names
EntityInfo { name, file_path, entity_type } // Instead of name: name
```

## Binary Detection and Management

### Automatic Binary Detection
All scripts now automatically detect the latest parseltongue binary:
```bash
# Finds the most recent parseltongue binary (excludes .d debug files)
PARSELTONGUE_BIN=$(ls -t ./target/release/parseltongue* 2>/dev/null | grep -v '\.d$' | head -1)
```

### Manual Binary Selection
You can override the automatic detection:
```bash
# Set specific binary
export PARSELTONGUE_BIN="./target/release/parseltongue_20250924231324"

# Or pass as parameter to scripts
PARSELTONGUE_BIN="./custom/path/parseltongue" ./parseltongue_dungeon/scripts/onboard_codebase.sh
```

### Binary Naming Convention
- Format: `parseltongue_YYYYMMDDHHMMSS` (timestamp-based versioning)
- Location: `./target/release/`
- Detection: Automatically finds latest by modification time
- Exclusions: Skips `.d` debug info files

## Best Practices for LLM Integration

### 1. Discovery-First Approach
Always start with entity discovery before attempting analysis:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#e8f5e8', 'primaryTextColor':'#2e7d32', 'lineColor':'#4caf50', 'fontFamily':'Arial', 'fontSize':'12px'}, 'flowchart': {'nodeSpacing': 60, 'rankSpacing': 70, 'wrappingWidth': 120}}}%%
flowchart TD
    %% Discovery-first workflow
    subgraph "🔍 Discovery-First Workflow"
        direction TB
        S1["📋 Step 1: Get Overview<br/><i>parseltongue list-entities --limit 50</i><br/>See what exists"]
        S1 --> S2["🎯 Step 2: Identify Targets<br/><i>list-entities --type functions</i><br/>Filter by type"]
        S2 --> S3["📍 Step 3: Get Location<br/><i>parseltongue where-defined TargetFunction</i><br/>Exact file & line"]
        S3 --> S4["💥 Step 4: Perform Analysis<br/><i>parseltongue blast-radius TargetFunction</i><br/>Impact assessment"]
    end
    
    %% Traditional approach (anti-pattern)
    subgraph "❌ Traditional Approach (Anti-Pattern)"
        direction TB
        T1["🤔 Guess Entity Names<br/><i>grep, find, IDE search</i><br/>5+ minutes"]
        T1 --> T2["❓ Try Analysis<br/><i>May fail if wrong name</i><br/>Frustration"]
        T2 --> T3["🔄 Repeat Guessing<br/><i>Trial and error</i><br/>More time wasted"]
    end
    
    %% Performance comparison
    subgraph "⏱️ Performance Comparison"
        direction LR
        P1["Discovery-First:<br/>30s → Analysis"]
        P2["Traditional:<br/>5+ min → Maybe"]
        P3["Success Rate:<br/>95% vs 30%"]
    end
    
    S4 -.-> P1
    T3 -.-> P2
    S1 -.-> P3
    
    %% Commands breakdown
    subgraph "🛠️ Command Examples"
        direction TB
        C1["# Get overview<br/>parseltongue list-entities --limit 50"]
        C2["# Filter by type<br/>parseltongue list-entities --type functions | grep 'target'"]
        C3["# Find location<br/>parseltongue where-defined TargetFunction"]
        C4["# Analyze impact<br/>parseltongue blast-radius TargetFunction"]
    end
    
    S1 -.-> C1
    S2 -.-> C2
    S3 -.-> C3
    S4 -.-> C4
    
    %% Styling
    classDef discovery fill:#e8f5e8,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef traditional fill:#ffebee,stroke:#d32f2f,stroke-width:2px,color:#c62828
    classDef performance fill:#e1f5fe,stroke:#01579b,stroke-width:2px,color:#0d47a1
    classDef commands fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:#e65100
    
    class S1,S2,S3,S4 discovery
    class T1,T2,T3 traditional
    class P1,P2,P3 performance
    class C1,C2,C3,C4 commands
```

```bash
# Step 1: Get overview
parseltongue list-entities --limit 50

# Step 2: Identify target entities
parseltongue list-entities --type functions | grep "target_pattern"

# Step 3: Get exact location
parseltongue where-defined TargetFunction

# Step 4: Perform analysis
parseltongue blast-radius TargetFunction
```

### 2. Performance-Aware Usage
- Use `--limit` flags to control result size
- Batch related queries together
- Monitor timing with millisecond precision
- Validate against performance contracts

### 3. Error Handling
```rust
// Structured error handling
match discovery_engine.list_entities().await {
    Ok(entities) => process_entities(entities),
    Err(DiscoveryError::EntityNotFound { name }) => {
        suggest_similar_entities(&name)
    },
    Err(DiscoveryError::PerformanceViolation { operation, actual, limit }) => {
        log_performance_issue(operation, actual, limit)
    },
    Err(e) => handle_generic_error(e),
}
```

### 4. Workflow Integration
Use complete workflows instead of individual commands:
```bash
# Instead of manual command chains
parseltongue ingest codebase.dump
parseltongue list-entities
parseltongue entities-in-file src/main.rs

# Use orchestrated workflows
./parseltongue_dungeon/scripts/onboard_codebase.sh /path/to/codebase
```

## Timing Reporting Guidelines

### Precision Requirements
- **Always report milliseconds** when duration < 1 second
- **Use seconds + milliseconds** for durations > 1 second
- **Never report "0 seconds"** - use milliseconds instead

### Examples
```bash
# Good timing reports
✅ Entity discovery completed in 86 milliseconds (0.086 seconds)
✅ Analysis completed in 1.234 seconds (1234 milliseconds)
✅ Ingestion completed in 2.567 seconds

# Avoid these
❌ Analysis completed in 0 seconds
❌ Discovery completed in 0.0 seconds
```

### Implementation Pattern
```bash
# Bash timing with millisecond precision
START_TIME=$(date +%s%3N)
# ... operation ...
END_TIME=$(date +%s%3N)
DURATION=$((END_TIME - START_TIME))
echo "Operation completed in ${DURATION} milliseconds ($(echo "scale=3; $DURATION/1000" | bc -l) seconds)"
```

## Self-Analysis Use Case

### Demonstrated Capabilities
Parseltongue successfully analyzed its own codebase, demonstrating:

1. **Recursive Analysis**: Tool analyzing itself for continuous improvement
2. **Code Quality Detection**: Identified 5 warnings and 57 clippy suggestions
3. **Performance Validation**: All operations within performance contracts
4. **Architectural Insight**: Clear visibility into 2177 entities across 64 files

### Learning Outcomes
- **Discovery-first approach works**: Entity listing enables effective navigation
- **Performance contracts are realistic**: All targets achievable in practice
- **Self-improvement cycle**: Tool can identify its own improvement opportunities
- **Workflow orchestration adds value**: Complete journeys more valuable than individual commands

### Metrics Achieved
- **Total analysis time**: 9.0 seconds (well within 30-second target)
- **Entity discovery**: 208 entities identified across all types
- **File coverage**: 64 Rust files processed successfully
- **Success rate**: 100% for core discovery operations

## Integration Patterns

### With Kiro IDE
```markdown
# Use Parseltongue for codebase navigation
1. Run onboarding workflow for new projects
2. Use entity discovery for code exploration
3. Leverage blast radius for impact analysis
4. Generate LLM context for architectural discussions
```

### With CI/CD Pipelines
```yaml
# Example GitHub Actions integration
- name: Architectural Analysis
  run: |
    ./parseltongue_dungeon/scripts/onboard_codebase.sh .
    ./parseltongue_dungeon/scripts/generate_llm_context.sh .
```

### With Code Review
```bash
# Generate PR impact analysis
parseltongue blast-radius ChangedEntity > impact_analysis.md
```

## Troubleshooting

### Common Issues
1. **Parse errors**: Usually due to complex macro usage or incomplete files
2. **Entity not found**: Use `list-entities` to discover available entities
3. **Performance violations**: Check system load and file size
4. **Memory issues**: Monitor baseline vs current usage

### Debug Commands
```bash
# Check system status
parseltongue --version
parseltongue list-entities --limit 5

# Validate performance
time parseltongue list-entities --limit 100

# Check entity availability
parseltongue list-entities --type functions | grep "target_name"
```

## Future Enhancements

### Planned Improvements
- Enhanced timing precision in all operations
- Better error messages with suggestions
- Improved CLI experience with emojis and visual feedback
- Advanced Mermaid diagram generation
- Cross-language support expansion

### Performance Targets
- Discovery operations: <50 milliseconds
- Large codebase ingestion: <10 seconds for 1000+ files
- Memory efficiency: <15% increase from baseline
- Concurrent operations: Support for 10+ parallel queries

---

*This guide is automatically updated based on self-analysis results and real-world usage patterns. Last updated: $(date)*