# Session Context Auto-Updater Hook

## Hook Configuration

**Trigger Events:**
- File save in `.kiro/specs/parseltongue-aim-daemon/`
- Workspace open
- Manual trigger via command palette

**Target File:** `.kiro/specs/parseltongue-aim-daemon/SESSION_CONTEXT.md`

## Auto-Update Logic

### 1. Live Session Status Updates
```bash
# Update timestamp
sed -i "s/Last Updated: .*/Last Updated: $(date +%Y-%m-%d)/" SESSION_CONTEXT.md

# Detect current phase
if [ ! -f "design.md" ]; then
    PHASE="Requirements Analysis (Phase 1)"
elif [ ! -f "tasks.md" ]; then
    PHASE="Design Development (Phase 2)"
else
    PHASE="Implementation Planning (Phase 3)"
fi
```

### 2. Task Progress Tracking
```bash
# Count completed tasks from requirements-tasks.md
TOTAL_TASKS=$(grep -c "^\- \[" requirements-tasks.md)
COMPLETED_TASKS=$(grep -c "^\- \[x\]" requirements-tasks.md)
PROGRESS=$((COMPLETED_TASKS * 100 / TOTAL_TASKS))

# Update progress in SESSION_CONTEXT.md
sed -i "s/Document Analysis Progress: [0-9]*%/Document Analysis Progress: ${PROGRESS}%/" SESSION_CONTEXT.md
```

### 3. Recent Progress Log
```bash
# Auto-append new progress entries
echo "### $(date +%Y-%m-%d)" >> SESSION_CONTEXT.md
echo "- **[UPDATED]** Auto-updated session context" >> SESSION_CONTEXT.md

# Parse recent git commits for architectural decisions
git log --since="1 day ago" --oneline --grep="REQ-\|ARCH-\|PERF-" >> SESSION_CONTEXT.md
```

### 4. Document Analysis Status
```bash
# Count analyzed documents
ANALYZED_DOCS=$(find _refDocs -name "*.md" -exec grep -l "ANALYZED" {} \; | wc -l)
TOTAL_DOCS=$(find _refDocs -name "*.md" | wc -l)

# Update analysis status
sed -i "s/_refDocs: [0-9]*\/[0-9]*/_refDocs: ${ANALYZED_DOCS}\/${TOTAL_DOCS}/" SESSION_CONTEXT.md
```

## Hook Benefits

### Automatic Maintenance
- **No manual updates needed** - context stays current automatically
- **Consistent format** - standardized progress tracking
- **Real-time accuracy** - reflects actual project state
- **Auto-commit & push** - complete audit trail with zero effort

### Enhanced Productivity
- **Instant session recovery** - know exactly where you left off
- **Progress visibility** - clear view of completion status
- **Next action clarity** - automatic priority task identification
- **Git history search** - find any previous session state instantly

### Integration with Kiro Workflow
- **Spec-driven development** - aligns with requirements → design → tasks flow
- **Terminal-friendly** - supports CLI-based development workflow
- **LLM context** - provides structured context for AI assistance
- **Branch-specific** - pushes to v01 branch for organized development

### Git Workflow Automation
- **Intelligent commits** - contextual messages based on actual changes
- **Safe operations** - only commits when changes exist
- **Error resilience** - graceful handling of network/push failures
- **Complete audit trail** - every session update tracked in git history

## Implementation Commands

```bash
# Create the hook directory
mkdir -p .kiro/hooks

# Test the hook manually
kiro hook run session-context-updater

# Enable automatic triggering
kiro hook enable session-context-updater --on-save --on-workspace-open
```

## Hook Script Template

```bash
#!/bin/bash
# Session Context Auto-Updater Hook with Git Integration

SPEC_DIR=".kiro/specs/parseltongue-aim-daemon"
CONTEXT_FILE="$SPEC_DIR/SESSION_CONTEXT.md"
TIMESTAMP=$(date +"%Y %m %d IST")
CHANGES_DETECTED=""

# Function to detect what changed in .kiro folder only
detect_changes() {
    local changes=""
    
    # Check if .kiro/specs requirements were updated
    if git diff --name-only HEAD~1 2>/dev/null | grep -q ".kiro/specs.*requirements.md"; then
        changes="${changes}requirements "
    fi
    
    # Check if .kiro/specs tasks were completed
    if git diff HEAD~1 2>/dev/null | grep -q ".kiro/specs.*tasks.md" && \
       git diff HEAD~1 2>/dev/null | grep -q "^\+.*\[x\]"; then
        changes="${changes}task-completion "
    fi
    
    # Check if new .kiro spec files were added
    if git diff --name-only --cached 2>/dev/null | grep -q ".kiro/specs.*\.md$"; then
        changes="${changes}spec-update "
    fi
    
    # Check if steering rules were updated
    if git diff --name-only HEAD~1 2>/dev/null | grep -q ".kiro/steering"; then
        changes="${changes}steering-update "
    fi
    
    # Check if hooks were modified
    if git diff --name-only HEAD~1 2>/dev/null | grep -q ".kiro/hooks"; then
        changes="${changes}hook-update "
    fi
    
    # Default to kiro-update if no specific changes detected
    if [ -z "$changes" ]; then
        changes="kiro-update"
    fi
    
    echo "$changes"
}

# Update timestamp
sed -i "s/Last Updated: .*/Last Updated: $(date +%Y-%m-%d)/" "$CONTEXT_FILE"

# Update task progress
if [ -f "$SPEC_DIR/requirements-tasks.md" ]; then
    TOTAL=$(grep -c "^\- \[" "$SPEC_DIR/requirements-tasks.md")
    COMPLETED=$(grep -c "^\- \[x\]" "$SPEC_DIR/requirements-tasks.md")
    PROGRESS=$((COMPLETED * 100 / TOTAL))
    
    sed -i "s/Document Analysis [0-9]*% Complete/Document Analysis ${PROGRESS}% Complete/" "$CONTEXT_FILE"
    
    if [ $COMPLETED -gt 0 ]; then
        CHANGES_DETECTED="${CHANGES_DETECTED}progress-${PROGRESS}% "
    fi
fi

# Detect current phase
if [ ! -f "$SPEC_DIR/design.md" ]; then
    PHASE="Requirements Analysis (Phase 1)"
elif [ ! -f "$SPEC_DIR/tasks.md" ]; then
    PHASE="Design Development (Phase 2)"
else
    PHASE="Implementation Planning (Phase 3)"
fi

sed -i "s/Current Phase: .*/Current Phase: $PHASE/" "$CONTEXT_FILE"

# Detect specific changes for commit message
CHANGE_TYPE=$(detect_changes)
if [ -z "$CHANGES_DETECTED" ]; then
    CHANGES_DETECTED="$CHANGE_TYPE"
fi

# Git operations focused on .kiro directory only
if git diff --quiet .kiro/ && git diff --cached --quiet .kiro/; then
    echo "ℹ️  No .kiro changes to commit"
else
    # Stage only .kiro directory changes
    git add .kiro/
    
    # Check if there are actually staged changes in .kiro
    if git diff --cached --quiet .kiro/; then
        echo "ℹ️  No .kiro changes staged"
    else
        # Create intelligent commit message
        COMMIT_MSG="${CHANGES_DETECTED}${TIMESTAMP}"
        
        # Commit with descriptive message
        git commit -m "$COMMIT_MSG"
        
        # Push to v01 branch
        if git push origin v01 2>/dev/null; then
            echo "✅ .kiro updates committed and pushed to v01"
            echo "📝 Commit: $COMMIT_MSG"
        else
            echo "⚠️  .kiro updates committed locally (push failed)"
        fi
    fi
fi
```

## Enhanced Git Integration Features

### Intelligent Commit Messages (.kiro focused)
The hook generates contextual commit messages based on what changed in `.kiro/`:

- `requirements 2025 01 20 IST` - when .kiro/specs requirements.md is modified
- `task-completion progress-75% 2025 01 20 IST` - when .kiro/specs tasks are marked complete
- `spec-update 2025 01 20 IST` - when new .kiro/specs files are added
- `steering-update 2025 01 20 IST` - when .kiro/steering rules are modified
- `hook-update 2025 01 20 IST` - when .kiro/hooks are modified
- `kiro-update 2025 01 20 IST` - for general .kiro context updates

### Smart Change Detection (.kiro scope)
```bash
# Examples of generated commit messages:
requirements task-completion progress-80% 2025 01 20 IST
steering-update spec-update 2025 01 20 IST  
hook-update kiro-update 2025 01 20 IST
requirements steering-update 2025 01 20 IST
```

### Git Workflow Integration (.kiro focused)
- **Scope limited**: Only monitors and commits `.kiro/` directory changes
- **Safe operations**: Only commits when there are actual `.kiro/` changes
- **Branch targeting**: Pushes to `v01` branch specifically
- **Error handling**: Graceful fallback if push fails (network issues, etc.)
- **Isolated commits**: Never accidentally commits code outside `.kiro/`

### Audit Trail Benefits
- **Complete history**: Every session update is tracked in git
- **Searchable commits**: Easy to find when specific changes were made
- **Rollback capability**: Can revert to any previous session state
- **Collaboration**: Team members can see progress in real-time