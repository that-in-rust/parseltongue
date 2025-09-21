# Hook System Status

## Active Hooks

### Unified Progress Tracker
- **File**: `.kiro/hooks/unified-progress-tracker.kiro.hook`
- **Trigger**: File saved on `**/*` (excludes `.git/**/*`)
- **Script**: `.kiro/unified-progress-tracker.sh`
- **Behavior**:
  - Always: Repository snapshots + session context updates
  - Only .kiro/ changes: Git commits to v01 branch
- **Status**: ✅ Active and tested

## Hook Behavior Verification

### Test Results (2025-01-20)
- ✅ `.kiro/test-spec.md` → Git commit created
- ✅ `test-regular-file.md` → No git commit (correct)
- ✅ Both files → Repository snapshots generated

## Key Scripts
- `.kiro/unified-progress-tracker.sh` - Main automation
- `.kiro/tree-with-wc.sh` - Repository analysis utility

## Git Integration
- **Target Branch**: v01
- **Commit Pattern**: "unified-progress [category] [timestamp]"
- **Scope**: Only `.kiro/` directory changes committed