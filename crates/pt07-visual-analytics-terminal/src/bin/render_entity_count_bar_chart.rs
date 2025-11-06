//! Visualization Binary 1: Entity Count Bar Chart
//!
//! Renders a horizontal bar chart showing entity counts by type.
//! By default, filters to implementation-only (Pareto principle).
//!
//! ## Usage
//! ```bash
//! pt07-render-entity-count-bar-chart --db parseltongue.db
//! pt07-render-entity-count-bar-chart --db parseltongue.db --include-tests
//! ```
//!
//! ## Output Example
//! ```text
//! ╔═══════════════════════════════════════════╗
//! ║      Entity Count by Type (Impl Only)    ║
//! ╠═══════════════════════════════════════════╣
//! ║ Function    [████████████░░] 120  (45%)  ║
//! ║ Struct      [████████░░░░░░] 80   (30%)  ║
//! ║ Enum        [████░░░░░░░░░░] 40   (15%)  ║
//! ║ Trait       [██░░░░░░░░░░░░] 20   (7%)   ║
//! ║ Module      [█░░░░░░░░░░░░░] 10   (3%)   ║
//! ╚═══════════════════════════════════════════╝
//!
//! Total Implementation Entities: 270
//! ```

use anyhow::Result;
use clap::Parser;
use parseltongue_core::entities::CodeEntity;
use pt07_visual_analytics_terminal::core::{
    filter_implementation_entities_only,
    filter_include_all_entity_types,
};
use pt07_visual_analytics_terminal::database::Pt07DbAdapter;
use pt07_visual_analytics_terminal::save_visualization_output_to_file;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name = "pt07-render-entity-count-bar-chart")]
#[command(about = "Render entity count bar chart from CozoDB")]
struct Args {
    /// Path to CozoDB database file
    #[arg(long)]
    db: String,

    /// Include test entities (default: implementation-only)
    #[arg(long, default_value_t = false)]
    include_tests: bool,
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

    // Query entities from CozoDB using Pt07DbAdapter
    let adapter = Pt07DbAdapter::connect_to_database_from_path(&args.db).await?;
    let all_entities = adapter.query_all_entities_from_database().await?;

    // Apply filter based on --include-tests flag
    let filtered_entities = if args.include_tests {
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
    let title = if args.include_tests {
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
        if args.include_tests { "All" } else { "Implementation" },
        total
    ));

    // Auto-save to timestamped file and print to stdout
    save_visualization_output_to_file(
        "pt07-render-entity-count-bar-chart",
        &command_args,
        &output,
    )?;

    Ok(())
}
