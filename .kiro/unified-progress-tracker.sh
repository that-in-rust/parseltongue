#!/bin/bash

# Unified Progress Tracker Script
# Consolidates repository snapshots, file tracking, and session context updates
# Replaces multiple overlapping hooks with single intelligent system

set -e

# Get git repository root to ensure all paths are relative to repo
GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$GIT_ROOT"

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
COMMIT_TIMESTAMP=$(date '+%Y%m%d %H%M')
SNAPSHOT_DIR="$GIT_ROOT/.kiro/file-snapshots"
CURRENT_SNAPSHOT="$SNAPSHOT_DIR/current-snapshot.md"
PREVIOUS_SNAPSHOT="$SNAPSHOT_DIR/previous-snapshot.md"
CHANGE_LOG="$SNAPSHOT_DIR/change-log.md"
TEMP_SNAPSHOT="/tmp/unified_snapshot_$$.md"

# Ensure directories exist
mkdir -p "$SNAPSHOT_DIR"

echo "🔄 Unified Progress Tracker - $TIMESTAMP"

# Function to generate comprehensive repository snapshot
generate_repository_snapshot() {
    echo "# Repository Snapshot - $TIMESTAMP" > "$TEMP_SNAPSHOT"
    echo "" >> "$TEMP_SNAPSHOT"

    # Summary statistics - exclude .git but include other hidden files
    TOTAL_FILES=$(find "$GIT_ROOT" -type f ! -path "$GIT_ROOT/.git/*" ! -path "$GIT_ROOT/target/*" ! -path "$GIT_ROOT/node_modules/*" | wc -l)
    TOTAL_LINES=$(find "$GIT_ROOT" -type f ! -path "$GIT_ROOT/.git/*" -name "*.md" -o -name "*.rs" -o -name "*.toml" -o -name "*.json" -o -name "*.txt" -o -name "*.yml" -o -name "*.yaml" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    TOTAL_WORDS=$(find "$GIT_ROOT" -type f ! -path "$GIT_ROOT/.git/*" -name "*.md" -o -name "*.rs" -o -name "*.toml" -o -name "*.json" -o -name "*.txt" -o -name "*.yml" -o -name "*.yaml" | xargs wc -w 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")

    echo "## Summary Statistics" >> "$TEMP_SNAPSHOT"
    echo "- **Total Files**: $(printf "%'d" $TOTAL_FILES)" >> "$TEMP_SNAPSHOT"
    echo "- **Total Lines**: $(printf "%'d" $TOTAL_LINES)" >> "$TEMP_SNAPSHOT"
    echo "- **Total Words**: $(printf "%'d" $TOTAL_WORDS)" >> "$TEMP_SNAPSHOT"
    echo "- **Snapshot Time**: $TIMESTAMP" >> "$TEMP_SNAPSHOT"
    echo "" >> "$TEMP_SNAPSHOT"

    # Spec progress summary
    if [ -d "$GIT_ROOT/.kiro/specs" ]; then
        echo "## Spec Progress Summary" >> "$TEMP_SNAPSHOT"
        echo "" >> "$TEMP_SNAPSHOT"
        echo "| Spec Name | Phase | Progress | Files |" >> "$TEMP_SNAPSHOT"
        echo "|-----------|-------|----------|-------|" >> "$TEMP_SNAPSHOT"
        
        find "$GIT_ROOT/.kiro/specs" -mindepth 1 -maxdepth 1 -type d | sort | while read -r spec_dir; do
            spec_name=$(basename "$spec_dir")
            
            # Detect phase
            if [ ! -f "$spec_dir/requirements.md" ]; then
                phase="Init"
                files="0/3"
            elif [ ! -f "$spec_dir/design.md" ]; then
                phase="Requirements"
                files="1/3"
            elif [ ! -f "$spec_dir/tasks.md" ]; then
                phase="Design"
                files="2/3"
            else
                phase="Tasks"
                files="3/3"
                
                # Check task progress if tasks.md exists
                if [ -f "$spec_dir/tasks.md" ]; then
                    total_tasks=$(grep -c "^- \[" "$spec_dir/tasks.md" 2>/dev/null || echo "0")
                    completed_tasks=$(grep -c "^- \[x\]" "$spec_dir/tasks.md" 2>/dev/null || echo "0")
                    
                    # Ensure variables are numeric
                    total_tasks=${total_tasks:-0}
                    completed_tasks=${completed_tasks:-0}
                    
                    if [ "$total_tasks" -gt 0 ] && [ "$completed_tasks" -ge 0 ]; then
                        progress=$((completed_tasks * 100 / total_tasks))
                        phase="Implementation (${progress}%)"
                    fi
                fi
            fi
            
            echo "| $spec_name | $phase | - | $files |" >> "$TEMP_SNAPSHOT"
        done
        
        echo "" >> "$TEMP_SNAPSHOT"
    fi

    # File inventory with counts
    echo "## File Inventory" >> "$TEMP_SNAPSHOT"
    echo "" >> "$TEMP_SNAPSHOT"
    echo "| File Path | Lines | Words | Size |" >> "$TEMP_SNAPSHOT"
    echo "|-----------|-------|-------|------|" >> "$TEMP_SNAPSHOT"

    find "$GIT_ROOT" -type f ! -path "$GIT_ROOT/.git/*" ! -path "$GIT_ROOT/target/*" ! -path "$GIT_ROOT/node_modules/*" | sort | while read -r file; do
        if [ -f "$file" ]; then
            # Convert absolute path to relative path from git root
            rel_path=$(realpath --relative-to="$GIT_ROOT" "$file")
            if file "$file" | grep -q "text"; then
                lines=$(wc -l < "$file" 2>/dev/null || echo "0")
                words=$(wc -w < "$file" 2>/dev/null || echo "0")
            else
                lines="[binary]"
                words="[binary]"
            fi
            size=$(ls -lh "$file" 2>/dev/null | awk '{print $5}' || echo "?")
            echo "| $rel_path | $lines | $words | $size |" >> "$TEMP_SNAPSHOT"
        fi
    done
}

# Function to update session context across all specs
update_session_context() {
    local updated_count=0
    
    # Find all SESSION_CONTEXT.md files in spec directories
    if [ -d "$GIT_ROOT/.kiro/specs" ]; then
        find "$GIT_ROOT/.kiro/specs" -name "SESSION_CONTEXT.md" -type f | while read -r context_file; do
            spec_dir=$(dirname "$context_file")
            
            # Update timestamp
            sed -i "s/Last Updated: .*/Last Updated: $(date +%Y-%m-%d)/" "$context_file"
            
            # Update task progress if tasks.md exists
            if [ -f "$spec_dir/tasks.md" ]; then
                TOTAL_TASKS=$(grep -c "^- \[" "$spec_dir/tasks.md" 2>/dev/null || echo "0")
                COMPLETED_TASKS=$(grep -c "^- \[x\]" "$spec_dir/tasks.md" 2>/dev/null || echo "0")
                
                # Ensure variables are numeric
                TOTAL_TASKS=${TOTAL_TASKS:-0}
                COMPLETED_TASKS=${COMPLETED_TASKS:-0}
                
                if [ "$TOTAL_TASKS" -gt 0 ] && [ "$COMPLETED_TASKS" -ge 0 ]; then
                    PROGRESS=$((COMPLETED_TASKS * 100 / TOTAL_TASKS))
                    sed -i "s/Progress: [0-9]*%/Progress: ${PROGRESS}%/" "$context_file"
                fi
            fi
            
            # Detect current phase based on existing files
            if [ ! -f "$spec_dir/requirements.md" ]; then
                PHASE="Initialization"
            elif [ ! -f "$spec_dir/design.md" ]; then
                PHASE="Requirements Analysis"
            elif [ ! -f "$spec_dir/tasks.md" ]; then
                PHASE="Design Development"
            else
                # Check if any tasks are incomplete
                if grep -q "^- \[ \]" "$spec_dir/tasks.md" 2>/dev/null; then
                    PHASE="Implementation"
                else
                    PHASE="Complete"
                fi
            fi
            
            sed -i "s/Current Phase: .*/Current Phase: $PHASE/" "$context_file"
            updated_count=$((updated_count + 1))
        done
        
        if [ "$updated_count" -gt 0 ]; then
            echo "✅ Updated $updated_count session context files"
        fi
    fi
}

# Function to detect change type for commit messages
detect_change_type() {
    local change_type="general"
    
    # Check what types of files were changed in .kiro/
    if git diff --cached --name-only .kiro/ | grep -q "requirements\.md"; then
        change_type="requirements"
    elif git diff --cached --name-only .kiro/ | grep -q "design\.md"; then
        change_type="design"
    elif git diff --cached --name-only .kiro/ | grep -q "tasks\.md"; then
        change_type="tasks"
    elif git diff --cached --name-only .kiro/ | grep -q "SESSION_CONTEXT\.md"; then
        change_type="session"
    elif git diff --cached --name-only .kiro/ | grep -q "file-snapshots/"; then
        change_type="snapshots"
    elif git diff --cached --name-only .kiro/ | grep -q "hooks/"; then
        change_type="hooks"
    elif git diff --cached --name-only .kiro/ | grep -q "steering/"; then
        change_type="steering"
    fi
    
    echo "$change_type"
}

# Function to generate delta report
generate_delta_report() {
    if [ ! -f "$PREVIOUS_SNAPSHOT" ]; then
        echo "📝 Initial snapshot created" >> "$CHANGE_LOG"
        return
    fi

    echo "" >> "$CHANGE_LOG"
    echo "## Delta Report - $TIMESTAMP" >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"

    # Extract previous stats
    PREV_FILES=$(grep "Total Files" "$PREVIOUS_SNAPSHOT" | sed 's/.*: //' | tr -d ',' | tr -d '*' || echo "0")
    PREV_LINES=$(grep "Total Lines" "$PREVIOUS_SNAPSHOT" | sed 's/.*: //' | tr -d ',' | tr -d '*' || echo "0")
    PREV_WORDS=$(grep "Total Words" "$PREVIOUS_SNAPSHOT" | sed 's/.*: //' | tr -d ',' | tr -d '*' || echo "0")

    # Calculate changes (ensure variables are numeric)
    PREV_FILES=${PREV_FILES:-0}
    PREV_LINES=${PREV_LINES:-0}
    PREV_WORDS=${PREV_WORDS:-0}
    TOTAL_FILES=${TOTAL_FILES:-0}
    TOTAL_LINES=${TOTAL_LINES:-0}
    TOTAL_WORDS=${TOTAL_WORDS:-0}
    
    FILE_DIFF=$((TOTAL_FILES - PREV_FILES))
    LINE_DIFF=$((TOTAL_LINES - PREV_LINES))
    WORD_DIFF=$((TOTAL_WORDS - PREV_WORDS))

    echo "### Summary Changes" >> "$CHANGE_LOG"
    echo "- **File Count**: $FILE_DIFF ($(printf "%'d" $TOTAL_FILES) total)" >> "$CHANGE_LOG"
    echo "- **Line Count**: $(printf "%'d" $LINE_DIFF) ($(printf "%'d" $TOTAL_LINES) total)" >> "$CHANGE_LOG"
    echo "- **Word Count**: $(printf "%'d" $WORD_DIFF) ($(printf "%'d" $TOTAL_WORDS) total)" >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"

    # Detect specific file changes
    if [ -f "$PREVIOUS_SNAPSHOT" ] && [ -f "$TEMP_SNAPSHOT" ]; then
        echo "### File-Level Changes" >> "$CHANGE_LOG"
        
        # Extract file lists for comparison
        grep "^| \." "$PREVIOUS_SNAPSHOT" | awk -F'|' '{print $2}' | sed 's/^ *//;s/ *$//' | sort > /tmp/prev_files.txt 2>/dev/null || touch /tmp/prev_files.txt
        grep "^| \." "$TEMP_SNAPSHOT" | awk -F'|' '{print $2}' | sed 's/^ *//;s/ *$//' | sort > /tmp/curr_files.txt 2>/dev/null || touch /tmp/curr_files.txt
        
        # Find added files
        ADDED=$(comm -13 /tmp/prev_files.txt /tmp/curr_files.txt 2>/dev/null || echo "")
        if [ -n "$ADDED" ] && [ "$ADDED" != "" ]; then
            echo "**Added Files:**" >> "$CHANGE_LOG"
            echo "$ADDED" | head -10 | while read -r file; do
                echo "- $file" >> "$CHANGE_LOG"
            done
            echo "" >> "$CHANGE_LOG"
        fi
        
        # Find removed files
        REMOVED=$(comm -23 /tmp/prev_files.txt /tmp/curr_files.txt 2>/dev/null || echo "")
        if [ -n "$REMOVED" ] && [ "$REMOVED" != "" ]; then
            echo "**Removed Files:**" >> "$CHANGE_LOG"
            echo "$REMOVED" | head -10 | while read -r file; do
                echo "- $file" >> "$CHANGE_LOG"
            done
            echo "" >> "$CHANGE_LOG"
        fi
        
        # Cleanup temp files
        rm -f /tmp/prev_files.txt /tmp/curr_files.txt
    fi
    
    echo "---" >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
}

# Main execution
echo "📊 Generating repository snapshot..."

# Move current to previous if it exists
if [ -f "$CURRENT_SNAPSHOT" ]; then
    cp "$CURRENT_SNAPSHOT" "$PREVIOUS_SNAPSHOT"
fi

# Generate new snapshot
generate_repository_snapshot

# Update session context
update_session_context

# Initialize change log if needed
if [ ! -f "$CHANGE_LOG" ]; then
    echo "# Repository Change Log" > "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
    echo "Unified tracking of all repository changes with comprehensive delta reporting." >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
fi

# Generate delta report
generate_delta_report

# Move temp snapshot to current
mv "$TEMP_SNAPSHOT" "$CURRENT_SNAPSHOT"

# Git operations (only .kiro directory)
# Check for unstaged changes, staged changes, and untracked files in .kiro/
UNTRACKED_KIRO=$(git ls-files --others --exclude-standard .kiro/ | wc -l)
if git diff --quiet .kiro/ && git diff --cached --quiet .kiro/ && [ "$UNTRACKED_KIRO" -eq 0 ]; then
    echo "ℹ️  No .kiro changes to commit"
else
    git add .kiro/
    
    if ! git diff --cached --quiet .kiro/; then
        CHANGE_TYPE=$(detect_change_type)
        COMMIT_MSG="unified-progress [$CHANGE_TYPE] $COMMIT_TIMESTAMP"
        git commit -m "$COMMIT_MSG"
        
        CURRENT_BRANCH=$(git branch --show-current)
        if git push origin "$CURRENT_BRANCH" 2>/dev/null; then
            echo "✅ Changes committed and pushed to $CURRENT_BRANCH: $COMMIT_MSG"
        else
            echo "⚠️  Changes committed locally (push failed): $COMMIT_MSG"
        fi
    fi
fi

echo "✅ Unified progress tracking complete"
echo "📊 Files: $(printf "%'d" $TOTAL_FILES) | Lines: $(printf "%'d" $TOTAL_LINES) | Words: $(printf "%'d" $TOTAL_WORDS)"

echo "This finally works"