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

### Root Directory Task Reports
- `TASK_17_IMPLEMENTATION_SUMMARY.md` - Task 17 completion report
- `TASK_18_TDD_IMPLEMENTATION_SUMMARY.md` - TDD implementation summary
- `TASK_19_TDD_IMPLEMENTATION_SUMMARY.md` - Task 19 TDD summary
- `TASK_20_TDD_IMPLEMENTATION_SUMMARY.md` - Task 20 TDD summary
- `TASK_21_INTEGRATION_SUMMARY.md` - Integration task summary
- `TASK_23_FINAL_VALIDATION_REPORT.md` - Final validation report
- `TASK_23_FINAL_VALIDATION_REPORT_ACTUAL.md` - Actual validation results
- `TASK_23_IMPLEMENTATION_SUMMARY.md` - Task 23 implementation
- `TASK_23_PERFORMANCE_VALIDATION_SUMMARY.md` - Performance validation
- `TASK_25_AVENGERS_CLI_ENHANCEMENT_SUMMARY.md` - CLI enhancement summary## 🔧 Bu
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
4. Reference `.kiro/parseltongue-llm-guide.md` for LLM best practices## 📋 F
ile Organization Recommendations

### Suggested Cleanup Actions:

#### 1. Move Task Reports to `/docs/task_reports/`
```bash
mkdir -p docs/task_reports
mv TASK_*_SUMMARY.md docs/task_reports/
mv COMPREHENSIVE_INTEGRATION_TESTS_SUMMARY.md docs/task_reports/
mv PERFORMANCE_MONITORING_IMPLEMENTATION.md docs/task_reports/
mv WORKSPACE_IMPLEMENTATION_VERIFICATION.md docs/task_reports/
```

#### 2. Move Analysis Files to `/analysis/`
```bash
mkdir -p analysis
mv parseltongue_self_analysis.dump analysis/
mv parseltongue_self_dump.txt analysis/
mv parseltongue_snapshot.json analysis/
mv parseltongue_visualization.html analysis/
mv test_simple.dump analysis/
```

#### 3. Move Demo Files to `/demos/`
```bash
mkdir -p demos
mv workspace_demo* demos/
mv test_workspace_standalone* demos/
```

#### 4. Update Root Directory Structure
After cleanup, root should contain only:
- Core build files (Cargo.toml, Cargo.lock)
- Main documentation (README.md, this TOC)
- Essential executables (pt)
- Core directories (src/, tests/, docs/, etc.)

---

*This TOC serves as the definitive guide to the Parseltongue project structure and is essential for LLM-assisted development workflows.*