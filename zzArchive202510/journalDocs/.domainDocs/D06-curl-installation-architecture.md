# D06: Curl Installation Architecture

**Pattern**: Single-command installation modeled after z.sh, rustup, and homebrew
**Philosophy**: Per-repository isolation with versioned local documentation
**Date**: 2025-10-31

---

## Core Installation Pattern

### User Command
```bash
curl -sSL https://raw.githubusercontent.com/amuldotexe/parseltongue/main/install.sh | bash
```

**What This Does**:
1. Downloads `install.sh` from GitHub main branch
2. Executes installation script in user's current repository
3. Sets up complete parseltongue environment locally
4. No global installation, no system-wide changes

---

## Folder Structure Created

```
<user-git-repo>/
├── parseltongue                    # Binary executable
├── .claude/
│   ├── agents/
│   │   └── parseltongue-reasoning-orchestrator.md  # Main agent file
│   └── steering/
│       ├── S01-README-MOSTIMP.md                   # MVP ultra-minimalist principles
│       └── S02-code-conventions.md                 # 12-layer idiomatic Rust patterns
└── .parseltongue/
    ├── parseltongue.db             # CozoDB database (created at runtime)
    └── .gitignore                  # Auto-generated to ignore DB file
```

---

## File Inventory & Sources

### 1. Binary (`parseltongue`)
- **Source**: GitHub Releases (latest tag)
- **Platform**: Detects macOS arm64 initially, expandable to Linux/Windows
- **Location**: Repository root (or optionally ~/.local/bin/)
- **Permissions**: chmod +x applied automatically

### 2. Agent File (`.claude/agents/parseltongue-reasoning-orchestrator.md`)
- **Source**: GitHub repo `/agents/parseltongue-reasoning-orchestrator.md`
- **Purpose**: Main orchestration logic for Claude Code
- **Contains**:
  - Phase-by-phase workflow instructions
  - Tool capability matrix (what each tool can/cannot do)
  - @ references to steering docs
  - Critical decision points and loops
  - Ultra-minimalist MVP principles inline

### 3. Steering Documents (`.claude/steering/`)

#### S01-README-MOSTIMP.md
- **Source**: GitHub repo `/.steeringDocs/S01-README-MOSTIMP.md`
- **Purpose**: MVP ultra-minimalist philosophy
- **Key Content**:
  - Target users: ~10 people
  - Tool 5 & 6 minimalist rules (NO backups, NO config complexity)
  - Context bloat prevention principles
  - Simplicity-over-features mandate

#### S02-code-conventions.md (or S07-idiomatic-rust-patterns.md)
- **Source**: GitHub repo `/.steeringDocs/S02-code-conventions.md`
- **Purpose**: Comprehensive idiomatic Rust patterns
- **Key Content**:
  - 12-layer Rust architecture (L1: Core → L12: Domain patterns)
  - Ownership, lifetimes, Result/Option patterns
  - Functional programming idioms
  - thiserror/anyhow usage guidelines
  - Performance-conscious patterns

### 4. Runtime Directory (`.parseltongue/`)
- **Created**: By install script (empty initially)
- **Purpose**: Houses CozoDB database and temporary files
- **Gitignored**: Auto-added to .gitignore to prevent DB commits

---

## @ Reference System

### How It Works
Agent markdown files can reference local files using `@` syntax:

```markdown
## Phase 3, Step B: Code Generation

**CRITICAL**: Apply @.claude/steering/S02-code-conventions.md when generating future_code

LLM will:
1. Read micro-PRD and CodeGraphContext.json
2. Generate Rust code following @.claude/steering/S02-code-conventions.md (12 layers)
3. Verify idiomatic patterns against @.claude/steering/S02 during rubber duck debugging
```

### Benefits
1. **Fresh Context**: LLM reads steering docs on-demand, not pre-loaded
2. **Versioned Knowledge**: Steering docs match binary version (no drift)
3. **Concise Agent**: Orchestration logic stays minimal, references external knowledge
4. **Local Override**: Users can customize .claude/steering/ for project-specific conventions

---

## Installation Flow (Detailed)

### Step 1: Environment Detection
```bash
# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

# Validate: Currently supports macOS arm64
if [ "$OS" != "Darwin" ] || [ "$ARCH" != "arm64" ]; then
    echo "❌ Only macOS arm64 supported currently"
    exit 1
fi
```

### Step 2: Download Binary
```bash
# Fetch latest release tag
LATEST_TAG=$(curl -s https://api.github.com/repos/amuldotexe/parseltongue/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Download binary for platform
BINARY_URL="https://github.com/amuldotexe/parseltongue/releases/download/${LATEST_TAG}/parseltongue-${OS}-${ARCH}"
curl -L -o parseltongue "$BINARY_URL"
chmod +x parseltongue
```

### Step 3: Create Folder Structure
```bash
# Create directories
mkdir -p .claude/agents
mkdir -p .claude/steering
mkdir -p .parseltongue

# Add .parseltongue/ to .gitignore
if ! grep -q ".parseltongue/" .gitignore 2>/dev/null; then
    echo ".parseltongue/" >> .gitignore
fi
```

### Step 4: Download Agent & Steering Docs
```bash
# Download agent file
curl -L -o .claude/agents/parseltongue-reasoning-orchestrator.md \
    "https://raw.githubusercontent.com/amuldotexe/parseltongue/main/agents/parseltongue-reasoning-orchestrator.md"

# Download steering docs
curl -L -o .claude/steering/S01-README-MOSTIMP.md \
    "https://raw.githubusercontent.com/amuldotexe/parseltongue/main/.steeringDocs/S01-README-MOSTIMP.md"

curl -L -o .claude/steering/S02-code-conventions.md \
    "https://raw.githubusercontent.com/amuldotexe/parseltongue/main/.steeringDocs/S02-code-conventions.md"
```

### Step 5: Verify Installation
```bash
# Check binary works
./parseltongue --version

# Print success message
echo "✅ Parseltongue installed successfully!"
echo ""
echo "📁 Files created:"
echo "   - ./parseltongue (binary)"
echo "   - ./.claude/agents/parseltongue-reasoning-orchestrator.md"
echo "   - ./.claude/steering/S01-README-MOSTIMP.md"
echo "   - ./.claude/steering/S02-code-conventions.md"
echo "   - ./.parseltongue/ (runtime directory)"
echo ""
echo "🚀 Usage:"
echo "   In Claude Code: @agent-parseltongue-reasoning-orchestrator 'Fix panic in #1234'"
```

---

## Architectural Rationale

### Why Curl | Bash Pattern?

**Proven by z.sh, rustup, homebrew**:
- ✅ Single command installation (friction-free onboarding)
- ✅ Always downloads latest version
- ✅ No package manager dependencies
- ✅ Cross-platform expandable (macOS → Linux → Windows)
- ✅ Version pinning possible (curl specific tag)

### Why .claude/ Folder Structure?

**Per-Repository Isolation**:
- ✅ Different projects can use different parseltongue versions
- ✅ Steering docs match binary version (no drift)
- ✅ Project-specific customizations possible
- ✅ No global state pollution
- ✅ Claude Code auto-discovers .claude/agents/

**Subfolders Rationale**:
- `agents/` - Where Claude Code expects agent markdown files
- `steering/` - Domain knowledge documents (@ referenceable)
- Clean separation of orchestration vs knowledge

### Why .parseltongue/ for Runtime?

**Isolation Benefits**:
- ✅ Database (parseltongue.db) separate from source code
- ✅ Gitignored by default (no accidental commits)
- ✅ Tool-specific namespace (no .cache/ or .config/ pollution)
- ✅ Easy cleanup (`rm -rf .parseltongue/` resets state)

### Why @ References Instead of Inline?

**Maintainability**:
- ✅ Agent.md stays concise (orchestration only)
- ✅ Steering docs updatable independently
- ✅ LLM gets fresh context per invocation
- ✅ Users can override with project-specific patterns
- ✅ No duplication between agent and docs

**Example Comparison**:
```markdown
❌ BAD (inline):
## Step B01: Code Generation
Apply these Rust patterns:
1. Use Result<T, E> for recoverable errors
2. Use Option<T> for nullable values
3. Accept &str, return String
4. Use thiserror for library errors
5. Use anyhow for application errors
... (200 more lines of patterns)

✅ GOOD (@ reference):
## Step B01: Code Generation
Apply @.claude/steering/S02-code-conventions.md (12-layer Rust patterns)
```

---

## Version Management Strategy

### Binary Versioning
- GitHub Releases use semantic versioning: v0.1.0, v0.2.0, etc.
- Install script defaults to latest tag
- Users can pin version: `curl ... | PARSELTONGUE_VERSION=v0.1.0 bash`

### Steering Doc Versioning
- Steering docs downloaded from same tag as binary
- Ensures compatibility (binary v0.1.0 → steering docs from v0.1.0 branch/tag)
- Breaking changes in docs → minor version bump

### Upgrade Path
```bash
# Re-run install script (overwrites binary + docs)
curl -sSL https://raw.githubusercontent.com/amuldotexe/parseltongue/main/install.sh | bash

# Or manual:
rm parseltongue .claude/steering/* .parseltongue/*
curl ... | bash
```

---

## Security Considerations

### Curl | Bash Risks
- ❌ Executes remote code (mitigated by GitHub repo trust)
- ✅ Users can inspect install.sh before running
- ✅ Checksum verification possible (future enhancement)
- ✅ No sudo required (local installation only)

### Mitigation Strategy
1. **Transparent Script**: install.sh is simple, auditable (< 100 lines)
2. **No Sudo**: Never requests elevated privileges
3. **Idempotent**: Safe to re-run (overwrites existing files)
4. **GitHub Trust**: Hosted on verified amuldotexe/parseltongue repo

---

## Future Enhancements

### Platform Expansion
- [ ] Linux x86_64 support
- [ ] Linux arm64 support (Raspberry Pi, AWS Graviton)
- [ ] Windows WSL2 support
- [ ] Native Windows PowerShell installer

### Advanced Features
- [ ] Checksum verification (SHA256 of binary)
- [ ] Version pinning via environment variable
- [ ] Offline installation (pre-downloaded tarball)
- [ ] Auto-update command (`parseltongue update`)
- [ ] Uninstall script (`curl ... | bash -s uninstall`)

### Configuration Options
- [ ] Custom installation path (PARSELTONGUE_INSTALL_DIR)
- [ ] Skip steering docs download (SKIP_STEERING=1)
- [ ] Custom steering docs URL (STEERING_REPO)

---

## Comparison to Alternatives

| Approach | Pros | Cons |
|----------|------|------|
| **Curl \| Bash** (Our choice) | Single command, version control, proven pattern | Requires internet, trust in script |
| **Homebrew** | Managed updates, familiar to Mac users | macOS-only, adds dependency, global install |
| **Cargo Install** | Rust ecosystem native, builds from source | Slow (compilation), requires Rust toolchain |
| **Manual Download** | No script execution | Multi-step, error-prone, no auto-setup |
| **Git Clone** | Full repo access | Large download, manual binary build, no structure |

---

## Related Documents

- **S01-README-MOSTIMP.md**: MVP ultra-minimalist principles (installed to .claude/steering/)
- **S02-code-conventions.md**: 12-layer Rust patterns (installed to .claude/steering/)
- **P00.md**: Complete system workflow diagram (shows @ reference integration)
- **install.sh**: Actual installation script (to be created in repo root)
- **agents/parseltongue-reasoning-orchestrator.md**: Main agent file (to be created)

---

## Usage Example (End-to-End)

```bash
# User navigates to their Rust project
cd ~/projects/my-rust-app

# Install parseltongue with single command
curl -sSL https://raw.githubusercontent.com/amuldotexe/parseltongue/main/install.sh | bash

# Output:
# ✅ Parseltongue installed successfully!
# 📁 Files created: ...
# 🚀 Usage: ...

# Open Claude Code in this repo
# Invoke agent
> @agent-parseltongue-reasoning-orchestrator "Fix panic in GitHub #1234"

# Agent runs, reading:
# - @.claude/steering/S01-README-MOSTIMP.md (MVP principles)
# - @.claude/steering/S02-code-conventions.md (Rust patterns during code gen)
# - Executes 6-tool pipeline
# - Creates .parseltongue/parseltongue.db
# - Generates code following idiomatic Rust patterns
# - Commits fix with git

# Done! No manual config, no global install pollution
```

---

**Key Insight**: This architecture treats documentation as versioned code artifacts, installed locally with the tool, ensuring capability clarity and preventing LLM confusion about what each tool can do.
