//! WASM Bindings - Layer 3 (JavaScript Interface)
//!
//! JavaScript bindings for WASM visualization
//! Following steering docs L1→L2→L3 architecture principles
//!
//! # Performance Contracts
//! - <50ms load time for graphs with ≤1000 nodes
//! - <16ms render time for initial view
//! - <100ms interaction response time
//! - Memory safe JavaScript interop

use crate::wasm_core::WASMCoreEngine;
use crate::wasm_renderer::{WASMRenderer, RenderConfig, RenderedScene, LayoutAlgorithm};
use crate::isg::OptimizedISG;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Main WASM visualization interface
#[wasm_bindgen]
pub struct WASMVisualization {
    core_engine: WASMCoreEngine,
    renderer: WASMRenderer,
    current_scene: Option<RenderedScene>,
}

#[wasm_bindgen]
impl WASMVisualization {
    /// Create new WASM visualization instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WASMVisualization, JsValue> {
        let visualization = WASMVisualization {
            core_engine: WASMCoreEngine::new(),
            renderer: WASMRenderer::new(),
            current_scene: None,
        };
        Ok(visualization)
    }

    /// Create visualization with custom configuration
    #[wasm_bindgen]
    pub fn with_config(config_str: &str) -> Result<WASMVisualization, JsValue> {
        let config: RenderConfig = serde_json::from_str(config_str)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;

        let visualization = WASMVisualization {
            core_engine: WASMCoreEngine::new(),
            renderer: WASMRenderer::with_config(config),
            current_scene: None,
        };
        Ok(visualization)
    }

    /// Load ISG data from JSON string
    ///
    /// # Performance Contract
    /// - Must complete in <50ms for graphs with ≤1000 nodes
    #[wasm_bindgen]
    pub fn load_isg_from_json(&mut self, json_str: &str) -> Result<(), JsValue> {
        // Parse JSON to OptimizedISG
        let isg: OptimizedISG = serde_json::from_str(json_str)
            .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

        // Load into core engine
        self.core_engine.load_isg(&isg)
            .map_err(|e| JsValue::from_str(&format!("Load error: {}", e)))?;

        Ok(())
    }

    /// Load ISG data from JavaScript object
    #[wasm_bindgen]
    pub fn load_isg_from_js(&mut self, isg_js: JsValue) -> Result<(), JsValue> {
        // Convert JsValue to string first, then deserialize
        let isg_str = isg_js.as_string()
            .ok_or_else(|| JsValue::from_str("ISG must be a string"))?;

        let isg: OptimizedISG = serde_json::from_str(&isg_str)
            .map_err(|e| JsValue::from_str(&format!("ISG conversion error: {}", e)))?;

        self.core_engine.load_isg(&isg)
            .map_err(|e| JsValue::from_str(&format!("Load error: {}", e)))?;

        Ok(())
    }

    /// Render current graph to scene
    ///
    /// # Performance Contract
    /// - <16ms for initial view
    /// - <100ms for interactions
    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<JsValue, JsValue> {
        let scene = self.renderer.render(self.core_engine.graph())
            .map_err(|e| JsValue::from_str(&format!("Render error: {}", e)))?;

        self.current_scene = Some(scene.clone());

        // Convert to JavaScript value
        Ok(JsValue::from_str(&serde_json::to_string(&scene)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?))
    }

    /// Get current scene if available
    #[wasm_bindgen]
    pub fn get_current_scene(&self) -> Result<JsValue, JsValue> {
        match &self.current_scene {
            Some(scene) => Ok(JsValue::from_str(&serde_json::to_string(scene)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)),
            None => Err(JsValue::from_str("No scene available - call render() first")),
        }
    }

    /// Update layout algorithm
    #[wasm_bindgen]
    pub fn set_layout_algorithm(&mut self, algorithm: &str) -> Result<(), JsValue> {
        let layout_alg = match algorithm {
            "breadthfirst" => LayoutAlgorithm::BreadthFirst,
            "forcedirected" => LayoutAlgorithm::ForceDirected,
            "hierarchical" => LayoutAlgorithm::Hierarchical,
            "circular" => LayoutAlgorithm::Circular,
            _ => return Err(JsValue::from_str(&format!("Unknown layout algorithm: {}", algorithm))),
        };

        let mut config = self.renderer.config().clone();
        config.layout_algorithm = layout_alg;
        self.renderer.update_config(config);

        Ok(())
    }

    /// Get available layout algorithms
    #[wasm_bindgen]
    pub fn get_available_layouts() -> JsValue {
        JsValue::from_str(r#"["breadthfirst", "forcedirected", "hierarchical", "circular"]"#)
    }

    /// Get current graph statistics
    #[wasm_bindgen]
    pub fn get_graph_stats(&self) -> JsValue {
        let graph = self.core_engine.graph();
        let stats = serde_json::json!({
            "node_count": graph.nodes.len(),
            "edge_count": graph.edges.len(),
            "layout_computed": graph.layout.computed,
            "layout_algorithm": graph.layout.algorithm
        });
        JsValue::from_str(&serde_json::to_string(&stats).unwrap_or_default())
    }

    /// Get performance metrics
    #[wasm_bindgen]
    pub fn get_metrics(&self) -> JsValue {
        let core_metrics = self.core_engine.metrics();
        let render_metrics = self.renderer.metrics();

        let metrics = serde_json::json!({
            "core": {
                "load_time_ms": core_metrics.load_time_ms,
                "render_time_ms": core_metrics.render_time_ms,
                "interaction_time_ms": core_metrics.interaction_time_ms,
                "memory_usage_bytes": core_metrics.memory_usage_bytes
            },
            "renderer": {
                "last_render_ms": render_metrics.last_render_ms,
                "total_render_ms": render_metrics.total_render_ms,
                "render_count": render_metrics.render_count,
                "average_render_ms": render_metrics.average_render_ms,
                "max_render_ms": render_metrics.max_render_ms
            }
        });

        JsValue::from_str(&serde_json::to_string(&metrics).unwrap_or_default())
    }

    /// Clear current graph and reset metrics
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.core_engine.clear();
        self.current_scene = None;
    }

    /// Test performance contracts
    #[wasm_bindgen]
    pub fn test_performance_contracts(&self) -> JsValue {
        let core_metrics = self.core_engine.metrics();
        let render_metrics = self.renderer.metrics();

        let load_ok = core_metrics.load_time_ms <= 50.0;
        let render_ok = if render_metrics.render_count == 1 {
            render_metrics.last_render_ms <= 16.0
        } else {
            render_metrics.last_render_ms <= 100.0
        };

        let results = serde_json::json!({
            "load_contract_satisfied": load_ok,
            "load_time_ms": core_metrics.load_time_ms,
            "load_limit_ms": 50.0,
            "render_contract_satisfied": render_ok,
            "render_time_ms": render_metrics.last_render_ms,
            "render_limit_ms": if render_metrics.render_count == 1 { 16.0 } else { 100.0 },
            "memory_usage_mb": core_metrics.memory_usage_bytes as f64 / 1_000_000.0
        });

        JsValue::from_str(&serde_json::to_string(&results).unwrap_or_default())
    }

    /// Export scene to SVG string
    #[wasm_bindgen]
    pub fn export_to_svg(&self) -> Result<String, JsValue> {
        match &self.current_scene {
            Some(scene) => self.generate_svg(scene),
            None => Err(JsValue::from_str("No scene available - call render() first")),
        }
    }

    /// Export scene to PNG (base64)
    #[wasm_bindgen]
    pub fn export_to_png(&self) -> Result<String, JsValue> {
        match &self.current_scene {
            Some(_) => {
                // TODO: Implement PNG export
                Err(JsValue::from_str("PNG export not yet implemented"))
            }
            None => Err(JsValue::from_str("No scene available - call render() first")),
        }
    }

    /// Handle mouse interaction (pan/zoom)
    #[wasm_bindgen]
    pub fn handle_mouse_interaction(&mut self, _x: f64, _y: f64, _zoom: f64) -> Result<(), JsValue> {
        // TODO: Implement mouse interaction handling
        // This would update the scene based on user input
        Ok(())
    }

    /// Handle node selection
    #[wasm_bindgen]
    pub fn select_node(&mut self, node_id: &str) -> Result<JsValue, JsValue> {
        // TODO: Implement node selection
        // This would highlight the selected node and show its details
        let details = serde_json::json!({
            "node_id": node_id,
            "selected": true,
            "details": "Node details not yet implemented"
        });
        Ok(JsValue::from_str(&serde_json::to_string(&details)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?))
    }
}

impl WASMVisualization {
    /// Generate SVG from rendered scene
    fn generate_svg(&self, scene: &RenderedScene) -> Result<String, JsValue> {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            scene.metadata.width, scene.metadata.height
        );

        // Add styles
        svg.push_str(r#"<style>"#);
        svg.push_str("text { font-family: Arial, sans-serif; font-size: 12px; }");
        svg.push_str("</style>");

        // Render edges
        for edge in &scene.edges {
            svg.push_str(&format!(
                r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" />"#,
                edge.path_data, edge.color, edge.width
            ));
        }

        // Render nodes
        for node in &scene.nodes {
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
                node.x, node.y, node.radius, node.color, node.border_color, node.border_width
            ));

            if node.label_visible {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" text-anchor="middle" fill="{}">{}</text>"#,
                    node.x, node.y + 4.0, node.label_color, html_escape::encode_text(&node.node.name)
                ));
            }
        }

        svg.push_str("</svg>");
        Ok(svg)
    }
}

/// Utility functions for JavaScript interop
#[wasm_bindgen]
pub fn wasm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn wasm_build_info() -> JsValue {
    let info = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "build_timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });
    JsValue::from_str(&serde_json::to_string(&info).unwrap_or_default())
}

/// Error handling utilities
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

#[wasm_bindgen]
pub fn console_log(msg: &str) {
    log(msg);
}

#[wasm_bindgen]
pub fn console_error(msg: &str) {
    error(msg);
}

#[wasm_bindgen]
pub fn console_warn(msg: &str) {
    warn(msg);
}

// Performance monitoring utilities
#[wasm_bindgen]
pub struct PerformanceTimer {
    start_time: f64,
}

#[wasm_bindgen]
impl PerformanceTimer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerformanceTimer {
        PerformanceTimer {
            start_time: js_sys::Date::now(),
        }
    }

    #[wasm_bindgen]
    pub fn elapsed_ms(&self) -> f64 {
        js_sys::Date::now() - self.start_time
    }

    #[wasm_bindgen]
    pub fn log_elapsed(&self, label: &str) {
        let elapsed = self.elapsed_ms();
        log(&format!("{}: {}ms", label, elapsed));
    }
}

// Memory usage monitoring
#[wasm_bindgen]
pub fn get_memory_usage() -> JsValue {
    let _memory = wasm_bindgen::memory();
    let usage = serde_json::json!({
        "wasm_memory_available": true,
        "note": "Memory usage tracking simplified for compatibility"
    });

    JsValue::from_str(&serde_json::to_string(&usage).unwrap_or_default())
}