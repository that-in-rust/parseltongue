#!/usr/bin/env bash
set -e

# Parseltongue Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/that-in-rust/parseltongue/main/install.sh | bash

REPO="that-in-rust/parseltongue"
BINARY_NAME="parseltongue"
VERSION="0.8.8"
ARCH="macos-arm64"
RELEASE_BINARY="parseltongue-v${VERSION}-${ARCH}"

echo "🔧 Installing Parseltongue v${VERSION}..."

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    echo "   Please run this script from your project's root directory"
    exit 1
fi

# Download binary
echo "📥 Downloading ${RELEASE_BINARY}..."
curl -L "https://github.com/${REPO}/releases/download/v${VERSION}/${RELEASE_BINARY}" -o "${BINARY_NAME}"
chmod +x "${BINARY_NAME}"

# Create .claude/.parseltongue directory
echo "📁 Creating .claude/.parseltongue/ directory..."
mkdir -p .claude/.parseltongue

# Download documentation
echo "📚 Downloading documentation..."

# Core docs
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/parseltongue-README.md" \
  -o .claude/.parseltongue/parseltongue-README.md

curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/Parseltonge-SOP.md" \
  -o .claude/.parseltongue/Parseltonge-SOP.md

# Steering documents
echo "📖 Downloading steering documents..."
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/S01-README-MOSTIMP.md" \
  -o .claude/.parseltongue/S01-README-MOSTIMP.md

curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/S05-tone-style-guide.md" \
  -o .claude/.parseltongue/S05-tone-style-guide.md

curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/S06-design101-tdd-architecture-principles.md" \
  -o .claude/.parseltongue/S06-design101-tdd-architecture-principles.md

curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/S77-IdiomaticRustPatterns.md" \
  -o .claude/.parseltongue/S77-IdiomaticRustPatterns.md

# Install ultrathink ISG explorer agent
echo "🤖 Installing Parseltongue Ultrathink ISG Explorer Agent..."
mkdir -p .claude/agents
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/agents/parseltongue-ultrathink-isg-explorer.md" \
  -o .claude/agents/parseltongue-ultrathink-isg-explorer.md

# Verify installation
if ./${BINARY_NAME} --version | grep -q "${VERSION}"; then
    echo "✅ Installation complete!"
    echo ""
    echo "📁 Installed files:"
    echo "   ./parseltongue (binary)"
    echo "   .claude/.parseltongue/parseltongue-README.md (main docs)"
    echo "   .claude/.parseltongue/Parseltonge-SOP.md (usage guide)"
    echo "   .claude/.parseltongue/S01-README-MOSTIMP.md (core principles)"
    echo "   .claude/.parseltongue/S05-tone-style-guide.md"
    echo "   .claude/.parseltongue/S06-design101-tdd-architecture-principles.md"
    echo "   .claude/.parseltongue/S77-IdiomaticRustPatterns.md"
    echo "   .claude/agents/parseltongue-ultrathink-isg-explorer.md (intelligent agent)"
    echo ""
    echo "Next steps:"
    echo "  1. Run: ./${BINARY_NAME} --help"
    echo "  2. Read: cat .claude/.parseltongue/parseltongue-README.md"
    echo "  3. Learn: cat .claude/.parseltongue/S01-README-MOSTIMP.md"
    echo "  4. Agent: @parseltongue-ultrathink-isg-explorer (in Claude Code)"
    echo ""
    echo "Start indexing:"
    echo "  ./${BINARY_NAME} pt01-folder-to-cozodb-streamer ./src --db parseltongue.db"
else
    echo "❌ Installation verification failed"
    exit 1
fi
