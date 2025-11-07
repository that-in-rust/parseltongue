#!/bin/bash

# Parseltongue v0.9.0 Install Script
# Automatic dual file export + EntityClass integration

set -e

echo "🐍 Parseltongue v0.9.0 Installer"
echo "=================================="
echo "Features: Automatic dual file export + EntityClass integration"
echo ""

# Check if git repo exists (required for installation)
if [ ! -d ".git" ]; then
    echo "❌ Error: Must be run from a git repository root"
    echo "   This is required for proper ISG analysis functionality"
    exit 1
fi

# Detect platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $PLATFORM in
    darwin)
        if [ "$ARCH" = "arm64" ]; then
            BINARY_URL="https://github.com/that-in-rust/parseltongue/releases/download/v0.9.0/parseltongue"
        else
            echo "❌ Error: macOS x86_64 not supported in this release"
            exit 1
        fi
        ;;
    linux)
        if [ "$ARCH" = "x86_64" ]; then
            BINARY_URL="https://github.com/that-in-rust/parseltongue/releases/download/v0.9.0/parseltongue"
        else
            echo "❌ Error: Linux ARM64 not supported in this release"
            exit 1
        fi
        ;;
    *)
        echo "❌ Error: Platform $PLATFORM not supported"
        exit 1
        ;;
esac

# Download binary as 'parseltongue'
echo "📥 Downloading Parseltongue v0.9.0 for $PLATFORM-$ARCH..."
curl -L -o parseltongue "$BINARY_URL"

# Make executable
chmod +x parseltongue

# Create .claude directories for agents
mkdir -p .claude/.parseltongue
mkdir -p .claude/agents

# Download agent files
echo "📥 Installing ISG Explorer agent..."
curl -L https://raw.githubusercontent.com/that-in-rust/parseltongue/main/.claude/agents/parseltongue-ultrathink-isg-explorer.md \
  -o .claude/agents/parseltongue-ultrathink-isg-explorer.md

# Download documentation
echo "📥 Installing documentation..."
curl -L https://raw.githubusercontent.com/that-in-rust/parseltongue/main/README.md \
  -o .claude/.parseltongue/README.md

# Verify installation
echo ""
echo "✅ Installation complete!"
echo ""
echo "🎯 v0.9.0 Features:"
echo "   • Automatic dual file export (production + test files)"
echo "   • EntityClass integration (CODE/TEST separation)"
echo "   • Progressive disclosure: 5K→30K→60K tokens"
echo "   • Backward compatible with existing scripts"
echo ""
echo "🚀 Quick start:"
echo "   ./parseltongue pt01-folder-to-cozodb-streamer . --db rocksdb:mycode.db"
echo "   ./parseltongue pt02-level01 --output analysis.json --db rocksdb:mycode.db"
echo "   # Creates: analysis.json (production) + analysis_test.json (tests)"
echo ""
echo "🤖 Agent usage:"
echo "   Restart Claude Code, then use: @parseltongue-ultrathink-isg-explorer"
echo ""
echo "📚 Documentation: .claude/.parseltongue/README.md"
