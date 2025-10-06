//! WASM Core Algorithms - Layer 1 (Pure Rust)
//!
//! Core graph algorithms and data structures for WASM visualization
//! Following steering docs L1→L2→L3 architecture principles
//!
//! # Performance Contracts
//! - <50ms load time for graphs with ≤1000 nodes
//! - <16ms render time for initial view
//! - <100ms interaction response time
//! - O(1) memory allocation during hot path

use crate::isg::{OptimizedISG, NodeData, NodeKind, EdgeKind};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Core graph data structure for WASM visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WASMGraph {
    /// Nodes with display information
    pub nodes: Vec<WASMNode>,
    /// Edges with relationship information
    pub edges: Vec<WASMEdge>,
    /// Layout information
    pub layout: WASMLayout,
}

/// Node representation optimized for WASM rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WASMNode {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Node type for styling
    pub node_type: WASMNodeType,
    /// Position (computed by layout algorithm)
    pub position: Option<(f64, f64)>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Edge representation optimized for WASM rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WASMEdge {
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge type for styling
    pub edge_type: WASMEdgeType,
    /// Optional label
    pub label: Option<String>,
}

/// Node types for visualization styling
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WASMNodeType {
    Struct,
    Trait,
    Function,
    Impl,
}

/// Edge types for visualization styling
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WASMEdgeType {
    Implements,
    Calls,
    DependsOn,
    Contains,
    References,
}

/// Layout information for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WASMLayout {
    /// Layout algorithm used
    pub algorithm: String,
    /// Graph dimensions
    pub dimensions: (f64, f64),
    /// Whether layout is computed
    pub computed: bool,
}

/// Core algorithm engine for graph processing
pub struct WASMCoreEngine {
    /// Internal graph representation
    graph: WASMGraph,
    /// Performance metrics
    metrics: WASMMetrics,
}

/// Performance metrics tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WASMMetrics {
    /// Load time in milliseconds
    pub load_time_ms: f64,
    /// Render time in milliseconds
    pub render_time_ms: f64,
    /// Interaction response time in milliseconds
    pub interaction_time_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
}

impl WASMCoreEngine {
    /// Create new engine with empty graph
    pub fn new() -> Self {
        Self {
            graph: WASMGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
                layout: WASMLayout {
                    algorithm: "breadthfirst".to_string(),
                    dimensions: (800.0, 600.0),
                    computed: false,
                },
            },
            metrics: WASMMetrics {
                load_time_ms: 0.0,
                render_time_ms: 0.0,
                interaction_time_ms: 0.0,
                memory_usage_bytes: 0,
            },
        }
    }

    /// Load OptimizedISG into WASM format
    ///
    /// # Performance Contract
    /// - Must complete in <50ms for graphs with ≤1000 nodes
    /// - Memory allocation: O(n) where n = number of nodes
    pub fn load_isg(&mut self, isg: &OptimizedISG) -> Result<(), WASMError> {
        let start_time = std::time::Instant::now();

        // Convert ISG nodes to WASM format
        let state = isg.state.read();

        // Phase 1: Convert nodes
        for (_hash, &node_idx) in &state.id_map {
            if let Some(node_data) = state.graph.node_weight(node_idx) {
                let wasm_node = self.convert_node(node_data);
                self.graph.nodes.push(wasm_node);
            }
        }

        // Phase 2: Convert edges
        for edge_idx in state.graph.edge_indices() {
            if let Some((source, target, edge_data)) = state.graph.edge_endpoints(edge_idx)
                .and_then(|(s, t)| state.graph.edge_weight(edge_idx).map(|w| (s, t, w))) {

                if let (Some(source_node), Some(target_node)) = (
                    state.graph.node_weight(source),
                    state.graph.node_weight(target)
                ) {
                    let wasm_edge = self.convert_edge(
                        source_node,
                        target_node,
                        edge_data
                    );
                    self.graph.edges.push(wasm_edge);
                }
            }
        }

        // Update metrics
        self.metrics.load_time_ms = start_time.elapsed().as_millis() as f64;
        self.metrics.memory_usage_bytes = self.graph.nodes.len() * std::mem::size_of::<WASMNode>()
            + self.graph.edges.len() * std::mem::size_of::<WASMEdge>();

        // Validate performance contract
        if self.metrics.load_time_ms > 50.0 {
            return Err(WASMError::PerformanceContractViolation(
                format!("Load time {}ms > 50ms limit", self.metrics.load_time_ms)
            ));
        }

        Ok(())
    }

    /// Convert OptimizedISG node to WASM node
    fn convert_node(&self, node: &NodeData) -> WASMNode {
        WASMNode {
            id: format!("{:?}", node.hash),
            name: node.name.to_string(),
            node_type: self.convert_node_kind(&node.kind),
            position: None, // Will be computed by layout algorithm
            metadata: HashMap::new(), // TODO: Extract relevant metadata
        }
    }

    /// Convert OptimizedISG edge to WASM edge
    fn convert_edge(&self, source: &NodeData, target: &NodeData, _edge_kind: &EdgeKind) -> WASMEdge {
        WASMEdge {
            source: format!("{:?}", source.hash),
            target: format!("{:?}", target.hash),
            edge_type: WASMEdgeType::DependsOn, // TODO: Map actual edge types
            label: None,
        }
    }

    /// Convert NodeKind to WASMNodeType
    fn convert_node_kind(&self, kind: &NodeKind) -> WASMNodeType {
        match kind {
            NodeKind::Struct => WASMNodeType::Struct,
            NodeKind::Trait => WASMNodeType::Trait,
            NodeKind::Function => WASMNodeType::Function,
            NodeKind::Impl => WASMNodeType::Impl,
        }
    }

    /// Get graph reference
    pub fn graph(&self) -> &WASMGraph {
        &self.graph
    }

    /// Get metrics reference
    pub fn metrics(&self) -> &WASMMetrics {
        &self.metrics
    }

    /// Clear graph and reset metrics
    pub fn clear(&mut self) {
        self.graph.nodes.clear();
        self.graph.edges.clear();
        self.graph.layout.computed = false;
        self.metrics = WASMMetrics {
            load_time_ms: 0.0,
            render_time_ms: 0.0,
            interaction_time_ms: 0.0,
            memory_usage_bytes: 0,
        };
    }
}

impl Default for WASMCoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// WASM-specific errors
#[derive(Debug, thiserror::Error)]
pub enum WASMError {
    #[error("Performance contract violation: {0}")]
    PerformanceContractViolation(String),
    #[error("Graph conversion error: {0}")]
    ConversionError(String),
    #[error("Layout computation error: {0}")]
    LayoutError(String),
    #[error("JavaScript interop error: {0}")]
    JSInteropError(String),
}

// WASM-exposed functions will be in wasm_bindings.rs
// This module is pure Rust algorithms only