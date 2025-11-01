//! # Simple Syntax Validation Tests (RED → GREEN → REFACTOR)
//!
//! Tests for the simplified Tool 4: tree-sitter syntax validation only

use pt04_syntax_preflight_validator::SimpleSyntaxValidator;

/// Test 1: Valid function syntax should pass
#[test]
fn test_valid_function_syntax() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    let valid_code = r#"
        fn calculate_sum(a: i32, b: i32) -> i32 {
            a + b
        }
    "#;

    let result = validator.validate_syntax(valid_code).expect("Validation failed");
    assert!(result.is_valid, "Valid function should pass syntax check");
    assert!(result.errors.is_empty(), "Should have no errors");
}

/// Test 2: Invalid syntax (missing paren) should fail
#[test]
fn test_invalid_function_syntax_missing_paren() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    let invalid_code = r#"
        fn broken_function( {
            println!("broken");
        }
    "#;

    let result = validator.validate_syntax(invalid_code).expect("Validation failed");
    assert!(!result.is_valid, "Invalid syntax should fail");
    assert!(!result.errors.is_empty(), "Should have syntax errors");
}

/// Test 3: Valid struct syntax should pass
#[test]
fn test_valid_struct_syntax() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    let valid_code = r#"
        pub struct Config {
            pub timeout: u64,
            pub retries: usize,
        }
    "#;

    let result = validator.validate_syntax(valid_code).expect("Validation failed");
    assert!(result.is_valid, "Valid struct should pass syntax check");
    assert!(result.errors.is_empty(), "Should have no errors");
}

/// Test 4: Missing closing brace should fail
#[test]
fn test_invalid_struct_missing_brace() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    let invalid_code = r#"
        pub struct Config {
            pub timeout: u64,
            pub retries: usize,
        // Missing closing brace
    "#;

    let result = validator.validate_syntax(invalid_code).expect("Validation failed");
    assert!(!result.is_valid, "Missing brace should fail");
    assert!(!result.errors.is_empty(), "Should have syntax errors");
}

/// Test 5: Valid impl block should pass
#[test]
fn test_valid_impl_syntax() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    let valid_code = r#"
        impl MyStruct {
            pub fn new() -> Self {
                Self {}
            }
        }
    "#;

    let result = validator.validate_syntax(valid_code).expect("Validation failed");
    assert!(result.is_valid, "Valid impl should pass syntax check");
}

/// Test 6: Multiple entities with valid syntax
#[test]
fn test_multiple_valid_entities() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    let valid_code = r#"
        pub struct User {
            pub name: String,
            pub age: u32,
        }

        impl User {
            pub fn new(name: String, age: u32) -> Self {
                Self { name, age }
            }

            pub fn greet(&self) -> String {
                format!("Hello, I'm {}", self.name)
            }
        }
    "#;

    let result = validator.validate_syntax(valid_code).expect("Validation failed");
    assert!(result.is_valid, "Multiple valid entities should pass");
    assert!(result.errors.is_empty());
}

/// Test 7: Type error should PASS syntax check (not our responsibility)
#[test]
fn test_type_error_passes_syntax_check() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    // This has a type error (returns i32, not String) but is syntactically valid
    let type_error_code = r#"
        fn broken() -> String {
            42  // Type error, but syntax is valid
        }
    "#;

    let result = validator.validate_syntax(type_error_code).expect("Validation failed");
    assert!(
        result.is_valid,
        "Type errors should pass syntax validation (cargo catches these)"
    );
}

/// Test 8: Import errors should PASS syntax check
#[test]
fn test_import_error_passes_syntax_check() {
    let mut validator = SimpleSyntaxValidator::new().expect("Failed to create validator");

    // This has an import error (module doesn't exist) but is syntactically valid
    let import_error_code = r#"
        use nonexistent::Module;

        fn test() {}
    "#;

    let result = validator.validate_syntax(import_error_code).expect("Validation failed");
    assert!(
        result.is_valid,
        "Import errors should pass syntax validation (cargo catches these)"
    );
}
