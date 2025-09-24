#!/bin/bash
# Parseltongue LLM Context Generation Script
# Usage: ./generate_llm_context.sh /path/to/codebase [entity_focus]

set -e

CODEBASE_PATH="${1:-$(pwd)}"
ENTITY_FOCUS="${2:-}"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
OUTPUT_DIR="./parseltongue_workspace/llm_context_${TIMESTAMP}"
PARSELTONGUE_BIN="${PARSELTONGUE_BIN:-./target/release/parseltongue_20250924231324}"

echo "🤖 Parseltongue LLM Context Generation"
echo "Codebase: $CODEBASE_PATH"
echo "Focus: ${ENTITY_FOCUS:-"General overview"}"
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

echo "📊 Step 1: Generating entity overview..."
START_TIME=$(date +%s)

# Generate comprehensive entity listings
$PARSELTONGUE_BIN list-entities --limit 200 > "$OUTPUT_DIR/all_entities.txt" 2>/dev/null || echo "Entity listing not available"
$PARSELTONGUE_BIN list-entities --type functions --limit 100 > "$OUTPUT_DIR/functions.txt" 2>/dev/null || echo "Function listing not available"
$PARSELTONGUE_BIN list-entities --type structs --limit 50 > "$OUTPUT_DIR/structs.txt" 2>/dev/null || echo "Struct listing not available"
$PARSELTONGUE_BIN list-entities --type traits --limit 30 > "$OUTPUT_DIR/traits.txt" 2>/dev/null || echo "Trait listing not available"

OVERVIEW_TIME=$(date +%s)
echo "✅ Entity overview completed in $((OVERVIEW_TIME - START_TIME)) seconds"

echo ""
echo "🎯 Step 2: Focused analysis..."

if [ -n "$ENTITY_FOCUS" ]; then
    echo "Analyzing focus entity: $ENTITY_FOCUS"
    
    # Get detailed information about the focused entity
    $PARSELTONGUE_BIN where-defined "$ENTITY_FOCUS" > "$OUTPUT_DIR/focus_definition.txt" 2>/dev/null || echo "Focus entity not found"
    $PARSELTONGUE_BIN blast-radius "$ENTITY_FOCUS" > "$OUTPUT_DIR/focus_impact.txt" 2>/dev/null || echo "Focus impact analysis not available"
    
    FOCUS_TIME=$(date +%s)
    echo "✅ Focused analysis completed in $((FOCUS_TIME - OVERVIEW_TIME)) seconds"
else
    FOCUS_TIME=$OVERVIEW_TIME
    echo "✅ Skipping focused analysis (no entity specified)"
fi

echo ""
echo "📝 Step 3: Generating LLM context document..."

# Generate comprehensive LLM context
{
    echo "# LLM Context: Codebase Analysis"
    echo "Generated: $(date)"
    echo "Codebase: $CODEBASE_PATH"
    if [ -n "$ENTITY_FOCUS" ]; then
        echo "Focus Entity: $ENTITY_FOCUS"
    fi
    echo ""
    
    echo "## Executive Summary"
    TOTAL_ENTITIES=$(wc -l < "$OUTPUT_DIR/all_entities.txt" 2>/dev/null || echo "0")
    FUNCTION_COUNT=$(wc -l < "$OUTPUT_DIR/functions.txt" 2>/dev/null || echo "0")
    STRUCT_COUNT=$(wc -l < "$OUTPUT_DIR/structs.txt" 2>/dev/null || echo "0")
    TRAIT_COUNT=$(wc -l < "$OUTPUT_DIR/traits.txt" 2>/dev/null || echo "0")
    
    echo "- Total Entities: $TOTAL_ENTITIES"
    echo "- Functions: $FUNCTION_COUNT"
    echo "- Structs: $STRUCT_COUNT"
    echo "- Traits: $TRAIT_COUNT"
    echo ""
    
    echo "## Key Functions (Top 20)"
    head -20 "$OUTPUT_DIR/functions.txt" 2>/dev/null || echo "No functions available"
    echo ""
    
    echo "## Key Structs (Top 15)"
    head -15 "$OUTPUT_DIR/structs.txt" 2>/dev/null || echo "No structs available"
    echo ""
    
    echo "## Key Traits (Top 10)"
    head -10 "$OUTPUT_DIR/traits.txt" 2>/dev/null || echo "No traits available"
    echo ""
    
    if [ -n "$ENTITY_FOCUS" ] && [ -f "$OUTPUT_DIR/focus_definition.txt" ]; then
        echo "## Focus Entity: $ENTITY_FOCUS"
        echo "### Definition"
        cat "$OUTPUT_DIR/focus_definition.txt"
        echo ""
        echo "### Impact Analysis"
        head -20 "$OUTPUT_DIR/focus_impact.txt" 2>/dev/null || echo "No impact analysis available"
        echo ""
    fi
    
    echo "## Architecture Patterns"
    echo "Based on entity analysis, this codebase appears to use:"
    
    # Detect common patterns
    if grep -q "Service" "$OUTPUT_DIR/all_entities.txt" 2>/dev/null; then
        echo "- Service pattern (multiple *Service entities found)"
    fi
    if grep -q "Repository\|Repo" "$OUTPUT_DIR/all_entities.txt" 2>/dev/null; then
        echo "- Repository pattern (data access abstraction)"
    fi
    if grep -q "Handler" "$OUTPUT_DIR/all_entities.txt" 2>/dev/null; then
        echo "- Handler pattern (request/response processing)"
    fi
    if grep -q "Builder" "$OUTPUT_DIR/all_entities.txt" 2>/dev/null; then
        echo "- Builder pattern (object construction)"
    fi
    if grep -q "Manager" "$OUTPUT_DIR/all_entities.txt" 2>/dev/null; then
        echo "- Manager pattern (resource management)"
    fi
    
    echo ""
    echo "## LLM Instructions"
    echo "When working with this codebase:"
    echo "1. Focus on the top 20 functions for understanding core functionality"
    echo "2. Key structs represent the main data models"
    echo "3. Traits define the main abstractions and interfaces"
    if [ -n "$ENTITY_FOCUS" ]; then
        echo "4. Pay special attention to $ENTITY_FOCUS and its relationships"
    fi
    echo "5. Use entity names exactly as listed (case-sensitive)"
    echo "6. Consider impact analysis before suggesting changes"
    
    echo ""
    echo "## Quick Reference Commands"
    echo "\`\`\`bash"
    echo "# Find entity definition"
    echo "$PARSELTONGUE_BIN where-defined EntityName"
    echo ""
    echo "# Analyze impact of changes"
    echo "$PARSELTONGUE_BIN blast-radius EntityName"
    echo ""
    echo "# List entities by type"
    echo "$PARSELTONGUE_BIN list-entities --type functions"
    echo "$PARSELTONGUE_BIN list-entities --type structs"
    echo "$PARSELTONGUE_BIN list-entities --type traits"
    echo "\`\`\`"
    
} > "$OUTPUT_DIR/llm_context.md"

CONTEXT_TIME=$(date +%s)
echo "✅ LLM context generated in $((CONTEXT_TIME - FOCUS_TIME)) seconds"

echo ""
echo "📋 Step 4: Creating focused instruction files..."

# Generate specific instruction files for common LLM tasks
{
    echo "# Codebase Analysis Instructions"
    echo ""
    echo "## Context"
    echo "You are analyzing a Rust codebase with the following characteristics:"
    echo "- Total entities: $TOTAL_ENTITIES"
    echo "- Functions: $FUNCTION_COUNT"
    echo "- Structs: $STRUCT_COUNT"
    echo "- Traits: $TRAIT_COUNT"
    echo ""
    echo "## Key Entities to Focus On"
    echo "### Top Functions"
    head -10 "$OUTPUT_DIR/functions.txt" 2>/dev/null | sed 's/^/- /'
    echo ""
    echo "### Top Structs"
    head -8 "$OUTPUT_DIR/structs.txt" 2>/dev/null | sed 's/^/- /'
    echo ""
    echo "### Top Traits"
    head -5 "$OUTPUT_DIR/traits.txt" 2>/dev/null | sed 's/^/- /'
    echo ""
    echo "## Analysis Guidelines"
    echo "1. Use exact entity names from the lists above"
    echo "2. Consider architectural patterns when making suggestions"
    echo "3. Always check impact before suggesting changes"
    echo "4. Focus on the most frequently used entities first"
    echo "5. Maintain Rust idioms and best practices"
} > "$OUTPUT_DIR/analysis_instructions.md"

{
    echo "# Refactor Planning Instructions"
    echo ""
    echo "## Before Making Changes"
    echo "1. Identify the target entity for refactoring"
    echo "2. Run impact analysis: \`$PARSELTONGUE_BIN blast-radius EntityName\`"
    echo "3. Review all affected entities and their relationships"
    echo "4. Plan changes in order of increasing risk"
    echo ""
    echo "## Risk Assessment"
    echo "- Low Risk (1-5 impacts): Standard refactoring approach"
    echo "- Medium Risk (6-20 impacts): Requires comprehensive testing"
    echo "- High Risk (21-50 impacts): Consider incremental approach"
    echo "- Critical Risk (50+ impacts): Architectural review required"
    echo ""
    echo "## Available Entities for Refactoring"
    head -30 "$OUTPUT_DIR/all_entities.txt" 2>/dev/null | sed 's/^/- /'
} > "$OUTPUT_DIR/refactor_instructions.md"

INSTRUCTIONS_TIME=$(date +%s)
echo "✅ Instruction files created in $((INSTRUCTIONS_TIME - CONTEXT_TIME)) seconds"

TOTAL_TIME=$((INSTRUCTIONS_TIME - START_TIME))

echo ""
echo "🎉 LLM Context Generation Complete!"
echo "Total time: ${TOTAL_TIME} seconds"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "📋 Generated Files:"
echo "  - llm_context.md: Comprehensive LLM context document"
echo "  - analysis_instructions.md: Instructions for codebase analysis"
echo "  - refactor_instructions.md: Instructions for refactoring planning"
echo "  - all_entities.txt: Complete entity listing ($TOTAL_ENTITIES entities)"
echo "  - functions.txt: Function entities ($FUNCTION_COUNT functions)"
echo "  - structs.txt: Struct entities ($STRUCT_COUNT structs)"
echo "  - traits.txt: Trait entities ($TRAIT_COUNT traits)"
if [ -n "$ENTITY_FOCUS" ]; then
    echo "  - focus_definition.txt: Definition of focus entity"
    echo "  - focus_impact.txt: Impact analysis of focus entity"
fi
echo ""
echo "🤖 LLM Usage:"
echo "  1. Provide llm_context.md as context to your LLM"
echo "  2. Use analysis_instructions.md for codebase analysis tasks"
echo "  3. Use refactor_instructions.md for refactoring tasks"
echo "  4. Reference entity files for specific entity information"

# Validate success criteria
if [ $TOTAL_TIME -lt 120 ]; then  # 2 minutes = 120 seconds
    echo "✅ SUCCESS: Context generation completed within 2-minute target"
else
    echo "⚠️  WARNING: Context generation took longer than 2-minute target"
fi