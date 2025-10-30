use anyhow::Result;
use console::style;
use parseltongue_core::storage::CozoDbStorage;

mod cli;

use cozodb_make_future_code_current::StateResetManager;

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

    println!("\n{}", style("Next Steps:").bold().yellow());
    println!("  1. Run Tool 1 (parseltongue-01) to re-index project");
    println!("  2. Run Tool 2 (parseltongue-02) to generate Future_Code");
    println!("  3. Validate and write changes with Tools 4-5");

    Ok(())
}
