# Parseltongue TDD Implementation Tracker - ULTRATHINK EDITION

> **Ultra-Minimalist MVP Target**: ~10 users - Simplicity over complexity - Each tool does ONE thing well and reliably
>
> **Current Status**: 15% Complete | 3/6 Tools Functional ✅ | Critical Path in Progress 🚀
>
> **Branch**: `ultrathink` | **Last Updated**: 2025-10-29

---

## 🎯 ULTRATHINK MISSION STATEMENT

**Goal**: Complete the 6-tool pipeline with functional, idiomatic Rust following TDD principles
**Philosophy**: RED → GREEN → REFACTOR with functional programming patterns
**Timeline**: 2-3 weeks to MVP completion
**Quality Standard**: Every line of code backed by tests, every pattern idiomatic Rust

---

## 📊 Current State Assessment (ACCURATE - 2025-10-29)

### ✅ What We Have (Proven & Tested)

**Architecture Documentation (100% COMPLETE)**:
- [x] P01PRDL1Minimal.md - Ultra-minimalist principles and user stories
- [x] P02PRDL2Detailed.md - Detailed technical specifications
- [x] P03PRDL3VisualWorkflow.md - Visual workflow diagrams
- [x] P04PRDL4VisualJTBD.md - User journey and Jobs-To-Be-Done
- [x] P05PRDL5CommandsList.md - Command interface specifications
- [x] P06PRDL6AgentTruthSource.md - Agent reasoning orchestration
- [x] P07Arch01.md - Complete high-level and low-level design

**System Architecture (100% COMPLETE)**:
- [x] 4-Entity Architecture: LLM, CozoDB, CodeGraphContext.json, Codebase
- [x] 6-Tool Pipeline: folder-to-cozoDB-streamer → LLM-to-cozoDB-writer → LLM-cozoDB-to-context-writer → rust-preflight-code-simulator → LLM-cozoDB-to-code-writer → cozoDB-make-future-code-current
- [x] Ultra-Minimalist Design Principles: NO backup options, NO configuration complexity, single reliable operations
- [x] Multi-language Strategy: Tree-sitter foundation + Rust-first enhancements
- [x] Temporal Versioning System: (current_ind, future_ind, Future_Action) state tracking

**User Journey & Workflow (100% COMPLETE)**:
- [x] 5-Phase Workflow: Setup → Change Reasoning → Validation → File Writing → State Reset
- [x] 95% Agentic Interface + 5% Manual CLI usage pattern
- [x] Performance Targets: <30s indexing for 50k LOC, <1GB memory usage, <100k token context limit

**Working Tools (3/6 - 50% FUNCTIONAL)**:
- [x] **Tool 1 (parseltongue-01)**: folder-to-cozoDB-streamer ✅ COMPLETE
  - 6/6 tests passing (4 lib + 2 binary)
  - Tree-sitter parsing with ISGL1 key generation
  - Extracts 45 entities from 6 files in 16ms
  - CLI interface operational

- [x] **Tool 2 (parseltongue-02)**: LLM-to-cozoDB-writer ✅ COMPLETE
  - 12/12 tests passing (9 lib + 3 binary)
  - Temporal state management implemented
  - LLM client integration (mock for testing)

- [x] **Tool 3 (parseltongue-03)**: LLM-cozoDB-to-context-writer 🟡 MOSTLY COMPLETE
  - 16/17 tests passing (12 lib + 3/4 binary)
  - Context generation and token counting working
  - **1 FAILING TEST**: Binary integration test needs fix

**Build Status**:
- ✅ `cargo check --workspace` - PASSING (minor warnings only)
- ✅ `cargo test --workspace` - 97% PASS RATE (33/34 tests)
- ✅ Code compiles cleanly across all crates

### ❌ Critical Implementation Gaps

**Blocking Issues (MUST FIX)**:
- [x] ~~No source code exists~~ - **FALSE** (3 tools implemented!)
- [ ] **CozoDB using mocks** - Real database integration missing 🚨 CRITICAL
- [ ] **Tool 4 incomplete** - rust-analyzer integration partial ⏳ IN PROGRESS
- [ ] **Tool 5 status unknown** - File writer needs investigation ❓ UNKNOWN
- [ ] **Tool 6 missing** - State reset not implemented ❌ MISSING
- [ ] **No end-to-end tests** - Pipeline integration untested 🚨 CRITICAL

**Missing Core Functionality**:
- [ ] **Real CozoDB Storage**: Currently using mocks, cannot persist data
- [ ] **rust-analyzer Integration**: LSP client partial implementation
- [ ] **File Writing**: Ultra-minimalist file operations (Tool 5)
- [ ] **State Reset**: Delete table + re-index workflow (Tool 6)
- [ ] **Agent Orchestrator**: 95% agentic workflow not connected

---

# Phase 1: Technical Specification (Week 1)
> **Goal**: Convert architectural vision into implementable technical details

## 1.1 Core Data Structures & Schemas

### 1.1.1 CozoDB Schema Design
- [x] **Define CodeGraph table structure**
  - [x] Column specifications: ISGL1_key (primary), Current_Code, Future_Code, interface_signature, TDD_Classification, lsp_meta_data, current_ind, future_ind, Future_Action
  - [x] Data types: TEXT for code, JSON for metadata, BOOLEAN for indicators
  - [x] Indexes: ISGL1_key lookup, temporal state queries, dependency traversals
  - [x] Constraints: Unique ISGL1_key, valid temporal state combinations

- [x] **Define Relationships tables**
  - [x] Dependency graph structure: entity_a, entity_b, relationship_type, strength
  - [x] Hopping analysis tables: hop_1, hop_2, hop_n for blast radius calculations
  - [x] Temporal relationship tracking: current_dependencies, future_dependencies

### 1.1.2 Data Format Specifications
- [x] **ISGL1 Key Format Specification**
  - [x] Base format: `filepath-filename-InterfaceName`
  - [x] Edge case handling: nested modules, generic types, trait implementations
  - [x] Collision avoidance: hash suffixes for duplicate names
  - [x] Reverse mapping: ISGL1 key to file location resolution

- [x] **CodeGraphContext.json Schema**
  - [x] JSON structure definition: entities array, relationships, metadata
  - [x] Size optimization: exclude Current_Code, compress interface signatures
  - [x] Token counting methodology for <100k limit enforcement
  - [x] Prioritization rules for context inclusion/exclusion

- [x] **Interface Signature Representation**
  - [x] Structured metadata format: function name, parameters, return type, visibility
  - [x] Tree-sitter AST integration: node types, ranges, extraction rules
  - [x] Language-agnostic abstraction: common interface patterns across languages
  - [x] Rust-first enhancements: trait bounds, lifetimes, generic parameters

### 1.1.3 Temporal Versioning Logic
- [x] **State Transition Rules**
  - [x] (1,1) → (1,0): Delete operation logic and validation
  - [x] (1,1) → (0,1): Create operation validation and uniqueness
  - [x] (1,1) → (1,1): Modify operation with Future_Code update
  - [x] Conflict resolution: handling simultaneous modifications

- [x] **Temporal Consistency Rules**
  - [x] Dependency consistency: ensuring modified entities don't break dependents
  - [x] State validation: preventing invalid temporal combinations
  - [x] Rollback logic: handling failed operations with clean state reset

## 1.2 Algorithm Specifications

### 1.2.1 ISGL1 Chunking Algorithm
- [x] **Tree-sitter Integration**
  - [x] Language detection and parser selection
  - [x] Interface boundary identification: functions, structs, traits, impl blocks
  - [x] Chunk creation rules: maximum size limits, logical boundaries
  - [x] Cross-language compatibility: common patterns across supported languages

- [x] **Chunk Processing Pipeline**
  - [x] File parsing → interface extraction → ISGL1 key generation
  - [x] Metadata enrichment: TDD classification, LSP data integration
  - [x] Dependency analysis: import/use statement parsing
  - [x] Validation: syntax checking, semantic validation

### 1.2.2 Context Optimization Algorithm
- [ ] **Token Counting Implementation**
  - [ ] Accurate token estimation methodology for different LLM providers
  - [ ] Code vs metadata token differentiation
  - [ ] JSON structure overhead calculation
  - [ ] Language-specific token counting rules

- [ ] **Prioritization Algorithms**
  - [ ] Relevance scoring: change proximity, dependency importance
  - [ ] Blast radius inclusion: N-hop dependency analysis
  - [ ] Size-based pruning: removing less critical entities when near limit
  - [ ] Cache optimization: avoiding re-generation of unchanged context

### 1.2.3 Hopping & Blast Radius Analysis
- [ ] **Graph Traversal Algorithms**
  - [ ] 1-hop analysis: direct dependencies only
  - [ ] 2-hop analysis: dependencies of dependencies
  - [ ] N-hop analysis: configurable depth analysis
  - [ ] Performance optimization: efficient query patterns

- [ ] **Impact Assessment**
  - [ ] Change impact scoring: modification vs creation vs deletion
  - [ ] Critical path identification: essential dependencies
  - [ ] Risk assessment: high-risk change detection

## 1.3 Integration Protocols

### 1.3.1 LLM Communication Protocol
- [x] **API Integration Specification**
  - [x] Provider selection: Claude API integration details
  - [x] Request format: prompt structure, context embedding, conversation history
  - [x] Response parsing: structured response extraction, validation
  - [x] Error handling: retry logic, fallback strategies

- [x] **Prompt Engineering**
  - [x] Context presentation: optimal CodeGraphContext.json formatting
  - [x] Task specification: clear instruction templates
  - [x] Response format: structured JSON response requirements
  - [x] Confidence threshold management: ≥80% confidence validation

### 1.3.2 rust-analyzer LSP Integration
- [x] **Language Server Management**
  - [x] Server startup and configuration: rust-analyzer binary path
  - [x] Communication protocol: JSON-RPC message handling
  - [x] Request types: textDocument/documentSymbol, textDocument/semanticTokens
  - [x] Metadata extraction: type information, trait implementations, usage analysis

- [x] **Enhanced Validation Features**
  - [x] Real-time syntax and semantic validation
  - [x] Compilation checking: build integration
  - [x] Performance analysis: type checking overhead

### 1.3.3 Tool Communication Protocols
- [ ] **Data Exchange Formats**
  - [ ] Tool input/output specifications: JSON schemas for each tool
  - [ ] File-based communication: temporary files, cleanup procedures
  - [ ] Memory-based communication: shared data structures, serialization
  - [ ] Error propagation: structured error reporting

---

# Phase 2: Development Infrastructure (Week 2)
> **Goal**: Establish complete development environment and tooling

## 2.1 Project Structure Setup

### 2.1.1 Crate Structure Creation
- [x] **Create parseltongue-01 (folder-to-cozoDB-streamer)** ✅
  - [x] Basic crate structure: Cargo.toml, src/lib.rs, src/main.rs ✅
  - [x] Module organization: parsing/, indexing/, storage/ ✅
  - [x] Binary target: CLI interface for manual execution ✅
  - [x] Test structure: unit tests, integration tests, performance tests ✅

- [x] **Create parseltongue-02 (LLM-to-cozoDB-writer)** ✅
  - [x] Basic crate structure: Cargo.toml, src/lib.rs, src/main.rs ✅
  - [x] Module organization: llm_client/, temporal_writer/, cli/, errors/ ✅
  - [x] Binary target: CLI interface for manual execution ✅
  - [x] Test structure: unit tests, integration tests, temporal state tests ✅

- [ ] **Create parseltongue-03 (LLM-cozoDB-to-context-writer)**
  - [ ] Basic crate structure: Cargo.toml, src/lib.rs, src/main.rs
  - [ ] Module organization: context/, optimization/, validation/
  - [ ] Binary target: CLI interface for manual execution
  - [ ] Test structure: unit tests, integration tests, size validation tests

- [ ] **Create parseltongue-04 (rust-preflight-code-simulator)**
  - [ ] Basic crate structure: Cargo.toml, src/lib.rs, src/main.rs
  - [ ] Module organization: validation/, compilation/, testing/
  - [ ] Binary target: CLI interface for manual execution
  - [ ] Test structure: unit tests, integration tests, validation tests

- [ ] **Create parseltongue-05 (LLM-cozoDB-to-code-writer)**
  - [ ] Basic crate structure: Cargo.toml, src/lib.rs, src/main.rs
  - [ ] Module organization: writing/, validation/, file_operations/
  - [ ] Binary target: CLI interface for manual execution
  - [ ] Test structure: unit tests, integration tests, write validation tests

- [ ] **Create parseltongue-06 (cozoDB-make-future-code-current)**
  - [ ] Basic crate structure: Cargo.toml, src/lib.rs, src/main.rs
  - [ ] Module organization: reset/, reindexing/, state_management/
  - [ ] Binary target: CLI interface for manual execution
  - [ ] Test structure: unit tests, integration tests, state reset tests

### 2.1.2 Workspace Configuration
- [ ] **Root Cargo.toml Setup**
  - [ ] Workspace members configuration for all 6 crates
  - [ ] Shared dependencies: CozoDB, tree-sitter, tokio, serde, anyhow, thiserror
  - [ ] Development dependencies: testing frameworks, benchmarking tools
  - [ ] Build configuration: optimization settings, feature flags

- [ ] **Shared Library Structure**
  - [ ] parseltongue-core crate: common types, traits, utilities
  - [ ] Common error handling: ParseltongError enum, Result types
  - [ ] Shared data structures: Entity, Context, temporal types
  - [ ] Common traits: Tool, Repository, Parser interfaces

## 2.2 Dependencies & External Integration

### 2.2.1 CozoDB Integration
- [ ] **CozoDB Dependency Setup**
  - [ ] Local path dependency configuration: `.doNotCommit/.refGitHubRepo/cozo/...`
  - [ ] Alternative: public crate dependency fallback
  - [ ] Database initialization: schema creation, migration handling
  - [ ] Connection management: connection pooling, transaction handling

- [ ] **Database Schema Implementation**
  - [ ] CodeGraph table creation with exact specifications
  - [ ] Index creation for performance optimization
  - [ ] Query implementation: temporal queries, relationship traversals
  - [ ] Data migration and backup strategies (if needed)

### 2.2.2 Tree-sitter Integration
- [ ] **Language Parser Setup**
  - [ ] Rust tree-sitter grammar: tree-sitter-rust dependency
  - [ ] Additional language support: tree-sitter-javascript, tree-sitter-python, etc.
  - [ ] Parser initialization: language detection, grammar loading
  - [ ] Error handling: parse failure recovery, partial parsing

- [ ] **Interface Extraction Implementation**
  - [ ] AST traversal algorithms: node visitors, pattern matching
  - [ ] Interface identification: function, struct, trait, impl detection
  - [ ] Signature extraction: parameters, return types, visibility modifiers
  - [ ] Cross-language abstraction: common interface patterns

### 2.2.3 LLM API Integration
- [ ] **Claude API Client**
  - [ ] HTTP client setup: reqwest or similar HTTP library
  - [ ] Authentication: API key management, security considerations
  - [ ] Request handling: retry logic, rate limiting, error handling
  - [ ] Response parsing: JSON extraction, validation, error checking

### 2.2.4 rust-analyzer LSP Integration
- [ ] **LSP Client Implementation**
  - [ ] LSP server communication: JSON-RPC over stdio or TCP
  - [ ] Server lifecycle: startup, configuration, shutdown
  - [ ] Request handling: documentSymbol, semanticTokens, type definition
  - [ ] Metadata extraction: type information, trait implementations

## 2.3 Build & Development Tooling

### 2.3.1 Build System Configuration
- [ ] **Optimization Settings**
  - [ ] Release configuration: maximum optimization, binary size
  - [ ] Development configuration: fast compilation, debug information
  - [ ] Test configuration: test-time optimization, coverage reporting
  - [ ] Benchmark configuration: performance measurement settings

- [ ] **Feature Flags**
  - [ ] Language support features: rust-only, multi-language
  - [ ] LSP integration features: enhanced-validation, basic-validation
  - [ ] Development features: debugging, logging, profiling
  - [ ] Testing features: mock-dependencies, test-fixtures

### 2.3.2 Testing Infrastructure
- [ ] **TDD Framework Setup**
  - [ ] Unit testing framework: built-in Rust testing with enhancements
  - [ ] Integration testing: end-to-end workflow testing
  - [ ] Performance testing: automated performance validation
  - [ ] Property-based testing: edge case discovery

- [ ] **Test Fixtures & Mocks**
  - [ ] Sample codebases: small, medium, large test projects
  - [ ] Mock implementations: CozoDB mocks, LLM API mocks
  - [ ] Test data management: temporary files, database cleanup
  - [ ] Performance contracts: automated validation

### 2.3.3 Development Tooling
- [ ] **Development Scripts**
  - [ ] Setup script: dependency installation, database initialization
  - [ ] Build script: automated build, test, validation
  - [ ] Cleaning script: temporary file cleanup, database reset
  - [ ] Performance testing script: benchmark execution

- [ ] **Documentation & Examples**
  - [ ] API documentation: rustdoc integration
  - [ ] Usage examples: command-line examples, integration patterns
  - [ ] Development guide: setup, contribution guidelines
  - [ ] Architecture documentation: design decisions, trade-offs

---

# Phase 3: Core Implementation (Weeks 3-4)
> **Goal**: Implement the 6-tool pipeline following TDD methodology

## 3.1 Tool 1 & 6 Foundation (Indexing & Reset)

### 3.1.1 Tool 1: folder-to-cozoDB-streamer Implementation
- [ ] **RED Phase: Write Failing Tests**
  - [ ] Create test for basic Rust file indexing
  - [ ] Create test for multi-language indexing
  - [ ] Create test for interface extraction accuracy
  - [ ] Create test for CozoDB storage operations
  - [ ] Create test for performance targets (<30s for 50k LOC)

- [ ] **GREEN Phase: Minimal Working Implementation**
  - [ ] Basic file traversal and language detection
  - [ ] Simple tree-sitter parsing interface
  - [ ] Basic CozoDB storage functionality
  - [ ] Minimal error handling

- [ ] **REFACTOR Phase: Production Implementation**
  - [ ] Enhanced parsing with language-specific optimizations
  - [ ] Memory-efficient streaming processing
  - [ ] Comprehensive error handling and recovery
  - [ ] Performance optimization and validation
  - [ ] LSP metadata integration for Rust projects

### 3.1.2 Tool 6: cozoDB-make-future-code-current Implementation
- [ ] **RED Phase: Write Failing Tests**
  - [ ] Create test for CodeGraph table deletion
  - [ ] Create test for temporal state reset
  - [ ] Create test for re-indexing workflow
  - [ ] Create test for git integration
  - [ ] Create test for clean state validation

- [ ] **GREEN Phase: Minimal Working Implementation**
  - [ ] Basic CozoDB table deletion
  - [ ] Simple state reset logic
  - [ ] Trigger Tool 1 re-indexing
  - [ ] Basic git operations

- [ ] **REFACTOR Phase: Production Implementation**
  - [ ] Robust database operations with proper cleanup
  - [ ] Comprehensive state management
  - [ ] Git integration with commit management
  - [ ] Error handling and recovery procedures
  - [ ] Performance optimization

### 3.1.3 End-to-End Basic Workflow
- [ ] **Integration Tests**
  - [ ] Test complete File → Database → Reset workflow
  - [ ] Test temporal state consistency
  - [ ] Test performance across the complete workflow
  - [ ] Test error handling and recovery

## 3.2 Tool 2 & 3 Intelligence Layer

### 3.2.1 Tool 2: LLM-to-cozoDB-writer Implementation
- [ ] **RED Phase: Write Failing Tests**
  - [ ] Create test for temporal state updates
  - [ ] Create test for Create operation (0,1) transition
  - [ ] Create test for Edit operation (1,1) with Future_Code
  - [ ] Create test for Delete operation (1,0) transition
  - [ ] Create test for data consistency validation
  - [ ] Create test for conflict resolution

- [ ] **GREEN Phase: Minimal Working Implementation**
  - [ ] Basic temporal flag management
  - [ ] Simple CozoDB update operations
  - [ ] Minimal data consistency checks
  - [ ] Basic error handling

- [ ] **REFACTOR Phase: Production Implementation**
  - [ ] Comprehensive temporal state management
  - [ ] Advanced data consistency validation
  - [ ] Sophisticated conflict resolution
  - [ ] Performance optimization with batching
  - [ ] Robust error handling and recovery

### 3.2.2 Tool 3: LLM-cozoDB-to-context-writer Implementation
- [ ] **RED Phase: Write Failing Tests**
  - [ ] Create test for context extraction from CozoDB
  - [ ] Create test for token counting accuracy
  - [ ] Create test for <100k token limit enforcement
  - [ ] Create test for Current_Code exclusion
  - [ ] Create test for context optimization algorithms
  - [ ] Create test for performance targets (<500ms generation)

- [ ] **GREEN Phase: Minimal Working Implementation**
  - [ ] Basic CozoDB query execution
  - [ ] Simple JSON context generation
  - [ ] Basic token counting
  - [ ] Minimal size validation

- [ ] **REFACTOR Phase: Production Implementation**
  - [ ] Optimized CozoDB query patterns
  - [ ] Advanced context optimization algorithms
  - [ ] Sophisticated token counting and size management
  - [ ] Performance optimization with caching
  - [ ] Robust error handling and edge case management

### 3.2.3 Iterative Reasoning Workflow
- [ ] **Integration Tests**
  - [ ] Test Read → Edit → Refine reasoning cycle
  - [ ] Test confidence threshold validation (≥80%)
  - [ ] Test context optimization in iterative loops
  - [ ] Test LLM communication protocol
  - [ ] Test error handling and recovery

## 3.3 Tool 4 & 5 Validation Layer

### 3.3.1 Tool 4: rust-preflight-code-simulator Implementation
- [ ] **RED Phase: Write Failing Tests**
  - [ ] Create test for syntax validation level
  - [ ] Create test for build validation (cargo build)
  - [ ] Create test for test validation (cargo test)
  - [ ] Create test for language auto-detection
  - [ ] Create test for clear success/failure indicators
  - [ ] Create test for error message quality

- [ ] **GREEN Phase: Minimal Working Implementation**
  - [ ] Basic syntax checking
  - [ ] Simple cargo build execution
  - [ ] Minimal cargo test execution
  - [ ] Basic language detection

- [ ] **REFACTOR Phase: Production Implementation**
  - [ ] Multi-level validation pipeline
  - [ ] Enhanced Rust validation with LSP integration
  - [ ] Graceful degradation for other languages
  - [ ] Performance optimization with parallel execution
  - [ ] Comprehensive error reporting and recovery

### 3.3.2 Tool 5: LLM-cozoDB-to-code-writer Implementation
- [ ] **RED Phase: Write Failing Tests**
  - [ ] Create test for file writing operations
  - [ ] Create test for Create operation (new file creation)
  - [ ] Create test for Modify operation (existing file update)
  - [ ] Create test for Delete operation (file deletion)
  - [ ] Create test for NO backup options enforcement
  - [ ] Create test for atomic operations and consistency

- [ ] **GREEN Phase: Minimal Working Implementation**
  - [ ] Basic file writing functionality
  - [ ] Simple create/modify/delete operations
  - [ ] Minimal file system consistency checks
  - [ ] Basic error handling

- [ ] **REFACTOR Phase: Production Implementation**
  - [ ] Atomic file operations with proper validation
  - [ **Ultra-Minimalist Constraint**: NO backup options implementation
  - [ ] Comprehensive file system consistency validation
  - [ ] Performance optimization with batching
  - [ ] Robust error handling and recovery procedures
  - [ ] Clear success/failure indicators with actionable error messages

### 3.3.3 Complete Workflow Integration
- [ ] **End-to-End Integration Tests**
  - [ ] Test complete 6-tool pipeline execution
  - [ ] Test temporal versioning across all tools
  - [ ] Test ultra-minimalist constraints enforcement
  - [ ] Test performance targets across complete workflow
  - [ ] Test error handling and recovery across pipeline
  - [ ] Test real-world bug fixing scenarios

## 3.4 User Interface & Agent Integration

### 3.4.1 CLI Interface Implementation
- [ ] **Command Structure Implementation**
  - [ ] Tool 1: `parseltongue index <path>` command
  - [ ] Tool 2: `parseltongue reason <query>` command
  - [ ] Tool 3: `parseltongue context [--query <query>]` command
  - [ ] Tool 4: `parseltongue validate [--level syntax|build|test]` command
  - [ ] Tool 5: `parseltongue write [--validate]` command
  - [ ] Tool 6: `parseltongue reset [--keep-git]` command

- [ ] **User Experience Enhancement**
  - [ ] Progress indicators for long-running operations
  - [ ] Clear error messages with actionable guidance
  - [ ] Command validation and help system
  - [ ] Configuration file support (if needed for development)

### 3.4.2 Agent Integration Setup
- [ ] **Claude Code Agent Implementation**
  - [ ] Create reasoning-orchestrator.md agent specification
  - [ ] Implement agent workflow with 5-phase reasoning
  - [ ] Test agent integration with all 6 tools
  - [ ] Validate agent reasoning quality and effectiveness

- [ ] **Manual CLI Workflow Support**
  - [ ] Enable manual execution of individual tools
  - [ ] Support mixed approach (agent reasoning + manual execution)
  - [ ] Provide clear documentation for power users

---

# Progress Tracking Dashboard

## Overall Progress: 15% (Architecture Complete ✅ | Tool 01 Working 🎉)

> **🚀 MAJOR MILESTONE ACHIEVED**: parseltongue-01 (folder-to-cozoDB-streamer) is fully functional!
> - ✅ Successfully processes Rust files using tree-sitter parsing
> - ✅ Generates ISGL1 keys for code entities
> - ✅ Extracts 45 entities from 6 files in 16ms
> - ✅ CLI interface working with comprehensive options
> - ✅ All tests passing (4 lib tests + 2 binary tests)
> - ✅ Following TDD-first principles with ultra-minimalist approach

### Phase 1: Technical Specification - 22/41 tasks complete (54%)
- [x] 1.1 Core Data Structures & Schemas - 12/12 tasks ✅
- [x] 1.2 Algorithm Specifications - 4/6 tasks ✅
- [x] 1.3 Integration Protocols - 6/8 tasks ✅

### Phase 2: Development Infrastructure - 5/31 tasks complete (16%)
- [x] 2.1 Project Structure Setup - 4/13 tasks ✅
- [ ] 2.2 Dependencies & External Integration - 0/10 tasks
- [ ] 2.3 Build & Development Tooling - 0/8 tasks

### Phase 3: Core Implementation - 0/47 tasks complete (0%)
- [ ] 3.1 Tool 1 & 6 Foundation - 0/10 tasks
- [ ] 3.2 Tool 2 & 3 Intelligence Layer - 0/14 tasks
- [ ] 3.3 Tool 4 & 5 Validation Layer - 0/16 tasks
- [ ] 3.4 User Interface & Agent Integration - 0/7 tasks

---

# Risk Management & Decision Log

## High-Risk Areas
1. **CozoDB Integration Complexity** - Local path dependency may require setup
2. **LLM API Integration** - Rate limiting, cost management, reliability
3. **Performance Targets** - <30s indexing for 50k LOC is aggressive
4. **Multi-language Support** - Tree-sitter integration complexity varies by language
5. **Ultra-Minimalist Constraints** - NO backup options may impact user experience

## Mitigation Strategies
1. **Prototype Early**: Implement core algorithms before full pipeline
2. **Performance Validation**: Implement performance tests from the beginning
3. **Incremental Integration**: Test each tool individually before pipeline integration
4. **Fallback Planning**: Alternative implementations for high-risk components

## Decision Log
- **Decision Date**: 2025-10-29
- **Architecture Finalization**: P01-P07 documents complete and approved
- **Implementation Approach**: TDD-first with ultra-minimalist MVP constraints
- **Target Timeline**: 4 weeks to complete MVP implementation

---

**Last Updated**: 2025-10-29
**Next Review**: End of Phase 1 (Week 1)
**Success Criteria**: All 6 tools working in complete workflow with performance targets met

---

# 🚀 ULTRATHINK IMPLEMENTATION PLAN (2025-10-29 → 2025-11-19)

## Implementation Philosophy

**TDD Approach**: RED → GREEN → REFACTOR
**Functional Rust**: Immutability, pure functions, composition over inheritance
**Idiomatic Patterns**: Use `that-in-rust-idiomatic-patterns` agent for code review
**Agent-Driven**: Leverage `Explore` and `general-purpose` agents for complex tasks

---

## Week 1: Critical Path Foundation (Oct 29 - Nov 4)

### Task 1.1: Fix Failing Test in parseltongue-03 ⚡ IMMEDIATE
**Status**: 🔴 RED (test failing)
**Priority**: P0 - Blocking clean test suite
**Estimated Time**: 30 minutes

**TDD Approach**:
- 🔴 **RED**: Test currently failing (1/17 in parseltongue-03 binary)
- 🟢 **GREEN**: Fix the root cause to make test pass
- 🔵 **REFACTOR**: Ensure idiomatic Rust patterns

**Steps**:
```bash
# Step 1: Identify failing test
cargo test -p parseltongue-03 --bin parseltongue-03 -- --nocapture

# Step 2: Analyze failure reason
# Expected: Binary integration test for context generation

# Step 3: Apply fix with TDD discipline
# - Keep implementation minimal
# - Ensure test passes
# - Refactor for clarity

# Step 4: Verify all tests pass
cargo test -p parseltongue-03
```

**Success Criteria**:
- [ ] All 17 tests in parseltongue-03 passing
- [ ] No regressions in other tests
- [ ] Code follows functional Rust patterns

---

### Task 1.2: Investigate Tool 4, 5, 6 Status 🔍 DISCOVERY
**Status**: ❓ UNKNOWN
**Priority**: P0 - Need accurate assessment before planning
**Estimated Time**: 1 hour
**Agent**: `Explore` agent for thorough codebase analysis

**TDD Approach**:
- 🔍 **DISCOVERY**: Map current implementation state
- 📊 **ASSESSMENT**: Identify what exists vs what's needed
- 📋 **PLAN**: Create detailed task breakdown per tool

**Steps with Explore Agent**:
```bash
# Use Explore agent to analyze:
# 1. Tool 4 (parseltongue-04) - rust-preflight-code-simulator
#    - What's implemented vs mocked
#    - rust-analyzer integration status
#    - Test coverage and gaps

# 2. Tool 5 (parseltongue-05) - LLM-cozoDB-to-code-writer
#    - File writing implementation status
#    - Safety checks and validation
#    - Ultra-minimalist compliance (no backups)

# 3. Tool 6 (parseltongue-06) - cozoDB-make-future-code-current
#    - State reset implementation
#    - Table deletion and re-indexing
#    - Integration with Tool 1
```

**Deliverable**:
- [ ] Detailed gap analysis for Tools 4, 5, 6
- [ ] Updated TDD-Tracker.md with accurate status
- [ ] Implementation plan for each tool

---

### Task 1.3: Research CozoDB Integration from .doNotCommit/ 📚 LEARNING
**Status**: ⏳ PENDING
**Priority**: P0 - Blocks real data persistence
**Estimated Time**: 2 hours
**Agent**: `Explore` agent for reference repository analysis

**TDD Approach**:
- 📚 **LEARN**: Study CozoDB patterns from reference repo
- 🧪 **PROTOTYPE**: Create minimal working example
- ✅ **VALIDATE**: Ensure patterns match our architecture

**Steps with Explore Agent**:
```bash
# Explore .doNotCommit/.refGitHubRepo/ for CozoDB examples
# Focus areas:
# 1. Schema definition patterns
# 2. Query execution patterns
# 3. Transaction management
# 4. Error handling
# 5. Connection pooling
# 6. Integration with Rust async/await
```

**Questions to Answer**:
- [ ] How to define CodeGraph table schema in CozoDB?
- [ ] What's the query syntax for temporal versioning?
- [ ] How to handle transactions and rollbacks?
- [ ] What's the performance profile for our use case?
- [ ] Are there any gotchas or limitations?

**Deliverable**:
- [ ] CozoDB integration pattern document
- [ ] Working prototype with test data
- [ ] Schema definition for CodeGraph table

---

### Task 1.4: Enable Real CozoDB Storage in parseltongue-02 🗄️ CRITICAL
**Status**: 🔴 RED (using mocks)
**Priority**: P0 - Unblocks entire pipeline
**Estimated Time**: 4-6 hours
**Dependencies**: Task 1.3 complete

**TDD Approach - Detailed**:

#### Phase A: Write Failing Tests (RED)
```rust
// File: crates/parseltongue-02/tests/cozo_storage_integration.rs

#[tokio::test]
async fn test_real_cozo_connection() {
    // RED: This test will fail initially
    let db = CozoDbClient::new("mem").await.unwrap();
    assert!(db.is_connected().await);
}

#[tokio::test]
async fn test_create_code_graph_schema() {
    // RED: Schema creation not implemented
    let db = CozoDbClient::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Verify schema exists
    let tables = db.list_tables().await.unwrap();
    assert!(tables.contains(&"CodeGraph".to_string()));
}

#[tokio::test]
async fn test_insert_code_entity() {
    // RED: Insert operation not implemented
    let db = CozoDbClient::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    let entity = CodeEntity {
        isgl1_key: "test-file-rs-MyStruct".to_string(),
        current_code: Some("struct MyStruct {}".to_string()),
        interface_signature: "struct MyStruct".to_string(),
        tdd_classification: TddClassification::CodeImplementation,
        current_ind: true,
        future_ind: false,
        future_action: None,
        // ... other fields
    };

    db.insert_entity(&entity).await.unwrap();

    // Verify insertion
    let retrieved = db.get_entity("test-file-rs-MyStruct").await.unwrap();
    assert_eq!(retrieved.isgl1_key, entity.isgl1_key);
}

#[tokio::test]
async fn test_temporal_state_update() {
    // RED: Temporal update not implemented
    let db = CozoDbClient::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert entity
    let entity = /* ... */;
    db.insert_entity(&entity).await.unwrap();

    // Update temporal state (1,1) -> (1,0) for delete
    db.update_temporal_state(
        "test-file-rs-MyStruct",
        TemporalUpdate {
            future_ind: false,
            future_action: Some(FutureAction::Delete),
        }
    ).await.unwrap();

    // Verify update
    let updated = db.get_entity("test-file-rs-MyStruct").await.unwrap();
    assert_eq!(updated.current_ind, true);
    assert_eq!(updated.future_ind, false);
    assert_eq!(updated.future_action, Some(FutureAction::Delete));
}
```

#### Phase B: Minimal Implementation (GREEN)
```rust
// File: crates/parseltongue-02/src/storage/cozo_client.rs

use cozo::Cozo;
use anyhow::Result;

pub struct CozoDbClient {
    db: Cozo,
}

impl CozoDbClient {
    pub async fn new(engine: &str) -> Result<Self> {
        let db = Cozo::new(engine)?;
        Ok(Self { db })
    }

    pub async fn is_connected(&self) -> bool {
        // Simple connectivity check
        self.db.run_script("?[a] := [[1]]", Default::default()).is_ok()
    }

    pub async fn create_schema(&self) -> Result<()> {
        // Define CodeGraph relation schema
        let schema_query = r#"
            :create CodeGraph {
                isgl1_key: String =>
                current_code: String?,
                future_code: String?,
                interface_signature: String,
                tdd_classification: String,
                lsp_meta_data: Json?,
                current_ind: Bool,
                future_ind: Bool,
                future_action: String?
            }
        "#;

        self.db.run_script(schema_query, Default::default())?;
        Ok(())
    }

    pub async fn insert_entity(&self, entity: &CodeEntity) -> Result<()> {
        // Insert entity into CodeGraph
        let query = r#"
            ?[isgl1_key, current_code, interface_signature,
              tdd_classification, current_ind, future_ind] <- [
                [$isgl1_key, $current_code, $interface_signature,
                 $tdd_classification, $current_ind, $future_ind]
            ]
            :put CodeGraph { isgl1_key =>
                current_code, interface_signature,
                tdd_classification, current_ind, future_ind
            }
        "#;

        let params = /* convert entity to CozoDB params */;
        self.db.run_script(query, params)?;
        Ok(())
    }

    // ... other methods
}
```

#### Phase C: Refactor for Idiomatic Rust (REFACTOR)
```rust
// Use idiomatic-patterns agent to review:
// 1. Error handling patterns (Result vs Option)
// 2. Async/await usage
// 3. Resource management (RAII)
// 4. Functional composition
// 5. Type safety improvements
```

**Success Criteria**:
- [ ] All integration tests passing with real CozoDB
- [ ] Schema matches P07Arch01.md specifications
- [ ] CRUD operations working for CodeEntity
- [ ] Temporal state updates functional
- [ ] Performance within targets (<1ms queries)
- [ ] Code reviewed by idiomatic-patterns agent

---

### Task 1.5: Complete Tool 4 (rust-preflight-code-simulator) 🔬 VALIDATION
**Status**: 🟡 PARTIAL (structure exists, needs completion)
**Priority**: P1 - Critical for safe code changes
**Estimated Time**: 6-8 hours
**Agent**: `that-in-rust-idiomatic-patterns` for implementation review

**TDD Approach - Multi-Level Validation**:

#### Level 1: Syntax Validation (RED → GREEN → REFACTOR)
```rust
// File: crates/parseltongue-04/tests/syntax_validation_tests.rs

#[tokio::test]
async fn test_valid_rust_syntax_passes() {
    // RED: Syntax validator not implemented
    let validator = RustPreflightValidator::new().await.unwrap();

    let valid_code = r#"
        fn hello_world() -> String {
            "Hello, World!".to_string()
        }
    "#;

    let result = validator.validate_syntax(valid_code).await.unwrap();
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
}

#[tokio::test]
async fn test_invalid_rust_syntax_fails() {
    // RED: Should detect syntax errors
    let validator = RustPreflightValidator::new().await.unwrap();

    let invalid_code = r#"
        fn broken_function( {
            // Missing closing parenthesis
        }
    "#;

    let result = validator.validate_syntax(invalid_code).await.unwrap();
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].contains("expected")); // Descriptive error
}
```

#### Level 2: Build Validation (RED → GREEN → REFACTOR)
```rust
#[tokio::test]
async fn test_valid_code_builds() {
    // RED: Build validation not implemented
    let validator = RustPreflightValidator::new().await.unwrap();

    let buildable_code = r#"
        pub struct MyStruct {
            field: i32,
        }

        impl MyStruct {
            pub fn new(field: i32) -> Self {
                Self { field }
            }
        }
    "#;

    let result = validator.validate_build(buildable_code).await.unwrap();
    assert!(result.is_valid);
}

#[tokio::test]
async fn test_type_error_caught_in_build() {
    // RED: Should catch type errors
    let validator = RustPreflightValidator::new().await.unwrap();

    let type_error_code = r#"
        fn add_numbers(a: i32, b: i32) -> String {
            a + b // Type mismatch: returns i32, not String
        }
    "#;

    let result = validator.validate_build(type_error_code).await.unwrap();
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.contains("mismatched types")));
}
```

#### Level 3: Test Validation (RED → GREEN → REFACTOR)
```rust
#[tokio::test]
async fn test_passing_tests_validate() {
    // RED: Test validation not implemented
    let validator = RustPreflightValidator::new().await.unwrap();

    let code_with_tests = r#"
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_add() {
                assert_eq!(add(2, 3), 5);
            }
        }
    "#;

    let result = validator.validate_tests(code_with_tests).await.unwrap();
    assert!(result.is_valid);
    assert_eq!(result.tests_passed, 1);
    assert_eq!(result.tests_failed, 0);
}

#[tokio::test]
async fn test_failing_tests_detected() {
    // RED: Should detect test failures
    let validator = RustPreflightValidator::new().await.unwrap();

    let code_with_failing_test = r#"
        pub fn buggy_add(a: i32, b: i32) -> i32 {
            a - b // BUG: Should be +, not -
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_add() {
                assert_eq!(buggy_add(2, 3), 5); // Will fail
            }
        }
    "#;

    let result = validator.validate_tests(code_with_failing_test).await.unwrap();
    assert!(!result.is_valid);
    assert_eq!(result.tests_failed, 1);
}
```

#### Implementation: Functional Rust Pattern
```rust
// File: crates/parseltongue-04/src/validator.rs

use std::process::Command;
use anyhow::Result;

pub struct RustPreflightValidator {
    temp_dir: tempfile::TempDir,
}

impl RustPreflightValidator {
    pub async fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        Ok(Self { temp_dir })
    }

    pub async fn validate_syntax(&self, code: &str) -> Result<ValidationResult> {
        // Use syn crate for syntax parsing (functional approach)
        syn::parse_file(code)
            .map(|_| ValidationResult::valid())
            .or_else(|e| Ok(ValidationResult::invalid(vec![e.to_string()])))
    }

    pub async fn validate_build(&self, code: &str) -> Result<ValidationResult> {
        // Create temporary Cargo project
        let project_path = self.create_temp_project(code)?;

        // Run cargo build
        let output = Command::new("cargo")
            .args(&["build", "--quiet"])
            .current_dir(&project_path)
            .output()?;

        // Parse output functionally
        Self::parse_build_output(output)
    }

    pub async fn validate_tests(&self, code: &str) -> Result<ValidationResult> {
        // Create temporary Cargo project with tests
        let project_path = self.create_temp_project(code)?;

        // Run cargo test
        let output = Command::new("cargo")
            .args(&["test", "--quiet"])
            .current_dir(&project_path)
            .output()?;

        // Parse test results functionally
        Self::parse_test_output(output)
    }

    // Private helper methods using functional patterns
    fn create_temp_project(&self, code: &str) -> Result<PathBuf> {
        // Create minimal Cargo.toml + src/lib.rs
        // Use functional file operations
        todo!("Implement with functional Rust patterns")
    }

    fn parse_build_output(output: std::process::Output) -> Result<ValidationResult> {
        // Functional parsing of cargo output
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(ValidationResult::valid())
        } else {
            let errors = stderr
                .lines()
                .filter(|line| line.contains("error:"))
                .map(|line| line.to_string())
                .collect();
            Ok(ValidationResult::invalid(errors))
        }
    }

    fn parse_test_output(output: std::process::Output) -> Result<ValidationResult> {
        // Functional parsing of test results
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Extract test counts using functional patterns
        let (passed, failed) = stdout
            .lines()
            .find(|line| line.contains("test result:"))
            .and_then(Self::parse_test_result_line)
            .unwrap_or((0, 0));

        Ok(ValidationResult {
            is_valid: failed == 0,
            tests_passed: passed,
            tests_failed: failed,
            errors: vec![],
        })
    }

    fn parse_test_result_line(line: &str) -> Option<(usize, usize)> {
        // Functional parsing: "test result: ok. 5 passed; 0 failed"
        todo!("Parse using functional patterns")
    }
}
```

**Success Criteria**:
- [ ] All 3 validation levels working (syntax, build, tests)
- [ ] Functional Rust patterns throughout
- [ ] Reviewed by idiomatic-patterns agent
- [ ] Performance: <5s for typical validation
- [ ] Clear, actionable error messages

---

## Week 2: Missing Tools Implementation (Nov 5 - Nov 11)

### Task 2.1: Implement Tool 5 (LLM-cozoDB-to-code-writer) 📝 FILE WRITING
**Status**: ❓ UNKNOWN (needs investigation first)
**Priority**: P1 - Critical for applying changes
**Estimated Time**: 6-8 hours
**Agent**: `that-in-rust-idiomatic-patterns` for review

**Ultra-Minimalist Requirements** (from P07Arch01.md):
- ✅ NO backup options or file versioning
- ✅ NO multiple safety levels or rollbacks
- ✅ Direct file system operations
- ✅ Single reliable write operation

**TDD Approach**:

#### Phase A: Write Failing Tests (RED)
```rust
// File: crates/parseltongue-05/tests/file_writer_tests.rs

#[tokio::test]
async fn test_create_new_file() {
    // RED: File creation not implemented
    let temp_dir = tempfile::tempdir().unwrap();
    let writer = FileWriter::new(temp_dir.path()).await.unwrap();

    let entity = CodeEntity {
        isgl1_key: "src-new_module-rs-MyStruct".to_string(),
        future_code: Some("struct MyStruct {}".to_string()),
        future_action: Some(FutureAction::Create),
        // ...
    };

    let result = writer.write_entity(&entity).await.unwrap();

    assert!(result.is_success);
    assert!(temp_dir.path().join("src/new_module.rs").exists());
}

#[tokio::test]
async fn test_modify_existing_file() {
    // RED: File modification not implemented
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("src/existing.rs");

    // Create initial file
    std::fs::write(&file_path, "fn old_code() {}").unwrap();

    let writer = FileWriter::new(temp_dir.path()).await.unwrap();

    let entity = CodeEntity {
        isgl1_key: "src-existing-rs-NewFunc".to_string(),
        future_code: Some("fn new_code() {}".to_string()),
        future_action: Some(FutureAction::Edit),
        // ...
    };

    let result = writer.write_entity(&entity).await.unwrap();

    assert!(result.is_success);
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("fn new_code()"));
}

#[tokio::test]
async fn test_delete_file() {
    // RED: File deletion not implemented
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("src/delete_me.rs");

    // Create file to delete
    std::fs::write(&file_path, "fn to_delete() {}").unwrap();
    assert!(file_path.exists());

    let writer = FileWriter::new(temp_dir.path()).await.unwrap();

    let entity = CodeEntity {
        isgl1_key: "src-delete_me-rs-ToDelete".to_string(),
        future_action: Some(FutureAction::Delete),
        // ...
    };

    let result = writer.write_entity(&entity).await.unwrap();

    assert!(result.is_success);
    assert!(!file_path.exists());
}

#[tokio::test]
async fn test_ultra_minimalist_no_backups() {
    // RED: Verify NO backup files are created
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("src/file.rs");

    std::fs::write(&file_path, "original content").unwrap();

    let writer = FileWriter::new(temp_dir.path()).await.unwrap();

    let entity = CodeEntity {
        future_code: Some("new content".to_string()),
        future_action: Some(FutureAction::Edit),
        // ...
    };

    writer.write_entity(&entity).await.unwrap();

    // Verify NO backup files exist
    let backup_patterns = ["*.bak", "*.backup", "*~", "*.old"];
    for pattern in &backup_patterns {
        let backup_files: Vec<_> = glob::glob(
            &format!("{}/**/{}", temp_dir.path().display(), pattern)
        ).unwrap().collect();
        assert!(backup_files.is_empty(), "No backup files should exist");
    }
}
```

#### Phase B: Minimal Implementation (GREEN)
```rust
// File: crates/parseltongue-05/src/file_writer.rs

use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct FileWriter {
    root_path: PathBuf,
}

impl FileWriter {
    pub async fn new(root_path: &Path) -> Result<Self> {
        Ok(Self {
            root_path: root_path.to_path_buf(),
        })
    }

    pub async fn write_entity(&self, entity: &CodeEntity) -> Result<WriteResult> {
        match entity.future_action {
            Some(FutureAction::Create) => self.create_file(entity).await,
            Some(FutureAction::Edit) => self.modify_file(entity).await,
            Some(FutureAction::Delete) => self.delete_file(entity).await,
            None => Ok(WriteResult::no_op()),
        }
    }

    async fn create_file(&self, entity: &CodeEntity) -> Result<WriteResult> {
        let file_path = self.resolve_file_path(&entity.isgl1_key)?;

        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Write file (ultra-minimalist: direct write, no backups)
        let content = entity.future_code
            .as_ref()
            .ok_or_else(|| anyhow!("Future code missing for create operation"))?;

        tokio::fs::write(&file_path, content).await?;

        Ok(WriteResult::success(file_path))
    }

    async fn modify_file(&self, entity: &CodeEntity) -> Result<WriteResult> {
        let file_path = self.resolve_file_path(&entity.isgl1_key)?;

        // Ultra-minimalist: Direct overwrite, no backups
        let content = entity.future_code
            .as_ref()
            .ok_or_else(|| anyhow!("Future code missing for edit operation"))?;

        tokio::fs::write(&file_path, content).await?;

        Ok(WriteResult::success(file_path))
    }

    async fn delete_file(&self, entity: &CodeEntity) -> Result<WriteResult> {
        let file_path = self.resolve_file_path(&entity.isgl1_key)?;

        // Ultra-minimalist: Direct delete, no backups
        tokio::fs::remove_file(&file_path).await?;

        Ok(WriteResult::success(file_path))
    }

    fn resolve_file_path(&self, isgl1_key: &str) -> Result<PathBuf> {
        // Parse ISGL1 key: "filepath-filename-InterfaceName"
        // Extract filepath-filename portion
        let parts: Vec<&str> = isgl1_key.rsplitn(2, '-').collect();
        let path_part = parts.get(1)
            .ok_or_else(|| anyhow!("Invalid ISGL1 key format"))?;

        // Convert to file path: "src-module" -> "src/module.rs"
        let file_path = path_part.replace('-', "/") + ".rs";

        Ok(self.root_path.join(file_path))
    }
}
```

#### Phase C: Refactor (REFACTOR)
- Use idiomatic-patterns agent to review
- Ensure functional patterns (immutability, pure functions where possible)
- Add comprehensive error types (thiserror)
- Validate ultra-minimalist compliance

**Success Criteria**:
- [ ] All file operations working (create, modify, delete)
- [ ] NO backup files created (ultra-minimalist verified)
- [ ] Atomic operations where possible
- [ ] Clear error messages
- [ ] Functional Rust patterns throughout
- [ ] Reviewed by idiomatic-patterns agent

---

### Task 2.2: Implement Tool 6 (cozoDB-make-future-code-current) 🔄 STATE RESET
**Status**: ❌ MISSING
**Priority**: P1 - Required for iterative workflow
**Estimated Time**: 4-6 hours

**Ultra-Minimalist Requirements** (from P07Arch01.md):
- ✅ NO backup metadata files or snapshots
- ✅ NO configuration options or complexity
- ✅ Delete table → Re-index → Complete
- ✅ Clean state for each iteration

**TDD Approach**:

#### Phase A: Write Failing Tests (RED)
```rust
// File: crates/parseltongue-06/tests/state_reset_tests.rs

#[tokio::test]
async fn test_delete_code_graph_table() {
    // RED: Table deletion not implemented
    let db = CozoDbClient::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert some test data
    db.insert_entity(&test_entity()).await.unwrap();

    let state_reset = StateResetManager::new(db).await.unwrap();

    state_reset.delete_code_graph_table().await.unwrap();

    // Verify table is gone
    let tables = state_reset.list_tables().await.unwrap();
    assert!(!tables.contains(&"CodeGraph".to_string()));
}

#[tokio::test]
async fn test_reset_triggers_reindex() {
    // RED: Re-indexing trigger not implemented
    let db = CozoDbClient::new("mem").await.unwrap();
    let indexer = FileStreamer::new(/* ... */);
    let state_reset = StateResetManager::new(db, indexer).await.unwrap();

    let test_dir = tempfile::tempdir().unwrap();
    // Create test Rust files
    std::fs::write(test_dir.path().join("test.rs"), "fn test() {}").unwrap();

    state_reset.reset_and_reindex(test_dir.path()).await.unwrap();

    // Verify re-indexing happened
    let entities = state_reset.get_all_entities().await.unwrap();
    assert!(!entities.is_empty());
}

#[tokio::test]
async fn test_ultra_minimalist_no_backups() {
    // RED: Verify NO backup metadata files created
    let db = CozoDbClient::new("mem").await.unwrap();
    let state_reset = StateResetManager::new(db).await.unwrap();

    state_reset.reset_and_reindex(test_path).await.unwrap();

    // Verify NO backup files exist
    // Check for .backup, .snapshot, .meta files
    // Should find NONE (ultra-minimalist)
}
```

#### Phase B: Minimal Implementation (GREEN)
```rust
// File: crates/parseltongue-06/src/state_reset.rs

pub struct StateResetManager {
    db: CozoDbClient,
    indexer: FileStreamer,
}

impl StateResetManager {
    pub async fn new(db: CozoDbClient, indexer: FileStreamer) -> Result<Self> {
        Ok(Self { db, indexer })
    }

    pub async fn delete_code_graph_table(&self) -> Result<()> {
        // Ultra-minimalist: Simple table deletion
        let query = ":rm CodeGraph";
        self.db.run_script(query, Default::default()).await?;
        Ok(())
    }

    pub async fn reset_and_reindex(&self, codebase_path: &Path) -> Result<()> {
        // Step 1: Delete CodeGraph table (ultra-minimalist)
        self.delete_code_graph_table().await?;

        // Step 2: Recreate schema
        self.db.create_schema().await?;

        // Step 3: Trigger re-indexing (call Tool 1)
        self.indexer.index_directory(codebase_path).await?;

        Ok(())
    }
}
```

**Success Criteria**:
- [ ] Table deletion working
- [ ] Re-indexing triggered automatically
- [ ] NO backup files created (ultra-minimalist verified)
- [ ] Clean state reset for iterative workflow
- [ ] Integration with Tool 1 (file streamer)

---

## Week 3: Integration & Testing (Nov 12 - Nov 18)

### Task 3.1: Create End-to-End Integration Test 🔗 PIPELINE
**Status**: ⏳ PENDING
**Priority**: P0 - Validates complete workflow
**Estimated Time**: 8-10 hours

**TDD Approach - Complete Pipeline Test**:

```rust
// File: tests/e2e_pipeline_test.rs

#[tokio::test]
async fn test_complete_6_tool_pipeline() {
    // This is the ultimate integration test
    // Tests all 6 tools working together

    // Setup test environment
    let test_repo = setup_test_rust_repo().await;
    let db = CozoDbClient::new("mem").await.unwrap();

    // TOOL 1: Index codebase
    let indexer = FileStreamer::new(&db);
    indexer.index_directory(&test_repo).await.unwrap();

    // Verify indexing
    let entities = db.get_all_entities().await.unwrap();
    assert!(!entities.is_empty());

    // TOOL 2: LLM writes temporal changes
    let llm_writer = LlmToCozoWriter::new(&db);
    let change_request = "Fix the bug in calculate_total()";
    llm_writer.process_request(change_request).await.unwrap();

    // Verify temporal flags set
    let changed_entities = db.get_entities_with_future_action().await.unwrap();
    assert!(!changed_entities.is_empty());

    // TOOL 3: Generate context for LLM reasoning
    let context_writer = ContextWriter::new(&db);
    let context = context_writer.generate_context().await.unwrap();

    // Verify context size < 100k tokens
    assert!(context.token_count() < 100_000);

    // TOOL 4: Validate proposed changes
    let validator = RustPreflightValidator::new().await.unwrap();
    for entity in &changed_entities {
        if let Some(future_code) = &entity.future_code {
            let result = validator.validate_all(future_code).await.unwrap();
            assert!(result.is_valid, "Validation failed: {:?}", result.errors);
        }
    }

    // TOOL 5: Write changes to files (ultra-minimalist)
    let file_writer = FileWriter::new(&test_repo);
    for entity in &changed_entities {
        file_writer.write_entity(entity).await.unwrap();
    }

    // Verify files changed on disk
    // Run actual cargo test to ensure changes work
    let test_output = Command::new("cargo")
        .args(&["test"])
        .current_dir(&test_repo)
        .output()
        .unwrap();
    assert!(test_output.status.success(), "Tests should pass after changes");

    // TOOL 6: Reset state for next iteration
    let state_reset = StateResetManager::new(db.clone(), indexer.clone());
    state_reset.reset_and_reindex(&test_repo).await.unwrap();

    // Verify clean state
    let reset_entities = db.get_all_entities().await.unwrap();
    assert!(reset_entities.iter().all(|e| e.future_action.is_none()));

    println!("✅ COMPLETE 6-TOOL PIPELINE TEST PASSED!");
}

fn setup_test_rust_repo() -> TempDir {
    // Create minimal Rust project with intentional bug
    let temp = tempfile::tempdir().unwrap();

    // Create Cargo.toml
    std::fs::write(
        temp.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
        "#
    ).unwrap();

    // Create src/lib.rs with bug
    std::fs::create_dir(temp.path().join("src")).unwrap();
    std::fs::write(
        temp.path().join("src/lib.rs"),
        r#"
pub fn calculate_total(items: &[i32]) -> i32 {
    items.iter().sum() - 1  // BUG: Should not subtract 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total() {
        assert_eq!(calculate_total(&[1, 2, 3]), 6);
    }
}
        "#
    ).unwrap();

    temp
}
```

**Success Criteria**:
- [ ] End-to-end test passes
- [ ] All 6 tools integrated
- [ ] Real code changes applied
- [ ] Tests pass after changes
- [ ] State reset works
- [ ] Performance within targets

---

## Tracking & Reporting

### Daily Progress Updates
- Update this file daily with completed tasks
- Mark tasks as: 🔴 RED | 🟢 GREEN | 🔵 REFACTOR | ✅ COMPLETE
- Track time spent vs estimated
- Note blockers and dependencies

### Code Review Checkpoints
- After each GREEN phase: Run `that-in-rust-idiomatic-patterns` agent
- After each REFACTOR: Verify functional Rust patterns
- Before marking complete: Full test suite passes

### Success Metrics
- [ ] All 34+ tests passing (currently 33/34)
- [ ] All 6 tools functional
- [ ] End-to-end pipeline working
- [ ] Performance targets met
- [ ] Idiomatic Rust throughout
- [ ] Zero technical debt

---

**ULTRATHINK MANTRA**:
> RED → GREEN → REFACTOR → REVIEW → REPEAT
> Every line tested, every pattern idiomatic, every tool ultra-minimalist