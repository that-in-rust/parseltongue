# Ruby Extraction Failure - Architecture & Dependency Analysis

## System Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Parseltongue v0.8.8                          │
│                   Entity Extraction System                        │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                    TWO PARALLEL IMPLEMENTATIONS                  │
│                         (INCONSISTENT)                           │
└─────────────────────────────────────────────────────────────────┘

┌────────────────────────────────┐  ┌────────────────────────────────┐
│  Query-Based Extractor (✅)    │  │  Manual Tree-Walking (❌)      │
│  parseltongue-core             │  │  pt01-folder-to-cozodb-streamer│
├────────────────────────────────┤  ├────────────────────────────────┤
│ • 12 languages supported       │  │ • Only Rust implemented        │
│ • Uses .scm query files        │  │ • 11 languages broken          │
│ • Declarative pattern matching │  │ • Imperative tree traversal    │
│ • Well-tested                  │  │ • Missing extraction functions │
│ • Used by: pt02, pt03, pt04    │  │ • Used by: pt01 (Level 0)      │
└────────────────────────────────┘  └────────────────────────────────┘
          ✅ COMPLETE                         ❌ INCOMPLETE
```

---

## Dependency Flow for Ruby File Processing

```
USER: parseltongue pt01-folder-to-cozodb-streamer /path/to/campfire
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 1. CLI Entry Point (main.rs)                                    │
│    - Parses command-line arguments                              │
│    - Creates StreamerConfig                                     │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 2. FileStreamerImpl::stream_files()                             │
│    crates/pt01-folder-to-cozodb-streamer/src/streamer.rs:337    │
│                                                                  │
│    - Walks directory tree (WalkDir)                             │
│    - Filters files by extension (.rb, .py, .rs, etc.)           │
│    - Finds: 311 Ruby files                                      │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 3. FileStreamerImpl::stream_file(path: &Path)                   │
│    crates/pt01-folder-to-cozodb-streamer/src/streamer.rs:401    │
│                                                                  │
│    For each file (e.g., "app/models/user.rb"):                  │
│    - Read file content ✅                                        │
│    - Detect language from extension: Language::Ruby ✅           │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 4. key_generator.parse_source(content, file_path)               │
│    crates/pt01-folder-to-cozodb-streamer/src/streamer.rs:408    │
│                                                                  │
│    Uses: Isgl1KeyGeneratorImpl                                  │
│    ↓                                                             │
│    Isgl1KeyGeneratorImpl::parse_source()                        │
│    crates/.../src/isgl1_generator.rs:131                        │
│                                                                  │
│    - Get Ruby parser ✅                                          │
│    - Parse with tree-sitter-ruby ✅                              │
│    - Returns: Valid AST tree ✅                                  │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 5. extract_entities(tree, source, file_path, Language::Ruby)    │
│    crates/.../src/isgl1_generator.rs:179                        │
│                                                                  │
│    let mut entities = Vec::new();                               │
│    let mut dependencies = Vec::new();                           │
│                                                                  │
│    // Pass 1: Extract entities                                  │
│    walk_node(&root, source, file_path, Language::Ruby,          │
│              &mut entities, &mut dependencies);                 │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 6. walk_node() - THE FAILURE POINT ❌                            │
│    crates/.../src/isgl1_generator.rs:219                        │
│                                                                  │
│    fn walk_node(..., language: Language, ...) {                 │
│        if language == Language::Rust && node.kind() == "fn" {   │
│            // Extract Rust functions                            │
│        } else {                                                 │
│            match language {                                     │
│                Language::Rust => extract_rust_entities(...),    │
│                Language::Python => {                            │
│                    // TODO: Implement Python extraction         │
│                }                                                │
│                _ => {}  ← Ruby hits this and DOES NOTHING ❌     │
│            }                                                    │
│        }                                                        │
│                                                                  │
│        // Recurse to children                                   │
│        for child in node.children() {                           │
│            walk_node(child, ...);  ← Children also do nothing   │
│        }                                                        │
│    }                                                            │
│                                                                  │
│    Result:                                                      │
│    - Entire Ruby AST traversed                                  │
│    - Zero entities captured                                     │
│    - entities Vec remains empty: []                             │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 7. Returns to stream_file()                                      │
│                                                                  │
│    let (parsed_entities, dependencies) =                        │
│        key_generator.parse_source(...)?;                        │
│                                                                  │
│    parsed_entities.len() == 0  ← EMPTY!                         │
│                                                                  │
│    // Try to process entities...                                │
│    for entity in parsed_entities {  // Loop never executes      │
│        // ... generate keys, store in DB                        │
│    }                                                            │
│                                                                  │
│    Result: 0 entities stored in database                        │
└──────────────────────────────┬──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│ 8. Final Statistics                                              │
│                                                                  │
│    Files processed: 311 ✅                                       │
│    Entities created: 0 ❌                                        │
│    Errors: 356 ⚠️  (downstream failures expecting entities)     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Tree-sitter Ruby Integration Analysis

### Parser Initialization (✅ WORKS)

```rust
// crates/.../src/isgl1_generator.rs:86
init_parser!(Language::Ruby, &tree_sitter_ruby::LANGUAGE.into());
```

**Status:** ✅ SUCCESS
- tree-sitter-ruby v0.23.1 loaded
- Compatible with tree-sitter 0.24.x
- Parser creates valid AST for Ruby code

### Test Validation

```bash
$ cargo test test_multiple_languages_basic_parsing

Test: ("test.rb", "def hello\nend", "Ruby")
  ↓
  generator.parse_source(code, path)
  ↓
  Result: Ok((entities, deps))
  ↓
  assert!(result.is_ok()) ✅ PASSES

  BUT DOESN'T CHECK: entities.len() > 0
  ACTUAL: entities.len() == 0 ❌
```

**The test is misleading!** It only verifies:
1. Parser doesn't crash ✅
2. parse_source() returns Ok ✅

It DOES NOT verify:
3. Entities are extracted ❌

---

## Ruby AST Structure vs Extraction Logic

### What Tree-sitter Ruby Produces (✅ CORRECT)

```ruby
class User
  def greet
    puts "Hello"
  end
end

module Helper
  def self.format(text)
    text.upcase
  end
end
```

**Tree-sitter AST:**
```
(program
  (class
    name: (constant) "User"  ← Should extract as EntityType::Class
    (body_statement
      (method
        name: (identifier) "greet"  ← Should extract as EntityType::Method
        (body_statement ...))))
  (module
    name: (constant) "Helper"  ← Should extract as EntityType::Module
    (body_statement
      (singleton_method
        name: (identifier) "format"  ← Should extract as EntityType::Method
        ...))))
```

### What Extraction Logic Does (❌ NOTHING)

```rust
fn walk_node(node, ..., language: Language::Ruby, entities, ...) {
    // language is Ruby, so first `if` is false
    if language == Language::Rust && node.kind() == "function_item" {
        // NOT EXECUTED
    } else {
        match language {
            Language::Rust => { /* NOT EXECUTED */ }
            Language::Python => { /* NOT EXECUTED */ }
            _ => {}  ← EXECUTES THIS - DOES NOTHING
        }
    }

    // Recurses to children, but they also do nothing
    for child in node.children() {
        walk_node(child, ..., Language::Ruby, entities, ...);
        // child.kind() might be "class", "method", "module"
        // But same logic: match _ => {} does nothing
    }

    // Returns with entities still empty
}
```

**Result:** Every Ruby AST node (class, module, method) is visited but ignored.

---

## Missing Extraction Functions

### What EXISTS for Rust (✅ Reference Implementation)

```rust
// Line 332-360
fn extract_rust_entities(
    &self,
    node: &tree_sitter::Node<'_>,
    source: &str,
    file_path: &Path,
    entities: &mut Vec<ParsedEntity>,
) {
    match node.kind() {
        "function_item" => { /* Extract function */ }
        "struct_item" => {
            if let Some(name) = self.extract_struct_name(node, source) {
                entities.push(ParsedEntity {
                    entity_type: EntityType::Struct,
                    name,
                    language: Language::Rust,
                    line_range: (start, end),
                    file_path: file_path.to_string_lossy().to_string(),
                    metadata: HashMap::new(),
                });
            }
        }
        _ => {}
    }
}

fn extract_struct_name(&self, node, source) -> Option<String> {
    for child in node.children() {
        if child.kind() == "type_identifier" {
            return Some(source[child.byte_range()].to_string());
        }
    }
    None
}
```

### What SHOULD EXIST for Ruby (❌ MISSING)

```rust
// DOES NOT EXIST - NEEDS TO BE IMPLEMENTED
fn extract_ruby_entities(
    &self,
    node: &tree_sitter::Node<'_>,
    source: &str,
    file_path: &Path,
    entities: &mut Vec<ParsedEntity>,
) {
    match node.kind() {
        "class" => {
            if let Some(name) = self.extract_constant_name(node, source) {
                entities.push(ParsedEntity {
                    entity_type: EntityType::Class,
                    name,
                    language: Language::Ruby,
                    line_range: (start, end),
                    file_path: file_path.to_string_lossy().to_string(),
                    metadata: HashMap::new(),
                });
            }
        }
        "module" => { /* Similar to class */ }
        "method" | "singleton_method" => { /* Extract method */ }
        _ => {}
    }
}

fn extract_constant_name(&self, node, source) -> Option<String> {
    for child in node.children() {
        if child.kind() == "constant" {
            return Some(source[child.byte_range()].to_string());
        }
    }
    None
}

fn extract_method_name(&self, node, source) -> Option<String> {
    for child in node.children() {
        if child.kind() == "identifier" {
            return Some(source[child.byte_range()].to_string());
        }
    }
    None
}
```

**Status:** NONE of these functions exist in the codebase.

---

## Query-Based System (Alternative Architecture)

### How QueryBasedExtractor Works (✅ COMPLETE)

```rust
// crates/parseltongue-core/src/query_extractor.rs:84-105

impl QueryBasedExtractor {
    pub fn new() -> Result<Self> {
        let mut queries = HashMap::new();

        // Embed query files at compile time
        queries.insert(
            Language::Ruby,
            include_str!("../../../entity_queries/ruby.scm").to_string()
        );

        // ... 11 more languages

        let mut parsers = HashMap::new();
        Self::init_parser(&mut parsers, Language::Ruby,
                         &tree_sitter_ruby::LANGUAGE.into())?;

        Ok(Self { queries, parsers })
    }

    pub fn parse_source(
        &mut self,
        source: &str,
        file_path: &Path,
        language: Language,
    ) -> Result<(Vec<ParsedEntity>, Vec<DependencyEdge>)> {
        // Get query for language
        let query_source = self.queries.get(&language).unwrap();

        // Parse with tree-sitter
        let tree = parser.parse(source, None).unwrap();

        // Execute query on tree
        let entities = self.execute_query(&tree, source, file_path,
                                         language, query_source)?;

        Ok((entities, vec![]))
    }
}
```

**Status:** ✅ FULLY IMPLEMENTED for Ruby and 11 other languages

### Ruby Query File (✅ EXISTS)

```scheme
; entity_queries/ruby.scm

; Classes
(class
  name: (constant) @name) @definition.class

; Modules
(module
  name: (constant) @name) @definition.module

; Methods
(method
  name: (identifier) @name) @definition.method

; Singleton methods (class methods)
(singleton_method
  name: (identifier) @name) @definition.method
```

**Status:** ✅ COMPLETE and tested

### Why Two Systems?

```
┌───────────────────────────────────────────────────────────────┐
│                     Historical Context                         │
├───────────────────────────────────────────────────────────────┤
│ Phase 1: Manual Tree-Walking (pt01)                           │
│   - Built first for Rust                                      │
│   - Imperative, language-specific extraction                  │
│   - Direct tree traversal with match statements               │
│                                                                │
│ Phase 2: Query-Based System (parseltongue-core)               │
│   - Built later for scalability                               │
│   - Declarative .scm query files                              │
│   - Tree-sitter query language (more powerful)                │
│   - Added for pt02, pt03, pt04 tools                          │
│                                                                │
│ Problem: pt01 never migrated to new system                    │
│   - Still uses old manual tree-walking                        │
│   - Rust works (was implemented first)                        │
│   - 11 other languages never got extraction functions         │
│   - Query files exist but pt01 doesn't use them               │
└───────────────────────────────────────────────────────────────┘
```

---

## Module Dependency Graph

```
┌─────────────────────────────────────────────────────────────────┐
│                      parseltongue (workspace)                    │
└─────────────────────────────────────────────────────────────────┘
                               │
        ┌──────────────────────┴──────────────────────┐
        ↓                                             ↓
┌──────────────────────┐                  ┌──────────────────────┐
│ parseltongue-core    │                  │ pt01-folder-to-      │
│                      │                  │ cozodb-streamer      │
├──────────────────────┤                  ├──────────────────────┤
│ ✅ QueryBasedExtract│                  │ ❌ Manual Tree-Walk  │
│    - 12 languages    │                  │    - Only Rust       │
│    - .scm queries    │                  │    - No queries used │
│    - Well-tested     │                  │                      │
│                      │                  │ Uses:                │
│ Used by:             │                  │ • tree-sitter-ruby   │
│ • pt02-level00       │                  │ • tree-sitter-python │
│ • pt02-level01       │                  │ • ... (11 parsers)   │
│ • pt02-level02       │                  │                      │
│ • pt03-*             │                  │ Missing:             │
│ • pt04-*             │                  │ • extract_ruby_*()   │
│                      │                  │ • extract_python_*() │
│                      │                  │ • ... (11 functions) │
└──────────────────────┘                  └──────────────────────┘
         ✅                                        ❌
    ALL TOOLS WORK                           ONLY pt01 BROKEN
```

**Key Insight:** The problem is ISOLATED to pt01-folder-to-cozodb-streamer. All other tools use QueryBasedExtractor and work fine.

---

## Impact Analysis: Campfire Codebase

### Expected Entity Distribution

```
Basecamp Campfire (Ruby/Rails app)
├─ 311 Ruby files
├─ Estimated entities:
│  ├─ ~100 classes (models, controllers, services)
│  ├─ ~50 modules (helpers, concerns)
│  ├─ ~1,500 methods (instance + class methods)
│  └─ Total: ~1,650 entities
│
└─ Actual extraction:
   ├─ Classes: 0 ❌
   ├─ Modules: 0 ❌
   ├─ Methods: 0 ❌
   └─ Total: 0 ❌ (100% failure)
```

### Error Cascade

```
311 files processed
  ↓
  Each file: parse_source() returns ([], [])  ← Empty entities
  ↓
  stream_file() tries to process empty list
  ↓
  Downstream code expects entities
  ↓
  Validation/storage failures
  ↓
  356 errors logged (~1.1 errors per file)
```

**Error types likely include:**
- Empty entity list warnings
- Missing ISGL1 keys
- Database insertion failures
- LSP metadata fetch failures (no entities to query)

---

## Comparison Matrix

| Feature | Manual Tree-Walk (pt01) | Query-Based (core) |
|---------|-------------------------|---------------------|
| **Languages Supported** | 1 (Rust only) | 12 (all) |
| **Ruby Extraction** | ❌ Broken | ✅ Works |
| **Python Extraction** | ❌ Broken (TODO) | ✅ Works |
| **Maintainability** | ❌ Low (imperative) | ✅ High (declarative) |
| **Code Volume** | ~400 lines per language | ~20 lines per language |
| **Test Coverage** | ❌ Rust only | ✅ All languages |
| **Extensibility** | ❌ Hard (write Rust) | ✅ Easy (write .scm) |
| **Performance** | ~Same | ~Same |
| **Used By** | pt01 only | pt02, pt03, pt04 |

---

## Root Cause Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                    ROOT CAUSE CHAIN                              │
├─────────────────────────────────────────────────────────────────┤
│ 1. pt01 uses manual tree-walking extraction                     │
│    └─ Implementation: Isgl1KeyGeneratorImpl                      │
│                                                                  │
│ 2. walk_node() only handles Rust                                │
│    └─ Ruby/Python/etc. fall to `_ => {}` wildcard               │
│                                                                  │
│ 3. No extract_ruby_entities() function exists                   │
│    └─ Required helper functions also missing                    │
│                                                                  │
│ 4. Test only checks parsing, not extraction                     │
│    └─ test_multiple_languages_basic_parsing() is misleading     │
│                                                                  │
│ 5. No integration tests for non-Rust languages                  │
│    └─ Bug went undetected                                       │
│                                                                  │
│ RESULT: 0 entities extracted from 311 Ruby files                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Fix Complexity Estimates

### Option A: Implement Manual Extraction for Each Language

```
Per Language Implementation:
├─ extract_{lang}_entities() function: ~100 lines
├─ Helper functions (extract_*_name): ~50 lines
├─ EntityType variants: ~5 lines
├─ Tests: ~150 lines
└─ Total: ~305 lines per language

Languages Needed:
├─ Ruby (priority 1)
├─ Python (priority 2, has TODO)
├─ JavaScript/TypeScript
├─ Java
├─ Go
├─ C/C++
├─ C#
├─ PHP
├─ Swift
├─ Scala
└─ Total: 11 languages × 305 lines = 3,355 lines of code

Estimated Effort: 2-3 weeks full-time
```

### Option B: Migrate to Query-Based System

```
Refactoring Work:
├─ Modify FileStreamerImpl::stream_file(): ~20 lines
├─ Replace Isgl1KeyGeneratorImpl with QueryBasedExtractor: ~30 lines
├─ Update tests: ~50 lines
├─ Delete old code: ~400 lines
└─ Total: ~100 lines changed, 400 deleted

Benefits:
├─ All 12 languages work immediately ✅
├─ Leverage existing .scm query files ✅
├─ Reduce codebase complexity ✅
└─ Easier to maintain going forward ✅

Estimated Effort: 2-3 days
```

**Recommendation:** Option B (10x faster, more maintainable)

---

## Conclusion

The Ruby extraction failure is a **systemic architectural issue**, not a tree-sitter or query file problem. The root cause is **incomplete implementation** of language support in the manual tree-walking system used exclusively by pt01-folder-to-cozodb-streamer.

**Quick Fix:** Implement `extract_ruby_entities()` (tactical, addresses immediate need)

**Strategic Fix:** Refactor pt01 to use QueryBasedExtractor (faster, solves all 11 languages)
