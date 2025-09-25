# LLM Instructions: Architecture Review with Parseltongue

## Purpose
These instructions guide LLMs in conducting thorough architecture reviews using Parseltongue's discovery and analysis capabilities.

## Architecture Review Framework

### Review Scope Levels
- **Component Review**: Single entity or small group of related entities
- **Module Review**: Complete module or subsystem
- **System Review**: Entire codebase architecture
- **Integration Review**: Cross-system or API boundaries

## Pre-Review Discovery Phase

### Step 1: Architectural Inventory (5-10 minutes)
```bash
# Get complete entity overview
parseltongue list-entities --limit 200 > architectural_inventory.txt

# Categorize by architectural significance
parseltongue list-entities --type traits --limit 50 > interfaces.txt
parseltongue list-entities --type structs --limit 100 > data_models.txt
parseltongue list-entities --type functions --limit 150 > behaviors.txt
```

**LLM Analysis Tasks:**
1. **Interface Analysis**: Identify all trait definitions and their purposes
2. **Data Model Analysis**: Map struct relationships and data flow
3. **Behavior Analysis**: Understand function responsibilities and patterns

### Step 2: Dependency Mapping (10-15 minutes)
```bash
# Analyze key architectural entities
for entity in $(head -20 architectural_inventory.txt); do
    parseltongue blast-radius "$entity" > "dependencies_${entity}.txt"
done
```

**LLM Analysis Tasks:**
1. **Coupling Analysis**: Identify highly coupled entities
2. **Layer Violations**: Detect inappropriate cross-layer dependencies
3. **Circular Dependencies**: Find potential circular reference issues
4. **Hub Entities**: Identify central entities with high fan-in/fan-out

## Architecture Review Dimensions

### 1. Structural Quality Assessment

#### Layering and Separation of Concerns
```markdown
## Layering Analysis

### Presentation Layer
- Entities: [List handler/controller entities]
- Responsibilities: [User interface, request/response handling]
- Dependencies: [Should only depend on business layer]

### Business Logic Layer  
- Entities: [List service/domain entities]
- Responsibilities: [Core business rules and workflows]
- Dependencies: [Should not depend on presentation or infrastructure]

### Data Access Layer
- Entities: [List repository/data entities]
- Responsibilities: [Data persistence and retrieval]
- Dependencies: [Should be implementation details]

### Infrastructure Layer
- Entities: [List utility/infrastructure entities]
- Responsibilities: [Cross-cutting concerns]
- Dependencies: [Should be injected, not directly coupled]
```

#### Cohesion and Coupling Metrics
```markdown
## Coupling Analysis

### High Coupling Entities (>20 dependencies)
- [EntityName]: [dependency count] - [assessment]
- [Recommendation for decoupling]

### Low Cohesion Indicators
- [Entities with mixed responsibilities]
- [Recommendations for separation]
```

### 2. Design Pattern Compliance

#### Pattern Detection Template
```markdown
## Design Patterns Analysis

### Repository Pattern
- Implementation: [List repository entities]
- Compliance: [Good/Needs Improvement/Missing]
- Issues: [Specific problems found]

### Service Pattern
- Implementation: [List service entities]
- Compliance: [Good/Needs Improvement/Missing]
- Issues: [Specific problems found]

### Factory/Builder Pattern
- Implementation: [List builder entities]
- Compliance: [Good/Needs Improvement/Missing]
- Issues: [Specific problems found]

### Observer/Event Pattern
- Implementation: [List event-related entities]
- Compliance: [Good/Needs Improvement/Missing]
- Issues: [Specific problems found]
```

### 3. Error Handling Architecture

#### Error Flow Analysis
```bash
# Find error-related entities
parseltongue list-entities | grep -i "error\|exception\|result"
```

```markdown
## Error Handling Review

### Error Types
- [List error enum/struct entities]
- Coverage: [Comprehensive/Partial/Missing]

### Error Propagation
- [Map error flow through layers]
- Consistency: [Good/Inconsistent]

### Recovery Strategies
- [Document recovery mechanisms]
- Completeness: [Adequate/Insufficient]
```

### 4. Performance Architecture

#### Performance-Critical Path Analysis
```markdown
## Performance Architecture

### Hot Paths (High-frequency operations)
- [Identify performance-critical entities]
- Optimization Level: [Optimized/Needs Work/Unknown]

### Resource Management
- [Memory management patterns]
- [Connection pooling strategies]
- [Caching implementations]

### Scalability Patterns
- [Async/await usage]
- [Concurrency patterns]
- [Load distribution strategies]
```

## Architecture Quality Gates

### Quality Gate 1: Structural Integrity
- [ ] Clear layer separation with appropriate dependencies
- [ ] No circular dependencies between major components
- [ ] High cohesion within modules, low coupling between modules
- [ ] Consistent naming and organization patterns

### Quality Gate 2: Design Pattern Consistency
- [ ] Appropriate use of established design patterns
- [ ] Consistent implementation of similar patterns
- [ ] No anti-patterns or code smells
- [ ] Clear interfaces and abstractions

### Quality Gate 3: Error Handling Robustness
- [ ] Comprehensive error type definitions
- [ ] Consistent error propagation strategy
- [ ] Appropriate error recovery mechanisms
- [ ] Clear error boundaries and handling

### Quality Gate 4: Performance Readiness
- [ ] Performance-critical paths identified and optimized
- [ ] Appropriate resource management strategies
- [ ] Scalability patterns implemented where needed
- [ ] No obvious performance bottlenecks

## Review Output Templates

### Component Review Template
```markdown
# Component Architecture Review: [ComponentName]

## Overview
- Purpose: [Component's role in system]
- Entities: [List of entities in component]
- Dependencies: [Key dependencies]

## Structural Analysis
- Cohesion: [High/Medium/Low]
- Coupling: [Low/Medium/High]
- Complexity: [Simple/Moderate/Complex]

## Design Quality
- Pattern Compliance: [Good/Needs Improvement]
- Error Handling: [Robust/Adequate/Insufficient]
- Performance: [Optimized/Adequate/Concerning]

## Recommendations
1. [Specific improvement recommendations]
2. [Priority and effort estimates]
3. [Risk assessment for changes]
```

### System Review Template
```markdown
# System Architecture Review

## Executive Summary
- Overall Architecture Quality: [Excellent/Good/Needs Improvement/Poor]
- Key Strengths: [List top 3 strengths]
- Critical Issues: [List top 3 issues]
- Recommended Actions: [Priority-ordered list]

## Detailed Analysis
[Include component-level analysis for each major subsystem]

## Architecture Evolution Plan
1. **Immediate Actions** (1-2 weeks)
2. **Short-term Improvements** (1-3 months)  
3. **Long-term Evolution** (3-12 months)

## Risk Assessment
- **High Risk Areas**: [Components requiring immediate attention]
- **Technical Debt**: [Accumulated issues and remediation plan]
- **Scalability Concerns**: [Potential bottlenecks and solutions]
```

## Integration with Development Process

### Pre-Development Review
- Architecture review before major feature development
- Impact analysis for proposed changes
- Design validation against architectural principles

### Ongoing Review Process
- Regular architecture health checks
- Dependency drift monitoring
- Pattern compliance validation

### Post-Development Review
- Architecture impact assessment after major changes
- Technical debt identification and planning
- Lessons learned integration

## Success Metrics

### Review Effectiveness Indicators:
1. **Issue Detection Rate**: Percentage of architectural issues identified
2. **Resolution Tracking**: Progress on recommended improvements
3. **Quality Trend**: Architecture quality improvement over time
4. **Developer Satisfaction**: Team confidence in architectural decisions

### Time Targets:
- **Component Review**: 1-2 hours
- **Module Review**: 4-8 hours
- **System Review**: 1-2 days
- **Integration Review**: 2-4 hours

## Common Architecture Anti-Patterns to Detect

### Structural Anti-Patterns
- **God Object**: Single entity with too many responsibilities
- **Spaghetti Code**: Tangled dependencies without clear structure
- **Lava Flow**: Dead code that's never removed
- **Golden Hammer**: Overuse of single pattern/technology

### Design Anti-Patterns
- **Circular Dependencies**: Components depending on each other
- **Inappropriate Intimacy**: Components knowing too much about each other
- **Feature Envy**: Methods using more features of other classes than their own
- **Data Clumps**: Same group of data items appearing together repeatedly

Use Parseltongue's blast-radius analysis to detect these patterns systematically.