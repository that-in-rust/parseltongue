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
use pt02_llm_cozodb_to_context_writer::cozodb_adapter::CozoDbAdapter;

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

    // Execute export based on level using dual file export
    match config.level {
        0 => {
            cli.verbose_print("Exporting Level 0: Pure edge list (dual files)");
            
            let exporter = pt02_llm_cozodb_to_context_writer::exporters::Level0Exporter::new();
            let storage = CozoDbAdapter::connect(&config.db_path).await?;
            
            // Extract base output name (remove .json extension if present)
            let output_str = config.output_path.to_string_lossy();
            let base_output = output_str
                .strip_suffix(".json")
                .unwrap_or(&output_str);
            
            exporter.export_dual_files(
                &storage,
                base_output,
                &config.where_filter
            ).await?;
            
            cli.verbose_print(&format!("✓ Level 0 dual export completed"));
            cli.verbose_print(&format!("  Output files: {}.json, {}_test.json", base_output, base_output));
        }
        1 => {
            cli.verbose_print("Exporting Level 1: Node-centric + ISG + Temporal (dual files)");
            
            let exporter = pt02_llm_cozodb_to_context_writer::exporters::Level1Exporter::new();
            let storage = CozoDbAdapter::connect(&config.db_path).await?;
            
            // Extract base output name
            let output_str = config.output_path.to_string_lossy();
            let base_output = output_str
                .strip_suffix(".json")
                .unwrap_or(&output_str);
            
            exporter.export_dual_files(
                &storage,
                base_output,
                config.include_code,
                &config.where_filter
            ).await?;
            
            cli.verbose_print(&format!("✓ Level 1 dual export completed"));
            cli.verbose_print(&format!("  Output files: {}.json, {}_test.json", base_output, base_output));
        }
        2 => {
            cli.verbose_print("Exporting Level 2: + Type system essentials (dual files)");
            
            let exporter = pt02_llm_cozodb_to_context_writer::exporters::Level2Exporter::new();
            let storage = CozoDbAdapter::connect(&config.db_path).await?;
            
            // Extract base output name
            let output_str = config.output_path.to_string_lossy();
            let base_output = output_str
                .strip_suffix(".json")
                .unwrap_or(&output_str);
            
            exporter.export_dual_files(
                &storage,
                base_output,
                config.include_code,
                &config.where_filter
            ).await?;
            
            cli.verbose_print(&format!("✓ Level 2 dual export completed"));
            cli.verbose_print(&format!("  Output files: {}.json, {}_test.json", base_output, base_output));
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid export level: {}", config.level));
        }
    }

    cli.verbose_print("✓ PT02 export completed successfully");
    Ok(())
}
