# Hook Automation Guide - Steering Rules

## Purpose

This document provides comprehensive guidance on the automated hook system that supports the requirements-tasks methodology and ensures continuous progress tracking, session continuity, and development workflow automation.

## Hook Architecture Overview

The hook system implements the session continuity requirements from the requirements-tasks methodology, providing automated tracking of development progress, repository snapshots, and intelligent git operations.

### Core Principles

1. **Comprehensive Monitoring**: Track all file changes to maintain complete development context
2. **Intelligent Commits**: Only commit `.kiro/` directory changes with contextual messages
3. **Session Continuity**: Automatically update SESSION_CONTEXT.md with current progress
4. **Progress Tracking**: Calculate and update task completion percentages
5. **Repository Snapshots**: Generate delta reports showing development evolution

## Active Hooks

### 1. Unified Progress Tracker (Automatic)

**File**: `.kiro/hooks/unified-progress-tracker.kiro.hook`

**Purpose**: Comprehensive automation that handles repository snapshots, file change tracking, session context updates, and git operations.

**Trigger**: `fileSaved` on `**/*` excluding `.git/**/*` (any file save in repository except git folder)

**Actions**:
- Generates comprehensive repository snapshots with complete file inventory and spec progress summary
- Tracks ALL files with absolute path resolution from git repository root
- Counts lines/words across expanded file types (.md, .rs, .toml, .json, .txt, .yml, .yaml)
- Updates all SESSION_CONTEXT.md files across multiple specs with phase detection
- Calculates task completion from tasks.md files in each spec directory
- Creates structured git commit messages with categorized change types
- Commits only `.kiro/` directory changes to current branch with untracked file detection

**Script**: `.kiro/unified-progress-tracker.sh`

**Benefits**:
- Zero-effort progress tracking
- Complete audit trail of development sessions
- Automatic session recovery context
- Intelligent change categorization

### 2. Unified Progress Tracker (Manual)

**File**: `.kiro/hooks/unified-progress-manual.kiro.hook`

**Purpose**: Manual trigger for the same comprehensive progress tracking functionality.

**Trigger**: `manual` (user-initiated via Agent Hooks panel)

**Use Cases**:
- Force progress update without file changes
- Generate snapshots before major milestones
- Manual session context refresh
- Troubleshooting or testing hook functionality

### 3. Source to Docs Sync

**File**: `.kiro/hooks/source-docs-sync.kiro.hook`

**Purpose**: Monitors all source files and configuration changes to trigger documentation updates.

**Trigger**: `fileSaved` on source files (`.rs`, `.py`, `.ts`, `.js`, etc.) with SHORT debounce

**Actions**:
- Analyzes source code changes across multiple languages
- Updates README.md and docs/ folder automatically
- Maintains synchronization between code and documentation
- Focuses on API changes, new features, and architectural modifications
- Provides comprehensive coverage for polyglot repositories

## Hook Integration with Steering Methodology

### Requirements-Tasks Methodology Support

The hooks directly implement the session continuity requirements:

#### Progress Tracking Standards (Automated)
- **Status Indicators**: Automatically updated in SESSION_CONTEXT.md
- **Completion Tracking**: Real-time calculation of task percentages
- **Quality Assurance**: Verification of requirements quality standards
- **Success Metrics Dashboard**: Automated progress reporting

#### Session Continuity Requirements (Automated)
- **Context Persistence**: SESSION_CONTEXT.md automatically maintained
- **Git History**: Complete development timeline with intelligent commits
- **Recovery Protocol**: Instant session state recovery from context files
- **Architecture Reference**: Automatic updates to architecture-backlog.md

### MVP Discipline Framework Integration

The hooks enforce MVP constraints through intelligent filtering:

#### What Gets Tracked
- ✅ **Rust-focused changes**: Enhanced tracking for .rs files
- ✅ **Performance monitoring**: File count and complexity tracking
- ✅ **LLM-terminal integration**: Structured context generation
- ✅ **Core development**: Requirements, design, tasks progression

#### What Gets Filtered
- ❌ **Non-MVP features**: Advanced concepts moved to backlogs
- ❌ **Noise reduction**: Only meaningful changes trigger commits
- ❌ **Scope creep**: Focus maintained on core constraints
- ❌ **Unnecessary complexity**: Simple, deterministic operations

## Hook Configuration Standards

### File Naming Convention
- **Pattern**: `{hook-name}.kiro.hook`
- **Location**: `.kiro/hooks/`
- **Examples**: `unified-progress-tracker.kiro.hook`, `source-docs-sync.kiro.hook`

### Pattern Configuration
- **Include Patterns**: Use `patterns` array to specify which files to monitor
- **Exclude Patterns**: Use `excludePatterns` array to exclude specific paths (e.g., `.git/**/*`)
- **Common Exclusions**: `.git/**/*`, `target/**/*`, `node_modules/**/*`, `build/**/*`

### JSON Structure Requirements
```json
{
  "enabled": true,
  "name": "Hook Display Name",
  "description": "Clear description of hook purpose and functionality",
  "version": "1",
  "when": {
    "type": "fileSaved|manual|userTriggered",
    "patterns": ["**/*"],
    "excludePatterns": [".git/**/*"]
  },
  "then": {
    "type": "shell|askAgent",
    "command": "script-path.sh"
  }
}
```

### Trigger Types
- **`fileSaved`**: Automatic trigger on file save events
- **`manual`**: User-initiated via Agent Hooks panel
- **`userTriggered`**: User-initiated with pattern matching

### Action Types
- **`shell`**: Direct script execution (preferred for automation)
- **`askAgent`**: AI-driven actions (preferred for analysis tasks)

## Script Architecture

### Unified Progress Tracker Script

**Location**: `.kiro/unified-progress-tracker.sh`

**Core Functions**:
1. `generate_repository_snapshot()` - Creates comprehensive file inventory with spec progress tracking
2. `update_session_context()` - Updates all SESSION_CONTEXT.md files across multiple specs
3. `generate_delta_report()` - Tracks changes between snapshots with file-level analysis
4. `detect_change_type()` - Categorizes changes for structured commit messages

**Git Integration**:
- **Scope**: Only commits `.kiro/` directory changes
- **Branch**: Pushes to current branch (auto-detected with `git branch --show-current`)
- **Messages**: Intelligent categorization with structured format `unified-progress [category] timestamp`
- **Safety**: Verifies changes exist (staged, unstaged, and untracked) before committing
- **Repository Root**: Uses `git rev-parse --show-toplevel` for absolute path resolution

**Performance Characteristics**:
- **Execution Time**: ~2-5 seconds for typical repositories
- **Memory Usage**: Minimal, processes files incrementally
- **Disk Impact**: Only writes to `.kiro/` directory
- **Network**: Only pushes when changes exist

**File Tracking Scope**:
- **File Count**: ALL files in repository (including .git) except target/ and node_modules/
- **Line/Word Count**: Text files (.md, .rs, .toml, .json, .txt, .yml, .yaml)
- **Inventory**: All non-binary files with detailed metrics
- **Delta Tracking**: Comprehensive change detection between snapshots

## Troubleshooting Guide

### Hook Not Appearing in Agent Hooks Panel

1. **Verify File Location**: Must be in `.kiro/hooks/` directory
2. **Check File Extension**: Must end with `.kiro.hook`
3. **Validate JSON**: Use `python3 -m json.tool filename.kiro.hook`
4. **Refresh IDE**: Cmd+R (Mac) or Ctrl+R (Windows/Linux)
5. **Check Script Permissions**: `chmod +x script-name.sh`

### Hook Appearing But Not Executing

1. **Script Existence**: Verify script file exists at specified path
2. **Execute Permissions**: Ensure script is executable
3. **Manual Test**: Run script directly to check for errors
4. **Check Logs**: Use Kiro command palette "View Logs"
5. **Path Issues**: Ensure script path is relative to workspace root

### Git Operations Failing

1. **Branch Detection**: Script auto-detects current branch with `git branch --show-current`
2. **Remote Access**: Verify git push permissions for current branch
3. **Repository Root**: Ensure script runs from git repository root (auto-detected)
4. **Untracked Files**: Script now detects and includes untracked files in `.kiro/`
5. **Network Issues**: Test git connectivity manually

## Best Practices

### Hook Development
- **Single Responsibility**: Each hook should have one clear purpose
- **Error Handling**: Scripts should handle failures gracefully
- **Logging**: Include informative output for debugging
- **Testing**: Test hooks manually before enabling automation

### Performance Optimization
- **Debouncing**: Use appropriate delays for file-based triggers
- **Filtering**: Only process relevant file types
- **Incremental**: Avoid full repository scans when possible
- **Caching**: Store intermediate results to avoid recomputation

### Security Considerations
- **Script Validation**: Only execute trusted scripts
- **Path Restrictions**: Keep scripts within `.kiro/` directory
- **Permission Limits**: Use minimal required permissions
- **Input Sanitization**: Validate all external inputs

## Integration with Development Workflow

### Daily Development Cycle
1. **File Save** → Unified Progress Tracker runs automatically from git repository root
2. **Progress Updated** → All SESSION_CONTEXT.md files across specs reflect current state
3. **Changes Committed** → Only `.kiro/` changes pushed to current branch with structured messages
4. **Context Preserved** → Complete session state maintained with spec progress tracking

### Session Recovery
1. **Open Project** → Read SESSION_CONTEXT.md for current state
2. **Check Progress** → Review task completion percentages
3. **Continue Work** → Pick up from documented next actions
4. **Automatic Tracking** → All progress automatically captured

### Team Collaboration
1. **Shared Context** → SESSION_CONTEXT.md provides team visibility
2. **Git History** → Complete development timeline available
3. **Progress Transparency** → Real-time completion tracking
4. **Consistent Workflow** → Standardized automation across team

## Maintenance and Updates

### Regular Maintenance Tasks
- **Script Updates**: Keep automation scripts current with methodology changes
- **Hook Validation**: Periodically verify all hooks are functioning
- **Performance Review**: Monitor execution times and optimize as needed
- **Documentation Sync**: Keep this guide updated with changes

### Version Management
- **Hook Versioning**: Increment version numbers for significant changes
- **Script Backup**: Maintain previous versions in zzzArchive
- **Migration Guide**: Document changes when updating hook configurations
- **Compatibility**: Ensure hooks work across Kiro IDE versions
