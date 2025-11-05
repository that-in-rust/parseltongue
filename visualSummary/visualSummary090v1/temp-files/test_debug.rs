use std::path::Path;
use pt01_folder_to_cozodb_streamer::test_detector::{DefaultTestDetector, TestDetector};

fn main() {
    let detector = DefaultTestDetector::new();
    
    // Test with debug test file
    let test_path = Path::new("debug_test.rs");
    let test_content = r#"
#[test]
fn debug_test_function() {
    assert!(true);
}
"#;
    
    let result = detector.detect_test_from_path_and_name(test_path, test_content);
    println!("Debug test file classification: {:?}", result);
    
    // Test with actual test file
    let real_test_path = Path::new("crates/parseltongue-core/tests/swift_declaration_analysis.rs");
    let real_test_content = r#"
/// Analyze how Swift distinguishes between class/struct/enum
use tree_sitter::{Parser, Language};

#[test]
fn analyze_swift_declaration_types() {
    let swift_code = r#"
class MyClass {
    var x: Int
}
"#;
    assert!(true);
}
"#;
    
    let real_result = detector.detect_test_from_path_and_name(real_test_path, real_test_content);
    println!("Real test file classification: {:?}", real_result);
}
