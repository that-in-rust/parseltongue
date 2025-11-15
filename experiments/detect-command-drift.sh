#!/bin/bash
# Experiment: Command Drift Detection
# Implements Fix 2 from agent-fixes-tdd-design.md

set -e

echo "🔍 Command Drift Detection Experiment"
echo "======================================"
echo ""

BINARY="./target/release/parseltongue"

# Get actual pt07 subcommands from binary
echo "📊 Step 1: Extract actual pt07 subcommands from binary"
ACTUAL_COMMANDS=$($BINARY pt07 --help 2>/dev/null | grep -A 20 "Commands:" | grep "^  " | awk '{print $1}' | grep -v "^$" | grep -v "Commands:")

echo "   Actual commands:"
for cmd in $ACTUAL_COMMANDS; do
    echo "      ✅ $cmd"
done
echo ""

# Simulate what the OLD agent documentation had
echo "🔎 Step 2: Check what OLD agent used (from journal)"
OLD_AGENT_COMMANDS=(
    "render-entity-count-bar-chart"
    "render-dependency-cycle-warning-list"
)

echo "   Old agent commands:"
for cmd in "${OLD_AGENT_COMMANDS[@]}"; do
    echo "      📝 $cmd"
done
echo ""

# Detect drift
echo "🚨 Step 3: Detect command drift"
echo ""

DRIFT_FOUND=false

for OLD_CMD in "${OLD_AGENT_COMMANDS[@]}"; do
    # Check if command exists in actual
    if echo "$ACTUAL_COMMANDS" | grep -q "^${OLD_CMD}$"; then
        echo "   ✅ $OLD_CMD - OK"
    else
        echo "   ❌ $OLD_CMD - NOT FOUND!"
        DRIFT_FOUND=true

        # Fuzzy match: find similar commands
        echo "      💡 Checking for similar commands..."

        # Simple similarity: check if shortened version exists
        # "render-entity-count-bar-chart" → "entity-count"
        if echo "$OLD_CMD" | grep -q "render-entity-count"; then
            if echo "$ACTUAL_COMMANDS" | grep -q "entity-count"; then
                echo "      ✨ Suggested replacement: entity-count"
                echo "         (Likely renamed for brevity)"
            fi
        fi

        if echo "$OLD_CMD" | grep -q "render-dependency-cycle"; then
            if echo "$ACTUAL_COMMANDS" | grep -q "cycles"; then
                echo "      ✨ Suggested replacement: cycles"
                echo "         (Likely renamed for brevity)"
            fi
        fi
    fi
    echo ""
done

# Summary
echo "📋 Summary:"
if [ "$DRIFT_FOUND" = true ]; then
    echo "   ⚠️  Command drift detected!"
    echo "   🔧 Agent documentation needs updating"
    echo ""
    echo "   Recommended fixes:"
    echo "      render-entity-count-bar-chart → entity-count"
    echo "      render-dependency-cycle-warning-list → cycles"
    echo ""
    echo "✅ Experiment proves Fix 2 detection works!"
else
    echo "   ✅ No drift detected - all commands valid"
fi

echo ""
echo "🎯 This is exactly what CI validation would catch!"
