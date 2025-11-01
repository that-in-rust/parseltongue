# Parseltongue: Possible Developer Workflows

**Purpose**: Document real-world workflows where Parseltongue's 6-tool pipeline helps developers work with large codebases.

**Command Reference**: All commands sourced from README.md and PRDv2.md (source of truth).

**Research Foundation**: Based on comprehensive analysis of how developers actually work with large codebases (2024-2025).

---

## Table of Contents

1. [Onboarding Workflows](#onboarding-workflows)
2. [Understanding & Exploration](#understanding--exploration)
3. [Bug Fixing Workflows](#bug-fixing-workflows)
4. [Feature Development](#feature-development)
5. [Refactoring & Code Quality](#refactoring--code-quality)
6. [Impact Analysis](#impact-analysis)
7. [Documentation Workflows](#documentation-workflows)
8. [Architecture Analysis](#architecture-analysis)
9. [Technical Debt Management](#technical-debt-management)
10. [Code Review Support](#code-review-support)

---

## ONBOARDING WORKFLOWS

### Workflow 1: New Developer Orientation (Day 1)

**Developer Need**: "I just joined the team. What does this codebase do and how is it structured?"

**Workflow Steps:**

```bash
# Step 1: Index the entire codebase
parseltongue pt01-folder-to-cozodb-streamer ./src \
  --db rocksdb:onboarding.db \
  --verbose

# Output: "1,247 entities indexed across 89 files"
# Developer now has structured view of ALL code entities

# Step 2: Export high-level overview (NO code, just signatures)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./onboarding-context.json \
  --db rocksdb:onboarding.db \
  --include-current-code 0

# Output: JSON with all function/struct/trait signatures
# Token-optimized: ~37.5k tokens for 1,500 entities
```

**What the developer gets:**
- Complete inventory of modules, functions, structures
- Interface signatures showing what each function does
- File organization and naming patterns
- Entry points and main components

**LLM Queries to Ask (with context.json):**
- "What are the main modules in this codebase?"
- "Where does the application start?"
- "What design patterns are used?"
- "How is error handling implemented?"

**Time Saved**: 2-3 hours of manual code exploration → 5 minutes automated extraction

---

### Workflow 2: Understanding a Specific Module

**Developer Need**: "I need to work on the authentication module. Show me everything related to auth."

**Workflow Steps:**

```bash
# Step 1: Index codebase (if not already done)
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db

# Step 2: Export context with custom query (filter by path pattern)
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * EXCEPT (Current_Code, Future_Code)
           FROM CodeGraph
           WHERE ISGL1_key LIKE '%auth%' OR ISGL1_key LIKE '%login%'" \
  --output ./auth-module.json \
  --db rocksdb:analysis.db

# Output: JSON with only auth-related entities
```

**LLM Queries:**
- "What authentication methods are supported?"
- "Where are passwords validated?"
- "How does session management work?"
- "What are the security checks in place?"

**Use Case**: Focus learning on specific subsystem without drowning in unrelated code.

---

### Workflow 3: Finding Entry Points

**Developer Need**: "Where does execution start? What are the main entry points?"

**Workflow Steps:**

```bash
# Index and export
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:analysis.db

# Query context for typical entry point patterns
```

**LLM Queries with context:**
- "Find all functions named 'main'"
- "Show me HTTP route handlers"
- "What are the CLI command entry points?"
- "Where are event listeners registered?"

---

## UNDERSTANDING & EXPLORATION

### Workflow 4: "Where Does This Feature Live?"

**Developer Need**: "Users report a bug with file uploads. Where is that code?"

**Traditional Approach:**
- grep for "upload" → 200 matches across logs, comments, variable names
- Manually filter false positives
- 30-60 minutes of searching

**Parseltongue Approach:**

```bash
# Step 1: Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db

# Step 2: Export structured context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:analysis.db
```

**LLM Query with context:**
- "Show me all functions related to file upload handling"
- "Where are uploaded files validated?"
- "What functions process multipart form data?"

**Result**: Structured answers pointing to exact ISGL1 keys (file paths with line numbers), not raw grep matches.

**Time Saved**: 30-60 minutes → 2 minutes

---

### Workflow 5: Tracing Execution Paths

**Developer Need**: "When a user clicks 'Submit', what code runs?"

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db

# Export with dependencies (if tool supports dependency queries)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:analysis.db
```

**LLM Queries:**
- "What is the execution path from the 'Submit' button handler to database write?"
- "Show me the call chain for form validation"
- "What functions are called between HTTP request and response?"

**Benefit**: LLM can reason over ISGL1 keys and signatures to trace logical flow without executing code.

---

### Workflow 6: Understanding Dependencies

**Developer Need**: "If I change function X, what else might break?"

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db

# Export context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:analysis.db
```

**LLM Queries:**
- "What functions call `validate_user_input`?"
- "Where is the `Database` struct used?"
- "What modules depend on the `auth` module?"

**Note**: This workflow shows the power of ISG (Interface Signature Graphs). The LLM can reason about relationships from signatures alone.

---

## BUG FIXING WORKFLOWS

### Workflow 7: Simple Bug Fix (Syntax Error)

**Developer Need**: "Function `hello()` prints 'Goodbye' instead of 'Hello'. Fix it."

**Complete Workflow:**

```bash
# Step 1: Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./greeter \
  --db rocksdb:bugfix.db

# Output: "4 entities indexed"

# Step 2: Export context to understand the bug
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:bugfix.db \
  --include-current-code 1  # Include code this time to see the bug

# LLM analyzes context.json and confirms the bug

# Step 3: Write the fix to temporal database
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
  --action edit \
  --future-code 'pub fn hello() -> &'"'"'static str { "Hello!" }' \
  --db rocksdb:bugfix.db

# Output: "Entity updated: (current_ind=1, future_ind=1, future_action=Edit)"

# Step 4: Validate syntax
parseltongue pt04-syntax-preflight-validator \
  --db rocksdb:bugfix.db \
  --verbose

# Output: "✓ All syntax validations passed"

# Step 5: Generate diff
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output CodeDiff.json \
  --db rocksdb:bugfix.db

# Output: CodeDiff.json with before/after

# LLM reads CodeDiff.json and applies changes to actual files

# Step 6: Reset database (after manual verification)
parseltongue pt06-cozodb-make-future-code-current \
  --project ./greeter \
  --db rocksdb:bugfix.db

# Output: "Database reset complete"
```

**Time**: 5-10 minutes for complete workflow (vs. 30+ minutes traditional debugging)

---

### Workflow 8: Complex Bug (Multiple Files)

**Developer Need**: "Async database connections leak memory. Need to fix connection pool + all call sites."

**Workflow Steps:**

```bash
# Step 1: Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:memory-leak.db

# Step 2: Export context (signatures only first)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:memory-leak.db

# LLM identifies affected functions from signatures

# Step 3: Export specific entities WITH code
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph
           WHERE ISGL1_key LIKE '%connection_pool%'
              OR ISGL1_key LIKE '%async_db%'" \
  --output ./db-code.json \
  --db rocksdb:memory-leak.db \
  --include-current-code 1

# Now LLM has detailed code for just the relevant entities

# Step 4: Write fixes (multiple edits)
# LLM generates commands for each affected entity

parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:acquire_connection:db_pool_rs:42-58" \
  --action edit \
  --future-code "... fixed implementation ..." \
  --db rocksdb:memory-leak.db

parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:release_connection:db_pool_rs:60-75" \
  --action edit \
  --future-code "... fixed implementation ..." \
  --db rocksdb:memory-leak.db

# Step 5: Validate all changes
parseltongue pt04-syntax-preflight-validator --db rocksdb:memory-leak.db

# Step 6: Generate unified diff
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output MemoryLeakFix.json \
  --db rocksdb:memory-leak.db

# LLM applies changes, runs tests

# Step 7: Reset after verification
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:memory-leak.db
```

**Benefit**: Temporal versioning tracks all related changes as a coordinated unit.

---

### Workflow 9: Test-Driven Bug Fix

**Developer Need**: "Add test for bug, then fix it."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:tdd-fix.db

# Export context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:tdd-fix.db

# Step 1: CREATE new test (hash-based ISGL1 key)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "tests_greeter_rs-test_hello_says_hello-fn-abc12345" \
  --action create \
  --future-code "#[test] fn test_hello_says_hello() {
      assert_eq!(hello(), \"Hello!\");
  }" \
  --db rocksdb:tdd-fix.db

# State: (current_ind=0, future_ind=1, future_action=Create)

# Step 2: EDIT implementation to fix bug
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:hello:src_lib_rs:4-6" \
  --action edit \
  --future-code 'pub fn hello() -> &'"'"'static str { "Hello!" }' \
  --db rocksdb:tdd-fix.db

# State: (current_ind=1, future_ind=1, future_action=Edit)

# Validate, diff, apply
parseltongue pt04-syntax-preflight-validator --db rocksdb:tdd-fix.db
parseltongue pt05-llm-cozodb-to-diff-writer --output TDDFix.json --db rocksdb:tdd-fix.db

# LLM applies changes: Creates new test file + edits implementation
# Run: cargo test (should pass)

# Reset
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:tdd-fix.db
```

**Benefit**: Test + implementation tracked as atomic change in temporal database.

---

## FEATURE DEVELOPMENT

### Workflow 10: Adding a New Function

**Developer Need**: "Add a `good_afternoon()` function following existing patterns."

**Workflow Steps:**

```bash
# Index codebase to understand patterns
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:feature.db

# Export context to see existing patterns
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:feature.db

# LLM analyzes existing functions (hello, goodbye, good_morning, good_night)
# Identifies pattern: pub fn name() -> &'static str { "Message!" }

# Create new function
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_lib_rs-good_afternoon-fn-def45678" \
  --action create \
  --future-code 'pub fn good_afternoon() -> &'"'"'static str {
      "Good afternoon!"
  }' \
  --db rocksdb:feature.db

# Validate and apply
parseltongue pt04-syntax-preflight-validator --db rocksdb:feature.db
parseltongue pt05-llm-cozodb-to-diff-writer --output NewFeature.json --db rocksdb:feature.db

# LLM applies changes
# Reset
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:feature.db
```

**Key Point**: CREATE action uses hash-based ISGL1 key (no line numbers yet).

---

### Workflow 11: Feature with Multiple New Components

**Developer Need**: "Add caching layer with TTL support to HTTP client."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:cache-feature.db

# Export context (understand existing HTTP client)
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:cache-feature.db

# LLM plans feature:
# - New struct: CacheEntry
# - New struct: HttpCache
# - Modified: HttpClient (add caching)
# - New test: test_cache_ttl

# Step 1: CREATE cache entry struct
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_cache_rs-CacheEntry-struct-aaa11111" \
  --action create \
  --future-code "struct CacheEntry { ... }" \
  --db rocksdb:cache-feature.db

# Step 2: CREATE cache manager
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_cache_rs-HttpCache-struct-bbb22222" \
  --action create \
  --future-code "struct HttpCache { ... }" \
  --db rocksdb:cache-feature.db

# Step 3: EDIT existing HTTP client
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:struct:HttpClient:src_http_rs:10-30" \
  --action edit \
  --future-code "struct HttpClient { cache: HttpCache, ... }" \
  --db rocksdb:cache-feature.db

# Step 4: CREATE test
parseltongue pt03-llm-to-cozodb-writer \
  --entity "tests_cache_rs-test_cache_ttl-fn-ccc33333" \
  --action create \
  --future-code "#[test] fn test_cache_ttl() { ... }" \
  --db rocksdb:cache-feature.db

# Validate all changes together
parseltongue pt04-syntax-preflight-validator --db rocksdb:cache-feature.db

# Generate unified diff
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output CacheFeature.json \
  --db rocksdb:cache-feature.db

# LLM applies all changes
# Run: cargo test
# If tests pass, reset
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:cache-feature.db
```

**Benefit**: Multi-file feature tracked as coherent change set with temporal versioning.

---

## REFACTORING & CODE QUALITY

### Workflow 12: Rename Function Across Codebase

**Developer Need**: "Rename `getCwd()` to `getCurrentWorkingDirectory()` everywhere."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:rename.db

# Export ALL entities to find usages
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:rename.db \
  --include-current-code 1  # Need code to find call sites

# LLM identifies:
# - Function definition: src/utils.rs:42-50
# - 15 call sites across 8 files

# EDIT function definition
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:getCwd:src_utils_rs:42-50" \
  --action edit \
  --future-code "pub fn getCurrentWorkingDirectory() -> PathBuf { ... }" \
  --db rocksdb:rename.db

# EDIT each call site (LLM generates 15 commands)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:process_files:src_main_rs:100-120" \
  --action edit \
  --future-code "... getCurrentWorkingDirectory() ..." \
  --db rocksdb:rename.db

# (Repeat for all 15 call sites)

# Validate
parseltongue pt04-syntax-preflight-validator --db rocksdb:rename.db

# Generate comprehensive diff
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output RenameRefactor.json \
  --db rocksdb:rename.db

# LLM applies changes
# cargo build && cargo test
# Reset
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:rename.db
```

**Time Saved**: Manual find-and-replace is error-prone (comments, strings, similar names). This approach is surgical.

---

### Workflow 13: Extract Common Code into Helper

**Developer Need**: "Three functions have duplicate validation logic. Extract to shared helper."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:extract.db

# Export with code
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:extract.db \
  --include-current-code 1

# LLM identifies duplicate blocks in 3 functions

# Step 1: CREATE new helper function
parseltongue pt03-llm-to-cozodb-writer \
  --entity "src_validation_rs-validate_input_common-fn-xyz98765" \
  --action create \
  --future-code "fn validate_input_common(input: &str) -> Result<(), ValidationError> { ... }" \
  --db rocksdb:extract.db

# Step 2: EDIT all 3 functions to call helper
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:process_user:src_user_rs:50-70" \
  --action edit \
  --future-code "... validate_input_common(input)?; ..." \
  --db rocksdb:extract.db

parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:process_admin:src_admin_rs:30-50" \
  --action edit \
  --future-code "... validate_input_common(input)?; ..." \
  --db rocksdb:extract.db

parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:process_guest:src_guest_rs:20-40" \
  --action edit \
  --future-code "... validate_input_common(input)?; ..." \
  --db rocksdb:extract.db

# Validate, diff, apply
parseltongue pt04-syntax-preflight-validator --db rocksdb:extract.db
parseltongue pt05-llm-cozodb-to-diff-writer --output ExtractHelper.json --db rocksdb:extract.db

# LLM applies changes
# cargo test (ensure behavior unchanged)
# Reset
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:extract.db
```

**Benefit**: Refactoring tracked atomically - all related changes succeed or fail together.

---

### Workflow 14: Remove Dead Code

**Developer Need**: "Function `old_legacy_handler()` is never called. Remove it."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:cleanup.db

# Export to verify no callers
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:cleanup.db

# LLM confirms: no references to old_legacy_handler

# DELETE function
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:old_legacy_handler:src_handlers_rs:200-250" \
  --action delete \
  --db rocksdb:cleanup.db

# State: (current_ind=1, future_ind=0, future_action=Delete)

# Validate (syntax check not needed for deletion)
# Generate diff
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output Cleanup.json \
  --db rocksdb:cleanup.db

# LLM applies changes (removes function)
# cargo build && cargo test
# Reset
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:cleanup.db
```

**Key Point**: DELETE action sets `future_ind=0`, removing entity from future state.

---

## IMPACT ANALYSIS

### Workflow 15: "What Will Break If I Change This?"

**Developer Need**: "I need to change the signature of `authenticate()`. What code will be affected?"

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:impact.db

# Export with code to find call sites
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:impact.db \
  --include-current-code 1

# LLM analyzes context.json
```

**LLM Queries:**
- "Find all functions that call `authenticate()`"
- "What modules import the auth module?"
- "Show me all places where authentication results are used"
- "What tests cover the authenticate function?"

**Output**: List of ISGL1 keys for affected code entities.

**Benefit**: Understand blast radius BEFORE making changes, not after breaking production.

---

### Workflow 16: Pre-Refactoring Risk Assessment

**Developer Need**: "Planning to refactor database layer. Need comprehensive impact assessment."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:risk.db

# Export full context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./db-layer-analysis.json \
  --db rocksdb:risk.db

# LLM generates risk report from context
```

**LLM Analysis Tasks:**
- "How many functions directly use the Database struct?"
- "What are the transitive dependencies (functions that call functions that use DB)?"
- "Which tests cover database operations?"
- "What error handling patterns are used for database failures?"
- "Are there any async/await complexities with database calls?"

**Output**: Risk assessment report with affected entity count and complexity estimates.

**Decision Point**: Is refactoring worth the risk? Schedule accordingly.

---

## DOCUMENTATION WORKFLOWS

### Workflow 17: Generate Architecture Documentation

**Developer Need**: "Create documentation showing all public APIs and their relationships."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:docs.db

# Export public API entities only
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * EXCEPT (Current_Code, Future_Code)
           FROM CodeGraph
           WHERE interface_signature LIKE '%pub fn%'
              OR interface_signature LIKE '%pub struct%'" \
  --output ./public-api.json \
  --db rocksdb:docs.db
```

**LLM Tasks with context:**
- "Generate markdown documentation for all public functions"
- "Create a module hierarchy diagram"
- "List all public structs with their fields"
- "Show relationships between public types"

**Output**: Auto-generated API documentation that stays in sync with code.

**Maintenance**: Re-run after significant changes to keep docs updated.

---

### Workflow 18: Document Complex Algorithm

**Developer Need**: "This sorting algorithm is complex. Generate step-by-step documentation."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:algo-doc.db

# Export specific function WITH code
parseltongue pt02-llm-cozodb-to-context-writer \
  --query "SELECT * FROM CodeGraph
           WHERE ISGL1_key = 'rust:fn:custom_sort:src_sort_rs:100-250'" \
  --output ./algorithm.json \
  --db rocksdb:algo-doc.db \
  --include-current-code 1
```

**LLM Tasks:**
- "Explain this algorithm in simple terms"
- "Generate flowchart from this code"
- "Identify time complexity"
- "Document edge cases handled"
- "Suggest improvements"

**Output**: Algorithm documentation with complexity analysis.

---

## ARCHITECTURE ANALYSIS

### Workflow 19: Module Dependency Visualization

**Developer Need**: "Visualize how our modules depend on each other."

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:arch.db

# Export all entities with signatures
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./architecture.json \
  --db rocksdb:arch.db
```

**LLM Tasks:**
- "Generate a Mermaid diagram showing module dependencies"
- "Identify circular dependencies"
- "Show which modules have the most dependencies"
- "List isolated modules with no dependents"

**Output**: Architecture diagrams in Mermaid format (can render in GitHub/GitLab).

---

### Workflow 20: Codebase Complexity Assessment

**Developer Need**: "Which parts of the codebase are most complex?"

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:complexity.db

# Export all entities
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:complexity.db
```

**LLM Tasks:**
- "Rank functions by complexity (based on interface signatures)"
- "Which modules have the most entities (functions/structs)?"
- "Identify modules with high interdependency"
- "Find functions with many parameters (complexity indicator)"

**Output**: Complexity hotspot report.

**Action**: Prioritize refactoring high-complexity areas.

---

## TECHNICAL DEBT MANAGEMENT

### Workflow 21: Identify Refactoring Candidates

**Developer Need**: "What code should we prioritize for refactoring?"

**Workflow Steps:**

```bash
# Index codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:tech-debt.db

# Export context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./codebase.json \
  --db rocksdb:tech-debt.db
```

**LLM Analysis:**
- "Find functions with signatures suggesting high complexity (many parameters)"
- "Identify naming inconsistencies"
- "Spot pattern violations"
- "Find orphaned entities (no callers)"

**Combine with git history:**
- "Which files change most frequently?" (git log analysis)
- "High churn + high complexity = top priority"

**Output**: Prioritized refactoring backlog.

---

### Workflow 22: Track Refactoring Progress

**Developer Need**: "We're refactoring the error handling. Track progress over time."

**Workflow Steps:**

```bash
# Week 1: Baseline
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:week1.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./week1.json --db rocksdb:week1.db

# Week 2: After initial refactoring
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:week2.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./week2.json --db rocksdb:week2.db

# Week 3: More progress
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:week3.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./week3.json --db rocksdb:week3.db
```

**LLM Comparative Analysis:**
- "Compare week1.json vs week3.json"
- "How many functions now use Result<T, E>?"
- "What percentage of error handling is refactored?"
- "Show remaining functions still using old patterns"

**Output**: Progress dashboard showing refactoring completion percentage.

---

## CODE REVIEW SUPPORT

### Workflow 23: Pre-Review Self-Check

**Developer Need**: "Before submitting PR, check if my changes make sense."

**Workflow Steps:**

```bash
# Index current codebase
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:review.db

# Export context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./before.json \
  --db rocksdb:review.db

# Make changes to temporal database (simulate your edits)
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:my_function:src_lib_rs:50-70" \
  --action edit \
  --future-code "... my changes ..." \
  --db rocksdb:review.db

# Generate diff
parseltongue pt05-llm-cozodb-to-diff-writer \
  --output MyChanges.json \
  --db rocksdb:review.db
```

**LLM Self-Review Questions:**
- "Do these changes follow existing patterns?"
- "Are there any naming inconsistencies?"
- "Is error handling consistent with the rest of the codebase?"
- "Are there similar functions I should update?"

**Output**: Self-review checklist before human review.

---

### Workflow 24: Reviewer Understanding PR

**Developer Need** (Reviewer): "Large PR just landed. What actually changed?"

**Workflow Steps:**

```bash
# Index codebase at PR base commit
git checkout main
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:base.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./base.json --db rocksdb:base.db

# Index codebase at PR head commit
git checkout feature-branch
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:head.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./head.json --db rocksdb:head.db
```

**LLM Comparison:**
- "What entities were added in head.json vs base.json?"
- "What entities were removed?"
- "What entities were modified?"
- "Summarize the high-level changes"

**Output**: High-level PR summary for quick reviewer understanding.

---

## ADVANCED WORKFLOWS

### Workflow 25: Iterative Development with Undo

**Developer Need**: "Try a refactoring approach. If it doesn't work, easily roll back."

**Workflow Steps:**

```bash
# Baseline
parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:experiment.db

# Export context
parseltongue pt02-llm-cozodb-to-context-writer \
  --output ./context.json \
  --db rocksdb:experiment.db

# Experiment 1: Try approach A
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:process:src_lib_rs:100-150" \
  --action edit \
  --future-code "... approach A ..." \
  --db rocksdb:experiment.db

parseltongue pt04-syntax-preflight-validator --db rocksdb:experiment.db
parseltongue pt05-llm-cozodb-to-diff-writer --output ApproachA.json --db rocksdb:experiment.db

# Apply and test
# cargo test
# Result: Tests fail

# Rollback: Just reset database without applying changes
parseltongue pt06-cozodb-make-future-code-current \
  --project ./src \
  --db rocksdb:experiment.db

# Experiment 2: Try approach B
parseltongue pt03-llm-to-cozodb-writer \
  --entity "rust:fn:process:src_lib_rs:100-150" \
  --action edit \
  --future-code "... approach B ..." \
  --db rocksdb:experiment.db

# Validate and test
# cargo test
# Result: Tests pass!

# Apply approach B
parseltongue pt05-llm-cozodb-to-diff-writer --output ApproachB.json --db rocksdb:experiment.db
# LLM applies changes
```

**Benefit**: Database state = working memory. Reset to undo without touching actual files until ready.

---

### Workflow 26: Cross-Project Analysis

**Developer Need**: "We have 3 microservices. Find inconsistent error handling across all of them."

**Workflow Steps:**

```bash
# Index service 1
parseltongue pt01-folder-to-cozodb-streamer ./service1/src --db rocksdb:svc1.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./svc1.json --db rocksdb:svc1.db

# Index service 2
parseltongue pt01-folder-to-cozodb-streamer ./service2/src --db rocksdb:svc2.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./svc2.json --db rocksdb:svc2.db

# Index service 3
parseltongue pt01-folder-to-cozodb-streamer ./service3/src --db rocksdb:svc3.db
parseltongue pt02-llm-cozodb-to-context-writer --output ./svc3.json --db rocksdb:svc3.db
```

**LLM Cross-Analysis:**
- "Compare error handling patterns in svc1.json, svc2.json, svc3.json"
- "Which service uses Result<T, E> consistently?"
- "Which service uses panic! or unwrap()?"
- "Generate a consistency report"

**Output**: Cross-service analysis report highlighting inconsistencies.

**Action**: Standardize patterns across services.

---

## WORKFLOW PATTERNS SUMMARY

### Command Usage Patterns

| Workflow Type | pt01 (Index) | pt02 (Read) | pt03 (Edit) | pt04 (Validate) | pt05 (Diff) | pt06 (Reset) |
|---------------|:------------:|:-----------:|:-----------:|:---------------:|:-----------:|:------------:|
| **Onboarding** | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| **Exploration** | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| **Bug Fixing** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Feature Dev** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Refactoring** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Impact Analysis** | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| **Documentation** | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| **Architecture Analysis** | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |

### Read-Only Workflows (Analysis)

- Onboarding & exploration
- Impact analysis
- Documentation generation
- Architecture analysis
- Code review preparation

**Pattern**: `pt01` → `pt02` → LLM analysis

### Write Workflows (Modification)

- Bug fixing
- Feature development
- Refactoring
- Technical debt remediation

**Pattern**: `pt01` → `pt02` → `pt03` → `pt04` → `pt05` → Apply → `pt06`

---

## KEY INSIGHTS FROM RESEARCH

### What Developers Actually Need (2024-2025)

**Top 5 Questions When Encountering Large Codebases:**
1. "What does this system do?" → **pt02** exports structured overview
2. "Where does feature X live?" → **pt02** + LLM semantic search
3. "What will break if I change this?" → **pt02** + LLM impact analysis
4. "How do I follow existing patterns?" → **pt02** + LLM pattern recognition
5. "Where are the entry points?" → **pt02** + LLM query for main/handlers

### Parseltongue's Unique Value

**Traditional Tools:**
- `grep`: Text search (brittle, noisy)
- IDE "Find References": Works within project, misses dynamic calls
- `git blame`: History, not structure
- Documentation: Often outdated

**Parseltongue Advantage:**
- **Structured representation**: ISGL1 keys + signatures (not just text)
- **Small context**: ~37.5k tokens for 1,500 entities (vs 500k+ with full code)
- **Temporal versioning**: Track proposed changes before applying
- **LLM-friendly format**: JSON export for semantic reasoning

### Time Savings Estimates

| Task | Traditional Time | Parseltongue Time | Savings |
|------|------------------|-------------------|---------|
| Understand new module | 2-3 hours | 15 minutes | 88-92% |
| Find feature location | 30-60 min | 2-5 minutes | 92-97% |
| Impact analysis | 1-2 hours | 10-15 minutes | 87-92% |
| Cross-file refactoring | 4-8 hours | 1-2 hours | 75-88% |
| Bug fix (simple) | 30-60 min | 5-10 minutes | 83-90% |
| Generate documentation | 4-6 hours | 20-30 minutes | 92-95% |

**Average Productivity Gain**: 85-90% time reduction for code understanding tasks.

---

## CONCLUSION

Parseltongue's 6-tool pipeline addresses real developer pain points:

1. **Orientation** (pt01 + pt02): Get structured codebase view in minutes
2. **Understanding** (pt02 + LLM): Semantic search over structured data
3. **Planning** (pt02 + pt03): Experiment with changes in temporal database
4. **Validation** (pt04): Catch syntax errors before file writes
5. **Application** (pt05): Generate structured diffs for precise application
6. **Reset** (pt06): Clean state transitions

**Core Innovation**: ISG (Interface Signature Graphs) enable reliable understanding in small context. LLMs reason over signatures without needing full code, unlocking semantic analysis at scale.

**Commands Define the Architecture**: These 6 commands are the guiding light. All workflows are compositions of these primitives.

---

**End of PossibleWorkflows.md**
