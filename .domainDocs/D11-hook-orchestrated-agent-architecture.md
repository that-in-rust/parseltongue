# D11: Hook-Orchestrated Agent Architecture for Parseltongue

**Date**: 2025-10-31
**Status**: Research & Architectural Design
**Decision**: Hook-based deterministic orchestration with AI-driven reasoning layer

---

## ESSENCE (Minto Pyramid Top)

**Core Decision**: Parseltongue will use **PostToolUse hooks for deterministic pipeline orchestration** combined with an **AI agent for contextual reasoning**, creating a hybrid architecture where hooks enforce correct sequencing and the LLM provides intelligent decision-making.

**Why This Matters**: The 6-tool pipeline requires both deterministic control (hooks) and intelligent reasoning (agent). Hooks ensure tools execute in correct order with validation, while the agent handles complex reasoning about code changes using temporal versioning in CozoDB.

**Installation Model**: `curl | bash` downloads binaries + creates `.claude/` structure (agents/, hooks/, commands/) while keeping documentation folders at repo root (.steeringDocs/, .domainDocs/, .prdArchDocs/, .journalDocs/, .langRefDocs/).

---

## SUMMARY (Layer 1: Key Findings)

### 1. Architecture Pattern Selected

**Hook-Orchestrated Agent Pattern**:
- **PostToolUse hooks** (.claude/hooks/) provide deterministic pipeline control
- **Agent** (.claude/agents/parseltongue-reasoning-orchestrator.md) provides AI reasoning
- **Separation of concerns**: Hooks enforce workflow sequencing, agent makes intelligent decisions

### 2. Skills vs Agents Decision

**Selected: Agent (not Skills API)**:
- Skills are API-registered reusable capabilities (external registration required)
- Agents are markdown files with workflow orchestration logic (local, simpler)
- **For MVP (~10 users)**: Agent suffices, Skills add unnecessary API complexity

### 3. Folder Structure Decision

**Doc folders at repo root, .claude/ for orchestration**:
- `.steeringDocs/`, `.domainDocs/`, `.prdArchDocs/`, `.journalDocs/`, `.langRefDocs/` remain at repo root (version-controlled knowledge)
- `.claude/` contains agents/, hooks/, commands/, settings.local.json (orchestration-specific files)
- **Rationale**: Clear separation between project knowledge (docs) and orchestration mechanism (claude)

### 4. Installation Flow Architecture

**curl | bash pattern** (z.sh-style):
- Single command: `curl -sSL https://github.com/you/parseltongue/releases/latest/download/install.sh | bash`
- Downloads: 6 binaries → ~/.parseltongue/bin/
- Creates: .claude/ structure with agent, hooks, settings
- **Does NOT duplicate docs**: Doc folders already in repo

---

## DETAILED LAYERS (Minto Pyramid Expansion)

### Layer 2: Research Process & Methodology

#### 2.1 Initial Research Questions

1. **Skills vs Agents**: What's the difference and which is better for Parseltongue?
2. **Hooks Mechanism**: How do PostToolUse hooks work in Claude Code?
3. **Folder Structure**: Where should .steeringDocs/ live relative to .claude/?
4. **Installation Pattern**: How to implement curl | bash with .claude/ setup?

#### 2.2 Research Sources

- Claude Code documentation (docs.claude.com)
- Analyzed .claude/ folder patterns from examples
- User's essay on "Skills vs. Agents: Choosing the Right Tool" (provided in answers)
- GitHub installation patterns (rustup.rs, z.sh)

---

### Layer 3: Skills vs Agents Analysis

#### 3.1 Skills Defined

**What are Skills?**:
- Reusable, packaged sets of instructions registered via Skills API
- Provide Claude with expert knowledge for specific tasks
- Automatically invoked by AI when relevant to a prompt
- Example: A "code linting" skill that Claude uses when it detects linting needs

**Skills API Structure**:
```typescript
{
  "skill_id": "parseltongue-01",
  "name": "Index Codebase to CozoDB",
  "description": "Parse codebase with tree-sitter...",
  "instructions": "Run folder-to-cozodb-streamer...",
  "input_schema": { "directory": "string", "output_db": "string" }
}
```

**Skills Characteristics**:
- **AI-invoked**: Claude decides when to use them
- **API-managed**: Require external registration
- **Cross-project**: Can be used across multiple projects
- **Discovery**: AI can discover registered skills

#### 3.2 Agents Defined

**What are Agents?**:
- Markdown files with YAML frontmatter in .claude/agents/
- Handle complex, multi-step workflows with reasoning
- Can spawn subagents and coordinate multiple tools
- Example: Parseltongue orchestrator managing 6-tool pipeline

**Agent File Structure**:
```markdown
---
name: parseltongue-reasoning-orchestrator
description: Fix Rust bugs via 6-tool pipeline
tools: Read, Bash, Write, Glob, Grep
model: inherit
---

# Agent Instructions
[Workflow orchestration logic here]
```

**Agent Characteristics**:
- **User-invoked**: `@agent-name` to call explicitly
- **Local files**: No external API needed
- **Project-specific**: Tailored to specific workflow
- **Contextual**: Access project files via Read tool

#### 3.3 Comparison Matrix

| Feature | Skills | Agents |
|---------|--------|--------|
| **Invocation** | AI-driven (automatic) | User-driven (`@agent-name`) |
| **Location** | Skills API (external) | `.claude/agents/` (local) |
| **Complexity** | Single-purpose, parameterized | Multi-step workflows |
| **Discovery** | AI discovers via API | User must know name |
| **Setup** | Requires API registration | Just markdown file |
| **Best For** | Reusable cross-project tasks | Project-specific orchestration |
| **MVP Suitability** | ❌ Adds API complexity | ✅ Simple local setup |

#### 3.4 Decision: Agent for Parseltongue

**Selected: Agent**

**Rationale**:
1. **MVP Simplicity**: ~10 users don't need API infrastructure
2. **Workflow Complexity**: 6-tool pipeline needs multi-step orchestration (agent specialty)
3. **Project-Specific**: Parseltongue workflow is unique, not reusable cross-project
4. **Local Control**: Markdown files easier to debug/iterate than API calls
5. **User Expectation**: Explicit `@agent-parseltongue-reasoning-orchestrator` invocation clearer than implicit skill triggering

**Future Option**: If Parseltongue scales beyond 100 users, could wrap individual tools (folder-to-cozodb-streamer, etc.) as Skills for discoverability while keeping agent for orchestration.

---

### Layer 4: Hooks Architecture

#### 4.1 What Are Hooks?

**Definition**: Hooks are custom scripts (Python/Bash) that execute automatically at specific points in the Claude Code lifecycle.

**Hook Types**:
- **PreToolUse**: Runs before tool execution (validation, logging)
- **PostToolUse**: Runs after tool execution (tracking, pipeline orchestration)
- **SubagentStop**: Runs when subagent completes

**Configuration** (in .claude/settings.local.json):
```json
{
  "hooks": {
    "PostToolUse": [
      ".claude/hooks/pipeline-orchestrator.py",
      ".claude/hooks/tool-state-tracker.py"
    ]
  }
}
```

#### 4.2 Hook Signature (Python Example)

```python
#!/usr/bin/env python3
def on_post_tool_use(tool_name, tool_input, tool_output, exit_code):
    """
    Called after every tool execution

    Args:
        tool_name: Name of the tool that executed
        tool_input: Input parameters to the tool
        tool_output: Output/result from the tool
        exit_code: Exit code (0 = success)
    """
    # Deterministic orchestration logic here
    pass
```

#### 4.3 Hook Use Cases for Parseltongue

**Pipeline Orchestration** (.claude/hooks/pipeline-orchestrator.py):
- **Phase 1 → Phase 2**: Detect `folder-to-cozodb-streamer` completion, log indexing stats
- **Phase 3 loops**: Track `llm-to-cozodb-writer` temporal state changes
- **Phase 4 validation**:
  - `rust-preflight-code-simulator` exit code check → if failed, trigger Phase 3 return
  - `cargo build` exit code check → if failed, trigger Phase 3 return
  - `cargo test` exit code check → if failed, trigger Phase 3 return
- **Phase 6 reset**: Log `cozodb-make-future-code-current` completion

**State Tracking** (.claude/hooks/tool-state-tracker.py):
- Log all parseltongue tool invocations to `.parseltongue/usage.log`
- Track CozoDB state changes for debugging
- Record temporal versioning updates

#### 4.4 Hooks vs Agent Reasoning

**Key Distinction** (from user's essay):

> "Use Hooks to orchestrate Agents for programmatic orchestration: When you need a guaranteed, repeatable sequence of complex operations, Hooks are the perfect conductors for an orchestra of Agents."

**For Parseltongue**:
- **Hooks enforce sequencing**: Tool 1 must complete before Tool 2, validation failures loop back to Phase 3
- **Agent provides reasoning**: LLM decides what code changes to make, generates CozoDB queries, assesses confidence
- **Hooks cannot call Skills**: Skills are AI-invoked, Hooks are deterministic scripts
- **Hooks can trigger Agent re-invocation**: If validation fails, Hook can prompt agent to retry

---

### Layer 5: Folder Structure Architecture

#### 5.1 Initial Design (REJECTED)

**Original Plan**: Move .steeringDocs/ into .claude/steering/

**Rationale for rejection**:
- User feedback: "docs like steeringDocs/ and domainDocs/ journalDocs/ langRefDocs/ ultrathink you are doing fine just change according to this"
- **Insight**: Documentation is project knowledge (version-controlled), not orchestration mechanism

#### 5.2 Final Design (ACCEPTED)

**Doc Folders at Repo Root**:
```
repo/
├── .steeringDocs/       # Project principles, coding conventions, TDD guidance
├── .domainDocs/         # Domain research, architectural decisions
├── .prdArchDocs/        # PRD documentation (P00-P06)
├── .journalDocs/        # Development journals, TDD trackers
├── .langRefDocs/        # Language-specific reference docs
├── .claude/             # Claude Code orchestration files
│   ├── agents/
│   │   └── parseltongue-reasoning-orchestrator.md
│   ├── hooks/
│   │   ├── pipeline-orchestrator.py
│   │   └── tool-state-tracker.py
│   ├── commands/        # Optional: slash commands like /index, /validate
│   └── settings.local.json
├── CLAUDE.md            # Auto-loaded project context
└── [rest of codebase]
```

**Clear Separation**:
- **Knowledge Base** (root-level docs): Version-controlled, project-specific knowledge
- **Orchestration Layer** (.claude/): Workflow automation, hook scripts, agent coordination
- **Installation Segregation**: `curl | bash` script creates .claude/, docs already in repo

#### 5.3 Agent Access to Docs

**Pattern**: Agent uses `Read` tool to access docs

```markdown
# In .claude/agents/parseltongue-reasoning-orchestrator.md

## Available Documentation (Access via Read tool)

- Ultra-minimalist principles: `.steeringDocs/S01-README-MOSTIMP.md`
- Rust conventions: `.steeringDocs/S02-code-conventions.md`
- TDD architecture: `.steeringDocs/S06-design101-tdd-architecture-principles.md`
- PRD documentation: `.prdArchDocs/P06PRDL6AgentTruthSource.md`
```

**No special referencing syntax needed** - just file paths with Read tool.

---

### Layer 6: Installation Flow Architecture

#### 6.1 Installation Command

```bash
curl -sSL https://github.com/your-org/parseltongue/releases/latest/download/install.sh | bash
```

**Pattern**: z.sh-style single-command setup (documented in D06-curl-installation-architecture.md)

#### 6.2 Installation Script Flow

**install.sh responsibilities**:

1. **Detect OS/Architecture**:
   ```bash
   OS=$(uname -s | tr '[:upper:]' '[:lower:]')
   ARCH=$(uname -m)
   # macOS arm64, macOS x64, Linux x64
   ```

2. **Download Binaries**:
   ```bash
   INSTALL_DIR="$HOME/.parseltongue/bin"
   TOOLS=(
       "folder-to-cozodb-streamer"
       "llm-to-cozodb-writer"
       "llm-cozodb-to-context-writer"
       "rust-preflight-code-simulator"
       "llm-cozodb-to-diff-writer"
       "cozodb-make-future-code-current"
   )
   for tool in "${TOOLS[@]}"; do
       curl -sSL "${RELEASE_URL}/${tool}-${PLATFORM}" -o "${INSTALL_DIR}/${tool}"
       chmod +x "${INSTALL_DIR}/${tool}"
   done
   ```

3. **Add to PATH**:
   ```bash
   echo "export PATH=\"\$HOME/.parseltongue/bin:\$PATH\"" >> ~/.zshrc
   ```

4. **Create .claude/ Structure**:
   ```bash
   mkdir -p .claude/{agents,hooks,commands}
   ```

5. **Download Orchestration Files**:
   ```bash
   # Agent markdown
   curl -sSL "${RELEASE_URL}/parseltongue-reasoning-orchestrator.md" \
       -o .claude/agents/parseltongue-reasoning-orchestrator.md

   # Hooks
   curl -sSL "${RELEASE_URL}/pipeline-orchestrator.py" \
       -o .claude/hooks/pipeline-orchestrator.py
   curl -sSL "${RELEASE_URL}/tool-state-tracker.py" \
       -o .claude/hooks/tool-state-tracker.py
   chmod +x .claude/hooks/*.py
   ```

6. **Create settings.local.json**:
   ```bash
   cat > .claude/settings.local.json <<'EOF'
   {
     "permissions": {
       "allow": [
         "Bash(cargo:*)",
         "Bash(folder-to-cozodb-streamer:*)",
         "Bash(llm-to-cozodb-writer:*)",
         "Bash(llm-cozodb-to-context-writer:*)",
         "Bash(rust-preflight-code-simulator:*)",
         "Bash(llm-cozodb-to-diff-writer:*)",
         "Bash(cozodb-make-future-code-current:*)",
         "Bash(git:*)",
         "Read(//**)",
         "Write(//**)"
       ]
     },
     "hooks": {
       "PostToolUse": [
         ".claude/hooks/pipeline-orchestrator.py",
         ".claude/hooks/tool-state-tracker.py"
       ]
     }
   }
   EOF
   ```

7. **Initialize Empty CozoDB**:
   ```bash
   mkdir -p .parseltongue
   touch .parseltongue/parseltongue.db
   ```

8. **Display Success Message**:
   ```bash
   echo "✅ Parseltongue installed successfully!"
   echo "Usage: @parseltongue-reasoning-orchestrator 'Fix panic in GitHub #1234'"
   ```

#### 6.3 What Gets Installed vs What's Already There

**Installed by curl | bash**:
- 6 binaries → `~/.parseltongue/bin/`
- `.claude/agents/parseltongue-reasoning-orchestrator.md`
- `.claude/hooks/pipeline-orchestrator.py`
- `.claude/hooks/tool-state-tracker.py`
- `.claude/settings.local.json`
- `.parseltongue/parseltongue.db` (empty)

**Already in repo (NOT duplicated by installer)**:
- `.steeringDocs/` (version-controlled)
- `.domainDocs/` (version-controlled)
- `.prdArchDocs/` (version-controlled)
- `.journalDocs/` (version-controlled)
- `.langRefDocs/` (version-controlled)
- `CLAUDE.md` (version-controlled)

**Clear Separation**: Installation only adds orchestration layer, not knowledge base.

---

### Layer 7: Hook-Orchestrated Workflow (Deterministic Sequencing)

#### 7.1 Phase 1: ISG Creation

**Tool**: `folder-to-cozodb-streamer`

**Hook Behavior**:
```python
# .claude/hooks/pipeline-orchestrator.py
if 'folder-to-cozodb-streamer' in tool_name and exit_code == 0:
    log_event("Phase 1 complete: Codebase indexed to CozoDB")
    # Extract stats from tool_output
    # Optionally notify agent: "Indexing complete, proceed to Phase 2"
```

**Agent Role**: Invokes Tool 1, reviews indexing analytics

#### 7.2 Phase 2: MicroPRD Refinement

**Tools**: `llm-cozodb-to-context-writer` (context extraction only)

**Hook Behavior**: None (pure AI reasoning phase)

**Agent Role**:
- Extracts context (ISGL1 + interface_signature + TDD_classification + lsp_meta_data)
- **EXCLUDES current_code** to prevent bloat
- Iterates with user to refine micro-PRD (2 iterations)

#### 7.3 Phase 3: Temporal Simulation

**Tools**: `llm-to-cozodb-writer` (write changes), `llm-cozodb-to-context-writer` (read context)

**Hook Behavior**:
```python
if 'llm-to-cozodb-writer' in tool_name and exit_code == 0:
    log_event("Temporal state updated in CozoDB")
    track_state_change(tool_input)  # Log which entities changed
```

**Agent Role**:
- **READ-EDIT-READ-EDIT cycle**:
  - READ: Extract context with `llm-cozodb-to-context-writer`
  - EDIT: Update temporal state with `llm-to-cozodb-writer`
  - READ: Verify changes
  - EDIT: Refine
- **Confidence gate**: Continue loop until confidence ≥80%

**Iteration**: Unlimited A01→A02→B01→B02 cycles until confident

#### 7.4 Phase 4: Pre-Flight & File Writing

**Tools**: `rust-preflight-code-simulator`, `llm-cozodb-to-diff-writer`, `cargo build`, `cargo test`

**Hook Behavior** (Critical deterministic control):
```python
# Syntax validation
if 'rust-preflight-code-simulator' in tool_name:
    if exit_code != 0:
        log_event("Syntax validation FAILED: Return to Phase 3")
        # Hook can inject prompt to agent context suggesting Phase 3 return

# CodeDiff generation
elif 'llm-cozodb-to-diff-writer' in tool_name and exit_code == 0:
    log_event("CodeDiff.json generated")
    if not Path('CodeDiff.json').exists():
        log_event("ERROR: CodeDiff.json missing after Tool 5")

# Build validation
elif tool_name == 'Bash' and 'cargo build' in tool_input:
    if exit_code != 0:
        log_event("Build FAILED: Return to Phase 3")

# Test validation
elif tool_name == 'Bash' and 'cargo test' in tool_input:
    if exit_code != 0:
        log_event("Tests FAILED: Return to Phase 3")
```

**Agent Role**:
- Run Tool 4 (pre-flight syntax check)
- Run Tool 5 (generate CodeDiff.json)
- Read CodeDiff.json and apply changes to files
- Run cargo build + cargo test

**Hook-Enforced Loops**:
- Syntax errors → Phase 3
- Build errors → Phase 3
- Test errors → Phase 3

#### 7.5 Phase 6: State Reset

**Tool**: `cozodb-make-future-code-current`

**Hook Behavior**:
```python
if 'cozodb-make-future-code-current' in tool_name and exit_code == 0:
    log_event("Phase 6 complete: Database state reset")
```

**Agent Role**: Run Tool 6, create git commit

---

### Layer 8: Commands (Optional Power-User Feature)

#### 8.1 Slash Commands in .claude/commands/

**Purpose**: Quick access to individual tools without full agent workflow

**Examples**:
- `/index` - Run `folder-to-cozodb-streamer` on current directory
- `/validate` - Run `rust-preflight-code-simulator` on future_code
- `/reset` - Run `cozodb-make-future-code-current` to reset state

**Structure**:
```markdown
<!-- .claude/commands/index.md -->
---
description: Index current codebase to CozoDB (Tool 1)
---

Run `folder-to-cozodb-streamer` on the current repository:
1. Detect project language (Rust-first)
2. Parse with tree-sitter
3. Generate ISGL1 keys
4. Store in `.parseltongue/parseltongue.db`

Usage: `/index [--dir ./src]`
```

**Installation**: `curl | bash` script can optionally create these

**MVP Decision**: Start without commands (agent-only), add later if users request

---

### Layer 9: Implementation Priorities

#### 9.1 MVP Phase (4 hours - Agent Integration)

**Priority 0 (Blocking)**:
1. Create `.claude/agents/parseltongue-reasoning-orchestrator.md` with YAML frontmatter
2. Create `.claude/hooks/pipeline-orchestrator.py` with PostToolUse hook logic
3. Create `.claude/hooks/tool-state-tracker.py` for logging
4. Update `.claude/settings.local.json` to register hooks
5. Test agent invocation: `@parseltongue-reasoning-orchestrator "test bug"`

**Deliverable**: Working agent that orchestrates 6-tool pipeline with hook-based validation loops

#### 9.2 Post-MVP Enhancements

**Priority 1 (Nice-to-have)**:
- Slash commands (`/index`, `/validate`, `/reset`)
- Output styles (minimal vs verbose)
- Enhanced hook logging with metrics

**Priority 2 (Future scale)**:
- Skills API integration (if user base grows beyond 100 users)
- Cross-project reusability
- Agent marketplace listing

---

## DECISION SUMMARY

### Architecture Selected

**Hook-Orchestrated Agent Pattern**:
- Hooks provide deterministic pipeline control (PostToolUse enforcement)
- Agent provides AI reasoning (contextual decision-making)
- Clear separation: deterministic execution vs intelligent reasoning

### Key Architectural Choices

1. **Agent (not Skills)**: Markdown file in `.claude/agents/`, no API needed
2. **Hooks for Orchestration**: `.claude/hooks/` with Python scripts
3. **Docs at Root**: `.steeringDocs/`, `.domainDocs/`, etc. stay version-controlled
4. **curl | bash Installation**: Downloads binaries + creates `.claude/` structure
5. **No Doc Duplication**: Installer doesn't copy docs (already in repo)

### Implementation Path

**Phase 1 (MVP)**: Agent + Hooks (4 hours)
**Phase 2 (Optional)**: Slash commands (4 hours)
**Phase 3 (Future)**: Skills API integration (if scaling beyond 100 users)

---

## REFERENCES

- D06-curl-installation-architecture.md: z.sh-style installation pattern
- User's essay: "Skills vs. Agents: Choosing the Right Tool for Automation with Claude Code Hooks"
- Claude Code documentation: docs.claude.com (agent structure, hook signatures)
- P06PRDL6AgentTruthSource.md: Existing agent specification (to be adapted)

---

## NEXT STEPS

1. **Implement Agent File**: Create content for `.claude/agents/parseltongue-reasoning-orchestrator.md` based on P06
2. **Implement Hooks**: Write Python scripts for pipeline orchestration and state tracking
3. **Create install.sh**: Write installation script for GitHub releases
4. **Update P00.md**: Add installation structure section (1-liner summaries)
5. **Test Workflow**: Run full agent workflow with hooks on sample bug fix
