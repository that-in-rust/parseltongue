# Parseltongue Tree-Sitter Usage Analysis

## Executive Summary

Parseltongue currently uses **tree-sitter 0.22.6** (from Cargo.lock) with language bindings for 13 programming languages. There is a **critical API divergence** between PT01 (isgl1_generator.rs) and PT04 (simple_validator.rs) regarding how language constants are accessed:

- **PT01**: Uses direct constant references (e.g., `&tree_sitter_rust::LANGUAGE`)
- **PT04**: Uses function calls (e.g., `&tree_sitter_rust::LANGUAGE()`)

This difference indicates either:
1. PT04 is targeting a newer tree-sitter grammar version with function-based LANGUAGE constants
2. PT01 needs updating to match the current API
3. There's incomplete migration between API versions

---

## Section 1: Current Tree-Sitter Usage

### 1.1 Dependency Configuration

**File**: `/Cargo.toml` (workspace root)

**Current Versions** (from Cargo.lock):
```
tree-sitter = 0.22.6 (core API)
tree-sitter-rust = 0.23.x
tree-sitter-python = 0.25.x
tree-sitter-javascript = 0.25.x
tree-sitter-typescript = 0.23.x
tree-sitter-go = 0.25.x
tree-sitter-java = 0.23.x
tree-sitter-c = 0.24.x
tree-sitter-cpp = 0.23.x
tree-sitter-ruby = 0.23.x
tree-sitter-php = 0.24.x
tree-sitter-c-sharp = 0.23.x
tree-sitter-swift = 0.7.x
tree-sitter-kotlin = 0.3.x
tree-sitter-scala = 0.24.x
```

**Crates Using Tree-Sitter**:
- `parseltongue-core`: Direct dependency (minimal usage)
- `pt01-folder-to-cozodb-streamer`: Full language support (entity extraction)
- `pt04-syntax-preflight-validator`: Full language support (syntax validation)

---

## Section 2: Parser Initialization Patterns

### 2.1 PT01 Implementation (isgl1_generator.rs)

**Location**: `crates/pt01-folder-to-cozodb-streamer/src/isgl1_generator.rs:62-94`

**Pattern**: Uses constant references directly
```rust
macro_rules! init_parser {
    ($lang:expr, $grammar:expr) => {
        let mut parser = Parser::new();
        if parser.set_language($grammar).is_ok() {
            parsers.insert($lang, Arc::new(Mutex::new(parser)));
        }
    };
}

init_parser!(Language::Rust, &tree_sitter_rust::LANGUAGE);      // Constant reference
init_parser!(Language::TypeScript, &tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
init_parser!(Language::Kotlin, &tree_sitter_kotlin::language()); // EXCEPTION: Function call
```

**Key Observations**:
- Most languages use `&LANGUAGE` (constant reference)
- Kotlin is an exception: `&tree_sitter_kotlin::language()` (function call)
- Stores parsers in `HashMap<Language, Arc<Mutex<Parser>>>`
- Thread-safe with Mutex for shared state across threads

### 2.2 PT04 Implementation (simple_validator.rs)

**Location**: `crates/pt04-syntax-preflight-validator/src/simple_validator.rs:52-75`

**Pattern**: Uses function calls for all languages
```rust
macro_rules! init_parser {
    ($lang:expr, $grammar:expr) => {
        let mut parser = Parser::new();
        if parser.set_language($grammar).is_ok() {
            parsers.insert($lang, parser);
        }
    };
}

init_parser!(Language::Rust, &tree_sitter_rust::LANGUAGE());      // Function call
init_parser!(Language::TypeScript, &tree_sitter_typescript::LANGUAGE_TYPESCRIPT()); // Function call
init_parser!(Language::Kotlin, &tree_sitter_kotlin::language());
```

**Key Observations**:
- ALL languages use function calls (e.g., `LANGUAGE()`)
- Stores parsers in simple `HashMap<Language, Parser>` (no Arc/Mutex)
- Not thread-safe by default (would need Arc<Mutex> for async usage)
- Simpler approach but less suitable for concurrent access

### 2.3 API Divergence Explanation

This is the **critical API difference**:

**Tree-sitter < 0.20**: Language constants were direct static references
```rust
pub const LANGUAGE: Language = Language { /* ... */ };
```

**Tree-sitter 0.20+**: Language constants became functions to enable lazy initialization
```rust
pub fn LANGUAGE() -> Language { /* ... */ }
// or older grammar versions:
pub const LANGUAGE: Language = Language { /* ... */ };
```

**Current State in Parseltongue**:
- **PT01 assumes**: Constant references (pre-0.20 style)
- **PT04 assumes**: Function calls (0.20+ style)

This suggests **PT04 was created/updated more recently** than PT01 and reflects the newer tree-sitter grammar API.

---

## Section 3: Language Enum vs Implementation Coverage

### 3.1 Language Enum Definition

**Location**: `crates/parseltongue-core/src/entities.rs:12-28`

**Defined Languages** (13 total):
1. Rust
2. JavaScript
3. TypeScript
4. Python
5. Java
6. Cpp
7. Go
8. Ruby
9. Php
10. CSharp
11. Swift
12. Kotlin
13. Scala

### 3.2 LanguageSpecificSignature Enum Coverage

**Location**: `crates/parseltongue-core/src/entities.rs:313-327`

**Implemented Language-Specific Signatures** (5 variants):
1. Rust(RustSignature)
2. JavaScript(JavascriptSignature)
3. TypeScript(TypeScriptSignature)
4. Python(PythonSignature)
5. Java(JavaSignature)

**NOT YET IMPLEMENTED** (8 languages):
1. Cpp → Falls through to default Rust signature
2. Go → Falls through to default Rust signature
3. Ruby → Falls through to default Rust signature
4. Php → Falls through to default Rust signature
5. CSharp → Falls through to default Rust signature
6. Swift → Falls through to default Rust signature
7. Kotlin → Falls through to default Rust signature
8. Scala → Falls through to default Rust signature

**Code Reference** (streamer.rs:214-238):
```rust
fn create_language_signature(&self, language: &Language) -> LanguageSpecificSignature {
    match language {
        Language::Rust => LanguageSpecificSignature::Rust(RustSignature { /* ... */ }),
        Language::Python => LanguageSpecificSignature::Python(PythonSignature { /* ... */ }),
        _ => LanguageSpecificSignature::Rust(RustSignature { /* ... */ }),  // Fallback!
    }
}
```

---

## Section 4: Parser Usage in PT01 (Folder-to-CozoDB Streamer)

### 4.1 Initialization

**File**: `crates/pt01-folder-to-cozodb-streamer/src/isgl1_generator.rs`

**Class**: `Isgl1KeyGeneratorImpl`

```rust
pub struct Isgl1KeyGeneratorImpl {
    parsers: HashMap<Language, Arc<Mutex<Parser>>>,
}
```

### 4.2 Entity Extraction Flow

1. **parse_source()** (line 130-152):
   - Takes source code and file path
   - Calls `get_language_type()` to detect language from extension
   - Acquires parser from HashMap
   - Parses with tree-sitter
   - Calls `extract_entities()` for two-pass extraction
   - Returns `(Vec<ParsedEntity>, Vec<DependencyEdge>)`

2. **extract_entities()** (line 174-196):
   - **Pass 1**: `walk_node()` extracts all entities
   - **Pass 2**: `extract_dependencies_pass2()` (Rust-only) extracts dependencies

3. **Rust-Specific Entity Types** (walk_node, line 228-240):
   - `function_item` → EntityType::Function (with test attribute detection)
   - `struct_item` → EntityType::Struct
   - Other node types skipped

4. **Test Detection** (check_preceding_test_attribute, line 382-409):
   - Checks immediate preceding sibling for `#[test]`, `#[tokio::test]`, `#[async_test]`
   - Stores `"is_test": "true"` in metadata
   - Later converted to `EntityClass::TestImplementation`

### 4.3 Dependency Extraction

**For Rust Only** (extract_dependencies_pass2, line 199-215):
- Extracts `call_expression` nodes (function calls)
- Creates `DependencyEdge` with `EdgeType::Calls`
- Uses ISGL1 keys for both source and target entities
- Records source location (file:line)

**Note**: Python, C++, Go, Java, etc. extraction is stubbed with `// TODO: Implement Python entity extraction`

### 4.4 Language Detection

**Method**: `get_language_type()` (line 154-170):
```rust
fn get_language_type(&self, file_path: &Path) -> Result<Language> {
    let path_buf = file_path.to_path_buf();
    let language = Language::from_file_path(&path_buf)
        .ok_or_else(|| StreamerError::UnsupportedFileType { ... })?;
    
    if self.parsers.contains_key(&language) {
        Ok(language)
    } else {
        Err(StreamerError::UnsupportedFileType { ... })
    }
}
```

Uses `Language::from_file_path()` from entities.rs for extension-based detection.

---

## Section 5: Syntax Validation in PT04

### 5.1 Validator Structure

**File**: `crates/pt04-syntax-preflight-validator/src/simple_validator.rs`

```rust
pub struct SimpleSyntaxValidator {
    parsers: HashMap<Language, Parser>,
}
```

**Key Difference from PT01**:
- No Arc/Mutex (stored directly in HashMap)
- Single-threaded by default
- Simpler to initialize

### 5.2 Validation Flow

**Method**: `validate_syntax()` (line 83-108):
1. Get parser for specified language
2. Parse code: `parser.parse(code, None)`
3. Check root node: `root.has_error()`
4. If errors found, recursively collect with `collect_syntax_errors()`
5. Return `ValidationResult { is_valid, errors: Vec<String> }`

### 5.3 Error Collection

**Method**: `collect_syntax_errors()` (line 111-143):
- Walks parse tree recursively
- Collects error nodes: `node.is_error() || node.is_missing()`
- Formats error messages with line/column info (0-based, converted to 1-based)
- Handles both error and missing nodes

**Example Error Message**:
```
"Syntax error at line 5, column 10 (ends at line 5, column 15)"
"Missing syntax element at line 3, column 2"
```

### 5.4 What PT04 Does NOT Validate

Per documentation (src/lib.rs):
- Type errors (cargo build handles)
- Import resolution (cargo build handles)
- Lifetime issues (cargo build handles)
- Logic bugs (tests handle)
- Only validates parse tree structure

---

## Section 6: Supported vs Missing Implementations

### 6.1 All 13 Languages Have Parsers

**PT01 Parser Initialization** (all 13):
✓ Rust, Python, JavaScript, TypeScript, Go, Java, C++, Ruby, PHP, C#, Swift, Kotlin, Scala

**PT04 Parser Initialization** (all 13):
✓ Rust, Python, JavaScript, TypeScript, Go, Java, C++, Ruby, PHP, C#, Swift, Kotlin, Scala

### 6.2 Entity Extraction Status

**Fully Implemented**:
✓ Rust (functions, structs, enums, traits, impls, modules, test detection)

**Stubbed/TODO**:
✗ Python: "TODO: Implement Python entity extraction"
✗ All others: Not handled in walk_node()

**Fallback for Core Types**:
- All unsupported languages fall back to Rust-style signature in `create_language_signature()`
- Loses language-specific semantic information

### 6.3 LanguageSpecificSignature Variants

**Full Implementation** (language-specific metadata):
- Rust(RustSignature): generics, lifetimes, where_clauses, attributes, trait_impl
- JavaScript(JavascriptSignature): parameters, return_type, is_async, is_arrow
- TypeScript(TypeScriptSignature): parameters, return_type, generics, is_async
- Python(PythonSignature): parameters, return_type, is_async, decorators
- Java(JavaSignature): access_modifier, parameters, return_type, throws, is_static, generics

**Missing** (8 languages):
- Cpp, Go, Ruby, Php, CSharp, Swift, Kotlin, Scala: No specific signature types
- Would require: CppSignature, GoSignature, RubySignature, PhpSignature, CSharpSignature, SwiftSignature, KotlinSignature, ScalaSignature

---

## Section 7: Architecture Constraints

### 7.1 Parser Storage Strategy

**Current**: HashMap with Language key
```rust
HashMap<Language, Arc<Mutex<Parser>>>  // PT01: Thread-safe
HashMap<Language, Parser>              // PT04: Single-threaded
```

**Constraints**:
- Must initialize all parsers at startup (slow for rarely-used languages)
- Memory overhead: 13 parsers × ~2-5MB each = ~30-65MB
- Lazy initialization would improve startup time

### 7.2 Entity Type Duplication

**Problem**: Two EntityType enums:
1. `crate::isgl1_generator::EntityType` (PT01 internal)
   - Function, Struct, Enum, Trait, Impl, Module, Variable
2. `parseltongue_core::entities::EntityType` (public API)
   - Function, Method, Struct, Enum, Trait, Interface, Module, ImplBlock, Macro, ProcMacro, TestFunction, Class, Variable, Constant

**Conversion Required**: streamer.rs:199-212 must manually convert
**Missing PT01 Support**: Macro, ProcMacro, TestFunction, Class, Constant

### 7.3 Language Filter Threading

**PT01**: Uses Arc<Mutex<Parser>> to share parsers across threads
- FileStreamerImpl is async (tokio)
- Multiple files could be parsed concurrently
- BUT: Each parser has individual Mutex (allows concurrent parsing)

**PT04**: Uses plain Parser (not thread-safe)
- SimpleSyntaxValidator needs to be `mut` for `validate_syntax()`
- Cannot share same validator across async boundaries safely

### 7.4 Stateful Parser Issue

**Problem**: `Parser` is stateful
```rust
pub struct SimpleSyntaxValidator {
    parsers: HashMap<Language, Parser>,  // ← Mutable state
}

pub fn validate_syntax(&mut self, code: &str, language: Language) -> Result<ValidationResult>
    // ↑ Requires &mut
```

**Implication**: Each validator instance needs its own parsers. Cannot share across concurrent tasks without Arc<Mutex>.

---

## Section 8: Breaking Changes from 0.20 to 0.22

### 8.1 Language Constant API Change

**0.20 and earlier**:
```rust
pub const LANGUAGE: Language = Language { /* ... */ };
parser.set_language(&tree_sitter_rust::LANGUAGE)  // Direct reference
```

**0.22+** (some grammars):
```rust
pub fn LANGUAGE() -> Language { /* ... */ }
parser.set_language(&tree_sitter_rust::LANGUAGE())  // Function call
```

**Migration Path**:
- Update all `&LANGUAGE` to `&LANGUAGE()`
- Update all `&LANGUAGE_VARIANT` to `&LANGUAGE_VARIANT()`
- Kotlin was early adopter: `&tree_sitter_kotlin::language()`

### 8.2 Parser Lifecycle

**Both 0.20 and 0.22**: Same pattern
```rust
let mut parser = Parser::new();
parser.set_language(language).ok()  // Must call before parse()
let tree = parser.parse(source, previous_tree)
```

### 8.3 Potential Deprecation Path

**tree-sitter 0.22.x**: Still supports both patterns (likely)
**tree-sitter 0.23+**: Unknown (not yet released widely)

**Current Risk**: Mixing patterns in same codebase will cause compilation errors when dependencies are updated to newer grammar versions.

---

## Section 9: Multi-Language Support Requirements

### 9.1 What's Needed for Full Multi-Language Support

**For Each Language (CppSignature, GoSignature, etc.)**:
1. Create language-specific signature struct in entities.rs
2. Add variant to LanguageSpecificSignature enum
3. Implement tree-sitter AST parsing in PT01
4. Update create_language_signature() in streamer.rs
5. Update TddClassification for language-specific complexity metrics

**Example for C++**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CppSignature {
    pub parameters: Vec<JavaParameter>,  // Reuse if compatible
    pub return_type: String,
    pub template_parameters: Vec<String>,
    pub is_inline: bool,
    pub is_virtual: bool,
    pub is_static: bool,
    pub access_level: AccessModifier,
}
```

### 9.2 Test Detection for Non-Rust Languages

**Current**: Only Rust detects test functions
```rust
fn check_preceding_test_attribute(node: &tree_sitter::Node<'_>, source: &str) -> bool
    // Checks for #[test], #[tokio::test], #[async_test]
```

**Needed for**:
- Python: `def test_*()`, `@pytest.mark.*` decorators
- JavaScript: `test()`, `it()`, `describe()` calls
- Java: `@Test` annotations
- C++: `GTEST_TEST()`, `TEST()` macros
- Go: `func Test*()` naming convention

### 9.3 Dependency Extraction Complexity

**Current (Rust-only)**:
- Extract `call_expression` nodes
- Match callee to known entities
- Create `DependencyEdge` with `EdgeType::Calls`

**Challenges for Other Languages**:
- Python: Method calls via `.` operator, different AST structure
- JavaScript: Dynamic calls, prototypes, hoisting
- Java: Inheritance, interface implementation, method overloading
- C++: Macros expand at preprocessing stage, not visible in AST

**Current Limitation**: No dependencies extracted for non-Rust languages

---

## Section 10: Implementation Patterns Summary

### 10.1 Parser Initialization Patterns

| File | Pattern | Thread-Safe | Notes |
|------|---------|-------------|-------|
| PT01 isgl1_generator.rs | `&LANGUAGE` (constants) | Yes (Arc<Mutex>) | Direct references |
| PT04 simple_validator.rs | `&LANGUAGE()` (functions) | No | Function calls |
| Language Detection | Extension-based | Yes | via Language::from_file_path() |

### 10.2 Entity Extraction Patterns

| Language | Implemented | Entity Types | Test Detection | Dependencies |
|----------|------------|--------------|---|---|
| Rust | Full | Fn, Struct, Enum, Trait, Impl, Mod | Yes (#[test]) | Yes (calls) |
| Python | Stub | None | No | No |
| JavaScript | None | None | No | No |
| TypeScript | None | None | No | No |
| Go | None | None | No | No |
| Java | None | None | No | No |
| C++ | None | None | No | No |
| Ruby | None | None | No | No |
| PHP | None | None | No | No |
| C# | None | None | No | No |
| Swift | None | None | No | No |
| Kotlin | None | None | No | No |
| Scala | None | None | No | No |

### 10.3 Signature Coverage

| Language | Signature Type | Fields | Implementation |
|----------|---|---|---|
| Rust | RustSignature | generics, lifetimes, where_clauses, attributes, trait_impl | Full |
| JavaScript | JavascriptSignature | parameters, return_type, is_async, is_arrow | Full |
| TypeScript | TypeScriptSignature | parameters, return_type, generics, is_async | Full |
| Python | PythonSignature | parameters, return_type, is_async, decorators | Full |
| Java | JavaSignature | access_modifier, parameters, return_type, throws, is_static, generics | Full |
| Others (8) | Fallback RustSignature | Empty/generic | Incomplete |

---

## Section 11: Critical Issues & Risks

### 11.1 API Incompatibility (HIGH RISK)

**Issue**: PT01 and PT04 use different tree-sitter grammar APIs
- PT01: `&LANGUAGE` (constant references)
- PT04: `&LANGUAGE()` (function calls)

**Risk**: 
- Grammar crate updates will break one or both implementations
- Inconsistency will accumulate over time
- Creates maintenance burden

**Mitigation**:
1. Standardize on one pattern (recommend `LANGUAGE()`)
2. Create wrapper abstraction if needed
3. Add CI tests for both patterns

### 11.2 Incomplete Multi-Language Support (MEDIUM RISK)

**Issue**: 8 of 13 languages have parser but no entity extraction
- Parsers initialized but not used
- All non-Rust languages fall back to Rust signature
- Loses language-specific metadata

**Risk**:
- False sense of multi-language support
- Inconsistent semantic information across languages
- Cannot extract dependencies for non-Rust code

**Mitigation**:
- Either: Implement all 13 languages properly
- Or: Document as "Rust-first, others syntax-check only"
- Or: Remove unused language parsers from initialization

### 11.3 Thread-Safety Inconsistency (MEDIUM RISK)

**Issue**: 
- PT01 is async-safe (Arc<Mutex<Parser>>)
- PT04 is single-threaded (plain HashMap<Language, Parser>)

**Risk**:
- PT04 cannot be safely shared in async contexts
- Potential race conditions if used concurrently
- Redesign needed for multi-threaded usage

**Mitigation**:
- PT04: Add Arc<Mutex> wrapper for parsers
- Or: Document as single-threaded only
- Or: Use thread-local storage if appropriate

### 11.4 Duplicate Entity Type Enums (LOW RISK)

**Issue**: Two EntityType definitions
- `isgl1_generator::EntityType` (7 variants)
- `parseltongue_core::entities::EntityType` (14 variants)

**Risk**:
- Manual conversion required (streamer.rs:199-212)
- Missing variants (Macro, ProcMacro, TestFunction, Class, Constant)
- Will cause errors when new variants are added

**Mitigation**:
- Consolidate to single enum in parseltongue-core
- Or: Add missing variants to both enums

---

## Section 12: Architecture Recommendations

### 12.1 Standardize Language Constant API

**Recommendation**: Update PT01 to match PT04 (function calls)

```rust
// Current PT01 (problematic)
init_parser!(Language::Rust, &tree_sitter_rust::LANGUAGE);

// Recommended (future-proof)
init_parser!(Language::Rust, &tree_sitter_rust::LANGUAGE());
```

**Rationale**: 
- Function calls more likely to be supported across future versions
- Kotlin already uses this pattern
- PT04 already uses this pattern (more recent code)

### 12.2 Create Language Parser Abstraction

**Recommendation**: Wrap parser initialization logic

```rust
pub struct LanguageParserFactory;

impl LanguageParserFactory {
    pub fn create_parser(language: Language) -> Result<Parser> {
        let mut parser = Parser::new();
        
        let language_fn = match language {
            Language::Rust => &tree_sitter_rust::LANGUAGE(),
            Language::Python => &tree_sitter_python::LANGUAGE(),
            Language::JavaScript => &tree_sitter_javascript::LANGUAGE(),
            // ... etc
        };
        
        parser.set_language(language_fn)
            .map_err(|e| /* ... */)?;
        Ok(parser)
    }
}
```

**Benefits**:
- Single location for all parser initialization
- Easy to update if API changes
- Reusable across PT01 and PT04

### 12.3 Add Language Support Matrix

**Recommendation**: Document actual support level for each language

```rust
pub struct LanguageSupportMatrix {
    language: Language,
    has_parser: bool,              // Grammar available
    can_extract_entities: bool,    // AST walking implemented
    signature_type: SignatureType, // Language-specific or fallback
    test_detection: bool,          // Can identify tests
    dependency_extraction: bool,   // Can extract calls/uses
}
```

**Rationale**: 
- Prevents false claims of multi-language support
- Enables feature-gating
- Clear upgrade path for future work

### 12.4 Implement Lazy Parser Initialization

**Recommendation**: Don't initialize all 13 parsers at startup

```rust
pub struct LazyParserCache {
    cache: Arc<Mutex<HashMap<Language, Parser>>>,
}

impl LazyParserCache {
    pub async fn get_or_init(&self, language: Language) -> Result<Arc<Mutex<Parser>>> {
        let mut cache = self.cache.lock().await;
        
        if !cache.contains_key(&language) {
            cache.insert(language, Self::create_parser(language)?);
        }
        
        Ok(Arc::new(Mutex::new(cache.get(&language).unwrap().clone())))
    }
}
```

**Benefits**:
- Faster startup (only initialize used languages)
- Lower memory footprint
- Better for tools that only need 1-2 languages

---

## Conclusion

Parseltongue's tree-sitter integration is **functional but incomplete**:

1. **API Status**: Currently using tree-sitter 0.22.6 with API divergence between PT01 and PT04
2. **Language Support**: 13 languages have parsers, but only Rust has full entity extraction
3. **Implementation**: Rust-focused with Python stub, all others fall back to Rust signature
4. **Thread-Safety**: PT01 is async-safe, PT04 is not (needs fixing)
5. **Multi-Language**: Not ready for production multi-language support without significant work

**Recommended Priority**:
1. **HIGH**: Standardize language constant API (function calls)
2. **HIGH**: Add thread-safe wrapper to PT04
3. **MEDIUM**: Implement Python and JavaScript entity extraction
4. **MEDIUM**: Create language support matrix
5. **LOW**: Extend to remaining 8 languages
