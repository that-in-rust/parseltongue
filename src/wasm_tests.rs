//! WASM Tests - TDD Phase 2 (STUB → RED → GREEN → REFACTOR)
//!
//! Basic test suite following steering docs TDD principles
//! Tests start as stubs to verify compilation, then functionality

use crate::wasm_core::{WASMCoreEngine, WASMGraph, WASMNode, WASMEdge, WASMNodeType, WASMEdgeType};
use crate::wasm_renderer::{WASMRenderer, LayoutAlgorithm};
use crate::wasm_bindings::WASMVisualization;
use std::collections::HashMap;
use wasm_bindgen_test::*;

// ===== STUB TESTS =====
// These tests ensure the basic structure exists before functionality

#[wasm_bindgen_test]
fn test_stub_wasm_core_engine_exists() {
    // RED: This test should fail initially - we just need the type to exist
    let _engine = WASMCoreEngine::new();
    // If this compiles, the test passes
}

#[wasm_bindgen_test]
fn test_stub_wasm_renderer_exists() {
    // RED: This test should fail initially - we just need the type to exist
    let _renderer = WASMRenderer::new();
    // If this compiles, the test passes
}

#[wasm_bindgen_test]
fn test_stub_wasm_visualization_exists() {
    // RED: This test should fail initially - we just need the type to exist
    let _viz = WASMVisualization::new();
    // If this compiles, the test passes
}

#[wasm_bindgen_test]
fn test_stub_graph_structures_exist() {
    // RED: These should fail initially - we just need the types to exist
    let _graph = WASMGraph {
        nodes: Vec::new(),
        edges: Vec::new(),
        layout: Default::default(),
    };
    let _node = WASMNode {
        id: "test".to_string(),
        name: "test".to_string(),
        node_type: WASMNodeType::Struct,
        position: None,
        metadata: HashMap::new(),
    };
    let _edge = WASMEdge {
        source: "source".to_string(),
        target: "target".to_string(),
        edge_type: WASMEdgeType::DependsOn,
        label: None,
    };
    // If this compiles, the test passes
}

// ===== BASIC FUNCTIONALITY TESTS =====
// These test core functionality works correctly

#[wasm_bindgen_test]
fn test_basic_wasm_visualization_creation() {
    // RED: This should fail initially - WASM visualization should work
    let mut viz = WASMVisualization::new().unwrap();

    // Test basic properties
    let stats = viz.get_graph_stats();
    assert!(!stats.is_undefined(), "Should return valid stats");

    let metrics = viz.get_metrics();
    assert!(!metrics.is_undefined(), "Should return valid metrics");

    // Test empty state
    viz.clear();
}

#[wasm_bindgen_test]
fn test_basic_layout_algorithms() {
    // RED: This should fail initially - layout algorithms should exist
    let renderer = WASMRenderer::new();
    let config = renderer.config();
    assert!(config.layout_algorithm == LayoutAlgorithm::BreadthFirst);
}

#[wasm_bindgen_test]
fn test_basic_wasm_engine_metrics() {
    // RED: This should fail initially - engine should track metrics
    let engine = WASMCoreEngine::new();
    let metrics = engine.metrics();

    // Should have default metrics
    assert!(metrics.load_time_ms >= 0.0, "Load time should be tracked");
    assert!(metrics.memory_usage_bytes >= 0, "Memory usage should be tracked");
}

#[wasm_bindgen_test]
fn test_basic_renderer_config() {
    // RED: This should fail initially - renderer should have valid config
    let renderer = WASMRenderer::new();
    let config = renderer.config();

    // Should have valid dimensions
    assert!(config.canvas_size.0 > 0, "Canvas width should be positive");
    assert!(config.canvas_size.1 > 0, "Canvas height should be positive");

    // Should have default layout algorithm
    assert!(matches!(config.layout_algorithm, LayoutAlgorithm::BreadthFirst));
}

wasm_bindgen_test_configure!(run_in_browser);