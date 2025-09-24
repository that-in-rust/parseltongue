#!/bin/bash
# Parseltongue Wrapper Script - Copy this to your project

# Set the path to your parseltongue binary
PARSELTONGUE_BIN="./parseltongue"

# If binary doesn't exist locally, try global
if [ ! -f "$PARSELTONGUE_BIN" ]; then
    PARSELTONGUE_BIN="parseltongue"
fi

# Quick aliases for common operations
case "$1" in
    "map"|"onboard")
        echo "🗺️  Mapping codebase..."
        $PARSELTONGUE_BIN onboard .
        ;;
    "find")
        if [ -z "$2" ]; then
            echo "Usage: $0 find <entity_name>"
            exit 1
        fi
        echo "🔍 Finding: $2"
        $PARSELTONGUE_BIN where-defined "$2"
        ;;
    "impact")
        if [ -z "$2" ]; then
            echo "Usage: $0 impact <entity_name>"
            exit 1
        fi
        echo "💥 Impact analysis for: $2"
        $PARSELTONGUE_BIN feature-start "$2"
        ;;
    "safe")
        if [ -z "$2" ]; then
            echo "Usage: $0 safe <target>"
            exit 1
        fi
        echo "🛡️  Safety check for: $2"
        $PARSELTONGUE_BIN refactor-check "$2"
        ;;
    "trace")
        if [ -z "$2" ]; then
            echo "Usage: $0 trace <entity_name>"
            exit 1
        fi
        echo "🔗 Tracing usage: $2"
        $PARSELTONGUE_BIN debug "$2"
        ;;
    "context")
        if [ -z "$2" ]; then
            echo "Usage: $0 context <entity_name>"
            exit 1
        fi
        echo "🤖 Generating LLM context for: $2"
        $PARSELTONGUE_BIN generate-context "$2"
        ;;
    *)
        echo "Parseltongue Quick Commands:"
        echo "  $0 map              - Map the entire codebase"
        echo "  $0 find <entity>    - Find where entity is defined"
        echo "  $0 impact <entity>  - Analyze impact of changes"
        echo "  $0 safe <target>    - Check refactoring safety"
        echo "  $0 trace <entity>   - Trace entity usage"
        echo "  $0 context <entity> - Generate LLM context"
        echo ""
        echo "Or use parseltongue directly: $PARSELTONGUE_BIN --help"
        ;;
esac