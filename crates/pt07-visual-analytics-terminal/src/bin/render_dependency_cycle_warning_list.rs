//! Visualization Binary 2: Dependency Cycle Warning List
//!
//! Detects and renders warnings for circular dependencies in the codebase.
//! By default, filters to implementation-only (Pareto principle).
//!
//! ## Usage
//! ```bash
//! pt07-render-dependency-cycle-warning-list --db parseltongue.db
//! pt07-render-dependency-cycle-warning-list --db parseltongue.db --include-tests
//! ```
//!
//! ## Output Example
//! ```text
//! ╔═══════════════════════════════════════════════╗
//! ║    Circular Dependency Warnings (Impl Only)  ║
//! ╠═══════════════════════════════════════════════╣
//! ║ ⚠️  CYCLE DETECTED (length: 3)                ║
//! ║    rust:fn:parse -> rust:fn:validate ->      ║
//! ║    rust:fn:format -> rust:fn:parse           ║
//! ╠═══════════════════════════════════════════════╣
//! ║ ⚠️  CYCLE DETECTED (length: 2)                ║
//! ║    rust:fn:read -> rust:fn:write ->          ║
//! ║    rust:fn:read                               ║
//! ╚═══════════════════════════════════════════════╝
//!
//! Total Cycles Found: 2
//! ✅ Recommendation: Refactor to eliminate circular dependencies
//! ```

use anyhow::Result;
use clap::Parser;
use parseltongue_core::entities::{CodeEntity, DependencyEdge};
use pt07_visual_analytics_terminal::core::{
    filter_implementation_edges_only,
    filter_implementation_entities_only,
    filter_include_all_edge_types,
    filter_include_all_entity_types,
};
use pt07_visual_analytics_terminal::save_visualization_output_to_file;
use std::collections::{HashMap, HashSet};

#[derive(Parser, Debug)]
#[command(name = "pt07-render-dependency-cycle-warning-list")]
#[command(about = "Detect and render circular dependency warnings")]
struct Args {
    /// Path to CozoDB database file
    #[arg(long)]
    db: String,

    /// Include test entities (default: implementation-only)
    #[arg(long, default_value_t = false)]
    include_tests: bool,
}

/// Simple cycle detection using DFS
///
/// Returns list of cycles found (each cycle is a Vec of entity keys)
fn detect_cycles_in_dependency_graph(
    edges: &[DependencyEdge],
) -> Vec<Vec<String>> {
    // Build adjacency list
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for edge in edges {
        graph
            .entry(edge.from_key.as_str().to_string())
            .or_insert_with(Vec::new)
            .push(edge.to_key.as_str().to_string());
    }

    let mut cycles: Vec<Vec<String>> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut rec_stack: HashSet<String> = HashSet::new();
    let mut path: Vec<String> = Vec::new();

    // DFS from each node
    for node in graph.keys() {
        if !visited.contains(node) {
            dfs_detect_cycle(
                node,
                &graph,
                &mut visited,
                &mut rec_stack,
                &mut path,
                &mut cycles,
            );
        }
    }

    cycles
}

fn dfs_detect_cycle(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs_detect_cycle(neighbor, graph, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(neighbor) {
                // Found a cycle - extract it from path
                if let Some(cycle_start) = path.iter().position(|n| n == neighbor) {
                    let cycle: Vec<String> = path[cycle_start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }
    }

    path.pop();
    rec_stack.remove(node);
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Build command args string for auto-save
    let command_args = if args.include_tests {
        format!("--db {} --include-tests", args.db)
    } else {
        format!("--db {}", args.db)
    };

    // TODO: Query entities and edges from CozoDB
    // For now, using stub data
    let all_entities: Vec<CodeEntity> = vec![];
    let all_edges: Vec<DependencyEdge> = vec![];

    // Apply filter based on --include-tests flag
    let filtered_entities = if args.include_tests {
        filter_include_all_entity_types(all_entities)
    } else {
        filter_implementation_entities_only(all_entities)
    };

    // Build impl_keys set from filtered entities
    let impl_keys: HashSet<String> = filtered_entities
        .iter()
        .map(|e| e.isgl1_key.clone())
        .collect();

    // Filter edges
    let filtered_edges = if args.include_tests {
        filter_include_all_edge_types(all_edges, &impl_keys)
    } else {
        filter_implementation_edges_only(all_edges, &impl_keys)
    };

    // Detect cycles
    let cycles = detect_cycles_in_dependency_graph(&filtered_edges);

    // Build visualization output
    let title = if args.include_tests {
        "Circular Dependency Warnings (All)"
    } else {
        "Circular Dependency Warnings (Impl Only)"
    };

    let mut output = String::new();
    output.push_str("╔═══════════════════════════════════════════════╗\n");
    output.push_str(&format!("║ {:^45} ║\n", title));
    output.push_str("╠═══════════════════════════════════════════════╣\n");

    if cycles.is_empty() {
        output.push_str("║ ✅ No circular dependencies detected!        ║\n");
    } else {
        for (idx, cycle) in cycles.iter().enumerate() {
            if idx > 0 {
                output.push_str("╠═══════════════════════════════════════════════╣\n");
            }
            output.push_str(&format!(
                "║ ⚠️  CYCLE DETECTED (length: {})                 ║\n",
                cycle.len()
            ));

            // Show cycle path (simplified)
            let cycle_str = cycle.join(" -> ");
            let cycle_with_return = format!("{} -> {}", cycle_str, cycle[0]);

            // Wrap long lines
            let max_width = 43;
            let words: Vec<&str> = cycle_with_return.split(" -> ").collect();
            let mut current_line = String::from("   ");

            for (i, word) in words.iter().enumerate() {
                let addition = if i == 0 {
                    word.to_string()
                } else {
                    format!(" -> {}", word)
                };

                if current_line.len() + addition.len() > max_width {
                    output.push_str(&format!("║ {:43} ║\n", current_line));
                    current_line = format!("   {}", word);
                } else {
                    current_line.push_str(&addition);
                }
            }

            if !current_line.trim().is_empty() {
                output.push_str(&format!("║ {:43} ║\n", current_line));
            }
        }
    }

    output.push_str("╚═══════════════════════════════════════════════╝\n");
    output.push_str(&format!("\nTotal Cycles Found: {}\n", cycles.len()));

    if !cycles.is_empty() {
        output.push_str("✅ Recommendation: Refactor to eliminate circular dependencies\n");
    }

    // Auto-save to timestamped file and print to stdout
    save_visualization_output_to_file(
        "pt07-render-dependency-cycle-warning-list",
        &command_args,
        &output,
    )?;

    Ok(())
}
