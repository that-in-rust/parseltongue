//! Filter dependency edges to only show implementation-to-implementation
//!
//! When analyzing production code quality, we only want to see dependencies
//! between production code entities, not test-to-test or test-to-code edges.
//!
//! ## TDD Contract
//! - **Precondition**: Valid `Vec<DependencyEdge>` and `HashSet<String>` of implementation keys
//! - **Postcondition**: Returns only edges where BOTH from_key AND to_key are in impl_keys
//! - **Error Conditions**: None (filtering is infallible)

use pt02_llm_cozodb_to_context_writer::DependencyEdge;
use std::collections::HashSet;

/// Filter edges to only keep implementation-to-implementation dependencies
///
/// This ensures dependency analysis focuses on production code relationships.
/// Test code dependencies are analyzed separately.
///
/// # Arguments
/// * `edges` - All dependency edges from CozoDB
/// * `impl_keys` - Set of ISGL1 keys for implementation entities only
///
/// # Returns
/// Only edges where both endpoints are implementation code
///
/// # Example
/// ```no_run
/// use parseltongue_core::entities::DependencyEdge;
/// use std::collections::HashSet;
/// use pt07_visual_analytics_terminal::core::filter_implementation_edges_only;
///
/// let impl_keys: HashSet<String> = ["rust:fn:foo", "rust:fn:bar"]
///     .iter().map(|s| s.to_string()).collect();
/// let all_edges = vec![/* from CozoDB */];
/// let impl_edges = filter_implementation_edges_only(all_edges, &impl_keys);
/// // impl_edges contains ONLY implementation-to-implementation edges
/// ```
pub fn filter_implementation_edges_only(
    edges: Vec<DependencyEdge>,
    impl_keys: &HashSet<String>,
) -> Vec<DependencyEdge> {
    edges
        .into_iter()
        .filter(|edge| {
            impl_keys.contains(edge.from_key.as_str()) && impl_keys.contains(edge.to_key.as_str())
        })
        .collect()
}

/// Include all edges (when --include-tests flag is passed)
///
/// No filtering applied - returns all edges as-is.
///
/// # Arguments
/// * `edges` - All edges from CozoDB
/// * `_all_keys` - Ignored (kept for API consistency)
///
/// # Returns
/// Same vec, unchanged
pub fn filter_include_all_edge_types(
    edges: Vec<DependencyEdge>,
    _all_keys: &HashSet<String>,
) -> Vec<DependencyEdge> {
    edges  // No filtering
}

#[cfg(test)]
mod tests {
    use super::*;
    use pt02_llm_cozodb_to_context_writer::DependencyEdge;

    fn create_test_edge(from: &str, to: &str, edge_type: &str) -> DependencyEdge {
        DependencyEdge {
            from_key: from.to_string(),
            to_key: to.to_string(),
            edge_type: edge_type.to_string(),
        }
    }

    #[test]
    fn test_filter_keeps_only_impl_to_impl_edges() {
        // Arrange: Define implementation keys
        let impl_keys: HashSet<String> = [
            "rust:fn:prod_a",
            "rust:fn:prod_b",
        ].iter().map(|s| s.to_string()).collect();

        // Create edges (mix of impl-impl, test-test, impl-test)
        let edges = vec![
            create_test_edge("rust:fn:prod_a", "rust:fn:prod_b", "calls"),     // Keep
            create_test_edge("rust:fn:test_a", "rust:fn:test_b", "calls"),     // Filter
            create_test_edge("rust:fn:prod_a", "rust:fn:test_a", "calls"),     // Filter (impl->test)
            create_test_edge("rust:fn:test_b", "rust:fn:prod_b", "calls"),     // Filter (test->impl)
        ];

        // Act
        let result = filter_implementation_edges_only(edges, &impl_keys);

        // Assert: Only 1 edge remains (impl-impl)
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from_key, "rust:fn:prod_a");
        assert_eq!(result[0].to_key, "rust:fn:prod_b");
    }

    #[test]
    fn test_filter_handles_empty_edges() {
        // Arrange
        let impl_keys: HashSet<String> = HashSet::new();
        let edges: Vec<DependencyEdge> = vec![];

        // Act
        let result = filter_implementation_edges_only(edges, &impl_keys);

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_handles_empty_impl_keys() {
        // Arrange: Edges exist but no impl keys
        let impl_keys: HashSet<String> = HashSet::new();
        let edges = vec![
            create_test_edge("rust:fn:a", "rust:fn:b", "calls"),
        ];

        // Act: Should filter out all edges (no impl keys to match)
        let result = filter_implementation_edges_only(edges, &impl_keys);

        // Assert: All filtered out
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_preserves_edge_types() {
        // Arrange
        let impl_keys: HashSet<String> = [
            "rust:fn:a",
            "rust:fn:b",
            "rust:fn:c",
        ].iter().map(|s| s.to_string()).collect();

        let edges = vec![
            create_test_edge("rust:fn:a", "rust:fn:b", "calls"),
            create_test_edge("rust:fn:b", "rust:fn:c", "uses"),
            create_test_edge("rust:fn:c", "rust:fn:a", "implements"),
        ];

        // Act
        let result = filter_implementation_edges_only(edges, &impl_keys);

        // Assert: All preserved with correct types
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].edge_type, "calls");
        assert_eq!(result[1].edge_type, "uses");
        assert_eq!(result[2].edge_type, "implements");
    }

    #[test]
    fn test_include_all_returns_unchanged_edges() {
        // Arrange
        let all_keys: HashSet<String> = HashSet::new();  // Unused
        let edges = vec![
            create_test_edge("rust:fn:test_a", "rust:fn:test_b", "calls"),
            create_test_edge("rust:fn:prod_a", "rust:fn:prod_b", "calls"),
        ];
        let original_count = edges.len();

        // Act
        let result = filter_include_all_edge_types(edges, &all_keys);

        // Assert: Count unchanged
        assert_eq!(result.len(), original_count);
    }

    #[test]
    fn test_filter_handles_self_loops() {
        // Arrange: Edge from entity to itself
        let impl_keys: HashSet<String> = [
            "rust:fn:recursive",
        ].iter().map(|s| s.to_string()).collect();

        let edges = vec![
            create_test_edge("rust:fn:recursive", "rust:fn:recursive", "calls"),
        ];

        // Act
        let result = filter_implementation_edges_only(edges, &impl_keys);

        // Assert: Self-loop preserved if both ends are impl
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from_key, result[0].to_key);
    }
}
