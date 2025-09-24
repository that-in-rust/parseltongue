#!/bin/bash
# Parseltongue Debug Workflow Script
# Usage: ./debug_entity.sh FunctionName

set -e

FUNCTION_NAME="${1}"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
OUTPUT_DIR="./parseltongue_workspace/debug_${TIMESTAMP}"
# Find the latest parseltongue binary automatically
if [ -z "$PARSELTONGUE_BIN" ]; then
    PARSELTONGUE_BIN=$(ls -t ./target/release/parseltongue* 2>/dev/null | head -1)
    if [ -z "$PARSELTONGUE_BIN" ]; then
        echo "❌ Error: No parseltongue binary found in ./target/release/"
        echo "Build with: cargo build --release"
        exit 1
    fi
fi

echo "🐛 Parseltongue Debug Workflow"
echo "Function: $FUNCTION_NAME"
echo "Output: $OUTPUT_DIR"
echo "Timestamp: $TIMESTAMP"
echo ""

# Validate inputs
if [ -z "$FUNCTION_NAME" ]; then
    echo "❌ Error: Function name required"
    echo "Usage: $0 FunctionName"
    exit 1
fi

if [ ! -f "$PARSELTONGUE_BIN" ]; then
    echo "❌ Error: Parseltongue binary not found: $PARSELTONGUE_BIN"
    echo "Build with: cargo build --release"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "📍 Step 1: Locating function..."
START_TIME=$(date +%s)

# Find function definition
$PARSELTONGUE_BIN where-defined "$FUNCTION_NAME" > "$OUTPUT_DIR/function_location.txt" 2>/dev/null || {
    echo "❌ Function '$FUNCTION_NAME' not found"
    echo "💡 Try: $PARSELTONGUE_BIN list-entities --type functions | grep -i '$FUNCTION_NAME'"
    exit 1
}

LOCATION_TIME=$(date +%s)
echo "✅ Function located in $((LOCATION_TIME - START_TIME)) seconds"

echo ""
echo "🔗 Step 2: Finding callers..."

# Find who calls this function
$PARSELTONGUE_BIN blast-radius "$FUNCTION_NAME" | grep "CALLS" > "$OUTPUT_DIR/callers.txt" 2>/dev/null || echo "No callers found" > "$OUTPUT_DIR/callers.txt"

CALLER_COUNT=$(wc -l < "$OUTPUT_DIR/callers.txt" 2>/dev/null || echo "0")

CALLERS_TIME=$(date +%s)
echo "✅ Caller analysis completed in $((CALLERS_TIME - LOCATION_TIME)) seconds"
echo "   Found $CALLER_COUNT callers"

echo ""
echo "🎯 Step 3: Finding usage sites..."

# Find entities that use this function
$PARSELTONGUE_BIN blast-radius "$FUNCTION_NAME" | grep "USES\|CALLS" > "$OUTPUT_DIR/usage_sites.txt" 2>/dev/null || echo "No usage sites found" > "$OUTPUT_DIR/usage_sites.txt"

USAGE_COUNT=$(wc -l < "$OUTPUT_DIR/usage_sites.txt" 2>/dev/null || echo "0")

USAGE_TIME=$(date +%s)
echo "✅ Usage analysis completed in $((USAGE_TIME - CALLERS_TIME)) seconds"
echo "   Found $USAGE_COUNT usage sites"

echo ""
echo "🔍 Step 4: Minimal change scope analysis..."

# Determine minimal change scope
{
    echo "# Debug Analysis Report"
    echo "Function: $FUNCTION_NAME"
    echo "Generated: $(date)"
    echo ""
    echo "## Function Location"
    cat "$OUTPUT_DIR/function_location.txt"
    echo ""
    echo "## Direct Callers ($CALLER_COUNT)"
    head -10 "$OUTPUT_DIR/callers.txt"
    if [ "$CALLER_COUNT" -gt 10 ]; then
        echo "... and $((CALLER_COUNT - 10)) more"
    fi
    echo ""
    echo "## All Usage Sites ($USAGE_COUNT)"
    head -15 "$OUTPUT_DIR/usage_sites.txt"
    if [ "$USAGE_COUNT" -gt 15 ]; then
        echo "... and $((USAGE_COUNT - 15)) more"
    fi
    echo ""
    echo "## Minimal Change Scope"
    
    if [ "$USAGE_COUNT" -eq 0 ]; then
        echo "- ✅ Function appears unused - safe to modify or remove"
        echo "- 📝 Verify with grep search in codebase"
        echo "- 🧪 Add deprecation warning before removal"
    elif [ "$USAGE_COUNT" -le 3 ]; then
        echo "- ✅ Low impact - modify function and update $USAGE_COUNT usage sites"
        echo "- 📝 Update unit tests for function and callers"
        echo "- 🧪 Test all usage sites after changes"
    elif [ "$USAGE_COUNT" -le 10 ]; then
        echo "- ⚠️  Medium impact - careful coordination required"
        echo "- 📝 Create comprehensive test suite first"
        echo "- 🧪 Test all $USAGE_COUNT usage sites"
        echo "- 👥 Consider code review for changes"
    else
        echo "- 🚨 High impact - consider refactoring approach"
        echo "- 📝 Extensive testing required"
        echo "- 🧪 Integration tests for all usage patterns"
        echo "- 👥 Team review mandatory"
        echo "- 🔄 Consider backward-compatible changes first"
    fi
    
    echo ""
    echo "## Debug Strategy"
    echo "1. Add logging/debugging to function: $FUNCTION_NAME"
    echo "2. Test with minimal caller first"
    echo "3. Verify behavior with top 3 usage sites"
    echo "4. Check edge cases and error conditions"
    
    echo ""
    echo "## Files to Review"
    # Extract unique file paths from location and usage
    {
        cat "$OUTPUT_DIR/function_location.txt" 2>/dev/null || true
        cat "$OUTPUT_DIR/usage_sites.txt" 2>/dev/null || true
    } | grep -o '[^[:space:]]*\.rs' | sort -u | head -10
    
} > "$OUTPUT_DIR/debug_report.md"

SCOPE_TIME=$(date +%s)
echo "✅ Scope analysis completed in $((SCOPE_TIME - USAGE_TIME)) seconds"

TOTAL_TIME=$((SCOPE_TIME - START_TIME))

echo ""
echo "🎉 Debug Analysis Complete!"
echo "Total time: ${TOTAL_TIME} seconds"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "📋 Generated Files:"
echo "  - function_location.txt: Function definition location"
echo "  - callers.txt: Direct callers ($CALLER_COUNT found)"
echo "  - usage_sites.txt: All usage sites ($USAGE_COUNT found)"
echo "  - debug_report.md: Complete debug analysis and strategy"
echo ""
echo "🔍 Summary:"
echo "  Function: $FUNCTION_NAME"
echo "  Callers: $CALLER_COUNT"
echo "  Usage Sites: $USAGE_COUNT"
echo "  Analysis Time: ${TOTAL_TIME} seconds"

# Validate success criteria
if [ $TOTAL_TIME -lt 180 ]; then  # 3 minutes = 180 seconds
    echo "✅ SUCCESS: Debug analysis completed within 3-minute target"
else
    echo "⚠️  WARNING: Debug analysis took longer than 3-minute target"
fi

echo ""
echo "🔍 Next Steps:"
echo "  1. Review debug_report.md for complete analysis"
echo "  2. Start with minimal change scope recommendations"
echo "  3. Add debugging/logging to function"
echo "  4. Test with identified callers and usage sites"