# D12: .parseltongue/ Folder - Minimal Context Strategy

**Created**: 2025-11-01
**Status**: Proposal for Discussion
**Problem**: Agent context bloat reduces effectiveness (240KB+ docs vs optimal ~30KB)

---

## The Context Budget Problem

**Current State Analysis**:
- `.steeringDocs/`: 240KB total (7 files)
  - S77.IdiomaticRustPatterns.md: 102KB
  - S02-code-conventions.md: 53KB
  - S06-design101-tdd-architecture-principles.md: 29KB
  - refMermaidFile.md: 34KB
  - Others: ~20KB combined
- `TDD-Tracker.md`: 88KB
- `CLAUDE.md`: 16KB
- `.prdArchDocs/`: 188KB total (10 files)

**Total Available Documentation**: ~532KB

**Agent Reality Check**:
- Optimal context load: 30-50KB for responsiveness
- Current approach: Load everything → context saturation
- Result: Agents struggle with signal-to-noise ratio

---

## Proposed `.parseltongue/` Structure

**Philosophy**: Core principles in context, detailed references fetched on-demand.

### Essential Files (Always in Context)

**1. S01-README-MOSTIMP.md** (2.3KB) ✅
```
Source: .steeringDocs/S01-README-MOSTIMP.md
Content:
  - Ultra-minimalist MVP principles (10 users, simplicity over features)
  - TDD-First: RED → GREEN → REFACTOR
  - Executable specifications philosophy
  - 8 architectural principles
  - "The why" behind every decision
Rationale: Foundation document - must be always accessible
```

**2. QUICKSTART.md** (8-10KB, to be created)
```
Content:
  - 6-tool pipeline compact diagram
  - CodeGraph table schema (just the definition)
  - Essential commands per tool
  - Current status: "6/6 tools functional ✅"
  - Reference map: "For X, read Y"
  - Installation: curl | bash pattern
Rationale: Fast orientation for agents starting fresh
```

**3. P01PRDL1Minimal.md** (18KB → condensed to 10KB?)
```
Source: .prdArchDocs/P01PRDL1Minimal.md
Content:
  - MVP principles (NO backups, NO config complexity)
  - Context optimization strategy (exclude current_code by default)
  - 5-phase user workflow (Setup → Reasoning → Validation → Writing → Reset)
  - Tool simplicity rules
Rationale: User journey and workflow philosophy
Question: Can we condense to 10KB by removing redundancies?
```

**Total Context Load**: ~28-30KB (vs. current 240KB+)

---

## What Stays Outside (Read on Demand)

### Reference Tier (Fetch When Needed)

**Rust Patterns**:
- S02-code-conventions.md (53KB) → "For 12-layer Rust conventions, read .steeringDocs/S02"
- S77.IdiomaticRustPatterns.md (102KB) → "For advanced patterns, read .steeringDocs/S77"

**Architecture Details**:
- S06-design101-tdd-architecture-principles.md (29KB) → "For TDD architecture, read .steeringDocs/S06"
- S08-workspace-architecture-strategy.md (15KB) → "For workspace design, read .steeringDocs/S08"

**Implementation Tracking**:
- TDD-Tracker.md (88KB) → "For detailed implementation status, read TDD-Tracker.md"
- Should create STATUS.md (~2KB) with just:
  ```
  Tool 1 (parseltongue-01): ✅ Complete | 6/6 tests
  Tool 2 (parseltongue-02): ✅ Complete | 5/5 tests
  Tool 3 (parseltongue-03): ✅ Complete | 8/8 tests
  Tool 4 (parseltongue-04): ✅ Complete | 6/6 tests
  Tool 5 (parseltongue-05): ✅ Complete | 7/7 tests
  Tool 6 (parseltongue-06): ✅ Complete | 5/5 tests
  ```

**PRD Details**:
- P00.md (42KB) → Multi-level visual architecture
- P02-P06 (varies) → Detailed specs, workflows, JTBD diagrams

**User Guide**:
- CLAUDE.md (16KB) → Comprehensive development guide (current primary doc)

---

## Design Rationale

### Why This Split?

**Always-In-Context (S01, QUICKSTART, P01)**:
- Answers: "What is this system? Why does it exist? How do I use it?"
- Frequency: Referenced in every agent invocation
- Size: Small enough to not bloat context (~30KB)
- Role: Orientation and decision-making foundation

**On-Demand References (Everything Else)**:
- Answers: "How do I implement X in idiomatic Rust?"
- Frequency: Task-specific (e.g., only when writing new Rust code)
- Size: Can be large (50-100KB) because loaded selectively
- Role: Deep technical guidance

### Agent Workflow Pattern

```
1. Agent starts → Reads .parseltongue/ (~30KB context)
2. User asks: "Add a new CozoDB query for dependency tracking"
3. Agent thinks: "I need Rust patterns and CozoDB examples"
4. Agent reads: .steeringDocs/S02, .domainDocs/D08 (on-demand)
5. Agent implements with full context
```

**Key Insight**: Agents should read "just enough to decide what else to read," not "everything upfront."

---

## Open Questions for Decision

### 1. P01 Inclusion?
- **Current**: 18KB (borderline large)
- **Option A**: Include as-is in `.parseltongue/`
- **Option B**: Condense to 10KB by removing redundancies
- **Option C**: Move to reference tier, create ultra-minimal summary in QUICKSTART.md

**Recommendation**: Option B (condense to 10KB) - user journey is core, but can be tighter.

### 2. Status Tracking?
- **Problem**: TDD-Tracker.md is 88KB (too large for constant context)
- **Solution**: Create `STATUS.md` (~2KB) with just completion checkmarks?
- **Alternative**: Include status summary in QUICKSTART.md?

**Recommendation**: Add status to QUICKSTART.md (5-7 lines, ~500 bytes).

### 3. Database Schema Location?
- **Option A**: Full CodeGraph schema in QUICKSTART.md
- **Option B**: Just reference: "For schema, read parseltongue-core/src/entities.rs"
- **Option C**: Minimal schema (column names + types only) in QUICKSTART.md

**Recommendation**: Option C - column names and types (~15 lines) is essential for understanding the system.

### 4. Commands Location?
- **Current**: Comprehensive commands in CLAUDE.md
- **Should we**: Extract essential commands (1-2 per tool) into QUICKSTART.md?

**Recommendation**: Yes - QUICKSTART.md shows minimal usage, CLAUDE.md stays as comprehensive reference.

---

## Implementation Plan

### Phase 1: Create QUICKSTART.md
```markdown
# Parseltongue Quickstart

## System Status
6/6 tools functional ✅ | Ready for production 🚀

## The 6-Tool Pipeline
[Compact mermaid diagram]

## CodeGraph Schema
[Table definition: 10 columns with types]

## Essential Commands
Tool 1: cargo run --package parseltongue-01 -- --dir ./src
Tool 2: cargo run --package parseltongue-02 -- --input changes.json
[etc.]

## Reference Map
- Rust patterns → .steeringDocs/S02
- TDD architecture → .steeringDocs/S06
- Implementation details → TDD-Tracker.md
```

### Phase 2: Condense P01 (Optional)
- Remove redundant examples
- Consolidate tool simplicity rules
- Target: 10KB from 18KB

### Phase 3: Create `.parseltongue/` Folder
```bash
mkdir .parseltongue
cp .steeringDocs/S01-README-MOSTIMP.md .parseltongue/
cp QUICKSTART.md .parseltongue/
cp .prdArchDocs/P01PRDL1Minimal.md .parseltongue/  # or condensed version
```

### Phase 4: Update Agent Instructions
Modify agent orchestrator to:
- Load `.parseltongue/` files in initial context
- Provide references to detailed docs as needed
- Never load S02/S77/TDD-Tracker upfront

---

## Success Metrics

**Before**:
- Agent context: ~240KB (steering) + 88KB (tracker) = 328KB
- Agent startup: "Here's everything, good luck"
- Task execution: Struggle with signal-to-noise

**After**:
- Agent context: ~30KB (parseltongue folder)
- Agent startup: "Here's the foundation, I'll fetch details as needed"
- Task execution: Focused, context-aware, efficient

**Measurable Improvement**:
- Context load: 328KB → 30KB (91% reduction)
- Agent response time: Faster (less to process upfront)
- Task accuracy: Higher (clearer signal from core principles)

---

## Appendix: File Size Reference

```
.steeringDocs/
├── S01-README-MOSTIMP.md              2,288 bytes  ✅ INCLUDE
├── S02-code-conventions.md           53,408 bytes  ❌ ON-DEMAND
├── S05-tone-style-guide.md            3,670 bytes  ❌ ON-DEMAND
├── S06-design101-tdd...              29,107 bytes  ❌ ON-DEMAND
├── S08-workspace-architecture...     15,325 bytes  ❌ ON-DEMAND
├── S77.IdiomaticRustPatterns.md     102,438 bytes  ❌ ON-DEMAND
└── refMermaidFile.md                 34,498 bytes  ❌ ON-DEMAND

.prdArchDocs/
├── P01PRDL1Minimal.md                18,000 bytes  ⚠️  MAYBE (condense?)
├── P00.md                            42,000 bytes  ❌ ON-DEMAND
└── [others]                         ~148,000 bytes ❌ ON-DEMAND

Root:
├── TDD-Tracker.md                    87,763 bytes  ❌ ON-DEMAND
├── CLAUDE.md                         15,599 bytes  ❌ ON-DEMAND
└── QUICKSTART.md                     ~8,000 bytes  ✅ CREATE & INCLUDE
```

---

## Next Steps

1. **Discuss with user**: Confirm approach and answer open questions
2. **Create QUICKSTART.md**: Based on decisions above
3. **Create `.parseltongue/` folder**: Populate with S01 + QUICKSTART + P01 (or condensed)
4. **Update agent orchestrator**: Load minimal context, reference detailed docs
5. **Test agent effectiveness**: Measure response quality with reduced context

**Decision Authority**: User (@amuldotexe)
**Implementation**: Once approved, can be done in <1 hour
