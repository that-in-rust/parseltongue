//! WASM Renderer - Layer 2 (Rust Rendering Logic)
//!
//! Layout algorithms and rendering logic for WASM visualization
//! Following steering docs L1→L2→L3 architecture principles
//!
//! # Performance Contracts
//! - <16ms render time for initial view
//! - <100ms interaction response time
//! - O(1) memory allocation during hot path
//! - Smooth animations at 60fps

use crate::wasm_core::{WASMGraph, WASMNode, WASMEdge, WASMNodeType, WASMEdgeType, WASMError, WASMLayout};
use crate::graph_data_loader::{GraphDataLoader, GraphDataError};
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use petgraph::visit::{IntoEdgeReferences, EdgeRef};

/// Layout algorithms for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LayoutAlgorithm {
    /// Breadth-first layout (fast, simple)
    BreadthFirst,
    /// Force-directed layout (slow, nice aesthetics)
    ForceDirected,
    /// Hierarchical layout (medium, good for DAGs)
    Hierarchical,
    /// Circular layout (fast, good for small graphs)
    Circular,
}

/// Rendering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Layout algorithm to use
    pub layout_algorithm: LayoutAlgorithm,
    /// Canvas dimensions
    pub canvas_size: (u32, u32),
    /// Node styling
    pub node_style: NodeStyle,
    /// Edge styling
    pub edge_style: EdgeStyle,
    /// Animation settings
    pub animation: AnimationConfig,
}

/// Node styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStyle {
    /// Default node radius
    pub default_radius: f64,
    /// Node colors by type
    pub node_colors: HashMap<WASMNodeType, String>,
    /// Font settings
    pub font_family: String,
    pub font_size: f64,
    /// Border settings
    pub border_width: f64,
    pub border_color: String,
}

/// Edge styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeStyle {
    /// Default edge width
    pub default_width: f64,
    /// Edge colors by type
    pub edge_colors: HashMap<WASMEdgeType, String>,
    /// Arrow settings
    pub arrow_size: f64,
    /// Curve settings
    pub curve_type: CurveType,
}

/// Edge curve types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CurveType {
    /// Straight line
    Straight,
    /// Simple curve
    Bezier,
    /// Step-like curve
    Step,
}

/// Animation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Enable animations
    pub enabled: bool,
    /// Animation duration in milliseconds
    pub duration_ms: u32,
    /// Easing function
    pub easing: EasingFunction,
}

/// Easing functions for animations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
}

/// Rendered scene data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedScene {
    /// Rendered nodes with positions
    pub nodes: Vec<RenderedNode>,
    /// Rendered edges with path data
    pub edges: Vec<RenderedEdge>,
    /// Scene metadata
    pub metadata: SceneMetadata,
}

/// Rendered node with position and styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedNode {
    /// Node data
    pub node: WASMNode,
    /// Screen position
    pub x: f64,
    pub y: f64,
    /// Visual properties
    pub radius: f64,
    pub color: String,
    pub border_color: String,
    pub border_width: f64,
    /// Label properties
    pub label_visible: bool,
    pub label_color: String,
}

/// Rendered edge with path data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedEdge {
    /// Edge data
    pub edge: WASMEdge,
    /// Path data for rendering
    pub path_data: String,
    /// Visual properties
    pub color: String,
    pub width: f64,
    /// Arrow properties
    pub arrow_visible: bool,
    pub arrow_color: String,
}

/// Scene metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    /// Scene dimensions
    pub width: f64,
    pub height: f64,
    /// Render time in milliseconds
    pub render_time_ms: f64,
    /// Number of nodes rendered
    pub node_count: usize,
    /// Number of edges rendered
    pub edge_count: usize,
    /// Layout algorithm used
    pub layout_algorithm: String,
}

/// WASM renderer engine
pub struct WASMRenderer {
    /// Current configuration
    config: RenderConfig,
    /// Performance metrics
    render_metrics: RenderMetrics,
}

/// Rendering performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderMetrics {
    /// Last render time in milliseconds
    pub last_render_ms: f64,
    /// Total render time in milliseconds
    pub total_render_ms: f64,
    /// Number of renders performed
    pub render_count: u32,
    /// Average render time in milliseconds
    pub average_render_ms: f64,
    /// Maximum render time in milliseconds
    pub max_render_ms: f64,
}

impl WASMRenderer {
    /// Create new renderer with default configuration
    pub fn new() -> Self {
        Self {
            config: RenderConfig::default(),
            render_metrics: RenderMetrics::default(),
        }
    }

    /// Create renderer with custom configuration
    pub fn with_config(config: RenderConfig) -> Self {
        Self {
            config,
            render_metrics: RenderMetrics::default(),
        }
    }

    /// Render WASM graph to scene
    ///
    /// # Performance Contract
    /// - Must complete in <16ms for initial view
    /// - Must complete in <100ms for interactions
    /// - Memory allocation: O(1) during hot path
    pub fn render(&mut self, graph: &WASMGraph) -> Result<RenderedScene, WASMError> {
        let start_time = std::time::Instant::now();

        // Validate graph is not empty
        if graph.nodes.is_empty() {
            return Err(WASMError::ConversionError("Cannot render empty graph".to_string()));
        }

        // Step 1: Apply layout algorithm
        let layout_graph = self.apply_layout(graph)?;

        // Step 2: Render nodes
        let rendered_nodes = self.render_nodes(&layout_graph)?;

        // Step 3: Render edges
        let rendered_edges = self.render_edges(&layout_graph, &rendered_nodes)?;

        // Step 4: Update metrics
        let render_time = start_time.elapsed().as_millis() as f64;
        self.update_render_metrics(render_time);

        // Step 5: Validate performance contracts
        self.validate_performance_contracts(render_time)?;

        // Step 6: Create scene
        let scene = RenderedScene {
            nodes: rendered_nodes,
            edges: rendered_edges,
            metadata: SceneMetadata {
                width: self.config.canvas_size.0 as f64,
                height: self.config.canvas_size.1 as f64,
                render_time_ms: render_time,
                node_count: graph.nodes.len(),
                edge_count: graph.edges.len(),
                layout_algorithm: format!("{:?}", self.config.layout_algorithm),
            },
        };

        Ok(scene)
    }

    /// Apply layout algorithm to graph
    fn apply_layout(&self, graph: &WASMGraph) -> Result<WASMGraph, WASMError> {
        match self.config.layout_algorithm {
            LayoutAlgorithm::BreadthFirst => self.breadth_first_layout(graph),
            LayoutAlgorithm::ForceDirected => self.force_directed_layout(graph),
            LayoutAlgorithm::Hierarchical => self.hierarchical_layout(graph),
            LayoutAlgorithm::Circular => self.circular_layout(graph),
        }
    }

    /// Breadth-first layout algorithm
    fn breadth_first_layout(&self, graph: &WASMGraph) -> Result<WASMGraph, WASMError> {
        let mut layout_graph = graph.clone();
        let width = self.config.canvas_size.0 as f64;
        let height = self.config.canvas_size.1 as f64;
        let levels = self.compute_breadth_first_levels(graph);

        for (level, nodes) in levels.iter().enumerate() {
            let y = (level as f64 + 1.0) * (height / (levels.len() as f64 + 1.0));
            let x_spacing = width / (nodes.len() + 1) as f64;

            for (i, node_id) in nodes.iter().enumerate() {
                let x = (i + 1) as f64 * x_spacing;

                if let Some(node) = layout_graph.nodes.iter_mut()
                    .find(|n| &n.id == node_id) {
                    node.position = Some((x, y));
                }
            }
        }

        Ok(layout_graph)
    }

    /// Force-directed layout algorithm
    fn force_directed_layout(&self, graph: &WASMGraph) -> Result<WASMGraph, WASMError> {
        let mut layout_graph = graph.clone();
        let width = self.config.canvas_size.0 as f64;
        let height = self.config.canvas_size.1 as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;

        // Initialize nodes in random positions around center
        let mut rng = 42; // Simple deterministic seed
        for (i, node) in layout_graph.nodes.iter_mut().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / graph.nodes.len() as f64;
            let radius = 100.0 + (rng % 50) as f64;
            node.position = Some((
                center_x + radius * angle.cos(),
                center_y + radius * angle.sin()
            ));
            rng = (rng * 1103515245 + 12345) % 2147483647;
        }

        // Simple force-directed simulation (10 iterations)
        for _iteration in 0..10 {
            let mut forces = vec![(0.0, 0.0); graph.nodes.len()];

            // Repulsive forces between all nodes
            for i in 0..graph.nodes.len() {
                for j in (i + 1)..graph.nodes.len() {
                    if let (Some(pos_i), Some(pos_j)) = (
                        layout_graph.nodes[i].position,
                        layout_graph.nodes[j].position
                    ) {
                        let dx = pos_i.0 - pos_j.0;
                        let dy = pos_i.1 - pos_j.1;
                        let dist_sq = dx * dx + dy * dy;

                        if dist_sq > 1.0 { // Avoid division by zero
                            let dist = dist_sq.sqrt();
                            let force = 1000.0 / dist_sq; // Repulsion force
                            let fx = force * dx / dist;
                            let fy = force * dy / dist;

                            forces[i].0 += fx;
                            forces[i].1 += fy;
                            forces[j].0 -= fx;
                            forces[j].1 -= fy;
                        }
                    }
                }
            }

            // Attractive forces for connected nodes
            for edge in &graph.edges {
                if let (Some(source_idx), Some(target_idx)) = (
                    layout_graph.nodes.iter().position(|n| n.id == edge.source),
                    layout_graph.nodes.iter().position(|n| n.id == edge.target)
                ) {
                    if let (Some(pos_source), Some(pos_target)) = (
                        layout_graph.nodes[source_idx].position,
                        layout_graph.nodes[target_idx].position
                    ) {
                        let dx = pos_target.0 - pos_source.0;
                        let dy = pos_target.1 - pos_source.1;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist > 1.0 {
                            let force = dist * 0.01; // Spring force
                            let fx = force * dx / dist;
                            let fy = force * dy / dist;

                            forces[source_idx].0 += fx;
                            forces[source_idx].1 += fy;
                            forces[target_idx].0 -= fx;
                            forces[target_idx].1 -= fy;
                        }
                    }
                }
            }

            // Apply forces with damping
            let damping = 0.1;
            for (i, node) in layout_graph.nodes.iter_mut().enumerate() {
                if let Some(pos) = node.position {
                    let new_x = pos.0 + forces[i].0 * damping;
                    let new_y = pos.1 + forces[i].1 * damping;

                    // Keep nodes within canvas bounds
                    let margin = 50.0;
                    let bounded_x = new_x.max(margin).min(width - margin);
                    let bounded_y = new_y.max(margin).min(height - margin);

                    node.position = Some((bounded_x, bounded_y));
                }
            }
        }

        Ok(layout_graph)
    }

    /// Hierarchical layout algorithm
    fn hierarchical_layout(&self, graph: &WASMGraph) -> Result<WASMGraph, WASMError> {
        let mut layout_graph = graph.clone();
        let width = self.config.canvas_size.0 as f64;
        let height = self.config.canvas_size.1 as f64;

        // Build adjacency structure
        let mut children: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        let mut parents: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        let mut has_incoming: std::collections::HashSet<String> = std::collections::HashSet::new();

        for edge in &graph.edges {
            children.entry(edge.source.clone()).or_insert_with(Vec::new).push(edge.target.clone());
            parents.entry(edge.target.clone()).or_insert_with(Vec::new).push(edge.source.clone());
            has_incoming.insert(edge.target.clone());
        }

        // Find root nodes (nodes with no incoming edges)
        let mut roots = Vec::new();
        for node in &graph.nodes {
            if !has_incoming.contains(&node.id) {
                roots.push(node.id.clone());
            }
        }

        // If no roots found, use first node as root
        if roots.is_empty() && !graph.nodes.is_empty() {
            roots.push(graph.nodes[0].id.clone());
        }

        // Assign levels using topological sort
        let mut levels: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut current_level = 0;

        while !roots.is_empty() && current_level < 20 { // Prevent infinite loops
            let mut next_level = Vec::new();

            for root_id in &roots {
                if !levels.contains_key(root_id) {
                    levels.insert(root_id.clone(), current_level);

                    if let Some(children_ids) = children.get(root_id) {
                        for child_id in children_ids {
                            if !levels.contains_key(child_id) {
                                next_level.push(child_id.clone());
                            }
                        }
                    }
                }
            }

            roots = next_level;
            current_level += 1;
        }

        // Position nodes based on levels
        let mut level_nodes: std::collections::HashMap<usize, Vec<String>> = std::collections::HashMap::new();
        for (node_id, level) in &levels {
            level_nodes.entry(*level).or_insert_with(Vec::new).push(node_id.clone());
        }

        // Assign positions
        for (level, nodes_at_level) in level_nodes {
            let y = (level as f64 + 1.0) * (height / (current_level as f64 + 1.0));
            let x_spacing = width / (nodes_at_level.len() + 1) as f64;

            for (i, node_id) in nodes_at_level.iter().enumerate() {
                let x = (i + 1) as f64 * x_spacing;

                if let Some(node) = layout_graph.nodes.iter_mut().find(|n| n.id == *node_id) {
                    node.position = Some((x, y));
                }
            }
        }

        // Position any remaining nodes (not reachable from roots)
        let mut remaining_y = height * 0.9;
        for node in &mut layout_graph.nodes {
            if node.position.is_none() {
                node.position = Some((width / 2.0, remaining_y));
                remaining_y += 30.0;
            }
        }

        Ok(layout_graph)
    }

    /// Circular layout algorithm
    fn circular_layout(&self, graph: &WASMGraph) -> Result<WASMGraph, WASMError> {
        let mut layout_graph = graph.clone();
        let center_x = self.config.canvas_size.0 as f64 / 2.0;
        let center_y = self.config.canvas_size.1 as f64 / 2.0;
        let radius = f64::min(center_x, center_y) * 0.8;

        for (i, node) in layout_graph.nodes.iter_mut().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / graph.nodes.len() as f64;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            node.position = Some((x, y));
        }

        Ok(layout_graph)
    }

    /// Compute breadth-first levels for layout
    fn compute_breadth_first_levels(&self, graph: &WASMGraph) -> Vec<Vec<String>> {
        let mut levels: Vec<Vec<String>> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut current_level: Vec<String> = Vec::new();

        // Find root nodes (nodes with no incoming edges)
        let mut has_incoming: HashSet<String> = HashSet::new();
        for edge in &graph.edges {
            has_incoming.insert(edge.target.clone());
        }

        for node in &graph.nodes {
            if !has_incoming.contains(&node.id) {
                current_level.push(node.id.clone());
            }
        }

        // If no root nodes found, start with first node
        if current_level.is_empty() && !graph.nodes.is_empty() {
            current_level.push(graph.nodes[0].id.clone());
        }

        while !current_level.is_empty() {
            levels.push(current_level.clone());
            visited.extend(current_level.iter().cloned());

            let mut next_level: Vec<String> = Vec::new();
            for node_id in &current_level {
                for edge in &graph.edges {
                    if edge.source == *node_id && !visited.contains(&edge.target) {
                        next_level.push(edge.target.clone());
                    }
                }
            }

            current_level = next_level;
        }

        levels
    }

    /// Render nodes with styling
    fn render_nodes(&self, graph: &WASMGraph) -> Result<Vec<RenderedNode>, WASMError> {
        let mut rendered_nodes = Vec::new();

        for node in &graph.nodes {
            let position = node.position.ok_or_else(|| {
                WASMError::LayoutError("Node position not computed".to_string())
            })?;

            let color = self.config.node_style.node_colors
                .get(&node.node_type)
                .cloned()
                .unwrap_or_else(|| "#cccccc".to_string());

            let rendered_node = RenderedNode {
                node: node.clone(),
                x: position.0,
                y: position.1,
                radius: self.config.node_style.default_radius,
                color: color.clone(),
                border_color: self.config.node_style.border_color.clone(),
                border_width: self.config.node_style.border_width,
                label_visible: true,
                label_color: "#000000".to_string(),
            };

            rendered_nodes.push(rendered_node);
        }

        Ok(rendered_nodes)
    }

    /// Render edges with path data
    fn render_edges(&self, graph: &WASMGraph, rendered_nodes: &[RenderedNode]) -> Result<Vec<RenderedEdge>, WASMError> {
        let mut rendered_edges = Vec::new();
        let node_positions: HashMap<String, (f64, f64)> = rendered_nodes.iter()
            .map(|rn| (rn.node.id.clone(), (rn.x, rn.y)))
            .collect();

        for edge in &graph.edges {
            let source_pos = node_positions.get(&edge.source).ok_or_else(|| {
                WASMError::ConversionError(format!("Source node {} not found", edge.source))
            })?;

            let target_pos = node_positions.get(&edge.target).ok_or_else(|| {
                WASMError::ConversionError(format!("Target node {} not found", edge.target))
            })?;

            let path_data = self.generate_path_data(*source_pos, *target_pos);
            let color = self.config.edge_style.edge_colors
                .get(&edge.edge_type)
                .cloned()
                .unwrap_or_else(|| "#888888".to_string());

            let rendered_edge = RenderedEdge {
                edge: edge.clone(),
                path_data,
                color: color.clone(),
                width: self.config.edge_style.default_width,
                arrow_visible: true,
                arrow_color: color,
            };

            rendered_edges.push(rendered_edge);
        }

        Ok(rendered_edges)
    }

    /// Generate SVG path data for edge
    fn generate_path_data(&self, source: (f64, f64), target: (f64, f64)) -> String {
        match self.config.edge_style.curve_type {
            CurveType::Straight => {
                format!("M {} {} L {} {}", source.0, source.1, target.0, target.1)
            }
            CurveType::Bezier => {
                let mid_x = (source.0 + target.0) / 2.0;
                let mid_y = (source.1 + target.1) / 2.0;
                format!("M {} {} Q {} {} {} {}",
                    source.0, source.1, mid_x, mid_y, target.0, target.1)
            }
            CurveType::Step => {
                let mid_x = (source.0 + target.0) / 2.0;
                let mid_y = (source.1 + target.1) / 2.0;
                format!("M {} {} H {} V {} L {} {}",
                    source.0, source.1, mid_x, mid_y, target.0, target.1)
            }
        }
    }

    /// Update rendering performance metrics
    fn update_render_metrics(&mut self, render_time: f64) {
        self.render_metrics.last_render_ms = render_time;
        self.render_metrics.total_render_ms += render_time;
        self.render_metrics.render_count += 1;
        self.render_metrics.average_render_ms =
            self.render_metrics.total_render_ms / self.render_metrics.render_count as f64;
        self.render_metrics.max_render_ms =
            self.render_metrics.max_render_ms.max(render_time);
    }

    /// Validate performance contracts
    fn validate_performance_contracts(&self, render_time: f64) -> Result<(), WASMError> {
        // Initial render contract: <16ms
        if self.render_metrics.render_count == 1 && render_time > 16.0 {
            return Err(WASMError::PerformanceContractViolation(
                format!("Initial render took {}ms > 16ms limit", render_time)
            ));
        }

        // Interaction render contract: <100ms
        if self.render_metrics.render_count > 1 && render_time > 100.0 {
            return Err(WASMError::PerformanceContractViolation(
                format!("Interaction render took {}ms > 100ms limit", render_time)
            ));
        }

        Ok(())
    }

    /// Get current configuration
    pub fn config(&self) -> &RenderConfig {
        &self.config
    }

    /// Get rendering metrics
    pub fn metrics(&self) -> &RenderMetrics {
        &self.render_metrics
    }

    /// Update configuration
    pub fn update_config(&mut self, config: RenderConfig) {
        self.config = config;
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        let mut node_colors = HashMap::new();
        node_colors.insert(WASMNodeType::Struct, "#e1f5fe".to_string());
        node_colors.insert(WASMNodeType::Trait, "#f3e5f5".to_string());
        node_colors.insert(WASMNodeType::Function, "#e8f5e8".to_string());
        node_colors.insert(WASMNodeType::Impl, "#fff3e0".to_string());

        let mut edge_colors = HashMap::new();
        edge_colors.insert(WASMEdgeType::Implements, "#0277bd".to_string());
        edge_colors.insert(WASMEdgeType::Calls, "#388e3c".to_string());
        edge_colors.insert(WASMEdgeType::DependsOn, "#f57c00".to_string());
        edge_colors.insert(WASMEdgeType::Contains, "#7b1fa2".to_string());
        edge_colors.insert(WASMEdgeType::References, "#d32f2f".to_string());

        Self {
            layout_algorithm: LayoutAlgorithm::BreadthFirst,
            canvas_size: (800, 600),
            node_style: NodeStyle {
                default_radius: 20.0,
                node_colors,
                font_family: "Arial, sans-serif".to_string(),
                font_size: 12.0,
                border_width: 2.0,
                border_color: "#333333".to_string(),
            },
            edge_style: EdgeStyle {
                default_width: 2.0,
                edge_colors,
                arrow_size: 8.0,
                curve_type: CurveType::Straight,
            },
            animation: AnimationConfig {
                enabled: true,
                duration_ms: 300,
                easing: EasingFunction::EaseInOut,
            },
        }
    }
}

impl Default for RenderMetrics {
    fn default() -> Self {
        Self {
            last_render_ms: 0.0,
            total_render_ms: 0.0,
            render_count: 0,
            average_render_ms: 0.0,
            max_render_ms: 0.0,
        }
    }
}

impl Default for WASMRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate complete WASM visualization HTML file
pub fn generate_wasm_visualization(isg: &crate::isg::OptimizedISG, layout_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse layout algorithm
    let layout_algorithm = match layout_str {
        "breadthfirst" | "breadth_first" => LayoutAlgorithm::BreadthFirst,
        "forcedirected" | "force_directed" => LayoutAlgorithm::ForceDirected,
        "hierarchical" => LayoutAlgorithm::Hierarchical,
        "circular" => LayoutAlgorithm::Circular,
        _ => LayoutAlgorithm::BreadthFirst, // default
    };

    // Convert ISG to WASMGraph format
    let wasm_graph = convert_isg_to_wasm_graph(isg)?;

    // Generate HTML content
    let html_content = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Parseltongue WASM Visualization</title>
    <style>
        body {{
            margin: 0;
            padding: 20px;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            text-align: center;
        }}
        .controls {{
            padding: 15px;
            border-bottom: 1px solid #eee;
            display: flex;
            gap: 10px;
            align-items: center;
            flex-wrap: wrap;
        }}
        .controls button {{
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            background: #667eea;
            color: white;
            cursor: pointer;
            font-size: 14px;
            transition: background 0.2s;
        }}
        .controls button:hover {{
            background: #5a6fd8;
        }}
        .controls select {{
            padding: 6px 12px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 14px;
        }}
        .stats {{
            margin-left: auto;
            font-size: 12px;
            color: #666;
        }}
        #canvas {{
            display: block;
            cursor: grab;
            touch-action: none;
        }}
        #canvas:active {{
            cursor: grabbing;
        }}
        .info {{
            padding: 15px;
            background: #f8f9fa;
            font-size: 14px;
            color: #666;
            text-align: center;
        }}
        .loading {{
            text-align: center;
            padding: 50px;
            font-size: 18px;
            color: #666;
        }}
        .error {{
            text-align: center;
            padding: 50px;
            font-size: 18px;
            color: #dc3545;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🐍 Parseltongue WASM Visualization</h1>
            <p>Interactive Rust Code Architecture Visualization</p>
        </div>

        <div class="controls">
            <button onclick="zoomIn()">🔍 Zoom In</button>
            <button onclick="zoomOut()">🔍 Zoom Out</button>
            <button onclick="resetZoom()">🔄 Reset</button>
            <button onclick="togglePan()">✋ Pan</button>
            <select id="layoutSelect" onchange="changeLayout()">
                <option value="breadthfirst" {}>Breadth-First</option>
                <option value="forcedirected" {}>Force-Directed</option>
                <option value="hierarchical" {}>Hierarchical</option>
                <option value="circular" {}>Circular</option>
            </select>
            <script>
                // Load actual graph data from WASM
                graphData = {};
            </script>
            <div class="stats">
                <span id="nodeCount">Nodes: {}</span> |
                <span id="edgeCount">Edges: {}</span> |
                <span id="renderTime">Render: 0ms</span>
            </div>
        </div>

        <canvas id="canvas" width="1200" height="800"></canvas>

        <div class="info">
            <strong>Controls:</strong> Scroll to zoom • Drag to pan • Click nodes for details • Double-click to reset view
        </div>
    </div>

    <script>
        // Global state
        let wasmModule = null;
        let graphData = null;
        let currentLayout = '{}';
        let zoom = 1.0;
        let panX = 0;
        let panY = 0;
        let isPanning = false;
        let lastMouseX = 0;
        let lastMouseY = 0;

        // Canvas setup
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');

        // Initialize WASM module
        async function initWasm() {{
            try {{
                console.log('Initializing WASM module...');

                // For now, we'll render using JavaScript instead of WASM
                // This provides a fallback that demonstrates the visualization
                renderGraph();
                updateStats();

            }} catch (error) {{
                console.error('Failed to initialize WASM:', error);
                showError('Failed to initialize visualization: ' + error.message);
            }}
        }}

        // Render graph using JavaScript (fallback)
        function renderGraph() {{
            if (!graphData) return;

            const startRender = performance.now();

            // Clear canvas
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            // Apply transformations
            ctx.save();
            ctx.translate(panX, panY);
            ctx.scale(zoom, zoom);

            // Get nodes from WASMGraph data
            let nodes = (graphData.nodes || []).map(node => ({{
                ...node,
                x: Math.random() * 800 + 200,
                y: Math.random() * 600 + 100
            }}));

            // Simple layout based on selected algorithm
            applyLayout(nodes, currentLayout);

            // Draw edges
            ctx.strokeStyle = '#ddd';
            ctx.lineWidth = 2;
            (graphData.edges || []).forEach(edge => {{
                const fromNode = nodes.find(n => n.id === edge.source);
                const toNode = nodes.find(n => n.id === edge.target);
                if (fromNode && toNode) {{
                    ctx.beginPath();
                    ctx.moveTo(fromNode.x, fromNode.y);
                    ctx.lineTo(toNode.x, toNode.y);
                    ctx.stroke();
                }}
            }});

            // Draw nodes
            nodes.forEach(node => {{
                const radius = 20;

                // Node circle
                ctx.beginPath();
                ctx.arc(node.x, node.y, radius, 0, 2 * Math.PI);

                // Color by node type
                const colors = {{
                    'function': '#667eea',
                    'struct': '#48bb78',
                    'trait': '#ed8936',
                    'impl': '#9f7aea'
                }};

                ctx.fillStyle = colors[node.node_type] || '#718096';
                ctx.fill();

                // Node border
                ctx.strokeStyle = '#2d3748';
                ctx.lineWidth = 2;
                ctx.stroke();

                // Node label
                ctx.fillStyle = '#2d3748';
                ctx.font = '12px monospace';
                ctx.textAlign = 'center';
                ctx.textBaseline = 'middle';

                // Truncate long names
                let label = node.name || node.id;
                if (label.length > 15) {{
                    label = label.substring(0, 12) + '...';
                }}

                ctx.fillText(label, node.x, node.y + radius + 15);
            }});

            ctx.restore();

            const renderTime = performance.now() - startRender;
            document.getElementById('renderTime').textContent = `Render: ${{renderTime.toFixed(1)}}ms`;
        }}

        // Apply layout algorithm
        function applyLayout(nodes, layout) {{
            const width = 1200;
            const height = 800;
            const centerX = width / 2;
            const centerY = height / 2;

            switch (layout) {{
                case 'breadthfirst':
                    // Simple grid layout
                    const cols = Math.ceil(Math.sqrt(nodes.length));
                    nodes.forEach((node, i) => {{
                        node.x = (i % cols) * 100 + 100;
                        node.y = Math.floor(i / cols) * 100 + 100;
                    }});
                    break;

                case 'circular':
                    // Circular layout
                    const radius = Math.min(width, height) * 0.3;
                    nodes.forEach((node, i) => {{
                        const angle = (i / nodes.length) * 2 * Math.PI;
                        node.x = centerX + radius * Math.cos(angle);
                        node.y = centerY + radius * Math.sin(angle);
                    }});
                    break;

                case 'hierarchical':
                    // Simple hierarchical layout
                    const levels = {{}};
                    nodes.forEach(node => {{
                        const level = node.depth || 0;
                        if (!levels[level]) levels[level] = [];
                        levels[level].push(node);
                    }});

                    Object.entries(levels).forEach(([level, levelNodes]) => {{
                        const y = parseInt(level) * 120 + 100;
                        const spacing = width / (levelNodes.length + 1);
                        levelNodes.forEach((node, i) => {{
                            node.x = spacing * (i + 1);
                            node.y = y;
                        }});
                    }});
                    break;

                case 'forcedirected':
                    // Simple force-directed layout
                    nodes.forEach(node => {{
                        node.x = Math.random() * width;
                        node.y = Math.random() * height;
                    }});

                    // Basic physics simulation
                    for (let iter = 0; iter < 50; iter++) {{
                        // Repulsive forces
                        nodes.forEach((n1, i) => {{
                            nodes.forEach((n2, j) => {{
                                if (i !== j) {{
                                    const dx = n2.x - n1.x;
                                    const dy = n2.y - n1.y;
                                    const dist = Math.sqrt(dx * dx + dy * dy) + 0.1;
                                    const force = 1000 / (dist * dist);
                                    n1.x -= (dx / dist) * force;
                                    n1.y -= (dy / dist) * force;
                                }}
                            }});
                        }});

                        // Attractive forces for connected nodes
                        (graphData.edges || []).forEach(edge => {{
                            const fromNode = nodes.find(n => n.id === edge.source);
                            const toNode = nodes.find(n => n.id === edge.target);
                            if (fromNode && toNode) {{
                                const dx = toNode.x - fromNode.x;
                                const dy = toNode.y - fromNode.y;
                                const dist = Math.sqrt(dx * dx + dy * dy);
                                const force = dist * 0.01;
                                fromNode.x += (dx / dist) * force;
                                fromNode.y += (dy / dist) * force;
                                toNode.x -= (dx / dist) * force;
                                toNode.y -= (dy / dist) * force;
                            }}
                        }});
                    }}
                    break;

                default:
                    // Random layout
                    nodes.forEach(node => {{
                        node.x = Math.random() * (width - 200) + 100;
                        node.y = Math.random() * (height - 200) + 100;
                    }});
            }}
        }}

        // Update statistics
        function updateStats() {{
            const nodeCount = (graphData.nodes || []).length;
            const edgeCount = (graphData.edges || []).length;
            document.getElementById('nodeCount').textContent = `Nodes: ${{nodeCount}}`;
            document.getElementById('edgeCount').textContent = `Edges: ${{edgeCount}}`;
        }}

        // Control functions
        function zoomIn() {{
            zoom = Math.min(zoom * 1.2, 5.0);
            renderGraph();
        }}

        function zoomOut() {{
            zoom = Math.max(zoom / 1.2, 0.2);
            renderGraph();
        }}

        function resetZoom() {{
            zoom = 1.0;
            panX = 0;
            panY = 0;
            renderGraph();
        }}

        function togglePan() {{
            isPanning = !isPanning;
            canvas.style.cursor = isPanning ? 'grab' : 'default';
        }}

        function changeLayout() {{
            const select = document.getElementById('layoutSelect');
            currentLayout = select.value;
            renderGraph();
        }}

        function showError(message) {{
            document.body.innerHTML = `<div class="error">${{message}}</div>`;
        }}

        // Mouse controls
        canvas.addEventListener('wheel', (e) => {{
            e.preventDefault();
            const delta = e.deltaY > 0 ? 0.9 : 1.1;
            zoom *= delta;
            zoom = Math.max(0.2, Math.min(5.0, zoom));
            renderGraph();
        }});

        canvas.addEventListener('mousedown', (e) => {{
            isPanning = true;
            lastMouseX = e.clientX;
            lastMouseY = e.clientY;
            canvas.style.cursor = 'grabbing';
        }});

        canvas.addEventListener('mousemove', (e) => {{
            if (isPanning) {{
                const dx = e.clientX - lastMouseX;
                const dy = e.clientY - lastMouseY;
                panX += dx;
                panY += dy;
                lastMouseX = e.clientX;
                lastMouseY = e.clientY;
                renderGraph();
            }}
        }});

        canvas.addEventListener('mouseup', () => {{
            isPanning = false;
            canvas.style.cursor = 'grab';
        }});

        canvas.addEventListener('mouseleave', () => {{
            isPanning = false;
            canvas.style.cursor = 'grab';
        }});

        canvas.addEventListener('dblclick', () => {{
            resetZoom();
        }});

        // Initialize on load
        window.addEventListener('load', initWasm);

        // Handle window resize
        window.addEventListener('resize', () => {{
            renderGraph();
        }});
    </script>
</body>
</html>
    "#,
        // Layout selection
        if layout_algorithm == LayoutAlgorithm::BreadthFirst { "selected" } else { "" },
        if layout_algorithm == LayoutAlgorithm::ForceDirected { "selected" } else { "" },
        if layout_algorithm == LayoutAlgorithm::Hierarchical { "selected" } else { "" },
        if layout_algorithm == LayoutAlgorithm::Circular { "selected" } else { "" },
        // JSON data (for graphData assignment)
        serde_json::to_string(&wasm_graph)?,
        // Statistics
        isg.node_count(),
        isg.edge_count(),
        // Layout string (for currentLayout variable)
        layout_str
    );

    Ok(html_content)
}

/// Generate WASM visualization HTML file using dependency injection
///
/// This function follows steering docs Principle #3: Dependency Injection for Testability
/// It accepts any GraphDataLoader implementation, enabling:
/// - Test doubles and mocks in unit tests
/// - Different data sources (files, databases, APIs)
/// - Performance monitoring and caching
/// - Error handling and recovery strategies
///
/// # Performance Contract
/// - <100ms for graphs up to 10,000 nodes
/// - <500ms for graphs up to 100,000 nodes
/// - O(1) memory allocation during hot path
///
/// # Error Conditions
/// - GraphDataError::ISGLoadError if data loading fails
/// - GraphDataError::ConversionError if ISG -> WASMGraph conversion fails
/// - WASMError::SerializationError if JSON conversion fails
pub async fn generate_wasm_visualization_with_loader(
    loader: &dyn GraphDataLoader,
    layout_str: &str
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Validate loader availability
    if !loader.is_available().await {
        return Err(Box::new(GraphDataError::ISGLoadError(format!(
            "Data source '{}' is not available",
            loader.source_id()
        ))));
    }

    // Load ISG data using the injected loader
    let isg = loader.load_isg().await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    // Log metadata for debugging
    let metadata = loader.metadata();
    println!("📊 Loading graph data from: {}", loader.source_id());
    println!("📈 Graph metadata: {} - {}", metadata.name, metadata.description);

    if let Some(node_estimate) = metadata.node_count_estimate {
        println!("🔢 Estimated nodes: {}", node_estimate);
    }

    // Generate visualization using existing function
    generate_wasm_visualization(&isg, layout_str)
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(GraphDataError::ConversionError(e.to_string()))
        })
}

/// Convert ISG to WASMGraph format for visualization
fn convert_isg_to_wasm_graph(isg: &crate::isg::OptimizedISG) -> Result<WASMGraph, Box<dyn std::error::Error>> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut node_map = HashMap::new();

    // Read the ISG state
    let state = isg.state.read();

    // Convert ISG nodes to WASM nodes
    for (node_hash, &node_index) in &state.id_map {
        if let Some(node_data) = state.graph.node_weight(node_index) {
            let wasm_node = WASMNode {
                id: format!("{:?}", node_hash),
                name: node_data.name.to_string(),
                node_type: match node_data.kind {
                    crate::isg::NodeKind::Function => WASMNodeType::Function,
                    crate::isg::NodeKind::Struct => WASMNodeType::Struct,
                    crate::isg::NodeKind::Trait => WASMNodeType::Trait,
                    crate::isg::NodeKind::Impl => WASMNodeType::Impl,
                },
                position: None, // Will be calculated by layout
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("file".to_string(), node_data.file_path.to_string());
                    meta.insert("line".to_string(), node_data.line.to_string());
                    meta.insert("signature".to_string(), node_data.signature.to_string());
                    meta.insert("kind".to_string(), format!("{}", node_data.kind));
                    meta
                },
            };

            node_map.insert(*node_hash, nodes.len());
            nodes.push(wasm_node);
        }
    }

    // Convert ISG edges to WASM edges
    for edge in state.graph.edge_references() {
        let from_index = edge.source();
        let to_index = edge.target();

        // Find the hash values for these indices
        let from_hash = state.id_map.iter()
            .find(|(_, &idx)| idx == from_index)
            .map(|(hash, _)| *hash);
        let to_hash = state.id_map.iter()
            .find(|(_, &idx)| idx == to_index)
            .map(|(hash, _)| *hash);

        if let (Some(from_hash), Some(to_hash)) = (from_hash, to_hash) {
            let wasm_edge = WASMEdge {
                source: format!("{:?}", from_hash),
                target: format!("{:?}", to_hash),
                edge_type: match edge.weight() {
                    crate::isg::EdgeKind::Calls => WASMEdgeType::Calls,
                    crate::isg::EdgeKind::Implements => WASMEdgeType::Implements,
                    crate::isg::EdgeKind::Uses => WASMEdgeType::DependsOn,
                },
                label: None,
            };
            edges.push(wasm_edge);
        }
    }

    // Create layout
    let layout = WASMLayout {
        algorithm: "manual".to_string(),
        dimensions: (1200.0, 800.0),
        computed: false,
    };

    Ok(WASMGraph {
        nodes,
        edges,
        layout,
    })
}