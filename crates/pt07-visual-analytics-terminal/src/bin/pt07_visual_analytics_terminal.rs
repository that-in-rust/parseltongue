//! Unified pt07 Binary: Visual Analytics Terminal
//!
//! Industry standard single-binary CLI with subcommands.
//! Follows patterns from cargo, git, ripgrep.
//!
//! ## Usage
//! ```bash
//! # Run all visualizations (default)
//! pt07-visual-analytics-terminal --db code.db
//!
//! # Run all with test entities
//! pt07-visual-analytics-terminal --db code.db --include-tests
//!
//! # Run specific visualization
//! pt07-visual-analytics-terminal --db code.db entity-count
//! pt07-visual-analytics-terminal --db code.db cycles
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use pt07_visual_analytics_terminal::save_visualization_output_to_file;
use pt07_visual_analytics_terminal::visualizations::{
    render_entity_count_bar_chart_visualization,
    render_dependency_cycle_warning_list_visualization,
};

#[derive(Parser, Debug)]
#[command(name = "pt07-visual-analytics-terminal")]
#[command(about = "Visual analytics for code graphs")]
#[command(version)]
struct Cli {
    /// Path to CozoDB database
    #[arg(long)]
    db: String,

    /// Include test entities (default: implementation-only)
    #[arg(long, default_value_t = false)]
    include_tests: bool,

    /// Specific visualization to run (default: all)
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Entity count bar chart
    EntityCount,
    /// Circular dependency warnings
    Cycles,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Build command args string for auto-save
    let command_args = if cli.include_tests {
        format!("--db {} --include-tests", cli.db)
    } else {
        format!("--db {}", cli.db)
    };

    // Determine which visualizations to run
    let run_all = cli.command.is_none();

    // Run entity count visualization
    if run_all || matches!(cli.command, Some(Commands::EntityCount)) {
        eprintln!("📊 Running entity count visualization...");

        let output = render_entity_count_bar_chart_visualization(
            &cli.db,
            cli.include_tests
        ).await?;

        save_visualization_output_to_file(
            "pt07-entity-count",
            &command_args,
            &output,
        )?;
    }

    // Run cycle detection visualization
    if run_all || matches!(cli.command, Some(Commands::Cycles)) {
        eprintln!("🔄 Running cycle detection visualization...");

        let output = render_dependency_cycle_warning_list_visualization(
            &cli.db,
            cli.include_tests
        ).await?;

        save_visualization_output_to_file(
            "pt07-cycles",
            &command_args,
            &output,
        )?;
    }

    eprintln!("\n✅ Visualization complete!");

    Ok(())
}
