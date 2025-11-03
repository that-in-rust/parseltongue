/// Debug test to diagnose Swift query compilation error
use tree_sitter::{Query, Language};

#[test]
fn test_swift_query_compilation() {
    let swift_query = include_str!("../../../entity_queries/swift.scm");
    let swift_lang: Language = tree_sitter_swift::LANGUAGE.into();

    println!("\n=== Swift Query Content ===");
    println!("{}", swift_query);
    println!("=== End Query Content ===\n");

    match Query::new(&swift_lang, swift_query) {
        Ok(query) => {
            println!("✅ Swift query compiled successfully!");
            println!("Number of patterns: {}", query.pattern_count());
            println!("Capture names: {:?}", query.capture_names());
        }
        Err(e) => {
            panic!("❌ Swift query compilation FAILED: {:?}", e);
        }
    }
}

#[test]
fn test_ruby_query_compilation_baseline() {
    // Ruby works - use as baseline
    let ruby_query = include_str!("../../../entity_queries/ruby.scm");
    let ruby_lang: Language = tree_sitter_ruby::LANGUAGE.into();

    println!("\n=== Ruby Query Content (Working Baseline) ===");
    println!("{}", ruby_query);
    println!("=== End Query Content ===\n");

    match Query::new(&ruby_lang, ruby_query) {
        Ok(query) => {
            println!("✅ Ruby query compiled successfully!");
            println!("Number of patterns: {}", query.pattern_count());
            println!("Capture names: {:?}", query.capture_names());
        }
        Err(e) => {
            panic!("❌ Ruby query compilation FAILED: {:?}", e);
        }
    }
}
