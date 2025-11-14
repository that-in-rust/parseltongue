//! Mermaid diagram generation for dependency graphs
//!
//! # Purpose
//!
//! Generates GitHub-native Mermaid diagrams with semantic edge directionality.
//! Based on research findings: PDG/SDG (Program/System Dependence Graph) model.
//!
//! # Edge Semantics
//!
//! Different edge types have different visual representations:
//! - `Calls`, `Uses`: `-->` (solid arrow, horizontal peer-to-peer)
//! - `Implements`: `-.->` (dotted arrow, upward concrete → abstract)
//! - Future extensions: `==>` (thick arrow, downward abstract → concrete)
//!
//! # Design Decision (Research-Based)
//!
//! Mermaid is used for VISUALIZATION, not as primary data format:
//! - ✅ PRIMARY: JSON with typed edges (canonical, queryable)
//! - ✅ RENDER: Mermaid from JSON (human-readable, GitHub-native)
//! - ❌ AVOID: Mermaid as storage (no schema validation, lossy)

use crate::entities::{CodeEntity, DependencyEdge, EdgeType, EntityClass};
use anyhow::Result;
use std::collections::HashSet;

/// Configuration for Mermaid diagram generation
#[derive(Debug, Clone)]
pub struct MermaidConfig {
    /// Maximum nodes to include (prevents huge diagrams)
    pub max_nodes: usize,
    /// Maximum edges to include (prevents visual clutter)
    pub max_edges: usize,
    /// Include entity labels (false = only keys)
    pub include_labels: bool,
    /// Markdown wrapper (true = wrap in ```mermaid ... ```)
    pub markdown_wrapped: bool,
}

impl Default for MermaidConfig {
    fn default() -> Self {
        Self {
            max_nodes: 100,
            max_edges: 200,
            include_labels: true,
            markdown_wrapped: true,
        }
    }
}

/// Mermaid diagram generator for dependency graphs
///
/// # Contract
///
/// **Preconditions**:
/// - Entities and edges slices are valid (may be empty)
/// - Edge keys reference entities in the entity slice
///
/// **Postconditions**:
/// - Returns valid Mermaid graph syntax
/// - Edge directionality matches semantic meaning
/// - Respects max_nodes/max_edges limits
///
/// **Example**:
/// ```rust,ignore
/// let config = MermaidConfig::default();
/// let mermaid = render_graph_as_mermaid(&entities, &edges, &config)?;
/// // Output: ```mermaid\ngraph TD\n  A -->|Calls| B\n```
/// ```
pub fn render_graph_as_mermaid(
    entities: &[CodeEntity],
    edges: &[DependencyEdge],
    config: &MermaidConfig,
) -> Result<String> {
    let mut output = String::new();

    // Markdown wrapper (optional)
    if config.markdown_wrapped {
        output.push_str("```mermaid\n");
    }

    // Graph declaration (TD = top-down layout)
    output.push_str("graph TD\n");

    // Filter entities referenced by edges (avoid orphans)
    let referenced_keys = collect_referenced_keys(edges);
    let filtered_entities: Vec<_> = entities
        .iter()
        .filter(|e| referenced_keys.contains(e.isgl1_key.as_str()))
        .take(config.max_nodes)
        .collect();

    // Render nodes (optional, for explicit labels)
    if config.include_labels && filtered_entities.len() < 50 {
        for entity in &filtered_entities {
            let node_id = sanitize_node_id(&entity.isgl1_key);
            let node_label = sanitize_label(&get_entity_name(entity));
            output.push_str(&format!("    {}[\"{}\"]\n", node_id, node_label));
        }
        output.push('\n');
    }

    // Render edges with semantic arrow styles
    let truncated_edges = &edges[..edges.len().min(config.max_edges)];
    for edge in truncated_edges {
        let from_id = sanitize_node_id(edge.from_key.as_str());
        let to_id = sanitize_node_id(edge.to_key.as_str());
        let arrow = edge_type_to_arrow(edge.edge_type);
        let label = edge.edge_type.as_str();

        output.push_str(&format!(
            "    {} {}|\"{}\"| {}\n",
            from_id, arrow, label, to_id
        ));
    }

    // Close markdown wrapper
    if config.markdown_wrapped {
        output.push_str("```\n");
    }

    Ok(output)
}

/// Map EdgeType to Mermaid arrow syntax based on semantic directionality
///
/// # Semantics
///
/// - **Implements** (concrete → abstract): Dotted arrow (upward)
/// - **Calls** (peer → peer): Solid arrow (horizontal)
/// - **Uses** (consumer → provider): Solid arrow (horizontal)
///
/// # Future Extensions
///
/// When EdgeType enum is extended (Phase 2):
/// - `Extends`: `-.->` (dotted, upward like Implements)
/// - `Instantiates`: `==>` (thick, downward abstract → concrete)
/// - `Contains`: `-->` (solid, structural)
fn edge_type_to_arrow(edge_type: EdgeType) -> &'static str {
    match edge_type {
        EdgeType::Implements => "-.->", // Dotted: upward (concrete → abstract)
        EdgeType::Calls => "-->",       // Solid: horizontal (caller → callee)
        EdgeType::Uses => "-->",        // Solid: horizontal (consumer → provider)
    }
}

/// Collect all entity keys referenced in edges
fn collect_referenced_keys(edges: &[DependencyEdge]) -> HashSet<&str> {
    let mut keys = HashSet::new();
    for edge in edges {
        keys.insert(edge.from_key.as_str());
        keys.insert(edge.to_key.as_str());
    }
    keys
}

/// Sanitize entity key for use as Mermaid node ID
///
/// Mermaid node IDs must be alphanumeric + underscore.
/// ISG keys like "rust:fn:main:src_main_rs:1-10" → "rust_fn_main_src_main_rs_1_10"
fn sanitize_node_id(key: &str) -> String {
    key.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

/// Sanitize label text for Mermaid (escape quotes, limit length)
fn sanitize_label(label: &str) -> String {
    let escaped = label.replace('"', "&quot;");
    if escaped.len() > 50 {
        format!("{}...", &escaped[..47])
    } else {
        escaped
    }
}

/// Extract entity name from CodeEntity's interface signature
fn get_entity_name(entity: &CodeEntity) -> String {
    // Try to extract name from interface signature JSON
    if let Ok(sig) = serde_json::from_str::<serde_json::Value>(&entity.interface_signature) {
        if let Some(name) = sig.get("name").and_then(|v| v.as_str()) {
            return name.to_string();
        }
    }
    // Fallback: extract from isgl1_key (e.g., "rust:fn:main:..." → "main")
    entity.isgl1_key
        .split(':')
        .nth(2)
        .unwrap_or(&entity.isgl1_key)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{DependencyEdge, EdgeType, CodeEntity, EntityClass, InterfaceSignature, TemporalState, EntityMetadata, TddClassification, LspMetadata};

    fn create_test_entity(name: &str, key: &str) -> CodeEntity {
        CodeEntity {
            isgl1_key: key.to_string(),
            temporal_state: TemporalState::Current,
            interface_signature: InterfaceSignature::Function {
                name: name.to_string(),
                parameters: vec![],
                return_type: "()".to_string(),
                visibility: crate::entities::Visibility::Public,
                is_async: false,
                generics: vec![],
                where_clauses: vec![],
            },
            current_code: None,
            future_code: None,
            tdd_classification: TddClassification::Implementation,
            lsp_metadata: None,
            metadata: EntityMetadata {
                file_path: "./test.rs".to_string(),
                line_range: Some((1, 10)),
                language: crate::entities::Language::Rust,
            },
            entity_class: EntityClass::CodeImplementation,
        }
    }

    fn create_test_edge(from: &str, to: &str, edge_type: EdgeType) -> DependencyEdge {
        DependencyEdge::new(from, to, edge_type, None).unwrap()
    }

    #[test]
    fn test_sanitize_node_id() {
        assert_eq!(
            sanitize_node_id("rust:fn:main:src_main_rs:1-10"),
            "rust_fn_main_src_main_rs_1_10"
        );
        assert_eq!(sanitize_node_id("simple_key"), "simple_key");
    }

    #[test]
    fn test_sanitize_label() {
        assert_eq!(sanitize_label("process_data"), "process_data");
        assert_eq!(
            sanitize_label("func_with_\"quotes\""),
            "func_with_&quot;quotes&quot;"
        );

        let long_name = "a".repeat(60);
        let sanitized = sanitize_label(&long_name);
        assert!(sanitized.ends_with("..."));
        assert_eq!(sanitized.len(), 50);
    }

    #[test]
    fn test_edge_type_to_arrow() {
        assert_eq!(edge_type_to_arrow(EdgeType::Calls), "-->");
        assert_eq!(edge_type_to_arrow(EdgeType::Uses), "-->");
        assert_eq!(edge_type_to_arrow(EdgeType::Implements), "-.->"); // Dotted for upward
    }

    #[test]
    fn test_render_simple_graph() {
        let entities = vec![
            create_test_entity("main", "rust:fn:main:src_main_rs:1-10"),
            create_test_entity("helper", "rust:fn:helper:src_main_rs:20-30"),
        ];

        let edges = vec![create_test_edge(
            "rust:fn:main:src_main_rs:1-10",
            "rust:fn:helper:src_main_rs:20-30",
            EdgeType::Calls,
        )];

        let config = MermaidConfig {
            max_nodes: 10,
            max_edges: 10,
            include_labels: true,
            markdown_wrapped: true,
        };

        let result = render_graph_as_mermaid(&entities, &edges, &config).unwrap();

        assert!(result.contains("```mermaid"));
        assert!(result.contains("graph TD"));
        assert!(result.contains("-->|\"Calls\"|"));
        assert!(result.contains("```\n"));
    }

    #[test]
    fn test_render_implements_edge() {
        let entities = vec![
            create_test_entity("MyStruct", "rust:struct:MyStruct:src_lib_rs:1-10"),
            create_test_entity("MyTrait", "rust:trait:MyTrait:src_lib_rs:20-30"),
        ];

        let edges = vec![create_test_edge(
            "rust:struct:MyStruct:src_lib_rs:1-10",
            "rust:trait:MyTrait:src_lib_rs:20-30",
            EdgeType::Implements,
        )];

        let config = MermaidConfig::default();
        let result = render_graph_as_mermaid(&entities, &edges, &config).unwrap();

        // Implements should use dotted arrow (upward: concrete → abstract)
        assert!(result.contains("-.->|\"Implements\"|"));
    }

    #[test]
    fn test_empty_graph() {
        let entities = vec![];
        let edges = vec![];
        let config = MermaidConfig::default();

        let result = render_graph_as_mermaid(&entities, &edges, &config).unwrap();

        assert!(result.contains("```mermaid"));
        assert!(result.contains("graph TD"));
        assert!(result.contains("```\n"));
    }

    #[test]
    fn test_max_edges_limit() {
        let entities = vec![
            create_test_entity("a", "rust:fn:a:src_lib_rs:1-5"),
            create_test_entity("b", "rust:fn:b:src_lib_rs:6-10"),
            create_test_entity("c", "rust:fn:c:src_lib_rs:11-15"),
        ];

        let edges = vec![
            create_test_edge("rust:fn:a:src_lib_rs:1-5", "rust:fn:b:src_lib_rs:6-10", EdgeType::Calls),
            create_test_edge("rust:fn:b:src_lib_rs:6-10", "rust:fn:c:src_lib_rs:11-15", EdgeType::Calls),
            create_test_edge("rust:fn:a:src_lib_rs:1-5", "rust:fn:c:src_lib_rs:11-15", EdgeType::Uses),
        ];

        let config = MermaidConfig {
            max_edges: 2, // Limit to 2 edges
            ..Default::default()
        };

        let result = render_graph_as_mermaid(&entities, &edges, &config).unwrap();

        // Should only contain 2 edges
        let edge_count = result.matches("-->").count() + result.matches("-.->").count();
        assert_eq!(edge_count, 2);
    }
}
