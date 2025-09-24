#!/bin/bash
# Parseltongue Feature Impact Analysis Script
# Usage: ./feature_impact.sh EntityName

set -e

ENTITY_NAME="${1}"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
OUTPUT_DIR="./parseltongue_workspace/feature_impact_${TIMESTAMP}"
PARSELTONGUE_BIN="${PARSELTONGUE_BIN:-./target/release/parseltongue_20250924231324}"

echo "🎯 Parseltongue Feature Impact Analysis"
echo "Entity: $ENTITY_NAME"
echo "Output: $OUTPUT_DIR"
echo "Timestamp: $TIMESTAMP"
echo ""

# Validate inputs
if [ -z "$ENTITY_NAME" ]; then
    echo "❌ Error: Entity name required"
    echo "Usage: $0 EntityName"
    exit 1
fi

if [ ! -f "$PARSELTONGUE_BIN" ]; then
    echo "❌ Error: Parseltongue binary not found: $PARSELTONGUE_BIN"
    echo "Build with: cargo build --release"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "🔍 Step 1: Finding entity definition..."
START_TIME=$(date +%s)

# Find where the entity is defined
$PARSELTONGUE_BIN where-defined "$ENTITY_NAME" > "$OUTPUT_DIR/definition.txt" 2>/dev/null || {
    echo "❌ Entity '$ENTITY_NAME' not found"
    echo "💡 Try: $PARSELTONGUE_BIN list-entities | grep -i '$ENTITY_NAME'"
    exit 1
}

DEFINITION_TIME=$(date +%s)
echo "✅ Definition found in $((DEFINITION_TIME - START_TIME)) seconds"

echo ""
echo "💥 Step 2: Calculating blast radius..."

# Calculate blast radius impact
$PARSELTONGUE_BIN blast-radius "$ENTITY_NAME" > "$OUTPUT_DIR/blast_radius.txt" 2>/dev/null || {
    echo "❌ Blast radius calculation failed"
    exit 1
}

# Extract impact metrics
IMPACT_COUNT=$(grep -c "IMPACT:" "$OUTPUT_DIR/blast_radius.txt" 2>/dev/null || echo "0")

BLAST_TIME=$(date +%s)
echo "✅ Blast radius calculated in $((BLAST_TIME - DEFINITION_TIME)) seconds"
echo "   Impact count: $IMPACT_COUNT entities"

echo ""
echo "📊 Step 3: Risk assessment..."

# Categorize risk level based on impact count
if [ "$IMPACT_COUNT" -le 5 ]; then
    RISK_LEVEL="LOW"
    RISK_COLOR="🟢"
elif [ "$IMPACT_COUNT" -le 20 ]; then
    RISK_LEVEL="MEDIUM"
    RISK_COLOR="🟡"
elif [ "$IMPACT_COUNT" -le 50 ]; then
    RISK_LEVEL="HIGH"
    RISK_COLOR="🟠"
else
    RISK_LEVEL="CRITICAL"
    RISK_COLOR="🔴"
fi

RISK_TIME=$(date +%s)
echo "✅ Risk assessment completed in $((RISK_TIME - BLAST_TIME)) seconds"
echo "   Risk level: $RISK_COLOR $RISK_LEVEL"

echo ""
echo "📋 Step 4: Generating change recommendations..."

# Generate change recommendations based on risk level
{
    echo "# Feature Impact Analysis Report"
    echo "Entity: $ENTITY_NAME"
    echo "Generated: $(date)"
    echo "Risk Level: $RISK_LEVEL ($IMPACT_COUNT impacts)"
    echo ""
    echo "## Definition Location"
    cat "$OUTPUT_DIR/definition.txt"
    echo ""
    echo "## Impact Analysis"
    cat "$OUTPUT_DIR/blast_radius.txt"
    echo ""
    echo "## Recommendations"
    
    case $RISK_LEVEL in
        "LOW")
            echo "- ✅ Safe to modify with standard testing"
            echo "- 📝 Write unit tests for the modified entity"
            echo "- 🔍 Review impacted entities for compatibility"
            ;;
        "MEDIUM")
            echo "- ⚠️  Moderate risk - requires careful testing"
            echo "- 📝 Write comprehensive unit and integration tests"
            echo "- 👥 Consider code review with team member"
            echo "- 🔍 Test all impacted entities"
            ;;
        "HIGH")
            echo "- 🚨 High risk - extensive testing required"
            echo "- 📝 Write comprehensive test suite"
            echo "- 👥 Mandatory code review with senior team member"
            echo "- 🔍 Integration testing for all impacted areas"
            echo "- 📊 Consider feature flags for gradual rollout"
            ;;
        "CRITICAL")
            echo "- 🔴 Critical risk - architectural change required"
            echo "- 📝 Comprehensive test suite with edge cases"
            echo "- 👥 Architecture review with entire team"
            echo "- 🔍 Full regression testing"
            echo "- 📊 Feature flags and gradual rollout mandatory"
            echo "- 🚀 Consider breaking change into smaller increments"
            ;;
    esac
    
    echo ""
    echo "## Test Strategy"
    echo "1. Unit tests for $ENTITY_NAME modifications"
    echo "2. Integration tests for top 5 impacted entities"
    echo "3. Regression tests for critical paths"
    echo "4. Performance tests if applicable"
    
} > "$OUTPUT_DIR/impact_report.md"

REPORT_TIME=$(date +%s)
echo "✅ Recommendations generated in $((REPORT_TIME - RISK_TIME)) seconds"

TOTAL_TIME=$((REPORT_TIME - START_TIME))

echo ""
echo "🎉 Feature Impact Analysis Complete!"
echo "Total time: ${TOTAL_TIME} seconds"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "📋 Generated Files:"
echo "  - definition.txt: Entity definition location"
echo "  - blast_radius.txt: Complete impact analysis"
echo "  - impact_report.md: Risk assessment and recommendations"
echo ""
echo "🔍 Summary:"
echo "  Entity: $ENTITY_NAME"
echo "  Risk Level: $RISK_COLOR $RISK_LEVEL"
echo "  Impact Count: $IMPACT_COUNT entities"
echo "  Analysis Time: ${TOTAL_TIME} seconds"

# Validate success criteria
if [ $TOTAL_TIME -lt 300 ]; then  # 5 minutes = 300 seconds
    echo "✅ SUCCESS: Analysis completed within 5-minute target"
else
    echo "⚠️  WARNING: Analysis took longer than 5-minute target"
fi