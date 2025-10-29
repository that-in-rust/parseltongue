## Section 0B: User Journey & Command Usage (JTBD)

### MVP Ultra-Minimalist Principles (~10 users)
**Target**: ~10 users - focus on essential functionality that works reliably
**Philosophy**: Simplicity over complexity - each tool does ONE thing well
**Tool 5**: NO backup options, NO multiple safety levels, NO configuration complexity
**Tool 6**: NO backup metadata files, NO configuration options
**Validation**: Basic build/test verification only (no complex safety nets)
**Goal**: Maximum reliability through ultra-minimalist approach

### End-to-End User Workflow

```mermaid
---
config:
  flowchart:
    defaultRenderer: "dagre"
  themeVariables:
    primaryColor: "#f0f9ff"
    primaryTextColor: "#0c4a6e"
    primaryBorderColor: "#0ea5e9"
    lineColor: "#38bdf8"
    secondaryColor: "#f0fdf4"
    tertiaryColor: "#fefce8"
    quaternaryColor: "#fff7ed"
    background: "#ffffff"
    fontFamily: "Arial, sans-serif"
    fontSize: "13px"
---
flowchart TD
    subgraph Setup ["User Setup Phase"]
        Start["User discovers<br/>Parseltongue"] --> Download["Download parseltongue<br/>binary from GitHub"]
        Download --> CopyBinary["Copy binary to<br/>GitHub repo root"]
        CopyBinary --> CreateAgents["Create .claude/agents/<br/>directory"]
        CreateAgents --> CopyAgent["Copy reasoning-orchestrator.md<br/>to agents folder"]
        CopyAgent --> SetupComplete["Setup Complete<br/>Ready to use"]
    end

    SetupComplete --> PrimaryWorkflow["Primary Workflow:<br/>Agentic Interface"]

    subgraph PrimaryPath ["Agentic Workflow (95% of users)"]
        PrimaryWorkflow --> ClaudeStart["@agent-parseltongue-reasoning-orchestrator<br/>in Claude Code"]
        ClaudeStart --> RepoConfirm["Confirm git repository<br/>location"]
        RepoConfirm --> AutoIndex["Auto-index codebase<br/>Tool 1: folder-to-cozoDB-streamer<br/>(10 minutes)"]

        subgraph ParallelPreparation ["Parallel Preparation"]
            AutoIndex --> |"Indexing runs<br/>in background"| UserThinking["User prepares<br/>bug description<br/>while indexing"]
            UserThinking --> NaturalRequest["User: 'Fix panic in<br/>GitHub #1234' or<br/>'Fix segfault from error.log'"]
            AutoIndex --> |"Indexing complete<br/>analytics ready"| MicroPRD["Create Micro-PRD<br/>Bug analysis & refinement"]
        end

        subgraph IterativeReasoning ["Iterative LLM Reasoning Cycle"]
            MicroPRD --> Tool3Read["Tool 3: Read Context<br/>LLM-cozoDB-to-context-writer"]
            Tool3Read --> LLMReason["LLM reasoning<br/>Rubber duck debugging"]
            LLMReason --> Tool2Edit["Tool 2: Edit CozoDB<br/>LLM-to-cozoDB-writer"]
            Tool2Edit --> ConfidenceCheck{"Confidence<br/>≥ 80%?"}
            ConfidenceCheck --> |"No<br/>Refine"| Tool3Read
            ConfidenceCheck --> |"Yes<br/>Proceed"| Validation
        end

        Validation --> Tool5["Tool 5: LLM-cozoDB-to-code-writer<br/>Single Reliable Write<br/>(No Backup Options)"]
        Tool5 --> Tool6["Tool 6: cozoDB-make-future-code-current<br/>Delete Table +<br/>Re-trigger Indexing"]
        Tool6 --> GitCommit["Auto-git commit<br/>of changes"]
        GitCommit --> AgentSuccess["Workflow<br/>Complete"]
    end

    subgraph AdvancedOptions ["Advanced Options (5% of users)"]
        AgentSuccess --> AdvancedChoice{Need more<br/>control?}
        AdvancedChoice --> |"Manual CLI<br/>intervention"| CliPath["Direct Tool<br/>Commands"]
        AdvancedChoice --> |"Custom<br/>workflow"| MixedPath["Mixed<br/>Approach"]
        AdvancedChoice --> |"Continue with<br/>agent"| NewRequest["New change<br/>request"]

        CliPath --> ManualTools["parseltongue read/reason/<br/>simulate/write/reset"]
        MixedPath --> HybridTools["Agent reasoning +<br/>manual execution"]

        ManualTools --> ResumeAgent["Resume agent<br/>workflow"]
        MixedPath --> ResumeAgent
        ResumeAgent --> ClaudeStart
    end

    subgraph CommonScenarios ["Bug Fixing Scenarios"]
        AgentSuccess --> Scenario1["Panic/Segfault<br/>Issue #1234"]
        AgentSuccess --> Scenario2["Logic Error<br/>Issue #5678"]
        AgentSuccess --> Scenario3["Memory Leak<br/>Issue #9012"]

        Scenario1 --> TimeEstimate1["3-5 minutes<br/>Panic analysis + fix"]
        Scenario2 --> TimeEstimate2["10-15 minutes<br/>Logic tracing + correction"]
        Scenario3 --> TimeEstimate3["5-10 minutes<br/>Memory analysis + cleanup"]
    end

    %% Feedback loops
    AgentSuccess --> |"New change request"| ClaudeStart
    NewRequest --> ClaudeStart

    %% Error handling and recovery loops (MVP: Minimal Verification)
    Validation --> |"Validation fails"| LLMReason
    Tool5 --> BuildCheck["Build Check:<br/>cargo build"]
    BuildCheck --> |"Build fails"| Tool5
    BuildCheck --> TestCheck["Test Check:<br/>cargo test"]
    TestCheck --> |"Tests fail"| LLMReason
    TestCheck --> GitCommit["Auto-git commit<br/>of changes"]

    %% Note: Manual CLI workflow follows similar error handling patterns
  %% with appropriate tool references for direct command usage
```
