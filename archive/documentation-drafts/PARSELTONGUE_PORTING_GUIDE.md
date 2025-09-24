# Parseltongue Porting Guide for LLM Integration

## 🎯 Task 27 Completion: Explicit Porting Instructions

This document provides **EXPLICIT** instructions for which files and folders need to be ported to new codebases for LLM integration with Parseltongue.

## 📁 Clean Project Structure (Post-Cleanup)

### ✅ Organized Directory Structure
```
parseltongue/
├── src/                           # Core implementation
├── tests/                         # Comprehensive test suite
├── docs/                          # All documentation
│   └── task_reports/             # Task implementation reports (moved from root)
├── analysis/                      # Analysis dumps and outputs (moved from root)
├── demos/                         # Demo files (moved from root)
├── parseltongue_dungeon/          # LLM integration toolkit
├── .kiro/                         # Kiro IDE specifications
├── examples/                      # Usage examples
├── scripts/                       # Build and utility scripts
├── README.md                      # Main project documentation
├── PARSELTONGUE_PROJECT_TOC.md    # Complete table of contents
├── LLM_INTEGRATION_GUIDE.md       # LLM integration instructions
├── pt                             # Workflow orchestrator (executable)
├── Cargo.toml                     # Rust project configuration
└── Cargo.lock                     # Dependency lock file
```

## 🚀 EXPLICIT FILES TO PORT FOR LLM INTEGRATION

### **CRITICAL TIER 1: MINIMUM VIABLE SYSTEM** 
Copy these files for basic Parseltongue functionality:

```bash
# Core Discovery Engine (REQUIRED)
src/discovery/engine.rs
src/discovery/concurrent_discovery_engine.rs
src/discovery/enhanced_isg_node.rs
src/discovery/types.rs
src/discovery/mod.rs

# Core ISG System (REQUIRED)
src/isg.rs
src/lib.rs

# CLI Interface (REQUIRED)
src/cli.rs
src/main.rs

# Build Configuration (REQUIRED)
Cargo.toml
Cargo.lock

# Workflow Orchestrator (REQUIRED)
pt
```

**Result**: Basic entity discovery and CLI functionality

### **ESSENTIAL TIER 2: FULL LLM WORKFLOW INTEGRATION**
Add these files for complete LLM-assisted development:

```bash
# Advanced Discovery Features
src/discovery/blast_radius_analyzer.rs
src/discovery/output_formatter.rs
src/discovery/json_output.rs
src/discovery/indexes.rs
src/discovery/workspace_manager.rs
src/discovery/concrete_workflow_orchestrator.rs

# LLM Integration Scripts (CRITICAL FOR LLM WORKFLOWS)
parseltongue_dungeon/scripts/onboard_codebase.sh
parseltongue_dungeon/scripts/generate_llm_context.sh
parseltongue_dungeon/scripts/debug_entity.sh
parseltongue_dungeon/scripts/feature_impact.sh
parseltongue_dungeon/scripts/self_analysis_and_cleanup.sh

# LLM Instructions & Prompts
parseltongue_dungeon/llm_instructions/codebase_analysis.md
parseltongue_dungeon/llm_instructions/architecture_review.md
parseltongue_dungeon/llm_instructions/refactor_planning.md

# Kiro Integration
.kiro/parseltongue-llm-guide.md
.kiro/parseltongue-code-quality-report.md
```

**Result**: Complete LLM-assisted development workflows

### **DOCUMENTATION TIER 3: UNDERSTANDING & EXTENSION**
Include these for comprehensive understanding:

```bash
# Core Documentation
README.md
PARSELTONGUE_PROJECT_TOC.md
LLM_INTEGRATION_GUIDE.md

# Specifications
.kiro/specs/parseltongue-v2-discovery-first/requirements.md
.kiro/specs/parseltongue-v2-discovery-first/design.md
.kiro/specs/parseltongue-v2-discovery-first/tasks.md

# Architecture Documentation
docs/ARCHITECTURE_OVERVIEW.md
docs/ISG_EXPLAINED.md
docs/ONBOARDING_GUIDE.md

# Implementation Summaries
parseltongue_dungeon/IMPLEMENTATION_SUMMARY.md
parseltongue_dungeon/README.md
```

**Result**: Complete understanding and extension capabilities

## 🔧 STEP-BY-STEP PORTING INSTRUCTIONS

### Step 1: Create New Project Structure
```bash
# Create new Rust project
cargo new your_project_name
cd your_project_name

# Create required directories
mkdir -p src/discovery
mkdir -p parseltongue_dungeon/{scripts,llm_instructions,demo_outputs}
mkdir -p .kiro/specs/parseltongue-v2-discovery-first
mkdir -p docs
mkdir -p tests
```

### Step 2: Copy TIER 1 Files (Minimum Viable)
```bash
# Copy core discovery engine
cp /path/to/parseltongue/src/discovery/engine.rs src/discovery/
cp /path/to/parseltongue/src/discovery/concurrent_discovery_engine.rs src/discovery/
cp /path/to/parseltongue/src/discovery/enhanced_isg_node.rs src/discovery/
cp /path/to/parseltongue/src/discovery/types.rs src/discovery/
cp /path/to/parseltongue/src/discovery/mod.rs src/discovery/

# Copy core system files
cp /path/to/parseltongue/src/isg.rs src/
cp /path/to/parseltongue/src/lib.rs src/
cp /path/to/parseltongue/src/cli.rs src/
cp /path/to/parseltongue/src/main.rs src/

# Copy build configuration
cp /path/to/parseltongue/Cargo.toml .
cp /path/to/parseltongue/Cargo.lock .

# Copy workflow orchestrator
cp /path/to/parseltongue/pt .
chmod +x pt
```

### Step 3: Copy TIER 2 Files (LLM Integration)
```bash
# Copy advanced discovery features
cp /path/to/parseltongue/src/discovery/blast_radius_analyzer.rs src/discovery/
cp /path/to/parseltongue/src/discovery/output_formatter.rs src/discovery/
cp /path/to/parseltongue/src/discovery/json_output.rs src/discovery/
cp /path/to/parseltongue/src/discovery/indexes.rs src/discovery/
cp /path/to/parseltongue/src/discovery/workspace_manager.rs src/discovery/
cp /path/to/parseltongue/src/discovery/concrete_workflow_orchestrator.rs src/discovery/

# Copy LLM integration scripts
cp -r /path/to/parseltongue/parseltongue_dungeon/scripts/* parseltongue_dungeon/scripts/
chmod +x parseltongue_dungeon/scripts/*.sh

# Copy LLM instructions
cp -r /path/to/parseltongue/parseltongue_dungeon/llm_instructions/* parseltongue_dungeon/llm_instructions/

# Copy Kiro integration
cp /path/to/parseltongue/.kiro/parseltongue-llm-guide.md .kiro/
cp /path/to/parseltongue/.kiro/parseltongue-code-quality-report.md .kiro/
```

### Step 4: Copy TIER 3 Files (Documentation)
```bash
# Copy core documentation
cp /path/to/parseltongue/README.md .
cp /path/to/parseltongue/PARSELTONGUE_PROJECT_TOC.md .
cp /path/to/parseltongue/LLM_INTEGRATION_GUIDE.md .

# Copy specifications
cp -r /path/to/parseltongue/.kiro/specs/parseltongue-v2-discovery-first/* .kiro/specs/parseltongue-v2-discovery-first/

# Copy architecture documentation
cp /path/to/parseltongue/docs/ARCHITECTURE_OVERVIEW.md docs/
cp /path/to/parseltongue/docs/ISG_EXPLAINED.md docs/
cp /path/to/parseltongue/docs/ONBOARDING_GUIDE.md docs/

# Copy implementation summaries
cp /path/to/parseltongue/parseltongue_dungeon/IMPLEMENTATION_SUMMARY.md parseltongue_dungeon/
cp /path/to/parseltongue/parseltongue_dungeon/README.md parseltongue_dungeon/
```

### Step 5: Verify Integration
```bash
# Test compilation
cargo build

# Test basic functionality
cargo test

# Test LLM workflows
./pt onboard
./parseltongue_dungeon/scripts/generate_llm_context.sh
```

## 🎯 LLM WORKFLOW COMMANDS

### Essential Commands After Porting:
```bash
# Complete codebase onboarding (generates LLM context)
./pt onboard

# Generate entity-specific context for LLM
./parseltongue_dungeon/scripts/debug_entity.sh <entity_name>

# Analyze feature impact for LLM planning
./parseltongue_dungeon/scripts/feature_impact.sh <feature_description>

# Generate comprehensive LLM context
./parseltongue_dungeon/scripts/generate_llm_context.sh

# Self-analysis and improvement
./parseltongue_dungeon/scripts/self_analysis_and_cleanup.sh
```

## 📋 FILE DEPENDENCIES (CRITICAL RELATIONSHIPS)

### Core Dependencies (Must Port Together):
- `src/isg.rs` ← Required by → `src/discovery/enhanced_isg_node.rs`
- `src/discovery/engine.rs` ← Required by → `src/discovery/concurrent_discovery_engine.rs`
- `src/cli.rs` ← Depends on → `src/discovery/output_formatter.rs`
- `pt` ← Orchestrates → `src/discovery/concrete_workflow_orchestrator.rs`

### LLM Integration Dependencies:
- All `parseltongue_dungeon/scripts/*.sh` ← Require → `pt` (shell orchestrator)
- `parseltongue_dungeon/llm_instructions/*.md` ← Reference → `.kiro/parseltongue-llm-guide.md`
- Workflow scripts ← Depend on → `src/discovery/workspace_manager.rs`

## ⚠️ CRITICAL PORTING NOTES

1. **File Paths**: Update any hardcoded paths in scripts when porting
2. **Dependencies**: Ensure all Rust dependencies in `Cargo.toml` are available in target environment
3. **Permissions**: Make shell scripts executable after copying: `chmod +x parseltongue_dungeon/scripts/*.sh`
4. **Context Generation**: Run `./pt onboard` before LLM analysis to generate proper context
5. **Workspace Management**: Ensure `parseltongue_workspace/` directory is writable for analysis persistence

## 🎯 VALIDATION CHECKLIST

### After Porting, Verify:
- [ ] `cargo build` compiles successfully
- [ ] `cargo test` passes basic tests
- [ ] `./pt --help` shows available commands
- [ ] `./pt onboard` generates analysis workspace
- [ ] `./parseltongue_dungeon/scripts/generate_llm_context.sh` creates context files
- [ ] LLM instructions are accessible in `parseltongue_dungeon/llm_instructions/`

## 📊 FOLDER CLEANUP SUMMARY

### ✅ Completed Cleanup Actions:
1. **Task Reports** → Moved to `docs/task_reports/` (14 files organized)
2. **Analysis Files** → Moved to `analysis/` (5 files consolidated)
3. **Demo Files** → Moved to `demos/` (4 files organized)
4. **Root Directory** → Cleaned to 8 essential files only
5. **Documentation** → Comprehensive TOC and guides created

### Result:
- **Clean project structure** with purpose-driven directories
- **Explicit porting instructions** with exact file paths
- **Complete LLM integration** with ready-to-use workflows
- **Comprehensive documentation** for understanding and extension

---

**This guide provides EXPLICIT, step-by-step instructions for porting Parseltongue to any new codebase with full LLM integration capabilities.**