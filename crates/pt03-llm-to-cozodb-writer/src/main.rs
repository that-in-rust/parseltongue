//! Main entry point for parseltongue-02.
//!
//! S01 Ultra-Minimalist Implementation with Progressive Disclosure:
//! - Simple Interface (80%): --entity --action --future-code
//! - Advanced Interface (20%): --query (raw Datalog)
//! - NO automatic LLM calls (LLM runs externally, passes changes via CLI)
//! - Direct CozoDB writes only

use anyhow::Result;
use console::style;

use pt03_llm_to_cozodb_writer::{cli::CliConfig, InterfaceMode};

use parseltongue_core::storage::CozoDbStorage;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = CliConfig::build_cli();
    let matches = cli.try_get_matches();

    match matches {
        Ok(matches) => {
            // Parse interface mode (Simple or Advanced)
            let mode = CliConfig::parse_interface_mode(&matches);

            println!(
                "{}",
                style("Running Tool 2: llm-to-cozodb-writer").cyan()
            );

            // Execute based on interface mode
            match run_writer(mode).await {
                Ok(()) => {
                    println!(
                        "{}",
                        style("✓ Entity updated successfully").green().bold()
                    );
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

/// Run the writer with Progressive Disclosure pattern (S01)
///
/// Supports two interfaces:
/// - Simple: Generates Datalog from --entity --action --future-code
/// - Advanced: Executes raw --query Datalog
///
/// NO validation, NO safety checks - trust the user (S01 principle)
async fn run_writer(mode: InterfaceMode) -> Result<()> {
    // Extract database path and query based on interface mode
    let (db_path, query) = match &mode {
        InterfaceMode::Simple(config) => {
            println!("  Using Simple Interface (Create/Edit/Delete)");
            println!("  Entity: {}", config.entity_key);
            println!("  Action: {:?}", config.action);
            let query = config.to_datalog();
            (&config.db_path, query)
        }
        InterfaceMode::Advanced(config) => {
            println!("  Using Advanced Interface (Raw Datalog)");
            (&config.db_path, config.query.clone())
        }
    };

    // Connect to database
    let storage = CozoDbStorage::new(db_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    println!("  Executing Datalog query...");

    // Execute Datalog query via CozoDB (S01: trust the user)
    storage
        .execute_query(&query)
        .await
        .map_err(|e| anyhow::anyhow!("Query execution failed: {}", e))?;

    println!(
        "{}",
        style("✓ Datalog query executed successfully").green()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pt03_llm_to_cozodb_writer::{AdvancedQueryConfig, EntityAction, SimpleUpdateConfig};

    #[test]
    fn test_simple_interface_mode() {
        let config = SimpleUpdateConfig {
            entity_key: "rust:fn:test:lib_rs:1-5".to_string(),
            action: EntityAction::Edit,
            future_code: Some("fn test() {}".to_string()),
            db_path: "test.db".to_string(),
        };

        let mode = InterfaceMode::Simple(config.clone());

        match mode {
            InterfaceMode::Simple(c) => {
                assert_eq!(c.entity_key, "rust:fn:test:lib_rs:1-5");
                assert_eq!(c.db_path, "test.db");
            }
            _ => panic!("Expected Simple mode"),
        }
    }

    #[test]
    fn test_advanced_interface_mode() {
        let config = AdvancedQueryConfig {
            query: "?[a] := [[1]]".to_string(),
            db_path: "test.db".to_string(),
        };

        let mode = InterfaceMode::Advanced(config);

        match mode {
            InterfaceMode::Advanced(c) => {
                assert_eq!(c.query, "?[a] := [[1]]");
                assert_eq!(c.db_path, "test.db");
            }
            _ => panic!("Expected Advanced mode"),
        }
    }
}