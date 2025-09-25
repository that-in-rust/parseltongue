# 🤖 LLM Prompts for Parseltongue

## Copy-Paste Ready Prompts

### 1. Codebase Analysis Prompt
```
I'm analyzing a Rust codebase. Here's the entity map from Parseltongue:

[PASTE PARSELTONGUE OUTPUT HERE]

Based on this analysis:
1. What are the main architectural patterns?
2. Which entities are most critical (high connectivity)?
3. What are potential refactoring opportunities?
4. Where should I focus for performance improvements?
```

### 2. Feature Development Prompt
```
I want to add a new feature to this Rust codebase. Here's the impact analysis:

[PASTE `parseltongue feature-start <entity>` OUTPUT]

Help me:
1. Plan the implementation approach
2. Identify integration points
3. Suggest test strategies
4. Highlight potential risks
```

### 3. Refactoring Safety Prompt
```
I'm planning to refactor this code. Here's the safety analysis:

[PASTE `parseltongue refactor-check <target>` OUTPUT]

Please:
1. Confirm the refactoring is safe
2. Suggest the safest refactoring order
3. Identify any missed dependencies
4. Recommend testing strategies
```

### 4. Debug Analysis Prompt
```
I'm debugging an issue with this entity. Here's the usage trace:

[PASTE `parseltongue debug <entity>` OUTPUT]

Help me:
1. Understand the call flow
2. Identify potential failure points
3. Suggest debugging strategies
4. Find related entities to investigate
```

### 5. Code Review Prompt
```
I'm reviewing changes to this Rust code. Here's the context:

[PASTE `parseltongue generate-context <entity>` OUTPUT]

Please review for:
1. Architectural consistency
2. Potential breaking changes
3. Performance implications
4. Testing completeness
```

## Quick Commands for LLM Context

```bash
# Generate rich context for any entity
parseltongue generate-context MyStruct --format markdown

# Get blast radius for impact analysis  
parseltongue query blast-radius MyFunction --depth 3

# List all entities for overview
parseltongue list-entities --format json

# Get file-specific context
parseltongue entities-in-file src/main.rs
```

## Integration with AI Tools

### Claude/ChatGPT
1. Run parseltongue command
2. Copy output
3. Paste into prompt template above
4. Get architectural insights

### GitHub Copilot
Use parseltongue output as context comments:
```rust
// Context from: parseltongue generate-context UserService
// Dependencies: AuthService, DatabasePool, ValidationService
// Callers: UserController, AdminController
// Usage pattern: Service layer with dependency injection

impl UserService {
    // Implementation here
}
```

### Cursor/AI IDEs
Include parseltongue analysis in your project context files for better AI suggestions.