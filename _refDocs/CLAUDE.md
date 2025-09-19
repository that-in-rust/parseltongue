# Claude Configuration & Context Management Strategies

## CRITICAL: Large File Reading Protocol
**BEFORE reading any text, PDF, JSON, JS, or similar files:**
1. **ALWAYS** get a line count first: `wc -l filename`
2. If file is **> 500 lines**, use chunked reading strategy:
   - Read in 500-line chunks using `head -n N` and `tail -n +M`
   - Track chunks read vs total file size
   - Ensure comprehensive coverage of entire file
3. **NEVER** read large files without first understanding their size
4. **ALWAYS** verify complete file coverage when using chunked approach

## Documentation Pyramid Steering System

### Meta-Pattern: Cascading Detail Principle

**Core Philosophy**: Documentation must flow from abstract governance to concrete implementation, with each level containing maximum appropriate detail for its purpose. This creates a perfect cascade where developers only need to reference the appropriate level for their current task.

#### The Five-Level Documentation Pyramid
```
requirements.md (TOP - Governing Rules & Strategic Vision)
    ↓ [What & Why]
architecture.md (System Design & Component Relationships)
    ↓ [Where & How Components Relate]
architecture-L2.md (Implementation Patterns & TDD Strategies)
    ↓ [How to Build It Right]
design.md (Complete Technical Contracts & Interfaces)
    ↓ [What the Code Must Look Like]
tasks.md (BOTTOM - Maximum Implementation Detail)
    ↓ [Exactly How to Write Every Line]
```

#### Level-by-Level Responsibility Framework

**Level 1: requirements.md (Governing Rules)**
- **Purpose**: Strategic vision, constraints, and non-negotiable rules
- **Content**: Anti-coordination mandates, 5 Critical Gaps, business requirements
- **Detail Level**: High-level principles with specific, measurable constraints
- **When to Reference**: Setting project direction, making architectural decisions
- **Key Question**: "What are we building and what rules must we never break?"

**Level 2: architecture.md (System Design)**
- **Purpose**: Component relationships, data flow, and deployment architecture
- **Content**: System components, APIs, database schema, deployment model
- **Detail Level**: Technical architecture with clear component boundaries
- **When to Reference**: Understanding how pieces fit together
- **Key Question**: "Where does everything live and how do they relate?"

**Level 3: architecture-L2.md (Implementation Patterns)**
- **Purpose**: How to implement using proven patterns and methodologies
- **Content**: TDD workflows, Rails parity strategies, code organization patterns
- **Detail Level**: Implementation methodologies with code structure examples
- **When to Reference**: Deciding how to approach coding tasks
- **Key Question**: "How should we build this following established patterns?"

**Level 4: design.md (Technical Contracts)**
- **Purpose**: Complete interface specifications and type contracts
- **Content**: Function signatures, type definitions, error hierarchies, APIs
- **Detail Level**: Precise technical specifications without implementation
- **When to Reference**: Writing function signatures and understanding interfaces
- **Key Question**: "What must the code look like at the contract level?"

**Level 5: tasks.md (Maximum Implementation Detail)**
- **Purpose**: Exact step-by-step implementation instructions
- **Content**: Specific file paths, complete function implementations, test cases, examples
- **Detail Level**: Maximum concrete detail - copy-paste ready when appropriate
- **When to Reference**: Actually writing the code
- **Key Question**: "Exactly how do I implement this specific feature?"

#### Document Synchronization Protocol

**Verification Checklist** (Run when any document is saved):
1. **Level Boundaries**: Does this document stay within its abstraction level?
2. **Content Overlap**: Is there duplication with other levels that should be consolidated?
3. **Detail Appropriateness**: Is the detail level correct for this pyramid level?
4. **Cross-References**: Are all pyramid references accurate and up-to-date?
5. **Traceability**: Can each requirement be traced through to implementation?

**Level Purity Rules**:
- **NO level creep**: Implementation examples belong ONLY in tasks.md
- **NO duplication**: TDD patterns belong ONLY in architecture-L2.md
- **NO abstraction at bottom**: tasks.md must contain maximum detail
- **NO vagueness at top**: requirements.md must have specific, measurable constraints

#### Before You Code Decision Tree

```
Need to understand constraints? → requirements.md (Anti-coordination rules)
Need to understand system design? → architecture.md (Components & flow)
Need implementation patterns? → architecture-L2.md (TDD & Rails parity)
Need function signatures? → design.md (Technical contracts)
Need to write actual code? → tasks.md (Maximum implementation detail)
```

#### Document Synchronization Status
- **Pyramid Structure**: ✅ Well-designed five-level cascade
- **Cross-References**: ✅ All documents reference hierarchy position
- **Level Boundaries**: ✅ Fixed - TDD patterns consolidated in architecture-L2.md
- **Implementation Detail**: ✅ Enhanced - tasks.md now contains complete code examples
- **Requirement Traceability**: ✅ Implemented - Standardized REQ-ID system with full traceability
- **Meta-Pattern Integration**: ✅ Complete - 7 documentation excellence principles documented
- **Verification Protocols**: ✅ Active - Automated checks and validation procedures

## Requirement Traceability System

### Meta-Pattern: REQ-ID Standardization

**Learned from Documentation Analysis**: Inconsistent requirement numbering creates traceability gaps. Every requirement must have a unique, hierarchical ID that flows through all implementation levels.

### Standardized REQ-ID Format
```
REQ-[CATEGORY]-[NUMBER].[SUBNUMBER]
Examples:
REQ-ARCH-001.0  // Architecture constraints
REQ-GAP-001.0   // Critical gap implementation
REQ-FUNC-001.0  // Functional requirements
REQ-TECH-001.0  // Technical requirements
```

### Critical Gap Mapping (Fixed)
| Gap ID | Standard REQ-ID | Implementation Location | Test Coverage |
|--------|-----------------|----------------------|---------------|
| Gap #1 | REQ-GAP-001.0 | MessageService::create_message_with_deduplication | prop_dedup_idempotent |
| Gap #2 | REQ-GAP-002.0 | WebSocketBroadcaster::handle_reconnection | test_reconnect_missed |
| Gap #3 | REQ-GAP-003.0 | DatabaseWriter::serialize_writes | prop_concurrent_writes_serialized |
| Gap #4 | REQ-GAP-004.0 | AuthService::create_secure_session | prop_token_entropy_sufficient |
| Gap #5 | REQ-GAP-005.0 | PresenceService::track_with_ttl | prop_presence_ttl_cleanup |

### Traceability Enforcement Protocol

**Meta-Pattern Implementation**: Every code change must reference its REQ-ID in commit messages and code comments:

```rust
// REQ-GAP-001.0: Message deduplication with UNIQUE constraint
pub async fn create_message_with_deduplication(
    &self,
    data: CreateMessageData,
) -> Result<DeduplicatedMessage<Verified>, MessageError> {
    // Implementation handles UNIQUE constraint violation
    // as specified in REQ-GAP-001.0 requirements
}
```

**Commit Message Format**:
```
feat(message-service): implement deduplication [REQ-GAP-001.0]

- Add UNIQUE constraint on (client_message_id, room_id)
- Handle constraint violations gracefully
- Return existing message for duplicate client_message_id
- Property tests validate idempotent behavior
```

### Automated Traceability Verification

**Meta-Pattern**: Automated scripts ensure complete requirement coverage:

```bash
# Verify all REQ-IDs have implementations
./scripts/traceability-check.sh

# Expected output:
✅ REQ-GAP-001.0: Found in src/services/message_service.rs:55
✅ REQ-GAP-002.0: Found in src/websocket/broadcaster.rs:120
✅ REQ-GAP-003.0: Found in src/database/writer.rs:89
✅ REQ-GAP-004.0: Found in src/auth/service.rs:45
✅ REQ-GAP-005.0: Found in src/presence/service.rs:67
```

### Cross-Reference Matrix

**Meta-Pattern Implementation**: Every document references REQ-IDs:

- **requirements.md**: Defines REQ-IDs with acceptance criteria
- **architecture.md**: Maps REQ-IDs to system components
- **architecture-L2.md**: Maps REQ-IDs to TDD patterns
- **design.md**: Maps REQ-IDs to function signatures
- **tasks.md**: Maps REQ-IDs to complete implementations

### Implementation Verification

**Meta-Pattern**: Every REQ-ID must have:
1. **Definition**: Clear requirement in requirements.md
2. **Design**: Function signature in design.md
3. **Implementation**: Complete code in tasks.md
4. **Test**: Property test validating behavior
5. **Traceability**: REQ-ID references in code and commits

## Minto Pyramid Principle for Documentation

### Meta-Pattern: Minto Pyramid Principle

**Core Philosophy**: All documentation must follow the Minto Pyramid Principle - **Conclusion and Recommendation First**, then Supporting Arguments, then Detailed Evidence. This creates a communication structure that enables immediate understanding while providing comprehensive detail for implementers.

#### Minto Pyramid Structure for Documentation

```
CONCLUSION/RECOMMENDATION (Essence - What we should do)
    ↓ [The "So What?" - Immediate Action]
SUPPORTING ARGUMENTS (Why - Key reasons and logic)
    ↓ [The "Why" - Rationale and reasoning]
DETAILED EVIDENCE (How - Implementation details and proof)
    ↓ [The "How" - Specific execution steps]
```

#### Implementation in Documentation Pyramid

Each level of our documentation pyramid now follows Minto structure:

**Level 1: requirements.md**
- **Conclusion**: What we must build (anti-coordination + 5 Critical Gaps)
- **Supporting**: Why these constraints are non-negotiable
- **Evidence**: Specific, measurable requirements and acceptance criteria

**Level 2: architecture.md**
- **Conclusion**: System architecture and component relationships
- **Supporting**: Why this architecture solves our problems
- **Evidence**: Detailed design decisions, API contracts, data models

**Level 3: architecture-L2.md**
- **Conclusion**: Implementation patterns and methodologies
- **Supporting**: Why these patterns ensure quality and maintainability
- **Evidence**: Code examples, TDD workflows, best practices

**Level 4: design.md**
- **Conclusion**: Complete technical contracts and interfaces
- **Supporting**: Why these contracts satisfy the requirements
- **Evidence**: Function signatures, type definitions, error hierarchies

**Level 5: tasks.md**
- **Conclusion**: Exact implementation steps
- **Supporting**: Why this approach is correct and efficient
- **Evidence**: Complete code examples, test cases, deployment scripts

#### Benefits for LLM Communication

**Immediate Intent Recognition**: LLMs grasp the core requirement immediately from the conclusion, then use supporting arguments to understand context, and finally dive into implementation details.

**Reduced Ambiguity**: Clear structure eliminates interpretation errors - the conclusion states what must be achieved, supporting arguments explain why, and evidence provides how.

**Efficient Processing**: LLMs can process specifications in a top-down manner, understanding the essence before getting lost in implementation details.

#### Minto in Action: Example Specification

```markdown
## Message Deduplication Service [REQ-GAP-001.0]

### Conclusion
Implement idempotent message creation using UNIQUE constraint on (client_message_id, room_id) to prevent duplicate messages from rapid clicking.

### Supporting Arguments
- **User Experience**: Rapid clicking should not create duplicate messages
- **Data Integrity**: Each client_message_id must be unique within a room
- **Performance**: Constraint violation handling must be efficient (<200ms p99)
- **Reliability**: System must gracefully handle concurrent creation attempts

### Evidence
#### Database Schema
```sql
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    client_message_id UUID NOT NULL,
    room_id UUID NOT NULL,
    -- ... other fields
    UNIQUE(client_message_id, room_id)
);
```

#### Interface Contract
```rust
pub trait MessageService: Send + Sync {
    async fn create_message_with_deduplication(
        &self,
        data: CreateMessageData,
    ) -> Result<DeduplicatedMessage<Verified>, MessageError>;
}
```

#### Property Test
```rust
proptest! {
    #[test]
    fn prop_dedup_idempotent(data in any::<CreateMessageData>()) {
        // Same client_message_id always returns same message
    }
}
```

#### Implementation Algorithm
1. Execute INSERT with UNIQUE constraint
2. On constraint violation, SELECT existing message
3. Return existing message with Verified status
4. Log deduplication event for analytics
```

#### Verification Protocol

**Minto Compliance Check**: Each document must pass the "Conclusion First" test:
1. Can a reader understand the core requirement from the first 3 lines?
2. Are supporting arguments clearly separated from implementation details?
3. Is evidence provided to substantiate each claim?
4. Does the structure flow from essence to details?

## Meta-Patterns Integration

### Meta-Patterns Learned from Documentation Analysis

**Documentation Excellence Principles Discovered**:

#### 1. **Cascading Detail Principle**
- **Learning**: Each pyramid level must contain exactly the right detail for its purpose
- **Implementation**: Abstract → Concrete with no level overlap
- **Verification**: Check level boundaries on every document change

#### 2. **Content Purity Rule**
- **Learning**: Implementation code belongs ONLY in tasks.md, TDD patterns ONLY in architecture-L2.md
- **Implementation**: Strict separation prevents confusion and duplication
- **Verification**: Level purity checklist on every save

#### 3. **REQ-ID Standardization**
- **Learning**: Inconsistent numbering breaks traceability
- **Implementation**: Hierarchical REQ-ID system with automated verification
- **Verification**: Traceability scripts validate complete coverage

#### 4. **Maximum Detail at Bottom**
- **Learning**: tasks.md must contain complete, copy-ready implementations
- **Implementation**: Full file examples with property tests and integration scenarios
- **Verification**: Code examples compile and pass tests

#### 5. **Meta-Pattern Documentation**
- **Learning**: Document the patterns themselves for consistency
- **Implementation**: Meta-patterns section explains the "why" behind structure
- **Verification**: New patterns follow established meta-pattern framework

#### 6. **Synchronization Enforcement**
- **Learning**: Documentation drifts without active maintenance
- **Implementation**: Automated checks and verification protocols
- **Verification**: Synchronization status tracking and validation

#### 7. **Anti-Coordination in Documentation**
- **Learning**: Complex coordination creates maintenance overhead
- **Implementation**: Simple, direct patterns with clear boundaries
- **Verification**: Level independence ensures minimal coordination needs

### Meta-Pattern Application Protocol

**When Adding New Documentation**:
1. **Check Level Boundaries**: Does this content belong at this pyramid level?
2. **Verify REQ-ID Coverage**: Is every requirement properly referenced?
3. **Apply Meta-Patterns**: Does this follow established documentation patterns?
4. **Maintain Purity**: No content overlap with other levels
5. **Ensure Traceability**: Clear path from requirement to implementation

**When Modifying Existing Documentation**:
1. **Update Cross-References**: Maintain pyramid integrity
2. **Verify REQ-ID Links**: Ensure all references remain valid
3. **Check Level Boundaries**: No content creep between levels
4. **Run Verification**: Execute automated checks where available
5. **Update Meta-Patterns**: Document new patterns discovered

### Meta-Pattern Benefits Realized

1. **Consistency**: All documentation follows the same patterns
2. **Maintainability**: Clear rules make updates predictable
3. **Traceability**: REQ-ID system ensures complete coverage
4. **Efficiency**: Developers know exactly where to find information
5. **Quality**: Verification protocols prevent documentation drift
6. **Scalability**: Meta-patterns scale to new features and requirements

#### Cascading Detail Principle Benefits

1. **Clear Implementation Path**: Developers know exactly which document to reference
2. **Prevents Decision Paralysis**: Each level has specific, focused purpose
3. **Maximum Efficiency**: No searching through multiple docs for specific information
4. **Perfect Scalability**: New features follow the same cascade pattern
5. **Quality Assurance**: Each level validates the level above it

**Meta-Pattern Verification**: Every time a specification document is modified, verify that it:
- Respects its pyramid level boundaries
- Maintains appropriate detail for its position
- Provides clear references to adjacent levels
- Enables developers to work efficiently at their current abstraction level

## Universal Context Management System

### SESSION_CONTEXT.md Integration

**Primary Context File**: `SESSION_CONTEXT.md` serves as the universal session context file with persistent progress tracking across Claude sessions.

#### Context Recovery Protocol

**When Context is Lost** (Use SESSION_CONTEXT.md):
```bash
# Quick Context Recovery
cat SESSION_CONTEXT.md | grep -A 20 "Live Session Status"

# Architecture Quick Reference
cat .kiro/steering/anti-coordination.md

# Todo List Status
grep -A 10 "Active Todo List" SESSION_CONTEXT.md

# 5 Critical Gaps Status
grep -A 15 "5 Critical Gaps Implementation Status" SESSION_CONTEXT.md
```

**Session Recovery Template** (Auto-generated in SESSION_CONTEXT.md):
```
=== CONTEXT RECOVERY ===
Project: Campfire Rust + React Rewrite
Branch: [current branch from SESSION_CONTEXT.md]
Current Focus: [priority task from active todos]
Last Action: [last completed from progress log]
Key Decisions: Anti-coordination + 5 Critical Gaps + TDD-first
Next Steps: [planned actions from next session template]
=== END RECOVERY ===
```

#### Universal Context File Structure

**SESSION_CONTEXT.md Contains**:
- **Live Session Status**: Current branch, phase, priority tasks
- **Active Todo List**: Auto-synced with TodoWrite tool
- **Progress Log**: Complete session history with timestamps
- **Architecture Compliance**: Anti-coordination verification checklist
- **5 Critical Gaps**: Implementation status tracking
- **Documentation Pyramid**: Current state and synchronization
- **Technology Stack**: Dependencies and configuration status
- **Recovery Commands**: Quick context restoration

#### Usage Patterns

**Update Cadence**:
1. **Every Major Milestone**: Complete todo section update
2. **Session Start**: Verify live status and priority tasks
3. **Architecture Changes**: Update compliance checklist
4. **Daily Sync**: Refresh progress log and next actions

**Integration Commands**:
- `/recover-context`: Display live session status from SESSION_CONTEXT.md
- `/update-todos`: Sync TodoWrite tool with SESSION_CONTEXT.md
- `/check-compliance`: Verify architecture constraints
- `/next-steps`: Show priority tasks and next actions

### Token Management Strategy

When approaching limits, compress in this order:
1. **Keep**: Project name, anti-coordination constraints, current file
2. **Reference**: SESSION_CONTEXT.md for detailed state (don't embed)
3. **Summarize**: Architecture overview, methodology, completed tasks
4. **Omit**: Historical context, detailed explanations, examples

### Quick Commands
- `/recover-context` - Display SESSION_CONTEXT.md live status
- `/show-context` - Display SESSION_CONTEXT.md complete content
- `/update-todos` - Sync TodoWrite with SESSION_CONTEXT.md
- `/check-compliance` - Verify anti-coordination constraints

## Project Overview
This is a Rust + React web application rewrite project. Current branch: feature/campfire-rust-rewrite-spec

## Large File Reading Strategy

### Systematic Chunked Reading
For files >500 lines, read in chunks with complete coverage:

```bash
# Check total size first
wc -l large_file.md

# Read in 500-line chunks
head -n 500 large_file.md           # First chunk
tail -n +501 large_file.md | head -n 500  # Second chunk
tail -n +1001 large_file.md | head -n 500 # Third chunk
# Continue until end
```

### Verification Protocol
- Track chunks read vs total file size
- Use grep to find specific sections: `grep -n "keyword" large_file.md`
- Cross-reference with document structure
- Maintain reading log for complex files

### Context-Aware Alternative
For structured documents:
1. Read first 200 lines (structure/overview)
2. Read middle section (core content)
3. Read last 200 lines (conclusions)
4. Use keyword search for specific topics

## Core Context Management Strategies

### Effective Development Workflow
1. **Targeted Search**: Use specific file/method references rather than broad requests
2. **Direct Context**: Provide relevant code directly in prompts when possible
3. **Plan Mode**: Use `Shift+Tab` twice for read-only analysis before implementation
4. **Focused Sessions**: Use `/clear` between distinct tasks to prevent context bloat
5. **Memory Files**: This CLAUDE.md provides persistent project context

### Development Best Practices
6. **Iterative Approach**: Work in small chunks (2-3 files), review diffs, commit frequently
7. **Parallel Sessions**: Run multiple Claude instances for isolated context (separate worktrees)
8. **External Integration**: Use `grep`/`rg` to find code, format clearly before processing
9. **Large Files**: Use chunked reading strategy, ask for summaries first
10. **Checklist Planning**: Use markdown checklists for complex tasks to prevent overload

## Project-Specific Architecture Patterns

### Code Conventions (from .kiro/steering/)
- **Maximum 500 lines per file** - Split larger files into smaller modules
- **Rails-style modules**: models/, handlers/, services/, middleware/
- **Clear module boundaries** - No circular dependencies
- **Single responsibility** - Each file has one clear purpose
- **Result<T, E> only** - No custom error types unless absolutely necessary
- **Flat error handling** - Avoid nested Result chains
- **Direct SQL** with sqlx::query! macros - No query builders beyond sqlx

### Anti-Coordination Architecture (Critical Constraints)
**MANDATORY FOR MVP**: These constraints prevent coordination complexity madness

#### 🚫 **FORBIDDEN PATTERNS**
- **NO coordination layers, coordinators, or event buses**
- **NO distributed transactions, sagas, or event sourcing**
- **NO circuit breakers, retry queues, or complex error recovery**
- **NO cross-tab coordination or global state synchronization**
- **NO microservices, service mesh, or distributed architecture**
- **NO message queues, event streams, or async coordination**
- **NO complex state machines or coordination protocols**

#### ✅ **MANDATORY SIMPLICITY PATTERNS**
- **Direct SQLite operations** - Simple INSERT/UPDATE/SELECT queries
- **Basic WebSocket broadcasting** - Direct room-based message sending
- **Rails-style session management** - Simple cookie-based authentication
- **Simple error handling** - Basic Result<T, E> with user-friendly messages
- **Direct function calls** - No async coordination between components
- **Single binary deployment** - No orchestration or service discovery

#### 📏 **COMPLEXITY LIMITS**
- **Maximum 50 total files** in entire codebase (backend + frontend)
- **No file over 500 lines** - Split large files into smaller modules
- **Maximum 3 async operations per request** - Keep request handling simple
- **No more than 2 levels of error handling** - Avoid nested Result chains
- **Single database connection pool** - No distributed data management

#### 🎯 **RAILS PARITY RULE**
- **If Rails doesn't do it, we don't do it** - Use Rails as the complexity ceiling
- **Replicate Rails patterns exactly** - Don't "improve" on proven Rails behavior
- **Evidence-based additions only** - New patterns require Rails precedent
- **Simple beats clever** - Choose obvious solutions over optimized ones

### TDD-First Development Philosophy
**Core Principle**: Define complete function signatures, type contracts, and property tests before writing any implementation code.

#### TDD Development Cycle
1. **Type Contracts**: Define complete function signatures with all error cases
2. **Property Tests**: Specify behavior through property-based testing
3. **Integration Contracts**: Define service boundaries and interaction patterns
4. **Implementation**: Type-guided implementation following contracts
5. **Validation**: Comprehensive testing validates contract compliance

### Rust Idiomatic Patterns (L1-L3)

#### L1: Core Language Patterns
- **Ownership and Borrowing Mastery**: Prefer borrowing over ownership transfer
- **Type-Driven Design with Newtypes**: Use newtypes for all domain IDs
- **Zero-Cost Abstractions with Iterators**: Functional style with zero runtime cost
- **Making invalid states unrepresentable** through enums and phantom types

#### L2: Standard Library Patterns
- **Smart Pointer Decision Matrix**: Use appropriate pointers for thread safety
- **Collection Patterns**: Accept slices, store owned, return owned
- **RAII for automatic resource cleanup**

#### L3: Ecosystem Patterns
- **Error Handling Strategy**: thiserror for libraries, anyhow for applications
- **Async Patterns**: Actor pattern for state, structured concurrency with JoinSet
- **Web Application Patterns**: Type-safe request handling with Axum

### React Patterns (2025 Edition)
- **Type-Safe Component Architecture**: Branded types for domain safety
- **State Management**: Zustand + Immer for predictable state updates
- **WebSocket Integration**: Automatic reconnection with React Query
- **Optimistic Updates**: Immediate UI feedback with rollback capabilities
- **Performance Optimization**: Strategic memoization and virtual scrolling
- **Error Resilience**: Comprehensive error boundaries with graceful fallbacks

### Anti-Coordination Principles
- **NO Redis** - Use SQLite for everything with proper testing
- **NO message queues** - Use tokio channels with actor pattern
- **NO event stores** - Direct database operations
- **NO coordination frameworks** - Simple message passing
- **Single binary** with embedded assets using rust-embed

## Current Configuration
- Verbose mode: enabled
- Notifications: terminal_bell
- Output style: Explanatory
- Auto-compact: disabled

## Working Practices

### Effective Prompting
- Be specific and incremental rather than broad
- Use role-playing: "You are a Rust expert and React architect"
- Include documentation and examples in prompts
- Generate tests or types first, then implement
- Iterate and refine based on feedback

### Quality Assurance
- Small commits/checkpoints after each Claude operation
- Test early and often using existing test suite
- Use code reviews with separate Claude instances
- Leverage static analysis tools (cargo clippy, ESLint, etc.)
- Follow Plan-Implement-Review loop

### Technology Stack Constraints
- **Backend**: Rust with Axum, SQLite with sqlx, tokio for async
- **Frontend**: React with TypeScript, Zustand + Immer, React Query
- **Database**: SQLite only with compile-time query validation
- **Real-time**: WebSocket with actor pattern and automatic reconnection
- **Testing**: Property-based testing with proptest, integration tests with real SQLite
- **Forbidden**: Redis, message queues, event stores, coordination frameworks

### Architecture Understanding
- Start with broad questions, progressively narrow down
- Use focused queries for specific subsystems
- Ask Claude to define project-specific terminology
- Generate automated architecture notes for complex modules

## Commands Reference
- `/clear` - Reset conversation history
- `/agents` - List available agents
- `/config` - Configure settings
- `Shift+Tab` twice - Enter plan mode
- `claude --permission-mode plan` - Start in plan mode

## Important Notes
- Claude Code uses GLM-4.5 model via Z.AI integration
- Hybrid model setup: GLM-4.5 for complex tasks, GLM-4.5-Air for auxiliary tasks
- Always review changes before committing
- Use version control diligently throughout rewrite process

## Executable Specification Methodology

### Definition of Done
Implementation is considered "flawless" when all verification steps pass:
1. **Static Analysis**: `cargo fmt --check && cargo clippy -- -D warnings` (Zero errors)
2. **Unit/Property Tests**: `cargo test --lib` (100% Pass Rate)
3. **Integration Tests**: `cargo test --test integration` (100% Pass Rate)
4. **E2E User Journeys**: `npm run test:e2e` (All journeys pass)
5. **Constraint Compliance**: Manual review against anti-coordination constraints
6. **Rails Parity Validation**: Behavioral equivalence verified

### TDD Development Cycle
```
TYPE CONTRACTS → PROPERTY TESTS → INTEGRATION CONTRACTS → IMPLEMENTATION → VALIDATION
       ↓               ↓                    ↓                  ↓             ↓
   Complete        Behavior           Service            Type-Guided    Comprehensive
   Interface       Properties         Boundaries         Implementation    Testing
   Design          Specification      Definition         Following         Validation
                                                        Contracts
```

### Critical Architecture Overview
**System Components**: Client Layer → Server Layer → Service Layer → Data Layer
**Data Flow**: HTTP/WebSocket → API Handlers → Services → Database Writer → SQLite
**Real-time**: ActionCable-inspired WebSocket broadcasting with room-based channels
**Deployment**: Single binary with embedded React SPA, SQLite in mounted volume

## 5 Critical Gaps Implementation Contracts

**Core Architecture**: These gaps represent the only coordination complexity we accept - each has proven Rails solutions that we replicate exactly.

### Gap #1: Message Deduplication
- **Problem**: Rapid clicking creates duplicate messages
- **Rails Solution**: UNIQUE constraints + graceful handling
- **Implementation**: `create_message_with_deduplication` with UNIQUE constraint on (client_message_id, room_id)
- **Test**: Property test ensures same client_message_id always returns same message

### Gap #2: WebSocket Reconnection
- **Problem**: Network drops cause missed messages
- **Rails Solution**: ActionCable connection state tracking
- **Implementation**: `handle_websocket_reconnection` with last_seen_message_id tracking
- **Test**: Integration test with real WebSocket disconnect/reconnect cycles

### Gap #3: Write Serialization
- **Problem**: Concurrent writes corrupt database
- **Rails Solution**: Connection pooling serialization
- **Implementation**: `DedicatedWriter` task with mpsc channel for all write operations
- **Test**: Property test verifies concurrent writes are properly serialized

### Gap #4: Session Security
- **Problem**: Weak tokens enable session hijacking
- **Rails Solution**: SecureRandom cryptographic generation
- **Implementation**: `create_secure_session` with 32+ character alphanumeric tokens
- **Test**: Property test verifies sufficient token entropy

### Gap #5: Presence Tracking
- **Problem**: Connection state becomes inconsistent
- **Rails Solution**: Heartbeat cleanup with TTL
- **Implementation**: `update_user_presence` with 60-second heartbeat and cleanup
- **Test**: Property test verifies TTL-based cleanup works correctly

## Key Pattern References

### Core Architecture Patterns
- **Type Contracts**: Always define complete function signatures before implementation
- **Property Tests**: Use proptest for invariants and edge cases
- **Actor Pattern**: Use message passing for state management, avoid shared mutable state
- **RAII**: Automatic resource cleanup through Drop trait
- **Zero-Cost Abstractions**: Leverage Rust's performance without complexity

### Anti-Coordination Principles
- **Rails Parity**: If Rails doesn't do it, we don't do it
- **Direct Operations**: Simple INSERT/UPDATE/SELECT queries, no coordination layer
- **Single Binary**: Embedded assets, no orchestration or service discovery
- **Simple Error Handling**: Basic Result<T, E> with user-friendly messages

### TDD-First Methodology
- **Complete Function Signatures**: All input/output types and error cases specified
- **Property-Based Testing**: Define behavior through mathematical invariants
- **Integration Contracts**: Service boundaries and interaction patterns defined first
- **Type-Guided Implementation**: Implementation follows complete type contracts