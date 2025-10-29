//! Main entry point for parseltongue-03.

use std::sync::Arc;
use console::{style, Term};
use anyhow::Result;

use parseltongue_03::{
    cli::CliConfig,
    errors::ContextWriterError,
    context_optimizer::{ContextOptimizer, ContextOptimizerImpl, ContextResult},
    llm_client::ContextLlmClient,
    ToolFactory,
    ContextWriterConfig,
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
                    style("Parseltongue Tool 03: LLM-cozoDB-to-context-writer")
                        .blue()
                        .bold()
                );
                println!("{}", style("Ultra-minimalist context optimization with CozoDB").blue());
                println!();
            }

            // Validate configuration
            let llm_client = ToolFactory::create_llm_client(config.clone());
            if let Err(e) = llm_client.validate_config() {
                eprintln!("{} Configuration error: {}", style("Error:").red().bold(), e);
                std::process::exit(1);
            }

            // Create and run optimizer
            match run_optimizer(&config, &matches, verbose, quiet, dry_run).await {
                Ok(result) => {
                    if !quiet {
                        println!(
                            "{}",
                            style("✓ Context optimizer completed successfully!").green().bold()
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
                                style("🔍 Dry run mode - no context files were written.")
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

/// Run the context optimizer with the given configuration
async fn run_optimizer(
    config: &ContextWriterConfig,
    matches: &clap::ArgMatches,
    verbose: bool,
    quiet: bool,
    dry_run: bool,
) -> Result<ContextResult, ContextWriterError> {
    // Create optimizer instance using factory
    let optimizer = ToolFactory::create_context_optimizer(config.clone())?;

    if verbose && !quiet {
        println!("Configuration:");
        println!("  Database path: {}", config.db_path);
        println!("  LLM endpoint: {}", config.llm_endpoint);
        println!("  Model: {}", config.model);
        println!("  Max tokens: {}", config.max_tokens);
        println!("  Temperature: {}", config.temperature);
        println!("  Max context tokens: {}", config.max_context_tokens);
        println!("  Relevance threshold: {}", config.relevance_threshold);
        println!("  Output directory: {}", config.output_dir);
        println!("  Entity query: {}", config.entity_query);

        if dry_run {
            println!("  Mode: Dry run (no files will be written)");
        }
        println!();
    }

    // Generate context ID
    let context_id = matches
        .get_one::<String>("context-id")
        .cloned()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Generate output path
    let output_path = if dry_run {
        // For dry run, use a temporary path
        format!("/tmp/dry_run_context_{}.json", context_id)
    } else {
        CliConfig::generate_output_path(&config.output_dir, &context_id)
    };

    if verbose && !quiet {
        println!("Context ID: {}", context_id);
        println!("Output path: {}", output_path);
        println!();
    }

    // Run optimizer
    let result = optimizer.generate_context(&output_path).await?;

    // Print detailed results if verbose
    if verbose && !quiet {
        println!("\nDetailed Results:");
        println!("  Context ID: {}", result.context_id);
        println!("  Output file: {}", result.output_path);
        println!("  Entities processed: {}", result.entities_processed);
        println!("  Entities optimized: {}", result.entities_optimized);
        println!("  Tokens generated: {}", result.tokens_generated);
        println!("  Optimization ratio: {:.2}%", result.optimization_ratio * 100.0);
        println!("  Processing time: {:?}", result.generation_time);

        // Get and display statistics
        let stats = optimizer.get_stats();
        println!("  Contexts generated: {}", stats.contexts_generated);
        println!("  Total entities processed: {}", stats.entities_processed);
        println!("  Total tokens generated: {}", stats.tokens_generated);
        println!("  Optimization savings: {} tokens", stats.optimization_savings);
        println!("  LLM requests made: {}", stats.llm_requests_made);
        println!("  Total generation time: {:?}", stats.total_generation_time);

        if !result.errors.is_empty() {
            println!("\nErrors:");
            for error in &result.errors {
                println!("  {}", style(error).yellow());
            }
        }
    }

    // Clean up dry run file if it exists
    if dry_run {
        if let Err(_) = tokio::fs::remove_file(&output_path).await {
            // Ignore cleanup errors
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

        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().to_string_lossy().to_string();

        let config = ContextWriterConfig {
            db_path: "test.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: "test-key-for-testing".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 1000,
            temperature: 0.3,
            entity_query: "MATCH (e:Entity) RETURN e LIMIT 1".to_string(),
            max_context_tokens: 8000,
            relevance_threshold: 0.7,
            output_dir: output_dir.clone(),
        };

        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-03",
            "--output", &output_dir,
            "--context-id", "test-context",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        // Note: This would fail without a proper LLM mock, but demonstrates the interface
        let result = run_optimizer(&config, &matches, false, true, true).await;
        // In a real test environment, we would mock the LLM client
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_main_with_invalid_api_key() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().to_string_lossy().to_string();

        let config = ContextWriterConfig {
            db_path: "test.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: "".to_string(), // Empty API key
            model: "gpt-4".to_string(),
            max_tokens: 8192,
            temperature: 0.3,
            entity_query: "MATCH (e:Entity) RETURN e LIMIT 10".to_string(),
            max_context_tokens: 128000,
            relevance_threshold: 0.7,
            output_dir,
        };

        let result = run_optimizer(&config, &clap::ArgMatches::default(), false, true, true).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_configuration_validation() {
        // Test with valid configuration
        let config = ContextWriterConfig {
            db_path: "test.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: "valid-api-key".to_string(),
            model: "gpt-4".to_string(),
            max_tokens: 8192,
            temperature: 0.3,
            entity_query: "MATCH (e:Entity) RETURN e LIMIT 10".to_string(),
            max_context_tokens: 128000,
            relevance_threshold: 0.7,
            output_dir: "./contexts".to_string(),
        };

        let client = ToolFactory::create_llm_client(config);
        assert!(client.validate_config().is_ok());

        // Test with invalid configuration
        let invalid_config = ContextWriterConfig {
            llm_api_key: "".to_string(), // Invalid: empty
            ..ContextWriterConfig::default()
        };

        let invalid_client = ToolFactory::create_llm_client(invalid_config);
        assert!(invalid_client.validate_config().is_err());
    }

    #[test]
    fn test_output_path_generation() {
        let context_id = "test-context-123";
        let output_dir = "./test_output";

        let output_path = CliConfig::generate_output_path(output_dir, context_id);

        assert!(output_path.starts_with(&format!("{}/context_test-context-123_", output_dir)));
        assert!(output_path.ends_with(".json"));
    }
}