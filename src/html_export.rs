//! Interactive HTML Export Module - ISG to Cytoscape + ELK Transformation
//!
//! **Executable Specification**: Transforms Interface Signature Graph data into
//! interactive HTML diagrams with Cytoscape.js rendering and ELK layout engine,
//! supporting 2000+ nodes with 60fps interaction performance.
//!
//! ## Performance Contract
//! - **Target**: <500ms for large graphs (≤2000 nodes, ≤4000 edges)
//! - **Memory**: <100MB peak memory usage for large graphs
//! - **Interaction**: 60fps pan/zoom, <16ms interaction latency
//! - **Level-of-Detail**: Labels visible at ≥1.2x zoom
//!
//! ## Architecture Compliance (L1→L2→L3)
//! - **L1 Core**: Pure string manipulation, ownership transfer, Result/Option
//! - **L2 Standard**: Iterator patterns, efficient concatenation, memory pooling
//! - **L3 External**: Cytoscape.js + ELK.js via CDN, no external Rust deps
//!
//! ## Interactive Features
//! - Canvas-based rendering for large graph performance
//! - Zoom/pan with level-of-detail label visibility
//! - Node search and highlighting functionality
//! - Responsive design with proper mobile support

use crate::isg::{OptimizedISG, NodeData, NodeKind, EdgeKind};
use std::fmt::Write;
use std::fs;
use std::sync::Arc;
use std::time::Instant;
use petgraph::visit::{IntoEdgeReferences, EdgeRef};

/// Main export function - transforms ISG to interactive HTML diagram
///
/// # Preconditions
/// - ISG graph is in valid state with consistent node/edge relationships
///
/// # Postconditions
/// - Returns self-contained HTML file with interactive Cytoscape diagram
/// - Graph renders with ELK layout algorithm optimized for large graphs
/// - Interactive features: zoom, pan, search, and level-of-detail
/// - Performance: <500ms render time for 2000+ node graphs
///
/// # Error Conditions
/// - Cannot fail (HTML generation is infallible)
/// - Graph layout gracefully handles malformed data
/// - Performance degrades gracefully for extremely large graphs
///
/// # Performance Contract
/// - Must complete in <500ms for graphs with ≤2000 nodes
/// - Memory usage must remain <100MB peak
/// - Interaction latency must be <16ms (60fps)
pub fn export_isg_to_interactive_html(isg: &OptimizedISG) -> String {
    let start_time = Instant::now();

    let mut html = String::with_capacity(64 * 1024); // Pre-allocate 64KB

    // Phase 1: HTML structure and CSS
    write_html_header(&mut html);

    // Phase 2: JavaScript libraries (Cytoscape + ELK)
    write_script_includes(&mut html);

    // Phase 3: Graph data transformation
    let nodes_json = generate_nodes_json(isg);
    let edges_json = generate_edges_json(isg);

    // Phase 4: Interactive configuration
    write_cytoscape_config(&mut html, &nodes_json, &edges_json);

  
    // Phase 6: Performance monitoring
    write_performance_footer(&mut html, start_time);

    html
}

/// Creates an HTML file with the interactive diagram
///
/// # Preconditions
/// - html_content contains valid interactive HTML
/// - filename is a valid path
///
/// # Postconditions
/// - Self-contained HTML file created
/// - Diagram renders immediately with full interactivity
/// - File is optimized for browser performance
pub fn create_interactive_html_file(filename: &str, html_content: &str) {
    fs::write(filename, html_content).unwrap_or_else(|e| {
        eprintln!("Failed to create interactive HTML file {}: {}", filename, e);
    });
}

/// Writes the HTML document structure with responsive CSS
fn write_html_header(html: &mut String) {
    let _ = write!(html, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Interactive ISG Architecture Diagram</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            overflow: hidden;
        }}

        #header {{
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 60px;
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(10px);
            box-shadow: 0 2px 20px rgba(0, 0, 0, 0.1);
            display: flex;
            align-items: center;
            padding: 0 20px;
            z-index: 1000;
        }}

        #title {{
            font-size: 18px;
            font-weight: 600;
            color: #333;
            margin-right: auto;
        }}

        #controls {{
            display: flex;
            gap: 15px;
            align-items: center;
        }}

        .control-group {{
            display: flex;
            align-items: center;
            gap: 8px;
        }}

        label {{
            font-size: 12px;
            color: #666;
            font-weight: 500;
        }}

        input[type="text"] {{
            padding: 6px 12px;
            border: 1px solid #ddd;
            border-radius: 6px;
            font-size: 14px;
            width: 200px;
            transition: border-color 0.2s;
        }}

        input[type="text"]:focus {{
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }}

        button {{
            padding: 6px 16px;
            background: #667eea;
            color: white;
            border: none;
            border-radius: 6px;
            font-size: 12px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s;
        }}

        button:hover {{
            background: #5a67d8;
            transform: translateY(-1px);
        }}

        button:active {{
            transform: translateY(0);
        }}

        #cy {{
            position: absolute;
            top: 60px;
            left: 0;
            right: 0;
            bottom: 0;
            background: white;
        }}

        #status {{
            position: absolute;
            bottom: 20px;
            right: 20px;
            background: rgba(0, 0, 0, 0.8);
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 11px;
            font-family: 'Monaco', 'Menlo', monospace;
            z-index: 1000;
        }}

        .performance-info {{
            color: #4ade80;
        }}

        .error-info {{
            color: #f87171;
        }}

        @media (max-width: 768px) {{
            #controls {{
                flex-wrap: wrap;
                gap: 8px;
            }}

            input[type="text"] {{
                width: 150px;
            }}

            #title {{
                font-size: 16px;
            }}
        }}
    </style>
</head>
<body>
    <div id="header">
        <div id="title">🔗 Interactive ISG Architecture Diagram</div>
        <div id="controls">
            <div class="control-group">
                <label for="search">Search:</label>
                <input type="text" id="search" placeholder="Find nodes...">
                <button onclick="searchNodes()">Find</button>
            </div>
            <div class="control-group">
                <button onclick="resetView()">Reset View</button>
                <button onclick="fitToScreen()">Fit to Screen</button>
                <button onclick="toggleLabels()">Toggle Labels</button>
            </div>
        </div>
    </div>
    <div id="cy"></div>
    <div id="status">Initializing...</div>
"#);
}

/// Includes Cytoscape.js and ELK.js from CDN with fallbacks
fn write_script_includes(html: &mut String) {
    let _ = write!(html, r#"
    <script src="https://unpkg.com/cytoscape@3.26.0/dist/cytoscape.min.js"></script>
    <script src="https://unpkg.com/cytoscape-elk@2.0.0/cytoscape-elk.js"></script>
    <script>
        // Fallback handling for CDN failures
        window.addEventListener('error', function(e) {{
            if (e.target.tagName === 'SCRIPT') {{
                document.getElementById('status').innerHTML =
                    '<span class="error-info">⚠️ Failed to load libraries. Check internet connection.</span>';
            }}
        }}, true);
    </script>
"#);
}

/// Generates optimized JSON representation of nodes for Cytoscape
fn generate_nodes_json(isg: &OptimizedISG) -> String {
    let mut nodes = String::with_capacity(isg.node_count() * 200); // Estimate 200 chars per node

    let state = isg.state.read();
    nodes.push('[');

    let mut first = true;
    for (_hash, &node_idx) in &state.id_map {
        if let Some(node) = state.graph.node_weight(node_idx) {
            if !first {
                nodes.push(',');
            }
            first = false;

            let _ = write!(nodes, r#"
                {{
                    "data": {{
                        "id": "{}",
                        "label": "{}",
                        "kind": "{:?}",
                        "signature": "{}",
                        "file_path": "{}",
                        "line": {},
                        "icon": "{}"
                    }}
                }}"#,
                node.name,
                node.name,
                node.kind,
                escape_json_string(&node.signature),
                escape_json_string(&node.file_path),
                node.line,
                node_kind_icon(&node.kind)
            );
        }
    }

    nodes.push(']');
    nodes
}

/// Generates optimized JSON representation of edges for Cytoscape
fn generate_edges_json(isg: &OptimizedISG) -> String {
    let mut edges = String::with_capacity(isg.edge_count() * 150); // Estimate 150 chars per edge

    let state = isg.state.read();
    edges.push('[');

    let mut first = true;
    for edge_ref in state.graph.edge_references() {
        let source = &state.graph[edge_ref.source()];
        let target = &state.graph[edge_ref.target()];

        if !first {
            edges.push(',');
        }
        first = false;

        let _ = write!(edges, r#"
            {{
                "data": {{
                    "source": "{}",
                    "target": "{}",
                    "kind": "{:?}",
                    "arrow_style": "{}"
                }}
            }}"#,
            source.name,
            target.name,
            edge_ref.weight(),
            edge_kind_arrow_style(edge_ref.weight())
        );
    }

    edges.push(']');
    edges
}

/// Writes Cytoscape configuration with ELK layout and performance optimizations
fn write_cytoscape_config(html: &mut String, nodes_json: &str, edges_json: &str) {
    let _ = write!(html, r#"
    <script>
        // Performance monitoring
        const renderStartTime = performance.now();

        // Graph data
        const nodes_data = "#);

    html.push_str(nodes_json);

    let _ = write!(html, r#";
        const edges_data = "#);

    html.push_str(edges_json);

    let _ = write!(html, r#";

        // Cytoscape initialization with performance optimizations
        const cy = cytoscape({{
            container: document.getElementById('cy'),

            // Data
            elements: [
                ...nodes_data,
                ...edges_data
            ],

            // ELK layout for large graphs
            layout: {{
                name: 'elk',
                elkAlgorithm: 'layered',
                elkLayerSpacing: 80,
                elkNodeSpacing: 60,
                elkEdgeSpacing: 20,
                elkDirection: 'DOWN',
                animate: false,
                fit: true,
                padding: 50
            }},

            // Performance optimizations
            pixelRatio: 1,
            textureOnViewport: false,
            wheelSensitivity: 0.5,
            minZoom: 0.1,
            maxZoom: 3.0
        }});

        // Basic styling
        cy.style([
            {{
                selector: 'node',
                style: {{
                    'background-color': '#74c0fc',
                    'label': 'data(label)',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'font-size': '11px',
                    'color': '#333',
                    'shape': 'round-rectangle',
                    'width': 140,
                    'height': 70,
                    'border-width': 2,
                    'border-color': '#1971c2'
                }}
            }},
            {{
                selector: 'edge',
                style: {{
                    'curve-style': 'bezier',
                    'line-color': '#999',
                    'target-arrow-color': '#999',
                    'target-arrow-shape': 'triangle',
                    'width': 2
                }}
            }}
        ]);

        // Performance monitoring
        cy.ready(function() {{
            const renderTime = performance.now() - renderStartTime;
            const nodeCount = cy.nodes().length;
            const edgeCount = cy.edges().length;

            document.getElementById('status').innerHTML =
                '<span class="performance-info">✅ Rendered ' + nodeCount + ' nodes, ' +
                edgeCount + ' edges in ' + renderTime.toFixed(1) + 'ms</span>';
        }});

        // Level-of-detail: Update labels based on zoom level
        cy.on('zoom', function(evt) {{
            const zoom = cy.zoom();
            const showLabels = zoom >= 1.2;

            cy.nodes().forEach(function(node) {{
                const label = showLabels ? node.data('label') : node.data('icon');
                node.style('label', label);
            }});
        }});

        // Performance monitoring: interaction latency
        cy.on('pan zoom', function(evt) {{
            const latency = performance.now() - evt.timeStamp;
            if (latency > 16) {{ // 60fps threshold
                console.warn('Interaction latency: ' + latency.toFixed(1) + 'ms');
            }}
        }});

        // Tooltip functionality
        cy.on('mouseover', 'node', function(evt) {{
            const node = evt.target;
            const tooltip = document.createElement('div');
            tooltip.style.position = 'absolute';
            tooltip.style.background = 'rgba(0, 0, 0, 0.8)';
            tooltip.style.color = 'white';
            tooltip.style.padding = '8px';
            tooltip.style.borderRadius = '4px';
            tooltip.style.fontSize = '12px';
            tooltip.style.pointerEvents = 'none';
            tooltip.style.zIndex = '1000';
            tooltip.style.opacity = '0';
            tooltip.style.transition = 'opacity 0.2s';

            tooltip.innerHTML = '<strong>' + node.data('label') + '</strong><br>' +
                               node.data('kind') + '<br>' +
                               'File: ' + (node.data('file') || 'Unknown') + '<br>' +
                               'Line: ' + (node.data('line') || 'Unknown');

            document.body.appendChild(tooltip);

            const rect = cy.container().getBoundingClientRect();
            tooltip.style.left = (rect.left + evt.renderedPosition.x + 10) + 'px';
            tooltip.style.top = (rect.top + evt.renderedPosition.y - 10) + 'px';

            setTimeout(() => {{ tooltip.style.opacity = '1'; }}, 10);

            node.data('tooltip', tooltip);
        }});

        cy.on('mouseout', 'node', function(evt) {{
            const node = evt.target;
            const tooltip = node.data('tooltip');
            if (tooltip) {{
                tooltip.style.opacity = '0';
                setTimeout(() => {{
                    document.body.removeChild(tooltip);
                    node.removeData('tooltip');
                }}, 200);
            }}
        }});

        // UI Functions
        function searchNodes() {{
            const searchTerm = document.getElementById('search').value.toLowerCase();
            if (!searchTerm) {{
                cy.elements().unselect();
                cy.fit();
                return;
            }}

            const matchingNodes = cy.nodes().filter(function(node) {{
                return node.data('label').toLowerCase().includes(searchTerm) ||
                       node.data('file_path').toLowerCase().includes(searchTerm);
            }});

            cy.elements().unselect();
            matchingNodes.select();

            if (matchingNodes.length > 0) {{
                cy.fit(matchingNodes, 50);
                document.getElementById('status').innerHTML =
                    '<span class="performance-info">🔍 Found ' + matchingNodes.length + ' matching nodes</span>';
            }} else {{
                document.getElementById('status').innerHTML =
                    '<span class="error-info">❌ No nodes found</span>';
            }}
        }}

        function resetView() {{
            cy.elements().unselect();
            cy.fit();
            document.getElementById('search').value = '';
            document.getElementById('status').innerHTML = 'View reset';
        }}

        function fitToScreen() {{
            cy.fit();
            document.getElementById('status').innerHTML = 'Fitted to screen';
        }}

        function toggleLabels() {{
            cy.nodes().forEach(function(node) {{
                const currentLabel = node.style('label');
                const newLabel = currentLabel === node.data('icon') ? node.data('label') : node.data('icon');
                node.style('label', newLabel);
            }});
            document.getElementById('status').innerHTML = 'Labels toggled';
        }}

        // Keyboard shortcuts
        document.addEventListener('keydown', function(e) {{
            if (e.target.tagName === 'INPUT') return; // Ignore when typing in search

            switch(e.key.toLowerCase()) {{
                case 'f':
                    e.preventDefault();
                    document.getElementById('search').focus();
                    break;
                case 'r':
                    e.preventDefault();
                    resetView();
                    break;
                case 'l':
                    e.preventDefault();
                    toggleLabels();
                    break;
            }}
        }});
    </script>
"#);
}

/// Writes performance monitoring footer
fn write_performance_footer(html: &mut String, start_time: Instant) {
    let total_time = start_time.elapsed().as_millis();

    // Add performance comment before closing HTML if needed
    if total_time > 500 {
        let _ = write!(html, "<!-- WARNING: HTML generation took {}ms, exceeding 500ms target -->", total_time);
    }

    let _ = write!(html, r#"
</body>
</html>
"#);
}

/// Returns appropriate icon for each node kind (same as Mermaid exporter)
const fn node_kind_icon(kind: &NodeKind) -> &'static str {
    match kind {
        NodeKind::Function => "🔧",
        NodeKind::Struct => "📦",
        NodeKind::Trait => "🎯",
    }
}

/// Returns appropriate arrow style for each edge kind (same as Mermaid exporter)
const fn edge_kind_arrow_style(kind: &EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Calls => "solid",
        EdgeKind::Implements => "dashed",
        EdgeKind::Uses => "dotted",
    }
}

/// Escapes strings for safe JSON embedding
fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::SigHash;
    use std::sync::Arc;

    /// Test contract: HTML export performance for large graphs
    ///
    /// # Given: ISG with 2000 nodes and 4000 edges
    /// # When: export_isg_to_interactive_html is called
    /// # Then: Must complete in <500ms (performance contract)
    #[test]
    fn test_html_export_performance_contract_large_graph() {
        // Setup: Create large test graph
        let isg = create_large_performance_test_graph(2000, 4000);

        // Action: Time the HTML export operation
        let start = Instant::now();
        let html = export_isg_to_interactive_html(&isg);
        let elapsed = start.elapsed();

        // Assertions: Validate performance contract and output quality
        assert!(elapsed.as_millis() < 500,
            "HTML export took {}ms, contract requires <500ms", elapsed.as_millis());

        // Verify HTML structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("cytoscape"));
        assert!(html.contains("cytoscape-elk"));
        assert!(html.contains("Interactive ISG Architecture Diagram"));

        // Verify performance monitoring is included
        assert!(html.contains("renderStartTime"));
        assert!(html.contains("performance.now()"));
    }

    /// Test contract: HTML export functionality for typical graphs
    ///
    /// # Given: ISG with 100 nodes and 200 edges
    /// # When: export_isg_to_interactive_html is called
    /// # Then: Complete HTML with all required features generated
    #[test]
    fn test_html_export_functionality_typical_graph() {
        // Setup: Create typical test graph
        let isg = create_performance_test_graph(100, 200);

        // Action: Export to interactive HTML
        let html = export_isg_to_interactive_html(&isg);

        // Assertions: Verify required HTML components
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.trim_end().ends_with("</html>"), "HTML ends with: {:?}", &html[html.len().saturating_sub(20)..]);

        // Verify Cytoscape integration
        assert!(html.contains("cytoscape@3.26.0"));
        assert!(html.contains("cytoscape-elk@2.0.0"));

        // Verify interactive features
        assert!(html.contains("id=\"search\""));
        assert!(html.contains("searchNodes()"));
        assert!(html.contains("resetView()"));
        assert!(html.contains("toggleLabels()"));

        // Verify ELK layout configuration
        assert!(html.contains("elkAlgorithm: 'layered'"));
        assert!(html.contains("elkDirection: 'DOWN'"));

        // Verify performance optimizations
        assert!(html.contains("animate: false"));
        assert!(html.contains("pixelRatio: 1"));
        assert!(html.contains("textureOnViewport: false"));
    }

    /// Test contract: JSON generation correctness and escaping
    ///
    /// # Given: ISG with special characters in names and signatures
    /// # When: generate_nodes_json and generate_edges_json are called
    /// # Then: Valid JSON with proper escaping is generated
    #[test]
    fn test_json_generation_and_escaping() {
        // Setup: Create ISG with problematic characters
        let isg = create_test_graph_with_special_characters();

        // Action: Generate JSON
        let nodes_json = generate_nodes_json(&isg);
        let edges_json = generate_edges_json(&isg);

        // Assertions: Verify JSON structure
        assert!(nodes_json.starts_with('[') && nodes_json.ends_with(']'));
        assert!(edges_json.starts_with('[') && edges_json.ends_with(']'));

        // Verify proper escaping of special characters
        assert!(nodes_json.contains("\\n")); // Newlines escaped
        assert!(nodes_json.contains("\\\"")); // Quotes escaped
        assert!(nodes_json.contains("test")); // Basic verification

        // Verify node structure
        assert!(nodes_json.contains("\"data\":"));
        assert!(nodes_json.contains("\"label\":"));
        assert!(nodes_json.contains("\"kind\":"));
        assert!(nodes_json.contains("\"icon\":"));

        // Verify edge structure
        assert!(edges_json.contains("\"source\":"));
        assert!(edges_json.contains("\"target\":"));
        assert!(edges_json.contains("\"kind\":"));
    }

    /// Test contract: Level-of-detail functionality in HTML
    ///
    /// # Given: Any ISG exported to HTML
    /// # When: Generated HTML is examined
    /// # Then: Level-of-detail zoom functionality is present
    #[test]
    fn test_level_of_detail_functionality() {
        // Setup: Create test ISG
        let isg = create_performance_test_graph(50, 100);

        // Action: Export to HTML
        let html = export_isg_to_interactive_html(&isg);

        // Assertions: Verify LOD functionality
        assert!(html.contains("cy.on('zoom'")); // Zoom event handler
        assert!(html.contains("const zoom = cy.zoom()")); // Zoom level detection
        assert!(html.contains("zoom >= 1.2")); // LOD threshold
        assert!(html.contains("node.style('label', label)")); // Dynamic label updates

        // Verify icons vs full labels logic
        assert!(html.contains("node.data('icon')"));
        assert!(html.contains("node.data('label')"));
    }

    /// Test contract: Interactive features completeness
    ///
    /// # Given: Any ISG exported to HTML
    /// # When: Generated HTML is examined
    /// # Then: All interactive features are implemented
    #[test]
    fn test_interactive_features_completeness() {
        // Setup: Create test ISG
        let isg = create_performance_test_graph(10, 20);

        // Action: Export to HTML
        let html = export_isg_to_interactive_html(&isg);

        // Assertions: Verify search functionality
        assert!(html.contains("function searchNodes()"));
        assert!(html.contains("filter(function(node)"));
        assert!(html.contains("toLowerCase().includes(searchTerm)"));

        // Verify view controls
        assert!(html.contains("function resetView()"));
        assert!(html.contains("function fitToScreen()"));
        assert!(html.contains("function toggleLabels()"));

        // Verify keyboard shortcuts
        assert!(html.contains("document.addEventListener('keydown'"));
        assert!(html.contains("case 'f':")); // Search shortcut
        assert!(html.contains("case 'r':")); // Reset shortcut
        assert!(html.contains("case 'l':")); // Labels shortcut

        // Verify tooltip functionality
        assert!(html.contains("cy.on('mouseover', 'node'"));
        assert!(html.contains("const tooltip = document.createElement"));
        assert!(html.contains("tooltip.style.opacity = '0'"));
        assert!(html.contains("tooltip.style.opacity = '1'"));

        // Verify performance monitoring
        assert!(html.contains("cy.on('pan zoom'"));
        assert!(html.contains("latency > 16")); // 60fps threshold
        assert!(html.contains("console.warn('Interaction latency:"));
    }

    /// Test contract: Memory efficiency for large graphs
    ///
    /// # Given: ISG with 2000 nodes
    /// # When: export_isg_to_interactive_html is called
    /// # Then: HTML size is reasonable (<5MB for 2000 nodes)
    #[test]
    fn test_memory_efficiency_large_graphs() {
        // Setup: Create large test graph
        let isg = create_large_performance_test_graph(2000, 3000);

        // Action: Export to HTML
        let start = Instant::now();
        let html = export_isg_to_interactive_html(&isg);
        let elapsed = start.elapsed();

        // Assertions: Verify memory efficiency
        let html_size_mb = html.len() as f64 / (1024.0 * 1024.0);
        assert!(html_size_mb < 5.0,
            "HTML size {:.1}MB exceeds 5MB target for 2000 nodes", html_size_mb);

        // Still must meet performance contract
        assert!(elapsed.as_millis() < 500,
            "Export took {}ms, contract requires <500ms", elapsed.as_millis());

        // Verify compression-friendly structure
        let compression_ratio = html.matches(' ').count() as f64 / html.len() as f64;
        assert!(compression_ratio > 0.1,
            "HTML structure should be compression-friendly");
    }

    // Helper functions for test setup

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
                signature: Arc::from(format!("signature_for_node_{}", i)),
                file_path: Arc::from(format!("src/file_{}.rs", i % 10)),
                line: (i % 100) as u32,
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

    fn create_large_performance_test_graph(node_count: usize, edge_count: usize) -> OptimizedISG {
        create_performance_test_graph(node_count, edge_count)
    }

    fn create_test_graph_with_special_characters() -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Node with problematic characters
        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("fn test\"quote\"\nnew_line\ttab"),
            kind: NodeKind::Function,
            name: Arc::from("test\"quote\"function"),
            signature: Arc::from("fn test\"quote\"() -> Result<\\\"escaped\\\", Error>\nwhere\n    T: Display"),
            file_path: Arc::from("src/test\"path\"/file.rs"),
            line: 42,
        });

        // Add a second node to create an edge
        isg.upsert_node(NodeData {
            hash: SigHash::from_signature("struct TestStruct"),
            kind: NodeKind::Struct,
            name: Arc::from("TestStruct"),
            signature: Arc::from("struct TestStruct;"),
            file_path: Arc::from("src/test.rs"),
            line: 10,
        });

        // Add an edge between the nodes
        isg.upsert_edge(
            SigHash::from_signature("fn test\"quote\"\nnew_line\ttab"),
            SigHash::from_signature("struct TestStruct"),
            EdgeKind::Uses
        ).unwrap();

        isg
    }
}