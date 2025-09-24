# Parseltongue Project - Complete Table of Contents

## 🎯 Project Overview
Parseltongue v2 is a discovery-first architectural intelligence tool for Rust codebases. This TOC provides a comprehensive guide to all project files and their purposes.

## 📁 Core Project Structure

### `/src/` - Core Implementation
- **Main Entry Points**
  - `main.rs` - CLI application entry point
  - `lib.rs` - Library interface and module exports
  - `cli.rs` - Command-line interface implementation with Avengers-themed emojis

### `/src/discovery/` - Discovery Engine (Core Feature)
- **Core Engine**
  - `engine.rs` - Main discovery engine trait and implementation
  - `concurrent_discovery_engine.rs` - Thread-safe discovery with Arc<RwLock>
  - `simple_discovery_engine.rs` - Basic discovery implementation
  
- **Data Structures**
  - `enhanced_isg_node.rs` - ISG nodes with embedded file location data
  - `types.rs` - Core type definitions for discovery system
  - `indexes.rs` - Discovery indexes for efficient querying
  - `memory_optimized_indexes.rs` - Memory-efficient index implementations
- **Analysis & Output**
  - `blast_radius_analyzer.rs` - Human-readable impact analysis
  - `output_formatter.rs` - Multi-format output (human, JSON, PR summaries)
  - `json_output.rs` - JSON serialization for tooling integration
  
- **Workflow Orchestration**
  - `workflow_orchestrator.rs` - JTBD workflow trait definition
  - `concrete_workflow_orchestrator.rs` - Complete workflow implementations
  - `workspace_manager.rs` - Persistent analysis session management
  
- **Performance & Testing**
  - `performance_metrics.rs` - Built-in performance monitoring
  - `performance_benchmarks.rs` - Micro-benchmarks for critical paths
  - `integration_test.rs` - End-to-end integration tests
  - `*_tests.rs` - Various specialized test modules

### `/tests/` - Comprehensive Test Suite
- **Integration Tests**
  - `comprehensive_integration_tests.rs` - Full system integration validation
  - `end_to_end_workflow_validation.rs` - Complete JTBD workflow testing
  - `system_integration_final_wiring.rs` - Final system wiring validation-
 **Performance & Validation**
  - `task_23_performance_validation.rs` - Performance contract validation
  - `task_23_simple_validation.rs` - Basic functionality validation
  - `cross_platform_integration.rs` - Multi-platform testing
  
- **CLI & Workflow Testing**
  - `cli_end_to_end_integration.rs` - CLI command integration tests
  - `jtbd_workflow_commands_tests.rs` - Jobs-to-be-done workflow testing
  - `pt_shell_script_tests.rs` - Shell script integration testing

## 📚 Documentation Structure

### `/docs/` - Technical Documentation
- `ARCHITECTURE_OVERVIEW.md` - High-level system architecture
- `ISG_EXPLAINED.md` - In-memory Semantic Graph explanation
- `ONBOARDING_GUIDE.md` - Developer onboarding instructions
- `CLI_IMPLEMENTATION_SUMMARY.md` - CLI feature documentation
- `IMPLEMENTATION_NOTES.md` - Technical implementation details
- `CROSS_PLATFORM_TESTING.md` - Multi-platform testing strategy
- `DEMONSTRATION_RESULTS.md` - Demo results and validation##
# `/parseltongue_dungeon/` - LLM Integration Toolkit
**Purpose**: Ready-to-use scripts and instructions for LLM-assisted development

- **Scripts** (`/scripts/`)
  - `onboard_codebase.sh` - Complete codebase onboarding workflow
  - `generate_llm_context.sh` - Generate LLM-ready context files
  - `debug_entity.sh` - Debug specific entities with full context
  - `feature_impact.sh` - Analyze feature change impact
  - `self_analysis_and_cleanup.sh` - Self-improvement workflow
  - `timing_precision_demo.sh` - Performance timing demonstrations

- **LLM Instructions** (`/llm_instructions/`)
  - `codebase_analysis.md` - Instructions for codebase analysis
  - `architecture_review.md` - Architecture review guidelines
  - `refactor_planning.md` - Refactoring planning instructions

- **Demo Outputs** (`/demo_outputs/`)
  - `axum_exploration/` - Axum framework analysis demo
  - `parseltongue_self/` - Self-analysis demonstration

### `/.kiro/` - Kiro IDE Integration
- **Specifications** (`/specs/parseltongue-v2-discovery-first/`)
  - `requirements.md` - Complete requirements specification
  - `design.md` - Detailed system design document
  - `tasks.md` - Implementation task breakdown- **LLM Gu
ides & Reports**
  - `parseltongue-llm-guide.md` - LLM integration guide
  - `parseltongue-code-quality-report.md` - Code quality analysis
  - `task-24-completion-summary.md` - Task 24 completion report
  - `Bullet-Proof Mermaid Prompts_ Square-Perfect Diagrams from Any LLM.md` - Mermaid diagram guide

## 📊 Task Implementation Reports

### `/docs/task_reports/` - Implementation Task Documentation
- `TASK_17_IMPLEMENTATION_SUMMARY.md` - Discovery infrastructure setup
- `TASK_18_TDD_IMPLEMENTATION_SUMMARY.md` - TDD implementation for enhanced ISG nodes
- `TASK_19_TDD_IMPLEMENTATION_SUMMARY.md` - Entity listing and query interface
- `TASK_20_TDD_IMPLEMENTATION_SUMMARY.md` - File-based navigation implementation
- `TASK_21_INTEGRATION_SUMMARY.md` - System integration and wiring
- `TASK_23_FINAL_VALIDATION_REPORT.md` - Final validation report
- `TASK_23_FINAL_VALIDATION_REPORT_ACTUAL.md` - Actual validation results
- `TASK_23_IMPLEMENTATION_SUMMARY.md` - Task 23 detailed implementation
- `TASK_23_PERFORMANCE_VALIDATION_SUMMARY.md` - Performance validation results
- `TASK_25_AVENGERS_CLI_ENHANCEMENT_SUMMARY.md` - CLI enhancement with Avengers theme
- `TASK_27_CLEANUP_SUMMARY.md` - Folder structure cleanup and organization
- `COMPREHENSIVE_INTEGRATION_TESTS_SUMMARY.md` - Integration testing summary
- `PERFORMANCE_MONITORING_IMPLEMENTATION.md` - Performance monitoring setup
- `WORKSPACE_IMPLEMENTATION_VERIFICATION.md` - Workspace implementation verification## 🔧 Bu
ild & Development Files

### Core Build Configuration
- `Cargo.toml` - Rust project configuration and dependencies
- `Cargo.lock` - Dependency lock file
- `README.md` - Main project README with Mermaid diagrams

### Development Tools
- `pt` - Shell script for workflow orchestration (executable)
- `/scripts/run_cross_platform_tests.sh` - Cross-platform testing script

### IDE & Editor Configuration
- `.vscode/` - VS Code configuration
- `.cursorignore` - Cursor IDE ignore patterns
- `.trustedKiroCommands` - Trusted Kiro commands list

## 📁 User Feedback & Research

### `/A01UserFeedback/` - User Research & Feedback
- `README.md` - User feedback overview
- `PARSELTONGUE_BEST_PRACTICES_GUIDE.md` - Best practices guide
- `PARSELTONGUE_V2_RECOMMENDATIONS.md` - V2 recommendations
- `END_TO_END_WORKFLOW_VALIDATION_SUMMARY.md` - Workflow validation
- `RELATIONSHIP_ACCURACY_VALIDATION_SUMMARY.md` - Accuracy validation
- `workflow_patterns.md` - Workflow pattern documentation
- `workflow_templates.md` - Workflow templates#
# 🗄️ Archive & Historical Data

### `/zzzzArchive/` - Historical Implementation Data
- `parseltongue_codebase.dump` - Historical codebase dump
- `test_data/` - Test data for validation
- `_refTestDataAsLibraryTxt/` - Reference test data (Axum, etc.)
- `output/` - Historical output files

### `/parseltongue_workspace/` - Analysis Workspaces
- `onboarding_*/` - Onboarding analysis sessions
- `self_analysis_*/` - Self-analysis sessions with timestamps

## 🎯 Essential Files for New Codebase Integration

### **CRITICAL FILES** - Must be ported for LLM integration:

#### 1. Core Discovery Engine
```
src/discovery/engine.rs
src/discovery/concurrent_discovery_engine.rs
src/discovery/enhanced_isg_node.rs
src/discovery/blast_radius_analyzer.rs
src/discovery/output_formatter.rs
```

#### 2. Workflow Orchestration
```
src/discovery/concrete_workflow_orchestrator.rs
src/discovery/workspace_manager.rs
parseltongue_dungeon/scripts/onboard_codebase.sh
parseltongue_dungeon/scripts/generate_llm_context.sh
```#### 3. LL
M Integration Toolkit
```
parseltongue_dungeon/llm_instructions/codebase_analysis.md
parseltongue_dungeon/llm_instructions/architecture_review.md
parseltongue_dungeon/llm_instructions/refactor_planning.md
.kiro/parseltongue-llm-guide.md
```

#### 4. CLI & User Interface
```
src/cli.rs (with Avengers emojis)
pt (shell script orchestrator)
```

#### 5. Documentation & Specifications
```
.kiro/specs/parseltongue-v2-discovery-first/requirements.md
.kiro/specs/parseltongue-v2-discovery-first/design.md
README.md (with Mermaid diagrams)
docs/ARCHITECTURE_OVERVIEW.md
```

## 🚀 Quick Start for LLM Integration

### For New Codebase Setup:
1. Copy the **CRITICAL FILES** listed above
2. Run `parseltongue_dungeon/scripts/onboard_codebase.sh`
3. Use `parseltongue_dungeon/scripts/generate_llm_context.sh` for context generation
4. Reference `parseltongue_dungeon/llm_instructions/` for LLM prompts

### For Development:
1. Use `pt onboard` for codebase exploration
2. Use `pt feature-start` for impact analysis
3. Use `pt debug` for debugging workflows
4. Reference `.kiro/parseltongue-llm-guide.md` for LLM best practices

## 📋 File Organization Status

### ✅ Cleanup Actions Completed:

#### 1. Task Reports Organized in `/docs/task_reports/`
All task implementation summaries and reports have been moved to a dedicated directory for better organization and navigation.

#### 2. Analysis Files Organized in `/analysis/`
All analysis dumps, snapshots, and generated files have been consolidated into a dedicated analysis directory.

#### 3. Demo Files Organized in `/demos/`
All demo and standalone example files have been moved to a dedicated demos directory with proper documentation.

#### 4. Current Clean Root Directory Structure
After cleanup, root contains only essential files:
- **Core build files**: `Cargo.toml`, `Cargo.lock`
- **Main documentation**: `README.md`, `PARSELTONGUE_PROJECT_TOC.md`, `LLM_INTEGRATION_GUIDE.md`
- **Essential executable**: `pt` (workflow orchestrator)
- **IDE configuration**: `.cursorignore`, `.gitignore`, `.trustedKiroCommands`
- **Core directories**: `src/`, `tests/`, `docs/`, `parseltongue_dungeon/`, etc.

---

*This TOC serves as the definitive guide to the Parseltongue project structure and is essential for LLM-assisted development workflows.*