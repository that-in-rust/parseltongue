use anyhow::Result;
use console::style;

mod cli;

use parseltongue_04::{CodeValidator, DefaultRustValidator, ValidationType};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse_args();
    cli.validate()?;

    // Get code to validate
    let code = if let Some(snippet) = &cli.code_snippet {
        snippet.clone()
    } else if let Some(file_path) = &cli.file {
        std::fs::read_to_string(file_path)?
    } else {
        unreachable!("CLI validation ensures one of these is present")
    };

    // Create validator
    let validator = DefaultRustValidator::new();

    // Determine validation types to run
    let validation_types = match cli.validation_type {
        cli::ValidationTypeArg::All => ValidationType::all(),
        cli::ValidationTypeArg::Syntax => vec![ValidationType::Syntax],
        cli::ValidationTypeArg::Type => vec![ValidationType::Type],
        cli::ValidationTypeArg::BorrowChecker => vec![ValidationType::BorrowChecker],
        cli::ValidationTypeArg::Compilation => vec![ValidationType::Compilation],
        cli::ValidationTypeArg::Test => vec![ValidationType::Test],
    };

    // Run validation
    let report = validator.validate_specific(&code, validation_types).await?;

    // Output results
    match cli.output_format {
        cli::OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&report)?;
            println!("{}", json);
        }
        cli::OutputFormat::Text => {
            println!("\n{}", style("Parseltongue Tool 04: Rust Preflight Code Simulator").bold().cyan());
            println!("{}", style("=".repeat(60)).dim());

            if report.overall_valid {
                println!("{} Validation passed!", style("✓").green().bold());
            } else {
                println!("{} Validation failed!", style("✗").red().bold());
            }

            println!("\n{}", style("Validation Results:").bold());
            for result in &report.individual_results {
                let status = if result.is_valid {
                    style("PASS").green()
                } else {
                    style("FAIL").red()
                };
                println!("  [{:?}] {} ({}ms)", result.validation_type, status, result.execution_time_ms);

                if !result.errors.is_empty() && cli.verbose {
                    for error in &result.errors {
                        println!("    {}", style(error).red());
                    }
                }
            }

            println!("\n{}", style("Summary:").bold());
            println!("  Total time: {}ms", report.total_execution_time_ms);
            println!("  Memory usage: {} bytes", report.total_memory_usage_bytes);

            if !report.all_errors().is_empty() {
                println!("\n{}", style("Errors:").red().bold());
                for error in report.all_errors() {
                    println!("  - {}", error);
                }
            }

            if !report.all_warnings().is_empty() && cli.verbose {
                println!("\n{}", style("Warnings:").yellow().bold());
                for warning in report.all_warnings() {
                    println!("  - {}", warning);
                }
            }
        }
    }

    // Exit with appropriate code
    if report.overall_valid {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
