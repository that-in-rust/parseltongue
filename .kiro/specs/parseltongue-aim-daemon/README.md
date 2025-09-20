# Parseltongue AIM Daemon - Specification

**Mission**: Rust-only architectural intelligence with <12ms updates and zero-hallucination LLM context.

## .kiro Guidance Structure

### Steering Rules (Global Guidance)
Located in `.kiro/steering/` - These files provide project-wide guidance and constraints:

| File | Purpose | Status |
|------|---------|--------|
| [parseltongue-requirements-focus.md](../../steering/parseltongue-requirements-focus.md) | Core constraints, decision framework, success metrics | ✅ Active |
| [code-conventions.md](../../steering/code-conventions.md) | Coding standards and conventions | 🔴 Empty |

### Specification Files (Feature-Specific)
Located in `.kiro/specs/parseltongue-aim-daemon/` - These files define the specific feature:

| Document | Purpose | Status |
|----------|---------|--------|
| [requirements.md](./requirements.md) | 18 MVP requirements with REQ-ID system | ✅ Complete |
| [SESSION_CONTEXT.md](./SESSION_CONTEXT.md) | Progress tracking and session recovery | ✅ Complete |
| [requirements-tasks.md](./requirements-tasks.md) | Task tracking and document analysis | 🟡 In Progress |
| [architecture-backlog.md](./architecture-backlog.md) | Technical concepts from analysis | ✅ Complete |
| [backlog.md](./backlog.md) | Post-MVP features by version | ✅ Complete |
| [storage-architecture-options.md](./storage-architecture-options.md) | Storage research archive | ✅ Complete |
| [prompts/storage-architecture-analysis.md](./prompts/storage-architecture-analysis.md) | LLM analysis prompt | ✅ Complete |

## .kiro Guidance Hierarchy

```mermaid
graph TD
    subgraph "Global Steering (.kiro/steering/)"
        A[parseltongue-requirements-focus.md]
        B[code-conventions.md]
    end
    
    subgraph "Feature Spec (.kiro/specs/parseltongue-aim-daemon/)"
        C[requirements.md]
        D[design.md]
        E[tasks.md]
        F[backlog.md]
        G[SESSION_CONTEXT.md]
        H[requirements-tasks.md]
        I[architecture-backlog.md]
        J[storage-architecture-options.md]
        K[prompts/storage-architecture-analysis.md]
    end
    
    A --> C
    A --> D
    B --> D
    C --> D
    D --> E
    C --> F
    H --> C
    K --> D
    F --> L[Future Releases]
```

### Guidance Flow
1. **Steering Rules** provide global constraints and decision frameworks
2. **Requirements** are refined using steering guidance
3. **Design** incorporates both steering rules and requirements
4. **Tasks** are derived from approved design
5. **Implementation** follows tasks while adhering to steering rules

## .kiro Guidance Usage

### Terminal Commands
```bash
# View all steering rules
ls -la .kiro/steering/

# Read core constraints
cat .kiro/steering/parseltongue-requirements-focus.md

# View current requirements
cat .kiro/specs/parseltongue-aim-daemon/requirements.md

# Check session progress
cat .kiro/specs/parseltongue-aim-daemon/SESSION_CONTEXT.md
```

### Quick Start
- **New Contributors**: Read `.kiro/steering/parseltongue-requirements-focus.md` first
- **Implementers**: Study [requirements.md](./requirements.md) (18 requirements with REQ-ID system)
- **Current Status**: Requirements complete, design pending