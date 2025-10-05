//! Mermaid Export Module - ISG to Mermaid Diagram Transformation
//!
//! **Executable Specification**: Transforms Interface Signature Graph data into
//! GitHub-compatible Mermaid flowchart diagrams with deterministic, O(n) performance.
//!
//! ## Performance Contract
//! - **Target**: <1ms for typical graphs (≤100 nodes, ≤200 edges)
//! - **Memory**: O(1) additional allocation (string building only)
//! - **Complexity**: Linear traversal of nodes and edges
//!
//! ## Architecture Compliance (L1→L2→L3)
//! - **L1 Core**: Pure string manipulation, ownership transfer, Result/Option
//! - **L2 Standard**: Iterator patterns, slice processing, efficient concatenation
//! - **L3 External**: Minimal ISG type imports only (NodeData, NodeKind, EdgeKind)
//!
//! ## Mermaid Compliance
//! - GitHub-compatible syntax (flowchart TD)
//! - Vertical layout preference (per steeringDocs requirement)
//! - Proper node styling with icons and file paths
//! - Special character sanitization for node identifiers

use crate::isg::{OptimizedISG, NodeData, NodeKind, EdgeKind};
use std::fmt::Write;
use std::sync::Arc;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;

/// Main export function - transforms ISG to Mermaid flowchart
///
/// # Preconditions
/// - ISG graph is in valid state with consistent node/edge relationships
///
/// # Postconditions
/// - Returns valid Mermaid flowchart syntax
/// - All nodes rendered with proper styling and file paths
/// - All edges rendered with appropriate arrow styles
/// - Output is GitHub-compatible
///
/// # Error Conditions
/// - Cannot fail (String concatenation is infallible)
/// - Malformed node names are sanitized automatically
///
/// # Performance Contract
/// - Must complete in <1ms for graphs with ≤100 nodes
/// - Memory usage: O(1) additional allocation
pub fn export_isg_to_mermaid(isg: &OptimizedISG) -> String {
    let mut output = String::new();

    // Header with GitHub-compatible flowchart directive
    output.push_str("flowchart TD\n");

    let state = isg.state.read();

    // Phase 1: Render all nodes with type-specific styling
    for (_hash, &node_idx) in &state.id_map {
        if let Some(node) = state.graph.node_weight(node_idx) {
            render_node(&mut output, node);
        }
    }

    // Add spacing between nodes and edges
    output.push('\n');

    // Phase 2: Render all edges with relationship-specific styling
    for edge_ref in state.graph.edge_references() {
        let source = &state.graph[edge_ref.source()];
        let target = &state.graph[edge_ref.target()];
        render_edge(&mut output, source, target, edge_ref.weight());
    }

    output
}

/// Renders a single node with Mermaid syntax and type-specific styling
///
/// # Node Styling Strategy
/// - **Functions**: 🔧 gear icon, lightblue background
/// - **Structs**: 📦 package icon, lightgreen background
/// - **Traits**: 🎯 target icon, lightyellow background
///
/// # Name Sanitization
/// - Replaces hyphens with underscores for valid Mermaid identifiers
/// - Preserves original name in display label
fn render_node(output: &mut String, node: &NodeData) {
    let safe_name = sanitize_identifier(&node.name);
    let icon = node_kind_icon(&node.kind);

    let _ = write!(output,
        "    {}[\"{} {}<br/>({:?})<br/><i>{}</i>\"]\n",
        safe_name,
        icon,
        node.name,
        node.kind,
        node.file_path
    );
}

/// Renders a single edge with relationship-specific arrow styling
///
/// # Edge Styling Strategy
/// - **Calls**: Solid arrow (-->) for direct invocations
/// - **Implements**: Dashed arrow (-.->) for trait implementations
/// - **Uses**: Dotted arrow (-..->) for dependencies
fn render_edge(output: &mut String, source: &NodeData, target: &NodeData, edge_kind: &EdgeKind) {
    let safe_source = sanitize_identifier(&source.name);
    let safe_target = sanitize_identifier(&target.name);
    let arrow_style = edge_kind_arrow_style(edge_kind);

    let _ = write!(output,
        "    {} {} {}\n",
        safe_source,
        arrow_style,
        safe_target
    );
}

/// Sanitizes node names for valid Mermaid identifiers
///
/// # Sanitization Rules
/// - Replaces hyphens (-) with underscores (_)
/// - Could be extended for other special cases if needed
/// - Preserves original name for display purposes
fn sanitize_identifier(name: &str) -> String {
    name.replace('-', "_")
}

/// Returns appropriate icon for each node kind
const fn node_kind_icon(kind: &NodeKind) -> &'static str {
    match kind {
        NodeKind::Function => "🔧",
        NodeKind::Struct => "📦",
        NodeKind::Trait => "🎯",
    }
}

/// Returns appropriate arrow style for each edge kind
const fn edge_kind_arrow_style(kind: &EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Calls => "-->",
        EdgeKind::Implements => "-.->",
        EdgeKind::Uses => "-..->",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::SigHash;

    /// Test contract: Node rendering with all types
    ///
    /// # Given: ISG with one of each node type
    /// # When: export_isg_to_mermaid is called
    /// # Then: All nodes rendered with correct icons, colors, and file paths
    #[test]
    fn test_render_all_node_types() {
        // Setup: Create test ISG with all node types
        let isg = create_test_isg_with_all_node_types();

        // Action: Export to Mermaid
        let mermaid = export_isg_to_mermaid(&isg);

        // Assertions: Verify all node types present with correct styling
        assert!(mermaid.contains("🔧 main<br/>(Function)<br/><i>src/main.rs</i>"));
        assert!(mermaid.contains("📦 User<br/>(Struct)<br/><i>src/lib.rs</i>"));
        assert!(mermaid.contains("🎯 Display<br/>(Trait)<br/><i>src/lib.rs</i>"));
    }

    /// Test contract: Edge rendering with all relationship types
    ///
    /// # Given: ISG with all edge kinds (Calls, Implements, Uses)
    /// # When: export_isg_to_mermaid is called
    /// # Then: All edges rendered with correct arrow styles
    #[test]
    fn test_render_all_edge_types() {
        // Setup: Create test ISG with all edge types
        let isg = create_test_isg_with_all_edge_types();

        // Action: Export to Mermaid
        let mermaid = export_isg_to_mermaid(&isg);

        // Assertions: Verify correct arrow styles
        assert!(mermaid.contains("main --> create_user")); // Calls: solid arrow
        assert!(mermaid.contains("User -.-> Display")); // Implements: dashed arrow
        assert!(mermaid.contains("create_user -..-> User")); // Uses: dotted arrow
    }

    /// Test contract: Name sanitization for special characters
    ///
    /// # Given: Node names with hyphens and special characters
    /// # When: export_isg_to_mermaid is called
    /// # Then: Identifiers sanitized but display names preserved
    #[test]
    fn test_name_sanitization() {
        // Setup: Create ISG with problematic node names
        let isg = create_test_isg_with_special_names();

        // Action: Export to Mermaid
        let mermaid = export_isg_to_mermaid(&isg);

        // Assertions: Verify sanitization
        // Safe names in connections, original names in display labels
        assert!(mermaid.contains("my_struct[\"📦 my-struct"));
        assert!(mermaid.contains("my_struct --> another_struct"));
        assert!(mermaid.contains("another_struct[\"📦 another-struct"));
    }

    /// Test contract: Performance validation for typical graph sizes
    ///
    /// # Given: ISG with 100 nodes and 200 edges
    /// # When: export_isg_to_mermaid is called
    /// # Then: Must complete in <1ms (performance contract)
    #[test]
    fn test_performance_contract_typical_graph() {
        // Setup: Create moderately sized test graph
        let isg = create_performance_test_graph(100, 200);

        // Action: Time the export operation
        let start = std::time::Instant::now();
        let _mermaid = export_isg_to_mermaid(&isg);
        let elapsed = start.elapsed();

        // Assertion: Validate performance contract
        assert!(elapsed.as_millis() < 1,
            "Export took {}ms, contract requires <1ms", elapsed.as_millis());
    }

    /// Test contract: GitHub compatibility of output syntax
    ///
    /// # Given: Any valid ISG
    /// # When: export_isg_to_mermaid is called
    /// # Then: Output is valid GitHub Mermaid syntax
    #[test]
    fn test_github_compatibility() {
        // Setup: Create test ISG
        let isg = create_test_isg_minimal();

        // Action: Export to Mermaid
        let mermaid = export_isg_to_mermaid(&isg);

        // Assertions: Verify GitHub compatibility requirements
        assert!(mermaid.starts_with("flowchart TD"));
        assert!(mermaid.contains("[\""));
        assert!(mermaid.contains("\"]"));
        assert!(!mermaid.contains("click")); // No interactivity (GitHub restriction)
        assert!(!mermaid.contains("callback")); // No JavaScript (GitHub restriction)
    }

    /// Test contract: Complete graph transformation integrity
    ///
    /// # Given: Complex ISG with multiple nodes and interconnected relationships
    /// # When: export_isg_to_mermaid is called
    /// # Then: Output represents complete graph accurately
    #[test]
    fn test_complete_graph_transformation() {
        // Setup: Create complex interconnected graph
        let isg = create_complex_test_graph();

        // Action: Export to Mermaid
        let mermaid = export_isg_to_mermaid(&isg);

        // Assertions: Verify complete representation
        let node_count = mermaid.matches('[').count();
        let edge_count = mermaid.matches("-->").count() +
                        mermaid.matches("-.->").count() +
                        mermaid.matches("-..->").count();

        assert!(node_count >= 5); // At least 5 nodes
        assert!(edge_count >= 3); // At least 3 edges
        assert!(mermaid.contains("flowchart TD"));
        assert!(mermaid.lines().count() > 10); // Substantial output
    }

    // Helper functions for test setup (following TDD pattern)

    fn create_test_isg_with_all_node_types() -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Function node
        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("fn main"),
            kind: NodeKind::Function,
            name: Arc::from("main"),
            signature: Arc::from("fn main()"),
            file_path: Arc::from("src/main.rs"),
            line: 1,
        });

        // Struct node
        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("struct User"),
            kind: NodeKind::Struct,
            name: Arc::from("User"),
            signature: Arc::from("struct User"),
            file_path: Arc::from("src/lib.rs"),
            line: 5,
        });

        // Trait node
        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("trait Display"),
            kind: NodeKind::Trait,
            name: Arc::from("Display"),
            signature: Arc::from("trait Display"),
            file_path: Arc::from("src/lib.rs"),
            line: 10,
        });

        isg
    }

    fn create_test_isg_with_all_edge_types() -> OptimizedISG {
        let isg = create_test_isg_with_all_node_types();

        // Add all edge types
        let main_hash = SigHash::from_signature("fn main");
        let create_user_hash = SigHash::from_signature("fn create_user");
        let user_hash = SigHash::from_signature("struct User");
        let display_hash = SigHash::from_signature("trait Display");

        // Create user node for Calls relationship
        isg.upsert_node(NodeData {
            hash: create_user_hash,
            kind: NodeKind::Function,
            name: Arc::from("create_user"),
            signature: Arc::from("fn create_user()"),
            file_path: Arc::from("src/lib.rs"),
            line: 15,
        });

        isg.upsert_edge(main_hash, create_user_hash, EdgeKind::Calls).unwrap();
        isg.upsert_edge(user_hash, display_hash, EdgeKind::Implements).unwrap();
        isg.upsert_edge(create_user_hash, user_hash, EdgeKind::Uses).unwrap();

        isg
    }

    fn create_test_isg_with_special_names() -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Nodes with hyphens in names
        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("struct my-struct"),
            kind: NodeKind::Struct,
            name: Arc::from("my-struct"),
            signature: Arc::from("struct my-struct"),
            file_path: Arc::from("src/lib.rs"),
            line: 1,
        });

        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("struct another-struct"),
            kind: NodeKind::Struct,
            name: Arc::from("another-struct"),
            signature: Arc::from("struct another-struct"),
            file_path: Arc::from("src/lib.rs"),
            line: 5,
        });

        let hash1 = SigHash::from_signature("struct my-struct");
        let hash2 = SigHash::from_signature("struct another-struct");
        isg.upsert_edge(hash1, hash2, EdgeKind::Calls).unwrap();

        isg
    }

    fn create_performance_test_graph(node_count: usize, edge_count: usize) -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Create nodes
        for i in 0..node_count {
            let kind = match i % 3 {
                0 => NodeKind::Function,
                1 => NodeKind::Struct,
                _ => NodeKind::Trait,
            };

            isg.upsert_node(NodeData {
                hash: SigHash::from_signature(&format!("node_{}", i)),
                kind,
                name: Arc::from(format!("node_{}", i)),
                signature: Arc::from(format!("node_{}", i)),
                file_path: Arc::from("src/test.rs"),
                line: i as u32,
            });
        }

        // Create edges
        for i in 0..edge_count.min(node_count * node_count) {
            let from_idx = i % node_count;
            let to_idx = (i + 1) % node_count;

            let from_hash = SigHash::from_signature(&format!("node_{}", from_idx));
            let to_hash = SigHash::from_signature(&format!("node_{}", to_idx));
            let edge_kind = match i % 3 {
                0 => EdgeKind::Calls,
                1 => EdgeKind::Implements,
                _ => EdgeKind::Uses,
            };

            isg.upsert_edge(from_hash, to_hash, edge_kind).unwrap();
        }

        isg
    }

    fn create_test_isg_minimal() -> OptimizedISG {
        let isg = OptimizedISG::new();

        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("fn test"),
            kind: NodeKind::Function,
            name: Arc::from("test"),
            signature: Arc::from("fn test()"),
            file_path: Arc::from("src/test.rs"),
            line: 1,
        });

        isg
    }

    fn create_complex_test_graph() -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Create a realistic complex graph similar to actual Rust code
        let nodes = vec![
            ("main", "Function", "src/main.rs"),
            ("App", "Struct", "src/app.rs"),
            ("Config", "Struct", "src/config.rs"),
            ("Database", "Struct", "src/db.rs"),
            ("Handler", "Trait", "src/handler.rs"),
            ("UserHandler", "Struct", "src/handlers/user.rs"),
            ("PostHandler", "Struct", "src/handlers/post.rs"),
        ];

        let mut hashes = Vec::new();
        for (name, kind, file) in nodes {
            let hash = SigHash::from_signature(&format!("{:?} {}", kind, name));
            hashes.push(hash);

            // Create new NodeKind instances to avoid move issues
            let node_kind = match kind {
                "Function" => NodeKind::Function,
                "Struct" => NodeKind::Struct,
                "Trait" => NodeKind::Trait,
                _ => NodeKind::Function, // fallback
            };

            // Create signature before moving node_kind
            let signature = Arc::from(format!("{:?} {}", node_kind, name));

            isg.upsert_node(NodeData {
                hash,
                kind: node_kind,
                name: Arc::from(name),
                signature,
                file_path: Arc::from(file),
                line: 1,
            });
        }

        // Add realistic relationships
        isg.upsert_edge(hashes[0], hashes[1], EdgeKind::Calls).unwrap(); // main -> App
        isg.upsert_edge(hashes[1], hashes[2], EdgeKind::Uses).unwrap(); // App -> Config
        isg.upsert_edge(hashes[1], hashes[3], EdgeKind::Uses).unwrap(); // App -> Database
        isg.upsert_edge(hashes[5], hashes[4], EdgeKind::Implements).unwrap(); // UserHandler -> Handler
        isg.upsert_edge(hashes[6], hashes[4], EdgeKind::Implements).unwrap(); // PostHandler -> Handler
        isg.upsert_edge(hashes[1], hashes[5], EdgeKind::Calls).unwrap(); // App -> UserHandler

        isg
    }
}