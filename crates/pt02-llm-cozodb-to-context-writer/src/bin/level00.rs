//! PT02 Level 0: Pure Edge List Exporter
//!
//! # Usage
//!
//! ```bash
//! # Export all edges
//! pt02-level00 --where "ALL" --output edges.json
//!
//! # Filter by edge type
//! pt02-level00 --where "edge_type = 'depends_on'" --output deps.json
//! ```
//!
//! ## Level 0 Design
//!
//! - **Output**: Pure edge list (from_key, to_key, edge_type)
//! - **Token estimate**: ~2-5K tokens for ~2000 edges
//! - **Use case**: Dependency analysis, graph visualization
//! - **No --include-code flag**: Edges have no code

use anyhow::Result;
use clap::Parser;
use pt02_llm_cozodb_to_context_writer::{
    exporters::Level0Exporter,
    export_trait::LevelExporter,
    models::ExportConfig,
};
use std::path::PathBuf;

/// PT02 Level 0: Export pure edge list from CozoDB
#[derive(Parser, Debug)]
#[command(name = "pt02-level00")]
#[command(version, about = "Export pure edge list (Level 0)", long_about = None)]
struct Cli {
    /// Datalog WHERE clause or "ALL" (MANDATORY)
    ///
    /// Examples:
    ///   --where "ALL"
    ///   --where "edge_type = 'depends_on'"
    ///   --where "edge_type = 'implements'"
    ///
    /// Datalog syntax:
    ///   - AND: Use comma (,)     NOT &&
    ///   - OR: Use semicolon (;)  NOT ||
    ///   - Equality: Use =        NOT ==
    #[arg(long)]
    where_clause: String,

    /// Output JSON file path
    ///
    /// Default: ISGLevel00.json
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Database file path
    #[arg(long, default_value = "parseltongue.db")]
    db: String,

    /// Verbose output (show progress, token estimates)
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    fn validate(&self) -> Result<ExportConfig> {
        // Validate WHERE clause non-empty
        if self.where_clause.trim().is_empty() {
            return Err(anyhow::anyhow!(
                "WHERE clause cannot be empty. Use --where \"ALL\" to export all edges."
            ));
        }

        // Build config
        Ok(ExportConfig {
            level: 0,
            include_code: false,  // N/A for Level 0
            where_filter: self.where_clause.clone(),
            output_path: self.output.clone().unwrap_or_else(|| {
                PathBuf::from("ISGLevel00.json")
            }),
            db_path: self.db.clone(),
        })
    }

    fn verbose_print(&self, message: &str) {
        if self.verbose {
            eprintln!("[PT02-L0] {}", message);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Validate and get configuration
    let config = cli.validate()?;

    // Verbose logging
    cli.verbose_print("Starting PT02 Level 0 export (Pure Edge List)");
    cli.verbose_print(&format!("Database: {}", config.db_path));
    cli.verbose_print(&format!("Output: {:?}", config.output_path));
    cli.verbose_print(&format!("WHERE filter: {}", config.where_filter));

    // Create exporter
    let exporter = Level0Exporter::new();
    cli.verbose_print(&format!("Estimated tokens: ~{}", exporter.estimated_tokens()));

    // TODO: Connect to real CozoDB and export
    // For now, this is a stub showing the binary structure
    println!("PT02 Level 0 binary created successfully!");
    println!("TODO: Connect to CozoDB at {}", config.db_path);
    println!("TODO: Export edges to {:?}", config.output_path);

    Ok(())
}
