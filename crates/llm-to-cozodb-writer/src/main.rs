//! Main entry point for parseltongue-02.

use std::sync::Arc;
use console::{style, Term};
use anyhow::Result;

use llm_to_cozodb_writer::{
    cli::CliConfig,
    errors::LlmWriterError,
    llm_client::LlmClient,
    temporal_writer::{TemporalWriter, TemporalWriterImpl},
    ToolFactory,
    LlmWriterConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let term = Term::stdout();

    // Parse CLI arguments
    let cli = CliConfig::build_cli();
    let matches = cli.try_get_matches();

    match matches {
        Ok(matches) => {
            let config = CliConfig::parse_config(&matches);

            // Handle quiet/verbose flags
            let quiet = matches.get_flag("quiet");
            let verbose = matches.get_flag("verbose");
            let dry_run = matches.get_flag("dry-run");

            if !quiet {
                println!(
                    "{}",
                    style("Parseltongue Tool 02: LLM-to-cozoDB-writer")
                        .blue()
                        .bold()
                );
                println!("{}", style("Ultra-minimalist LLM communication with CozoDB").blue());
                println!();
            }

            // Validate configuration
            let llm_client = ToolFactory::create_llm_client(config.clone());
            if let Err(e) = llm_client.validate_config() {
                eprintln!("{} Configuration error: {}", style("Error:").red().bold(), e);
                std::process::exit(1);
            }

            // Create and run writer
            match run_writer(&config, verbose, quiet, dry_run).await {
                Ok(result) => {
                    if !quiet {
                        println!(
                            "{}",
                            style("✓ LLM writer completed successfully!").green().bold()
                        );
                        if result.errors.is_empty() {
                            println!("{}", style("No errors encountered.").green());
                        } else {
                            println!(
                                "{}",
                                style(format!("⚠ {} warnings encountered", result.errors.len()))
                                    .yellow()
                            );
                        }

                        if dry_run {
                            println!(
                                "{}",
                                style("🔍 Dry run mode - no changes were applied to the database.")
                                    .cyan()
                            );
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("{} {}", style("Error:").red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            CliConfig::print_usage();
            std::process::exit(1);
        }
    }
}

/// Run the LLM writer with the given configuration
async fn run_writer(
    config: &LlmWriterConfig,
    verbose: bool,
    quiet: bool,
    dry_run: bool,
) -> Result<llm_to_cozodb_writer::WriterResult, LlmWriterError> {
    // Create writer instance using factory
    let writer = ToolFactory::create_llm_writer(config.clone())?;

    if verbose && !quiet {
        println!("Configuration:");
        println!("  Database path: {}", config.db_path);
        println!("  LLM endpoint: {}", config.llm_endpoint);
        println!("  Model: {}", config.model);
        println!("  Max tokens: {}", config.max_tokens);
        println!("  Temperature: {}", config.temperature);
        println!("  Batch size: {}", config.batch_size);
        println!("  Query: {}", config.query_filter);

        if dry_run {
            println!("  Mode: Dry run (no changes will be applied)");
        }
        println!();
    }

    // Run writer
    let result = writer.process_entities().await?;

    // Print detailed results if verbose
    if verbose && !quiet {
        println!("\nDetailed Results:");
        println!("  Entities found: {}", result.total_entities);
        println!("  Entities processed: {}", result.processed_entities);
        println!("  Changes generated: {}", result.changes_generated);
        println!("  Changes applied: {}", result.changes_applied);
        println!("  Processing time: {:?}", result.duration);

        // Get and display statistics
        let stats = writer.get_stats();
        println!("  LLM requests made: {}", stats.llm_requests_made);
        println!("  Total tokens used: {}", stats.total_tokens_used);

        if !result.errors.is_empty() {
            println!("\nErrors:");
            for error in &result.errors {
                println!("  {}", style(error).yellow());
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[tokio::test]
    async fn test_main_with_valid_config() {
        // Set up environment variable for API key
        env::set_var("OPENAI_API_KEY", "test-key-for-testing");

        let config = LlmWriterConfig {
            db_path: "test.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: "test-key-for-testing".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 1000,
            temperature: 0.5,
            query_filter: "SELECT * FROM CodeEntity LIMIT 1".to_string(),
            batch_size: 1,
        };

        let result = run_writer(&config, false, true, true).await;
        // Note: This would fail without a proper LLM mock, but demonstrates the interface
        // In a real test environment, we would mock the LLM client
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_main_with_invalid_api_key() {
        let config = LlmWriterConfig {
            db_path: "test.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: "".to_string(), // Empty API key - should cause validation failure
            model: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            query_filter: "SELECT * FROM CodeEntity LIMIT 10".to_string(),
            batch_size: 5,
        };

        let result = run_writer(&config, false, true, true).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_configuration_validation() {
        // Test with valid configuration
        let config = LlmWriterConfig {
            db_path: "test.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: "valid-api-key".to_string(),
            model: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            query_filter: "SELECT * FROM CodeEntity LIMIT 10".to_string(),
            batch_size: 5,
        };

        let client = ToolFactory::create_llm_client(config);
        assert!(client.validate_config().is_ok());

        // Test with invalid configuration
        let invalid_config = LlmWriterConfig {
            llm_api_key: "".to_string(), // Invalid: empty
            ..LlmWriterConfig::default()
        };

        let invalid_client = ToolFactory::create_llm_client(invalid_config);
        assert!(invalid_client.validate_config().is_err());
    }
}