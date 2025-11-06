//! Visualization implementations
//!
//! This module contains the core visualization logic extracted from binaries.
//! All visualizations can be called directly from the unified pt07 binary.

use anyhow::Result;
use crate::core::{
    detect_cycles_in_dependency_graph,
    filter_implementation_edges_only,
    filter_implementation_entities_only,
    filter_include_all_edge_types,
    filter_include_all_entity_types,
};
use crate::database::Pt07DbAdapter;
use std::collections::{HashMap, HashSet};

/// Render entity count bar chart visualization
///
/// Returns the visualization as a string for display/saving.
pub async fn render_entity_count_bar_chart_visualization(
    db_path: &str,
    include_tests: bool,
) -> Result<String> {
    // Query entities from CozoDB
    let adapter = Pt07DbAdapter::connect_to_database_from_path(db_path).await?;
    let all_entities = adapter.query_all_entities_from_database().await?;

    // Apply filter based on include_tests flag
    let filtered_entities = if include_tests {
        filter_include_all_entity_types(all_entities)
    } else {
        filter_implementation_entities_only(all_entities)
    };

    // Count entities by type
    let mut counts: HashMap<String, usize> = HashMap::new();
    for entity in &filtered_entities {
        let type_name = format!("{:?}", entity.interface_signature.entity_type);
        *counts.entry(type_name).or_insert(0) += 1;
    }

    // Calculate total
    let total: usize = counts.values().sum();

    // Build visualization output
    let title = if include_tests {
        "Entity Count by Type (All)"
    } else {
        "Entity Count by Type (Impl Only)"
    };

    let mut output = String::new();
    output.push_str("╔═══════════════════════════════════════════╗\n");
    output.push_str(&format!("║ {:^41} ║\n", title));
    output.push_str("╠═══════════════════════════════════════════╣\n");

    if total == 0 {
        output.push_str("║  No entities found in database            ║\n");
    } else {
        // Sort by count descending
        let mut sorted_counts: Vec<_> = counts.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

        for (entity_type, count) in sorted_counts {
            let percentage = (*count as f64 / total as f64 * 100.0) as usize;
            let bar_length = (percentage as f64 / 100.0 * 14.0) as usize;
            let filled = "█".repeat(bar_length);
            let empty = "░".repeat(14 - bar_length);

            output.push_str(&format!(
                "║ {:10} [{}{}] {:3}  ({:2}%)  ║\n",
                entity_type, filled, empty, count, percentage
            ));
        }
    }

    output.push_str("╚═══════════════════════════════════════════╝\n");
    output.push_str(&format!("\nTotal {} Entities: {}\n",
        if include_tests { "All" } else { "Implementation" },
        total
    ));

    Ok(output)
}

/// Render dependency cycle warning list visualization
///
/// Returns the visualization as a string for display/saving.
pub async fn render_dependency_cycle_warning_list_visualization(
    db_path: &str,
    include_tests: bool,
) -> Result<String> {
    // Query entities and edges from CozoDB
    let adapter = Pt07DbAdapter::connect_to_database_from_path(db_path).await?;
    let all_entities = adapter.query_all_entities_from_database().await?;
    let all_edges = adapter.query_all_edges_from_database().await?;

    // Apply filter based on include_tests flag
    let filtered_entities = if include_tests {
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
    let filtered_edges = if include_tests {
        filter_include_all_edge_types(all_edges, &impl_keys)
    } else {
        filter_implementation_edges_only(all_edges, &impl_keys)
    };

    // Detect cycles
    let cycles = detect_cycles_in_dependency_graph(&filtered_edges);

    // Build visualization output
    let title = if include_tests {
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

    Ok(output)
}
