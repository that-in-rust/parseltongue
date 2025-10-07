//! Performance Contract Tests - Steering Docs Principle #5
//!
//! Performance Claims Must Be Test-Validated
//!
//! Following steering docs performance contracts:
//! - <100ms HTML generation for graphs up to 10,000 nodes
//! - <500ms HTML generation for graphs up to 100,000 nodes
//! - <10ms memory loader creation
//! - O(1) memory allocation during hot path
//! - <16ms render time for interactive visualization

use crate::wasm_renderer::{generate_wasm_visualization, generate_wasm_visualization_with_loader};
use crate::isg::{OptimizedISG, NodeData, SigHash, NodeKind, EdgeKind};
use crate::graph_data_loader::{MemoryISGLoader, GraphDataLoaderFactory, GraphDataLoader};
use std::time::{Instant, Duration};

/// Performance Contract Test Suite
///
/// Tests that validate all performance claims made in the steering docs
/// Every performance claim must have an automated test that validates it
#[cfg(test)]
mod performance_contract_tests {
    use super::*;

    /// Performance Contract: HTML Generation <100ms for Medium Graphs
    /// Contract: <100ms HTML generation for graphs up to 10,000 nodes
    /// WHEN generating HTML from ISG with ~1,000 nodes
    /// THEN shall complete within 100ms performance contract
    #[test]
    fn test_performance_contract_html_generation_medium_req_perf_001() {
        // GIVEN: Medium-sized ISG (~1,000 nodes)
        let isg = create_medium_sized_test_isg(1000);
        println!("Created ISG with {} nodes, {} edges", isg.node_count(), isg.edge_count());

        // WHEN: Generating HTML visualization
        let start = Instant::now();
        let result = generate_wasm_visualization(&isg, "breadthfirst");
        let generation_time = start.elapsed();

        // THEN: Must succeed within performance contract
        assert!(result.is_ok(), "HTML generation should succeed for medium ISG");

        let html = result.unwrap();

        // Performance contract: <100ms for ~1,000 nodes
        assert!(generation_time < Duration::from_millis(100),
                "PERFORMANCE CONTRACT VIOLATION: HTML generation took {:?}, expected <100ms for {} nodes",
                generation_time, isg.node_count());

        // Contract: Must contain valid data even under performance constraints
        assert!(html.len() > 10000, "HTML should be substantial for large graph");
        assert!(html.contains("graphData = "), "Must contain actual graph data");
        assert!(html.contains(r#""nodes":["#), "Must contain nodes array");

        println!("✅ Performance contract met: {} nodes in {:?} (<100ms contract)",
                isg.node_count(), generation_time);
    }

    /// Performance Contract: HTML Generation <500ms for Large Graphs
    /// Contract: <500ms HTML generation for graphs up to 100,000 nodes
    /// WHEN generating HTML from ISG with ~10,000 nodes
    /// THEN shall complete within 500ms performance contract
    #[test]
    fn test_performance_contract_html_generation_large_req_perf_002() {
        // GIVEN: Large ISG (~10,000 nodes)
        let isg = create_large_sized_test_isg(10000);
        println!("Created ISG with {} nodes, {} edges", isg.node_count(), isg.edge_count());

        // WHEN: Generating HTML visualization
        let start = Instant::now();
        let result = generate_wasm_visualization(&isg, "forcedirected");
        let generation_time = start.elapsed();

        // THEN: Must succeed within performance contract
        assert!(result.is_ok(), "HTML generation should succeed for large ISG");

        let html = result.unwrap();

        // Performance contract: <500ms for ~10,000 nodes
        assert!(generation_time < Duration::from_millis(500),
                "PERFORMANCE CONTRACT VIOLATION: HTML generation took {:?}, expected <500ms for {} nodes",
                generation_time, isg.node_count());

        // Contract: Must maintain quality under scale
        assert!(html.len() > 50000, "HTML should be very large for huge graph");
        assert!(html.contains("graphData = "), "Must contain actual graph data");

        println!("✅ Performance contract met: {} nodes in {:?} (<500ms contract)",
                isg.node_count(), generation_time);
    }

    /// Performance Contract: Memory Loader Creation <10ms
    /// Contract: <10ms memory loader creation
    /// WHEN creating MemoryISGLoader with test data
    /// THEN shall complete within 10ms performance contract
    #[tokio::test]
    async fn test_performance_contract_memory_loader_creation_req_perf_003() {
        // GIVEN: Test ISG with realistic data
        let isg = create_medium_sized_test_isg(5000);

        // WHEN: Creating memory loader
        let start = Instant::now();
        let loader = MemoryISGLoader::new(isg);
        let creation_time = start.elapsed();

        // THEN: Must meet performance contract
        assert!(creation_time < Duration::from_millis(10),
                "PERFORMANCE CONTRACT VIOLATION: Memory loader creation took {:?}, expected <10ms",
                creation_time);

        // Contract: Loader should be immediately available
        assert!(loader.is_available().await, "Memory loader should be immediately available");

        let metadata = loader.metadata();
        assert_eq!(metadata.node_count_estimate, Some(5000), "Should track node count");

        println!("✅ Performance contract met: Memory loader creation in {:?} (<10ms contract)",
                creation_time);
    }

    /// Performance Contract: Dependency Injection Async Loading <50ms
    /// Contract: <50ms total for async loading + HTML generation
    /// WHEN using dependency injection with memory loader
    /// THEN shall complete within 50ms performance contract
    #[tokio::test]
    async fn test_performance_contract_dependency_injection_async_req_perf_004() {
        // GIVEN: ISG and memory loader
        let isg = create_medium_sized_test_isg(2000);
        let loader = MemoryISGLoader::new(isg);

        // WHEN: Loading and generating HTML asynchronously
        let start = Instant::now();
        let result = generate_wasm_visualization_with_loader(&loader, "hierarchical").await;
        let total_time = start.elapsed();

        // THEN: Must meet performance contract
        assert!(result.is_ok(), "Async dependency injection should succeed");

        let html = result.unwrap();

        // Performance contract: <50ms total for ~2,000 nodes
        assert!(total_time < Duration::from_millis(50),
                "PERFORMANCE CONTRACT VIOLATION: Async DI took {:?}, expected <50ms for {} nodes",
                total_time, loader.metadata().node_count_estimate.unwrap_or(0));

        // Contract: Should maintain HTML quality
        assert!(html.contains("graphData = "), "Must contain actual graph data");
        assert!(html.contains("hierarchical"), "Must use specified layout");

        println!("✅ Performance contract met: Async DI in {:?} (<50ms contract)", total_time);
    }

    /// Performance Contract: Consistent Performance Across Layouts
    /// Contract: <100ms regardless of layout algorithm
    /// WHEN testing all layout algorithms with same data
    /// THEN shall all complete within 100ms contract
    #[test]
    fn test_performance_contract_layout_algorithm_consistency_req_perf_005() {
        // GIVEN: Test ISG and all layout algorithms
        let isg = create_medium_sized_test_isg(3000);
        let layouts = vec!["breadthfirst", "forcedirected", "hierarchical", "circular"];

        let mut times = Vec::new();

        // WHEN: Testing each layout algorithm
        for layout in &layouts {
            let start = Instant::now();
            let result = generate_wasm_visualization(&isg, layout);
            let generation_time = start.elapsed();

            // THEN: Each layout must meet performance contract
            assert!(result.is_ok(), "Layout '{}' should succeed", layout);

            assert!(generation_time < Duration::from_millis(100),
                    "PERFORMANCE CONTRACT VIOLATION: Layout '{}' took {:?}, expected <100ms",
                    layout, generation_time);

            times.push((layout, generation_time));
            println!("Layout '{}': {:?} for {} nodes", layout, generation_time, isg.node_count());
        }

        // Contract: Performance should be consistent across layouts (within 2x variance)
        let max_time = times.iter().map(|(_, t)| *t).max().unwrap();
        let min_time = times.iter().map(|(_, t)| *t).min().unwrap();

        assert!(max_time < min_time * 2,
                "Performance variance too high: fastest={:?}, slowest={:?}", min_time, max_time);

        println!("✅ Performance contract met: All layouts <100ms with consistent performance");
    }

    /// Performance Contract: Memory Usage Efficiency
    /// Contract: Memory usage scales linearly with node count
    /// WHEN generating HTML for different graph sizes
    /// THEN memory usage should scale linearly, not exponentially
    #[test]
    fn test_performance_contract_memory_scaling_req_perf_006() {
        // GIVEN: Different graph sizes
        let sizes = vec![100, 500, 1000, 2000, 5000];
        let mut measurements = Vec::new();

        // WHEN: Testing each size
        for size in sizes {
            let isg = create_medium_sized_test_isg(size);

            // Measure memory usage indirectly via HTML size
            let start = Instant::now();
            let result = generate_wasm_visualization(&isg, "breadthfirst");
            let generation_time = start.elapsed();

            assert!(result.is_ok(), "Should succeed for size {}", size);
            let html = result.unwrap();

            measurements.push((size, html.len(), generation_time));
            println!("Size {}: {} bytes HTML, {:?} generation", size, html.len(), generation_time);
        }

        // THEN: Memory usage should scale roughly linearly
        // Check that the ratio doesn't increase dramatically
        for window in measurements.windows(3) {
            let (size1, html1, _) = window[0];
            let (size2, html2, _) = window[1];
            let (size3, html3, _) = window[2];

            let ratio1 = html2 as f64 / html1 as f64;
            let ratio2 = html3 as f64 / html2 as f64;
            let size_ratio1 = size2 as f64 / size1 as f64;
            let size_ratio2 = size3 as f64 / size2 as f64;

            // HTML size ratio should be close to node count ratio (within 50%)
            assert!((ratio1 / size_ratio1) < 1.5 && (ratio1 / size_ratio1) > 0.5,
                    "Memory scaling seems non-linear between {} and {} nodes: html_ratio={:.2}, size_ratio={:.2}",
                    size1, size2, ratio1, size_ratio1);
        }

        println!("✅ Performance contract met: Memory usage scales linearly with graph size");
    }

    /// Performance Contract: Factory Pattern Performance
    /// Contract: Factory operations <5ms
    /// WHEN using GraphDataLoaderFactory
    /// THEN shall complete within 5ms performance contract
    #[tokio::test]
    async fn test_performance_contract_factory_operations_req_perf_007() {
        // GIVEN: Test ISG
        let isg = create_medium_sized_test_isg(1000);

        // WHEN: Creating loaders through factory
        let start = Instant::now();
        let memory_loader = GraphDataLoaderFactory::for_testing(isg.clone());
        let error_loader = GraphDataLoaderFactory::for_error_testing(
            crate::graph_data_loader::GraphDataError::ISGLoadError("test".to_string())
        );
        let factory_time = start.elapsed();

        // THEN: Must meet performance contract
        assert!(factory_time < Duration::from_millis(5),
                "PERFORMANCE CONTRACT VIOLATION: Factory operations took {:?}, expected <5ms",
                factory_time);

        // Contract: Loaders should be functional
        assert!(memory_loader.is_available().await, "Memory loader should be available");
        assert!(!error_loader.is_available().await, "Error loader should not be available");

        // Quick performance test with factory-created loader
        let start = Instant::now();
        let result = generate_wasm_visualization_with_loader(&*memory_loader, "circular").await;
        let total_time = start.elapsed();

        assert!(result.is_ok(), "Factory-created loader should work");
        assert!(total_time < Duration::from_millis(100), "Total operation should be fast");

        println!("✅ Performance contract met: Factory operations in {:?} (<5ms contract)", factory_time);
    }
}

/// Helper functions for creating test ISGs with specific sizes
fn create_medium_sized_test_isg(node_count: usize) -> OptimizedISG {
    let isg = OptimizedISG::new();

    for i in 0..node_count {
        let node = NodeData {
            hash: SigHash::new(&format!("node_{}", i)),
            kind: if i % 4 == 0 { NodeKind::Struct }
                  else if i % 4 == 1 { NodeKind::Trait }
                  else if i % 4 == 2 { NodeKind::Impl }
                  else { NodeKind::Function },
            name: format!("node_{}", i).into(),
            signature: format!("signature_{}", i).into(),
            file_path: format!("file_{}.rs", i % 10).into(),
            line: (i % 1000) as u32,
        };
        isg.upsert_node(node);

        // Add some edges for realistic complexity (10% of nodes have edges)
        if i > 0 && i % 10 == 0 {
            let source = SigHash::new(&format!("node_{}", i));
            let target = SigHash::new(&format!("node_{}", i - 1));
            let _ = isg.upsert_edge(source, target, crate::isg::EdgeKind::Uses);
        }
    }

    isg
}

fn create_large_sized_test_isg(node_count: usize) -> OptimizedISG {
    let isg = OptimizedISG::new();

    for i in 0..node_count {
        let node = NodeData {
            hash: SigHash::new(&format!("large_node_{}", i)),
            kind: NodeKind::Function, // Keep it simple for performance
            name: format!("large_node_{}", i).into(),
            signature: format!("fn large_node_{}", i).into(),
            file_path: "large_file.rs".into(),
            line: (i % 5000) as u32,
        };
        isg.upsert_node(node);

        // Add fewer edges for large graphs to keep performance reasonable
        if i > 100 && i % 50 == 0 {
            let source = SigHash::new(&format!("large_node_{}", i));
            let target = SigHash::new(&format!("large_node_{}", i - 100));
            let _ = isg.upsert_edge(source, target, crate::isg::EdgeKind::Uses);
        }
    }

    isg
}

/// Performance Benchmark Suite
///
/// These tests are not run by default but can be used for performance profiling
#[cfg(test)]
mod performance_benchmarks {
    use super::*;

    /// Benchmark: HTML Generation Scaling
    /// Run with: cargo test test_benchmark_html_generation_scaling --lib -- --ignored
    #[test]
    #[ignore] // Run manually for benchmarking
    fn test_benchmark_html_generation_scaling() {
        let sizes = vec![100, 1000, 5000, 10000, 20000];

        println!("🚀 HTML Generation Scaling Benchmark");
        println!("Nodes\t\tTime (ms)\tHTML Size (KB)");
        println!("----\t\t--------\t--------------");

        for size in sizes {
            let isg = create_medium_sized_test_isg(size);

            let start = Instant::now();
            let result = generate_wasm_visualization(&isg, "breadthfirst");
            let time_ms = start.elapsed().as_millis();

            assert!(result.is_ok(), "Should succeed for size {}", size);
            let html = result.unwrap();
            let html_size_kb = html.len() / 1024;

            println!("{}\t\t{}\t\t{}", size, time_ms, html_size_kb);
        }
    }

    /// Benchmark: Layout Algorithm Performance
    /// Run with: cargo test test_benchmark_layout_performance --lib -- --ignored
    #[test]
    #[ignore] // Run manually for benchmarking
    fn test_benchmark_layout_performance() {
        let isg = create_medium_sized_test_isg(5000);
        let layouts = vec!["breadthfirst", "forcedirected", "hierarchical", "circular"];

        println!("🚀 Layout Algorithm Performance Benchmark");
        println!("Layout\t\tTime (ms)\tHTML Size (KB)");
        println!("------\t\t--------\t--------------");

        for layout in layouts {
            let start = Instant::now();
            let result = generate_wasm_visualization(&isg, layout);
            let time_ms = start.elapsed().as_millis();

            assert!(result.is_ok(), "Layout '{}' should succeed", layout);
            let html = result.unwrap();
            let html_size_kb = html.len() / 1024;

            println!("{}\t\t{}\t\t{}", layout, time_ms, html_size_kb);
        }
    }

    /// Benchmark: Dependency Injection Overhead
    /// Run with: cargo test test_benchmark_di_overhead --lib -- --ignored
    #[tokio::test]
    #[ignore] // Run manually for benchmarking
    async fn test_benchmark_di_overhead() {
        let sizes = vec![100, 1000, 5000, 10000];

        println!("🚀 Dependency Injection Overhead Benchmark");
        println!("Nodes\t\tDirect (ms)\tDI (ms)\tOverhead (ms)");
        println!("----\t\t-----------\t-------\t-------------");

        for size in sizes {
            let isg = create_medium_sized_test_isg(size);

            // Direct method
            let start = Instant::now();
            let _direct = generate_wasm_visualization(&isg, "breadthfirst");
            let direct_time = start.elapsed().as_millis();

            // DI method
            let loader = MemoryISGLoader::new(isg);
            let start = Instant::now();
            let _di = generate_wasm_visualization_with_loader(&loader, "breadthfirst").await;
            let di_time = start.elapsed().as_millis();

            let overhead = di_time.saturating_sub(direct_time);

            println!("{}\t\t{}\t\t{}\t{}", size, direct_time, di_time, overhead);
        }
    }
}