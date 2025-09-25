# Task 1: The "GitHub Browser" Developer Experience

## 🎯 Goal
**30-second value demonstration for developers who land on the GitHub repo**

## 🚀 Implementation Plan

### Step 1: Create One-Liner Demo
```bash
# This should work immediately on any machine
curl -L https://github.com/user/parseltongue/releases/latest/download/parseltongue -o parseltongue && chmod +x parseltongue && ./parseltongue onboard .
```

### Step 2: Pre-Built Demo Output
Create a file showing Parseltongue analysis of a famous Rust project:

**File: `DEMO_OUTPUT_AXUM.md`**
```markdown
# Parseltongue Analysis: Axum Web Framework

## 30-Second Architecture Overview
- **1,147 entities** discovered in 139ms
- **Key Components**: Router (47 usages), Handler (23 usages), Middleware (15 usages)
- **Request Flow**: Router → Handler → Response (3-step pipeline)
- **Extension Points**: 12 trait implementations for custom behavior

## Instant Entity Discovery
```bash
./parseltongue where-defined Router
# → axum/src/routing/mod.rs:156
# → Used by: 47 files, 23 direct callers

./parseltongue debug Handler  
# → Call chain: Router::route() → Handler::call() → Response
# → Critical for: request processing, middleware integration
```

## Before vs After
**Before Parseltongue**: 2+ hours reading docs and code
**After Parseltongue**: 30 seconds to understand architecture
```

### Step 3: Visual Proof in README
Update README.md with:
1. **Hero section**: "Understand any Rust codebase in 30 seconds"
2. **Live demo**: Show actual Parseltongue output for Axum
3. **One-liner**: Copy-paste command that works immediately
4. **Value proof**: Before/after comparison

### Step 4: Instant Setup Script
```bash
#!/bin/bash
# setup.sh - One-line Parseltongue setup

echo "🐍 Setting up Parseltongue..."
curl -L https://github.com/user/parseltongue/releases/latest/download/parseltongue -o parseltongue
chmod +x parseltongue
echo "✅ Ready! Try: ./parseltongue onboard ."
```

## 📊 Success Metrics
- [ ] Developer understands value in <30 seconds
- [ ] Setup works with one command
- [ ] Demo shows real, recognizable output
- [ ] Clear next steps provided

## 🧪 Test Protocol
1. Show README to fresh developer
2. Time how long to understand value
3. Test one-liner setup on clean machine
4. Verify demo output is compelling
5. Check if they know what to do next