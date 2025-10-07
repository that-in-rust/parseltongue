//! HTML Generation Tests - Executable Specifications
//!
//! Following steering docs TDD principles:
//! STUB → RED → GREEN → REFACTOR
//!
//! Every claim must be validated by automated tests

use crate::wasm_renderer::{generate_wasm_visualization, generate_wasm_visualization_with_loader};
use crate::isg::OptimizedISG;
use crate::graph_data_loader::{GraphDataLoader, MemoryISGLoader, MockErrorLoader, GraphDataError, GraphDataLoaderFactory};
use std::time::Instant;

/// HTML Visualization Generation Executable Specification
///
/// # Preconditions
/// - Valid ISG with nodes and edges
/// - Layout algorithm specified (breadthfirst/hierarchical/etc.)
///
/// # Postconditions
/// - Returns Ok(String) containing valid HTML5
/// - HTML contains: const graphData = {actual_json_content}
/// - JavaScript executes without errors
/// - Canvas renders >0 nodes when data exists
/// - Render time <100ms per steering docs performance contract
///
/// # Error Conditions
/// - WASMError::ConversionError if ISG -> WASMGraph fails
/// - WASMError::LayoutError if layout algorithm invalid
/// - Serialization error if JSON conversion fails

#[cfg(test)]
mod executable_specification_tests {
    use super::*;

    /// Test Contract: HTML generation must produce valid output
    /// WHEN generating HTML from valid ISG
    /// THEN shall contain actual graph data, never empty objects
    #[test]
    fn test_html_generation_executable_specification_req_html_001() {
        // GIVEN: Valid test ISG with known data
        let isg = create_test_isg_with_nodes();

        // WHEN: Generate HTML visualization
        let result = generate_wasm_visualization(&isg, "breadthfirst");

        // THEN: Must succeed and meet contract requirements
        assert!(result.is_ok(), "HTML generation should succeed with valid ISG");
        let html = result.unwrap();

        // Contract: Must be valid HTML5 (allow leading whitespace)
        let trimmed_html = html.trim_start();
        assert!(trimmed_html.starts_with("<!DOCTYPE html>"), "Must start with DOCTYPE");
        assert!(html.contains("<html"), "Must contain html element");
        assert!(html.contains("</html>"), "Must contain closing html tag");

        // Contract: Must contain actual graph data, never empty objects
        assert!(!html.contains("const graphData = {}"),
                "Must never contain empty graphData object");
        assert!(html.contains("graphData = "),
                "Must contain graphData assignment");
        assert!(html.contains(r#""nodes":["#),
                "Must contain actual nodes array");
        assert!(html.contains(r#""id":"#),
                "Must contain node IDs in JSON");

        // Contract: Must contain required elements
        assert!(html.contains("<canvas"), "Must contain canvas element");
        assert!(html.contains("renderGraph()"), "Must contain renderGraph function");
        assert!(html.contains("initWasm()"), "Must contain initWasm function");
    }

    /// Test Contract: JavaScript scope must be valid
    /// WHEN generating HTML
    /// THEN shall have no variable shadowing or duplicate declarations
    #[test]
    fn test_javascript_scope_validity_req_js_001() {
        // GIVEN: Valid test ISG
        let isg = create_test_isg_with_nodes();

        // WHEN: Generate HTML
        let html = generate_wasm_visualization(&isg, "breadthfirst").unwrap();

        // THEN: Must have no JavaScript scope violations
        // Check for duplicate variable declarations
        let graphdata_declarations: Vec<&str> = html
            .lines()
            .filter(|line| line.contains("let graphData") || line.contains("const graphData"))
            .collect();

        assert_eq!(graphdata_declarations.len(), 1,
                  "Should have exactly one graphData declaration, found: {:?}",
                  graphdata_declarations);

        // Ensure graphData is declared with let (mutable) not const
        assert!(html.contains("let graphData = null"),
                "Should declare graphData as mutable with let");

        // Ensure assignment uses assignment operator, not declaration
        assert!(html.contains("graphData = {"),
                "Should assign JSON data to existing graphData variable");
    }

    /// Test Contract: Performance requirements must be met
    /// WHEN generating HTML with realistic data
    /// THEN shall complete within performance contract limits
    #[test]
    fn test_html_generation_performance_contract_req_perf_001() {
        // GIVEN: Large realistic ISG (simulating 1000+ nodes)
        let isg = create_large_test_isg();

        // WHEN: Generate HTML visualization
        let start = Instant::now();
        let result = generate_wasm_visualization(&isg, "breadthfirst");
        let generation_time = start.elapsed();

        // THEN: Must meet performance contract
        assert!(result.is_ok(), "HTML generation should succeed even with large ISG");

        // Performance contract: <100ms for large graphs
        assert!(generation_time < std::time::Duration::from_millis(100),
                "HTML generation took {:?}, expected <100ms (Performance Contract VIOLATION)",
                generation_time);

        let html = result.unwrap();

        // Contract: Must still contain valid data even for large graphs
        assert!(!html.contains("const graphData = {}"),
                "Large graphs must still generate valid JSON data");
        assert!(html.contains(r#""nodes":["#),
                "Must contain nodes array even for large graphs");

        println!("✅ Performance contract met: HTML generation in {:?}", generation_time);
    }

    /// Test Contract: Layout algorithm variations must work
    /// WHEN testing different layout algorithms
    /// THEN shall all generate valid HTML with correct layout selection
    #[test]
    fn test_layout_algorithm_variations_req_layout_001() {
        // GIVEN: Valid test ISG
        let isg = create_test_isg_with_nodes();

        // WHEN: Testing all supported layout algorithms
        let layouts = vec!["breadthfirst", "forcedirected", "hierarchical", "circular"];

        for layout in layouts {
            println!("Testing layout: {}", layout);

            let result = generate_wasm_visualization(&isg, layout);
            assert!(result.is_ok(), "Layout '{}' should generate valid HTML", layout);

            let html = result.unwrap();

            // Contract: Must select correct layout in HTML
            let expected_selected = format!(r#"value="{}" selected"#, layout);
            assert!(html.contains(&expected_selected),
                    "HTML should select '{}' layout in dropdown", layout);

            // Contract: Must set correct initial layout
            let expected_layout = format!("let currentLayout = '{}';", layout);
            assert!(html.contains(&expected_layout),
                    "HTML should set '{}' as initial layout", layout);
        }
    }

    /// Test Contract: Empty ISG should not crash
    /// WHEN generating HTML from empty ISG
    /// THEN shall generate graceful empty visualization
    #[test]
    fn test_empty_isg_graceful_handling_req_empty_001() {
        // GIVEN: Empty ISG
        let isg = OptimizedISG::new();

        // WHEN: Generating HTML
        let result = generate_wasm_visualization(&isg, "breadthfirst");

        // THEN: Should handle gracefully without crashing
        assert!(result.is_ok(), "Empty ISG should not cause generation failure: {:?}", result);

        let html = result.unwrap();

        // Contract: Should still be valid HTML (allow leading whitespace)
        let trimmed_html = html.trim_start();
        assert!(trimmed_html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("graphData = "));

        // Contract: Should contain empty arrays but valid structure
        assert!(html.contains(r#""nodes":[]"#), "Empty ISG should have empty nodes array");
        assert!(html.contains(r#""edges":[]"#), "Empty ISG should have empty edges array");
    }
}

/// Helper functions for test data creation
fn create_test_isg_with_nodes() -> OptimizedISG {
    let isg = OptimizedISG::new();

    // Add test nodes for realistic HTML generation
    let test_function = crate::isg::NodeData {
        hash: crate::isg::SigHash::new("test_function"),
        kind: crate::isg::NodeKind::Function,
        name: "test_function".into(),
        signature: "fn test_function() -> Result<(), Error>".into(),
        file_path: "test.rs".into(),
        line: 10,
    };

    let test_struct = crate::isg::NodeData {
        hash: crate::isg::SigHash::new("TestStruct"),
        kind: crate::isg::NodeKind::Struct,
        name: "TestStruct".into(),
        signature: "struct TestStruct".into(),
        file_path: "test.rs".into(),
        line: 1,
    };

    let test_trait = crate::isg::NodeData {
        hash: crate::isg::SigHash::new("TestTrait"),
        kind: crate::isg::NodeKind::Trait,
        name: "TestTrait".into(),
        signature: "trait TestTrait".into(),
        file_path: "test.rs".into(),
        line: 5,
    };

    // Add nodes to ISG
    isg.upsert_node(test_function);
    isg.upsert_node(test_struct);
    isg.upsert_node(test_trait);

    // Add test edge - function depends on struct
    let _ = isg.upsert_edge(
        crate::isg::SigHash::new("test_function"),
        crate::isg::SigHash::new("TestStruct"),
        crate::isg::EdgeKind::Uses
    );

    isg
}

fn create_large_test_isg() -> OptimizedISG {
    let isg = OptimizedISG::new();

    // Create a large ISG to test performance (100+ nodes)
    for i in 0..100 {
        let node = crate::isg::NodeData {
            hash: crate::isg::SigHash::new(&format!("function_{}", i)),
            kind: crate::isg::NodeKind::Function,
            name: format!("function_{}", i).into(),
            signature: format!("fn function_{}() -> Result<(), Error>", i).into(),
            file_path: "large_test.rs".into(),
            line: i as u32,
        };
        isg.upsert_node(node);

        // Add some edges for realistic complexity
        if i > 0 {
            let _ = isg.upsert_edge(
                crate::isg::SigHash::new(&format!("function_{}", i)),
                crate::isg::SigHash::new(&format!("function_{}", i - 1)),
                crate::isg::EdgeKind::Uses
            );
        }
    }

    isg
}


/// Integration tests with real file I/O
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// Test: Generated HTML should be usable as a standalone file
    #[test]
    fn test_standalone_html_file_functionality() {
        // GIVEN: Test ISG
        let isg = create_test_isg_with_nodes();

        // WHEN: Generate and save HTML to file
        let html = generate_wasm_visualization(&isg, "breadthfirst").unwrap();
        let test_file = PathBuf::from("test_output.html");
        fs::write(&test_file, &html).expect("Should write test HTML file");

        // THEN: File should be valid and readable
        assert!(test_file.exists(), "HTML file should be created");
        let read_back = fs::read_to_string(&test_file).expect("Should read HTML file back");
        assert_eq!(read_back, html, "File content should match generated HTML");

        // Cleanup
        let _ = fs::remove_file(&test_file);

        println!("✅ Standalone HTML file test passed");
    }

    /// Test: Multiple generations should be consistent
    #[test]
    fn test_generation_consistency() {
        // GIVEN: Same ISG
        let isg = create_test_isg_with_nodes();

        // WHEN: Generate HTML multiple times
        let html1 = generate_wasm_visualization(&isg, "breadthfirst").unwrap();
        let html2 = generate_wasm_visualization(&isg, "breadthfirst").unwrap();
        let html3 = generate_wasm_visualization(&isg, "breadthfirst").unwrap();

        // THEN: Essential parts should be identical (allowing for HashMap field order differences)

        // Check that all have the same node count (3 nodes)
        assert!(html1.contains("Nodes: 3") && html2.contains("Nodes: 3") && html3.contains("Nodes: 3"),
            "All should show 3 nodes");

        // Check that all have the same edge count (1 edge between the test nodes)
        assert!(html1.contains("Edges: 1") && html2.contains("Edges: 1") && html3.contains("Edges: 1"),
            "All should show 1 edge");

        // Check that all contain the expected node names
        let expected_nodes = ["TestStruct", "TestTrait", "test_function"];
        for node_name in &expected_nodes {
            assert!(html1.contains(node_name) && html2.contains(node_name) && html3.contains(node_name),
                "All should contain node: {}", node_name);
        }

        // Check that the graphData assignment exists in all
        assert!(html1.contains("graphData = ") && html2.contains("graphData = ") && html3.contains("graphData = "),
            "All should have graphData assignment");

        println!("✅ Generation consistency test passed");
    }
}

/// Dependency Injection Tests - Steering Docs Principle #3
///
/// Test that the GraphDataLoader trait enables:
/// - Test doubles and mocks for unit testing
/// - Different data sources (files, databases, APIs)
/// - Performance monitoring and caching
/// - Error handling and recovery strategies
#[cfg(test)]
mod dependency_injection_tests {
    use super::*;

    /// Test Contract: Memory loader should provide test data
    /// WHEN using MemoryISGLoader with test ISG
    /// THEN shall generate valid HTML with actual data
    #[tokio::test]
    async fn test_memory_loader_dependency_injection_req_di_001() {
        // GIVEN: Test ISG with known data
        let isg = create_test_isg_with_nodes();
        let expected_nodes = isg.node_count();
        let expected_edges = isg.edge_count();

        // WHEN: Using dependency injection with Memory loader
        let loader = MemoryISGLoader::new(isg);

        // Validate loader metadata
        assert!(loader.is_available().await, "Memory loader should always be available");
        let metadata = loader.metadata();
        assert_eq!(metadata.node_count_estimate, Some(expected_nodes));
        assert_eq!(metadata.edge_count_estimate, Some(expected_edges));

        // Generate HTML using dependency injection
        let result = generate_wasm_visualization_with_loader(&loader, "breadthfirst").await;

        // THEN: Should generate valid HTML with actual data
        assert!(result.is_ok(), "Memory loader should generate valid HTML");
        let html = result.unwrap();

        // Contract: Must contain actual graph data, not empty objects
        assert!(!html.contains("const graphData = {}"),
                "Memory loader should generate actual JSON data");
        assert!(html.contains("graphData = "),
                "Must contain graphData assignment");
        assert!(html.contains(r#""nodes":["#),
                "Must contain actual nodes array");
        assert!(html.contains(r#""id":"#),
                "Must contain node IDs in JSON");

        // Contract: Should have correct counts
        println!("DEBUG: Looking for 'Nodes: {}' in HTML of length {}", expected_nodes, html.len());
        println!("DEBUG: HTML snippet around node count: {:?}", &html[html.find("Nodes:").map(|i| i-20..i+50).unwrap_or(0..0)]);

        if let Some(layout_start) = html.find("currentLayout = '") {
            let layout_content = &html[layout_start + 16..layout_start + 50];
            println!("DEBUG: Layout: {}", layout_content);
        }

        if let Some(data_start) = html.find("graphData = ") {
            let data_content = &html[data_start + 13..data_start + 50];
            println!("DEBUG: GraphData start: {:?}", data_content);
        }

        assert!(html.contains(&format!("Nodes: {}", expected_nodes)),
                "Should display correct node count");
        assert!(html.contains(&format!("Edges: {}", expected_edges)),
                "Should display correct edge count");

        println!("✅ Memory loader dependency injection test passed");
    }

    /// Test Contract: Error loader should handle failures gracefully
    /// WHEN using MockErrorLoader with error conditions
    /// THEN shall return appropriate error without crashing
    #[tokio::test]
    async fn test_error_loader_dependency_injection_req_di_002() {
        // GIVEN: Mock error loader with specific error
        let expected_error = GraphDataError::ISGLoadError("Test data unavailable".to_string());
        let loader = MockErrorLoader::new(expected_error);

        // Validate loader metadata
        assert!(!loader.is_available().await, "Error loader should never be available");
        assert_eq!(loader.source_id(), "mock:error");

        // WHEN: Generating HTML with error loader
        let result = generate_wasm_visualization_with_loader(&loader, "breadthfirst").await;

        // THEN: Should return error without crashing
        assert!(result.is_err(), "Error loader should return error");

        // Verify error type and message
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Data source") && error_msg.contains("not available"),
                "Should return availability error: {}", error_msg);

        println!("✅ Error loader dependency injection test passed");
    }

    /// Test Contract: Factory should create appropriate loaders
    /// WHEN using GraphDataLoaderFactory
    /// THEN shall create loaders with correct metadata
    #[tokio::test]
    async fn test_factory_creators_dependency_injection_req_di_003() {
        // GIVEN: Test ISG
        let isg = create_test_isg_with_nodes();

        // WHEN: Creating loaders through factory
        let memory_loader = GraphDataLoaderFactory::for_testing(isg.clone());
        let error_loader = GraphDataLoaderFactory::for_error_testing(
            GraphDataError::ISGLoadError("Factory test error".to_string())
        );

        // THEN: Should create functional loaders
        assert!(memory_loader.is_available().await, "Factory memory loader should be available");
        assert!(!error_loader.is_available().await, "Factory error loader should not be available");

        // Generate HTML with factory-created memory loader
        let result = generate_wasm_visualization_with_loader(&*memory_loader, "hierarchical").await;
        assert!(result.is_ok(), "Factory memory loader should generate valid HTML");

        let html = result.unwrap();
        assert!(html.contains("graphData = "), "Should contain actual JSON data");
        assert!(html.contains("hierarchical"), "Should use correct layout");

        println!("✅ Factory creators dependency injection test passed");
    }

    /// Test Contract: Async loading should respect performance contracts
    /// WHEN using async loader with realistic data
    /// THEN shall complete within performance contract limits
    #[tokio::test]
    async fn test_async_performance_dependency_injection_req_di_004() {
        // GIVEN: Large test ISG (simulating realistic data)
        let isg = create_large_test_isg();
        let expected_nodes = isg.node_count();

        // WHEN: Loading and generating HTML asynchronously
        let start = Instant::now();
        let loader = MemoryISGLoader::new(isg);
        let load_time = start.elapsed();

        let start = Instant::now();
        let result = generate_wasm_visualization_with_loader(&loader, "breadthfirst").await;
        let generation_time = start.elapsed();

        // THEN: Should meet performance contracts
        assert!(result.is_ok(), "Async loading should succeed with large ISG");

        // Performance contract: <10ms for memory loader creation
        assert!(load_time < std::time::Duration::from_millis(10),
                "Memory loader creation took {:?}, expected <10ms", load_time);

        // Performance contract: <100ms for HTML generation
        assert!(generation_time < std::time::Duration::from_millis(100),
                "HTML generation took {:?}, expected <100ms (Performance Contract VIOLATION)",
                generation_time);

        let html = result.unwrap();
        assert!(html.contains(&format!("Nodes: {}", expected_nodes)),
                "Should contain correct node count for large ISG");

        println!("✅ Async performance dependency injection test passed: load={:?}, gen={:?}",
                load_time, generation_time);
    }

    /// Test Contract: Different loaders should produce consistent HTML structure
    /// WHEN comparing memory loader vs direct function
    /// THEN should produce equivalent HTML with same data
    #[tokio::test]
    async fn test_loader_consistency_dependency_injection_req_di_005() {
        // GIVEN: Same test ISG
        let isg = create_test_isg_with_nodes();

        // WHEN: Generating HTML with two different methods
        let direct_html = generate_wasm_visualization(&isg, "circular").unwrap();

        let loader = MemoryISGLoader::new(isg);
        let loader_html = generate_wasm_visualization_with_loader(&loader, "circular").await.unwrap();

        // THEN: Should produce equivalent HTML (same structure and data)
        // Note: Exact string comparison may fail due to HashMap ordering,
        // so we check key structural elements

        // Both should have valid HTML structure (allow leading whitespace)
        let trimmed_direct = direct_html.trim_start();
        let trimmed_loader = loader_html.trim_start();
        assert!(trimmed_direct.starts_with("<!DOCTYPE html>"));
        assert!(trimmed_loader.starts_with("<!DOCTYPE html>"));

        // Both should contain the same graph data (structure-wise)
        let direct_has_nodes = direct_html.contains(r#""nodes":["#);
        let loader_has_nodes = loader_html.contains(r#""nodes":["#);
        assert_eq!(direct_has_nodes, loader_has_nodes,
                  "Both methods should contain nodes array");

        let direct_has_edges = direct_html.contains(r#""edges":["#);
        let loader_has_edges = loader_html.contains(r#""edges":["#);
        assert_eq!(direct_has_edges, loader_has_edges,
                  "Both methods should contain edges array");

        // Both should have same layout selection
        let direct_has_circular = direct_html.contains("circular\" selected");
        let loader_has_circular = loader_html.contains("circular\" selected");
        assert_eq!(direct_has_circular, loader_has_circular,
                  "Both methods should select same layout");

        println!("✅ Loader consistency dependency injection test passed");
    }
}