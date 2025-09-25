# Task 3: The "Architecture Analysis" Developer Experience

## 🎯 Goal
**15-minute mastery of any Rust codebase with LLM-powered insights**

## 🚀 Implementation Plan

### Step 1: Performance-Guaranteed Analysis
```bash
# Must complete in <15 minutes for any Rust codebase
./parseltongue onboard . --performance-mode
# ✅ Guarantees: <15min total, <500μs queries, <2s file analysis
```

### Step 2: LLM-Optimized Output Formats
```bash
# JSON for programmatic analysis
./parseltongue onboard . --format json > analysis.json

# Markdown for LLM consumption
./parseltongue onboard . --format markdown > analysis.md

# Structured for specific LLM prompts
./parseltongue generate-llm-context . > llm_context.md
```

### Step 3: Architectural Analysis Prompts
**File: `distribution/architecture-analysis/claude-prompts.md`**
```markdown
# Architectural Analysis Prompts

## Prompt 1: System Overview
"I need to understand this Rust codebase architecture. Here's the Parseltongue analysis:

[PASTE FULL ONBOARD OUTPUT]

Please provide:
1. **System Architecture**: Main components and their relationships
2. **Data Flow**: How information moves through the system  
3. **Key Abstractions**: Important traits, structs, and patterns
4. **Complexity Hotspots**: Areas with high coupling or complexity
5. **Extension Points**: Where new features should be added"

## Prompt 2: Change Impact Analysis
"I'm planning changes to this Rust system. Here's the current state:

[PASTE FEATURE-START OUTPUT]

Analyze:
1. **Blast Radius**: What will be affected by changes to [ENTITY]?
2. **Risk Assessment**: What could break and how to prevent it?
3. **Implementation Strategy**: Safest order of changes
4. **Testing Strategy**: What needs comprehensive testing?"

## Prompt 3: Performance Analysis
"Help me optimize this Rust codebase. Here's the structure:

[PASTE ONBOARD OUTPUT WITH --performance FLAG]

Focus on:
1. **Performance Bottlenecks**: High-traffic code paths
2. **Memory Efficiency**: Allocation patterns and optimization opportunities
3. **Concurrency**: Thread safety and parallelization opportunities
4. **Algorithmic Improvements**: Better data structures or algorithms"
```

### Step 4: Complex Codebase Examples
**File: `distribution/architecture-analysis/examples/`**
```
examples/
├── axum-analysis.md          # Web framework analysis
├── tokio-analysis.md         # Async runtime analysis  
├── serde-analysis.md         # Serialization library analysis
└── diesel-analysis.md        # ORM analysis
```

Each example shows:
1. **Raw Parseltongue output**
2. **LLM prompt used**
3. **LLM analysis result**
4. **Actionable insights derived**

### Step 5: Decision Support Framework
**File: `distribution/architecture-analysis/decision-framework.md`**
```markdown
# Architecture Decision Framework

## Before Making Changes
1. **Understand Current State**
   ```bash
   ./parseltongue onboard .
   ./parseltongue where-defined <target-entity>
   ```

2. **Assess Impact**
   ```bash
   ./parseltongue feature-start <target-entity>
   ./parseltongue query blast-radius <target-entity> --depth 3
   ```

3. **Check Safety**
   ```bash
   ./parseltongue refactor-check <target-entity>
   ./parseltongue query callers <target-entity>
   ```

4. **Get LLM Insights**
   ```bash
   ./parseltongue generate-context <target-entity> | pbcopy
   # Use with architectural analysis prompts
   ```

## Decision Matrix
| Change Type | Parseltongue Commands | LLM Prompt | Decision Criteria |
|-------------|----------------------|------------|-------------------|
| **New Feature** | `feature-start`, `where-defined` | System Overview + Change Impact | <5 files affected = Low Risk |
| **Refactoring** | `refactor-check`, `blast-radius` | Change Impact + Performance | <10 callers = Safe to proceed |
| **Performance** | `query dependencies`, `trace` | Performance Analysis | Critical path analysis |
| **Architecture** | `onboard`, `generate-context` | System Overview | Alignment with patterns |
```

## 📊 Success Metrics
- [ ] <15 minutes from unknown codebase to architectural understanding
- [ ] LLM produces actionable insights from Parseltongue output
- [ ] Decision framework enables confident change planning
- [ ] Examples work with real complex codebases

## 🧪 Test Protocol
1. **Large Codebase Test**: 10K+ line Rust project
2. **Time Constraint**: Must complete analysis in <15 minutes
3. **LLM Integration**: Feed output to Claude/ChatGPT
4. **Decision Quality**: Can make informed architectural decisions
5. **Confidence Level**: Developer feels confident about changes

## 🎯 Real-World Validation
Test with actual complex Rust projects:
- [ ] **Axum** (web framework) - routing and middleware analysis
- [ ] **Tokio** (async runtime) - concurrency and performance analysis  
- [ ] **Serde** (serialization) - trait system and macro analysis
- [ ] **Diesel** (ORM) - database abstraction and query building

Each test must demonstrate:
1. **Speed**: Complete analysis in <15 minutes
2. **Accuracy**: LLM insights match expert knowledge
3. **Actionability**: Insights lead to concrete next steps
4. **Confidence**: Developer can make informed decisions