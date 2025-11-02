#!/usr/bin/env bash
set -e

# Parseltongue Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/that-in-rust/parseltongue/main/install.sh | bash

REPO="that-in-rust/parseltongue"
BINARY_NAME="parseltongue"
VERSION="0.8.6"

echo "🔧 Installing Parseltongue v${VERSION}..."

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    echo "   Please run this script from your project's root directory"
    exit 1
fi

# Download binary
echo "📥 Downloading binary..."
curl -L "https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY_NAME}" -o "${BINARY_NAME}"
chmod +x "${BINARY_NAME}"

# Create .claude/.parseltongue directory
echo "📁 Creating .claude/.parseltongue/ directory..."
mkdir -p .claude/.parseltongue

# Download documentation
echo "📚 Downloading documentation..."
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/parseltongue-README.md" \
  -o .claude/.parseltongue/parseltongue-README.md

curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/.parseltongue/Parseltonge-SOP.md" \
  -o .claude/.parseltongue/Parseltonge-SOP.md

# Verify installation
if ./${BINARY_NAME} --version | grep -q "${VERSION}"; then
    echo "✅ Installation complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Run: ./${BINARY_NAME} --help"
    echo "  2. Read: cat .claude/.parseltongue/parseltongue-README.md"
    echo "  3. Learn: cat .claude/.parseltongue/Parseltonge-SOP.md"
    echo ""
    echo "Start indexing:"
    echo "  ./${BINARY_NAME} pt01-folder-to-cozodb-streamer ./src --db parseltongue.db"
else
    echo "❌ Installation verification failed"
    exit 1
fi
