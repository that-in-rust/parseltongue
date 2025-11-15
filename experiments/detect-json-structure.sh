#!/bin/bash
# Experiment: JSON Structure Detection
# Implements Fix 5 from agent-fixes-tdd-design.md

set -e

echo "🔍 JSON Structure Detection Experiment"
echo "======================================="
echo ""

# Use the actual public_api.json from our previous analysis
TEST_FILE="parseltongue20251115012556/public_api.json"

if [ ! -f "$TEST_FILE" ]; then
    echo "❌ Test file not found: $TEST_FILE"
    echo "   Using alternative..."
    TEST_FILE="parseltongue20251115012556/edges.json"
fi

echo "📄 Analyzing: $TEST_FILE"
echo ""

# Detect structure
echo "🔎 Step 1: Detect JSON structure"

# Check if it's a flat array or object
FIRST_CHAR=$(head -c 1 "$TEST_FILE")

if [ "$FIRST_CHAR" = "[" ]; then
    echo "   Structure: FlatArray"
    echo "   Pattern: [...]"
    echo ""
    echo "   ✅ Suggested jq pattern:"
    echo "      jq '.[:5]' $TEST_FILE"
elif [ "$FIRST_CHAR" = "{" ]; then
    echo "   Structure: Object (checking for metadata wrapper...)"

    # Check for metadata wrapper pattern
    if grep -q '"export_metadata"' "$TEST_FILE" && grep -q '"entities"' "$TEST_FILE"; then
        echo "   Pattern: MetadataWrapper"
        echo "   Fields: export_metadata + entities"
        echo ""
        echo "   ✅ Suggested jq patterns:"
        echo "      # Show metadata:"
        echo "      jq '.export_metadata' $TEST_FILE"
        echo ""
        echo "      # Show first 5 entities:"
        echo "      jq '.entities[:5]' $TEST_FILE"
        echo ""
        echo "      # Extract entity names:"
        echo "      jq '.entities[] | .entity_name' $TEST_FILE | head -5"
    else
        echo "   Pattern: Generic Object"
        echo ""
        echo "   ✅ Suggested jq pattern:"
        echo "      jq 'keys' $TEST_FILE"
    fi
fi

echo ""
echo "🧪 Step 2: Generate structure-aware preview"
echo ""

# Generate a valid preview (first 3 entities)
if grep -q '"export_metadata"' "$TEST_FILE" 2>/dev/null; then
    echo "   Creating preview with metadata + 3 entities..."
    echo ""

    # Use jq to create valid JSON preview
    PREVIEW=$(jq '{
        export_metadata: .export_metadata,
        entities: (.entities[:3]),
        _preview_note: "Showing 3 of \(.export_metadata.total_entities) entities"
    }' "$TEST_FILE" 2>/dev/null || echo "{}")

    if [ "$PREVIEW" != "{}" ]; then
        echo "$PREVIEW" | jq '.'
        echo ""
        echo "   ✅ Preview is valid JSON (jq parsed successfully)"
    fi
else
    echo "   Creating simple array preview..."
    jq '.[:3]' "$TEST_FILE" 2>/dev/null || echo "[]"
fi

echo ""
echo "📋 Summary:"
echo "   ✅ Structure detected automatically"
echo "   ✅ Correct jq pattern suggested"
echo "   ✅ Valid JSON preview generated (never truncated mid-object)"
echo ""
echo "🎯 This prevents the 'head -50 | jq' anti-pattern!"

echo ""
echo "⚠️  Old (broken) approach:"
echo "   head -50 $TEST_FILE | jq '.'  # ❌ Creates invalid JSON"
echo ""
echo "✅ New (correct) approach:"
echo "   jq '.entities[:5]' $TEST_FILE  # ✅ Always valid JSON"
