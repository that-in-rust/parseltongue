# Parseltongue Workflow Templates

## Overview
Ready-to-use templates for common parseltongue workflows. Copy, customize, and execute these templates for systematic code analysis.

---

## Template 1: New Codebase Analysis

### Bash Script Template
```bash
#!/bin/bash
# new_codebase_analysis.sh - Comprehensive analysis for unfamiliar codebases
# Usage: ./new_codebase_analysis.sh [dump_file]

DUMP_FILE=${1:-"dumps/main.dump"}
OUTPUT_DIR="analysis_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "=== New Codebase Analysis ==="
echo "Dump file: $DUMP_FILE"
echo "Output directory: $OUTPUT_DIR"
echo

# Step 1: Ingest the codebase
echo "1. Ingesting codebase..."
time ./bin/parseltongue ingest "$DUMP_FILE"

# Step 2: Discover main patterns
echo -e "\n2. Discovering architectural patterns..."
./bin/parseltongue debug --graph | grep -i "client\|server\|handler\|manager\|service" | head -20 > "$OUTPUT_DIR/main_patterns.txt"
echo "Main patterns found:"
cat "$OUTPUT_DIR/main_patterns.txt"

# Step 3: Interactive trait discovery
echo -e "\n3. Analyzing core traits..."
echo "Available traits:"
./bin/parseltongue debug --graph | grep -i "trait\|impl" | head -10

read -p "Enter main trait name to analyze: " trait_name
if [ ! -z "$trait_name" ]; then
    ./bin/parseltongue query what-implements "$trait_name" > "$OUTPUT_DIR/trait_${trait_name}_implementations.txt"
    echo "Implementations saved to $OUTPUT_DIR/trait_${trait_name}_implementations.txt"
fi

# Step 4: Component analysis
echo -e "\n4. Analyzing key components..."
read -p "Enter main component/struct name: " component_name
if [ ! -z "$component_name" ]; then
    ./bin/parseltongue generate-context "$component_name" --format human > "$OUTPUT_DIR/component_${component_name}_context.txt"
    ./bin/parseltongue query uses "$component_name" > "$OUTPUT_DIR/component_${component_name}_users.txt"
    echo "Component analysis saved to $OUTPUT_DIR/"
fi

# Step 5: Generate visualization
echo -e "\n5. Generating architectural visualization..."
./bin/parseltongue visualize --output "$OUTPUT_DIR/architecture.html"

echo -e "\n=== Analysis Complete ==="
echo "Results saved in: $OUTPUT_DIR/"
echo "Open $OUTPUT_DIR/architecture.html to explore the architecture visually"
```

### Manual Workflow Checklist
```markdown
## New Codebase Analysis Checklist

### Phase 1: Setup (2 minutes)
- [ ] Ingest codebase: `./bin/parseltongue ingest dumps/your-dump.txt`
- [ ] Verify ingestion: `./bin/parseltongue debug --graph | head -5`

### Phase 2: Pattern Discovery (3 minutes)
- [ ] Find main patterns: `./bin/parseltongue debug --graph | grep -i "client\|server\|handler" | head -10`
- [ ] Identify core traits: `./bin/parseltongue debug --graph | grep -i trait | head -10`
- [ ] Note key entities: ________________

### Phase 3: Architectural Understanding (3 minutes)
- [ ] Analyze main trait: `./bin/parseltongue query what-implements <MainTrait>`
- [ ] Count implementations: _______ implementations found
- [ ] Understand patterns: ________________

### Phase 4: Component Mapping (2 minutes)
- [ ] Analyze key component: `./bin/parseltongue generate-context <MainComponent> --format human`
- [ ] Find component usage: `./bin/parseltongue query uses <MainComponent>`
- [ ] Generate visualization: `./bin/parseltongue visualize --output architecture.html`

### Results Summary
- **Main architectural pattern**: ________________
- **Key components identified**: ________________
- **System complexity level**: Low/Medium/High
- **Confidence in understanding**: ___/10
```

---

## Template 2: Change Impact Analysis

### Bash Script Template
```bash
#!/bin/bash
# change_impact_analysis.sh - Assess impact before making changes
# Usage: ./change_impact_analysis.sh <entity_to_change>

ENTITY=$1
if [ -z "$ENTITY" ]; then
    echo "Usage: $0 <entity_name>"
    echo "Example: $0 ServerCommandHandler"
    exit 1
fi

OUTPUT_DIR="impact_analysis_${ENTITY}_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "=== Change Impact Analysis for: $ENTITY ==="
echo "Output directory: $OUTPUT_DIR"
echo

# Step 1: Find direct users
echo "1. Finding direct users..."
./bin/parseltongue query uses "$ENTITY" > "$OUTPUT_DIR/direct_users.txt"
USERS_COUNT=$(wc -l < "$OUTPUT_DIR/direct_users.txt")
echo "Found $USERS_COUNT direct users"

# Step 2: Find function callers (if it's a function)
echo -e "\n2. Finding function callers..."
./bin/parseltongue query calls "$ENTITY" > "$OUTPUT_DIR/function_callers.txt"
CALLERS_COUNT=$(wc -l < "$OUTPUT_DIR/function_callers.txt")
echo "Found $CALLERS_COUNT function callers"

# Step 3: Calculate blast radius
echo -e "\n3. Calculating blast radius..."
./bin/parseltongue query blast-radius "$ENTITY" > "$OUTPUT_DIR/blast_radius.txt"
echo "Blast radius analysis saved"

# Step 4: Generate comprehensive context
echo -e "\n4. Generating comprehensive context..."
./bin/parseltongue generate-context "$ENTITY" --format json > "$OUTPUT_DIR/comprehensive_context.json"
./bin/parseltongue generate-context "$ENTITY" --format human > "$OUTPUT_DIR/comprehensive_context.txt"

# Step 5: Risk assessment
echo -e "\n5. Risk Assessment:"
if [ $USERS_COUNT -gt 50 ]; then
    echo "🔴 HIGH RISK: $USERS_COUNT users - extensive impact expected"
elif [ $USERS_COUNT -gt 10 ]; then
    echo "🟡 MEDIUM RISK: $USERS_COUNT users - moderate impact expected"
else
    echo "🟢 LOW RISK: $USERS_COUNT users - limited impact expected"
fi

# Step 6: Generate recommendations
cat > "$OUTPUT_DIR/change_recommendations.md" << EOF
# Change Impact Analysis: $ENTITY

## Summary
- **Direct Users**: $USERS_COUNT
- **Function Callers**: $CALLERS_COUNT
- **Risk Level**: $([ $USERS_COUNT -gt 50 ] && echo "HIGH" || ([ $USERS_COUNT -gt 10 ] && echo "MEDIUM" || echo "LOW"))

## Recommendations

### Before Making Changes
- [ ] Review all direct users in direct_users.txt
- [ ] Check function callers in function_callers.txt
- [ ] Understand full context from comprehensive_context.txt

### Testing Strategy
- [ ] Unit tests for $ENTITY itself
- [ ] Integration tests for high-usage components
- [ ] Regression tests for critical paths

### Implementation Strategy
$(if [ $USERS_COUNT -gt 50 ]; then
    echo "- [ ] Consider backward compatibility"
    echo "- [ ] Plan phased rollout"
    echo "- [ ] Prepare migration guide"
elif [ $USERS_COUNT -gt 10 ]; then
    echo "- [ ] Ensure good test coverage"
    echo "- [ ] Consider deprecation warnings"
    echo "- [ ] Update documentation"
else
    echo "- [ ] Standard testing approach"
    echo "- [ ] Direct implementation acceptable"
fi)

### Files to Review
$(head -10 "$OUTPUT_DIR/direct_users.txt" | sed 's/^/- /')
$([ $USERS_COUNT -gt 10 ] && echo "- ... and $(($USERS_COUNT - 10)) more files")
EOF

echo -e "\n=== Analysis Complete ==="
echo "Results saved in: $OUTPUT_DIR/"
echo "Review change_recommendations.md for next steps"
```

### Manual Workflow Checklist
```markdown
## Change Impact Analysis Checklist

### Entity to Change: ________________

### Phase 1: Direct Impact (2 minutes)
- [ ] Find users: `./bin/parseltongue query uses <Entity>`
- [ ] Count users: _______ direct users found
- [ ] Find callers: `./bin/parseltongue query calls <Entity>`
- [ ] Count callers: _______ function callers found

### Phase 2: Comprehensive Analysis (3 minutes)
- [ ] Get full context: `./bin/parseltongue generate-context <Entity> --format human`
- [ ] Review dependencies and relationships
- [ ] Identify critical usage patterns

### Phase 3: Risk Assessment
- [ ] **User Count Risk**:
  - [ ] 0-10 users: LOW RISK 🟢
  - [ ] 11-50 users: MEDIUM RISK 🟡  
  - [ ] 50+ users: HIGH RISK 🔴
- [ ] **Usage Pattern Risk**:
  - [ ] Test files only: LOW RISK
  - [ ] Core functionality: HIGH RISK
  - [ ] Public API: HIGH RISK

### Phase 4: Change Strategy
- [ ] **Low Risk**: Direct implementation
- [ ] **Medium Risk**: Phased approach with good testing
- [ ] **High Risk**: Backward compatibility + migration plan

### Files to Review Before Changes
- [ ] ________________
- [ ] ________________
- [ ] ________________

### Testing Requirements
- [ ] Unit tests for the entity itself
- [ ] Integration tests for major users
- [ ] Regression tests for critical paths
```

---

## Template 3: Debugging and Tracing

### Bash Script Template
```bash
#!/bin/bash
# debugging_trace_analysis.sh - Trace execution paths and debug issues
# Usage: ./debugging_trace_analysis.sh <problem_entity>

PROBLEM_ENTITY=$1
if [ -z "$PROBLEM_ENTITY" ]; then
    echo "Usage: $0 <problem_entity>"
    echo "Example: $0 send_messages"
    echo "         $0 MessageHandler"
    exit 1
fi

OUTPUT_DIR="debug_trace_${PROBLEM_ENTITY}_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "=== Debugging Trace Analysis for: $PROBLEM_ENTITY ==="
echo "Output directory: $OUTPUT_DIR"
echo

# Step 1: Find execution paths
echo "1. Tracing execution paths..."
./bin/parseltongue query calls "$PROBLEM_ENTITY" > "$OUTPUT_DIR/execution_paths.txt"
CALLERS_COUNT=$(wc -l < "$OUTPUT_DIR/execution_paths.txt")
echo "Found $CALLERS_COUNT execution paths"

# Step 2: Trace data flow
echo -e "\n2. Tracing data flow..."
./bin/parseltongue query uses "$PROBLEM_ENTITY" > "$OUTPUT_DIR/data_flow.txt"
USERS_COUNT=$(wc -l < "$OUTPUT_DIR/data_flow.txt")
echo "Found $USERS_COUNT data flow connections"

# Step 3: Get comprehensive context
echo -e "\n3. Getting comprehensive context..."
./bin/parseltongue generate-context "$PROBLEM_ENTITY" --format human > "$OUTPUT_DIR/full_context.txt"
./bin/parseltongue generate-context "$PROBLEM_ENTITY" --format json > "$OUTPUT_DIR/full_context.json"

# Step 4: Check for circular dependencies
echo -e "\n4. Checking for circular dependencies..."
./bin/parseltongue query find-cycles "$PROBLEM_ENTITY" > "$OUTPUT_DIR/circular_dependencies.txt"
CYCLES_COUNT=$(wc -l < "$OUTPUT_DIR/circular_dependencies.txt")
if [ $CYCLES_COUNT -gt 0 ]; then
    echo "⚠️  Found $CYCLES_COUNT circular dependencies"
else
    echo "✅ No circular dependencies found"
fi

# Step 5: Generate debugging report
cat > "$OUTPUT_DIR/debugging_report.md" << EOF
# Debugging Analysis: $PROBLEM_ENTITY

## Execution Flow Analysis
- **Callers Found**: $CALLERS_COUNT
- **Data Flow Connections**: $USERS_COUNT
- **Circular Dependencies**: $CYCLES_COUNT

## Key Findings

### Execution Paths
$(head -10 "$OUTPUT_DIR/execution_paths.txt" | sed 's/^/- /')
$([ $CALLERS_COUNT -gt 10 ] && echo "- ... and $(($CALLERS_COUNT - 10)) more callers")

### Data Flow
$(head -10 "$OUTPUT_DIR/data_flow.txt" | sed 's/^/- /')
$([ $USERS_COUNT -gt 10 ] && echo "- ... and $(($USERS_COUNT - 10)) more users")

### Potential Issues
$(if [ $CYCLES_COUNT -gt 0 ]; then
    echo "🔴 **Circular Dependencies Detected**"
    echo "- Review circular_dependencies.txt for details"
    echo "- May cause infinite loops or stack overflow"
    echo ""
fi)

$(if [ $CALLERS_COUNT -eq 0 ]; then
    echo "🟡 **No Callers Found**"
    echo "- Function may be unused or entry point"
    echo "- Check if it's a public API or callback"
    echo ""
fi)

$(if [ $USERS_COUNT -eq 0 ]; then
    echo "🟡 **No Users Found**"
    echo "- Type may be internal or unused"
    echo "- Check if it's a trait or interface"
    echo ""
fi)

## Debugging Strategy

### 1. Trace Entry Points
$(head -5 "$OUTPUT_DIR/execution_paths.txt" | sed 's/^/- Start debugging from: /')

### 2. Check Data Flow
$(head -5 "$OUTPUT_DIR/data_flow.txt" | sed 's/^/- Verify data integrity at: /')

### 3. Validate Assumptions
- [ ] Check input parameters at entry points
- [ ] Verify state consistency in data flow
- [ ] Test error handling paths

### 4. Common Debug Points
- [ ] Function entry and exit
- [ ] Error handling branches
- [ ] State mutation points
- [ ] External dependencies

## Files to Examine
$(cat "$OUTPUT_DIR/execution_paths.txt" "$OUTPUT_DIR/data_flow.txt" | sort -u | head -15 | sed 's/^/- /')
EOF

echo -e "\n=== Debugging Analysis Complete ==="
echo "Results saved in: $OUTPUT_DIR/"
echo "Review debugging_report.md for investigation strategy"
```

### Manual Workflow Checklist
```markdown
## Debugging and Tracing Checklist

### Problem Entity: ________________

### Phase 1: Execution Tracing (2 minutes)
- [ ] Find callers: `./bin/parseltongue query calls <ProblemEntity>`
- [ ] Count execution paths: _______ callers found
- [ ] Identify entry points: ________________

### Phase 2: Data Flow Analysis (2 minutes)
- [ ] Find users: `./bin/parseltongue query uses <ProblemEntity>`
- [ ] Count data connections: _______ users found
- [ ] Map data dependencies: ________________

### Phase 3: Context Analysis (2 minutes)
- [ ] Get full context: `./bin/parseltongue generate-context <ProblemEntity> --format human`
- [ ] Check for cycles: `./bin/parseltongue query find-cycles <ProblemEntity>`
- [ ] Circular dependencies found: Yes/No

### Phase 4: Issue Identification
- [ ] **No Callers**: Unused code or entry point
- [ ] **No Users**: Internal type or interface
- [ ] **Many Callers**: High-impact component
- [ ] **Circular Dependencies**: Potential infinite loops

### Phase 5: Debugging Strategy
- [ ] **Entry Points to Debug**:
  - [ ] ________________
  - [ ] ________________
- [ ] **Data Flow to Verify**:
  - [ ] ________________
  - [ ] ________________
- [ ] **Critical Paths to Test**:
  - [ ] ________________
  - [ ] ________________

### Investigation Results
- **Root cause hypothesis**: ________________
- **Files to examine**: ________________
- **Tests to run**: ________________
- **Next debugging steps**: ________________
```

---

## Template 4: Architecture Documentation

### Bash Script Template
```bash
#!/bin/bash
# architecture_documentation.sh - Generate comprehensive architecture documentation
# Usage: ./architecture_documentation.sh [project_name]

PROJECT_NAME=${1:-"Project"}
OUTPUT_DIR="architecture_docs_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "=== Architecture Documentation Generator ==="
echo "Project: $PROJECT_NAME"
echo "Output directory: $OUTPUT_DIR"
echo

# Step 1: Generate overall visualization
echo "1. Generating architecture visualization..."
./bin/parseltongue visualize --output "$OUTPUT_DIR/architecture_overview.html"

# Step 2: Discover main patterns
echo -e "\n2. Discovering architectural patterns..."
./bin/parseltongue debug --graph | grep -i "trait\|struct\|enum" > "$OUTPUT_DIR/all_entities.txt"
./bin/parseltongue debug --graph | grep -i "client\|server\|handler\|manager\|service\|controller" > "$OUTPUT_DIR/main_patterns.txt"

# Step 3: Interactive component analysis
echo -e "\n3. Analyzing key components..."
echo "Main patterns found:"
head -10 "$OUTPUT_DIR/main_patterns.txt"

# Get top traits
TRAITS=$(./bin/parseltongue debug --graph | grep -i trait | head -5 | cut -d' ' -f1)
for trait in $TRAITS; do
    if [ ! -z "$trait" ]; then
        echo "Analyzing trait: $trait"
        ./bin/parseltongue query what-implements "$trait" > "$OUTPUT_DIR/trait_${trait}_analysis.txt"
        ./bin/parseltongue generate-context "$trait" --format human > "$OUTPUT_DIR/trait_${trait}_context.txt"
    fi
done

# Step 4: Generate architecture document
cat > "$OUTPUT_DIR/ARCHITECTURE.md" << EOF
# $PROJECT_NAME Architecture Documentation

**Generated**: $(date)  
**Tool**: Parseltongue automated analysis

## Overview

This document provides an architectural overview of $PROJECT_NAME based on automated code analysis using parseltongue.

## Architecture Visualization

Open [architecture_overview.html](architecture_overview.html) for an interactive view of the system architecture.

## Key Components

### Main Architectural Patterns
$(head -10 "$OUTPUT_DIR/main_patterns.txt" | sed 's/^/- /')

### Core Traits and Interfaces
$(for trait_file in "$OUTPUT_DIR"/trait_*_analysis.txt; do
    if [ -f "$trait_file" ]; then
        trait_name=$(basename "$trait_file" | sed 's/trait_\(.*\)_analysis.txt/\1/')
        impl_count=$(wc -l < "$trait_file")
        echo "- **$trait_name**: $impl_count implementations"
    fi
done)

## Component Details

$(for context_file in "$OUTPUT_DIR"/trait_*_context.txt; do
    if [ -f "$context_file" ]; then
        trait_name=$(basename "$context_file" | sed 's/trait_\(.*\)_context.txt/\1/')
        echo "### $trait_name"
        echo ""
        echo "\`\`\`"
        head -20 "$context_file"
        echo "\`\`\`"
        echo ""
    fi
done)

## System Statistics

- **Total Entities**: $(wc -l < "$OUTPUT_DIR/all_entities.txt")
- **Main Patterns**: $(wc -l < "$OUTPUT_DIR/main_patterns.txt")
- **Analysis Date**: $(date)

## Files Generated

- \`architecture_overview.html\` - Interactive architecture visualization
- \`all_entities.txt\` - Complete entity list
- \`main_patterns.txt\` - Key architectural patterns
- \`trait_*_analysis.txt\` - Individual trait analysis
- \`trait_*_context.txt\` - Detailed trait contexts

## Usage

1. Open \`architecture_overview.html\` in a web browser for visual exploration
2. Review trait analysis files for detailed component understanding
3. Use entity lists for targeted code exploration

EOF

echo -e "\n=== Architecture Documentation Complete ==="
echo "Results saved in: $OUTPUT_DIR/"
echo "Main document: $OUTPUT_DIR/ARCHITECTURE.md"
echo "Interactive view: $OUTPUT_DIR/architecture_overview.html"
```

---

## Template 5: AI-Assisted Analysis

### Context Generation Template
```bash
#!/bin/bash
# ai_context_generator.sh - Generate AI-ready contexts for multiple entities
# Usage: ./ai_context_generator.sh entity1 entity2 entity3...

if [ $# -eq 0 ]; then
    echo "Usage: $0 <entity1> [entity2] [entity3] ..."
    echo "Example: $0 ServerCommandHandler IggyClient send_messages"
    exit 1
fi

OUTPUT_DIR="ai_contexts_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "=== AI Context Generation ==="
echo "Entities: $@"
echo "Output directory: $OUTPUT_DIR"
echo

# Generate contexts for each entity
for entity in "$@"; do
    echo "Processing $entity..."
    
    # Generate both human and JSON formats
    ./bin/parseltongue generate-context "$entity" --format human > "$OUTPUT_DIR/${entity}_context.txt"
    ./bin/parseltongue generate-context "$entity" --format json > "$OUTPUT_DIR/${entity}_context.json"
    
    # Generate related queries
    ./bin/parseltongue query uses "$entity" > "$OUTPUT_DIR/${entity}_users.txt"
    ./bin/parseltongue query calls "$entity" > "$OUTPUT_DIR/${entity}_callers.txt"
done

# Create AI prompt template
cat > "$OUTPUT_DIR/ai_analysis_prompt.md" << EOF
# AI Analysis Prompt Template

## System Context
I'm analyzing a Rust codebase using parseltongue. I have generated contexts for the following entities:

$(for entity in "$@"; do
    echo "- **$entity**"
done)

## Individual Entity Contexts

$(for entity in "$@"; do
    echo "### $entity Context"
    echo ""
    echo "\`\`\`json"
    cat "$OUTPUT_DIR/${entity}_context.json"
    echo "\`\`\`"
    echo ""
    echo "**Users:**"
    echo "\`\`\`"
    head -10 "$OUTPUT_DIR/${entity}_users.txt"
    echo "\`\`\`"
    echo ""
    echo "**Callers:**"
    echo "\`\`\`"
    head -10 "$OUTPUT_DIR/${entity}_callers.txt"
    echo "\`\`\`"
    echo ""
done)

## Analysis Questions

Based on these contexts, please analyze:

1. **Architectural Patterns**: What design patterns are evident in these components?
2. **Component Relationships**: How do these entities interact with each other?
3. **System Design**: What does this tell us about the overall system architecture?
4. **Best Practices**: What Rust best practices are demonstrated here?
5. **Potential Issues**: Are there any code smells or potential problems?
6. **Extension Points**: How would you extend or modify these components?

## Specific Questions

### For Traits
- What is the purpose of each trait?
- How many implementations exist?
- What patterns do the implementations follow?

### For Structs/Types
- What is the role of each type in the system?
- What are the key dependencies?
- How is the type used throughout the codebase?

### For Functions
- What is the execution flow?
- What are the key dependencies?
- How is error handling implemented?

## Response Format

Please provide:
1. **Executive Summary** (2-3 sentences)
2. **Detailed Analysis** (organized by entity)
3. **Architectural Insights** (patterns and design principles)
4. **Recommendations** (improvements or extensions)
5. **Implementation Guidance** (how to work with these components)

EOF

# Create clipboard-ready version (macOS)
if command -v pbcopy &> /dev/null; then
    cat "$OUTPUT_DIR/ai_analysis_prompt.md" | pbcopy
    echo "✅ AI prompt copied to clipboard!"
fi

echo -e "\n=== AI Context Generation Complete ==="
echo "Results saved in: $OUTPUT_DIR/"
echo "Use ai_analysis_prompt.md with your AI assistant"
echo "Individual contexts available in JSON and text formats"
```

### AI Integration Checklist
```markdown
## AI-Assisted Analysis Checklist

### Entities to Analyze: ________________

### Phase 1: Context Generation (2 minutes)
- [ ] Generate contexts: `./bin/parseltongue generate-context <Entity> --format json`
- [ ] Copy to clipboard: `pbcopy` (macOS) or manual copy
- [ ] Verify context completeness

### Phase 2: AI Prompt Preparation (1 minute)
- [ ] Use specific prompt template
- [ ] Include system context (Rust, message broker, etc.)
- [ ] Add specific analysis goals
- [ ] Format for readability

### Phase 3: AI Analysis (5-10 minutes)
- [ ] Paste context into AI assistant
- [ ] Ask specific, targeted questions
- [ ] Request structured response
- [ ] Validate AI insights against code

### Phase 4: Validation (2 minutes)
- [ ] Cross-reference AI insights with actual code
- [ ] Check for hallucinations or errors
- [ ] Verify architectural claims
- [ ] Test implementation suggestions

### AI Prompt Template
```
CONTEXT: [Parseltongue output]
SYSTEM: Rust-based [system type]
GOAL: [Specific analysis goal]

QUESTIONS:
1. [Specific question 1]
2. [Specific question 2]
3. [Specific question 3]

FORMAT: [Desired response structure]
```

### Quality Checklist
- [ ] AI correctly identifies architectural patterns
- [ ] Recommendations are implementable
- [ ] Insights are non-obvious and valuable
- [ ] Response addresses all questions
- [ ] No obvious hallucinations or errors
```

---

## Customization Guide

### Adapting Templates for Your Domain

#### Message Broker Systems
```bash
# Focus on these patterns
grep -i "handler\|command\|message\|broker\|client\|server"

# Key entities to analyze
ServerCommandHandler, MessageBroker, IggyClient, send_messages
```

#### Web Frameworks
```bash
# Focus on these patterns  
grep -i "controller\|route\|middleware\|request\|response"

# Key entities to analyze
RequestHandler, Router, Middleware, HttpRequest
```

#### Database Systems
```bash
# Focus on these patterns
grep -i "query\|connection\|transaction\|schema\|table"

# Key entities to analyze
QueryBuilder, Connection, Transaction, Schema
```

### Performance Optimization

#### For Large Codebases
```bash
# Use targeted analysis instead of full dumps
# Focus on specific modules or components
# Generate smaller, focused dumps

# Example: Focus on core module only
grep -A 1000 "FILE: src/core" full-dump.txt > core-only-dump.txt
```

#### For Frequent Analysis
```bash
# Cache common contexts
mkdir -p cache/contexts/
./bin/parseltongue generate-context CommonEntity --format json > cache/contexts/CommonEntity.json

# Reuse cached contexts for AI analysis
```

### Integration with Development Workflow

#### Pre-commit Analysis
```bash
# Add to pre-commit hooks
./bin/parseltongue ingest current-changes.dump
./bin/parseltongue query blast-radius ChangedEntity
```

#### CI/CD Integration
```bash
# Add to CI pipeline
if [ "$CHANGED_FILES" ]; then
    ./change_impact_analysis.sh "$CHANGED_ENTITY"
    # Fail if high-risk changes without proper testing
fi
```

---

**Usage Notes**:
- Customize entity names for your specific codebase
- Adjust risk thresholds based on your system's characteristics  
- Add domain-specific patterns to discovery commands
- Integrate templates into your development workflow
- Use AI integration for complex architectural analysis