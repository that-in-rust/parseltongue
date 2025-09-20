# Parseltongue

Rust-based architectural intelligence system for real-time codebase analysis.

## Hook Automation System

### Unified Progress Tracker
Automatically tracks all development activity with intelligent git integration.

**Triggers**: Any file save (excludes `.git/` folder)  
**Actions**:
- Repository snapshots for all changes
- Session context updates
- Git commits only for `.kiro/` directory changes

**Status**: ✅ Active on v01 branch

### Usage
The hook runs automatically on file saves. Manual trigger available via Agent Hooks panel in Kiro IDE.

**Key Files**:
- `.kiro/hooks/unified-progress-tracker.kiro.hook` - Hook configuration
- `.kiro/unified-progress-tracker.sh` - Automation script
- `_refDocs/SESSION_CONTEXT.md` - Current session state

## Development

**Branch**: v01  
**Focus**: Rust-only architectural intelligence  
**Constraints**: <12ms updates, LLM-terminal integration  

See `.kiro/steering/` for detailed development guidelines.