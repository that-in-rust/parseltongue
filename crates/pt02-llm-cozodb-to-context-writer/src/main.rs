//! PT02 main entry point
//!
//! # Phase 1: STUB
//!
//! This file contains only the executable specification (contract).
//! Implementation marked with `todo!()` to be filled in Phase 3 (GREEN).
//!
//! ## Architecture
//!
//! Level-based progressive disclosure:
//! - Level 0: Pure edge list (~2-5K tokens)
//! - Level 1: Node-centric + ISG + Temporal (~30K tokens)
//! - Level 2: + Type system essentials (~60K tokens)

use anyhow::Result;
use clap::Parser;
use pt02_llm_cozodb_to_context_writer::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Validate and get configuration
    let config = cli.validate()?;

    // Verbose logging
    cli.verbose_print(&format!("Starting PT02 Level {} export", config.level));
    cli.verbose_print(&format!("Database: {}", config.db_path));
    cli.verbose_print(&format!("Output: {:?}", config.output_path));
    cli.verbose_print(&format!("WHERE filter: {}", config.where_filter));

    if config.level > 0 {
        cli.verbose_print(&format!(
            "Include code: {}",
            if config.include_code { "YES (expensive)" } else { "NO (cheap)" }
        ));
    }

    // Execute export based on level
    // TODO: Phase 3 (GREEN) - Implement actual export logic
    match config.level {
        0 => {
            cli.verbose_print("Exporting Level 0: Pure edge list");
            todo!("Level 0 export implementation (Phase 3)")
        }
        1 => {
            cli.verbose_print("Exporting Level 1: Node-centric + ISG + Temporal");
            todo!("Level 1 export implementation (Phase 3)")
        }
        2 => {
            cli.verbose_print("Exporting Level 2: + Type system essentials");
            todo!("Level 2 export implementation (Phase 4)")
        }
        _ => unreachable!("CLI validation should prevent invalid levels"),
    }
}
