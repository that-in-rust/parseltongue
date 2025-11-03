# Swift Entity Extraction Fix Report

**Date**: 2025-11-03
**Version**: v0.8.9
**Status**: ✅ RESOLVED

## Executive Summary

Swift entity extraction was failing with "Failed to create query" error in v0.8.9 despite 11 other languages working correctly. The root cause was **incorrect node type names** in the Swift query file (`entity_queries/swift.scm`). The fix required:

1. Correcting Swift node types in query file
2. Adding `Interface` entity type to support Swift protocols
3. Mapping `Interface` to `Trait` in pt01 integration layer

**Result**: Swift now successfully extracts functions, classes, structs, enums, and protocols.

---

## Root Cause Analysis

### 1. **Error Location**

```
File: crates/parseltongue-core/src/query_extractor.rs
Line: 243-244

let query = Query::new(&ts_lang, query_source)
    .context("Failed to create query")?;
```

**Error**: `QueryError { row: 12, column: 1, offset: 255, message: "struct_declaration", kind: NodeType }`

### 2. **The Problem: Incorrect Node Type Names**

The original `entity_queries/swift.scm` file used node types that **do not exist** in tree-sitter-swift grammar v0.7:

#### ❌ **INCORRECT (Original)**
```scheme
; Structs
(struct_declaration
  name: (type_identifier) @name) @definition.struct

; Enums
(enum_declaration
  name: (type_identifier) @name) @definition.enum
```

#### Why This Failed

Swift's tree-sitter grammar uses a **unified node type** for all type declarations:

| Swift Keyword | Tree-sitter Node Type | First Child |
|--------------|----------------------|-------------|
| `class MyClass` | `class_declaration` | `class` |
| `struct MyStruct` | `class_declaration` | `struct` |
| `enum MyEnum` | `class_declaration` | `enum` |
| `protocol MyProtocol` | `protocol_declaration` | `protocol` |

The grammar does NOT have separate `struct_declaration` or `enum_declaration` nodes.

### 3. **Discovery Process**

#### Step 1: AST Analysis
```swift
struct MyStruct {
    var id: Int
}
```

**Parsed as**:
```
class_declaration      ← NOT struct_declaration!
  struct "struct"      ← Keyword is first child
  type_identifier "MyStruct"
  class_body { ... }
```

#### Step 2: Query Compilation Test
```rust
match Query::new(&swift_lang, swift_query) {
    Ok(_) => println!("✅ Success"),
    Err(e) => println!("❌ Error: {:?}", e),
}
```

**Output**:
```
❌ Error: QueryError {
    row: 12,
    column: 1,
    offset: 255,
    message: "struct_declaration",
    kind: NodeType
}
```

This confirmed `struct_declaration` is not recognized by the Swift grammar.

---

## The Fix

### 1. **Corrected Swift Query File**

**File**: `/Users/amuldotexe/Projects/parseltongue/entity_queries/swift.scm`

#### ✅ **CORRECT (Fixed)**
```scheme
; Swift entity extraction queries
; Based on tree-sitter-swift grammar v0.7
;
; CRITICAL: Swift grammar uses `class_declaration` for ALL type declarations
; (class, struct, enum), NOT separate node types like `struct_declaration`.
; The first child keyword ("class", "struct", "enum") differentiates them,
; but for entity extraction we treat them all as class-like entities.

; Functions
(function_declaration
  name: (simple_identifier) @name) @definition.function

; Classes, Structs, Enums - ALL use class_declaration node
; Note: Cannot distinguish between class/struct/enum at query level
; without predicates, so all are tagged as @definition.class
(class_declaration
  name: (type_identifier) @name) @definition.class

; Protocols (has its own node type)
(protocol_declaration
  name: (type_identifier) @name) @definition.interface
```

### 2. **Added Interface Entity Type**

**File**: `crates/parseltongue-core/src/query_extractor.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Function,
    Struct,
    Enum,
    Trait,
    Interface,  // ← NEW: Swift protocols, Java/C#/TypeScript interfaces
    Impl,
    Module,
    Class,
    Method,
    Typedef,
    Namespace,
}
```

**Rationale**: Swift protocols are semantically different from Rust traits. Adding `Interface` provides better semantic accuracy for languages like Swift, Java, C#, and TypeScript.

### 3. **Updated Entity Type Mapping**

**File**: `crates/parseltongue-core/src/query_extractor.rs`

```rust
fn parse_entity_type(&self, capture_name: &str) -> Option<EntityType> {
    match capture_name {
        "definition.function" => Some(EntityType::Function),
        "definition.struct" => Some(EntityType::Struct),
        "definition.class" => Some(EntityType::Class),
        "definition.enum" => Some(EntityType::Enum),
        "definition.trait" => Some(EntityType::Trait),
        "definition.interface" => Some(EntityType::Interface),  // ← NEW
        "definition.impl" => Some(EntityType::Impl),
        "definition.module" => Some(EntityType::Module),
        "definition.method" => Some(EntityType::Method),
        "definition.typedef" => Some(EntityType::Typedef),
        "definition.namespace" => Some(EntityType::Namespace),
        _ => None,
    }
}
```

### 4. **Updated pt01 Integration**

**File**: `crates/pt01-folder-to-cozodb-streamer/src/isgl1_generator.rs`

```rust
fn map_query_entity_type(
    &self,
    query_type: &parseltongue_core::query_extractor::EntityType
) -> EntityType {
    match query_type {
        parseltongue_core::query_extractor::EntityType::Function => EntityType::Function,
        parseltongue_core::query_extractor::EntityType::Class => EntityType::Class,
        parseltongue_core::query_extractor::EntityType::Method => EntityType::Method,
        parseltongue_core::query_extractor::EntityType::Struct => EntityType::Struct,
        parseltongue_core::query_extractor::EntityType::Enum => EntityType::Enum,
        parseltongue_core::query_extractor::EntityType::Trait => EntityType::Trait,
        parseltongue_core::query_extractor::EntityType::Interface => EntityType::Trait,  // ← NEW
        parseltongue_core::query_extractor::EntityType::Impl => EntityType::Impl,
        parseltongue_core::query_extractor::EntityType::Module => EntityType::Module,
        parseltongue_core::query_extractor::EntityType::Namespace => EntityType::Namespace,
        parseltongue_core::query_extractor::EntityType::Typedef => EntityType::Typedef,
    }
}
```

**Note**: `Interface` maps to `Trait` in pt01's type system for database storage compatibility.

---

## Verification

### Test 1: Query Compilation
```bash
cargo test --test swift_query_debug test_swift_query_compilation -- --nocapture
```

**Result**:
```
✅ Swift query compiled successfully!
Number of patterns: 3
Capture names: ["name", "definition.function", "definition.class", "definition.interface"]
```

### Test 2: Entity Extraction
```bash
cargo test --test swift_integration_test -- --nocapture
```

**Result**:
```
=== Extracted Swift Entities ===
Function: calculateSum (lines 6-8)
Class: UserManager (lines 11-17)
Function: addUser (lines 14-16)
Class: Point (lines 20-29)
Function: distance (lines 24-28)
Class: Direction (lines 32-37)
Interface: Drawable (lines 40-42)

✅ Swift integration test passed! Extracted 7 entities
```

### Test 3: All Tests Passing
```bash
cargo test -p parseltongue-core --lib
cargo test --test query_based_extraction_test
```

**Result**: All 40+ tests pass ✅

---

## What Was Learned

### 1. **Tree-sitter Grammar Variability**

Different languages use different AST structures:

| Language | Type Declarations |
|----------|------------------|
| **Rust** | `struct_item`, `enum_item`, `impl_item`, `trait_item` (separate nodes) |
| **Swift** | `class_declaration` (unified node for class/struct/enum) |
| **Ruby** | `class`, `module` (simple nodes) |
| **Python** | `class_definition`, `function_definition` |

**Lesson**: Always inspect the actual AST output for each language. Don't assume node names.

### 2. **Query Debugging Process**

The systematic debugging workflow:

1. **Parse sample code** → Inspect AST structure
2. **Identify node types** → List all available node kinds
3. **Write minimal query** → Test each pattern individually
4. **Compile query** → Check for syntax errors
5. **Run query** → Verify it matches expected nodes
6. **Integrate** → Wire into extraction pipeline

### 3. **Semantic vs Structural Typing**

Swift's unified `class_declaration` node is **structurally accurate** (all are type declarations) but **semantically ambiguous** (can't distinguish class from struct).

**Trade-offs**:
- ✅ **Simpler grammar**: One rule for all type declarations
- ❌ **Loss of precision**: Can't differentiate class/struct/enum in queries
- ⚖️ **Pragmatic solution**: Tag all as `@definition.class`, good enough for entity extraction

---

## Impact

### Languages Now Working (12 total)

| Language | Status | Entity Types Extracted |
|----------|--------|----------------------|
| Rust ✅ | Working | fn, struct, enum, trait, impl, mod |
| Python ✅ | Working | class, function, method |
| JavaScript ✅ | Working | function, class, method |
| TypeScript ✅ | Working | function, class, interface, method |
| C ✅ | Working | function, struct, enum, typedef |
| C++ ✅ | Working | function, class, struct, namespace |
| Go ✅ | Working | function, struct, interface, method |
| Java ✅ | Working | class, method, interface, enum |
| Ruby ✅ | Working | class, module, method |
| PHP ✅ | Working | class, function, method, namespace |
| C# ✅ | Working | class, struct, interface, method, namespace |
| **Swift ✅** | **FIXED** | **function, class (struct/enum), protocol** |

### Kotlin Status

Kotlin remains disabled due to tree-sitter version incompatibility (0.20 vs 0.25).

---

## Files Modified

1. `/Users/amuldotexe/Projects/parseltongue/entity_queries/swift.scm`
   - Fixed node type names
   - Added documentation about Swift grammar quirks

2. `/Users/amuldotexe/Projects/parseltongue/crates/parseltongue-core/src/query_extractor.rs`
   - Added `Interface` to `EntityType` enum
   - Added `"definition.interface"` mapping in `parse_entity_type()`

3. `/Users/amuldotexe/Projects/parseltongue/crates/pt01-folder-to-cozodb-streamer/src/isgl1_generator.rs`
   - Added `Interface → Trait` mapping in `map_query_entity_type()`

---

## Recommendations

### For Future Language Support

1. **Always start with AST exploration**:
   ```rust
   let tree = parser.parse(code, None).unwrap();
   print_ast_structure(tree.root_node(), code);
   ```

2. **Test queries incrementally**:
   - Start with one pattern (e.g., just functions)
   - Verify compilation
   - Verify matching
   - Add next pattern

3. **Document grammar quirks**:
   - Add comments to .scm files explaining non-obvious choices
   - Note version-specific behavior

4. **Consider semantic precision**:
   - When grammar doesn't distinguish types, decide if precision matters
   - For Swift: class/struct/enum all tagged as `class` is acceptable
   - Could be refined later with predicates if needed

### For Query System Evolution

1. **Support predicates** for finer-grained matching:
   ```scheme
   (class_declaration
     . (struct) @keyword
     name: (type_identifier) @name) @definition.struct
   (#eq? @keyword "struct")
   ```

2. **Add query validation** in CI:
   - Test all .scm files compile against their grammars
   - Prevents shipping broken queries

3. **Create query testing framework**:
   - Standard test cases for each language
   - Automated verification of entity extraction

---

## Conclusion

Swift entity extraction now works correctly in v0.8.9. The fix demonstrates the importance of:

1. **Understanding tree-sitter grammar specifics** per language
2. **Systematic debugging** from query compilation to entity extraction
3. **Semantic extensibility** (adding `Interface` type for cross-language support)

All 12 languages (except Kotlin) now have working entity extraction via QueryBasedExtractor.

---

**Author**: Claude (Agent Games 2025)
**Review Status**: Ready for commit
**Next Steps**: Commit fix, update CHANGELOG, consider adding query validation CI
