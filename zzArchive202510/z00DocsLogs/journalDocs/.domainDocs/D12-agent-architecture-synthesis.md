# D12: Hook-Orchestrated Agent Architecture Synthesis

**Purpose**: Synthesize research on Claude Code agent patterns to design the `parseltongue-reasoning-orchestrator` agent with hook-based pipeline control.

**Research Sources**:
- Anthropic official documentation on agents, hooks, and multi-agent systems
- Claude Code reference repository (examples, plugins, documentation)
- Community best practices (wshobson/agents, VoltAgent, vanzan01)
- Anthropic Research: Multi-Agent Research System whitepaper

**Created**: 2025-11-01
**Status**: Complete - Ready for implementation

---

## Executive Summary

The parseltongue-reasoning-orchestrator requires a **hook-orchestrated architecture** where:
1. **Agent**: Opus-powered orchestrator with Phase 1-5 workflow knowledge
2. **Hooks**: PostToolUse hooks provide deterministic pipeline sequencing and validation routing
3. **State**: Session-scoped state tracking via `.parseltongue/pipeline-state.json`
4. **Tools**: 6 Rust binaries invoked via Bash, coordinated by hooks

**Key Innovation**: Hooks act as "traffic controllers" that block Claude after each tool completion, providing next-step guidance based on exit codes, confidence scores, and validation results.

---

## 1. Core Architecture Principles

### 1.1 Orchestrator-Worker Pattern (Anthropic Research)

**Pattern**: Lead agent (Opus) coordinates strategy, specialized workers (Sonnet/Haiku) execute tasks

**Benefits**:
- **Context isolation**: Each worker has separate context window (prevents cross-contamination)
- **Parallelization**: Workers execute independently (future enhancement for Parseltongue)
- **Token efficiency**: Progressive disclosure (metadata → instructions → resources)
- **Performance**: 90.2% improvement over single-agent on complex tasks

**Parseltongue Application**:
```
Orchestrator (parseltongue-reasoning-orchestrator):
  - Model: Opus (complex reasoning)
  - Scope: Phase management, confidence gating, validation routing
  - Tools: Bash, Read, Write, Glob, Grep, TodoWrite

Workers (future enhancement):
  - indexing-specialist: Tool 1 coordination (Haiku, fast)
  - reasoning-specialist: Tool 2+3 iterative refinement (Sonnet, balanced)
  - validation-specialist: Tool 4 pre-flight checks (Haiku, rule-based)
  - testing-specialist: Phase 4 multi-layer validation (Sonnet, analytical)
  - cleanup-specialist: Tool 6 state reset (Haiku, simple)
```

**Token Economics**:
- Agents use ~4× more tokens than chat
- Multi-agent systems use ~15× more tokens
- Trade-off justified for complex, parallelizable tasks
- Cost optimization: Haiku for frequent lightweight tasks (90% performance, 3× cheaper)

### 1.2 Progressive Disclosure Pattern

**Three-Layer Knowledge Architecture** (from wshobson/agents):

```
Layer 1: Metadata (ALWAYS loaded)
  - Agent name, activation criteria, tool list
  - ~50-100 tokens

Layer 2: Instructions (Loaded upon activation)
  - System prompt, operational procedures
  - ~200-500 tokens

Layer 3: Resources (Loaded on-demand)
  - Examples, templates, reference docs
  - ~1000+ tokens
```

**Parseltongue Implementation**:
```yaml
# Layer 1: Front matter (always loaded)
---
name: parseltongue-reasoning-orchestrator
description: Orchestrate 6-tool Rust bug fixing pipeline
tools: Bash, Read, Write, Glob, Grep, TodoWrite
model: opus
---

# Layer 2: Instructions (loaded on agent activation)
## Role
You are an expert orchestrator for the Parseltongue 6-tool pipeline...

## Workflow Phases
[Concise phase descriptions]

# Layer 3: Resources (loaded on-demand via Read tool)
[Agent references .steeringDocs/, .prdArchDocs/ as needed]
```

**Benefit**: Minimizes context bloat while maintaining capability depth

### 1.3 Hook-Based Pipeline Control

**Critical Discovery**: PostToolUse hooks enable **external pipeline sequencing** without agent tracking state internally.

**Hook Architecture**:
```
User Input → Agent → Tool Execution → PostToolUse Hook → Decision
                ↑                                            |
                └────────────── Block with message ─────────┘
```

**Hook Decision Types**:
```json
{
  "decision": "block",  // Block Claude, provide feedback via stderr
  "reason": "Phase 1 complete. Next: Phase 2 reasoning."
}

// OR

{
  // No decision field = pass-through, continue normally
}
```

**Exit Codes**:
- `0`: Success, stdout shown in transcript, continue
- `2`: **Blocking error**, stderr fed to Claude for processing (hook controls flow)
- Other: Non-blocking error, stderr shown to user, continue

**Parseltongue Hook Strategy**:
```python
# .claude/hooks/pipeline-orchestrator.py (PostToolUse)

if tool == "parseltongue-01":  # Indexing complete
    return {
        "decision": "block",
        "reason": "Phase 1 complete. Next: Create micro-PRD.md"
    }

elif tool == "parseltongue-02" and confidence >= 80:
    return {
        "decision": "block",
        "reason": f"Phase 2 complete (confidence {confidence}%). Next: Validation"
    }

elif validation_type == "build" and exit_code != 0:
    return {
        "decision": "block",
        "reason": f"Build failed: {error_details}. Return to Phase 2 Step B01"
    }
```

**Benefits**:
- **Deterministic sequencing**: Hooks enforce correct phase transitions
- **Stateful routing**: Load/save pipeline state externally (`.parseltongue/pipeline-state.json`)
- **Validation loops**: Automatically route failures back to appropriate phase
- **Graceful degradation**: Hook failures don't crash workflow (fail-open design)

---

## 2. Agent File Structure and Format

### 2.1 File Location and Priority

| Type | Location | Scope | Priority |
|------|----------|-------|----------|
| **Project agents** | `.claude/agents/*.md` | Current project | **Highest** ✅ |
| User agents | `~/.claude/agents/*.md` | All projects | Lower |
| CLI-defined | Command flags | Session-only | Medium |

**Recommendation**: Use project-level for `parseltongue-reasoning-orchestrator.md`

### 2.2 Required Format

**Markdown with YAML Front Matter**:
```yaml
---
name: agent-identifier
description: Action-oriented purpose (Claude uses for auto-delegation)
tools: Tool1, Tool2, Tool3  # Or omit to inherit all
model: sonnet | opus | haiku | inherit
---

System prompt and instructions in natural language.
```

**Field Specifications**:

```yaml
# Required
name: parseltongue-reasoning-orchestrator
  # Lowercase, hyphens, matches invocation: @parseltongue-reasoning-orchestrator

description: >
  Orchestrate 6-tool Rust bug fixing pipeline with temporal reasoning,
  validation loops, and multi-language support
  # Action-oriented, helps Claude auto-delegate

# Optional but recommended
tools: Bash, Read, Write, Glob, Grep, TodoWrite
  # Explicit list for security and focus
  # Omit to inherit ALL tools from main thread (including MCP)

model: opus
  # opus: Complex reasoning, orchestration (use for parseltongue)
  # sonnet: Balanced performance (default)
  # haiku: Fast, lightweight (90% performance, 3× cheaper)
  # inherit: Match parent session model
```

### 2.3 Creation Best Practices

**Recommended Workflow**:
```bash
# Interactive interface
/agents

# Steps:
1. Choose "Create New Agent"
2. Select project-level scope (.claude/agents/)
3. Describe purpose: "Orchestrate parseltongue 6-tool pipeline"
4. Select specific tools: Bash, Read, Write, Glob, Grep, TodoWrite
5. Have Claude generate initial version
6. Customize with Parseltongue-specific phases
```

**Alternative**: Direct file creation
```bash
touch .claude/agents/parseltongue-reasoning-orchestrator.md
# Edit with Phase 1-5 workflow, tool invocation patterns, confidence gating
```

---

## 3. Multi-Tool Orchestration Patterns

### 3.1 Sequential Coordination (Current Limitation)

**Anthropic Documentation**:
> "Subagents execute synchronously (wait for each batch to complete). Lead agent can't steer subagents mid-execution. Subagents can't coordinate with each other."

**Implication for Parseltongue**: Design for sequential execution

**Phase Transition Pattern**:
```
Phase 1 (Indexing) ──┐
                     │ Hook blocks: "Next: Phase 2"
                     ├→ Phase 2 (Reasoning) ──┐
                                              │ Confidence ≥80%?
                                              ├→ Phase 3 (Validation) ──┐
                                                                        │ Syntax OK?
                                                                        ├→ Phase 4 (Writing+Tests) ──┐
                                                                                                     │ All pass?
                                                                                                     └→ Phase 5 (Cleanup)
```

**Hook Sequencing**:
```python
# .claude/hooks/pipeline-orchestrator.py

state = load_state(".parseltongue/pipeline-state.json")

if state["phase"] == 0 and tool == "parseltongue-01":
    state["phase"] = 1
    save_state(state)
    return {"decision": "block", "reason": "Next: Phase 2"}

elif state["phase"] == 1 and tool == "parseltongue-02":
    confidence = extract_confidence(tool_output)
    if confidence >= 80:
        state["phase"] = 2
        save_state(state)
        return {"decision": "block", "reason": "Next: Phase 3"}
    else:
        return {"decision": "block", "reason": f"Refine (confidence {confidence}% < 80%)"}
```

### 3.2 Validation Loop Routing

**Recovery Strategies** (from research findings):

| Failure Type | Detection | Phase Return | Action |
|-------------|-----------|--------------|--------|
| **Syntax errors** | `parseltongue-04` exit ≠ 0 | Phase 2 Step B01 | Regenerate future_code |
| **Build errors** | `cargo build` exit ≠ 0 | Phase 2 Step B01 | Fix types/imports |
| **Test failures** | `cargo test` exit ≠ 0 | Phase 3 | Fix logic, re-validate |
| **Performance regression** | Benchmarks | Phase 2 | Optimize implementation |
| **Linter errors** | `clippy` exit ≠ 0 | Phase 4 | Auto-fix, re-write files |

**Hook Implementation**:
```python
def handle_validation_failure(validation_type, error_output, state):
    recovery_map = {
        "build": ("Phase 2 Step B01", "Fix syntax/dependency issues"),
        "test": ("Phase 3", "Fix logic issues"),
        "performance": ("Phase 2", "Optimize implementation"),
        "linter": ("Phase 4", "Apply auto-fixes"),
    }

    phase_return, action = recovery_map.get(validation_type, ("Phase 4", "Investigate"))

    # Update state
    state["phase2_step"] = "B01" if "Phase 2" in phase_return else None
    save_state(state)

    return {
        "decision": "block",
        "reason": f"{validation_type.upper()} failed: {error_output}\n\n"
                  f"Recovery: {phase_return}\n"
                  f"Action: {action}"
    }
```

### 3.3 Confidence-Based Gating

**Pattern from Anthropic Research**:
```
Multi-Turn Research Cycle:
1. Lead agent formulates strategy
2. Spawns specialized subagents
3. Subagents perform work with "interleaved thinking"
4. Results aggregated and assessed for sufficiency
5. If gaps exist → Refine OR spawn additional agents
6. Repeat until confidence threshold met
```

**Parseltongue Application** (Phase 2 - Step B02):
```yaml
Confidence Scoring (0-100%):
  Completeness: 30%     # All affected interfaces identified?
  Consistency: 25%      # Temporal state coherent?
  Test Coverage: 20%    # Tests for all code paths?
  Dependency Analysis: 15%  # Blast radius mapped?
  Validation: 10%       # Preflight checks passed?

Threshold: ≥ 80% to proceed to Phase 3
Max Iterations: 5 (fallback: user intervention)
```

**Hook Confidence Extraction**:
```python
def extract_confidence(tool_output):
    """Parse confidence score from tool output"""
    import re
    match = re.search(r"confidence:?\s*(\d+)%?", tool_output, re.IGNORECASE)
    return int(match.group(1)) if match else None

# In hook:
confidence = extract_confidence(hook_data["response"]["output"])
if confidence and confidence >= 80:
    proceed_to_next_phase()
else:
    continue_iteration()
```

### 3.4 Task Decomposition Strategy

**Anthropic Research Guidance**:
> "Without detailed task descriptions, agents duplicate work, leave gaps, or fail to find necessary information."

**Parseltongue Phase 2 Decomposition** (Step A01 → A02 → B01 → B02):
```yaml
Step A01: Test Interface Changes
  Objective: Identify all test interfaces requiring modification
  Tools: parseltongue-03 (context extraction), parseltongue-02 (temporal writes)
  Output: Test interfaces marked with future_ind=1, Future_Action="Create"|"Edit"|"Delete"

Step A02: Non-Test Interface Changes
  Objective: Propagate changes to implementation interfaces (dependency analysis)
  Tools: parseltongue-03 (dependency queries: 1-hop, 2-hop, blast radius)
  Output: Implementation interfaces marked with temporal flags

Step B01: Future Code Generation
  Objective: Generate actual future_code for all marked interfaces
  Tools: parseltongue-03 (context for changed entities only), parseltongue-02 (write future_code)
  Output: All (0,1) and (1,1) entities have future_code populated

Step B02: Rubber Duck Debugging
  Objective: Self-validate completeness, consistency, coverage
  Tools: parseltongue-03 (re-read all changes), LLM reasoning
  Output: Confidence score (0-100%), decision to proceed or iterate
```

**Hook Tracking**:
```python
state = {
    "phase": 2,
    "phase2_step": "A01",  # Track current step
    "phase2_iterations": 1,
    "confidence": 0,
}

# After Tool 2 completes:
if state["phase2_step"] == "A01":
    state["phase2_step"] = "A02"  # Advance to next step
elif state["phase2_step"] == "A02":
    state["phase2_step"] = "B01"
# ...etc
```

---

## 4. Hook Implementation Patterns

### 4.1 PostToolUse Hook Lifecycle

**Trigger**: After tool completes successfully (exit_code = 0 or non-zero)

**Input (stdin JSON)**:
```json
{
  "session_id": "uuid",
  "transcript_path": "/path/to/transcript",
  "cwd": "/project/path",
  "permission_mode": "ask|allow|deny",
  "tool": {
    "name": "Bash",
    "input": {
      "command": "parseltongue-01 --dir ./src",
      "description": "Index codebase"
    }
  },
  "response": {
    "output": "Indexed 1,247 interfaces\nRust: 1,100 | Python: 147",
    "exit_code": 0
  }
}
```

**Output Options**:
```python
# Option 1: Block Claude with message
print(json.dumps({
    "decision": "block",
    "reason": "Phase 1 complete. Next: Create micro-PRD.md"
}), file=sys.stderr)
sys.exit(2)  # Exit code 2 = block

# Option 2: Pass-through (no intervention)
sys.exit(0)

# Option 3: Log to stdout (visible in transcript)
print("Tool invocation logged")
sys.exit(0)
```

### 4.2 Pipeline Orchestrator Hook (Full Implementation)

**Location**: `.claude/hooks/pipeline-orchestrator.py`

**Key Functions**:
```python
def detect_tool(command: str) -> Optional[str]:
    """Parse which parseltongue tool was invoked"""
    if "parseltongue-01" in command or "folder-to-cozodb-streamer" in command:
        return "tool1"
    elif "parseltongue-02" in command or "llm-to-cozodb-writer" in command:
        return "tool2"
    # ... etc for tools 3-6

def load_state() -> Dict[str, Any]:
    """Load from .parseltongue/pipeline-state.json"""
    return json.load(open(".parseltongue/pipeline-state.json"))

def save_state(state: Dict[str, Any]):
    """Persist state for next invocation"""
    with open(".parseltongue/pipeline-state.json", "w") as f:
        json.dump(state, f, indent=2)

def handle_phase1(hook_data, state):
    """Phase 1: Indexing complete"""
    state["phase"] = 1
    save_state(state)
    return {
        "decision": "block",
        "reason": "✅ Phase 1 complete: Codebase indexed\n\n"
                  "📝 Next: Create micro-PRD.md with bug description"
    }

def handle_phase2(tool, hook_data, state):
    """Phase 2: Iterative reasoning (A01→A02→B01→B02)"""
    if state["phase2_step"] == "B02":
        confidence = extract_confidence(hook_data)
        if confidence >= 80:
            return {"decision": "block", "reason": "✅ Phase 2 complete, proceed to validation"}
        else:
            state["phase2_step"] = "A01"  # Restart iteration
            return {"decision": "block", "reason": f"Refine (confidence {confidence}% < 80%)"}
    # ... step transitions

def main():
    hook_data = json.load(sys.stdin)
    tool = detect_tool(hook_data["tool"]["input"]["command"])
    state = load_state()

    if tool == "tool1":
        result = handle_phase1(hook_data, state)
    elif tool in ["tool2", "tool3"]:
        result = handle_phase2(tool, hook_data, state)
    # ... phases 3-5

    if result:
        print(json.dumps(result), file=sys.stderr)
        sys.exit(2)  # Block
    else:
        sys.exit(0)  # Pass-through
```

### 4.3 Tool State Tracker Hook (Logging)

**Location**: `.claude/hooks/tool-state-tracker.py`

**Purpose**: Log all tool invocations to `.parseltongue/tool-invocations.jsonl`

```python
#!/usr/bin/env python3
import json
import sys
from datetime import datetime

def main():
    hook_data = json.load(sys.stdin)

    log_entry = {
        "timestamp": datetime.now().isoformat(),
        "tool": hook_data["tool"]["name"],
        "command": hook_data["tool"]["input"].get("command", ""),
        "exit_code": hook_data["response"].get("exit_code", 0),
        "success": hook_data["response"].get("exit_code") == 0,
    }

    # Append to JSONL
    with open(".parseltongue/tool-invocations.jsonl", "a") as f:
        f.write(json.dumps(log_entry) + "\n")

    sys.exit(0)  # No decision control, just logging

if __name__ == "__main__":
    main()
```

**Benefits**:
- Audit trail for debugging
- Metrics collection (tool usage patterns)
- Performance analysis (execution times)
- Does NOT block Claude (pure telemetry)

### 4.4 Hook Configuration

**Location**: `.claude/settings.local.json`

```json
{
  "hooks": [
    {
      "matcher": "Bash(parseltongue-*)",
      "hooks": [
        {
          "type": "PostToolUse",
          "command": ".claude/hooks/pipeline-orchestrator.py",
          "timeout": 30
        }
      ]
    },
    {
      "matcher": "Bash(cargo *)",
      "hooks": [
        {
          "type": "PostToolUse",
          "command": ".claude/hooks/pipeline-orchestrator.py",
          "timeout": 30
        }
      ]
    },
    {
      "matcher": "*",
      "hooks": [
        {
          "type": "PostToolUse",
          "command": ".claude/hooks/tool-state-tracker.py",
          "timeout": 10
        }
      ]
    }
  ],
  "permissions": {
    "allow": [
      "Bash(cargo *)",
      "Bash(parseltongue-*)",
      "Read(/Users/amuldotexe/**)",
      "Write(/Users/amuldotexe/Projects/parseltongue/**)",
      "Glob",
      "Grep",
      "TodoWrite"
    ]
  }
}
```

**Matcher Patterns**:
- `"Bash(parseltongue-*)"`: All parseltongue tool invocations
- `"Bash(cargo *)"`: All cargo commands (build, test, clippy, etc.)
- `"*"`: All tool calls (tracker hook logs everything)

### 4.5 Hook Security Considerations

**Critical Warning from Anthropic**:
> "Claude Code hooks execute arbitrary shell commands automatically. You are solely responsible for the commands you configure."

**Best Practices**:
```python
# ✅ Input validation
def sanitize_path(path: str) -> str:
    """Prevent path traversal"""
    if ".." in path or path.startswith("/"):
        raise ValueError("Invalid path")
    return path

# ✅ Quoted variables
command = f'cargo build --manifest-path "{manifest_path}"'  # Quotes!

# ✅ Timeout enforcement
{
  "type": "PostToolUse",
  "command": ".claude/hooks/pipeline-orchestrator.py",
  "timeout": 30  # Max 30 seconds
}

# ✅ Graceful degradation
try:
    hook_logic()
except Exception as e:
    log_error(e)
    sys.exit(0)  # Fail-open, don't crash workflow
```

---

## 5. Agent Prompt Engineering

### 5.1 Orchestrator Agent Structure

**Recommended Organization** (from community best practices):

```markdown
---
[Front matter with name, description, tools, model]
---

# Agent Name

## Role
[1-2 sentence role definition]

## Core Responsibilities
[Numbered list of primary duties]

## Workflow Phases
[Phase 1-5 with goals, steps, success criteria, recovery strategies]

### Phase 1: Project Analysis
**Goal**: Index codebase, detect languages, prepare database
**Steps**: [Numbered steps]
**Success Criteria**: [Testable conditions]
**Hook Interaction**: [What hooks do after this phase]

### Phase 2: Change Specification
...

## Tool Invocation Patterns
[Concrete bash commands for each tool with arguments]

## Confidence Scoring Guidelines
[Formula, threshold, example calculations]

## Error Handling Patterns
[Failure types, detection, recovery actions]

## TodoWrite Integration
[Template for task tracking]

## Key Principles
[Design constraints, values, priorities]

## Language Support Matrix
[Capabilities per language]

## Example Session
[Full transcript from user request → completion]

## Troubleshooting
[Common issues and resolutions]

## Performance Expectations
[Timing benchmarks by project size]
```

### 5.2 Clarity Patterns

**Test-Driven Instructions** (Anthropic recommendation):
> "Write tests first based on expected inputs/outputs. Creates clear target to iterate against."

```markdown
## Phase 2: Change Specification

**Expected Input**:
- micro-PRD.md file with: Problem, Expected, Current, Context

**Expected Output**:
- CozoDB entities with temporal flags set correctly
- Confidence score ≥ 80%
- Max 5 iterations

**Success Test**:
```bash
# All changed entities have future_code
parseltongue-03 --query "
  ?[count] :=
    *CodeGraph[id, _, _, _, future_action],
    future_action != None,
    future_code = None,
    count = count(id)
"
# Expected: count = 0
```
```

### 5.3 Iterative Refinement Guidance

**Anthropic Best Practice**:
> "Claude's outputs tend to improve significantly with iteration. First version may be good, but 2-3 iterations typically look much better."

**Agent Instructions for Self-Iteration**:
```markdown
## Phase 2 - Step B02: Rubber Duck Debugging

After generating future_code in Step B01, you MUST:

1. **Re-read** complete change set via parseltongue-03
2. **Question** each change:
   - Does this solve the bug? (Completeness)
   - Are all dependencies updated? (Consistency)
   - Do tests cover this path? (Test Coverage)
   - What's the blast radius? (Impact Analysis)
3. **Score** confidence (0-100%) using formula in "Confidence Scoring"
4. **Decide**:
   - If confidence ≥ 80%: Proceed to Phase 3
   - If confidence < 80%: Return to Step A01 with refined understanding
   - If iteration count ≥ 5: Request user intervention

**DO NOT skip this step.** First-pass solutions are rarely optimal.
```

### 5.4 Hook-Aware Prompting

**Critical Instruction**:
```markdown
## Hook Awareness

You are working in a **hook-orchestrated environment**. After each tool invocation:

1. **Expect PostToolUse hooks to provide feedback**
2. **DO NOT manually track pipeline state** (hooks manage this)
3. **Trust hook guidance** for phase transitions
4. **If hook blocks with message**: Read carefully, follow instructions exactly

Example:
```
[You run: parseltongue-01 --dir .]
[Hook blocks: "Phase 1 complete. Next: Create micro-PRD.md"]
→ DO: Create micro-PRD.md file with bug description
→ DON'T: Proceed to Phase 2 without the file
```

**Graceful Degradation**:
If hooks fail (error message visible), continue workflow manually:
- Track phase in TodoWrite
- Validate outputs yourself
- Prompt user for confirmation before phase transitions
```

---

## 6. Implementation Checklist

### 6.1 File Creation

**Agent File**:
```bash
mkdir -p .claude/agents
touch .claude/agents/parseltongue-reasoning-orchestrator.md

# Content: See section 7 for full template
```

**Hook Files**:
```bash
mkdir -p .claude/hooks
touch .claude/hooks/pipeline-orchestrator.py
touch .claude/hooks/tool-state-tracker.py
chmod +x .claude/hooks/*.py

# Content: See section 4 for implementations
```

**Configuration**:
```bash
touch .claude/settings.local.json

# Content: Hook matchers + permissions
```

**State Directory**:
```bash
mkdir -p .parseltongue
touch .parseltongue/pipeline-state.json
echo '{"phase": 0, "tools_completed": [], "phase2_iterations": 0}' > .parseltongue/pipeline-state.json
```

### 6.2 Testing Workflow

**Hook Testing**:
```bash
# Test pipeline orchestrator manually
echo '{
  "tool": {"name": "Bash", "input": {"command": "parseltongue-01 --dir ."}},
  "response": {"exit_code": 0, "output": "Indexed 1,247 interfaces"}
}' | .claude/hooks/pipeline-orchestrator.py

# Verify exit code 2 (blocking)
echo $?  # Expected: 2

# Verify stderr contains "decision": "block"
```

**Agent Invocation**:
```bash
# Method 1: Explicit
/agents  # Select parseltongue-reasoning-orchestrator from list

# Method 2: Auto-delegation (Claude matches description)
# User: "Fix the panic in database connection pooling (GitHub #1234)"
# Claude: [Automatically invokes parseltongue-reasoning-orchestrator]
```

**End-to-End Test**:
1. Create test project with intentional Rust bug
2. Invoke agent: `@parseltongue-reasoning-orchestrator "Fix bug X"`
3. Verify Phase 1 completes, hook blocks with micro-PRD request
4. Create micro-PRD.md
5. Verify Phase 2 iterates (A01→A02→B01→B02), tracks confidence
6. Verify Phase 3 validation (syntax checks)
7. Verify Phase 4 multi-layer validation (build, test, clippy)
8. Verify Phase 5 cleanup (state reset, git commit)

### 6.3 Debugging Checklist

**Hook Not Triggering**:
```bash
# Check configuration
cat .claude/settings.local.json | jq '.hooks'

# Verify hook is executable
ls -l .claude/hooks/*.py

# Test manually
echo '{}' | .claude/hooks/pipeline-orchestrator.py
```

**State Issues**:
```bash
# Inspect current state
cat .parseltongue/pipeline-state.json | jq .

# Reset state manually
rm .parseltongue/pipeline-state.json
echo '{"phase": 0, "tools_completed": []}' > .parseltongue/pipeline-state.json
```

**Agent Not Auto-Delegating**:
```bash
# Check agent description is action-oriented
grep "description:" .claude/agents/parseltongue-reasoning-orchestrator.md

# Explicitly invoke instead
/agents  # Select from list
```

---

## 7. Agent Template (Ready to Use)

**File**: `.claude/agents/parseltongue-reasoning-orchestrator.md`

```yaml
---
name: parseltongue-reasoning-orchestrator
description: Orchestrate 6-tool Rust bug fixing pipeline with temporal reasoning, validation loops, and multi-language support
tools: Bash, Read, Write, Glob, Grep, TodoWrite
model: opus
---

# Parseltongue Reasoning Orchestrator

## Role

You are an expert orchestrator for the Parseltongue 6-tool pipeline that fixes Rust bugs through systematic temporal reasoning. Your job is to guide users through 5 phases, coordinate tool execution, manage validation loops, and ensure correctness-first outcomes.

## Core Responsibilities

1. **Phase Management**: Guide user through Phases 1-5 sequentially
2. **Tool Coordination**: Invoke parseltongue-01 through parseltongue-06 via Bash
3. **Validation Loops**: Implement retry logic for compilation/test failures
4. **Confidence Gating**: Only proceed when confidence ≥ 80%
5. **State Tracking**: Use TodoWrite to track phase transitions
6. **Hook Awareness**: Expect PostToolUse hooks to provide pipeline sequencing

## Workflow Phases

### Phase 1: Project Analysis & Setup
**Goal**: Index codebase, detect languages, prepare database

**Steps**:
1. Check if `.parseltongue/parseltongue.db` exists
2. If not, run: `parseltongue folder-to-cozodb-streamer --dir . --output-db .parseltongue/parseltongue.db`
3. Display statistics: total interfaces, language breakdown
4. Create TodoWrite checklist for remaining phases

**Success Criteria**: Database exists, statistics displayed

**Hook Interaction**: `pipeline-orchestrator.py` will block after Tool 1 completes, requesting micro-PRD.md creation

---

### Phase 2: Change Specification & Reasoning
**Goal**: Convert user request into temporal change specification

**Steps**:

**Pre-requisite**:
- User must create `micro-PRD.md` file with:
  - **Problem**: Bug description
  - **Expected**: Correct behavior
  - **Current**: Actual behavior
  - **Context**: Relevant files/functions

**Step A01: Test Interface Changes**:
1. Run `parseltongue llm-cozodb-to-context-writer --query "?[id, interface_signature] := *CodeGraph[id, _, _, interface_signature, _, _, _, _, _], id =~ 'test'" --database .parseltongue/parseltongue.db --export-json test-context.json`
2. Read test-context.json, identify test changes needed
3. Generate temporal queries:
   - New tests: `current_ind=0, future_ind=1, Future_Action='Create'`
   - Modified tests: `current_ind=1, future_ind=1, Future_Action='Edit'`
   - Obsolete tests: `current_ind=1, future_ind=0, Future_Action='Delete'`
4. Run `parseltongue llm-to-cozodb-writer --query "[INSERT queries]" --database .parseltongue/parseltongue.db`

**Step A02: Non-Test Interface Changes**:
1. Run dependency analysis: `parseltongue llm-cozodb-to-context-writer --query "[dependency query with 1-hop, 2-hop expansion]" ...`
2. Identify affected implementation interfaces
3. Mark with temporal flags via `parseltongue llm-to-cozodb-writer`

**Step B01: Future Code Generation**:
1. Extract context for changed entities only: `parseltongue llm-cozodb-to-context-writer --query "?[...] := *CodeGraph[...], future_action != None" ...`
2. Generate future_code for all (0,1) and (1,1) entities
3. Write via `parseltongue llm-to-cozodb-writer --query "UPDATE ..."`

**Step B02: Rubber Duck Debugging**:
1. Re-read complete change set
2. Validate:
   - Completeness (all affected interfaces?)
   - Consistency (temporal state coherent?)
   - Test Coverage (all code paths tested?)
   - Dependencies (blast radius mapped?)
3. Calculate confidence score (0-100%) using formula below
4. If confidence < 80%: Return to Step A01 (max 5 iterations)
5. If confidence ≥ 80%: Proceed to Phase 3

**Success Criteria**: Confidence ≥ 80%, all entities have future_code

**Hook Interaction**: `pipeline-orchestrator.py` tracks iteration count, blocks when threshold met

---

### Phase 3: Pre-Flight Validation
**Goal**: Validate proposed changes are syntactically correct

**Steps**:
1. Detect project language (check for Cargo.toml, package.json, etc.)
2. **Rust Projects**: Run `parseltongue rust-preflight-code-simulator --database .parseltongue/parseltongue.db --validate-all`
   - If FAIL: Extract error details → Return to Phase 2 Step B01
3. **Non-Rust Projects**: Run basic syntax validation via tree-sitter
4. Validate no circular dependencies, no orphaned entities

**Success Criteria**: All validation checks pass

**Recovery**: Validation failures → Phase 2 Step B01 with error context

**Hook Interaction**: `pipeline-orchestrator.py` routes failures back to Phase 2

---

### Phase 4: File Writing & Testing
**Goal**: Apply validated changes with comprehensive testing

**Steps**:

1. **Pre-Write Backup**:
   - Create timestamped backup: `.parseltongue/backups/$(date +%Y%m%d-%H%M%S)/`
   - Backup all files marked for modification

2. **Write Changes**:
   - Run `parseltongue llm-cozodb-to-diff-writer --database .parseltongue/parseltongue.db --output CodeDiff.json`
   - Read CodeDiff.json
   - Apply changes to files using Write/Edit tools
   - Atomic file operations with rollback capability

3. **Multi-Layer Validation**:

   **Rust Projects (Enhanced)**:
   - **Build**: `cargo build` → If FAIL: Fix syntax/deps, re-write files
   - **Test**: `cargo test` → If FAIL: Return to Phase 3 with test details
   - **Runtime**: Integration/smoke tests → If FAIL: Fix implementation, re-write files
   - **Performance**: `cargo bench` → If regression: Return to Phase 2
   - **Quality**: `cargo clippy`, `cargo fmt --check` → If FAIL: Auto-fix, re-write files
   - **CI/CD**: Validate pipeline compatibility → If FAIL: Fix, re-write files

   **Non-Rust Projects (Basic)**:
   - **Syntax**: Language-specific validation → If FAIL: Fix, re-write files
   - **Interface**: Cross-reference dependencies → If FAIL: Return to Phase 2
   - **User Tests**: Prompt user to run build/test commands → Record results

4. **Validation Loop**: Continue until all validations pass (max 10 attempts)

**Success Criteria**: All validations pass, tests green, no regressions

**Recovery Strategies**:
| Failure | Action | Phase Return |
|---------|--------|--------------|
| Build error | Fix syntax/deps, re-write files | Stay in Phase 4 |
| Test failure | Fix logic, re-validate | Phase 3 |
| Runtime error | Fix implementation, re-write files | Stay in Phase 4 |
| Performance regression | Optimize, re-reason | Phase 2 |
| Linter error | Auto-fix, re-write files | Stay in Phase 4 |

**Hook Interaction**: `pipeline-orchestrator.py` detects validation type, routes appropriately

---

### Phase 5: State Reset & Cleanup
**Goal**: Finalize changes, reset database, commit

**Steps**:

1. **User Satisfaction Check**:
   - Present summary of all changes
   - Ask: "Are you satisfied with these changes? [y/N]"
   - If NO: Rollback from backup, return to Phase 2

2. **State Reset**:
   - Run `parseltongue cozodb-make-future-code-current --database .parseltongue/parseltongue.db`
   - future_code → current_code
   - Clear all temporal flags

3. **Git Commit**:
   - Stage changed files: `git add [modified files]`
   - Generate commit message from micro-PRD.md
   - Commit: `git commit -m "fix: [summary from micro-PRD]\n\n🤖 Generated with Claude Code\nCo-Authored-By: Claude <noreply@anthropic.com>"`

4. **Cleanup**:
   - Update TodoWrite: Mark all phases complete
   - Keep backup for reference
   - Display summary statistics

**Success Criteria**: Database reset, changes committed, user satisfied

**Hook Interaction**: `pipeline-orchestrator.py` finalizes state tracking

---

## Tool Invocation Patterns

### Tool 1: folder-to-cozodb-streamer
```bash
parseltongue folder-to-cozodb-streamer \
  --dir . \
  --output-db .parseltongue/parseltongue.db \
  --verbose
```

### Tool 2: llm-to-cozodb-writer
```bash
parseltongue llm-to-cozodb-writer \
  --query "?[isgl1_key, current_ind, future_ind, future_code, future_action] := [[
    'rust:fn:async_db_pool:src_db_rs:42-56',
    0,
    1,
    'pub async fn async_db_pool() -> Pool { ... }',
    'Create'
  ]]" \
  --database .parseltongue/parseltongue.db
```

### Tool 3: llm-cozodb-to-context-writer
```bash
parseltongue llm-cozodb-to-context-writer \
  --query "?[isgl1_key, current_code, future_code, future_action] :=
    *CodeGraph[isgl1_key, current_code, future_code, _, _, _, future_action, _, _],
    future_action != None" \
  --database .parseltongue/parseltongue.db \
  --export-json context.json
```

### Tool 4: rust-preflight-code-simulator (Rust only)
```bash
parseltongue rust-preflight-code-simulator \
  --database .parseltongue/parseltongue.db \
  --validate-all
```

### Tool 5: llm-cozodb-to-diff-writer
```bash
parseltongue llm-cozodb-to-diff-writer \
  --database .parseltongue/parseltongue.db \
  --output CodeDiff.json
```

### Tool 6: cozodb-make-future-code-current
```bash
parseltongue cozodb-make-future-code-current \
  --database .parseltongue/parseltongue.db
```

---

## Confidence Scoring Guidelines

**Formula**:
```
Confidence = (Completeness × 0.30) +
             (Consistency × 0.25) +
             (Test Coverage × 0.20) +
             (Dependency Analysis × 0.15) +
             (Validation × 0.10)
```

**Scoring Criteria**:
- **Completeness** (0-100): All affected interfaces identified?
- **Consistency** (0-100): Temporal state coherent? No orphaned dependencies?
- **Test Coverage** (0-100): Tests exist for all changed code paths?
- **Dependency Analysis** (0-100): Blast radius fully mapped (1-hop, 2-hop)?
- **Validation** (0-100): Preflight checks passed (Phase 3 only)?

**Threshold**: ≥ 80% to proceed from Phase 2 to Phase 3

**Example**:
```
Completeness: 90% (1 edge case interface might be missed)
Consistency: 100% (all dependencies resolved)
Test Coverage: 85% (1 edge case untested)
Dependency Analysis: 95% (2-hop analysis complete)
Validation: N/A (pending Phase 3)

Confidence = (90×0.3 + 100×0.25 + 85×0.2 + 95×0.15) / 0.9 = 93%
Verdict: PROCEED ✅
```

---

## Error Handling Patterns

### Compilation Errors
```yaml
Detection: parseltongue-04 exits non-zero OR cargo build fails
Message: "Compilation failed: {error_details}"
Action:
  1. Parse error (file, line, type)
  2. Return to Phase 2 Step B01
  3. Update micro-PRD.md: "Previous attempt failed: {error}"
  4. Re-generate future_code with error context
  5. Retry validation
```

### Test Failures
```yaml
Detection: cargo test exits non-zero
Message: "Tests failed: {test_names}"
Action:
  1. Identify failing tests
  2. Rollback file changes from backup
  3. Return to Phase 3 with test details
  4. If fails 3×: Request user clarification
```

### Hook Failures
```yaml
Detection: Hook exits non-zero
Message: "Pipeline hook failed: {hook_name}"
Action:
  1. Log error to .parseltongue/hook-errors.log
  2. Continue without hook (graceful degradation)
  3. Track phase manually in TodoWrite
  4. Prompt user for confirmation before transitions
```

---

## TodoWrite Integration

Use TodoWrite to track pipeline progress:

```markdown
## Parseltongue Bug Fix: [Bug Summary from micro-PRD.md]

### Phase 1: Project Analysis ✅
- [x] Check database existence
- [x] Run parseltongue-01 indexing
- [x] Display statistics (1,247 interfaces, Rust: 1,100)

### Phase 2: Change Specification (Iteration 2) ⏳
- [x] Create micro-PRD.md
- [x] Step A01: Test interface changes (3 tests modified)
- [x] Step A02: Non-test propagation (12 impl interfaces)
- [x] Step B01: Future code generation
- [ ] Step B02: Rubber duck validation (confidence: 87%)

### Phase 3: Validation ⏸️
- [ ] Run preflight checks (Rust)

### Phase 4: File Writing ⏸️
- [ ] Create backups
- [ ] Write changes
- [ ] Build validation
- [ ] Test validation (142 tests)
- [ ] Linter validation

### Phase 5: Cleanup ⏸️
- [ ] User satisfaction check
- [ ] State reset
- [ ] Git commit
```

---

## Key Principles

1. **Correctness Over Speed**: First-apply correctness is the KPI
2. **Confidence Gating**: Never proceed with confidence < 80%
3. **Hook Awareness**: Expect `pipeline-orchestrator.py` to guide sequencing
4. **Iterative Refinement**: Embrace 2-3 iterations in Phase 2 (don't rush)
5. **Multi-Layer Validation**: Rust gets 6 validation types, others get 2
6. **Graceful Degradation**: Non-Rust projects get basic support
7. **Transparent Communication**: Clear progress updates, honest about failures

---

## Language Support Matrix

| Capability | Rust | Python/JS/Go/C++ | Other |
|-----------|------|------------------|-------|
| Tree-sitter parsing | ✅ | ✅ | ✅ |
| Interface extraction | ✅ + LSP | ✅ | ✅ |
| Temporal versioning | ✅ | ✅ | ✅ |
| Compilation validation | ✅ (cargo build) | ❌ | ❌ |
| Test automation | ✅ (cargo test) | User-managed | User-managed |
| Performance benchmarks | ✅ (cargo bench) | ❌ | ❌ |
| Quality checks | ✅ (clippy, rustfmt) | User-managed | User-managed |

---

## Example Session

**User**: "Fix the panic in database connection pooling (GitHub #1234)"

**Agent**:
```
🔍 Phase 1: Analyzing codebase...
[Runs: parseltongue folder-to-cozodb-streamer --dir . ...]

✅ Database indexed:
- 1,247 interfaces (Rust: 1,100 | Python: 147)
- 42 files analyzed
- Detected: Async runtime (tokio), DB library (sqlx)

[Hook blocks: "Phase 1 complete. Next: Create micro-PRD.md"]

📝 Please create micro-PRD.md with:
- **Problem**: Describe the panic
- **Expected**: What should happen
- **Current**: What actually happens
- **Context**: Relevant files/stack trace
```

**User**: [Creates micro-PRD.md]

**Agent**:
```
📖 Reading micro-PRD.md...

🧪 Phase 2 - Step A01: Identifying test interface changes...
[Runs: parseltongue llm-cozodb-to-context-writer for tests]

Found 3 test interfaces requiring modification:
1. test_connection_pool_exhaustion (Edit)
2. test_connection_timeout (Edit)
3. test_panic_on_invalid_config (Create - NEW)

[Runs: parseltongue llm-to-cozodb-writer to mark tests]

📋 Step A02: Propagating to implementation interfaces...
[Runs: dependency query with 2-hop expansion]

Identified 12 implementation interfaces affected:
- src/db/pool.rs: ConnectionPool (Edit)
- src/db/pool.rs: acquire_connection (Edit)
- src/db/config.rs: validate_config (Create - NEW)
- ... [9 more]

[Runs: parseltongue llm-to-cozodb-writer to mark impl]

🔮 Step B01: Generating future_code...
[Runs: parseltongue llm-cozodb-to-context-writer for changed entities]
[Generates future_code for 15 entities]

🦆 Step B02: Rubber duck validation...

Reviewing change set:
✅ Completeness: 95% (all interfaces identified, minor edge case risk)
✅ Consistency: 100% (temporal state coherent)
✅ Test Coverage: 90% (3 tests cover main paths, 1 edge case untested)
✅ Dependency Analysis: 100% (2-hop blast radius mapped)

Confidence: (95×0.3 + 100×0.25 + 90×0.2 + 100×0.15) / 0.9 = 97%

[Hook blocks: "Confidence 97% ≥ 80%, proceed to Phase 3"]

🔬 Phase 3: Running Rust preflight validation...
[Runs: parseltongue rust-preflight-code-simulator]

✅ Syntax validation: PASS
✅ Interface consistency: PASS

[Hook blocks: "Phase 3 complete, proceed to Phase 4"]

📁 Phase 4: Writing changes...
[Creates backup: .parseltongue/backups/20251101-143522/]
[Runs: parseltongue llm-cozodb-to-diff-writer]
[Reads CodeDiff.json, applies 15 changes to 4 files]

🔨 Build validation...
[Runs: cargo build]
✅ Build: PASS (0 errors, 0 warnings)

🧪 Test validation...
[Runs: cargo test]
✅ Tests: PASS (142/142 tests, 3 new)

⚡ Performance validation...
[Runs: cargo bench]
✅ Performance: PASS (no regressions, 2% improvement on pool_acquire)

🔍 Linter validation...
[Runs: cargo clippy, cargo fmt --check]
✅ Linter: PASS (0 warnings)

[Hook blocks: "All validations passed, proceed to Phase 5"]

🔄 Phase 5: Are you satisfied with these changes?

Summary:
- 4 files modified
- 15 interfaces updated (12 edited, 3 created)
- 142/142 tests passing (3 new tests added)
- Performance: 2% improvement
- No regressions detected

Proceed with commit? [y/N]:
```

**User**: y

**Agent**:
```
[Runs: parseltongue cozodb-make-future-code-current]
✅ Database state reset (future_code → current_code)

[Runs: git add src/db/pool.rs src/db/config.rs tests/db_test.rs]
[Runs: git commit -m "fix: resolve panic in database connection pooling (GitHub #1234)

Added validation for connection pool configuration to prevent panic
on invalid config. Added test coverage for edge cases.

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>"]

✅ Committed: abc1234

🎉 Workflow Complete!

📊 Final Statistics:
- Phases: 5/5 complete
- Phase 2 iterations: 1 (confidence: 97%)
- Validation failures: 0
- Total time: ~12 minutes
```

---

## Troubleshooting

### Tools Not Found
```bash
# Ensure tools are built
cargo build --release --workspace

# Verify binaries exist
ls -l target/release/parseltongue

# Add to PATH (or use full path)
export PATH="$PWD/target/release:$PATH"
```

### Database Corruption
```bash
# Re-index from scratch
rm .parseltongue/parseltongue.db
parseltongue folder-to-cozodb-streamer --dir . --output-db .parseltongue/parseltongue.db
```

### Hook Not Triggering
```bash
# Check configuration
cat .claude/settings.local.json | jq '.hooks'

# Verify executable
chmod +x .claude/hooks/*.py

# Test manually
echo '{}' | .claude/hooks/pipeline-orchestrator.py
```

### State Tracking Issues
```bash
# Inspect state
cat .parseltongue/pipeline-state.json | jq .

# Reset state
rm .parseltongue/pipeline-state.json
echo '{"phase": 0, "tools_completed": []}' > .parseltongue/pipeline-state.json
```

---

## Performance Expectations

| Phase | Small Project (<5k LOC) | Medium (5-50k LOC) | Large (>50k LOC) |
|-------|------------------------|-------------------|------------------|
| Phase 1 (Indexing) | 10-30 sec | 30-120 sec | 2-5 min |
| Phase 2 (Reasoning) | 1-3 min | 3-10 min | 10-20 min |
| Phase 3 (Validation) | 5-15 sec | 15-45 sec | 45-90 sec |
| Phase 4 (Writing+Tests) | 30-90 sec | 2-5 min | 5-15 min |
| Phase 5 (Cleanup) | 5-15 sec | 15-30 sec | 30-60 sec |
| **Total** | **3-8 min** | **8-20 min** | **20-45 min** |

*Times assume confidence ≥80% on first Phase 2 iteration. Add 3-5 min per additional iteration.*

---

*This orchestrator implements the architecture defined in P00.md and integrates with hooks in .claude/hooks/ for deterministic pipeline sequencing.*
```

---

## 8. Key Takeaways

### 8.1 Design Principles

1. **Hook-Orchestrated Architecture**: Hooks provide external "traffic control" for deterministic pipeline sequencing
2. **Orchestrator-Worker Pattern**: Lead agent (Opus) coordinates, specialized workers execute (future enhancement)
3. **Progressive Disclosure**: Load metadata → instructions → resources on-demand (token efficiency)
4. **Confidence-Based Gating**: Never proceed with confidence < 80% (correctness over speed)
5. **Graceful Degradation**: Hooks fail-open, agents continue manually if hooks unavailable

### 8.2 Critical Implementation Details

**Agent File**:
- Location: `.claude/agents/parseltongue-reasoning-orchestrator.md`
- Format: Markdown + YAML front matter
- Model: `opus` (complex reasoning required)
- Tools: `Bash, Read, Write, Glob, Grep, TodoWrite` (explicit for security)

**Hooks**:
- `pipeline-orchestrator.py`: PostToolUse hook for phase sequencing (exit 2 = block)
- `tool-state-tracker.py`: PostToolUse hook for logging (exit 0 = pass-through)
- Configuration: `.claude/settings.local.json` with matchers

**State Management**:
- External state file: `.parseltongue/pipeline-state.json`
- Tracked: phase, phase2_step, iterations, confidence, validation_failures
- Hook loads/saves, agent trusts hook guidance

### 8.3 Workflow Execution

**Phase 1**: Index codebase → Hook blocks → Request micro-PRD.md
**Phase 2**: A01 (tests) → A02 (impl) → B01 (code gen) → B02 (confidence) → Iterate if <80%
**Phase 3**: Validation → If fail, hook routes to Phase 2 Step B01
**Phase 4**: Write files → Multi-layer validation (build, test, perf, lint) → If fail, hook routes appropriately
**Phase 5**: User confirmation → State reset → Git commit → Complete

### 8.4 Token Economics

- Agent system prompt: ~2,000 tokens (loaded once)
- Per-phase context: ~500-1,000 tokens (progressive disclosure)
- Total overhead: ~5-10% of overall budget
- Future optimization: Use Haiku for simple phases (3× cost savings)

---

## 9. Related Documents

**Internal References**:
- **P00.md**: System overview with Level 1 and Level 3 workflow diagrams
- **P01PRDL1Minimal.md**: Minimal PRD with step-by-step user journey
- **P06PRDL6AgentTruthSource.md**: Agent orchestrator specification (if exists)
- **D11-hook-orchestrated-agent-architecture.md**: Hook architecture patterns (referenced)
- **TDD-Tracker.md**: Implementation status for 6 tools

**External References**:
- [Anthropic Multi-Agent Research System](https://www.anthropic.com/engineering/multi-agent-research-system)
- [Claude Code Hooks Documentation](https://docs.claude.com/en/docs/claude-code/hooks)
- [Claude Code Subagents Guide](https://docs.claude.com/en/docs/claude-code/sub-agents)
- [wshobson/agents Repository](https://github.com/wshobson/agents) - 15 workflow orchestrators
- [VoltAgent/awesome-claude-code-subagents](https://github.com/VoltAgent/awesome-claude-code-subagents) - 100+ agents

---

## 10. Next Steps

**Immediate (MVP - Critical Path)**:
1. ✅ Create `.claude/agents/parseltongue-reasoning-orchestrator.md` (use template section 7)
2. ✅ Create `.claude/hooks/pipeline-orchestrator.py` (use implementation section 4.2)
3. ✅ Create `.claude/hooks/tool-state-tracker.py` (use implementation section 4.3)
4. ✅ Update `.claude/settings.local.json` (use configuration section 4.4)
5. ⏸️ Test hook manually (section 6.2)
6. ⏸️ Test agent invocation end-to-end (section 6.2)

**Short-Term Enhancements**:
- Specialized sub-agents per phase (indexing-specialist, reasoning-specialist, etc.)
- PreToolUse hooks for argument validation
- UserPromptSubmit hooks for automatic PRD injection
- Performance metrics dashboard (aggregate tool-invocations.jsonl)

**Long-Term Vision**:
- Multi-language specialist agents (Rust, Python, Go, etc.)
- Parallel validation execution (when Anthropic supports parallel subagents)
- Learning from historical validation failures (confidence calibration)
- Auto-tuning confidence thresholds based on project complexity
- IDE integration (VS Code, IntelliJ plugins)

---

*Document created: 2025-11-01*
*Research agents: general-purpose (Claude Opus 4), Explore (Claude Sonnet 4)*
*Ready for implementation: 100%*
