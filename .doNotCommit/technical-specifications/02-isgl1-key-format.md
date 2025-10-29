# ISGL1 Key Format Specification

## Overview

ISGL1 (Interface Signature Granular Language 1) keys provide a unique, human-readable identifier for every code entity across the entire codebase. This specification defines the exact format, edge case handling, and collision avoidance strategies for the Parseltongue MVP.

## Base Format

### Standard Format
```
<relative-filepath>-<filename>-<entity-name>
```

### Components

1. **relative-filepath**: Path from repository root to file containing the entity
2. **filename**: Name of the file (without extension)
3. **entity-name**: Name of the interface/function/struct/trait

### Examples

```
src/main.rs-main-main_function
src/lib.rs-lib-MyStruct
src/utils/mod.rs-utils-helper_function
src/models/user.rs-models-User
tests/integration_test.rs-tests-test_user_creation
```

## Format Rules

### Path Normalization

1. **Forward Slashes Only**: Always use `/` regardless of operating system
2. **No Leading Slash**: Relative paths should not start with `/`
3. **No Trailing Slash**: Paths should not end with `/`
4. **No Relative References**: Eliminate `.` and `..` from paths

**Normalization Example**:
```
Input:  "src\\models\\..\\utils\\helper.rs"
Output: "src/utils/helper.rs"
```

### Component Sanitization

1. **Replace Spaces**: Convert spaces to underscores `_`
2. **Remove Special Characters**: Remove characters except alphanumeric, underscore, hyphen
3. **Handle Duplicates**: Append hash suffix if collision occurs
4. **Case Preservation**: Maintain original case for readability

**Sanitization Examples**:
```
Input:  "my function name"
Output: "my_function_name"

Input:  "User-Manager_v2"
Output: "User-Manager_v2"

Input:  "invalid@char#here"
Output: "invalidcharhere"
```

## Language-Specific Patterns

### Rust Entities

#### Functions
```
<path>-<file>-<function_name>
```

**Examples**:
```
src/main.rs-main-main
src/utils/helpers.rs-helpers-calculate_total
src/api/endpoints.rs-endpoints-get_user
```

#### Structs
```
<path>-<file>-<struct_name>
```

**Examples**:
```
src/models/user.rs-models-User
src/config.rs-config-AppConfig
src/errors.rs-errors-APIError
```

#### Traits
```
<path>-<file>-<trait_name>
```

**Examples**:
```
src/traits/database.rs-traits-Database
src/traits/serializable.rs-traits-Serializable
```

#### impl Blocks
```
<path>-<file>-impl_<trait>_for_<struct>
```

**Examples**:
```
src/models/user.rs-models-impl_Display_for_User
src/models/user.rs-models-impl_Default_for_User
src/models/user.rs-models-impl_User
```

#### Modules
```
<path>-<file>-mod_<module_name>
```

**Examples**:
```
src/utils/mod.rs-utils-mod_helpers
src/api/mod.rs-api-mod_endpoints
```

### JavaScript/TypeScript Entities

#### Functions
```
<path>-<file>-<function_name>
```

**Examples**:
```
src/utils.js-utils-calculateTotal
src/api/handlers.js-handlers-getUser
src/components/Button.jsx-components-Button
```

#### Classes
```
<path>-<file>-<class_name>
```

**Examples**:
```
src/models/User.js-models-User
src/services/Database.js-services-Database
```

#### Interfaces (TypeScript)
```
<path>-<file>-interface_<interface_name>
```

**Examples**:
```
src/types/User.ts-types-interface_User
src/types/Config.ts-types-interface_Config
```

### Python Entities

#### Functions
```
<path>-<file>-<function_name>
```

**Examples**:
```
src/utils.py-utils-calculate_total
src/api/handlers.py-handlers-get_user
```

#### Classes
```
<path>-<file>-<class_name>
```

**Examples**:
```
src/models/user.py-models-User
src/services/database.py-services-Database
```

## Edge Case Handling

### Nested Entities

#### Generic Types (Rust)
```
<path>-<file>-<entity_name>_gen_<hash_of_generic_params>
```

**Examples**:
```
src/collections.rs-collections-HashMap_gen_abc123
src/async.rs-async-Result_gen_def456
```

#### Trait Implementations with Generics
```
<path>-<file>-impl_<trait>_for_<struct>_gen_<hash>
```

**Examples**:
```
src/models/user.rs-models-impl_Serialize_for_User_gen_ghi789
src/async/utils.rs-async-utils-impl_Future_for_Result_gen_jkl012
```

### Macro Definitions

#### Declarative Macros
```
<path>-<file>-macro_<macro_name>
```

**Examples**:
```
src/macros.rs-macros-macro_debug_print
src/utils/macros.rs-macros-macro_assert_eq
```

#### Procedural Macros
```
<path>-<file>-proc_macro_<macro_name>
```

**Examples**:
```
src/proc_macros.rs-proc_macros-derive_CustomDebug
src/attrs.rs-attrs-proc_macro_custom_attr
```

### Test Functions

#### Unit Tests
```
<path>-<file>-test_<test_name>
```

**Examples**:
```
src/utils.rs-utils-test_calculate_total
src/models/user.rs-models-test_user_creation
```

#### Integration Tests
```
<path>-<file>-integration_test_<test_name>
```

**Examples**:
```
tests/api_tests.rs-api_tests-integration_test_user_crud
tests/database_tests.rs-database_tests-integration_test_connection_pool
```

## Collision Avoidance

### Collision Detection

When two different entities would generate the same ISGL1 key, use the following strategy:

1. **Detect Collision**: Compare file paths and entity names
2. **Generate Hash**: Create SHA-256 hash of full entity context
3. **Append Hash**: Use first 8 characters of hash as suffix

### Hash Context

The hash is generated from:
```
{
    "file_path": "path/to/file.rs",
    "line_number": 42,
    "entity_type": "function",
    "entity_name": "my_function",
    "signature": "fn my_function(arg1: i32) -> String",
    "module_path": ["module", "submodule"]
}
```

### Collision Resolution Examples

**Before Collision**:
```
src/utils.rs-utils-helper_function
src/sub/utils.rs-utils-helper_function
```

**After Collision Resolution**:
```
src/utils.rs-utils-helper_function_a1b2c3d4
src/sub/utils.rs-utils-helper_function_e5f6g7h8
```

## Reverse Mapping

### ISGL1 to File Location

For operations like code writing and file modification, we need to map ISGL1 keys back to file locations:

**Mapping Format**:
```json
{
    "isgl1_key": "src/models/user.rs-models-User",
    "file_path": "src/models/user.rs",
    "entity_name": "User",
    "entity_type": "struct",
    "line_range": {
        "start": 15,
        "end": 42
    },
    "context": {
        "module_path": ["models"],
        "visibility": "public"
    }
}
```

### Mapping Algorithm

1. **Parse ISGL1**: Extract file path and entity name
2. **Query Database**: Look up entity metadata using ISGL1 key
3. **Extract Location**: Get file path, line numbers, and context information
4. **Validate**: Ensure file still exists and entity is in expected location

## Performance Considerations

### Key Length Optimization

Target ISGL1 key length: **< 100 characters** for most entities

**Optimization Strategies**:
1. **Path Truncation**: For very deep paths, truncate middle sections
2. **Abbreviation**: Use standard abbreviations for common directory names
3. **Hash Only**: For extremely long paths, use hash-based keys

**Example Path Truncation**:
```
Original: src/very/deep/nested/directory/structure/utils/helper.rs
Truncated: src/.../utils/helper.rs
```

### Index Optimization

ISGL1 keys are designed for efficient database indexing:

1. **Prefix Matching**: Enables efficient queries for files in same directory
2. **Lexical Ordering**: Natural sorting for human-readable listings
3. **Hash Distribution**: Ensures even distribution in hash-based indexes

## Validation Rules

### Key Format Validation

An ISGL1 key is valid if:

1. **Format Check**: Matches `<path>-<file>-<entity>` pattern
2. **Character Check**: Contains only allowed characters
3. **Length Check**: Under maximum length limit (255 characters)
4. **Existence Check**: References existing file and entity

### Validation Algorithm

```rust
fn validate_isgl1_key(key: &str) -> Result<(), ValidationError> {
    // 1. Check format
    if !key.contains('-') {
        return Err(ValidationError::InvalidFormat);
    }

    // 2. Check character validity
    if !key.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(ValidationError::InvalidCharacters);
    }

    // 3. Check length
    if key.len() > 255 {
        return Err(ValidationError::TooLong);
    }

    // 4. Check file existence (if validation is runtime)
    // ... file system check

    Ok(())
}
```

## MVP Constraints

Following ultra-minimalist principles for MVP (~10 users):

1. **Simple Format**: Basic `<path>-<file>-<entity>` format only
2. **Manual Collision Handling**: Document collisions clearly, let users resolve
3. **Basic Validation**: Essential validation only, no complex edge case handling
4. **Human Readable**: Prioritize readability over compactness
5. **Case Sensitive**: Maintain case sensitivity for better readability

This ISGL1 specification provides a robust foundation for entity identification while maintaining simplicity and reliability for the MVP target audience.