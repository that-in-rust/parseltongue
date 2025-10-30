use anyhow::Result;
use console::style;
use parseltongue_core::storage::CozoDbStorage;

mod cli;

use cozodb_make_future_code_current::StateResetManager;
use folder_to_cozodb_streamer::{streamer::FileStreamer, StreamerConfig, ToolFactory};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse_args();

    println!("\n{}", style("Parseltongue Tool 06: cozoDB-make-future-code-current").bold().cyan());
    println!("{}", style("Ultra-Minimalist State Reset Manager").dim());
    println!("{}", style("=".repeat(60)).dim());

    println!("\n{}", style("Configuration:").bold());
    println!("  Database: {}", cli.database.display());
    println!("  Project: {}", cli.project_path.display());

    println!("\n{}", style("Ultra-Minimalist Principles:").bold().yellow());
    println!("  {} NO BACKUP METADATA - Direct table deletion", style("✓").green());
    println!("  {} NO CONFIGURATION - Single deterministic operation", style("✓").green());
    println!("  {} NO ROLLBACK - Permanent state reset", style("✓").green());
    println!("  {} NO COMPLEXITY - Delete → Recreate → Re-index", style("✓").green());

    // Initialize CozoDB storage
    println!("\n{}", style("Initializing storage...").bold());
    let storage = CozoDbStorage::new(&format!("sqlite:{}", cli.database.display())).await?;
    if cli.verbose {
        println!("  {} Storage initialized", style("✓").green());
    }

    // Create state reset manager
    let manager = StateResetManager::new(storage);

    // Perform state reset
    println!("\n{}", style("Performing state reset...").bold().yellow());
    println!("  {} Deleting CodeGraph table", style("→").cyan());
    println!("  {} Recreating schema", style("→").cyan());

    let result = manager.reset(&cli.project_path).await?;

    // Display results
    println!("\n{}", style("Reset Complete!").bold().green());
    println!("  Success: {}", if result.success { style("✓").green() } else { style("✗").red() });
    println!("  Entities deleted: {}", result.entities_deleted);
    println!("  Schema recreated: {}", if result.schema_recreated { style("✓").green() } else { style("✗").red() });

    // PRD-compliant re-indexing (Tool 1 integration)
    if cli.reindex {
        println!("\n{}", style("Re-indexing project (Tool 1)...").bold().yellow());

        // Configure Tool 1 with same database
        let indexer_config = StreamerConfig {
            root_dir: cli.project_path.clone(),
            db_path: format!("rocksdb:{}", cli.database.display()),
            max_file_size: 1024 * 1024, // 1MB default
            include_patterns: vec!["*.rs".to_string()],
            exclude_patterns: vec!["target/**".to_string()],
            parsing_library: "tree-sitter".to_string(),
            chunking: "ISGL1".to_string(),
        };

        // Run Tool 1 streaming
        let streamer = ToolFactory::create_streamer(indexer_config).await?;
        let index_result = streamer.stream_directory().await?;

        if cli.verbose {
            println!("  {} Files processed: {}", style("✓").green(), index_result.processed_files);
            println!("  {} Entities created: {}", style("✓").green(), index_result.entities_created);
        }

        println!("\n{}", style("Complete Cycle Finished!").bold().green());
        println!("  {} Reset complete", style("✓").green());
        println!("  {} Re-indexing complete", style("✓").green());
        println!("  {} Ready for next iteration", style("✓").green());
    } else {
        println!("\n{}", style("Next Steps (Manual):").bold().yellow());
        println!("  1. Run Tool 1 (folder-to-cozodb-streamer) to re-index project");
        println!("  2. Run Tool 2 (LLM-to-cozoDB-writer) to generate Future_Code");
        println!("  3. Validate and write changes with Tools 4-5");
    }

    Ok(())
}
