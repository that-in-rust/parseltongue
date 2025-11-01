use anyhow::Result;
use console::style;
use parseltongue_core::storage::CozoDbStorage;

mod cli;

use pt06_cozodb_make_future_code_current::StateResetManager;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse_args();

    println!("\n{}", style("Parseltongue Tool 06: cozoDB-make-future-code-current").bold().cyan());
    println!("{}", style("Ultra-Minimalist State Reset Manager").dim());
    println!("{}", style("=".repeat(60)).dim());

    println!("\n{}", style("Configuration:").bold());
    println!("  Database: {}", cli.database);
    println!("  Project: {}", cli.project_path.display());

    println!("\n{}", style("Ultra-Minimalist Principles:").bold().yellow());
    println!("  {} NO BACKUP METADATA - Direct table deletion", style("✓").green());
    println!("  {} NO CONFIGURATION - Single deterministic operation", style("✓").green());
    println!("  {} NO ROLLBACK - Permanent state reset", style("✓").green());
    println!("  {} NO COMPLEXITY - Delete → Recreate → Re-index", style("✓").green());

    // Initialize CozoDB storage
    println!("\n{}", style("Initializing storage...").bold());
    // Accept database backend prefix from CLI (rocksdb: or sqlite:)
    let storage = CozoDbStorage::new(&cli.database).await?;
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

    // PRD-compliant re-indexing (call Tool 1 directly - NO config duplication)
    if cli.reindex {
        println!("\n{}", style("Re-indexing project (Tool 1)...").bold().yellow());

        // S01 Ultra-minimalist: Just call pt01 binary with same DB
        // NO config duplication - Tool 1 owns its own configuration
        let pt01_status = std::process::Command::new("parseltongue")
            .arg("pt01-folder-to-cozodb-streamer")
            .arg(&cli.project_path)
            .arg("--db")
            .arg(&cli.database)
            .arg(if cli.verbose { "--verbose" } else { "--quiet" })
            .status()?;

        if !pt01_status.success() {
            eprintln!("{}", style("✗ Re-indexing failed").red().bold());
            std::process::exit(1);
        }

        println!("\n{}", style("Complete Cycle Finished!").bold().green());
        println!("  {} Reset complete", style("✓").green());
        println!("  {} Re-indexing complete", style("✓").green());
        println!("  {} Ready for next iteration", style("✓").green());
    } else {
        println!("\n{}", style("Next Steps (Manual):").bold().yellow());
        println!("  1. Run: parseltongue pt01-folder-to-cozodb-streamer {} --db {}",
                 cli.project_path.display(), cli.database);
        println!("  2. Continue with workflow...");
    }

    Ok(())
}
