#!/bin/bash

# Repository Snapshot Generator Script
# Automatically generates comprehensive repository snapshots with delta tracking

set -e

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S IST')
SNAPSHOT_DIR=".kiro/file-snapshots"
CURRENT_SNAPSHOT="$SNAPSHOT_DIR/current-snapshot.md"
PREVIOUS_SNAPSHOT="$SNAPSHOT_DIR/previous-snapshot.md"
CHANGE_LOG="$SNAPSHOT_DIR/change-log.md"
TEMP_SNAPSHOT="/tmp/repo_snapshot_$$.md"

# Ensure snapshot directory exists
mkdir -p "$SNAPSHOT_DIR"

echo "Generating repository snapshot at $TIMESTAMP..."

# Generate file inventory with line and word counts
echo "# Repository Snapshot - $TIMESTAMP" > "$TEMP_SNAPSHOT"
echo "" >> "$TEMP_SNAPSHOT"

# Count all files except .git
TOTAL_FILES=$(find . -type f ! -path "./.git/*" | wc -l)
TOTAL_LINES=$(find . -type f ! -path "./.git/*" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
TOTAL_WORDS=$(find . -type f ! -path "./.git/*" -exec wc -w {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")

echo "## Summary Statistics" >> "$TEMP_SNAPSHOT"
echo "- Total Files: $TOTAL_FILES" >> "$TEMP_SNAPSHOT"
echo "- Total Lines: $(printf "%'d" $TOTAL_LINES)" >> "$TEMP_SNAPSHOT"
echo "- Total Words: $(printf "%'d" $TOTAL_WORDS)" >> "$TEMP_SNAPSHOT"
echo "" >> "$TEMP_SNAPSHOT"

echo "## File Inventory" >> "$TEMP_SNAPSHOT"
echo "" >> "$TEMP_SNAPSHOT"
echo "| File Path | Lines | Words |" >> "$TEMP_SNAPSHOT"
echo "|-----------|-------|-------|" >> "$TEMP_SNAPSHOT"

# Generate file listing with counts
find . -type f ! -path "./.git/*" | sort | while read -r file; do
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file" 2>/dev/null || echo "0")
        words=$(wc -w < "$file" 2>/dev/null || echo "0")
        echo "| $file | $lines | $words |" >> "$TEMP_SNAPSHOT"
    fi
done

# Move previous snapshot if it exists
if [ -f "$CURRENT_SNAPSHOT" ]; then
    cp "$CURRENT_SNAPSHOT" "$PREVIOUS_SNAPSHOT"
    
    # Generate delta report
    echo "" >> "$TEMP_SNAPSHOT"
    echo "## Changes Since Previous Snapshot" >> "$TEMP_SNAPSHOT"
    
    # Extract previous stats
    if [ -f "$PREVIOUS_SNAPSHOT" ]; then
        PREV_FILES=$(grep "Total Files:" "$PREVIOUS_SNAPSHOT" | sed 's/.*: //' | tr -d ',')
        PREV_LINES=$(grep "Total Lines:" "$PREVIOUS_SNAPSHOT" | sed 's/.*: //' | tr -d ',')
        PREV_WORDS=$(grep "Total Words:" "$PREVIOUS_SNAPSHOT" | sed 's/.*: //' | tr -d ',')
        
        FILE_DIFF=$((TOTAL_FILES - PREV_FILES))
        LINE_DIFF=$((TOTAL_LINES - PREV_LINES))
        WORD_DIFF=$((TOTAL_WORDS - PREV_WORDS))
        
        echo "- File Count Change: $FILE_DIFF" >> "$TEMP_SNAPSHOT"
        echo "- Line Count Change: $(printf "%'d" $LINE_DIFF)" >> "$TEMP_SNAPSHOT"
        echo "- Word Count Change: $(printf "%'d" $WORD_DIFF)" >> "$TEMP_SNAPSHOT"
    fi
fi

# Move temp snapshot to current
mv "$TEMP_SNAPSHOT" "$CURRENT_SNAPSHOT"

# Update change log
if [ ! -f "$CHANGE_LOG" ]; then
    echo "# Repository Change Log" > "$CHANGE_LOG"
    echo "" >> "$CHANGE_LOG"
fi

echo "## $TIMESTAMP - Automated Snapshot" >> "$CHANGE_LOG"
echo "**Type**: automated-snapshot" >> "$CHANGE_LOG"
echo "" >> "$CHANGE_LOG"
echo "### Summary" >> "$CHANGE_LOG"
echo "- **Total Files**: $TOTAL_FILES" >> "$CHANGE_LOG"
echo "- **Total Lines**: $(printf "%'d" $TOTAL_LINES)" >> "$CHANGE_LOG"
echo "- **Total Words**: $(printf "%'d" $TOTAL_WORDS)" >> "$CHANGE_LOG"

if [ -f "$PREVIOUS_SNAPSHOT" ]; then
    echo "- **File Change**: $FILE_DIFF" >> "$CHANGE_LOG"
    echo "- **Line Change**: $(printf "%'d" $LINE_DIFF)" >> "$CHANGE_LOG"
    echo "- **Word Change**: $(printf "%'d" $WORD_DIFF)" >> "$CHANGE_LOG"
fi

echo "" >> "$CHANGE_LOG"
echo "---" >> "$CHANGE_LOG"
echo "" >> "$CHANGE_LOG"

# Commit changes to git
git add .kiro/file-snapshots/
git commit -m "automated-snapshot repository-tracking $(date '+%Y %m %d IST')" || echo "No changes to commit"

echo "Repository snapshot generated successfully!"
echo "- Current snapshot: $CURRENT_SNAPSHOT"
echo "- Change log updated: $CHANGE_LOG"
echo "- Files tracked: $TOTAL_FILES"
echo "- Total lines: $(printf "%'d" $TOTAL_LINES)"
echo "- Total words: $(printf "%'d" $TOTAL_WORDS)"