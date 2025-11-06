# v0.9.1 Release Validation Plan

**Status**: 🚧 IN PROGRESS - DO NOT RELEASE YET
**Date**: 2025-11-06
**Goal**: Validate ALL commands before tagging v0.9.1

---

## Version Audit Results

### Current State (INCONSISTENT ❌)
| Location | Version Claimed | Correct? |
|----------|----------------|----------|
| README.md | v0.10.0 | ❌ NO - Should be v0.9.0 |
| Cargo.toml | v0.9.0 | ✅ YES |
| version-wise-scope-2025.md | v0.9.1 | ⚠️  PLANNED (not released) |

### Required Fixes
- [ ] README.md: Change v0.10.0 → v0.9.0 (current state)
- [ ] Document v0.9.1 as NEXT release (not current)
- [ ] Update PRDv2.md version references
- [ ] Update PT02PRDv1.md version references

---

## Command Testing Matrix

**Principle**: EVERY command in README.md must be tested and validated before release.

### Core Workflow Commands (6 Total)

#### 1. pt01-folder-to-cozodb-streamer (Ingestion)
- **Status**: ✅ TESTED (pt07-integration-test)
- **Result**: 1505 entities created from parseltongue codebase
- **Test Location**: `visualSummary/pt07-integration-test/01-ingestion.log`
- **Validation**: PASS

#### 2. pt02-level00 (Edge Export)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] Exports edges-only JSON
  - [ ] Exports edges-only TOON
  - [ ] Dual-format automatic
  - [ ] Token count matches PRD (5K tokens)
- **Test Location**: `visualSummary/v091-release-validation/02-pt02-level00/`
- **Validation**: NOT STARTED

#### 3. pt02-level01 (Entity Export)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] Exports entities JSON (14 fields)
  - [ ] Exports entities TOON (14 fields)
  - [ ] Dual-format automatic
  - [ ] Token count ~30K (signatures only)
  - [ ] Token count ~500K (with code)
  - [ ] `--include-code 0` and `--include-code 1` both work
- **Test Location**: `visualSummary/v091-release-validation/03-pt02-level01/`
- **Validation**: NOT STARTED

#### 4. pt02-level02 (Type-Aware Export)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] Exports entities JSON (22 fields)
  - [ ] Exports entities TOON (22 fields)
  - [ ] Dual-format automatic
  - [ ] Token count ~60K (signatures only)
  - [ ] Type system fields present (return_type, param_types, etc.)
- **Test Location**: `visualSummary/v091-release-validation/04-pt02-level02/`
- **Validation**: NOT STARTED

#### 5. pt03-llm-to-cozodb-writer (Edit Database)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] EDIT action updates future_code
  - [ ] CREATE action adds new entity
  - [ ] DELETE action marks entity for deletion
  - [ ] Temporal state correctly updated
- **Test Location**: `visualSummary/v091-release-validation/05-pt03-edit/`
- **Validation**: NOT STARTED

#### 6. pt04-syntax-preflight-validator (Validate)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] Validates syntax of future_code
  - [ ] Reports errors for invalid code
  - [ ] Exit code 0 for valid, non-zero for errors
- **Test Location**: `visualSummary/v091-release-validation/06-pt04-validate/`
- **Validation**: NOT STARTED

#### 7. pt05-llm-cozodb-to-diff-writer (Generate Diff)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] Generates CodeDiff.json
  - [ ] EDIT operations have correct line ranges
  - [ ] CREATE operations have file paths
  - [ ] DELETE operations have keys
- **Test Location**: `visualSummary/v091-release-validation/07-pt05-diff/`
- **Validation**: NOT STARTED

#### 8. pt06-cozodb-make-future-code-current (Reset)
- **Status**: ⏳ PENDING
- **What to Test**:
  - [ ] Deletes all entities
  - [ ] Re-runs pt01 ingestion
  - [ ] Resets temporal state
- **Test Location**: `visualSummary/v091-release-validation/08-pt06-reset/`
- **Validation**: NOT STARTED

---

## Optional/Experimental Commands

### pt07-visual-analytics-terminal
- **Status**: ✅ IMPLEMENTED (44 tests passing)
- **Decision**: Mark as EXPERIMENTAL, not blocking v0.9.1
- **Rationale**:
  - Not in core README.md workflow
  - Diagnostic tool, not production workflow
  - Can be released separately
- **Action**:
  - [ ] Add to README.md under "Experimental Tools" section
  - [ ] Document as non-blocking for v0.9.1
  - [ ] Keep tests passing, but don't require workflow integration

---

## Testing Workflow

### For Each Command:

1. **Create Test Subfolder**
   ```bash
   mkdir -p visualSummary/v091-release-validation/0X-command-name
   ```

2. **Write Test Script**
   ```bash
   # Create command.sh with exact command from README.md
   cat > command.sh <<'EOF'
   #!/bin/bash
   set -e

   # Command from README.md
   pt02-level00 --where "ALL" --output edges.json --db rocksdb:parseltongue.db
   EOF
   chmod +x command.sh
   ```

3. **Execute and Capture**
   ```bash
   ./command.sh > output.log 2>&1
   ```

4. **Validate Results**
   - Check exit code (0 = success)
   - Verify output files exist
   - Check file sizes reasonable
   - For TOON: verify both JSON and TOON created
   - Document in `validation.md`

5. **Update Matrix**
   - Mark status: ✅ PASS, ❌ FAIL, ⚠️  PARTIAL
   - Document any issues
   - If FAIL: create issue, fix before release

---

## Release Decision Criteria

### v0.9.1 is READY when:

- ✅ **All 8 core commands tested and passing**
- ✅ **Version numbers consistent across all docs**
- ✅ **TOON dual-format validated for all pt02 levels**
- ✅ **Token counts match PRD estimates (±10%)**
- ✅ **No blockers in issue tracker**

### v0.9.1 is NOT READY if:

- ❌ **Any core command fails**
- ❌ **Version numbers inconsistent**
- ❌ **TOON format broken or missing**
- ❌ **Token counts way off (>20% deviation)**

---

## Timeline Estimate

| Phase | Task | Time | Status |
|-------|------|------|--------|
| 1 | Version audit & fixes | 30 min | ⏳ IN PROGRESS |
| 2 | Test pt01 (already done) | 0 min | ✅ DONE |
| 3 | Test pt02-level00/01/02 | 45 min | ⏳ PENDING |
| 4 | Test pt03/04/05 | 45 min | ⏳ PENDING |
| 5 | Test pt06 | 15 min | ⏳ PENDING |
| 6 | Document pt07 as experimental | 15 min | ⏳ PENDING |
| 7 | Final review & commit | 30 min | ⏳ PENDING |
| **TOTAL** | **End-to-end validation** | **~3 hours** | **10% COMPLETE** |

---

## Next Steps

1. **NOW**: Fix version numbers (README.md, docs)
2. **NEXT**: Test pt02 commands (level00, level01, level02)
3. **THEN**: Test pt03-06 workflow
4. **FINALLY**: Review, commit, tag v0.9.1

**DO NOT TAG v0.9.1 UNTIL ALL CHECKS PASS**

---

**Last Updated**: 2025-11-06
**Updated By**: Claude Code (Sonnet 4.5)
**Status**: 🚧 VALIDATION IN PROGRESS
