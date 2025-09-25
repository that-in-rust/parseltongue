# Task 2: The "Kiro Steering User" Developer Experience

## 🎯 Goal
**Seamless integration into Kiro workflow with folder copy + one file**

## 🚀 Implementation Plan

### Step 1: Self-Contained Distribution
```
my-rust-project/
├── .kiro/
│   └── steering/
│       └── parseltongue.md  ← Copy this one file
├── parseltongue/            ← Copy this entire folder
│   ├── parseltongue         ← Binary
│   ├── pt                   ← Wrapper script
│   └── prompts.md           ← LLM prompts
└── src/
```

### Step 2: Complete Kiro Integration File
**File: `distribution/kiro-complete/parseltongue.md`**
```markdown
# Parseltongue Integration

## Quick Setup
```bash
# Copy parseltongue folder to your project root
cp -r /path/to/parseltongue ./parseltongue
./parseltongue/pt map
```

## Essential Workflow
```bash
# Before any code changes
./parseltongue/pt map                    # Understand codebase
./parseltongue/pt impact UserService     # Check change impact
./parseltongue/pt safe auth_module       # Verify safety

# During development
./parseltongue/pt find MyStruct          # Locate entities
./parseltongue/pt trace handle_request   # Understand flow
./parseltongue/pt context MyStruct       # Generate LLM context
```

## LLM Integration
Copy this output to Claude/ChatGPT:
```bash
./parseltongue/pt context MyStruct | pbcopy
```

Then use this prompt:
"Analyze this Rust code structure and suggest improvements: [paste output]"
```

### Step 3: LLM Prompt Templates
**File: `distribution/kiro-complete/llm-prompts.md`**
```markdown
# Copy-Paste LLM Prompts

## Architecture Analysis
"I'm analyzing a Rust codebase. Here's the Parseltongue analysis:

[PASTE OUTPUT HERE]

Please help me:
1. Identify the main architectural patterns
2. Find potential refactoring opportunities  
3. Suggest performance improvements
4. Highlight areas of technical debt"

## Feature Development
"I want to add a new feature. Here's the impact analysis:

[PASTE `./parseltongue/pt impact <entity>` OUTPUT]

Help me:
1. Plan the safest implementation approach
2. Identify all integration points
3. Suggest comprehensive test strategies"

## Code Review
"I'm reviewing changes to this code. Here's the context:

[PASTE `./parseltongue/pt context <entity>` OUTPUT]

Review for:
1. Breaking changes to dependent code
2. Performance implications
3. Architectural consistency"
```

### Step 4: Workflow Integration Examples
**File: `distribution/kiro-complete/workflow-examples.md`**
```markdown
# Real Workflow Examples

## Example 1: Adding Authentication
```bash
# Step 1: Understand current auth
./parseltongue/pt find AuthService
./parseltongue/pt trace authenticate

# Step 2: Check impact of changes
./parseltongue/pt impact AuthService

# Step 3: Generate context for LLM
./parseltongue/pt context AuthService > auth_context.md

# Step 4: Use with Claude
# "Help me add OAuth to this auth system: [paste auth_context.md]"
```

## Example 2: Refactoring Database Layer
```bash
# Step 1: Safety check
./parseltongue/pt safe DatabasePool

# Step 2: Find all usages
./parseltongue/pt trace DatabasePool

# Step 3: Plan changes with LLM
./parseltongue/pt context DatabasePool | pbcopy
# "Is it safe to refactor this database code? [paste]"
```
```

## 📊 Success Metrics
- [ ] Copy folder + one file = working integration
- [ ] All commands work from project root
- [ ] LLM prompts produce useful output
- [ ] Workflow examples are actionable

## 🧪 Test Protocol
1. Fresh Rust project
2. Copy distribution folder
3. Copy steering file
4. Run workflow examples
5. Test LLM integration
6. Verify all commands work