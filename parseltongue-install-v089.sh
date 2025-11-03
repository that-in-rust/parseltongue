#!/usr/bin/env bash
set -e

# Parseltongue Unified Install v089 - Agent Games 2025 Release
# This version installs: Binary v0.8.9 + Agent v0.8.9 + Documentation
# Usage: curl -fsSL https://raw.githubusercontent.com/that-in-rust/parseltongue/main/parseltongue-install-v089.sh | bash
# Always use the versioned script name for clarity - no ambiguity about which version you're getting!

INSTALL_VERSION="089"          # Version format: 089 = v0.8.9
BINARY_VERSION="0.8.9"         # Semantic version for binary
AGENT_VERSION="0.8.9"          # Agent version (matches binary)
REPO="that-in-rust/parseltongue"
BINARY_NAME="parseltongue"
ARCH="macos-arm64"
RELEASE_BINARY="parseltongue-v${BINARY_VERSION}-${ARCH}"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Parseltongue Unified Install v${INSTALL_VERSION}                      ║"
echo "║  Binary: v${BINARY_VERSION} | Agent: v${AGENT_VERSION} | Docs: Latest         ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check for existing installation and enable idempotent updates
if [ -f "./${BINARY_NAME}" ]; then
    EXISTING_VERSION=$(./parseltongue --version 2>/dev/null | grep -oE 'parseltongue [0-9.]+' | awk '{print $2}' || echo "unknown")
    echo "📦 Existing installation: v${EXISTING_VERSION}"
    echo "🔄 Update mode: Replacing all files with v${BINARY_VERSION} bundle"
    echo ""
fi

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    echo "   Please run this script from your project's root directory"
    exit 1
fi

# Download binary
echo "📥 Downloading ${RELEASE_BINARY}..."
curl -L "https://github.com/${REPO}/releases/download/v${BINARY_VERSION}/${RELEASE_BINARY}" -o "${BINARY_NAME}"
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
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/agents/parseltongue-ultrathink-isg-explorer.yaml" \
  -o .claude/agents/parseltongue-ultrathink-isg-explorer.yaml

# Verify installation
if ./${BINARY_NAME} --version | grep -q "${BINARY_VERSION}"; then
    echo "✅ Installation complete! (v${INSTALL_VERSION})"
    echo ""
    echo "📦 Installed Bundle v${INSTALL_VERSION}:"
    echo "   ./parseltongue (binary v${BINARY_VERSION})"
    echo "   .claude/.parseltongue/*.md (7 documentation files)"
    echo "   .claude/agents/parseltongue-ultrathink-isg-explorer.yaml (agent v${AGENT_VERSION})"
    echo ""
    echo "📁 Complete file list:"
    echo "   Binary: ./parseltongue"
    echo "   Docs:   .claude/.parseltongue/parseltongue-README.md"
    echo "           .claude/.parseltongue/Parseltonge-SOP.md"
    echo "           .claude/.parseltongue/S01-README-MOSTIMP.md"
    echo "           .claude/.parseltongue/S05-tone-style-guide.md"
    echo "           .claude/.parseltongue/S06-design101-tdd-architecture-principles.md"
    echo "           .claude/.parseltongue/S77-IdiomaticRustPatterns.md"
    echo "   Agent:  .claude/agents/parseltongue-ultrathink-isg-explorer.yaml"
    echo ""
    echo "Next steps:"
    echo "  1. Run: ./${BINARY_NAME} --help"
    echo "  2. Read: cat .claude/.parseltongue/parseltongue-README.md"
    echo "  3. Agent: @parseltongue-ultrathink-isg-explorer (in Claude Code)"
    echo ""
    echo "Start indexing:"
    echo "  ./${BINARY_NAME} pt01-folder-to-cozodb-streamer ./src --db parseltongue.db"
else
    echo "❌ Installation verification failed"
    exit 1
fi
