# Parseltongue Kiro Steering Integration

Copy this entire file to `.kiro/steering/parseltongue.md` in your project.

```markdown
---
inclusion: manual
---

# Parseltongue: Architectural Intelligence

## 🚀 Essential Commands (Copy-Paste Ready)

### Before Any Code Changes
```bash
# Map the codebase (run once per session)
parseltongue onboard .

# Analyze impact before changes
parseltongue feature-start <entity_name>

# Check refactoring safety
parseltongue refactor-check <target>
```

### During Development
```bash
# Find entity definitions instantly
parseltongue where-defined <entity_name>

# Trace usage patterns
parseltongue debug <entity_name>

# Generate LLM context
parseltongue generate-context <entity_name>
```

### Architecture Analysis
```bash
# Blast radius analysis
parseltongue query blast-radius <entity> --depth 3

# Dependency mapping
parseltongue query dependencies <entity>

# Caller analysis
parseltongue query callers <entity>
```

## 🎯 Workflow Integration

### 1. Feature Development Workflow
```bash
# Step 1: Understand the area
parseltongue feature-start UserService

# Step 2: Find integration points
parseltongue where-defined UserService
parseltongue query dependencies UserService

# Step 3: Check impact
parseltongue query blast-radius UserService --depth 2

# Step 4: Generate context for LLM
parseltongue generate-context UserService --format markdown
```

### 2. Refactoring Workflow
```bash
# Step 1: Safety check
parseltongue refactor-check auth_module

# Step 2: Understand current usage
parseltongue debug AuthService

# Step 3: Map dependencies
parseltongue query callers AuthService
parseltongue query dependencies AuthService

# Step 4: Plan changes with full context
parseltongue generate-context AuthService
```

### 3. Code Review Workflow
```bash
# Generate context for changed entities
parseltongue generate-context <changed_entity>

# Check blast radius of changes
parseltongue query blast-radius <changed_entity>

# Verify integration points
parseltongue query callers <changed_entity>
```

## 📊 Performance Expectations
- Entity queries: <100ms
- Blast radius: <500μs  
- File analysis: <2s for large files
- Complete onboarding: <15 minutes

## 🤖 LLM Integration Patterns

### Generate Rich Context
```bash
# For architectural discussions
parseltongue generate-context MyStruct --format markdown

# For impact analysis
parseltongue feature-start MyStruct | pbcopy  # macOS
parseltongue feature-start MyStruct | xclip   # Linux
```

### Common LLM Prompts
1. **Architecture Review**: "Analyze this Rust codebase structure: [paste parseltongue output]"
2. **Feature Planning**: "Help me implement this feature safely: [paste feature-start output]"
3. **Refactoring**: "Is this refactoring safe? [paste refactor-check output]"

## 🛠️ Setup (One-Time)
```bash
# Copy binary to project or PATH
cp /path/to/parseltongue ./parseltongue
chmod +x ./parseltongue

# Test installation
./parseltongue --version  # Should show: parseltongue 0.1.0
```

## 🎯 Best Practices

### Always Before Major Changes
1. Run `parseltongue onboard .` to refresh understanding
2. Use `parseltongue feature-start <entity>` for impact analysis
3. Check `parseltongue refactor-check <target>` for safety

### During Code Reviews
1. Generate context: `parseltongue generate-context <entity>`
2. Check blast radius: `parseltongue query blast-radius <entity>`
3. Verify no breaking changes to callers

### For New Team Members
1. Start with: `parseltongue onboard .`
2. Explore entities: `parseltongue list-entities`
3. Understand key components: `parseltongue generate-context <key_entity>`

## 🚫 What NOT to Use Parseltongue For
- Real-time monitoring (it's a manual analysis tool)
- Automated CI/CD (designed for human-driven analysis)
- Performance profiling (use proper profilers)
- Dependency management (use Cargo)

## 💡 Pro Tips
- Use `--format json` for tooling integration
- Combine with `jq` for filtering: `parseltongue list-entities --format json | jq '.functions'`
- Generate markdown reports: `parseltongue onboard . --output report.md`
- Use blast radius before any breaking changes
```

## Installation Note
Place the `parseltongue` binary in your project root or add to PATH. No other dependencies required.