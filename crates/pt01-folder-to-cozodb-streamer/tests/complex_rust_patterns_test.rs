//! Complex Rust Pattern Coverage Tests
//!
//! Tests for advanced dependency extraction patterns based on AST exploration.
//! See docs/TESTING_COMPLEX_RUST_PATTERNS.md for detailed analysis.

use pt01_folder_to_cozodb_streamer::{StreamerConfig, ToolFactory, FileStreamer};
use parseltongue_core::storage::CozoDbStorage;
use parseltongue_core::entities::DependencyEdge;
use tempfile::TempDir;

/// Test Pattern 1: Nested calls in struct construction
///
/// Pattern: Self { field: helper_call() }
/// Expected: helper_call() should be captured
/// Known issue: Attributed to impl:Config instead of method:new
#[tokio::test]
async fn test_nested_call_in_struct_construction() {
    let source = r#"
struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Self {
            settings: create_defaults(),
        }
    }
}

fn create_defaults() -> HashMap<String, String> {
    HashMap::new()
}
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Nested Call in Struct Construction ===");
    print_dependencies(&deps);

    // Verify the call is captured
    assert!(
        has_call(&deps, "create_defaults"),
        "Should capture create_defaults() call in struct construction"
    );

    // Document current attribution behavior
    let source_entity = find_call_source(&deps, "create_defaults");
    println!("Attribution: {:?}", source_entity);
    println!("Note: Currently attributed to impl:Config, will be method:new after enhancement");
}

/// Test Pattern 2: Chained method calls
///
/// Pattern: obj.method1().method2().method3()
/// Expected: All method calls in chain captured
#[tokio::test]
async fn test_chained_method_calls() {
    let source = r#"
fn process_users(users: Vec<User>) -> Vec<String> {
    users
        .iter()
        .filter(|u| validate(u))
        .map(|u| transform(u))
        .collect()
}

fn validate(u: &User) -> bool { true }
fn transform(u: &User) -> String { String::new() }
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Chained Method Calls ===");
    print_dependencies(&deps);

    // All method calls should be captured
    assert!(has_call(&deps, "iter"), "Should capture iter()");
    assert!(has_call(&deps, "filter"), "Should capture filter()");
    assert!(has_call(&deps, "map"), "Should capture map()");
    assert!(has_call(&deps, "collect"), "Should capture collect()");

    // Calls inside closures should also be captured
    assert!(has_call(&deps, "validate"), "Should capture validate() in closure");
    assert!(has_call(&deps, "transform"), "Should capture transform() in closure");
}

/// Test Pattern 3: Calls in control flow (if/else)
///
/// Pattern: if cond() { action1() } else { action2() }
/// Expected: All calls in all branches captured
#[tokio::test]
async fn test_calls_in_if_else_branches() {
    let source = r#"
fn handle_request(status: i32) -> i32 {
    if validate(status) {
        process(status)
    } else {
        fallback()
    }
}

fn validate(x: i32) -> bool { true }
fn process(x: i32) -> i32 { x }
fn fallback() -> i32 { 0 }
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Calls in If/Else Branches ===");
    print_dependencies(&deps);

    assert!(has_call(&deps, "validate"), "Should capture validate() in condition");
    assert!(has_call(&deps, "process"), "Should capture process() in if branch");
    assert!(has_call(&deps, "fallback"), "Should capture fallback() in else branch");
}

/// Test Pattern 4: Calls in match arms
///
/// Pattern: match x { A => fn1(), B => fn2() }
/// Expected: All calls in all arms captured
#[tokio::test]
async fn test_calls_in_match_arms() {
    let source = r#"
enum Status {
    Ok,
    Error,
    Pending,
}

fn handle(status: Status) {
    match status {
        Status::Ok => process_success(),
        Status::Error => handle_error(),
        Status::Pending => queue_retry(),
    }
}

fn process_success() {}
fn handle_error() {}
fn queue_retry() {}
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Calls in Match Arms ===");
    print_dependencies(&deps);

    assert!(has_call(&deps, "process_success"), "Should capture process_success() in match arm");
    assert!(has_call(&deps, "handle_error"), "Should capture handle_error() in match arm");
    assert!(has_call(&deps, "queue_retry"), "Should capture queue_retry() in match arm");
}

/// Test Pattern 5: Calls in loop bodies
///
/// Pattern: for x in items { process(x) }
/// Expected: All calls in loop body captured
#[tokio::test]
async fn test_calls_in_loop_bodies() {
    let source = r#"
fn batch_process(items: &[i32]) {
    for item in items {
        validate(item);
        transform(item);
        store(item);
    }
}

fn validate(x: &i32) {}
fn transform(x: &i32) {}
fn store(x: &i32) {}
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Calls in Loop Bodies ===");
    print_dependencies(&deps);

    assert!(has_call(&deps, "validate"), "Should capture validate() in loop");
    assert!(has_call(&deps, "transform"), "Should capture transform() in loop");
    assert!(has_call(&deps, "store"), "Should capture store() in loop");
}

/// Test Pattern 6: Multiple nested levels
///
/// Pattern: outer(inner(deepest()))
/// Expected: All nested calls captured
#[tokio::test]
async fn test_deeply_nested_calls() {
    let source = r#"
fn compute() -> i32 {
    outer(middle(inner(leaf())))
}

fn outer(x: i32) -> i32 { x }
fn middle(x: i32) -> i32 { x }
fn inner(x: i32) -> i32 { x }
fn leaf() -> i32 { 42 }
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Deeply Nested Calls ===");
    print_dependencies(&deps);

    assert!(has_call(&deps, "outer"), "Should capture outer()");
    assert!(has_call(&deps, "middle"), "Should capture middle()");
    assert!(has_call(&deps, "inner"), "Should capture inner()");
    assert!(has_call(&deps, "leaf"), "Should capture leaf()");
}

/// Test Pattern 7: Calls in closure bodies
///
/// Pattern: items.map(|x| { helper(x); transformer(x) })
/// Expected: All calls inside closure captured
#[tokio::test]
async fn test_calls_in_closure_bodies() {
    let source = r#"
fn transform_all(items: Vec<i32>) -> Vec<i32> {
    items.iter().map(|x| {
        let validated = validate(*x);
        let processed = process(validated);
        finalize(processed)
    }).collect()
}

fn validate(x: i32) -> i32 { x }
fn process(x: i32) -> i32 { x }
fn finalize(x: i32) -> i32 { x }
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Calls in Closure Bodies ===");
    print_dependencies(&deps);

    assert!(has_call(&deps, "validate"), "Should capture validate() in closure");
    assert!(has_call(&deps, "process"), "Should capture process() in closure");
    assert!(has_call(&deps, "finalize"), "Should capture finalize() in closure");
}

/// Test Known Limitation: Macro-wrapped calls
///
/// Pattern: println!("{}", helper())
/// Expected: println! captured, but helper() NOT captured (tree-sitter limitation)
#[tokio::test]
async fn test_known_limitation_macro_wrapped_calls() {
    let source = r#"
fn debug_demo() {
    println!("{}", expensive_computation());
    vec![factory(), factory()];
}

fn expensive_computation() -> i32 { 42 }
fn factory() -> i32 { 0 }
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Macro-Wrapped Calls (Known Limitation) ===");
    print_dependencies(&deps);

    // Document the limitation
    let has_expensive = has_call(&deps, "expensive_computation");
    let has_factory = has_call(&deps, "factory");

    println!("\nKNOWN LIMITATION: Calls inside macro token trees are not captured by tree-sitter");
    println!("  - expensive_computation() captured: {}", has_expensive);
    println!("  - factory() captured: {}", has_factory);
    println!("  - This is a tree-sitter architectural constraint, not a bug");

    // Don't assert - just document
    if has_expensive || has_factory {
        println!("  - Unexpectedly captured! Tree-sitter behavior may have changed.");
    }
}

/// Test Edge Case: Self calls and method calls on self
///
/// Pattern: self.method() and Self::associated()
/// Expected: Both captured
#[tokio::test]
async fn test_self_method_calls() {
    let source = r#"
struct Calculator {
    value: i32,
}

impl Calculator {
    fn new() -> Self {
        Self::default()
    }

    fn default() -> Self {
        Self { value: 0 }
    }

    fn compute(&self) -> i32 {
        self.helper()
    }

    fn helper(&self) -> i32 {
        self.value
    }
}
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Self and Method Calls ===");
    print_dependencies(&deps);

    // Should capture Self::default() and self.helper()
    assert!(has_call(&deps, "default"), "Should capture Self::default()");
    assert!(has_call(&deps, "helper"), "Should capture self.helper()");
}

/// Test Edge Case: Trait method calls
///
/// Pattern: value.trait_method()
/// Expected: Syntactic capture (but not trait resolution)
#[tokio::test]
async fn test_trait_method_calls() {
    let source = r#"
fn stringify(value: i32) -> String {
    let s = value.to_string();
    let c = s.clone();
    c
}
"#;

    let deps = extract_dependencies(source).await;

    println!("\n=== Trait Method Calls ===");
    print_dependencies(&deps);

    // Should capture method calls syntactically
    assert!(has_call(&deps, "to_string"), "Should capture to_string() call");
    assert!(has_call(&deps, "clone"), "Should capture clone() call");

    println!("Note: Trait resolution (which trait provides the method) requires semantic analysis");
}

// =============================================================================
// Helper Functions
// =============================================================================

async fn extract_dependencies(source: &str) -> Vec<DependencyEdge> {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    std::fs::write(&test_file, source).unwrap();

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
        streamer.stream_directory().await.unwrap();
    }

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    storage.get_all_dependencies().await.unwrap()
}

fn has_call(deps: &[DependencyEdge], target: &str) -> bool {
    deps.iter().any(|d| d.to_key.as_str().contains(target))
}

fn find_call_source(deps: &[DependencyEdge], target: &str) -> Option<String> {
    deps.iter()
        .find(|d| d.to_key.as_str().contains(target))
        .map(|d| d.from_key.as_str().to_string())
}

fn print_dependencies(deps: &[DependencyEdge]) {
    println!("Dependencies ({})", deps.len());
    for (i, dep) in deps.iter().enumerate() {
        let edge_type = match dep.edge_type {
            parseltongue_core::entities::EdgeType::Calls => "Calls",
            parseltongue_core::entities::EdgeType::Uses => "Uses",
            parseltongue_core::entities::EdgeType::Implements => "Implements",
        };
        println!("  {}. {} -> {} ({})",
            i + 1,
            dep.from_key.as_str().split(':').nth(2).unwrap_or("?"),
            dep.to_key.as_str().split(':').nth(2).unwrap_or("?"),
            edge_type
        );
    }
}
