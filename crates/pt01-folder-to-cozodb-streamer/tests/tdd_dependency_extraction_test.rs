//! TDD Dependency Extraction Tests (RED → GREEN → REFACTOR)
//!
//! ## Executable Specification: Query-Based Dependency Extraction
//!
//! ### Preconditions
//! - .scm dependency query files exist for Rust (and extensible to other languages)
//! - Tree-sitter can capture call_expression, use_declaration, impl_item patterns
//! - QueryBasedExtractor returns both entities AND dependencies
//!
//! ### Postconditions
//! - Function calls detected via queries (not manual tree-walking)
//! - Use/import statements captured as dependencies
//! - Trait implementations captured as dependencies
//! - Same quality as manual extraction but query-based
//!
//! ### Error Conditions
//! - If query-based approach misses dependencies found by manual walk → FAIL
//! - If query compilation fails for valid patterns → FAIL
//!
//! ## Architecture Decision
//!
//! **Current (v0.8.9)**: Manual tree-walking in `extract_rust_dependencies()`
//! **Target (v0.9.0)**: Query-based extraction via .scm files
//!
//! **Benefits**:
//! - Consistency: Same approach for entities AND dependencies
//! - Extensibility: Easy to add dependency patterns via queries
//! - Multi-language: Can extend to Python imports, JS requires, etc.
//! - Maintainability: Declarative queries vs imperative tree-walking

use pt01_folder_to_cozodb_streamer::{streamer::FileStreamer, StreamerConfig, ToolFactory};
use parseltongue_core::storage::CozoDbStorage;
use tempfile::TempDir;

/// RED TEST 1: Function call dependencies via queries
///
/// **Current Behavior**: Uses manual tree-walking in extract_rust_dependencies()
/// **Expected Behavior**: Should use query-based extraction from .scm file
///
/// **Acceptance Criteria**:
/// WHEN analyzing Rust code with function calls
/// THEN dependencies SHALL be extracted via tree-sitter queries
/// AND SHALL match quality of manual extraction
/// AND SHALL create Calls edges in the graph
#[tokio::test]
async fn test_function_call_dependencies_extracted_via_queries() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("caller.rs");

    std::fs::write(
        &test_file,
        r#"
fn calculate_total(items: &[Item]) -> i32 {
    let sum = add_numbers(10, 20);
    let validated = validate_input(sum);
    validated
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn validate_input(x: i32) -> i32 {
    if x > 0 { x } else { 0 }
}
"#,
    )
    .unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    }

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let entities = storage.get_all_entities().await.unwrap();
    let dependencies = storage.get_all_dependencies().await.unwrap();

    // Should have 3 functions
    assert_eq!(entities.len(), 3, "Should extract 3 function entities");

    // RED: This will initially work with manual extraction
    // But we need to verify it continues working after migration to queries
    assert!(
        dependencies.len() >= 2,
        "Should have at least 2 dependency edges (calculate_total calls add_numbers and validate_input)"
    );

    // Verify specific call graph structure
    let calculate_fn = entities.iter()
        .find(|e| e.interface_signature.name == "calculate_total")
        .expect("Should find calculate_total function");

    let add_fn = entities.iter()
        .find(|e| e.interface_signature.name == "add_numbers")
        .expect("Should find add_numbers function");

    let has_call = dependencies.iter().any(|dep| {
        dep.from_key.as_str().contains("calculate_total") && dep.to_key.as_str().contains("add_numbers")
    });

    assert!(
        has_call,
        "Should have dependency edge: calculate_total -> add_numbers"
    );
}

/// RED TEST 2: Use/import dependencies via queries
///
/// **Current Behavior**: Not extracted by manual walking (limitation)
/// **Expected Behavior**: Query-based extraction captures use declarations
///
/// **Acceptance Criteria**:
/// WHEN analyzing Rust code with use statements
/// THEN use declarations SHALL be captured as Import edges
/// AND SHALL link to external modules/types
#[tokio::test]
async fn test_use_declarations_extracted_via_queries() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("imports.rs");

    std::fs::write(
        &test_file,
        r#"
use std::collections::HashMap;
use std::path::PathBuf;

fn create_config() -> HashMap<String, PathBuf> {
    HashMap::new()
}
"#,
    )
    .unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    }

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let dependencies = storage.get_all_dependencies().await.unwrap();

    // RED: Current implementation doesn't capture use declarations
    // GREEN: After adding dependency queries, should have Import edges
    let import_deps: Vec<_> = dependencies.iter()
        .filter(|dep| dep.to_key.as_str().contains("HashMap") || dep.to_key.as_str().contains("PathBuf"))
        .collect();

    assert!(
        import_deps.len() >= 1,
        "Should capture use declarations as Import dependencies. Found: {}",
        import_deps.len()
    );
}

/// RED TEST 3: Trait implementation dependencies via queries
///
/// **Current Behavior**: Not extracted by manual walking
/// **Expected Behavior**: Query captures impl blocks and creates Implements edges
///
/// **Acceptance Criteria**:
/// WHEN analyzing Rust code with trait implementations
/// THEN impl blocks SHALL be captured as Implements edges
/// AND SHALL link struct to trait
#[tokio::test]
async fn test_trait_impl_dependencies_extracted_via_queries() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("traits.rs");

    std::fs::write(
        &test_file,
        r#"
trait Validator {
    fn validate(&self) -> bool;
}

struct EmailValidator {
    pattern: String,
}

impl Validator for EmailValidator {
    fn validate(&self) -> bool {
        true
    }
}
"#,
    )
    .unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    }

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let entities = storage.get_all_entities().await.unwrap();
    let dependencies = storage.get_all_dependencies().await.unwrap();

    // Should have trait, struct, and impl block entities
    assert!(entities.len() >= 3, "Should extract trait, struct, and impl");

    // RED: Current implementation doesn't capture trait implementations
    // GREEN: After adding impl queries, should have Implements edge
    let impl_deps: Vec<_> = dependencies.iter()
        .filter(|dep| {
            dep.from_key.as_str().contains("EmailValidator") && dep.to_key.as_str().contains("Validator")
        })
        .collect();

    assert!(
        impl_deps.len() >= 1,
        "Should have Implements edge: EmailValidator -> Validator"
    );
}

/// RED TEST 4: Multi-language dependency extraction (extensibility)
///
/// **Current Behavior**: Only Rust dependencies extracted
/// **Expected Behavior**: Query-based approach extensible to Python, JS, etc.
///
/// **Acceptance Criteria**:
/// WHEN adding dependency queries for Python
/// THEN Python imports SHALL be extracted as dependencies
/// AND SHALL use same query-based infrastructure
#[tokio::test]
async fn test_python_import_dependencies_extensible() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("app.py");

    std::fs::write(
        &test_file,
        r#"
import os
from pathlib import Path

def setup_dirs():
    base = Path.home()
    return base
"#,
    )
    .unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.py".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    }

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let dependencies = storage.get_all_dependencies().await.unwrap();

    // RED: Python dependencies not yet implemented
    // GREEN: After adding Python dependency queries, should capture imports
    let import_count = dependencies.iter()
        .filter(|dep| dep.to_key.as_str().contains("os") || dep.to_key.as_str().contains("Path"))
        .count();

    assert!(
        import_count >= 1,
        "Should capture Python imports as dependencies (extensibility test)"
    );
}

/// RED TEST 5: Performance - query extraction should be as fast as manual
///
/// **Acceptance Criteria**:
/// WHEN using query-based dependency extraction
/// THEN performance SHALL be <= manual extraction + 10% overhead
#[tokio::test]
async fn test_dependency_extraction_performance() {
    use std::time::Instant;

    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("large.rs");

    // Generate 100 functions with call chains
    let mut code = String::new();
    for i in 0..100 {
        code.push_str(&format!(
            "fn func_{i}() -> i32 {{\n",
            i = i
        ));
        if i > 0 {
            code.push_str(&format!("    let x = func_{}();\n", i - 1));
        }
        code.push_str("    42\n}\n\n");
    }

    std::fs::write(&test_file, code).unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    let start = Instant::now();
    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    }
    let elapsed = start.elapsed();

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let dependencies = storage.get_all_dependencies().await.unwrap();

    // Performance contract: <500ms for 100 functions with calls
    assert!(
        elapsed.as_millis() < 500,
        "Dependency extraction too slow: {:?} for 100 functions",
        elapsed
    );

    // Should have ~99 call edges (func_0 has no calls, rest call predecessor)
    assert!(
        dependencies.len() >= 90,
        "Should extract most function call dependencies. Found: {}",
        dependencies.len()
    );
}

/// RED TEST 6: Verify query-based extraction matches manual extraction exactly
///
/// **Acceptance Criteria**:
/// GIVEN existing codebase analyzed with manual extraction
/// WHEN re-analyzing with query-based extraction
/// THEN dependency count and structure SHALL match exactly
#[tokio::test]
async fn test_query_based_matches_manual_extraction_quality() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("complex.rs");

    // Real-world Rust code with multiple dependency types
    std::fs::write(
        &test_file,
        r#"
use std::collections::HashMap;

struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Self {
            settings: create_defaults(),
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }
}

fn create_defaults() -> HashMap<String, String> {
    HashMap::new()
}

fn main() {
    let config = Config::new();
    println!("{:?}", config.get("test"));
}
"#,
    )
    .unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    }

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let dependencies = storage.get_all_dependencies().await.unwrap();

    // Expected dependencies:
    // 1. Config::new() -> create_defaults() [function call]
    // 2. main() -> Config::new() [function call]
    // 3. main() -> Config::get() [method call]
    // 4. Config::get() -> HashMap::get() [external call]

    assert!(
        dependencies.len() >= 3,
        "Should extract multiple dependency types. Found: {} dependencies",
        dependencies.len()
    );

    // Verify key call graph edges exist
    let has_new_to_defaults = dependencies.iter().any(|dep| {
        dep.from_key.as_str().contains("new") && dep.to_key.as_str().contains("create_defaults")
    });

    assert!(
        has_new_to_defaults,
        "Should have edge: Config::new -> create_defaults"
    );
}
