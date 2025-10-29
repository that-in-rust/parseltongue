# Micro-PRD: Fix Compiler Warnings in Parseltongue Tools 1-3

## Bug Description
Parseltongue has 31 compiler warnings across parseltongue-core and Tools 1-3 that need to be fixed.

## Problem Statement
- **Severity**: Low (warnings don't break functionality)
- **Impact**: Code quality, clean compilation output
- **Scope**: 4 packages (parseltongue-core, parseltongue-01, parseltongue-02, parseltongue-03)

## Specific Issues

### parseltongue-core (3 warnings)
- `temporal.rs:541` - unused parameters `changes` and `conflicts` in `attempt_merge` function
- `temporal.rs:21` - unused field `pending_changes` in `TemporalVersioningManager` struct

### parseltongue-01 (1 warning)
- `isgl1_generator.rs:47` - unused field `rust_language` in `Isgl1KeyGeneratorImpl` struct

### parseltongue-02 (13 warnings)
- Multiple unused imports (6 total)
- Unused variable `total_entities`
- Unused variable `term`
- Dead code in LLM response structs (8 fields)

### parseltongue-03 (14 warnings)
- Multiple unused imports (7 total)
- Unused parameters `relationships`, `i`
- Unused variable `term`
- Dead code in LLM response structs (8 fields)

## Desired Behavior
- **Test-wise**: All 88 existing tests must continue passing
- **Behavior-wise**: No functional changes, only warning elimination
- **Functionality-wise**: Zero compiler warnings when running `cargo check --workspace`

## Fix Strategy
1. Prefix unused parameters/variables with `_`
2. Add `#[allow(dead_code)]` to fields needed for deserialization but not accessed
3. Remove genuinely unused imports
4. Remove truly unused struct fields

## Success Criteria
- ✅ `cargo check --workspace` produces zero warnings
- ✅ All 88 tests still pass
- ✅ No functional changes to code behavior
