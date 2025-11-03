#!/bin/bash

# Parseltongue Agent Installer
# Ultra-minimalist setup for Claude Code agent (project-local)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
AGENT_DIR=".claude/agents"
AGENT_FILE="parseltongue-reasoning-orchestrator.md"
AGENT_URL="https://raw.githubusercontent.com/that-in-rust/parseltongue/ultrathink/agent-parseltongue-reasoning-orchestrator.md"

echo -e "${CYAN}Parseltongue Agent Installer${NC}"
echo -e "${CYAN}Project-Local Setup${NC}"
echo ""

# Step 1: Create agent directory
if [ ! -d "$AGENT_DIR" ]; then
    echo -e "Creating ${YELLOW}$AGENT_DIR${NC}..."
    mkdir -p "$AGENT_DIR"
    echo -e "${GREEN}✓${NC} Directory created"
else
    echo -e "${GREEN}✓${NC} Directory exists: $AGENT_DIR"
fi

# Step 2: Check if agent already installed
if [ -f "$AGENT_DIR/$AGENT_FILE" ]; then
    echo -e "${YELLOW}⚠${NC}  Agent already installed at:"
    echo "  $AGENT_DIR/$AGENT_FILE"
    echo ""
    read -p "Reinstall? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${CYAN}Installation cancelled.${NC}"
        exit 0
    fi
fi

# Step 3: Download agent markdown
echo -e "Downloading agent markdown..."
if command -v curl &> /dev/null; then
    if curl -fsSL "$AGENT_URL" -o "$AGENT_DIR/$AGENT_FILE"; then
        echo -e "${GREEN}✓${NC} Agent downloaded"
    else
        echo -e "${RED}✗${NC} Download failed. Check your internet connection."
        echo ""
        echo "Manual installation:"
        echo "  1. Download: $AGENT_URL"
        echo "  2. Place in: $AGENT_DIR/$AGENT_FILE"
        exit 1
    fi
else
    echo -e "${RED}✗${NC} curl not found. Please install curl first."
    exit 1
fi

# Step 4: Verify installation
if [ -f "$AGENT_DIR/$AGENT_FILE" ]; then
    FILE_SIZE=$(wc -c < "$AGENT_DIR/$AGENT_FILE" | tr -d ' ')
    if [ "$FILE_SIZE" -gt 1000 ]; then
        echo -e "${GREEN}✓${NC} Installation verified ($FILE_SIZE bytes)"
    else
        echo -e "${RED}✗${NC} Downloaded file seems too small. Installation may have failed."
        exit 1
    fi
fi

# Success message
echo ""
echo -e "${GREEN}✓ Setup complete!${NC}"
echo ""
echo -e "${CYAN}Agent installed to:${NC} $AGENT_DIR/$AGENT_FILE"
echo ""
echo -e "${CYAN}Next steps:${NC}"
echo "  1. Commit to git: ${YELLOW}git add .claude/ && git commit -m \"Add Parseltongue agent\"${NC}"
echo "  2. Open Claude Code"
echo "  3. Type: ${YELLOW}@agent-parseltongue-reasoning-orchestrator${NC}"
echo "  4. Start reasoning!"
echo ""
echo -e "${CYAN}Documentation:${NC}"
echo "  https://github.com/that-in-rust/parseltongue"
