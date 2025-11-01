//! # Tool 5 CLI: CodeDiff.json Generator
//!
//! Generates CodeDiff.json from CozoDB for LLM consumption.

use anyhow::{Context, Result};
use clap::Parser;
use console::style;
use pt05_llm_cozodb_to_diff_writer::DiffGenerator;
use parseltongue_core::storage::CozoDbStorage;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "llm-cozodb-to-diff-writer")]
#[command(about = "Generates CodeDiff.json from CozoDB for LLM consumption")]
struct Cli {
    /// Path to CozoDB database
    #[arg(long, default_value = "./parseltongue.db")]
    database: String,

    /// Output path for CodeDiff.json
    #[arg(long, default_value = "./CodeDiff.json")]
    output: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "\n{}",
        style("Parseltongue Tool 05: LLM-cozoDB-to-diff-writer")
            .bold()
            .cyan()
    );
    println!("{}", style("CodeDiff.json Generator").dim());
    println!("{}", style("=".repeat(60)).dim());

    // Connect to CozoDB
    let storage = CozoDbStorage::new(&cli.database)
        .await
        .context("Failed to connect to CozoDB")?;

    if cli.verbose {
        println!("\n{}", style("Configuration:").bold());
        println!("  Database: {}", cli.database);
        println!("  Output: {}", cli.output.display());
    }

    // Generate diff (with dependency injection)
    let storage = Arc::new(storage);
    let generator = DiffGenerator::new(storage);
    let diff = generator
        .generate_diff()
        .await
        .context("Failed to generate CodeDiff")?;

    // Display summary
    println!("\n{}", style("Summary:").bold());
    println!("  Total changes: {}", diff.metadata.total_changes);
    println!("  {} Create: {}", style("➕").green(), diff.metadata.create_count);
    println!("  {} Edit: {}", style("✏️ ").yellow(), diff.metadata.edit_count);
    println!("  {} Delete: {}", style("🗑️ ").red(), diff.metadata.delete_count);

    if cli.verbose && !diff.changes.is_empty() {
        println!("\n{}", style("Changes:").bold());
        for change in &diff.changes {
            let icon = match change.operation {
                pt05_llm_cozodb_to_diff_writer::Operation::Create => style("➕").green(),
                pt05_llm_cozodb_to_diff_writer::Operation::Edit => style("✏️ ").yellow(),
                pt05_llm_cozodb_to_diff_writer::Operation::Delete => style("🗑️ ").red(),
            };
            println!("  {} {}", icon, change.file_path.display());
            if cli.verbose {
                println!("     ISGL1: {}", change.isgl1_key);
            }
        }
    }

    // Write JSON to file
    let json = diff
        .to_json_pretty()
        .context("Failed to serialize CodeDiff")?;
    std::fs::write(&cli.output, json).context("Failed to write CodeDiff.json")?;

    println!(
        "\n{}",
        style(format!("✅ CodeDiff.json written to: {}", cli.output.display()))
            .green()
            .bold()
    );

    println!("\n{}", style("Next Steps:").bold().cyan());
    println!("  1. LLM reads CodeDiff.json");
    println!("  2. LLM applies changes to codebase files");
    println!("  3. Run cargo build to verify compilation");
    println!("  4. Run cargo test to verify functionality");

    Ok(())
}
