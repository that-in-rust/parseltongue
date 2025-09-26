# User Journey: Architectural Guardrails for Change Validation

**ID**: UJ-037
**Source**: DTNotes03.md - Scope Cop Script
**Persona**: DevOps Engineer, Team Lead
**Workflow Type**: CI/CD, Quality Assurance

## Current Pain Points
- LLMs and developers may modify files outside intended architectural scope
- No automated validation that changes align with architectural expectations
- Architectural drift occurs gradually without detection
- Manual code review cannot catch all scope violations
- Changes may have unintended architectural consequences

## Proposed Solution
Implement "Scope Cop" - an architectural guardrail system that validates actual changes against expected architectural impact:

- Compare `git diff` results with Parseltongue impact analysis
- Act as CI/CD gate or pre-commit hook
- Prevent architectural violations before they enter the codebase
- Provide clear feedback on scope violations with specific file lists

## Technical Implementation
```bash
# Pre-commit hook or CI/CD integration
./pt-scope-cop.sh CoreEntityName

# Workflow:
# 1. Get expected impacted files from Parseltongue
# 2. Get actual changed files from git diff
# 3. Compare using comm to find violations
# 4. Report violations or approve changes
```

## Success Metrics
- **Architectural Integrity**: 95% reduction in out-of-scope changes
- **CI/CD Reliability**: Automated detection of architectural violations
- **Developer Feedback**: Clear, actionable violation reports
- **False Positive Rate**: <5% incorrect violation flags

## Integration Requirements
- Parseltongue impact analysis with `--format=files_only` output
- Git integration for change detection
- CI/CD pipeline integration (GitHub Actions, Jenkins, etc.)
- Pre-commit hook framework compatibility
- Clear violation reporting format

## Expected Outcomes
- Automated prevention of architectural scope violations
- Maintained architectural integrity across development cycles
- Reduced architectural debt accumulation
- Enhanced confidence in automated development workflows
- Clear feedback loop for developers on architectural boundaries

## Dependencies
- Parseltongue impact analysis functionality
- Git version control system
- Shell scripting environment (bash)
- CI/CD pipeline or pre-commit hook system
- File comparison utilities (comm, diff)

## Priority
**Critical** - Essential for maintaining architectural integrity in automated development environments

## Related Insights
- Links to TI-033: Architectural Scope Validation System
- Supports ST-025: Architectural-Aware Development Ecosystem
- Connects to UJ-010: Intelligent CI/CD Quality Gates (DTNote01.md)
- Relates to UJ-034: Blast Radius Guided Quality Assurance (DTNote01.md)