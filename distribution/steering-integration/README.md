# Parseltongue Steering Integration

## 🎯 **Quick Integration Guide**

### **Binary Usage**
```bash
# Copy the binary to your PATH
cp distribution/binaries/parseltongue /usr/local/bin/

# Or use directly
./distribution/binaries/parseltongue --help
```

### **Steering Documentation Integration**

#### **For Kiro Projects**
Add to your `.kiro/steering/parseltongue-integration.md`:

```markdown
# Parseltongue Integration

## Discovery Commands
- `parseltongue onboard .` - Complete codebase analysis
- `parseltongue list-entities` - Browse all entities
- `parseltongue where-defined <entity>` - Find entity location
- `parseltongue debug <entity>` - Trace usage patterns

## Workflow Integration
- Use `parseltongue feature-start <entity>` before major changes
- Run `parseltongue refactor-check <target>` before refactoring
- Generate context with `parseltongue generate-context <entity>`
```

#### **Essential Steering Rules**
```markdown
# Parseltongue Best Practices

## Before Code Changes
1. Run `parseltongue onboard .` to understand the codebase
2. Use `parseltongue feature-start <entity>` for impact analysis
3. Check `parseltongue refactor-check <target>` for safety

## During Development
- Use `parseltongue where-defined <entity>` to locate definitions
- Run `parseltongue debug <entity>` to understand usage patterns
- Generate LLM context with `parseltongue generate-context <entity>`

## Architecture Analysis
- Blast radius: `parseltongue query blast-radius <entity> --depth 3`
- Dependencies: `parseltongue query dependencies <entity>`
- Callers: `parseltongue query callers <entity>`
```

### **No Hooks Needed**
Parseltongue is designed to be **lightweight and manual**:
- No automatic file watching
- No background processes
- Run commands when you need analysis
- Perfect for steering documentation integration

### **Integration Patterns**

#### **1. Pre-Feature Development**
```bash
# Understand the area you're working in
parseltongue feature-start UserService
```

#### **2. Refactoring Safety**
```bash
# Check impact before changes
parseltongue refactor-check authentication_module
```

#### **3. Code Review Preparation**
```bash
# Generate comprehensive context
parseltongue generate-context MyStruct --format markdown
```

#### **4. Onboarding New Developers**
```bash
# Complete codebase overview
parseltongue onboard . --output onboarding-guide.md
```

## 📦 **Distribution Contents**
- `binaries/parseltongue` - Ready-to-use binary (4.3MB)
- `steering-integration/` - Documentation and examples
- No dependencies, no installation required
- Works on macOS, Linux, Windows