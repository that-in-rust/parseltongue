# File Change Tracker Hook

## Hook Configuration

**Trigger Events:**
- File save anywhere in the repository
- Manual trigger via command palette
- Workspace open

**Target Files:** 
- `.kiro/file-snapshots/current-snapshot.md` (current state)
- `.kiro/file-snapshots/previous-snapshot.md` (previous state)
- `.kiro/file-snapshots/change-log.md` (historical log)

## Hook Purpose

Track all file changes in the repository with word counts and generate delta reports showing:
- What files were added/removed/modified
- Word count changes for each file
- Timestamp of changes
- Maintain historical log of all changes

## Hook Script

```bash
#!/bin/bash
# File Change Tracker Hook

SNAPSHOTS_DIR=".kiro/file-snapshots"
CURRENT_SNAPSHOT="$SNAPSHOTS_DIR/current-snapshot.md"
PREVIOUS_SNAPSHOT="$SNAPSHOTS_DIR/previous-snapshot.md"
CHANGE_LOG="$SNAPSHOTS_DIR/change-log.md"
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S IST")

# Create snapshots directory if it doesn't exist
mkdir -p "$SNAPSHOTS_DIR"

# Function to generate current repository state
generate_current_state() {
    echo "# Repository Snapshot - $TIMESTAMP" > "$CURRENT_SNAPSHOT.tmp"
    echo "" >> "$CURRENT_SNAPSHOT.tmp"
    echo "## File Tree with Word Counts" >> "$CURRENT_SNAPSHOT.tmp"
    echo "" >> "$CURRENT_SNAPSHOT.tmp"
    
    # Generate tree with word counts for all files
    find . -type f \
        -not -path "./.git/*" \
        -not -path "./target/*" \
        -not -path "./node_modules/*" \
        -not -path "./.kiro/file-snapshots/*" \
        | sort | while read -r file; do
        
        # Get word count (handle binary files gracefully)
        if file "$file" | grep -q "text"; then
            WC=$(wc -w "$file" 2>/dev/null | awk '{print $1}')
        else
            WC="[binary]"
        fi
        
        # Get file size
        SIZE=$(ls -lh "$file" | awk '{print $5}')
        
        echo "- \`$file\` | $WC words | $SIZE" >> "$CURRENT_SNAPSHOT.tmp"
    done
    
    echo "" >> "$CURRENT_SNAPSHOT.tmp"
    echo "## Summary" >> "$CURRENT_SNAPSHOT.tmp"
    echo "" >> "$CURRENT_SNAPSHOT.tmp"
    
    # Count totals
    TOTAL_FILES=$(find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "./node_modules/*" -not -path "./.kiro/file-snapshots/*" | wc -l)
    TOTAL_WORDS=$(find . -name "*.md" -o -name "*.rs" -o -name "*.txt" | xargs wc -w 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    
    echo "- **Total Files**: $TOTAL_FILES" >> "$CURRENT_SNAPSHOT.tmp"
    echo "- **Total Words** (text files): $TOTAL_WORDS" >> "$CURRENT_SNAPSHOT.tmp"
    echo "- **Snapshot Time**: $TIMESTAMP" >> "$CURRENT_SNAPSHOT.tmp"
    
    mv "$CURRENT_SNAPSHOT.tmp" "$CURRENT_SNAPSHOT"
}

# Function to calculate and log changes
calculate_changes() {
    if [ ! -f "$PREVIOUS_SNAPSHOT" ]; then
        echo "📝 First snapshot created at $TIMESTAMP" >> "$CHANGE_LOG"
        return
    fi
    
    echo "" >> "$CHANGE_LOG"
    echo "## Change Delta - $TIMESTAMP" >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
    
    # Extract file lists from both snapshots
    grep "^- \`" "$PREVIOUS_SNAPSHOT" | sed 's/^- `\(.*\)` |.*/\1/' | sort > /tmp/prev_files.txt
    grep "^- \`" "$CURRENT_SNAPSHOT" | sed 's/^- `\(.*\)` |.*/\1/' | sort > /tmp/curr_files.txt
    
    # Find added files
    ADDED=$(comm -13 /tmp/prev_files.txt /tmp/curr_files.txt)
    if [ -n "$ADDED" ]; then
        echo "### ➕ Added Files" >> "$CHANGE_LOG"
        echo "$ADDED" | while read -r file; do
            WC=$(grep "^- \`$file\`" "$CURRENT_SNAPSHOT" | sed 's/.*| \([0-9]*\) words.*/\1/')
            echo "- \`$file\` ($WC words)" >> "$CHANGE_LOG"
        done
        echo "" >> "$CHANGE_LOG"
    fi
    
    # Find removed files
    REMOVED=$(comm -23 /tmp/prev_files.txt /tmp/curr_files.txt)
    if [ -n "$REMOVED" ]; then
        echo "### ➖ Removed Files" >> "$CHANGE_LOG"
        echo "$REMOVED" | while read -r file; do
            echo "- \`$file\`" >> "$CHANGE_LOG"
        done
        echo "" >> "$CHANGE_LOG"
    fi
    
    # Find modified files (word count changes)
    echo "### 📝 Modified Files" >> "$CHANGE_LOG"
    MODIFIED_COUNT=0
    
    comm -12 /tmp/prev_files.txt /tmp/curr_files.txt | while read -r file; do
        PREV_WC=$(grep "^- \`$file\`" "$PREVIOUS_SNAPSHOT" | sed 's/.*| \([0-9]*\) words.*/\1/' 2>/dev/null || echo "0")
        CURR_WC=$(grep "^- \`$file\`" "$CURRENT_SNAPSHOT" | sed 's/.*| \([0-9]*\) words.*/\1/' 2>/dev/null || echo "0")
        
        if [ "$PREV_WC" != "$CURR_WC" ] && [ "$PREV_WC" != "[binary]" ] && [ "$CURR_WC" != "[binary]" ]; then
            DELTA=$((CURR_WC - PREV_WC))
            if [ $DELTA -gt 0 ]; then
                echo "- \`$file\` (+$DELTA words: $PREV_WC → $CURR_WC)" >> "$CHANGE_LOG"
            else
                echo "- \`$file\` ($DELTA words: $PREV_WC → $CURR_WC)" >> "$CHANGE_LOG"
            fi
            MODIFIED_COUNT=$((MODIFIED_COUNT + 1))
        fi
    done
    
    if [ $MODIFIED_COUNT -eq 0 ]; then
        echo "- No word count changes detected" >> "$CHANGE_LOG"
    fi
    
    echo "" >> "$CHANGE_LOG"
    
    # Summary
    ADDED_COUNT=$(echo "$ADDED" | grep -c . || echo "0")
    REMOVED_COUNT=$(echo "$REMOVED" | grep -c . || echo "0")
    
    echo "**Summary**: $ADDED_COUNT added, $REMOVED_COUNT removed, $MODIFIED_COUNT modified" >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
    
    # Cleanup temp files
    rm -f /tmp/prev_files.txt /tmp/curr_files.txt
}

# Main execution
echo "🔍 Generating repository snapshot..."

# Move current to previous (if exists)
if [ -f "$CURRENT_SNAPSHOT" ]; then
    mv "$CURRENT_SNAPSHOT" "$PREVIOUS_SNAPSHOT"
fi

# Generate new current state
generate_current_state

# Calculate and log changes
calculate_changes

# Initialize change log if it doesn't exist
if [ ! -f "$CHANGE_LOG" ]; then
    echo "# Repository Change Log" > "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
    echo "This file tracks all changes in the repository with word count deltas." >> "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
fi

# Git operations (only commit .kiro changes)
if git diff --quiet .kiro/ && git diff --cached --quiet .kiro/; then
    echo "ℹ️  No .kiro changes to commit"
else
    git add .kiro/
    if ! git diff --cached --quiet .kiro/; then
        git commit -m "file-tracker-update $TIMESTAMP"
        if git push origin v01 2>/dev/null; then
            echo "✅ File change tracking updated and pushed to v01"
        else
            echo "⚠️  File change tracking updated locally (push failed)"
        fi
    fi
fi

echo "✅ Repository snapshot complete"
echo "📊 Current: $(grep "Total Files" "$CURRENT_SNAPSHOT" | sed 's/.*: //')"
echo "📝 Changes logged to: $CHANGE_LOG"
```

## File Structure Created

```
.kiro/file-snapshots/
├── current-snapshot.md      # Latest repository state
├── previous-snapshot.md     # Previous repository state  
├── change-log.md           # Historical change log
```

## Sample Output

### current-snapshot.md
```markdown
# Repository Snapshot - 2025-01-20 15:30:45 IST

## File Tree with Word Counts

- `./README.md` | 245 words | 2.1K
- `./_refDocs/Notes04.md` | 5498 words | 45K
- `./_refDocs/zz04MoreNotes.md` | 762 words | 40K
- `./src/main.rs` | 156 words | 1.2K

## Summary

- **Total Files**: 87
- **Total Words** (text files): 15,432
- **Snapshot Time**: 2025-01-20 15:30:45 IST
```

### change-log.md
```markdown
# Repository Change Log

## Change Delta - 2025-01-20 15:30:45 IST

### ➕ Added Files
- `./new-file.md` (123 words)

### 📝 Modified Files
- `./_refDocs/zz04MoreNotes.md` (+200 words: 562 → 762)
- `./requirements-tasks.md` (-15 words: 1200 → 1185)

**Summary**: 1 added, 0 removed, 2 modified
```

## Hook Benefits

### Change Tracking
- **Complete visibility** into repository evolution
- **Word count deltas** show content growth/reduction
- **File additions/removals** tracked automatically
- **Historical log** maintains complete change history

### Analysis Progress Monitoring
- **Perfect for document analysis** - see word count changes as files are processed
- **Track _refDocs and _refIdioms progress** automatically
- **Identify large files** that need attention
- **Monitor specification document growth**

### Integration with Development Workflow
- **Automatic snapshots** on file saves
- **Git integration** - commits tracking data to v01 branch
- **Terminal-friendly** output for quick status checks
- **Zero manual effort** - runs automatically

## Usage Commands

```bash
# Manual trigger
kiro hook run file-change-tracker

# View current snapshot
cat .kiro/file-snapshots/current-snapshot.md

# View recent changes
tail -50 .kiro/file-snapshots/change-log.md

# Compare snapshots
diff .kiro/file-snapshots/previous-snapshot.md .kiro/file-snapshots/current-snapshot.md
```

This hook will be invaluable for tracking the analysis progress and understanding exactly what's changing in the repository as we work through the requirements tasks!