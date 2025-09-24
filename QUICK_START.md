# 🚀 Parseltongue: 30-Second Setup

## What This Does
**Understand any Rust codebase in 30 seconds instead of 30 minutes.**

```bash
# Download and run - no installation needed
./parseltongue onboard .
# ✅ Complete codebase map in <15 minutes
# ✅ Find any entity instantly
# ✅ Understand blast radius before changes
```

## Copy-Paste Integration

### 1. Get the Binary
```bash
# Download from distribution/binaries/parseltongue (4.3MB)
chmod +x parseltongue
./parseltongue --version  # Should show: parseltongue 0.1.0
```

### 2. Essential Commands (Copy These)
```bash
# Understand the codebase
./parseltongue onboard .

# Before making changes
./parseltongue feature-start UserService
./parseltongue refactor-check auth_module

# Find anything instantly
./parseltongue where-defined MyStruct
./parseltongue debug handle_request
```

### 3. Kiro Steering Integration
Copy this to `.kiro/steering/parseltongue.md`:

```markdown
# Parseltongue Commands

## Before Code Changes
- `parseltongue onboard .` - Map the codebase (15 min)
- `parseltongue feature-start <entity>` - Impact analysis (5 min)
- `parseltongue refactor-check <target>` - Safety check (3 min)

## During Development  
- `parseltongue where-defined <entity>` - Find definitions
- `parseltongue debug <entity>` - Trace usage patterns
- `parseltongue generate-context <entity>` - LLM context

## Performance Expectations
- Entity queries: <100ms
- Blast radius: <500μs
- File analysis: <2s
```

## That's It
- ✅ No installation, no dependencies
- ✅ Works on any Rust project immediately  
- ✅ Perfect for steering docs and workflows
- ✅ 4.3MB binary, runs anywhere