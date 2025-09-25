#!/bin/bash
# Tree with Word Count Script
# Usage: ./.kiro/tree-with-wc.sh

echo "# Repository Tree with Word Counts"
echo "Generated: $(date '+%Y-%m-%d %H:%M:%S IST')"
echo ""

find . -type f \
    -not -path "./.git/*" \
    -not -path "./target/*" \
    -not -path "./node_modules/*" \
    -not -path "./.kiro/file-snapshots/*" \
    | sort | while read -r file; do
    
    # Get file size
    SIZE=$(ls -lh "$file" | awk '{print $5}')
    
    # Check if file is text and get word count and line count
    if file "$file" | grep -q "text"; then
        WC=$(wc -w "$file" 2>/dev/null | awk '{print $1}')
        LC=$(wc -l "$file" 2>/dev/null | awk '{print $1}')
        echo "$file | $LC lines | $WC words | $SIZE"
    else
        echo "$file | [binary] | $SIZE"
    fi
done

echo ""
echo "## Summary"
TOTAL_FILES=$(find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "./node_modules/*" -not -path "./.kiro/file-snapshots/*" | wc -l)
TOTAL_TEXT_FILES=$(find . -type f -not -path "./.git/*" -not -path "./target/*" -not -path "./node_modules/*" -not -path "./.kiro/file-snapshots/*" -exec file {} \; | grep -c "text")

echo "- **Total Files**: $TOTAL_FILES"
echo "- **Text Files**: $TOTAL_TEXT_FILES"
echo "- **Binary Files**: $((TOTAL_FILES - TOTAL_TEXT_FILES))"

# Calculate total lines and words for major directories
echo ""
echo "## Directory Line & Word Counts"

if [ -d "_refDocs" ]; then
    REFDOCS_LINES=$(find _refDocs -name "*.md" -o -name "*.txt" -o -name "*.html" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    REFDOCS_WORDS=$(find _refDocs -name "*.md" -o -name "*.txt" -o -name "*.html" | xargs wc -w 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    REFDOCS_FILES=$(find _refDocs -type f | wc -l)
    echo "- **_refDocs**: $REFDOCS_LINES lines | $REFDOCS_WORDS words across $REFDOCS_FILES files"
fi

if [ -d "_refIdioms" ]; then
    REFIDIOMS_LINES=$(find _refIdioms -name "*.md" -o -name "*.txt" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    REFIDIOMS_WORDS=$(find _refIdioms -name "*.md" -o -name "*.txt" | xargs wc -w 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    REFIDIOMS_FILES=$(find _refIdioms -type f | wc -l)
    echo "- **_refIdioms**: $REFIDIOMS_LINES lines | $REFIDIOMS_WORDS words across $REFIDIOMS_FILES files"
fi

if [ -d ".kiro" ]; then
    KIRO_LINES=$(find .kiro -name "*.md" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    KIRO_WORDS=$(find .kiro -name "*.md" | xargs wc -w 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    KIRO_FILES=$(find .kiro -type f | wc -l)
    echo "- **.kiro**: $KIRO_LINES lines | $KIRO_WORDS words across $KIRO_FILES files"
fi