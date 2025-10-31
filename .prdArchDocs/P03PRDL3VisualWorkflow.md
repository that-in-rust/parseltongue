## Section 0: Visual Architecture Overview

### Complete System Workflow



``` mermaid
---
config:
  flowchart:
    defaultRenderer: "dagre"
  themeVariables:
    primaryColor: "#f3f9ff"
    primaryTextColor: "#0d47a1"
    primaryBorderColor: "#2196f3"
    lineColor: "#42a5f5"
    secondaryColor: "#f1f8e9"
    tertiaryColor: "#fff3e0"
    background: "#ffffff"
    fontFamily: "Arial, sans-serif"
    fontSize: "14px"
---
flowchart TD
    User[User Request] --> |"Natural Language<br/>Change Request"| Claude[LLM Agent]

    subgraph ExternalAgent ["External LLM Agent"]
        Claude --> Orchestrator["Reasoning Orchestrator<br/>5-Phase Workflow"]
        Orchestrator --> Phase1["Phase 1<br/>Project Analysis"]
        Orchestrator --> Phase2["Phase 2<br/>Change Reasoning"]
        Orchestrator --> Phase3["Phase 3<br/>Validation"]
        Orchestrator --> Phase4["Phase 4<br/>File Writing"]
        Orchestrator --> Phase5["Phase 5<br/>State Reset"]
    end

    subgraph LanguageDetection ["Language Detection & Processing"]
        Phase1 --> LangDetect["Auto-detect<br/>Project Languages"]
        LangDetect --> RustFlow{"Rust<br/>Files?"}
        RustFlow --> |"Yes"| RustPath["Rust-Enhanced<br/>Processing"]
        RustFlow --> |"No"| MultiPath["Multi-Language<br/>Basic Processing"]
        RustPath --> Tool1
        MultiPath --> Tool1
    end

    subgraph Tools ["Parseltongue Unified Binary<br/>6-Tool Pipeline"]
        Tool1["`folder-to-cozodb-streamer`<br/>folder-to-cozoDB<br/>Multi-language Indexing<br/>(Starts immediately)"]

        subgraph ParallelPrep ["Parallel Preparation"]
            Tool1 --> |"Indexing 10min<br/>background"| UserPrepares["User prepares<br/>bug description"]
        end

        subgraph IterativeCycle ["Iterative Reasoning Cycle"]
            UserPrepares --> Tool3Read["`llm-cozodb-to-context-writer`<br/>Read Context<br/>LLM-cozoDB-to-context-writer"]
            Tool3Read --> LLMThink["LLM Reasoning<br/>Rubber Duck Debugging"]
            LLMThink --> Tool2Edit["`llm-to-cozodb-writer`<br/>Edit CozoDB<br/>LLM-to-cozoDB-writer"]
            Tool2Edit --> ConfidenceLoop{"Confidence<br/>≥80%?"}
            ConfidenceLoop --> |"No<br/>Refine"| Tool3Read
            ConfidenceLoop --> |"Yes<br/>Proceed"| Tool4
        end

        Tool4["`rust-preflight-code-simulator`<br/>rust-preflight<br/>Syntax Validation<br/>(Tree-sitter only, <20ms)"]
        Phase4 --> Tool5["`llm-cozodb-to-diff-writer`<br/>LLM-cozodb-to-diff-writer<br/>Generate CodeDiff.json<br/>(LLM Applies Changes)"]
        Phase5 --> Tool6["`cozodb-make-future-code-current`<br/>cozoDB-make-future-code-current<br/>Delete Table +<br/>Re-trigger Indexing"]
    end

    subgraph Database ["CozoDB Temporal States"]
        Tool1 --> CozoDB[(CozoDB<br/>Graph Database)]

        subgraph Temporal ["Temporal Versioning System"]
            Current["Current<br/>State"]
            Future["Future<br/>State"]

            Current --> State11["(1,1)<br/>Unchanged"]
            Current --> State10["(1,0)<br/>Delete"]
            Future --> State01["(0,1)<br/>Create"]
            Future --> State11Edit["(1,1)<br/>Modify"]
        end

        CozoDB --> State11
        CozoDB --> State10
        CozoDB --> State01
        CozoDB --> State11Edit
    end

    subgraph FileSystem ["File System Operations"]
        Tool5 --> DiffJSON["CodeDiff.json"]
        DiffJSON --> |"LLM Reads & Applies"| Files["Rust Source<br/>Files"]
        Files --> Modified["Modified<br/>Files"]
    end

    subgraph Phase2Detail ["Phase 2: Detailed Reasoning"]
        Tool2 --> TestInterface["Step A01<br/>Test Interface<br/>Changes"]
        TestInterface --> NonTestInterface["Step A02<br/>Non-Test<br/>Interface Changes"]
        NonTestInterface --> CodeSim["Step B01<br/>Code<br/>Simulation"]
        CodeSim --> RubberDuck["Step B02<br/>Rubber Duck<br/>Debugging"]

        TestInterface --> Hopping["Hopping &<br/>Blast Radius<br/>Queries"]
        Hopping --> |"2-Hop<br/>Dependency<br/>Analysis"| Context["LLM Context<br/>Generation"]
        NonTestInterface --> Context
        Context --> CodeSim
    end

    %% Feedback Loops
    RubberDuck --> |"Confidence ≥ 80%"| Phase3
    RubberDuck --> |"Needs<br/>Refinement"| Phase2
    Phase3 --> |"Validation<br/>Fails"| Phase2
    Phase4 --> |"Tests Fail"| Phase2
    Phase5 --> |"User<br/>Dissatisfied"| Phase4

    %% User Interaction
    Phase5 --> |"Satisfied?"| User
    User --> |"Yes ✅"| Complete["Workflow<br/>Complete"]
    User --> |"No ❌"| Rollback["Rollback<br/>Changes"]
```

### Temporal Versioning System Explained

**Core Innovation**: Time-based state tracking directly in CozoDB

| State | current_ind | future_ind | Meaning | Action |
|-------|-------------|------------|---------|--------|
| **Exists→Continues** | 1 | 1 | Code exists now and will continue | No change needed |
| **Exists→Delete** | 1 | 0 | Code exists now but will be removed | Mark for deletion |
| **Create→Exists** | 0 | 1 | Code doesn't exist but will be created | Create new code |
| **Exists→Modified** | 1 | 1 | Code exists and will be modified | Update with Future_Code |

**State Transition Flow**:
```
Phase 2: LLM sets (current_ind, future_ind, Future_Code, Future_Action)
Phase 4: `llm-cozodb-to-diff-writer` generates CodeDiff.json → LLM applies changes → Files reflect future state
Phase 5: Reset database → (1,1, current_code=Future_Code, future_ind=1)
```

### Hopping & Blast Radius Integration

**Built into cozo-to-context-writer** (not a separate tool):

```bash
# LLM generates these queries via cozo-to-context-writer:
parseltongue reason --query "
  ?[entity, hop_distance, dependency_type] :=
    *changed_entity[base_entity],
    *dependency_graph[base_entity, intermediate],
    *dependency_graph[intermediate, entity],
    hop_distance <= 3
" --context-filter "Future_Action != None"
```

**Query Capabilities**:
- **1-hop**: Direct dependencies only
- **2-hop**: Dependencies of dependencies
- **N-hop**: Configurable depth analysis
- **Blast Radius**: All entities affected by changes
- **Context Filtering**: Only load relevant code for LLM reasoning

### External Agent Philosophy

**MVP Ultra-Minimalist Principles (~10 users)**:
- **Target**: ~10 users - focus on essential functionality that works reliably
- **Philosophy**: Simplicity over complexity - each tool does ONE thing well
- **`llm-cozodb-to-diff-writer`**: NO backup options, NO multiple safety levels, NO configuration complexity (generates CodeDiff.json for LLM)
- **`cozodb-make-future-code-current`**: NO backup metadata files, NO configuration options
- **Goal**: Maximum reliability through simplicity

**Smart Orchestration, Simple Tools**:
- **LLM handles complexity**: Change reasoning, query generation, validation
- **Tools handle execution**: Deterministic, focused operations
- **Clean separation**: External reasoning vs internal tool execution
- **User interaction**: Conversational workflow with confirmation points

**Integration Points**:
1. **Natural Language → Structured Query** (Phase 2)
2. **Query Results → LLM Context** (Phase 2)
3. **LLM Reasoning → Tool Commands** (All phases)
4. **Tool Results → User Feedback** (All phases)

This architecture enables sophisticated code modification while keeping each tool simple, focused, and reliable.