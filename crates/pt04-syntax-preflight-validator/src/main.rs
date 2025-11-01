//! # Simplified Tool 4: Syntax Validator CLI
//!
//! Validates entities with future_code from CozoDB using tree-sitter syntax checking only.

use anyhow::{Context, Result};
use clap::Parser;
use console::style;
use parseltongue_core::storage::CozoDbStorage;
use pt04_syntax_preflight_validator::SimpleSyntaxValidator;

#[derive(Parser)]
#[command(name = "rust-preflight-code-simulator")]
#[command(about = "Simplified syntax validation for entities with future_code")]
struct Cli {
    /// Path to CozoDB database
    #[arg(long, default_value = "mem")]
    database: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "\n{}",
        style("Parseltongue Tool 04: Simplified Syntax Validator")
            .bold()
            .cyan()
    );
    println!("{}", style("=".repeat(60)).dim());

    // Connect to CozoDB
    let storage = CozoDbStorage::new(&cli.database)
        .await
        .context("Failed to connect to CozoDB")?;

    // Get entities with future_code (Create or Edit operations)
    let changed_entities = storage
        .get_changed_entities()
        .await
        .context("Failed to get changed entities from CozoDB")?;

    if changed_entities.is_empty() {
        println!(
            "{}",
            style("No entities with future_code found. Nothing to validate.")
                .yellow()
        );
        return Ok(());
    }

    println!(
        "\n{} entities with future_code found",
        style(changed_entities.len()).bold()
    );

    // Create validator
    let mut validator =
        SimpleSyntaxValidator::new().context("Failed to create syntax validator")?;

    // Validate each entity
    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut error_details = Vec::new();

    for entity in &changed_entities {
        if let Some(future_code) = &entity.future_code {
            match validator.validate_syntax(future_code) {
                Ok(result) => {
                    if result.is_valid {
                        valid_count += 1;
                        if cli.verbose {
                            println!("  {} {}", style("✓").green(), entity.isgl1_key);
                        }
                    } else {
                        invalid_count += 1;
                        println!("  {} {}", style("✗").red(), entity.isgl1_key);
                        for error in &result.errors {
                            println!("    {}", style(error).red().dim());
                            error_details.push((entity.isgl1_key.clone(), error.clone()));
                        }
                    }
                }
                Err(e) => {
                    invalid_count += 1;
                    println!(
                        "  {} {} - Validation error: {}",
                        style("✗").red(),
                        entity.isgl1_key,
                        style(e).red()
                    );
                }
            }
        }
    }

    // Print summary
    println!("\n{}", style("Summary:").bold());
    println!("  Total entities: {}", changed_entities.len());
    println!("  {} Valid syntax: {}", style("✓").green(), valid_count);
    println!("  {} Invalid syntax: {}", style("✗").red(), invalid_count);

    if invalid_count > 0 {
        println!(
            "\n{}",
            style("Syntax validation failed. Fix errors and retry.")
                .red()
                .bold()
        );
        std::process::exit(1);
    } else {
        println!(
            "\n{}",
            style("✅ All syntax checks passed! Ready for file writes (Tool 5).")
                .green()
                .bold()
        );
        Ok(())
    }
}
