# D13: Parseltongue Workflow Patterns

**Created**: 2025-11-01
**Purpose**: Document suggested orchestration patterns for AI IDEs using Parseltongue toolkit
**Context**: Extracted from PRDv1.md - these are orchestrator responsibilities, not Parseltongue features

---

## Pattern 1: Basic Bug Fix Workflow

**Use Case**: Single bug, straightforward fix, confidence high

**Steps**:

```bash
# Step 1: Index codebase (if not already indexed)
folder-to-cozodb-streamer ./src --db .parseltongue/db.cozo

# Step 2: Get context for reasoning
llm-cozodb-to-context-writer --db .parseltongue/db.cozo --output context.json

# (Orchestrator reads context.json, reasons about fix with LLM)

# Step 3: Write proposed changes
llm-to-cozodb-writer --db .parseltongue/db.cozo \
  --entity "rust:fn:add:src_lib_rs:2-4" \
  --action edit \
  --future-code "a + b  // FIXED"

# Step 4: Validate syntax
rust-preflight-code-simulator --db .parseltongue/db.cozo
# Exit code 0 = valid, non-zero = syntax errors

# Step 5: Generate diff
llm-cozodb-to-diff-writer --db .parseltongue/db.cozo --output diff.json

# (Orchestrator reads diff.json, applies changes to files)

# Step 6: Run cargo build && cargo test
# (Orchestrator's responsibility)

# Step 7: Reset state
cozodb-make-future-code-current --db .parseltongue/db.cozo --project ./
```

**Key Points**:
- Orchestrator drives the workflow (not Parseltongue)
- Orchestrator handles file I/O (Parseltongue only generates diff.json)
- Orchestrator runs build/test validation
- Orchestrator manages undo/rollback

---

## Pattern 2: Iterative Refinement Workflow

**Use Case**: Complex changes requiring multiple reasoning cycles, LLM needs to refine approach

**Steps**:

```bash
# Cycle 1: Initial reasoning
llm-cozodb-to-context-writer --db db.cozo --output context.json
# (LLM reasons, proposes changes)
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "..."

# Cycle 2: Re-read context with changes
llm-cozodb-to-context-writer --db db.cozo --filter changed --output context_changed.json
# (LLM re-reasons with new context, refines changes)
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "... (refined)"

# Cycle 3: Re-read again
llm-cozodb-to-context-writer --db db.cozo --filter changed --output context_changed.json
# (LLM validates, confirms changes look good)

# Repeat until LLM is confident (orchestrator decides when to stop)

# Then validate and apply
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff.json
# (Orchestrator applies, builds, tests)
```

**Confidence Assessment**:
- Orchestrator's responsibility (not Parseltongue's)
- Could be based on: LLM self-assessment, number of iterations, change stability, etc.
- No built-in confidence mechanism in Parseltongue

**When to Use**:
- Refactoring across multiple files
- Logic bugs requiring deep reasoning
- Changes with unclear scope initially

---

## Pattern 3: Batch Processing Workflow

**Use Case**: Multiple independent changes that can be validated together

**Steps**:

```bash
# Write all changes first (no validation between)
llm-to-cozodb-writer --db db.cozo --entity "entity1" --action edit --future-code "..."
llm-to-cozodb-writer --db db.cozo --entity "entity2" --action edit --future-code "..."
llm-to-cozodb-writer --db db.cozo --entity "entity3" --action delete
llm-to-cozodb-writer --db db.cozo --entity "entity4" --action create --future-code "..."

# Single validation pass
rust-preflight-code-simulator --db db.cozo

# Single diff generation
llm-cozodb-to-diff-writer --db db.cozo --output diff.json

# Orchestrator applies all changes atomically
# (Either all succeed or none)
```

**When to Use**:
- Related changes across multiple entities
- Renaming (delete old, create new)
- Parallel bug fixes
- Code cleanup tasks

**Atomicity**: Orchestrator's responsibility (Parseltongue doesn't enforce)

---

## Pattern 4: Test-Driven Development Workflow

**Use Case**: Write failing tests first, then implement to make them pass

**Steps**:

```bash
# Step 1: Write failing test
llm-to-cozodb-writer --db db.cozo \
  --entity "rust:fn:test_feature:src_lib_rs:100-110" \
  --action create \
  --future-code "#[test] fn test_feature() { assert_eq!(feature(), 42); }"

# Step 2: Generate diff, apply test
llm-cozodb-to-diff-writer --db db.cozo --output diff.json
# (Orchestrator applies)

# Step 3: Run cargo test (should fail)
# (Orchestrator verifies test fails)

# Step 4: Implement feature
llm-to-cozodb-writer --db db.cozo \
  --entity "rust:fn:feature:src_lib_rs:50-60" \
  --action create \
  --future-code "pub fn feature() -> i32 { 42 }"

# Step 5: Validate, apply, test (should pass)
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff.json
# (Orchestrator applies, cargo test passes)

# Step 6: Reset
cozodb-make-future-code-current --db db.cozo --project ./
```

**TDD Classification**: Parseltongue tracks this in `TDD_Classification` field:
- `TEST_IMPLEMENTATION`: Test code
- `CODE_IMPLEMENTATION`: Production code

---

## Pattern 5: Progressive Enhancement Workflow

**Use Case**: Incrementally add functionality, validating at each step

**Steps**:

```bash
# Enhancement 1: Add basic functionality
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "..."
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff1.json
# (Apply, build, test)

# Enhancement 2: Add error handling (build on Enhancement 1)
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "... (with error handling)"
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff2.json
# (Apply, build, test)

# Enhancement 3: Add documentation
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "... (with docs)"
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff3.json
# (Apply, build, test)

# Reset after all enhancements
cozodb-make-future-code-current --db db.cozo --project ./
```

**When to Use**:
- Feature additions with unclear full scope
- Learning mode (explore codebase while making changes)
- Risk-averse changes (validate each small step)

---

## Pattern 6: Rollback-Safe Exploration

**Use Case**: Experiment with changes without committing to them

**Steps**:

```bash
# Experiment with change A
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "... (approach A)"
llm-cozodb-to-context-writer --db db.cozo --filter changed --output context_A.json
# (LLM evaluates approach A)

# Don't like it? Try approach B (overwrites future_code)
llm-to-cozodb-writer --db db.cozo --entity "..." --action edit --future-code "... (approach B)"
llm-cozodb-to-context-writer --db db.cozo --filter changed --output context_B.json
# (LLM evaluates approach B)

# Like approach B? Apply it
rust-preflight-code-simulator --db db.cozo
llm-cozodb-to-diff-writer --db db.cozo --output diff.json
# (Apply)

# OR: Don't like either? Reset without applying
cozodb-make-future-code-current --db db.cozo --project ./
```

**Key Insight**: CozoDB state is ephemeral until you apply changes to files. Safe to experiment.

---

## Orchestrator Responsibilities Summary

**The AI IDE / orchestrator MUST handle**:

1. **File I/O**:
   - Reading `diff.json`
   - Applying changes to source files
   - Creating backups (if desired)

2. **Build/Test Validation**:
   - Running `cargo build`
   - Running `cargo test`
   - Parsing build errors
   - Determining if validation passed

3. **Undo/Rollback**:
   - Reverting file changes if validation fails
   - Managing git state
   - User confirmation before destructive operations

4. **Confidence Scoring**:
   - Deciding when to stop iterative refinement
   - LLM self-assessment parsing
   - Change stability detection

5. **User Interaction**:
   - Prompting user for confirmation
   - Showing progress (indexing, validating, etc.)
   - Displaying errors clearly
   - Asking clarifying questions

6. **Error Recovery**:
   - Handling tool failures gracefully
   - Retry logic for transient failures
   - Clear error messages to user

7. **State Management**:
   - When to re-index (codebase changed externally)
   - When to reset state (after successful changes)
   - Managing database file location

---

## Parseltongue Provides

**Infrastructure only**:

1. **Indexing**: Codebase → Database (Tool 1)
2. **Temporal State**: Safe change tracking (Tool 2)
3. **Context Generation**: Minimal LLM context (Tool 3)
4. **Syntax Validation**: Fast pre-flight checks (Tool 4)
5. **Diff Generation**: Structured change format (Tool 5)
6. **State Reset**: Fresh start after changes (Tool 6)

**NOT provided**:
- LLM reasoning
- File writing
- Build/test execution
- User interaction
- Orchestration logic
- Confidence assessment

---

## Example: Shell Script Orchestrator

**Minimal orchestrator demonstrating workflow**:

```bash
#!/bin/bash
# minimal_orchestrator.sh - Demonstrates Parseltongue workflow

set -e  # Exit on error

DB=".parseltongue/db.cozo"
BUG_DESCRIPTION="$1"

if [ -z "$BUG_DESCRIPTION" ]; then
  echo "Usage: $0 <bug_description>"
  exit 1
fi

# Step 1: Index codebase
echo "🔍 Indexing codebase..."
folder-to-cozodb-streamer ./src --db "$DB" --verbose

# Step 2: Get context
echo "📋 Generating context..."
llm-cozodb-to-context-writer --db "$DB" --output context.json

# Step 3: LLM reasoning (placeholder - real orchestrator would call LLM)
echo "🤖 LLM reasoning about: $BUG_DESCRIPTION"
echo "📄 Context available in: context.json"
echo ""
echo "⚠️  MANUAL STEP: Call LLM with context.json + bug description"
echo "⚠️  Then manually call llm-to-cozodb-writer with proposed changes"
echo ""
read -p "Press Enter after writing changes to database..."

# Step 4: Validate syntax
echo "✅ Validating syntax..."
if ! rust-preflight-code-simulator --db "$DB"; then
  echo "❌ Syntax validation failed"
  exit 1
fi

# Step 5: Generate diff
echo "📝 Generating diff..."
llm-cozodb-to-diff-writer --db "$DB" --output diff.json

# Step 6: Show diff (don't apply yet)
echo "📄 Proposed changes:"
cat diff.json
echo ""
read -p "Apply changes? (y/n): " confirm
if [ "$confirm" != "y" ]; then
  echo "❌ Cancelled"
  exit 1
fi

# Step 7: Apply changes (placeholder - real orchestrator would parse diff.json)
echo "⚠️  MANUAL STEP: Apply changes from diff.json to files"
read -p "Press Enter after applying changes..."

# Step 8: Build & test
echo "🔨 Building..."
if ! cargo build; then
  echo "❌ Build failed"
  exit 1
fi

echo "🧪 Testing..."
if ! cargo test; then
  echo "❌ Tests failed"
  exit 1
fi

# Step 9: Reset state
echo "🔄 Resetting database state..."
cozodb-make-future-code-current --db "$DB" --project ./

echo "✅ Bug fix complete!"
```

**Limitations**:
- Manual LLM calling (no LLM API integration)
- Manual diff application (no file writing logic)
- No error recovery
- No iterative refinement

**Purpose**: Demonstrates tool orchestration, not production-ready

---

## Integration Examples

### Claude Code Hook (Pseudocode)

```python
# .claude/hooks/parseltongue_fix.py

def fix_bug(bug_description: str) -> str:
    db = ".parseltongue/db.cozo"

    # Index if needed
    if not os.path.exists(db):
        run("folder-to-cozodb-streamer ./src --db " + db)

    # Get context
    run("llm-cozodb-to-context-writer --db " + db + " --output context.json")
    context = read_json("context.json")

    # LLM reasoning
    response = claude_code.ask_llm(
        f"Fix this bug: {bug_description}\n\nContext: {json.dumps(context)}"
    )

    # Parse LLM response, write to database
    for change in parse_response(response):
        run(f"llm-to-cozodb-writer --db {db} "
            f"--entity {change.key} --action {change.action} --future-code '{change.code}'")

    # Validate
    if run("rust-preflight-code-simulator --db " + db, check=False) != 0:
        return "Syntax validation failed"

    # Generate diff
    run("llm-cozodb-to-diff-writer --db " + db + " --output diff.json")
    diff = read_json("diff.json")

    # Apply changes
    apply_diff(diff)

    # Build & test
    if not build_and_test():
        rollback(diff)
        return "Build/test failed"

    # Reset
    run("cozodb-make-future-code-current --db " + db + " --project ./")

    return "Success"
```

### Cursor Integration (Pseudocode)

```javascript
// Cursor extension: parseltongue.js

async function fixBugWithParseltongue(bugDescription) {
  const db = ".parseltongue/db.cozo";

  // Index codebase
  await exec(`folder-to-cozodb-streamer ./src --db ${db}`);

  // Get context
  await exec(`llm-cozodb-to-context-writer --db ${db} --output context.json`);
  const context = await readJSON("context.json");

  // Call LLM (Cursor's built-in)
  const response = await cursor.askAI(
    `Fix: ${bugDescription}\n\nContext: ${JSON.stringify(context)}`
  );

  // Parse and write changes
  const changes = parseChanges(response);
  for (const change of changes) {
    await exec(
      `llm-to-cozodb-writer --db ${db} ` +
      `--entity ${change.key} --action ${change.action} --future-code '${change.code}'`
    );
  }

  // Validate, diff, apply
  await exec(`rust-preflight-code-simulator --db ${db}`);
  await exec(`llm-cozodb-to-diff-writer --db ${db} --output diff.json`);
  const diff = await readJSON("diff.json");
  await applyDiff(diff);

  // Build, test, reset
  await exec("cargo build && cargo test");
  await exec(`cozodb-make-future-code-current --db ${db} --project ./`);
}
```

---

## Common Pitfalls

### Pitfall 1: Not Resetting State

**Problem**: Forgetting to run Tool 6 after applying changes leaves database out of sync with files.

**Symptom**: Next workflow iteration uses stale `future_code` instead of actual file content.

**Solution**: Always run `cozodb-make-future-code-current` after successful file application.

### Pitfall 2: Applying Changes Before Validation

**Problem**: Skipping Tool 4 syntax validation and applying diff directly.

**Symptom**: Files end up with syntax errors, cargo build fails.

**Solution**: Always validate before applying (Tool 4 is cheap, ~20ms).

### Pitfall 3: Not Handling Build Failures

**Problem**: Applying changes, running cargo build, it fails, but not rolling back.

**Symptom**: Codebase left in broken state.

**Solution**: Orchestrator must handle rollback on build failure.

### Pitfall 4: Overwriting Database Mid-Workflow

**Problem**: Re-indexing (Tool 1) while `future_code` is set, losing proposed changes.

**Symptom**: LLM's proposed changes disappear.

**Solution**: Only re-index after reset (Tool 6) or before starting new workflow.

### Pitfall 5: Concurrent Access to Database

**Problem**: Multiple workflows accessing same database file simultaneously.

**Symptom**: Database corruption or lock errors.

**Solution**: Use separate database files per workflow, or implement locking in orchestrator.

---

**End of Workflow Patterns Document**

*These patterns are suggestions for orchestrator developers. Parseltongue toolkit is unopinionated about which pattern to use.*
