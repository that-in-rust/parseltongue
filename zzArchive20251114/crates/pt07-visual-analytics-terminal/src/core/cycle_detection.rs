//! Cycle detection algorithm for dependency graphs
//!
//! ## Algorithm: Depth-First Search (DFS) with Recursion Stack
//! - **Time Complexity**: O(V + E) where V = nodes, E = edges
//! - **Space Complexity**: O(V) for recursion stack and visited sets
//!
//! ## TDD Contract
//! - **Precondition**: Valid Vec<DependencyEdge> (can be empty)
//! - **Postcondition**: Returns Vec<Vec<String>> of all cycles found
//! - **Error Conditions**: None (infallible, returns empty vec if no cycles)
//!
//! ## References
//! - Tarjan's Strongly Connected Components algorithm
//! - DFS-based cycle detection for directed graphs

use pt02_llm_cozodb_to_context_writer::DependencyEdge;
use std::collections::{HashMap, HashSet};

/// Detect all cycles in a dependency graph
///
/// Uses DFS with recursion stack to find cycles in O(V + E) time.
///
/// # Arguments
/// * `edges` - List of directed edges representing dependencies
///
/// # Returns
/// Vec of cycles, where each cycle is a Vec of ISGL1 keys in the cycle.
/// Returns empty vec if no cycles found.
///
/// # Examples
/// ```
/// use pt07_visual_analytics_terminal::core::detect_cycles_in_dependency_graph;
/// use pt02_llm_cozodb_to_context_writer::DependencyEdge;
///
/// let edges = vec![
///     DependencyEdge {
///         from_key: "a".to_string(),
///         to_key: "b".to_string(),
///         edge_type: "depends_on".to_string(),
///     },
///     DependencyEdge {
///         from_key: "b".to_string(),
///         to_key: "a".to_string(),
///         edge_type: "depends_on".to_string(),
///     },
/// ];
///
/// let cycles = detect_cycles_in_dependency_graph(&edges);
/// assert_eq!(cycles.len(), 1);  // Found one cycle: a → b → a
/// ```
pub fn detect_cycles_in_dependency_graph(
    edges: &[DependencyEdge],
) -> Vec<Vec<String>> {
    // Build adjacency list representation
    let graph = build_adjacency_list_from_edges(edges);

    // Track visited nodes and recursion stack
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut current_path = Vec::new();
    let mut cycles = Vec::new();

    // Run DFS from each unvisited node
    for node in graph.keys() {
        if !visited.contains(node) {
            dfs_detect_cycles_in_graph(
                node,
                &graph,
                &mut visited,
                &mut rec_stack,
                &mut current_path,
                &mut cycles,
            );
        }
    }

    cycles
}

/// Build adjacency list from edge list
///
/// # Returns
/// HashMap where key = node, value = list of neighbors (outgoing edges)
fn build_adjacency_list_from_edges(
    edges: &[DependencyEdge],
) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for edge in edges {
        graph
            .entry(edge.from_key.clone())
            .or_insert_with(Vec::new)
            .push(edge.to_key.clone());

        // Ensure target node exists in graph even if it has no outgoing edges
        graph.entry(edge.to_key.clone()).or_insert_with(Vec::new);
    }

    graph
}

/// DFS traversal to detect cycles
///
/// # Algorithm
/// 1. Mark current node as visited and add to recursion stack
/// 2. For each neighbor:
///    - If neighbor is in recursion stack → cycle detected!
///    - If neighbor not visited → recursively visit it
/// 3. Remove node from recursion stack when done (backtrack)
///
/// # Cycle Detection
/// When we find a node already in the recursion stack, we extract the cycle
/// by taking all nodes in current_path from that node onwards.
fn dfs_detect_cycles_in_graph(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    current_path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    // Mark as visited and add to recursion stack
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());
    current_path.push(node.to_string());

    // Visit all neighbors
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            // Check for self-loop
            if neighbor == node {
                cycles.push(vec![node.to_string()]);
                continue;
            }

            // If neighbor is in recursion stack, we found a cycle
            if rec_stack.contains(neighbor) {
                // Extract cycle from current_path
                if let Some(cycle) = extract_cycle_from_path_to_node(current_path, neighbor) {
                    // Only add if not duplicate
                    if !is_duplicate_cycle(&cycles, &cycle) {
                        cycles.push(cycle);
                    }
                }
            }
            // If neighbor not visited, recursively visit
            else if !visited.contains(neighbor) {
                dfs_detect_cycles_in_graph(
                    neighbor,
                    graph,
                    visited,
                    rec_stack,
                    current_path,
                    cycles,
                );
            }
        }
    }

    // Backtrack: remove from recursion stack and path
    rec_stack.remove(node);
    current_path.pop();
}

/// Extract cycle from current path when we encounter a back edge
///
/// # Arguments
/// * `path` - Current DFS path
/// * `target` - Node that created the back edge (already in path)
///
/// # Returns
/// The cycle as a Vec of node names, or None if target not in path
fn extract_cycle_from_path_to_node(
    path: &[String],
    target: &str,
) -> Option<Vec<String>> {
    // Find position of target in path
    let pos = path.iter().position(|n| n == target)?;

    // Cycle is from target position to end of path
    Some(path[pos..].to_vec())
}

/// Check if a cycle is already in the cycles list
///
/// Cycles are considered duplicates if they contain the same nodes
/// (regardless of starting position or direction).
fn is_duplicate_cycle(
    cycles: &[Vec<String>],
    new_cycle: &[String],
) -> bool {
    for existing_cycle in cycles {
        if cycles_are_equivalent(existing_cycle, new_cycle) {
            return true;
        }
    }
    false
}

/// Check if two cycles are equivalent (same nodes, possibly rotated)
///
/// Example: [A, B, C] is equivalent to [B, C, A] and [C, A, B]
fn cycles_are_equivalent(
    cycle1: &[String],
    cycle2: &[String],
) -> bool {
    if cycle1.len() != cycle2.len() {
        return false;
    }

    // Convert to sets for simple comparison (ignores order)
    let set1: HashSet<_> = cycle1.iter().collect();
    let set2: HashSet<_> = cycle2.iter().collect();

    set1 == set2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_adjacency_list() {
        let edges = vec![
            DependencyEdge {
                from_key: "a".to_string(),
                to_key: "b".to_string(),
                edge_type: "depends_on".to_string(),
            },
            DependencyEdge {
                from_key: "b".to_string(),
                to_key: "c".to_string(),
                edge_type: "depends_on".to_string(),
            },
        ];

        let graph = build_adjacency_list_from_edges(&edges);

        assert_eq!(graph.get("a").unwrap(), &vec!["b".to_string()]);
        assert_eq!(graph.get("b").unwrap(), &vec!["c".to_string()]);
        assert!(graph.contains_key("c"));  // Target node exists
    }

    #[test]
    fn test_extract_cycle_from_path() {
        let path = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];

        let cycle = extract_cycle_from_path_to_node(&path, "b");
        assert_eq!(cycle, Some(vec!["b".to_string(), "c".to_string(), "d".to_string()]));
    }

    #[test]
    fn test_cycles_are_equivalent() {
        let cycle1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let cycle2 = vec!["b".to_string(), "c".to_string(), "a".to_string()];

        assert!(cycles_are_equivalent(&cycle1, &cycle2));
    }

    #[test]
    fn test_cycles_not_equivalent_different_nodes() {
        let cycle1 = vec!["a".to_string(), "b".to_string()];
        let cycle2 = vec!["c".to_string(), "d".to_string()];

        assert!(!cycles_are_equivalent(&cycle1, &cycle2));
    }

    #[test]
    fn test_detect_simple_cycle() {
        let edges = vec![
            DependencyEdge {
                from_key: "a".to_string(),
                to_key: "b".to_string(),
                edge_type: "depends_on".to_string(),
            },
            DependencyEdge {
                from_key: "b".to_string(),
                to_key: "a".to_string(),
                edge_type: "depends_on".to_string(),
            },
        ];

        let cycles = detect_cycles_in_dependency_graph(&edges);

        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].len(), 2);
    }

    #[test]
    fn test_detect_no_cycles_in_dag() {
        let edges = vec![
            DependencyEdge {
                from_key: "a".to_string(),
                to_key: "b".to_string(),
                edge_type: "depends_on".to_string(),
            },
            DependencyEdge {
                from_key: "b".to_string(),
                to_key: "c".to_string(),
                edge_type: "depends_on".to_string(),
            },
        ];

        let cycles = detect_cycles_in_dependency_graph(&edges);

        assert!(cycles.is_empty());
    }

    #[test]
    fn test_detect_self_loop() {
        let edges = vec![
            DependencyEdge {
                from_key: "a".to_string(),
                to_key: "a".to_string(),
                edge_type: "depends_on".to_string(),
            },
        ];

        let cycles = detect_cycles_in_dependency_graph(&edges);

        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0], vec!["a".to_string()]);
    }
}
