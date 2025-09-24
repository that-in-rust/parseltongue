# Parseltongue Workflow Templates

## Quick Start Templates

Copy and paste these templates, then customize the entity names for your specific codebase.

---

## Template 1: New to Codebase Analysis

### Time: 15-25 minutes | Confidence: 80% architectural understanding

```bash
#!/bin/bash
# New to Codebase Workflow Template
# Customize the entity names below for your domain

echo "=== PARSELTONGUE: NEW TO CODEBASE ANALYSIS ==="
echo "Target: [YOUR_CODEBASE_NAME]"
echo "Date: $(date)"
echo ""

# Phase 1: Discover Architectural Patterns (5-10 min)
echo "Phase 1: Discovering Architectural Patterns..."

# Generic patterns (customize these):
echo "1. Main trait implementations:"
parseltongue query what-implements Handler
parseltongue query what-implements Client
parseltongue query what-implements Server
parseltongue query what-implements Service

# Domain-specific patterns (uncomment and customize):
# For message streaming systems:
# parseltongue query what-implements ServerCommandHandler
# parseltongue query what-implements Source
# parseltongue query what-implements Sink

# For web frameworks:
# parseltongue query what-implements Router
# parseltongue query what-implements Middleware
# parseltongue query what-implements Controller

# For databases:
# parseltongue query what-implements Connection
# parseltongue query what-implements Transaction
# parseltongue query what-implements Query

echo ""

# Phase 2: Identify Core Components (5-10 min)
echo "Phase 2: Identifying Core Components..."

# Generic core types (customize these):
echo "2. Core component usage:"
parseltongue query uses Client
parseltongue query uses Server
parseltongue query uses Config
parseltongue query uses Error

# Domain-specific core types (uncomment and customize):
# For message streaming:
# parseltongue query uses Stream
# parseltongue query uses Message
# parseltongue query uses Topic

# For web frameworks:
# parseltongue query uses Request
# parseltongue query uses Response
# parseltongue query uses Route

# For databases:
# parseltongue query uses Connection
# parseltongue query uses Table
# parseltongue query uses Schema

echo ""

# Phase 3: Generate Visual Overview (2-5 min)
echo "Phase 3: Generating Visual Overview..."
echo "3. Creating visualization..."
parseltongue visualize

echo ""
echo "=== ANALYSIS COMPLETE ==="
echo "Next steps:"
echo "1. Open the generated HTML visualization"
echo "2. Review the architectural patterns found"
echo "3. Identify 3-5 core components for deeper study"
echo "4. Use Impact Analysis workflow before making changes"
```

### Customization Checklist
- [ ] Replace `[YOUR_CODEBASE_NAME]` with actual codebase name
- [ ] Uncomment and customize domain-specific patterns
- [ ] Add your system's specific trait names
- [ ] Add your system's core type names

---

## Template 2: Impact Analysis Before Changes

### Time: 7-15 minutes | Confidence: 90% impact understanding

```bash
#!/bin/bash
# Impact Analysis Workflow Template
# Set your target component below

TARGET_COMPONENT="[YOUR_COMPONENT_NAME]"  # e.g., "MessageHandler", "UserService", etc.
TARGET_FUNCTION="[YOUR_FUNCTION_NAME]"    # e.g., "process_request", "send_message", etc.

echo "=== PARSELTONGUE: IMPACT ANALYSIS ==="
echo "Target Component: $TARGET_COMPONENT"
echo "Target Function: $TARGET_FUNCTION"
echo "Date: $(date)"
echo ""

# Phase 1: Direct Impact Assessment (2-5 min)
echo "Phase 1: Direct Impact Assessment..."

if [ "$TARGET_COMPONENT" != "[YOUR_COMPONENT_NAME]" ]; then
    echo "1. Finding all usage points for $TARGET_COMPONENT:"
    USAGE_COUNT=$(parseltongue query uses $TARGET_COMPONENT | wc -l)
    parseltongue query uses $TARGET_COMPONENT
    echo "Usage count: $USAGE_COUNT"
    echo ""
fi

if [ "$TARGET_FUNCTION" != "[YOUR_FUNCTION_NAME]" ]; then
    echo "2. Finding all callers for $TARGET_FUNCTION:"
    CALLER_COUNT=$(parseltongue query calls $TARGET_FUNCTION | wc -l)
    parseltongue query calls $TARGET_FUNCTION
    echo "Caller count: $CALLER_COUNT"
    echo ""
fi

# Phase 2: Indirect Impact Assessment (3-7 min)
echo "Phase 2: Indirect Impact Assessment..."

echo "3. Checking test coverage impact:"
if [ "$TARGET_COMPONENT" != "[YOUR_COMPONENT_NAME]" ]; then
    parseltongue query uses $TARGET_COMPONENT | grep -i test
fi
if [ "$TARGET_FUNCTION" != "[YOUR_FUNCTION_NAME]" ]; then
    parseltongue query calls $TARGET_FUNCTION | grep -i test
fi
echo ""

# Phase 3: Risk Categorization (2-3 min)
echo "Phase 3: Risk Categorization..."

echo "4. Checking for critical system impact:"
if [ "$TARGET_COMPONENT" != "[YOUR_COMPONENT_NAME]" ]; then
    parseltongue query uses $TARGET_COMPONENT | grep -E "(main|server|client|core)"
fi

echo "5. Checking for public API impact:"
if [ "$TARGET_COMPONENT" != "[YOUR_COMPONENT_NAME]" ]; then
    parseltongue query uses $TARGET_COMPONENT | grep -E "(pub|public|api|interface)"
fi

echo ""
echo "=== RISK ASSESSMENT ==="
echo "Usage/Caller counts:"
if [ "$TARGET_COMPONENT" != "[YOUR_COMPONENT_NAME]" ]; then
    echo "- Component usage: $USAGE_COUNT"
fi
if [ "$TARGET_FUNCTION" != "[YOUR_FUNCTION_NAME]" ]; then
    echo "- Function callers: $CALLER_COUNT"
fi

echo ""
echo "Risk Level Guidelines:"
echo "- 1-5 uses: LOW risk (standard testing)"
echo "- 6-20 uses: MEDIUM risk (extended testing + review)"
echo "- 21-50 uses: HIGH risk (comprehensive testing + team review)"
echo "- 50+ uses: CRITICAL risk (phased rollout + extensive testing)"
```

### Customization Checklist
- [ ] Set `TARGET_COMPONENT` to your component name
- [ ] Set `TARGET_FUNCTION` to your function name
- [ ] Adjust grep patterns for your codebase conventions
- [ ] Modify risk thresholds based on your team's standards

---

## Template 3: Debugging and Tracing

### Time: 11-20 minutes | Confidence: 85% problem understanding

```bash
#!/bin/bash
# Debugging and Tracing Workflow Template
# Set your problem area below

PROBLEM_FUNCTION="[PROBLEMATIC_FUNCTION]"  # e.g., "handle_request", "process_data"
PROBLEM_STRUCT="[PROBLEMATIC_STRUCT]"      # e.g., "UserData", "MessageQueue"

echo "=== PARSELTONGUE: DEBUGGING AND TRACING ==="
echo "Problem Function: $PROBLEM_FUNCTION"
echo "Problem Struct: $PROBLEM_STRUCT"
echo "Date: $(date)"
echo ""

# Phase 1: Trace the Problem Area (3-5 min)
echo "Phase 1: Tracing Problem Area..."

if [ "$PROBLEM_FUNCTION" != "[PROBLEMATIC_FUNCTION]" ]; then
    echo "1. Finding callers of problematic function $PROBLEM_FUNCTION:"
    parseltongue query calls $PROBLEM_FUNCTION
    echo ""
fi

if [ "$PROBLEM_STRUCT" != "[PROBLEMATIC_STRUCT]" ]; then
    echo "2. Finding users of problematic struct $PROBLEM_STRUCT:"
    parseltongue query uses $PROBLEM_STRUCT
    echo ""
fi

echo "3. Checking error handling patterns:"
parseltongue query uses Error
echo ""

# Phase 2: Follow the Data Flow (5-10 min)
echo "Phase 2: Following Data Flow..."

echo "4. Tracing data flow patterns:"
# Generic data flow (customize for your domain):
parseltongue query calls process
parseltongue query calls handle
parseltongue query calls execute

# Domain-specific data flow (uncomment and customize):
# For message systems:
# parseltongue query calls send_message
# parseltongue query calls receive_message
# parseltongue query calls process_message

# For web systems:
# parseltongue query calls handle_request
# parseltongue query calls process_response
# parseltongue query calls middleware

# For databases:
# parseltongue query calls execute_query
# parseltongue query calls commit_transaction
# parseltongue query calls rollback

echo ""

echo "5. Finding transformation points:"
parseltongue query calls serialize
parseltongue query calls deserialize
parseltongue query calls convert
parseltongue query calls transform
echo ""

# Phase 3: Identify Related Components (3-5 min)
echo "Phase 3: Identifying Related Components..."

echo "6. Checking for circular dependencies:"
parseltongue query find-cycles
echo ""

echo "7. Looking for similar patterns:"
# Add your domain-specific similar patterns here
# parseltongue query what-implements [SimilarTrait]

echo ""
echo "=== DEBUGGING SUMMARY ==="
echo "Next steps:"
echo "1. Review the call chains and usage patterns above"
echo "2. Focus on the most frequently called functions"
echo "3. Check error handling in the identified components"
echo "4. Look for patterns in the transformation points"
echo "5. Investigate any circular dependencies found"
```

### Customization Checklist
- [ ] Set `PROBLEM_FUNCTION` to your problematic function
- [ ] Set `PROBLEM_STRUCT` to your problematic struct/type
- [ ] Uncomment and customize domain-specific data flow queries
- [ ] Add your system's specific transformation function names

---

## Domain-Specific Quick Templates

### Message Streaming Systems (like Iggy)
```bash
# Architectural discovery
parseltongue query what-implements ServerCommandHandler
parseltongue query what-implements Source
parseltongue query what-implements Sink

# Core components
parseltongue query uses IggyClient
parseltongue query uses Stream
parseltongue query uses Message

# Data flow tracing
parseltongue query calls send_messages
parseltongue query calls poll_messages
```

### Web Frameworks
```bash
# Architectural discovery
parseltongue query what-implements Handler
parseltongue query what-implements Router
parseltongue query what-implements Middleware

# Core components
parseltongue query uses Request
parseltongue query uses Response
parseltongue query uses Route

# Data flow tracing
parseltongue query calls handle_request
parseltongue query calls process_response
```

### Database Systems
```bash
# Architectural discovery
parseltongue query what-implements Connection
parseltongue query what-implements Transaction
parseltongue query what-implements Query

# Core components
parseltongue query uses Database
parseltongue query uses Table
parseltongue query uses Schema

# Data flow tracing
parseltongue query calls execute_query
parseltongue query calls commit
parseltongue query calls rollback
```

---

## Usage Instructions

1. **Choose the appropriate template** for your analysis needs
2. **Copy the template** to a new file (e.g., `analyze_codebase.sh`)
3. **Customize the entity names** using the checklist
4. **Make it executable**: `chmod +x analyze_codebase.sh`
5. **Run the analysis**: `./analyze_codebase.sh`
6. **Review the results** and follow the next steps

## Pro Tips

- **Save results**: Redirect output to files for complex analysis: `./analyze_codebase.sh > analysis_results.txt`
- **Filter results**: Use grep to focus on specific patterns: `parseltongue query uses Client | grep -v test`
- **Combine workflows**: Run multiple templates in sequence for comprehensive analysis
- **Document findings**: Keep notes on what you discover for future reference

## Success Metrics

After running these templates, you should have:
- [ ] Clear understanding of architectural patterns
- [ ] Complete impact assessment for planned changes
- [ ] Traced execution paths for debugging
- [ ] Identified next steps for investigation or implementation