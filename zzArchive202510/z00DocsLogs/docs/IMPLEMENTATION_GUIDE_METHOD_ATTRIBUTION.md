# Implementation Guide: Method-Level Attribution Fix

## Problem Statement

Calls from impl methods are currently attributed to the impl block instead of the specific method.

**Current behavior:**
```rust
impl Config {
    fn new() -> Self {
        Self { x: helper() }
    }
}

// Dependency: rust:impl:Config -> rust:fn:helper
```

**Desired behavior:**
```rust
// Dependency: rust:method:new -> rust:fn:helper
```

---

## Root Cause Analysis

### 1. Entity Extraction (entity_queries/rust.scm)

Current query **only** extracts impl blocks:
```scheme
; Impl blocks
(impl_item
  type: (type_identifier) @name) @definition.impl
```

Methods are **not** extracted as separate entities.

### 2. Dependency Attribution (query_extractor.rs)

```rust
fn find_containing_entity<'a>(
    &self,
    node: tree_sitter::Node<'_>,
    entities: &'a [ParsedEntity],
) -> Option<&'a ParsedEntity> {
    let node_line = node.start_position().row + 1;

    // Returns FIRST entity that contains the line
    entities.iter().find(|e| {
        e.line_range.0 <= node_line && node_line <= e.line_range.1
    })
}
```

**Problem**: Returns first match, which is the broad impl block (lines 6-18), not the specific method (lines 8-12).

---

## Solution: Two-Part Fix

### Part 1: Extract Methods as Entities

**File**: `/Users/amuldotexe/Projects/parseltongue/entity_queries/rust.scm`

**Add this query pattern:**

```scheme
; Methods within impl blocks (NEW)
(impl_item
  body: (declaration_list
    (function_item
      name: (identifier) @name) @definition.method))
```

**What this does**:
- Matches function_item nodes inside impl_item bodies
- Creates EntityType::Method for each method
- Gives each method its own line range (smaller than impl block)

**Tree-sitter AST structure**:
```
impl_item [lines 6-18]
  type: Config
  body: declaration_list
    function_item [lines 8-12]  ← This is what we capture
      name: "new"
      parameters: ...
      body: ...
```

### Part 2: Improve Containment Logic

**File**: `/Users/amuldotexe/Projects/parseltongue/crates/parseltongue-core/src/query_extractor.rs`

**Replace** `find_containing_entity()` (line 532-543) with:

```rust
/// Find the entity that contains a given AST node
///
/// Prefers the most specific entity (smallest line range) when multiple
/// entities contain the node. This ensures method calls are attributed to
/// the method, not the enclosing impl block.
fn find_containing_entity<'a>(
    &self,
    node: tree_sitter::Node<'_>,
    entities: &'a [ParsedEntity],
) -> Option<&'a ParsedEntity> {
    let node_line = node.start_position().row + 1;

    // Find all entities that contain this line
    let mut candidates: Vec<&ParsedEntity> = entities
        .iter()
        .filter(|e| e.line_range.0 <= node_line && node_line <= e.line_range.1)
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // Sort by specificity
    candidates.sort_by(|a, b| {
        // Primary: Prefer smaller line ranges (more specific)
        let a_range = a.line_range.1 - a.line_range.0;
        let b_range = b.line_range.1 - b.line_range.0;

        match a_range.cmp(&b_range) {
            std::cmp::Ordering::Equal => {
                // Secondary: Prefer methods/functions over impl blocks
                match (&a.entity_type, &b.entity_type) {
                    (EntityType::Method, EntityType::Impl) => std::cmp::Ordering::Less,
                    (EntityType::Impl, EntityType::Method) => std::cmp::Ordering::Greater,
                    (EntityType::Function, EntityType::Impl) => std::cmp::Ordering::Less,
                    (EntityType::Impl, EntityType::Function) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            },
            other => other,
        }
    });

    // Return most specific entity
    Some(candidates[0])
}
```

**What this does**:
1. Finds ALL entities containing the line (not just first)
2. Sorts by specificity: smaller range = more specific
3. Tie-breaker: prefer methods/functions over impl blocks
4. Returns the most specific match

---

## Example Walkthrough

### Before Fix

**Code**:
```rust
impl Config {              // Line 6 - Entity 1: impl:Config (lines 6-18)
    fn new() -> Self {     // Line 8 - NOT AN ENTITY (not extracted)
        Self {
            x: helper()    // Line 10 - Call site
        }
    }
}
```

**find_containing_entity(line 10)**:
- Checks: impl:Config (lines 6-18) → ✅ contains line 10
- Returns: impl:Config
- **Result**: `impl:Config -> helper`

### After Fix

**Code**:
```rust
impl Config {              // Line 6 - Entity 1: impl:Config (lines 6-18)
    fn new() -> Self {     // Line 8 - Entity 2: method:new (lines 8-12)
        Self {
            x: helper()    // Line 10 - Call site
        }
    }
}
```

**find_containing_entity(line 10)**:
- Candidates:
  - impl:Config (lines 6-18, range=12) → ✅ contains line 10
  - method:new (lines 8-12, range=4) → ✅ contains line 10
- Sorting:
  - method:new (range=4) < impl:Config (range=12)
- Returns: method:new
- **Result**: `method:new -> helper`

---

## Testing the Fix

### Test Case 1: Basic Method Call

```rust
impl Calculator {
    fn compute(&self) -> i32 {
        self.helper()
    }

    fn helper(&self) -> i32 {
        42
    }
}
```

**Expected**:
- `method:compute -> method:helper` (NOT `impl:Calculator -> helper`)

### Test Case 2: Constructor Pattern

```rust
impl Config {
    fn new() -> Self {
        Self {
            settings: create_defaults(),
        }
    }
}
```

**Expected**:
- `method:new -> create_defaults` (NOT `impl:Config -> create_defaults`)

### Test Case 3: Multiple Methods in Same Impl

```rust
impl Processor {
    fn step1(&self) {
        validate()
    }

    fn step2(&self) {
        process()
    }

    fn step3(&self) {
        finalize()
    }
}
```

**Expected**:
- `method:step1 -> validate`
- `method:step2 -> process`
- `method:step3 -> finalize`

All attributed correctly to their respective methods.

---

## Verification Steps

### 1. Run Existing Tests

```bash
cd crates/pt01-folder-to-cozodb-streamer
cargo test test_query_based_matches_manual_extraction_quality -- --nocapture
```

**Look for**:
```
=== EXTRACTED DEPENDENCIES ===
...
5. rust:method:new:... -> rust:fn:create_defaults:... (Calls)
```

Should say `method:new` instead of `impl:Config`.

### 2. Run New Complex Pattern Tests

```bash
cargo test test_nested_call_in_struct_construction -- --nocapture
```

**Look for**:
```
Attribution: Some("rust:method:new:...")
```

### 3. Verify No Regressions

```bash
cargo test tdd_dependency_extraction_test
```

All existing tests should still pass.

---

## Edge Cases Handled

### 1. Nested Impl Blocks (Trait Impl)

```rust
impl MyTrait for MyStruct {
    fn trait_method(&self) {
        helper()
    }
}
```

**Behavior**: Will prefer `method:trait_method` over `impl MyTrait for MyStruct`.

### 2. Free Functions (No Impl)

```rust
fn standalone() {
    helper()
}
```

**Behavior**: Attributed to `fn:standalone` (no change, works as before).

### 3. Nested Functions

```rust
fn outer() {
    fn inner() {
        helper()
    }
}
```

**Behavior**: Will prefer `fn:inner` (smaller range) over `fn:outer`.

### 4. Multiple Nested Levels

```rust
impl Outer {
    fn method(&self) {
        let closure = || {
            helper()
        };
    }
}
```

**Behavior**: Will prefer `method` over `impl`, but closures are still attributed to containing function (closures not extracted as entities).

---

## Performance Impact

**Minimal** - O(n log n) sort instead of O(n) linear search:

- Before: `entities.iter().find()` → O(n)
- After: `candidates.sort_by()` → O(k log k), where k = number of containing entities
- Typical case: k ≈ 1-2 (method + impl)
- Worst case: k ≈ 5 (deeply nested)

**Estimated overhead**: <1% (sorting 2-5 items is negligible).

---

## Rollout Plan

### Phase 1: Implementation (30 minutes)

1. Add method extraction query to `entity_queries/rust.scm`
2. Update `find_containing_entity()` in `query_extractor.rs`
3. Run tests to verify no regressions

### Phase 2: Validation (15 minutes)

1. Run complex pattern tests
2. Check test output shows `method:` instead of `impl:`
3. Verify all existing tests pass

### Phase 3: Documentation (15 minutes)

1. Update CHANGELOG with fix
2. Add note to README about method-level attribution
3. Update any examples that reference impl attribution

**Total time**: ~1 hour

---

## Alternative Approaches Considered

### Alternative 1: Keep Only Impl Extraction, Fix find_containing_entity

**Approach**: Don't extract methods, just make `find_containing_entity()` smarter.

**Problem**: Without method entities, there's nothing to prefer over impl blocks. The line-range heuristic fails because we don't know where methods are.

**Verdict**: ❌ Not viable

### Alternative 2: Extract Methods AND Impl Blocks, Use Hierarchy

**Approach**: Build parent-child relationship (method → impl).

**Problem**: Over-engineering. Line-range containment is sufficient and simpler.

**Verdict**: ❌ Unnecessary complexity

### Alternative 3: Current Solution (Extract Methods + Smart Containment)

**Why best**:
- ✅ Simple: Two small changes
- ✅ Maintainable: No new data structures
- ✅ Extensible: Works for nested functions too
- ✅ Performant: Minimal overhead

**Verdict**: ✅ Optimal solution

---

## Summary

**Problem**: Calls attributed to impl blocks instead of methods.

**Root cause**: Methods not extracted as entities + naive containment logic.

**Solution**:
1. Extract methods as entities (15 lines of query)
2. Improve containment to prefer smallest range (30 lines of Rust)

**Effort**: ~1 hour

**Impact**: Fixes main limitation, enables precise method-level dependency tracking.

**Risk**: Low - backwards compatible, no breaking changes.
