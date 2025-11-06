//! Integration tests for cycle detection algorithm
//!
//! ## TDD Contract (STUB Phase)
//! - **Precondition**: Valid `Vec<DependencyEdge>` representing a directed graph
//! - **Postcondition**: Returns `Vec<Vec<String>>` where each inner vec is a cycle (list of ISGL1 keys)
//! - **Error Conditions**: None (cycle detection is infallible, returns empty vec if no cycles)
//!
//! ## Algorithm: DFS with Recursion Stack
//! - **Time Complexity**: O(V + E) where V = nodes, E = edges
//! - **Space Complexity**: O(V) for recursion stack and visited sets
//!
//! ## Test Coverage
//! 1. Detects simple 2-node cycle (A → B → A)
//! 2. Detects 3-node cycle (A → B → C → A)
//! 3. Detects multiple independent cycles
//! 4. Returns empty vec for acyclic graph
//! 5. Handles empty edge list
//! 6. Handles self-loops (A → A)
//! 7. Handles complex graph with cycles and non-cycle paths

use anyhow::Result;
use pt07_visual_analytics_terminal::core::detect_cycles_in_dependency_graph;
use pt02_llm_cozodb_to_context_writer::DependencyEdge;

#[test]
fn test_detects_simple_two_node_cycle() {
    // Arrange: A → B → A
    let edges = vec![
        DependencyEdge {
            from_key: "rust:fn:a:src_lib_rs:10".to_string(),
            to_key: "rust:fn:b:src_lib_rs:20".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:b:src_lib_rs:20".to_string(),
            to_key: "rust:fn:a:src_lib_rs:10".to_string(),
            edge_type: "depends_on".to_string(),
        },
    ];

    // Act: Detect cycles
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: Found exactly 1 cycle
    assert_eq!(cycles.len(), 1, "Should detect exactly one cycle");

    // Cycle should contain both nodes
    let cycle = &cycles[0];
    assert_eq!(cycle.len(), 2, "Cycle should have 2 nodes");
    assert!(cycle.contains(&"rust:fn:a:src_lib_rs:10".to_string()));
    assert!(cycle.contains(&"rust:fn:b:src_lib_rs:20".to_string()));
}

#[test]
fn test_detects_three_node_cycle() {
    // Arrange: A → B → C → A
    let edges = vec![
        DependencyEdge {
            from_key: "rust:fn:a:src_lib_rs:10".to_string(),
            to_key: "rust:fn:b:src_lib_rs:20".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:b:src_lib_rs:20".to_string(),
            to_key: "rust:fn:c:src_lib_rs:30".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:c:src_lib_rs:30".to_string(),
            to_key: "rust:fn:a:src_lib_rs:10".to_string(),
            edge_type: "depends_on".to_string(),
        },
    ];

    // Act
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: Found exactly 1 cycle with 3 nodes
    assert_eq!(cycles.len(), 1, "Should detect exactly one cycle");

    let cycle = &cycles[0];
    assert_eq!(cycle.len(), 3, "Cycle should have 3 nodes");
    assert!(cycle.contains(&"rust:fn:a:src_lib_rs:10".to_string()));
    assert!(cycle.contains(&"rust:fn:b:src_lib_rs:20".to_string()));
    assert!(cycle.contains(&"rust:fn:c:src_lib_rs:30".to_string()));
}

#[test]
fn test_detects_multiple_independent_cycles() {
    // Arrange: Two separate cycles
    // Cycle 1: A → B → A
    // Cycle 2: X → Y → X
    let edges = vec![
        // Cycle 1
        DependencyEdge {
            from_key: "rust:fn:a:src_lib_rs:10".to_string(),
            to_key: "rust:fn:b:src_lib_rs:20".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:b:src_lib_rs:20".to_string(),
            to_key: "rust:fn:a:src_lib_rs:10".to_string(),
            edge_type: "depends_on".to_string(),
        },
        // Cycle 2
        DependencyEdge {
            from_key: "rust:fn:x:src_other_rs:100".to_string(),
            to_key: "rust:fn:y:src_other_rs:200".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:y:src_other_rs:200".to_string(),
            to_key: "rust:fn:x:src_other_rs:100".to_string(),
            edge_type: "depends_on".to_string(),
        },
    ];

    // Act
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: Found 2 cycles
    assert_eq!(cycles.len(), 2, "Should detect two independent cycles");

    // Verify both cycles exist (order doesn't matter)
    let has_ab_cycle = cycles.iter().any(|c|
        c.contains(&"rust:fn:a:src_lib_rs:10".to_string()) &&
        c.contains(&"rust:fn:b:src_lib_rs:20".to_string())
    );

    let has_xy_cycle = cycles.iter().any(|c|
        c.contains(&"rust:fn:x:src_other_rs:100".to_string()) &&
        c.contains(&"rust:fn:y:src_other_rs:200".to_string())
    );

    assert!(has_ab_cycle, "Should detect A-B cycle");
    assert!(has_xy_cycle, "Should detect X-Y cycle");
}

#[test]
fn test_returns_empty_for_acyclic_graph() {
    // Arrange: Acyclic graph (A → B → C, no back edges)
    let edges = vec![
        DependencyEdge {
            from_key: "rust:fn:a:src_lib_rs:10".to_string(),
            to_key: "rust:fn:b:src_lib_rs:20".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:b:src_lib_rs:20".to_string(),
            to_key: "rust:fn:c:src_lib_rs:30".to_string(),
            edge_type: "depends_on".to_string(),
        },
    ];

    // Act
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: No cycles found
    assert!(cycles.is_empty(), "Acyclic graph should have no cycles");
}

#[test]
fn test_handles_empty_edge_list() {
    // Arrange: Empty graph
    let edges: Vec<DependencyEdge> = vec![];

    // Act
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: Returns empty vec (not error)
    assert!(cycles.is_empty(), "Empty graph should return empty cycles vec");
}

#[test]
fn test_handles_self_loop() {
    // Arrange: A → A (self-loop)
    let edges = vec![
        DependencyEdge {
            from_key: "rust:fn:a:src_lib_rs:10".to_string(),
            to_key: "rust:fn:a:src_lib_rs:10".to_string(),
            edge_type: "depends_on".to_string(),
        },
    ];

    // Act
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: Self-loop detected as 1-node cycle
    assert_eq!(cycles.len(), 1, "Should detect self-loop as cycle");

    let cycle = &cycles[0];
    assert_eq!(cycle.len(), 1, "Self-loop should be 1-node cycle");
    assert_eq!(cycle[0], "rust:fn:a:src_lib_rs:10");
}

#[test]
fn test_handles_complex_graph_with_cycles_and_non_cycle_paths() {
    // Arrange: Complex graph
    // Cycle: A → B → C → A
    // Non-cycle paths: D → A, E → F
    let edges = vec![
        // Cycle part
        DependencyEdge {
            from_key: "rust:fn:a:src_lib_rs:10".to_string(),
            to_key: "rust:fn:b:src_lib_rs:20".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:b:src_lib_rs:20".to_string(),
            to_key: "rust:fn:c:src_lib_rs:30".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:c:src_lib_rs:30".to_string(),
            to_key: "rust:fn:a:src_lib_rs:10".to_string(),
            edge_type: "depends_on".to_string(),
        },
        // Non-cycle paths
        DependencyEdge {
            from_key: "rust:fn:d:src_lib_rs:40".to_string(),
            to_key: "rust:fn:a:src_lib_rs:10".to_string(),
            edge_type: "depends_on".to_string(),
        },
        DependencyEdge {
            from_key: "rust:fn:e:src_lib_rs:50".to_string(),
            to_key: "rust:fn:f:src_lib_rs:60".to_string(),
            edge_type: "depends_on".to_string(),
        },
    ];

    // Act
    let cycles = detect_cycles_in_dependency_graph(&edges);

    // Assert: Found exactly 1 cycle (A-B-C)
    assert_eq!(cycles.len(), 1, "Should detect exactly one cycle");

    let cycle = &cycles[0];
    assert_eq!(cycle.len(), 3, "Cycle should have 3 nodes");

    // Verify D, E, F are NOT in cycle
    assert!(!cycle.contains(&"rust:fn:d:src_lib_rs:40".to_string()));
    assert!(!cycle.contains(&"rust:fn:e:src_lib_rs:50".to_string()));
    assert!(!cycle.contains(&"rust:fn:f:src_lib_rs:60".to_string()));
}

#[test]
fn test_performance_with_large_graph() {
    // Arrange: Large acyclic graph (1000 nodes in chain)
    let mut edges = Vec::new();
    for i in 0..999 {
        edges.push(DependencyEdge {
            from_key: format!("rust:fn:fn{}:src_lib_rs:{}", i, i * 10),
            to_key: format!("rust:fn:fn{}:src_lib_rs:{}", i + 1, (i + 1) * 10),
            edge_type: "depends_on".to_string(),
        });
    }

    // Act: Should complete in <100ms
    let start = std::time::Instant::now();
    let cycles = detect_cycles_in_dependency_graph(&edges);
    let duration = start.elapsed();

    // Assert: No cycles, completes quickly
    assert!(cycles.is_empty(), "Linear chain should have no cycles");
    assert!(duration.as_millis() < 100, "Should complete in <100ms, took {:?}", duration);
}
