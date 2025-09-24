#!/bin/bash
# Parseltongue Self-Analysis Script (Simplified)
# Task 24: Use Parseltongue to analyze and improve itself

set -e

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
OUTPUT_DIR="./parseltongue_workspace/self_analysis_${TIMESTAMP}"
PARSELTONGUE_BIN="${PARSELTONGUE_BIN:-./target/release/parseltongue_20250924231324}"
CODEBASE_PATH="$(pwd)"

echo "🔍 Parseltongue Self-Analysis (Simplified)"
echo "Analyzing: $CODEBASE_PATH"
echo "Output: $OUTPUT_DIR"
echo "Binary: $PARSELTONGUE_BIN"
echo "Timestamp: $TIMESTAMP"
echo ""

# Validate parseltongue binary exists
if [ ! -f "$PARSELTONGUE_BIN" ]; then
    echo "❌ Error: Parseltongue binary not found: $PARSELTONGUE_BIN"
    echo "Build with: cargo build --release"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "📊 Step 1: Ingesting Parseltongue codebase..."
START_TIME=$(date +%s)

# Find all Rust files in src/ and tests/
find src/ tests/ examples/ -name "*.rs" -type f 2>/dev/null | sort > "$OUTPUT_DIR/rust_files.txt"
FILE_COUNT=$(wc -l < "$OUTPUT_DIR/rust_files.txt")

echo "Found $FILE_COUNT Rust files to analyze"

# Create codebase dump
echo "Creating comprehensive codebase dump..."
{
    while IFS= read -r file; do
        echo "FILE: $file"
        cat "$file" 2>/dev/null || echo "// Could not read file: $file"
        echo ""
    done < "$OUTPUT_DIR/rust_files.txt"
} > "$OUTPUT_DIR/parseltongue_codebase.dump"

INGEST_END=$(date +%s)
INGEST_DURATION=$((INGEST_END - START_TIME))
echo "✅ Ingestion completed in ${INGEST_DURATION} seconds"

echo ""
echo "🔍 Step 2: Running Parseltongue analysis on itself..."

# Ingest the codebase
ANALYSIS_START=$(date +%s)
echo "Ingesting codebase into Parseltongue..."
$PARSELTONGUE_BIN ingest "$OUTPUT_DIR/parseltongue_codebase.dump" 2>&1 | tee "$OUTPUT_DIR/ingest_log.txt"

INGEST_ANALYSIS_END=$(date +%s)
INGEST_ANALYSIS_DURATION=$((INGEST_ANALYSIS_END - ANALYSIS_START))
echo "✅ Parseltongue ingestion completed in ${INGEST_ANALYSIS_DURATION} seconds"

echo ""
echo "📋 Step 3: Generating comprehensive entity analysis..."

# Generate entity listings with timing
ENTITY_START=$(date +%s)

echo "Listing all entities..."
$PARSELTONGUE_BIN list-entities --limit 200 > "$OUTPUT_DIR/all_entities.txt" 2>&1 || echo "Entity listing failed"
ENTITY_COUNT=$(wc -l < "$OUTPUT_DIR/all_entities.txt" 2>/dev/null || echo "0")

echo "Listing functions..."
$PARSELTONGUE_BIN list-entities --type functions --limit 100 > "$OUTPUT_DIR/functions.txt" 2>&1 || echo "Function listing failed"
FUNCTION_COUNT=$(wc -l < "$OUTPUT_DIR/functions.txt" 2>/dev/null || echo "0")

echo "Listing structs..."
$PARSELTONGUE_BIN list-entities --type structs --limit 100 > "$OUTPUT_DIR/structs.txt" 2>&1 || echo "Struct listing failed"
STRUCT_COUNT=$(wc -l < "$OUTPUT_DIR/structs.txt" 2>/dev/null || echo "0")

echo "Listing traits..."
$PARSELTONGUE_BIN list-entities --type traits --limit 100 > "$OUTPUT_DIR/traits.txt" 2>&1 || echo "Trait listing failed"
TRAIT_COUNT=$(wc -l < "$OUTPUT_DIR/traits.txt" 2>/dev/null || echo "0")

echo "Listing enums..."
$PARSELTONGUE_BIN list-entities --type enums --limit 100 > "$OUTPUT_DIR/enums.txt" 2>&1 || echo "Enum listing failed"
ENUM_COUNT=$(wc -l < "$OUTPUT_DIR/enums.txt" 2>/dev/null || echo "0")

ENTITY_END=$(date +%s)
ENTITY_DURATION=$((ENTITY_END - ENTITY_START))
echo "✅ Entity analysis completed in ${ENTITY_DURATION} seconds"

echo ""
echo "🔍 Step 4: Identifying code quality issues..."

QUALITY_START=$(date +%s)

# Run cargo check to identify warnings
echo "Running cargo check for warnings..."
cargo check 2>&1 | tee "$OUTPUT_DIR/cargo_check.txt"

# Run clippy for additional lints
echo "Running clippy for code quality issues..."
cargo clippy --all-targets --all-features 2>&1 | tee "$OUTPUT_DIR/clippy_output.txt"

QUALITY_END=$(date +%s)
QUALITY_DURATION=$((QUALITY_END - QUALITY_START))
echo "✅ Quality analysis completed in ${QUALITY_DURATION} seconds"

echo ""
echo "🎯 Step 5: Analyzing key architectural components..."

ARCH_START=$(date +%s)

# Analyze key components using parseltongue
echo "Analyzing DiscoveryEngine..."
$PARSELTONGUE_BIN where-defined DiscoveryEngine > "$OUTPUT_DIR/discovery_engine_location.txt" 2>&1 || echo "DiscoveryEngine not found"

echo "Analyzing WorkflowOrchestrator..."
$PARSELTONGUE_BIN where-defined WorkflowOrchestrator > "$OUTPUT_DIR/workflow_orchestrator_location.txt" 2>&1 || echo "WorkflowOrchestrator not found"

echo "Analyzing InMemoryIsg..."
$PARSELTONGUE_BIN where-defined InMemoryIsg > "$OUTPUT_DIR/inmemory_isg_location.txt" 2>&1 || echo "InMemoryIsg not found"

# Find entities in key files
echo "Analyzing entities in discovery module..."
$PARSELTONGUE_BIN entities-in-file src/discovery/mod.rs > "$OUTPUT_DIR/discovery_mod_entities.txt" 2>&1 || echo "Discovery mod not found"

echo "Analyzing entities in main CLI..."
$PARSELTONGUE_BIN entities-in-file src/main.rs > "$OUTPUT_DIR/main_entities.txt" 2>&1 || echo "Main file not found"

ARCH_END=$(date +%s)
ARCH_DURATION=$((ARCH_END - ARCH_START))
echo "✅ Architectural analysis completed in ${ARCH_DURATION} seconds"

TOTAL_END=$(date +%s)
TOTAL_DURATION=$((TOTAL_END - START_TIME))

echo ""
echo "🎉 Self-Analysis Complete!"
echo "Total analysis time: ${TOTAL_DURATION} seconds"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "📋 Key Findings:"
echo "  - Analyzed $FILE_COUNT Rust files"
echo "  - Discovered $ENTITY_COUNT entities"
echo "  - Functions: $FUNCTION_COUNT, Structs: $STRUCT_COUNT, Traits: $TRAIT_COUNT, Enums: $ENUM_COUNT"
echo ""
echo "📊 Performance Summary:"
echo "  - File ingestion: ${INGEST_DURATION} seconds"
echo "  - Parseltongue analysis: ${INGEST_ANALYSIS_DURATION} seconds"
echo "  - Entity discovery: ${ENTITY_DURATION} seconds"
echo "  - Quality analysis: ${QUALITY_DURATION} seconds"
echo "  - Architectural analysis: ${ARCH_DURATION} seconds"

# Validate success criteria
if [ $TOTAL_DURATION -lt 30 ]; then  # 30 seconds
    echo "✅ SUCCESS: Self-analysis completed within 30-second target"
else
    echo "⚠️  WARNING: Self-analysis took longer than 30-second target"
fi

echo ""
echo "📝 Use Case Documentation:"
echo "This demonstrates Parseltongue's capability for recursive self-analysis,"
echo "enabling continuous code quality improvement and architectural validation."