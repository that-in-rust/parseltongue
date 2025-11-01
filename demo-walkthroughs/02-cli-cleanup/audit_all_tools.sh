#!/bin/bash
cd /Users/amuldotexe/Projects/parseltongue/demo-walkthroughs/02-cli-cleanup

echo "=== UNIFIED BINARY CLI AUDIT ==="
echo ""
for tool in "folder-to-cozodb-streamer" "llm-to-cozodb-writer" "llm-cozodb-to-context-writer" "rust-preflight-code-simulator" "llm-cozodb-to-diff-writer" "cozodb-make-future-code-current"; do
  echo "Tool: $tool"
  echo "---"
  ../../target/release/parseltongue $tool --help 2>&1 | grep -A 20 "Options:" | head -15
  echo ""
done
