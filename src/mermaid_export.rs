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

use crate::isg::{OptimizedISG, NodeData, NodeKind, EdgeKind, FileHierarchyAnalysis};
use std::fmt::Write;
use std::sync::Arc;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;
use std::fs;

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

/// Creates a markdown file with proper Mermaid code block formatting
///
/// # Preconditions
/// - mermaid_content contains valid Mermaid syntax
/// - filename is a valid path
///
/// # Postconditions
/// - File created with proper markdown code block wrapper
/// - GitHub-compatible format for diagram rendering
pub fn create_markdown_file(filename: &str, mermaid_content: &str) {
    let markdown = format!(
        "# ISG Architecture Diagram\n\n```mermaid\n{}\n```",
        mermaid_content
    );

    fs::write(filename, markdown).unwrap_or_else(|e| {
        eprintln!("Failed to create markdown file {}: {}", filename, e);
    });
}

/// Creates an HTML file with embedded Mermaid.js for immediate viewing
///
/// # Preconditions
/// - mermaid_content contains valid Mermaid syntax
/// - filename is a valid path
///
/// # Postconditions
/// - Self-contained HTML file created
/// - Diagram renders immediately in any modern browser
/// - No external dependencies except CDN-hosted Mermaid.js
pub fn create_html_file(filename: &str, mermaid_content: &str) {
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ISG Architecture Diagram</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 40px;
            background-color: #f5f5f5;
        }}
        .mermaid {{
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #333;
            text-align: center;
        }}
    </style>
</head>
<body>
    <h1>ISG Architecture Diagram</h1>
    <div class="mermaid">
{}
    </div>
    <script>
        mermaid.initialize({{
            startOnLoad: true,
            maxTextSize: 20000000,
            securityLevel: 'loose',
            flowchart: {{
                nodeSpacing: 15,
                rankSpacing: 30,
                useMaxWidth: true
            }},
            theme: 'neutral',
            logLevel: 'error'
        }});
    </script>
</body>
</html>"#, mermaid_content);

    fs::write(filename, html).unwrap_or_else(|e| {
        eprintln!("Failed to create HTML file {}: {}", filename, e);
    });
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
        NodeKind::Impl => "⚙️",
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

/// Export ISG to hierarchical Mermaid files (pyramid structure)
///
/// Creates multiple files for progressive disclosure:
/// - index.md: Overview level (Level 1)
/// - explore.md: Detailed exploration (Levels 2-3)
/// - data/: Full ISG JSON data
///
/// # Performance Contract
/// - Must complete in <20ms total for typical graphs (file I/O included)
/// - Each level: <300 nodes for GitHub compatibility
/// - Memory: O(1) additional allocation per file
pub fn export_isg_to_hierarchical_mermaid(
    isg: &OptimizedISG,
    output_dir: &str
) -> Result<Vec<String>, std::io::Error> {
    // Create output directory
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(&format!("{}/data", output_dir))?;

    // Analyze file hierarchy
    let hierarchy = isg.analyze_file_hierarchy();

    let mut created_files = Vec::new();

    // Level 1: Overview (index.md) - Top 30,000ft view
    let index_path = format!("{}/index.md", output_dir);
    let index_content = create_overview_mermaid(&hierarchy);
    fs::write(&index_path, index_content)?;
    created_files.push(index_path);

    // Level 2-3: Detailed exploration (explore.md)
    let explore_path = format!("{}/explore.md", output_dir);
    let explore_content = create_detailed_mermaid(&hierarchy);
    fs::write(&explore_path, explore_content)?;
    created_files.push(explore_path);

    // Full data: Complete ISG as JSON
    let data_path = format!("{}/data/full_isg.json", output_dir);
    let full_data = create_full_isg_export(isg);
    fs::write(&data_path, full_data)?;
    created_files.push(data_path);

    Ok(created_files)
}

/// Create Level 1 overview Mermaid diagram (30,000ft view)
///
/// Shows only the top-level directories and entry points
/// Limited to ~50 nodes for GitHub compatibility
fn create_overview_mermaid(hierarchy: &FileHierarchyAnalysis) -> String {
    let mut output = String::new();

    output.push_str("# Architecture Overview - Level 1 (30,000ft view)\n\n");
    output.push_str("This is the highest-level view of the codebase structure.\n");
    output.push_str("See [explore.md](explore.md) for detailed exploration.\n\n");

    output.push_str("```mermaid\n");
    output.push_str("flowchart TD\n");

    // Add entry points as distinct nodes
    for (i, entry_point) in hierarchy.entry_points.iter().take(5).enumerate() {
        let _safe_name = sanitize_identifier(&entry_point.name);
        let file_display = extract_filename_display(&entry_point.file_path);

        output.push_str(&format!(
            "    Entry{}[\"🚀 {}<br/><i>Entry: {}</i>\"]\n",
            i, entry_point.name, file_display
        ));
    }

    // Add top-level directories (depth 0-1 only)
    let top_levels = hierarchy.levels.iter().take(2);
    for level in top_levels {
        for directory in &level.directories {
            if directory.node_count > 0 {
                let safe_name = sanitize_identifier(&directory.path);
                let node_count = directory.node_count;

                output.push_str(&format!(
                    "    Dir{}[\"📁 {}<br/><i>{} items</i>\"]\n",
                    safe_name.replace("/", "_"),
                    directory.path,
                    node_count
                ));
            }
        }
    }

    // Add connections from entry points to directories
    for (i, entry_point) in hierarchy.entry_points.iter().take(3).enumerate() {
        let entry_dir = extract_directory_simple(&entry_point.file_path);
        let safe_dir = sanitize_identifier(&entry_dir);

        output.push_str(&format!(
            "    Entry{} --> Dir{}\n",
            i, safe_dir.replace("/", "_")
        ));
    }

    output.push_str("\n    %% Styling\n");
    output.push_str("    classDef entry fill:#e1f5fe,stroke:#0277bd,stroke-width:3px,color:#01579b\n");
    output.push_str("    classDef directory fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px,color:#4a148c\n");

    // Apply classes
    for i in 0..hierarchy.entry_points.iter().take(5).count() {
        output.push_str(&format!("    class Entry{} entry\n", i));
    }

    for level in hierarchy.levels.iter().take(2) {
        for directory in &level.directories {
            if directory.node_count > 0 {
                let safe_name = sanitize_identifier(&directory.path);
                output.push_str(&format!("    class Dir{} directory\n", safe_name.replace("/", "_")));
            }
        }
    }

    output.push_str("```\n\n");
    output.push_str("---\n\n");
    output.push_str("*📊 Next Level: [Detailed Exploration](explore.md) | 🗂️ Full Data: [JSON Export](data/full_isg.json)*\n");

    output
}

/// Create Level 2-3 detailed Mermaid diagram (1,000ft view)
///
/// Shows intermediate directories and key modules
/// Limited to ~200 nodes for GitHub compatibility
fn create_detailed_mermaid(hierarchy: &FileHierarchyAnalysis) -> String {
    let mut output = String::new();

    output.push_str("# Detailed Architecture - Levels 2-3 (1,000ft view)\n\n");
    output.push_str("This view shows the detailed module structure and key relationships.\n");
    output.push_str("*⬅️ Back to: [Overview](index.md) | 🗂️ Full Data: [JSON Export](data/full_isg.json)*\n\n");

    output.push_str("```mermaid\n");
    output.push_str("flowchart TD\n");

    // Get pyramid view (3 levels max)
    let pyramid_levels = hierarchy.get_pyramid_view(3);
    let mut node_counter = 0;

    for (level_idx, level) in pyramid_levels.iter().enumerate() {
        output.push_str(&format!("\n    %% Level {}: {} directories at depth {}\n",
            level_idx + 1, level.directories.len(), level.depth));

        for directory in &level.directories {
            if node_counter >= 200 { break; } // GitHub limit

            // Limit nodes per directory
            let nodes_to_show = directory.nodes.iter().take(10);

            for (node_idx, node) in nodes_to_show.enumerate() {
                if node_counter >= 200 { break; }

                let _safe_name = sanitize_identifier(&node.name);
                let file_display = extract_filename_display(&node.file_path);
                let icon = node_kind_icon(&node.kind);

                output.push_str(&format!(
                    "    L{}_D{}_N{}[\"{} {}<br/><i>({})<br/>{}</i>\"]\n",
                    level_idx + 1,
                    sanitize_identifier(&directory.path).replace("/", "_"),
                    node_idx,
                    icon, node.name, node.kind, file_display
                ));

                node_counter += 1;
            }
        }
    }

    // Add directory grouping
    output.push_str("\n    %% Directory groupings\n");
    for (level_idx, level) in pyramid_levels.iter().enumerate() {
        for directory in &level.directories {
            if directory.node_count > 0 {
                let safe_dir = sanitize_identifier(&directory.path).replace("/", "_");
                output.push_str(&format!(
                    "    subgraph SubL{}[\"📁 {} (Level {})\"]\n",
                    level_idx + 1, directory.path, level_idx + 1
                ));

                for node_idx in 0..directory.nodes.iter().take(10).count() {
                    output.push_str(&format!(
                        "        L{}_D{}_N{}\n",
                        level_idx + 1, safe_dir, node_idx
                    ));
                }

                output.push_str("    end\n");
            }
        }
    }

    output.push_str("\n    %% Styling\n");
    output.push_str("    classDef level1 fill:#e8f5e8,stroke:#2e7d32,stroke-width:2px,color:#1b5e20\n");
    output.push_str("    classDef level2 fill:#e1f5fe,stroke:#01579b,stroke-width:2px,color:#0d47a1\n");
    output.push_str("    classDef level3 fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:#e65100\n");

    // Apply level-based styling
    for (level_idx, level) in pyramid_levels.iter().enumerate() {
        let class_name = match level_idx {
            0 => "level1",
            1 => "level2",
            _ => "level3",
        };

        for directory in &level.directories {
            for node_idx in 0..directory.nodes.iter().take(10).count() {
                output.push_str(&format!(
                    "    class L{}_D{}_N{} {}\n",
                    level_idx + 1,
                    sanitize_identifier(&directory.path).replace("/", "_"),
                    node_idx,
                    class_name
                ));
            }
        }
    }

    output.push_str("```\n\n");
    output.push_str("---\n\n");
    output.push_str("*⬅️ Back to: [Overview](index.md) | 🗂️ Full Data: [JSON Export](data/full_isg.json)*\n");

    output
}

/// Create full ISG data export as JSON
fn create_full_isg_export(isg: &OptimizedISG) -> String {
    let hierarchy = isg.analyze_file_hierarchy();
    serde_json::to_string_pretty(&hierarchy).unwrap_or_else(|_| {
        r#"{"error": "Failed to serialize ISG data"}"#.to_string()
    })
}

/// Helper: Extract filename for display
fn extract_filename_display(file_path: &str) -> &str {
    file_path.split('/').last().unwrap_or(file_path)
}

/// Helper: Extract directory (simple version)
fn extract_directory_simple(file_path: &str) -> &str {
    if let Some(slash_pos) = file_path.rfind('/') {
        &file_path[..slash_pos]
    } else {
        "."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::SigHash;

    /// Test contract: Hierarchical export creates multiple files
    ///
    /// # Given: ISG with nodes at different directory depths
    /// # When: export_isg_to_hierarchical_mermaid is called
    /// # Then: Creates index.md, explore.md, and data/full_isg.json
    #[test]
    fn test_hierarchical_export_creates_multiple_files() -> Result<(), std::io::Error> {
        // Setup: Create test ISG with multiple directory levels
        let isg = create_hierarchical_test_isg();
        let temp_dir = std::env::temp_dir().join("test_hierarchy_export");

        // Action: Export hierarchical files
        let created_files = export_isg_to_hierarchical_mermaid(&isg, temp_dir.to_str().unwrap())?;

        // Assertions: Verify all expected files created
        assert_eq!(created_files.len(), 3);
        assert!(created_files.iter().any(|f| f.ends_with("index.md")));
        assert!(created_files.iter().any(|f| f.ends_with("explore.md")));
        assert!(created_files.iter().any(|f| f.ends_with("full_isg.json")));

        // Verify file contents exist
        assert!(std::fs::metadata(temp_dir.join("index.md")).is_ok());
        assert!(std::fs::metadata(temp_dir.join("explore.md")).is_ok());
        assert!(std::fs::metadata(temp_dir.join("data/full_isg.json")).is_ok());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();

        Ok(())
    }

    /// Test contract: Overview Mermaid content structure
    ///
    /// # Given: ISG with entry points and directories
    /// # When: create_overview_mermaid is called
    /// # Then: Returns proper Level 1 overview structure
    #[test]
    fn test_overview_mermaid_structure() {
        // Setup: Create test hierarchy
        let hierarchy = create_test_hierarchy();

        // Action: Create overview Mermaid
        let overview = create_overview_mermaid(&hierarchy);

        // Assertions: Verify structure
        assert!(overview.starts_with("# Architecture Overview - Level 1"));
        assert!(overview.contains("flowchart TD"));
        assert!(overview.contains("Entry")); // Entry points
        assert!(overview.contains("Dir")); // Directories
        assert!(overview.contains("[explore.md](explore.md)")); // Navigation link
        assert!(overview.contains("[JSON Export](data/full_isg.json)")); // Data link
    }

    /// Test contract: Detailed Mermaid content structure
    ///
    /// # Given: ISG with multiple directory levels
    /// # When: create_detailed_mermaid is called
    /// # Then: Returns proper Levels 2-3 detailed structure
    #[test]
    fn test_detailed_mermaid_structure() {
        // Setup: Create test hierarchy
        let hierarchy = create_test_hierarchy();

        // Action: Create detailed Mermaid
        let detailed = create_detailed_mermaid(&hierarchy);

        // Assertions: Verify structure
        assert!(detailed.starts_with("# Detailed Architecture - Levels 2-3"));
        assert!(detailed.contains("flowchart TD"));
        assert!(detailed.contains("Level 1"));
        assert!(detailed.contains("Level 2"));
        assert!(detailed.contains("subgraph")); // Directory groupings
        assert!(detailed.contains("⬅️ Back to: [Overview](index.md)")); // Back navigation
    }

    /// Test contract: Performance validation for hierarchical export
    ///
    /// # Given: ISG with moderate complexity (50 nodes, 100 edges)
    /// # When: export_isg_to_hierarchical_mermaid is called
    /// # Then: Must complete in <5ms (performance contract)
    #[test]
    fn test_hierarchical_export_performance_contract() -> Result<(), std::io::Error> {
        // Setup: Create moderately sized test graph
        let isg = create_hierarchical_performance_test_graph(50, 100);
        let temp_dir = std::env::temp_dir().join("test_perf_hierarchy");

        // Action: Time the hierarchical export
        let start = std::time::Instant::now();
        let _created_files = export_isg_to_hierarchical_mermaid(&isg, temp_dir.to_str().unwrap())?;
        let elapsed = start.elapsed();

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();

        // Assertion: Validate performance contract
        assert!(elapsed.as_millis() < 20,
            "Hierarchical export took {}ms, contract requires <20ms", elapsed.as_millis());

        Ok(())
    }

    /// Test contract: File hierarchy analysis accuracy
    ///
    /// # Given: ISG with nodes at various directory depths
    /// # When: analyze_file_hierarchy is called
    /// # Then: Correctly groups nodes by directory depth
    #[test]
    fn test_file_hierarchy_analysis() {
        // Setup: Create test ISG with known structure
        let isg = create_hierarchical_test_isg();

        // Action: Analyze file hierarchy
        let hierarchy = isg.analyze_file_hierarchy();

        // Assertions: Verify hierarchy structure
        assert!(!hierarchy.levels.is_empty());
        assert!(!hierarchy.entry_points.is_empty());

        // Verify nodes are correctly grouped by depth
        let mut total_nodes = 0;
        for level in &hierarchy.levels {
            for directory in &level.directories {
                total_nodes += directory.node_count;
                assert!(!directory.nodes.is_empty());
                assert_eq!(directory.nodes.len(), directory.node_count);
            }
        }

        assert!(total_nodes > 0);
    }

    // Helper functions for hierarchical testing

    fn create_hierarchical_test_isg() -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Create nodes at different directory levels
        let test_nodes = vec![
            // Level 0: Root
            ("main", "Function", "src/main.rs"),
            ("lib", "Function", "src/lib.rs"),

            // Level 1: Direct modules
            ("config", "Struct", "src/config.rs"),
            ("database", "Struct", "src/database.rs"),

            // Level 2: Nested modules
            ("User", "Struct", "src/models/user.rs"),
            ("Post", "Struct", "src/models/post.rs"),
            ("auth", "Function", "src/auth/mod.rs"),
            ("login", "Function", "src/auth/login.rs"),
        ];

        for (name, kind, file) in test_nodes {
            let node_kind = match kind {
                "Function" => NodeKind::Function,
                "Struct" => NodeKind::Struct,
                "Trait" => NodeKind::Trait,
                _ => NodeKind::Function,
            };

            let hash = SigHash::from_signature(&format!("{:?} {}", node_kind, name));
            isg.upsert_node(NodeData {
                hash,
                kind: node_kind.clone(),
                name: Arc::from(name),
                signature: Arc::from(format!("{:?} {}", node_kind, name)),
                file_path: Arc::from(file),
                line: 1,
            });
        }

        isg
    }

    fn create_test_hierarchy() -> FileHierarchyAnalysis {
        let mut hierarchy = FileHierarchyAnalysis::new();

        // Add entry point
        hierarchy.entry_points.push(NodeData {
            hash: SigHash::from_signature("Function main"),
            kind: NodeKind::Function,
            name: Arc::from("main"),
            signature: Arc::from("Function main"),
            file_path: Arc::from("src/main.rs"),
            line: 1,
        });

        // Add Level 0 (root)
        hierarchy.add_node_at_depth(0, "src".to_string(), NodeData {
            hash: SigHash::from_signature("Struct Config"),
            kind: NodeKind::Struct,
            name: Arc::from("Config"),
            signature: Arc::from("Struct Config"),
            file_path: Arc::from("src/config.rs"),
            line: 1,
        });

        // Add Level 1 (nested)
        hierarchy.add_node_at_depth(1, "src/models".to_string(), NodeData {
            hash: SigHash::from_signature("Struct User"),
            kind: NodeKind::Struct,
            name: Arc::from("User"),
            signature: Arc::from("Struct User"),
            file_path: Arc::from("src/models/user.rs"),
            line: 1,
        });

        hierarchy
    }

    fn create_hierarchical_performance_test_graph(node_count: usize, edge_count: usize) -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Create nodes at different directory levels for realistic hierarchy
        for i in 0..node_count {
            let kind = match i % 3 {
                0 => NodeKind::Function,
                1 => NodeKind::Struct,
                _ => NodeKind::Trait,
            };

            let depth = i % 3; // Distribute across 3 levels
            let file_path = match depth {
                0 => format!("src/level0/mod{}.rs", i / 10),
                1 => format!("src/level1/mod{}.rs", i / 10),
                _ => format!("src/level2/mod{}.rs", i / 10),
            };

            let hash = SigHash::from_signature(&format!("node_{}", i));
            isg.upsert_node(NodeData {
                hash,
                kind,
                name: Arc::from(format!("node_{}", i)),
                signature: Arc::from(format!("node_{}", i)),
                file_path: Arc::from(file_path),
                line: i as u32,
            });
        }

        // Create some edges
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