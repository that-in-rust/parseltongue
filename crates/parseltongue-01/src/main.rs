//! Main entry point for parseltongue-01.

use console::style;
use anyhow::Result;

use parseltongue_01::{
    cli::CliConfig,
    errors::StreamerError,
    streamer::FileStreamer,
    ToolFactory,
    StreamerConfig,
};

#[tokio::main]
async fn main() -> Result<()> {

    // Parse CLI arguments
    let cli = CliConfig::build_cli();
    let matches = cli.try_get_matches();

    match matches {
        Ok(matches) => {
            let config = CliConfig::parse_config(&matches);

            // Handle quiet/verbose flags
            let quiet = matches.get_flag("quiet");
            let verbose = matches.get_flag("verbose");

            if !quiet {
                println!(
                    "{}",
                    style("Parseltongue Tool 01: folder-to-cozoDB-streamer")
                        .blue()
                        .bold()
                );
                println!("{}", style("Ultra-minimalist code streaming to CozoDB").blue());
                println!();
            }

            // Create and run streamer
            match run_streamer(&config, verbose, quiet).await {
                Ok(result) => {
                    if !quiet {
                        println!(
                            "{}",
                            style("✓ Streaming completed successfully!").green().bold()
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

/// Run the file streamer with the given configuration
async fn run_streamer(
    config: &StreamerConfig,
    verbose: bool,
    quiet: bool,
) -> Result<parseltongue_01::StreamResult, StreamerError> {
    // Create streamer instance using factory (now async)
    let streamer = ToolFactory::create_streamer(config.clone()).await?;

    if verbose && !quiet {
        println!("Configuration:");
        println!("  Root directory: {}", config.root_dir.display());
        println!("  Database path: {}", config.db_path);
        println!("  Max file size: {} bytes", config.max_file_size);
        println!("  Include patterns: {:?}", config.include_patterns);
        println!("  Exclude patterns: {:?}", config.exclude_patterns);
        println!();
    }

    // Run streaming
    let result = streamer.stream_directory().await?;

    // Print detailed results if verbose
    if verbose && !quiet {
        println!("\nDetailed Results:");
        println!("  Files scanned: {}", result.total_files);
        println!("  Files processed: {}", result.processed_files);
        println!("  Entities created: {}", result.entities_created);
        println!("  Processing time: {:?}", result.duration);

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

    #[tokio::test]
    async fn test_main_with_valid_directory() {
        // Create temporary directory with test files
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test.rs");
        std::fs::write(
            &test_file_path,
            r#"fn test_function() {
    println!("Hello, world!");
}
"#,
        )
        .unwrap();

        // Verify file was created
        assert!(test_file_path.exists(), "Test file should exist");

        let config = StreamerConfig {
            root_dir: temp_dir.path().to_path_buf(),
            db_path: "mem".to_string(), // Use in-memory database for tests
            max_file_size: 1024 * 1024,
            include_patterns: vec!["*.rs".to_string()], // Simplified pattern
            exclude_patterns: vec![],
        };

        let result = run_streamer(&config, false, true).await;
        assert!(result.is_ok());

        // Verify entities were actually created
        let stream_result = result.unwrap();
        assert!(stream_result.total_files > 0, "Should have found at least one file");
        assert!(stream_result.entities_created > 0, "Should have created at least one entity");
    }

    #[tokio::test]
    async fn test_main_with_empty_directory() {
        let temp_dir = TempDir::new().unwrap();

        let config = StreamerConfig {
            root_dir: temp_dir.path().to_path_buf(),
            db_path: "mem".to_string(), // Use in-memory database for tests
            max_file_size: 1024 * 1024,
            include_patterns: vec!["**/*.rs".to_string()],
            exclude_patterns: vec![],
        };

        let result = run_streamer(&config, false, true).await;
        assert!(result.is_ok());

        let stream_result = result.unwrap();
        assert_eq!(stream_result.total_files, 0);
        assert_eq!(stream_result.processed_files, 0);
        assert_eq!(stream_result.entities_created, 0);
    }
}