# Rust Complexity Analysis - Quick Reference

## Essential Commands for Rust Pattern Analysis

### Trait Analysis
```bash
# Find all implementors of a trait
./bin/parseltongue query what-implements <TraitName>

# Find all users of a trait/type
./bin/parseltongue query uses <TypeName>

# Get comprehensive trait context
./bin/parseltongue generate-context <TraitName>
```

### Complex Type Analysis
```bash
# Get full type composition and dependencies
./bin/parseltongue generate-context <ComplexType>

# Find change impact
./bin/parseltongue query blast-radius <TypeName>
```

### Async Pattern Analysis
```bash
# Map async function dependencies
./bin/parseltongue generate-context <async_function>

# Find async function callers
./bin/parseltongue query calls <async_function>
```

### Serialization/Macro Analysis
```bash
# Find serialization patterns
./bin/parseltongue query calls to_bytes
./bin/parseltongue query uses from_bytes
./bin/parseltongue generate-context from_bytes
```

### Dependency Analysis
```bash
# Find circular dependencies
./bin/parseltongue query find-cycles <TypeName>

# Understand call chains
./bin/parseltongue query calls <FunctionName>
```

## Quick Analysis Workflows

### New Codebase Understanding
1. `./bin/parseltongue debug --graph | grep -i "client\|server" | head -10`
2. `./bin/parseltongue generate-context <MainType>`
3. `./bin/parseltongue query what-implements <CoreTrait>`

### Change Impact Assessment
1. `./bin/parseltongue query blast-radius <TypeToChange>`
2. `./bin/parseltongue query uses <TypeToChange>`
3. `./bin/parseltongue generate-context <TypeToChange>`

### Debugging Complex Issues
1. `./bin/parseltongue query calls <ProblemFunction>`
2. `./bin/parseltongue generate-context <ProblemFunction>`
3. `./bin/parseltongue query find-cycles <ProblemType>`

## Key Patterns in Message Streaming Systems

### Trait Composition Pattern
- Look for types implementing 10+ traits
- Use `generate-context` to see full composition
- Common in client architectures

### Async Dependency Chains
- Functions with 20+ dependencies are common
- Focus on state management and I/O operations
- Use `generate-context` for full dependency mapping

### Command/Serialization Pattern
- Universal `from_bytes`/`to_bytes` implementation
- Extensive test coverage for serialization
- Binary protocol support

## Pro Tips

1. **Start High-Level:** Begin with main abstractions, work down to details
2. **Combine Queries:** Use multiple query types for complete picture
3. **Focus on Patterns:** Look for architectural patterns, not individual functions
4. **Use JSON Output:** Add `--format json` for programmatic analysis
5. **Document Findings:** Create domain-specific analysis guides

## Common Issues and Solutions

### Hash-Only Output
- **Problem:** `blast-radius` returns hash values
- **Solution:** Use `generate-context` for readable names

### Generic Type Resolution
- **Problem:** Generic parameters not fully resolved
- **Solution:** Analyze concrete implementations with `what-implements`

### Macro Expansion
- **Problem:** Limited macro expansion details
- **Solution:** Focus on macro usage patterns and generated function calls