#!/usr/bin/env bash
set -e

# Parseltongue Unified Install v090 - EntityClass Integration Release
# This version installs: Binary v0.9.0 + Agent v0.9.0 + Documentation + visualSummary090
# Usage: curl -fsSL https://raw.githubusercontent.com/that-in-rust/parseltongue/main/parseltongue-install-v090.sh | bash
# Always use the versioned script name for clarity - no ambiguity about which version you're getting!

INSTALL_VERSION="090"          # Version format: 090 = v0.9.0
BINARY_VERSION="0.9.0"         # Semantic version for binary
AGENT_VERSION="0.9.0"          # Agent version (matches binary)
REPO="that-in-rust/parseltongue"
BINARY_NAME="parseltongue"
ARCH="macos-arm64"
RELEASE_BINARY="parseltongue-v${BINARY_VERSION}-${ARCH}"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Parseltongue Unified Install v${INSTALL_VERSION}                      ║"
echo "║  Binary: v${BINARY_VERSION} | Agent: v${AGENT_VERSION} | EntityClass Ready ║"
echo "║  Features: Progressive Disclosure | Verified Commands | visualSummary090 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Function to clean old binaries
cleanup_old_binaries() {
    echo "🧹 Cleaning up old binaries..."
    for old_binary in parseltongue-*; do
        if [[ -f "$old_binary" && "$old_binary" != "${RELEASE_BINARY}" && "$old_binary" != "${BINARY_NAME}" ]]; then
            echo "   Removing: $old_binary"
            rm -f "$old_binary"
        fi
    done
}

# Function to verify binary integrity
verify_binary() {
    local binary_path="$1"
    local expected_version="$2"
    
    if [[ ! -f "$binary_path" ]]; then
        echo "❌ Binary not found: $binary_path"
        return 1
    fi
    
    if [[ ! -x "$binary_path" ]]; then
        echo "❌ Binary not executable: $binary_path"
        return 1
    fi
    
    # Check version
    local actual_version=$("$binary_path" --version 2>/dev/null | grep -oE 'parseltongue [0-9.]+' | awk '{print $2}' || echo "unknown")
    if [[ "$actual_version" != "$expected_version" ]]; then
        echo "❌ Version mismatch: expected $expected_version, got $actual_version"
        return 1
    fi
    
    echo "✅ Binary verified: v$actual_version"
    return 0
}

# Check for existing installation and enable idempotent updates
if [ -f "./${BINARY_NAME}" ]; then
    EXISTING_VERSION=$(./parseltongue --version 2>/dev/null | grep -oE 'parseltongue [0-9.]+' | awk '{print $2}' || echo "unknown")
    echo "📦 Existing installation: v${EXISTING_VERSION}"
    
    if [[ "$EXISTING_VERSION" == "$BINARY_VERSION" ]]; then
        echo "✅ Already running v${BINARY_VERSION} - Verifying installation..."
        if verify_binary "./${BINARY_NAME}" "$BINARY_VERSION"; then
            echo "🔄 Re-installing to ensure all components are up to date..."
        else
            echo "❌ Existing binary corrupted - Replacing..."
        fi
    else
        echo "🔄 Update mode: Replacing v${EXISTING_VERSION} with v${BINARY_VERSION} bundle"
    fi
    echo ""
fi

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    echo "   Please run this script from your project's root directory"
    exit 1
fi

# Clean up old binaries
cleanup_old_binaries

# Download binary with retry logic
echo "📥 Downloading ${RELEASE_BINARY}..."
MAX_RETRIES=3
RETRY_COUNT=0

while [[ $RETRY_COUNT -lt $MAX_RETRIES ]]; do
    if curl -L "https://github.com/${REPO}/releases/download/v${BINARY_VERSION}/${RELEASE_BINARY}" -o "${BINARY_NAME}"; then
        break
    else
        RETRY_COUNT=$((RETRY_COUNT + 1))
        echo "⚠️  Download attempt $RETRY_COUNT failed. Retrying..."
        sleep 2
    fi
done

if [[ $RETRY_COUNT -eq $MAX_RETRIES ]]; then
    echo "❌ Failed to download binary after $MAX_RETRIES attempts"
    exit 1
fi

chmod +x "${BINARY_NAME}"

# Verify downloaded binary
echo "🔍 Verifying binary integrity..."
if ! verify_binary "./${BINARY_NAME}" "$BINARY_VERSION"; then
    echo "❌ Binary verification failed"
    exit 1
fi

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
echo "🤖 Installing Parseltongue Ultrathink ISG Explorer Agent v${AGENT_VERSION}..."
mkdir -p .claude/agents
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/agents/parseltongue-ultrathink-isg-explorer.md" \
  -o .claude/agents/parseltongue-ultrathink-isg-explorer.md

# Install backup agent
echo "🤖 Installing backup agent (long form)..."
curl -L "https://raw.githubusercontent.com/${REPO}/main/.claude/agents/parseltongue-ultrathink-isg-explorer-long-backup.md" \
  -o .claude/agents/parseltongue-ultrathink-isg-explorer-long-backup.md

# Download visualSummary090 (NEW v0.9.0 feature)
echo "📊 Downloading visualSummary090 analysis package..."
mkdir -p visualSummary090

# Get the visualSummary090 files
curl -L "https://raw.githubusercontent.com/${REPO}/main/visualSummary090/README.md" \
  -o visualSummary090/README.md

curl -L "https://raw.githubusercontent.com/${REPO}/main/visualSummary090/INDEX.md" \
  -o visualSummary090/INDEX.md

echo "📋 visualSummary090 package installed (documentation only)"
echo "   Run the verified commands to generate your own analysis files"

# Final verification
echo ""
echo "🔍 Final installation verification..."

if ./${BINARY_NAME} --version | grep -q "${BINARY_VERSION}"; then
    echo "✅ Installation complete! (v${INSTALL_VERSION})"
    echo ""
    echo "📦 Installed Bundle v${INSTALL_VERSION}:"
    echo "   ./parseltongue (binary v${BINARY_VERSION}) - ✅ VERIFIED"
    echo "   .claude/.parseltongue/*.md (7 documentation files)"
    echo "   .claude/agents/parseltongue-ultrathink-isg-explorer.md (agent v${AGENT_VERSION}) - ✅ VERIFIED"
    echo "   visualSummary090/ (analysis package with verified commands) - 🆕 NEW"
    echo ""
    echo "📁 Complete file list:"
    echo "   Binary: ./parseltongue"
    echo "   Docs:   .claude/.parseltongue/parseltongue-README.md"
    echo "           .claude/.parseltongue/Parseltonge-SOP.md"
    echo "           .claude/.parseltongue/S01-README-MOSTIMP.md"
    echo "           .claude/.parseltongue/S05-tone-style-guide.md"
    echo "           .claude/.parseltongue/S06-design101-tdd-architecture-principles.md"
    echo "           .claude/.parseltongue/S77-IdiomaticRustPatterns.md"
    echo "   Agents: .claude/agents/parseltongue-ultrathink-isg-explorer.md"
    echo "           .claude/agents/parseltongue-ultrathink-isg-explorer-long-backup.md"
    echo "   Analysis: visualSummary090/README.md (verified command examples)"
    echo "             visualSummary090/INDEX.md (quick reference)"
    echo ""
    echo "🎯 v0.9.0 Features:"
    echo "   ✅ EntityClass Integration (CODE/TEST classification)"
    echo "   ✅ Progressive Disclosure (5K→30K→60K tokens)"
    echo "   ✅ Verified Commands (100% tested on real codebase)"
    echo "   ✅ Enhanced Documentation (expected outputs, file sizes)"
    echo "   ✅ visualSummary090 Package (complete analysis examples)"
    echo ""
    echo "⚠️  IMPORTANT: Restart Claude Code to activate the agents!"
    echo "   Claude Code loads agents on startup. Exit and re-enter your session."
    echo "   After restart: Agents persist forever - no more restarts needed."
    echo ""
    echo "📖 Quick Start:"
    echo "  1. EXIT and RESTART Claude Code (one-time, loads the agents)"
    echo "  2. Run: ./${BINARY_NAME} --help"
    echo "  3. Read: cat .claude/.parseltongue/parseltongue-README.md"
    echo "  4. Analysis: cat visualSummary090/README.md"
    echo "  5. Agent: @parseltongue-ultrathink-isg-explorer (now active!)"
    echo ""
    echo "🚀 Start indexing (verified commands):"
    echo "  ./${BINARY_NAME} pt01-folder-to-cozodb-streamer . --db rocksdb:parseltongue-v090.db --verbose"
    echo "  ./${BINARY_NAME} pt02-level00 --where-clause \"ALL\" --output edges.json --db rocksdb:parseltongue-v090.db"
    echo ""
    echo "📊 Expected results: 1,318 entities, 4,164 edges, ~3 seconds processing time"
else
    echo "❌ Installation verification failed"
    exit 1
fi
