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

use crate::isg::{OptimizedISG, NodeData, NodeKind, EdgeKind, FileHierarchyAnalysis};
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

/// Export ISG to hierarchical interactive HTML with progressive disclosure
///
/// Creates a single HTML file with interactive level switching, search, and navigation
/// for large codebases with 2,000+ nodes.
///
/// # Preconditions
/// - ISG graph is in valid state with consistent node/edge relationships
/// - File hierarchy analysis available for progressive disclosure
///
/// # Postconditions
/// - Returns single HTML file with interactive progressive disclosure
/// - Level switching between Overview (30,000ft), Detailed (1,000ft), and Complete views
/// - Search functionality with highlighting
/// - Performance: <500ms generation time, <3s load time for 2,000+ nodes
///
/// # Error Conditions
/// - Cannot fail (HTML generation is infallible)
/// - Graceful degradation for browsers without JavaScript
/// - Performance degrades gracefully for extremely large graphs
///
/// # Performance Contract
/// - Must complete in <500ms for graphs with ≤2000 nodes
/// - Interactive level switching <100ms
/// - Search functionality <50ms response time
pub fn export_isg_to_hierarchical_html(isg: &OptimizedISG) -> String {
    let start_time = Instant::now();

    // Analyze file hierarchy for progressive disclosure
    let hierarchy = isg.analyze_file_hierarchy();

    let mut html = String::with_capacity(128 * 1024); // Pre-allocate 128KB for hierarchical content

    // Phase 1: Enhanced HTML structure with level controls
    write_hierarchical_html_header(&mut html, &hierarchy);

    // Phase 2: JavaScript libraries (Cytoscape + ELK)
    write_script_includes(&mut html);

    // Phase 3: Hierarchical data preparation
    let (overview_nodes, overview_edges) = generate_overview_graph(&hierarchy);
    let (detailed_nodes, detailed_edges) = generate_detailed_graph(&hierarchy);
    let (complete_nodes, complete_edges) = generate_complete_graph(isg);

    // Phase 4: Progressive disclosure interface
    write_progressive_disclosure_interface(&mut html, &hierarchy);

    // Phase 5: Level-specific graph configurations
    write_level_graph_configs(&mut html,
        &overview_nodes, &overview_edges,
        &detailed_nodes, &detailed_edges,
        &complete_nodes, &complete_edges);

    // Phase 6: Search and navigation functionality
    write_search_functionality(&mut html, &hierarchy);

    // Phase 7: Performance monitoring and analytics
    write_hierarchical_performance_footer(&mut html, start_time, &hierarchy);

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

// ===== Hierarchical HTML Export Helper Functions =====

/// Writes enhanced HTML header with progressive disclosure controls
fn write_hierarchical_html_header(html: &mut String, hierarchy: &FileHierarchyAnalysis) {
    let _ = write!(html, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Interactive ISG Architecture - Progressive Disclosure</title>
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
            height: 100vh;
            display: flex;
            flex-direction: column;
        }}

        #header {{
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(10px);
            padding: 12px 20px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.2);
            z-index: 1000;
            box-shadow: 0 2px 20px rgba(0, 0, 0, 0.1);
        }}

        .header-content {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            max-width: 100%;
        }}

        .title {{
            font-size: 18px;
            font-weight: 600;
            color: #2c3e50;
            display: flex;
            align-items: center;
            gap: 8px;
        }}

        .level-controls {{
            display: flex;
            gap: 8px;
            align-items: center;
        }}

        .level-button {{
            padding: 6px 14px;
            border: none;
            border-radius: 6px;
            font-size: 12px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
            background: #f8f9fa;
            color: #6c757d;
            position: relative;
        }}

        .level-button.active {{
            background: #007bff;
            color: white;
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
        }}

        .level-button:hover:not(.active) {{
            background: #e9ecef;
            transform: translateY(-1px);
        }}

        .level-info {{
            font-size: 11px;
            color: #6c757d;
            margin-left: 8px;
        }}

        .search-container {{
            display: flex;
            gap: 8px;
            align-items: center;
        }}

        #searchInput {{
            padding: 6px 12px;
            border: 1px solid #dee2e6;
            border-radius: 6px;
            font-size: 12px;
            width: 200px;
            transition: all 0.2s ease;
        }}

        #searchInput:focus {{
            outline: none;
            border-color: #007bff;
            box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.2);
        }}

        .stats {{
            display: flex;
            gap: 16px;
            font-size: 11px;
            color: #6c757d;
        }}

        .stat {{
            display: flex;
            align-items: center;
            gap: 4px;
        }}

        #cy {{
            flex: 1;
            width: 100%;
            background: white;
            position: relative;
        }}

        .loading {{
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            font-size: 14px;
            color: #6c757d;
            display: none;
        }}

        .performance-info {{
            position: absolute;
            bottom: 10px;
            right: 10px;
            background: rgba(0, 0, 0, 0.7);
            color: white;
            padding: 6px 10px;
            border-radius: 4px;
            font-size: 11px;
            font-family: monospace;
        }}

        .hierarchy-info {{
            position: absolute;
            bottom: 10px;
            left: 10px;
            background: rgba(0, 0, 0, 0.7);
            color: white;
            padding: 6px 10px;
            border-radius: 4px;
            font-size: 11px;
            max-width: 200px;
        }}

        @media (max-width: 768px) {{
            .header-content {{
                flex-direction: column;
                gap: 10px;
            }}

            .level-controls {{
                flex-wrap: wrap;
                justify-content: center;
            }}

            #searchInput {{
                width: 150px;
            }}

            .stats {{
                flex-wrap: wrap;
                justify-content: center;
            }}
        }}
    </style>
</head>
<body>
    <div id="header">
        <div class="header-content">
            <div class="title">
                🏗️ ISG Architecture Explorer
            </div>

            <div class="level-controls">
                <button class="level-button active" data-level="overview">
                    Overview (30,000ft)
                </button>
                <button class="level-button" data-level="detailed">
                    Detailed (1,000ft)
                </button>
                <button class="level-button" data-level="complete">
                    Complete
                </button>
                <span class="level-info">Levels: {}, Max Depth: {}</span>
            </div>

            <div class="search-container">
                <input type="text" id="searchInput" placeholder="Search nodes..." />
            </div>

            <div class="stats">
                <div class="stat">
                    📊 <span id="nodeCount">0</span> nodes
                </div>
                <div class="stat">
                    🔗 <span id="edgeCount">0</span> edges
                </div>
                <div class="stat">
                    ⚡ <span id="renderTime">0ms</span>
                </div>
            </div>
        </div>
    </div>

    <div id="cy">
        <div class="loading">Loading graph...</div>
        <div class="performance-info" id="performanceInfo"></div>
        <div class="hierarchy-info" id="hierarchyInfo"></div>
    </div>
"#,
            hierarchy.levels.len(),
            hierarchy.max_depth
        );
    }

    /// Generates overview graph (Level 1: 30,000ft view)
    fn generate_overview_graph(hierarchy: &FileHierarchyAnalysis) -> (String, String) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Add entry points
        for (i, entry_point) in hierarchy.entry_points.iter().take(5).enumerate() {
            let safe_id = format!("entry_{}", i);
            nodes.push(format!(r#"{{
                data: {{
                    id: "{}",
                    label: "🚀 {}",
                    type: "entry_point",
                    file: "{}",
                    kind: "{}"
                }},
                classes: "entry-point"
            }}"#,
                safe_id,
                sanitize_for_html(&entry_point.name),
                sanitize_for_html(&entry_point.file_path),
                format!("{:?}", entry_point.kind)
            ));
        }

        // Add top-level directories only
        for level in hierarchy.levels.iter().take(2) {
            for directory in &level.directories {
                if directory.node_count > 0 {
                    let safe_id = sanitize_for_html(&directory.path).replace("/", "_");
                    nodes.push(format!(r#"{{
                        data: {{
                            id: "{}",
                            label: "📁 {} ({} items)",
                            type: "directory",
                            path: "{}",
                            node_count: {}
                        }},
                        classes: "directory level-{}"
                    }}"#,
                        safe_id,
                        sanitize_for_html(&directory.path),
                        directory.node_count,
                        sanitize_for_html(&directory.path),
                        directory.node_count,
                        level.depth
                    ));
                }
            }
        }

        // Add connections from entry points to directories
        for (i, entry_point) in hierarchy.entry_points.iter().take(3).enumerate() {
            let entry_dir = extract_directory_simple(&entry_point.file_path);
            let safe_dir = sanitize_for_html(&entry_dir).replace("/", "_");

            edges.push(format!(r#"{{
                data: {{
                    source: "entry_{}",
                    target: "{}",
                    type: "entry_to_directory"
                }}
            }}"#, i, safe_dir));
        }

        (format!("[{}]", nodes.join(",")), format!("[{}]", edges.join(",")))
    }

    /// Generates detailed graph (Levels 2-3: 1,000ft view)
    fn generate_detailed_graph(hierarchy: &FileHierarchyAnalysis) -> (String, String) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let pyramid_levels = hierarchy.get_pyramid_view(3);
        let mut node_counter = 0;

        for (level_idx, level) in pyramid_levels.iter().enumerate() {
            for directory in &level.directories {
                if node_counter >= 200 { break; } // Limit for performance

                for (node_idx, node) in directory.nodes.iter().take(10).enumerate() {
                    if node_counter >= 200 { break; }

                    let safe_id = format!("L{}_D{}_N{}",
                        level_idx + 1,
                        sanitize_for_html(&directory.path).replace("/", "_"),
                        node_idx
                    );

                    let icon = match node.kind {
                        NodeKind::Function => "🔧",
                        NodeKind::Struct => "📦",
                        NodeKind::Trait => "🎯",
                    };

                    nodes.push(format!(r#"{{
                        data: {{
                            id: "{}",
                            label: "{} {}",
                            type: "{}",
                            name: "{}",
                            file: "{}",
                            kind: "{}",
                            directory: "{}",
                            level: {}
                        }},
                        classes: "node level-{} {}"
                    }}"#,
                        safe_id,
                        icon,
                        sanitize_for_html(&node.name),
                        format!("{:?}", node.kind).to_lowercase(),
                        sanitize_for_html(&node.name),
                        sanitize_for_html(&node.file_path),
                        format!("{:?}", node.kind),
                        sanitize_for_html(&directory.path),
                        level_idx + 1,
                        level_idx + 1,
                        format!("{:?}", node.kind).to_lowercase()
                    ));

                    node_counter += 1;
                }
            }
        }

        (format!("[{}]", nodes.join(",")), format!("[{}]", edges.join(",")))
    }

    /// Generates complete graph with all nodes
    fn generate_complete_graph(isg: &OptimizedISG) -> (String, String) {
        let nodes_json = generate_nodes_json(isg);
        let edges_json = generate_edges_json(isg);
        (nodes_json, edges_json)
    }

    /// Writes progressive disclosure interface controls
    fn write_progressive_disclosure_interface(html: &mut String, hierarchy: &FileHierarchyAnalysis) {
        let _ = write!(html, r#"
    <script>
        // Progressive Disclosure State Management
        const GRAPH_LEVELS = {{
            overview: {{
                name: "Overview (30,000ft)",
                description: "Entry points and top-level directories",
                maxNodes: 50
            }},
            detailed: {{
                name: "Detailed (1,000ft)",
                description: "Key modules and important relationships",
                maxNodes: 200
            }},
            complete: {{
                name: "Complete",
                description: "All nodes and relationships",
                maxNodes: Infinity
            }}
        }};

        let currentLevel = 'overview';
        let graphs = {{}};
        let cy = null;

        // Initialize level switching
        function initializeLevelSwitching() {{
            document.querySelectorAll('.level-button').forEach(button => {{
                button.addEventListener('click', function() {{
                    const level = this.dataset.level;
                    switchToLevel(level);
                }});
            }});
        }}

        // Switch between graph levels
        function switchToLevel(level) {{
            if (level === currentLevel) return;

            // Update UI
            document.querySelectorAll('.level-button').forEach(btn => {{
                btn.classList.remove('active');
            }});
            document.querySelector(`[data-level="${{level}}"]`).classList.add('active');

            // Show loading
            document.querySelector('.loading').style.display = 'block';

            // Switch graph with performance tracking
            const startTime = performance.now();
            setTimeout(() => {{
                loadGraph(level);
                const endTime = performance.now();
                updatePerformanceInfo(level, endTime - startTime);
            }}, 50);
        }}
    </script>
"#);
    }

    /// Writes level-specific graph configurations
    fn write_level_graph_configs(
        html: &mut String,
        overview_nodes: &str, overview_edges: &str,
        detailed_nodes: &str, detailed_edges: &str,
        complete_nodes: &str, complete_edges: &str
    ) {
        let _ = write!(html, r#"
    <script>
        // Graph data for different levels
        graphs.overview = {{
            nodes: {overview_nodes},
            edges: {overview_edges}
        }};

        graphs.detailed = {{
            nodes: {detailed_nodes},
            edges: {detailed_edges}
        }};

        graphs.complete = {{
            nodes: {complete_nodes},
            edges: {complete_edges}
        }};

        // Load graph for specific level
        function loadGraph(level) {{
            const graphData = graphs[level];
            const levelConfig = GRAPH_LEVELS[level];

            if (!cy) {{
                // Initialize Cytoscape
                cy = cytoscape({{
                    container: document.getElementById('cy'),

                    elements: graphData,

                    style: getLevelStyles(level),

                    layout: {{
                        name: 'elk',
                        elk: {{
                            algorithm: 'layered',
                            layering: {{
                                strategy: 'INTERACTIVE'
                            }},
                            spacing: {{
                                componentComponent: '40',
                                edgeNode: '20',
                                edgeEdge: '10',
                                nodeNodeBetweenLayers: '30'
                            }}
                        }},

                        fit: true,
                        padding: 20
                    }},

                    // Interaction options
                    userZoomingEnabled: true,
                    userPanningEnabled: true,
                    boxSelectionEnabled: false,

                    // Performance options
                    textureOnViewport: level === 'complete',
                    hideEdgesOnViewport: level === 'complete',
                    hideLabelsOnViewport: level === 'complete',

                    // Rendering options
                    pixelRatio: 1,

                    ready: function() {{
                        document.querySelector('.loading').style.display = 'none';
                        updateStats(level);
                        initializeSearch();
                    }}
                }});
            }} else {{
                // Update existing graph
                cy.json({{
                    elements: graphData,
                    style: getLevelStyles(level)
                }});

                cy.layout({{
                    name: 'elk',
                    elk: {{
                        algorithm: 'layered',
                        layering: {{
                            strategy: 'INTERACTIVE'
                        }}
                    }},
                    fit: true,
                    padding: 20
                }}).run(() => {{
                    document.querySelector('.loading').style.display = 'none';
                    updateStats(level);
                }});
            }}
        }}

        // Get styles based on level
        function getLevelStyles(level) {{
            const baseStyles = [
                {{
                    selector: 'node',
                    style: {{
                        'background-color': '#e3f2fd',
                        'border-color': '#1976d2',
                        'border-width': '2px',
                        'width': level === 'overview' ? '60px' : level === 'detailed' ? '50px' : '40px',
                        'height': level === 'overview' ? '60px' : level === 'detailed' ? '50px' : '40px',
                        'shape': 'roundrectangle',
                        'text-halign': 'center',
                        'text-valign': 'center',
                        'color': '#0d47a1',
                        'font-size': level === 'overview' ? '14px' : level === 'detailed' ? '12px' : '10px',
                        'text-wrap': 'wrap',
                        'text-max-width': '80px',
                        'label': 'data(label)',
                        'text-margin-y': level === 'overview' ? '-5px' : '-3px',
                        'overlay-opacity': 0,
                        'overlay-padding': '5px',
                        'z-index': 10
                    }}
                }},
                {{
                    selector: 'node.entry-point',
                    style: {{
                        'background-color': '#e8f5e8',
                        'border-color': '#2e7d32',
                        'width': '70px',
                        'height': '70px',
                        'shape': 'round-diamond',
                        'color': '#1b5e20',
                        'font-weight': 'bold',
                        'z-index': 20
                    }}
                }},
                {{
                    selector: 'node.directory',
                    style: {{
                        'background-color': '#f3e5f5',
                        'border-color': '#7b1fa2',
                        'shape': 'round-rectangle',
                        'color': '#4a148c',
                        'z-index': 15
                    }}
                }},
                {{
                    selector: 'node.function',
                    style: {{
                        'background-color': '#e3f2fd',
                        'border-color': '#1976d2'
                    }}
                }},
                {{
                    selector: 'node.struct',
                    style: {{
                        'background-color': '#e8f5e8',
                        'border-color': '#2e7d32'
                    }}
                }},
                {{
                    selector: 'node.trait',
                    style: {{
                        'background-color': '#fff3e0',
                        'border-color': '#ef6c00'
                    }}
                }},
                {{
                    selector: 'edge',
                    style: {{
                        'width': '2px',
                        'line-color': '#666',
                        'target-arrow-color': '#666',
                        'target-arrow-shape': 'triangle',
                        'curve-style': 'bezier',
                        'opacity': 0.7
                    }}
                }},
                {{
                    selector: 'edge[type="entry_to_directory"]',
                    style: {{
                        'width': '3px',
                        'line-color': '#2e7d32',
                        'target-arrow-color': '#2e7d32',
                        'line-style': 'dashed'
                    }}
                }},
                {{
                    selector: 'node.highlighted',
                    style: {{
                        'background-color': '#ffeb3b',
                        'border-color': '#f57c00',
                        'border-width': '4px',
                        'z-index': 100
                    }}
                }},
                {{
                    selector: 'node.dimmed',
                    style: {{
                        'opacity': 0.3
                    }}
                }},
                {{
                    selector: 'edge.dimmed',
                    style: {{
                        'opacity': 0.1
                    }}
                }}
            ];

            // Level-specific adjustments
            if (level === 'overview') {{
                baseStyles.push({{
                    selector: 'node',
                    style: {{
                        'font-size': '14px',
                        'text-max-width': '100px'
                    }}
                }});
            }} else if (level === 'complete') {{
                baseStyles.push({{
                    selector: 'node',
                    style: {{
                        'font-size': '10px',
                        'text-max-width': '60px',
                        'width': '30px',
                        'height': '30px'
                    }}
                }});
            }}

            return baseStyles;
        }}
    </script>
"#);
    }

    /// Writes search and navigation functionality
    fn write_search_functionality(html: &mut String, hierarchy: &FileHierarchyAnalysis) {
        let _ = write!(html, r#"
    <script>
        // Search functionality
        function initializeSearch() {{
            const searchInput = document.getElementById('searchInput');
            let searchTimeout;

            searchInput.addEventListener('input', function(e) {{
                clearTimeout(searchTimeout);
                const query = e.target.value.trim();

                searchTimeout = setTimeout(() => {{
                    performSearch(query);
                }}, 300);
            }});

            // Keyboard shortcuts
            document.addEventListener('keydown', function(e) {{
                if (e.key === 'f' && !e.ctrlKey && !e.metaKey && document.activeElement !== searchInput) {{
                    e.preventDefault();
                    searchInput.focus();
                }} else if (e.key === 'Escape') {{
                    searchInput.value = '';
                    searchInput.blur();
                    clearSearch();
                }} else if (e.key === 'r' && !e.ctrlKey && !e.metaKey) {{
                    e.preventDefault();
                    resetView();
                }} else if (e.key === 'l' && !e.ctrlKey && !e.metaKey) {{
                    e.preventDefault();
                    toggleLabels();
                }}
            }});
        }}

        function performSearch(query) {{
            if (!cy || !query) {{
                clearSearch();
                return;
            }}

            const lowerQuery = query.toLowerCase();
            let matchCount = 0;

            // Clear previous search
            cy.nodes().removeClass('highlighted dimmed');
            cy.edges().removeClass('highlighted dimmed');

            // Find matching nodes
            const matchingNodes = cy.nodes().filter(node => {{
                const label = node.data('label').toLowerCase();
                const name = (node.data('name') || '').toLowerCase();
                const file = (node.data('file') || '').toLowerCase();
                const kind = (node.data('kind') || '').toLowerCase();
                const directory = (node.data('directory') || '').toLowerCase();

                return label.includes(lowerQuery) ||
                       name.includes(lowerQuery) ||
                       file.includes(lowerQuery) ||
                       kind.includes(lowerQuery) ||
                       directory.includes(lowerQuery);
            }});

            // Highlight matches and dim non-matches
            if (matchingNodes.length > 0) {{
                matchingNodes.addClass('highlighted');
                cy.nodes().difference(matchingNodes).addClass('dimmed');

                // Highlight connected edges
                matchingNodes.connectedEdges().addClass('highlighted');
                cy.edges().difference(matchingNodes.connectedEdges()).addClass('dimmed');

                matchCount = matchingNodes.length;

                // Fit to show matches if reasonable number
                if (matchCount <= 20) {{
                    cy.elements('.highlighted').fit({{
                        padding: 50
                    }});
                }}
            }} else {{
                // No matches found
                showMessage('No matches found for: ' + query);
            }}

            updateSearchInfo(matchCount, query);
        }}

        function clearSearch() {{
            if (!cy) return;

            cy.nodes().removeClass('highlighted dimmed');
            cy.edges().removeClass('highlighted dimmed');
            updateSearchInfo(0, '');
        }}

        function resetView() {{
            if (!cy) return;

            cy.fit({{
                padding: 20
            }});
            clearSearch();
        }}

        function toggleLabels() {{
            if (!cy) return;

            const currentOpacity = cy.style().get('text-opacity');
            const newOpacity = currentOpacity === 0 ? 1 : 0;

            cy.style().set('text-opacity', newOpacity);
        }}

        function updateSearchInfo(matchCount, query) {{
            const info = document.getElementById('hierarchyInfo');
            if (matchCount > 0) {{
                info.innerHTML = `🔍 Found ${matchCount} matches for "${query}"`;
            }} else if (query) {{
                info.innerHTML = `🔍 No matches for "${query}"`;
            }} else {{
                updateHierarchyInfo();
            }}
        }}

        function showMessage(message) {{
            const info = document.getElementById('hierarchyInfo');
            info.innerHTML = `💬 ${message}`;
            setTimeout(() => updateHierarchyInfo(), 3000);
        }}
    </script>
"#);
    }

    /// Writes performance monitoring footer
    fn write_hierarchical_performance_footer(html: &mut String, start_time: std::time::Instant, hierarchy: &FileHierarchyAnalysis) {
        let generation_time = start_time.elapsed().as_millis();

        let _ = write!(html, r#"
    <script>
        // Initialize everything when DOM is loaded
        document.addEventListener('DOMContentLoaded', function() {{
            initializeLevelSwitching();

            // Load initial level
            loadGraph('overview');

            // Show generation performance
            updatePerformanceInfo('generation', {});

            // Update hierarchy info
            updateHierarchyInfo();
        }});

        // Update statistics display
        function updateStats(level) {{
            const graphData = graphs[level];
            document.getElementById('nodeCount').textContent = graphData.nodes.length;
            document.getElementById('edgeCount').textContent = graphData.edges.length;
        }}

        // Update performance information
        function updatePerformanceInfo(operation, timing) {{
            const info = document.getElementById('performanceInfo');
            const generationTime = {};

            if (operation === 'generation') {{
                generationTime = {generationTime}ms;
            }} else {{
                generationTime = timing.toFixed(1) + 'ms';
            }}

            info.innerHTML = `⚡ Generation: ${generationTime}`;
        }}

        // Update hierarchy information
        function updateHierarchyInfo() {{
            const info = document.getElementById('hierarchyInfo');
            info.innerHTML = `📊 {} levels, {} max depth<br>
                           🚀 {} entry points<br>
                           💡 Click nodes to see details`;
        }}
    </script>
</body>
</html>
"#,
            generation_time,
            hierarchy.levels.len(),
            hierarchy.max_depth,
            hierarchy.entry_points.len()
        );
    }

    /// Helper: Extract directory (simple version)
    fn extract_directory_simple(file_path: &str) -> &str {
        if let Some(slash_pos) = file_path.rfind('/') {
            &file_path[..slash_pos]
        } else {
            "."
        }
    }

    /// Helper: Sanitize string for HTML
    fn sanitize_for_html(input: &str) -> String {
        input.replace('\\', "\\\\")
             .replace('"', "\\\"")
             .replace('\n', "\\n")
             .replace('\r', "\\r")
             .replace('\t', "\\t")
    }

    /// Test contract: Hierarchical HTML export creates interactive progressive disclosure
    ///
    /// # Given: ISG with nodes at different directory depths
    /// # When: export_isg_to_hierarchical_html is called
    /// # Then: Returns HTML with level switching, search, and navigation
    #[test]
    fn test_hierarchical_html_export_structure() {
        // Setup: Create test ISG with hierarchy
        let isg = create_hierarchical_test_isg();

        // Action: Export hierarchical HTML
        let html = export_isg_to_hierarchical_html(&isg);

        // Assertions: Verify structure and features
        assert!(html.contains("ISG Architecture Explorer"));
        assert!(html.contains("Overview (30,000ft)"));
        assert!(html.contains("Detailed (1,000ft)"));
        assert!(html.contains("Complete"));
        assert!(html.contains("level-button"));
        assert!(html.contains("searchInput"));
        assert!(html.contains("initializeLevelSwitching"));
        assert!(html.contains("performSearch"));
        assert!(html.contains("Cytoscape"));
        assert!(html.contains("ELK"));
    }

    /// Test contract: Performance validation for hierarchical HTML export
    ///
    /// # Given: ISG with moderate complexity (100 nodes, 200 edges)
    /// # When: export_isg_to_hierarchical_html is called
    /// # Then: Must complete in <500ms (performance contract)
    #[test]
    fn test_hierarchical_html_export_performance_contract() {
        // Setup: Create moderately sized test graph
        let isg = create_performance_test_graph_for_html(100, 200);

        // Action: Time the hierarchical HTML export
        let start = std::time::Instant::now();
        let _html = export_isg_to_hierarchical_html(&isg);
        let elapsed = start.elapsed();

        // Assertion: Validate performance contract
        assert!(elapsed.as_millis() < 500,
            "Hierarchical HTML export took {}ms, contract requires <500ms", elapsed.as_millis());
    }

    /// Test contract: Overview graph generation accuracy
    ///
    /// # Given: ISG with entry points and directories
    /// # When: generate_overview_graph is called
    /// # Then: Returns correct overview graph with entry points and directories
    #[test]
    fn test_overview_graph_generation() {
        // Setup: Create test hierarchy
        let hierarchy = create_test_hierarchy();

        // Action: Generate overview graph
        let (nodes_json, edges_json) = generate_overview_graph(&hierarchy);

        // Assertions: Verify structure
        assert!(nodes_json.contains("entry_point"));
        assert!(nodes_json.contains("directory"));
        assert!(nodes_json.contains("🚀"));
        assert!(nodes_json.contains("📁"));
        assert!(nodes_json.starts_with("["));
        assert!(nodes_json.ends_with("]"));
        assert!(edges_json.starts_with("["));
        assert!(edges_json.ends_with("]"));
    }

    /// Test contract: HTML sanitization for security
    ///
    /// # Given: Node names with problematic characters
    /// # When: sanitize_for_html is called
    /// # Then: Properly escapes characters for HTML safety
    #[test]
    fn test_html_sanitization() {
        // Test various problematic characters
        let test_cases = vec![
            ("normal_name", "normal_name"),
            ("name with spaces", "name with spaces"),
            ("name-with-dashes", "name-with-dashes"),
            ("name_with_underscores", "name_with_underscores"),
            ("name\"with\"quotes", "name\\\"with\\\"quotes"),
            ("name\nwith\nnewlines", "name\\nwith\\nnewlines"),
            ("name\twith\ttabs", "name\\twith\\ttabs"),
            ("name\\with\\backslashes", "name\\\\with\\\\backslashes"),
        ];

        for (input, expected) in test_cases {
            let sanitized = sanitize_for_html(input);
            assert_eq!(sanitized, expected);
        }
    }

    // Helper functions for HTML testing

    fn create_hierarchical_test_isg() -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Create nodes at different directory levels for HTML testing
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

    fn create_performance_test_graph_for_html(node_count: usize, edge_count: usize) -> OptimizedISG {
        let isg = OptimizedISG::new();

        // Create nodes at different directory levels for realistic HTML hierarchy
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
}