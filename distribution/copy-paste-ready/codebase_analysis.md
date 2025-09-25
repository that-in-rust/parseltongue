# LLM Instructions: Codebase Analysis with Parseltongue

## Purpose
These instructions guide LLMs in performing effective codebase analysis using Parseltongue's discovery-first architectural intelligence capabilities.

## Prerequisites
- Parseltongue binary available as `parseltongue_YYYYMMDDHHMMSS`
- Codebase ingested with `parseltongue ingest codebase.dump`
- Entity listings generated with discovery commands

## Analysis Workflow

### Phase 1: Entity Discovery (2-3 minutes)
```bash
# Get comprehensive entity overview
parseltongue list-entities --limit 100 > entities_overview.txt

# Focus on key entity types
parseltongue list-entities --type functions --limit 50 > functions.txt
parseltongue list-entities --type structs --limit 30 > structs.txt
parseltongue list-entities --type traits --limit 20 > traits.txt
```

**LLM Analysis Tasks:**
1. Identify the top 10 most important functions based on naming patterns
2. Categorize structs by domain (data models, services, utilities)
3. Map traits to architectural patterns (Repository, Service, Handler)
4. Detect naming conventions and architectural styles

### Phase 2: Architectural Pattern Recognition (3-5 minutes)
```bash
# Look for common patterns
grep -i "service\|handler\|repository\|manager\|builder" entities_overview.txt
```

**LLM Analysis Tasks:**
1. **Service Pattern**: Look for `*Service` entities - these are business logic layers
2. **Repository Pattern**: Look for `*Repository` or `*Repo` - these are data access layers  
3. **Handler Pattern**: Look for `*Handler` - these are request/response processors
4. **Builder Pattern**: Look for `*Builder` - these are object construction utilities
5. **Manager Pattern**: Look for `*Manager` - these are resource management components

### Phase 3: Relationship Analysis (5-7 minutes)
```bash
# Analyze key entity relationships
parseltongue blast-radius KeyEntityName > entity_relationships.txt
```

**LLM Analysis Tasks:**
1. Map entity dependencies and call graphs
2. Identify architectural layers and boundaries
3. Detect potential circular dependencies
4. Find central/hub entities with high connectivity

### Phase 4: Domain Understanding (5-10 minutes)
**LLM Analysis Tasks:**
1. **Domain Modeling**: Group entities by business domain
2. **Data Flow**: Trace how data moves through the system
3. **Entry Points**: Identify main functions, HTTP handlers, CLI commands
4. **Error Handling**: Map error types and propagation patterns

## Analysis Templates

### Entity Categorization Template
```markdown
## Entity Analysis Summary

### Core Business Logic
- [List key service/business entities]

### Data Models
- [List main struct/data entities]

### Infrastructure
- [List utility/infrastructure entities]

### Interfaces/Contracts
- [List trait/interface entities]
```

### Architectural Pattern Template
```markdown
## Architectural Patterns Detected

### Layered Architecture
- Presentation Layer: [Handler entities]
- Business Layer: [Service entities]  
- Data Layer: [Repository entities]

### Design Patterns
- [List detected patterns with examples]

### Dependencies
- [Map key dependencies between layers]
```

## Common Analysis Pitfalls to Avoid

### ❌ Don't Do This
- Analyze entities without understanding their relationships
- Focus only on function names without considering structs/traits
- Make assumptions about architecture without blast-radius analysis
- Ignore error handling and utility entities

### ✅ Do This Instead
- Start with entity discovery, then analyze relationships
- Consider all entity types (functions, structs, traits) together
- Use blast-radius analysis to understand impact and dependencies
- Map complete data flow including error paths

## Success Criteria

### Effective Analysis Includes:
1. **Complete Entity Inventory**: All major entities identified and categorized
2. **Architectural Understanding**: Clear picture of system structure and patterns
3. **Relationship Mapping**: Understanding of how entities interact
4. **Domain Knowledge**: Business logic and data flow comprehension
5. **Actionable Insights**: Specific recommendations for development tasks

### Time Targets:
- Entity Discovery: <3 minutes
- Pattern Recognition: <5 minutes  
- Relationship Analysis: <7 minutes
- Domain Understanding: <10 minutes
- **Total Analysis Time: <15 minutes**

## Output Format

Structure your analysis as:
1. **Executive Summary** (2-3 sentences about the codebase)
2. **Entity Overview** (counts and key entities by type)
3. **Architectural Patterns** (detected patterns with examples)
4. **Key Relationships** (important dependencies and data flows)
5. **Development Recommendations** (actionable next steps)

## Integration with Development Workflow

After analysis, use insights for:
- **Feature Planning**: Impact analysis with `blast-radius` command
- **Debugging**: Entity location with `where-defined` command  
- **Refactoring**: Relationship analysis before changes
- **Code Review**: Architectural context for changes