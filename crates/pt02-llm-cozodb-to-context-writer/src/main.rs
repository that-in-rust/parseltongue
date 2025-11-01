//! Standalone PT02: LLM-cozoDB-to-context-writer
//!
//! Ultra-minimalist tool for exporting entity graphs from CozoDB to JSON.
//! Following S01 principles: NO LLM calls, NO complex optimization, just simple DB-to-JSON export.
//!
//! ## Philosophy (S01 Ultra-Minimalist)
//!
//! - NO automatic LLM calls (context optimization happens externally)
//! - NO HTTP requests (offline operation)
//! - Direct database export to JSON only
//! - 3 CLI arguments total
//!
//! ## Examples
//!
//! ```bash
//! # Export all entities
//! pt02-llm-cozodb-to-context-writer --output context.json --db rocksdb:demo.db
//!
//! # Export only changed entities
//! pt02-llm-cozodb-to-context-writer --output changes.json --db rocksdb:demo.db --filter changed
//!
//! # Export current entities only
//! pt02-llm-cozodb-to-context-writer --output current.json --filter current
//! ```

use console::style;
use anyhow::Result;
use std::io::Write;

use parseltongue_core::storage::CozoDbStorage;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments (S01: Minimal - only 3 args)
    let cli = pt02_llm_cozodb_to_context_writer::cli::CliConfig::build_cli();
    let matches = cli.get_matches();

    let output = matches.get_one::<String>("output").unwrap();
    let db = matches.get_one::<String>("db").unwrap();
    let filter = matches.get_one::<String>("filter").unwrap();

    println!("{}", style("Running Tool 2: pt02-llm-cozodb-to-context-writer").cyan());
    println!("  Database: {}", db);
    println!("  Filter: {}", filter);
    println!("  Output: {}", output);

    // Connect to database
    let storage = CozoDbStorage::new(db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Fetch entities based on filter
    let entities = match filter.as_str() {
        "all" => storage.get_all_entities().await?,
        "changed" => storage.get_changed_entities().await?,
        "current" => {
            // For MVP: "current" means entities where current_ind=true
            storage.get_all_entities().await?
                .into_iter()
                .filter(|e| e.temporal_state.current_ind)
                .collect()
        }
        _ => unreachable!("clap validation should prevent this"),
    };

    println!("  Found {} entities", entities.len());

    // Write to JSON file
    let json = serde_json::to_string_pretty(&entities)
        .map_err(|e| anyhow::anyhow!("Failed to serialize entities: {}", e))?;

    let mut file = std::fs::File::create(output)
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))?;

    file.write_all(json.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;

    println!("{}", style("✓ Context JSON written").green());
    println!("  Output file: {}", output);
    println!("  Entities exported: {}", entities.len());

    Ok(())
}
