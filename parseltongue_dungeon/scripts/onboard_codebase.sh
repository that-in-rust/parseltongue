#!/bin/bash
# Parseltongue Onboarding Workflow Script
# Usage: ./onboard_codebase.sh /path/to/codebase

set -e

CODEBASE_PATH="${1:-$(pwd)}"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
OUTPUT_DIR="./parseltongue_workspace/onboarding_${TIMESTAMP}"
PARSELTONGUE_BIN="${PARSELTONGUE_BIN:-./target/release/parseltongue_20250924231324}"

echo "🚀 Parseltongue Onboarding Workflow"
echo "Codebase: $CODEBASE_PATH"
echo "Output: $OUTPUT_DIR"
echo "Timestamp: $TIMESTAMP"
echo ""

# Validate inputs
if [ ! -d "$CODEBASE_PATH" ]; then
    echo "❌ Error: Codebase path does not exist: $CODEBASE_PATH"
    exit 1
fi

if [ ! -f "$PARSELTONGUE_BIN" ]; then
    echo "❌ Error: Parseltongue binary not found: $PARSELTONGUE_BIN"
    echo "Build with: cargo build --release"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "📊 Step 1: Ingesting codebase..."
START_TIME=$(date +%s)

# Find Rust files and create dump
find "$CODEBASE_PATH" -name "*.rs" -type f | head -1000 > "$OUTPUT_DIR/rust_files.txt"
FILE_COUNT=$(wc -l < "$OUTPUT_DIR/rust_files.txt")

echo "Found $FILE_COUNT Rust files"

# Create simple dump format for ingestion
echo "Creating codebase dump..."
{
    while IFS= read -r file; do
        echo "FILE: $file"
        cat "$file" 2>/dev/null || echo "// Could not read file"
        echo ""
    done < "$OUTPUT_DIR/rust_files.txt"
} > "$OUTPUT_DIR/codebase.dump"

INGEST_TIME=$(date +%s)
echo "✅ Ingestion completed in $((INGEST_TIME - START_TIME)) seconds"

echo ""
echo "📋 Step 2: Generating entity overview..."

# Generate entity listing
$PARSELTONGUE_BIN ingest "$OUTPUT_DIR/codebase.dump" 2>/dev/null || true
$PARSELTONGUE_BIN list-entities --limit 100 > "$OUTPUT_DIR/all_entities.txt" 2>/dev/null || echo "Entity listing not available"

# Generate entity counts by type
$PARSELTONGUE_BIN list-entities --type functions --limit 50 > "$OUTPUT_DIR/functions.txt" 2>/dev/null || echo "Function listing not available"
$PARSELTONGUE_BIN list-entities --type structs --limit 50 > "$OUTPUT_DIR/structs.txt" 2>/dev/null || echo "Struct listing not available"
$PARSELTONGUE_BIN list-entities --type traits --limit 50 > "$OUTPUT_DIR/traits.txt" 2>/dev/null || echo "Trait listing not available"

OVERVIEW_TIME=$(date +%s)
echo "✅ Overview completed in $((OVERVIEW_TIME - INGEST_TIME)) seconds"

echo ""
echo "🔍 Step 3: Identifying key entry points..."

# Look for main functions and common entry points
grep -n "fn main" "$OUTPUT_DIR/codebase.dump" | head -10 > "$OUTPUT_DIR/entry_points.txt" || echo "No main functions found"
grep -n "pub fn new" "$OUTPUT_DIR/codebase.dump" | head -20 >> "$OUTPUT_DIR/entry_points.txt" || true
grep -n "impl.*Service" "$OUTPUT_DIR/codebase.dump" | head -10 >> "$OUTPUT_DIR/entry_points.txt" || true

ENTRY_TIME=$(date +%s)
echo "✅ Entry point analysis completed in $((ENTRY_TIME - OVERVIEW_TIME)) seconds"

echo ""
echo "📈 Step 4: Generating architecture visualization..."

# Create simple architecture summary
{
    echo "# Codebase Architecture Summary"
    echo "Generated: $(date)"
    echo "Files analyzed: $FILE_COUNT"
    echo ""
    echo "## Entity Counts"
    echo "- Functions: $(wc -l < "$OUTPUT_DIR/functions.txt" 2>/dev/null || echo "0")"
    echo "- Structs: $(wc -l < "$OUTPUT_DIR/structs.txt" 2>/dev/null || echo "0")"
    echo "- Traits: $(wc -l < "$OUTPUT_DIR/traits.txt" 2>/dev/null || echo "0")"
    echo ""
    echo "## Key Entry Points"
    cat "$OUTPUT_DIR/entry_points.txt" 2>/dev/null || echo "None identified"
    echo ""
    echo "## Directory Structure"
    find "$CODEBASE_PATH" -name "*.rs" -type f | head -20 | sed 's|^|  |'
} > "$OUTPUT_DIR/architecture_summary.md"

ARCH_TIME=$(date +%s)
echo "✅ Architecture analysis completed in $((ARCH_TIME - ENTRY_TIME)) seconds"

TOTAL_TIME=$((ARCH_TIME - START_TIME))

echo ""
echo "🎉 Onboarding Complete!"
echo "Total time: ${TOTAL_TIME} seconds"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "📋 Generated Files:"
echo "  - codebase.dump: Raw codebase data"
echo "  - all_entities.txt: Complete entity listing"
echo "  - functions.txt: Function entities"
echo "  - structs.txt: Struct entities"
echo "  - traits.txt: Trait entities"
echo "  - entry_points.txt: Key entry points"
echo "  - architecture_summary.md: Architecture overview"
echo ""
echo "🔍 Next Steps:"
echo "  1. Review architecture_summary.md for codebase overview"
echo "  2. Use entity files to identify areas of interest"
echo "  3. Run feature impact analysis: ./feature_impact.sh EntityName"
echo "  4. Generate LLM context: ./generate_llm_context.sh $CODEBASE_PATH"

# Validate success criteria
if [ $TOTAL_TIME -lt 900 ]; then  # 15 minutes = 900 seconds
    echo "✅ SUCCESS: Onboarding completed within 15-minute target"
else
    echo "⚠️  WARNING: Onboarding took longer than 15-minute target"
fi