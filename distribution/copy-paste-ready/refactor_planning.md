# LLM Instructions: Refactor Planning with Parseltongue

## Purpose
These instructions guide LLMs in planning safe, effective refactoring using Parseltongue's impact analysis capabilities.

## Refactoring Risk Assessment Framework

### Risk Categories Based on Impact Count
- **Low Risk (1-5 impacts)**: Standard refactoring with unit tests
- **Medium Risk (6-20 impacts)**: Comprehensive testing required
- **High Risk (21-50 impacts)**: Incremental approach with integration tests
- **Critical Risk (50+ impacts)**: Architectural review and phased rollout

## Pre-Refactoring Analysis Workflow

### Step 1: Target Entity Identification (1-2 minutes)
```bash
# Find the entity to refactor
parseltongue where-defined EntityName > target_location.txt
```

**LLM Tasks:**
1. Verify entity exists and is correctly identified
2. Understand entity's current role and responsibilities
3. Document current behavior that needs to change

### Step 2: Impact Analysis (2-3 minutes)
```bash
# Analyze blast radius
parseltongue blast-radius EntityName > impact_analysis.txt
```

**LLM Tasks:**
1. Count total impacted entities
2. Categorize impacts by relationship type (CALLS, USES, IMPLEMENTS)
3. Identify production vs test code impacts
4. Map impact to architectural layers

### Step 3: Risk Assessment (1-2 minutes)
**LLM Risk Evaluation:**
```markdown
## Risk Assessment for [EntityName]

### Impact Metrics
- Total Impacts: [count]
- Production Code Impacts: [count]
- Test Code Impacts: [count]
- Risk Level: [LOW/MEDIUM/HIGH/CRITICAL]

### Risk Factors
- [ ] Core business logic entity
- [ ] Shared utility function
- [ ] Public API interface
- [ ] Database interaction layer
- [ ] Error handling component
```

## Refactoring Strategy Templates

### Low Risk Refactoring (1-5 impacts)
```markdown
## Low Risk Refactoring Plan

### Approach: Direct Modification
1. **Modify Target Entity**: Make changes directly to EntityName
2. **Update Tests**: Modify unit tests for EntityName
3. **Update Callers**: Update 1-5 calling entities
4. **Validate**: Run affected tests

### Timeline: 1-2 hours
### Testing Strategy: Unit tests + smoke tests
```

### Medium Risk Refactoring (6-20 impacts)
```markdown
## Medium Risk Refactoring Plan

### Approach: Test-First Modification
1. **Comprehensive Testing**: Write tests for all impacted entities
2. **Incremental Changes**: Modify EntityName in small steps
3. **Integration Testing**: Test entity interactions
4. **Rollback Plan**: Prepare revert strategy

### Timeline: 4-8 hours
### Testing Strategy: Unit + integration + regression tests
```

### High Risk Refactoring (21-50 impacts)
```markdown
## High Risk Refactoring Plan

### Approach: Incremental Migration
1. **Create New Interface**: Define new entity interface
2. **Parallel Implementation**: Implement alongside existing
3. **Gradual Migration**: Move callers incrementally
4. **Deprecate Old**: Remove old implementation last

### Timeline: 1-2 days
### Testing Strategy: Full test suite + manual verification
```

### Critical Risk Refactoring (50+ impacts)
```markdown
## Critical Risk Refactoring Plan

### Approach: Architectural Migration
1. **Architecture Review**: Team review of proposed changes
2. **Feature Flags**: Implement toggle between old/new
3. **Phased Rollout**: Deploy incrementally with monitoring
4. **Rollback Strategy**: Immediate revert capability

### Timeline: 1-2 weeks
### Testing Strategy: Full regression + performance + load tests
```

## Refactoring Execution Checklist

### Pre-Refactoring Validation
- [ ] Impact analysis completed and documented
- [ ] Risk level assessed and appropriate strategy selected
- [ ] All affected entities identified and understood
- [ ] Test strategy defined and approved
- [ ] Rollback plan prepared

### During Refactoring
- [ ] Changes made incrementally with frequent testing
- [ ] Each step validated before proceeding
- [ ] Impact entities updated as needed
- [ ] Tests passing at each checkpoint

### Post-Refactoring Validation
- [ ] All impacted entities still function correctly
- [ ] Performance impact measured and acceptable
- [ ] Documentation updated to reflect changes
- [ ] Team notified of changes and new patterns

## Common Refactoring Scenarios

### Scenario 1: Function Signature Change
```bash
# Analyze function usage
parseltongue blast-radius FunctionName | grep "CALLS"
```
**Strategy**: Update signature, then update all callers systematically

### Scenario 2: Struct Field Modification
```bash
# Find struct usage patterns
parseltongue blast-radius StructName | grep "USES"
```
**Strategy**: Add new field first, migrate usage, remove old field

### Scenario 3: Trait Interface Change
```bash
# Find all implementations
parseltongue blast-radius TraitName | grep "IMPLEMENTS"
```
**Strategy**: Extend trait first, migrate implementations, remove old methods

### Scenario 4: Module Reorganization
```bash
# Analyze cross-module dependencies
parseltongue list-entities | grep "ModuleName::"
```
**Strategy**: Create new module structure, move entities incrementally

## Integration with Development Tools

### Version Control Strategy
- Create feature branch for refactoring
- Commit frequently with descriptive messages
- Tag major milestones for easy rollback

### Testing Integration
- Run full test suite before starting
- Run affected tests after each change
- Add new tests for modified behavior

### Code Review Process
- Include impact analysis in PR description
- Highlight risk level and mitigation strategy
- Request appropriate reviewer based on risk level

## Success Metrics

### Refactoring Success Indicators:
1. **Functionality Preserved**: All existing behavior maintained
2. **Performance Maintained**: No significant performance regression
3. **Test Coverage**: All changes covered by tests
4. **Documentation Updated**: Changes reflected in documentation
5. **Team Understanding**: Changes communicated and understood

### Time Targets by Risk Level:
- **Low Risk**: <2 hours
- **Medium Risk**: <8 hours  
- **High Risk**: <2 days
- **Critical Risk**: <2 weeks

## Emergency Procedures

### If Refactoring Goes Wrong:
1. **Stop Immediately**: Don't continue with broken state
2. **Assess Damage**: Use impact analysis to understand scope
3. **Rollback Strategy**: Revert to last known good state
4. **Root Cause Analysis**: Understand what went wrong
5. **Revised Plan**: Create new approach based on learnings