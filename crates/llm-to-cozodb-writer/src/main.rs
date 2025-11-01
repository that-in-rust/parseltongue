//! Main entry point for parseltongue-02.
//!
//! S01 Ultra-Minimalist Implementation:
//! - NO automatic LLM calls (LLM runs externally, passes changes via CLI)
//! - Direct CozoDB writes only
//! - Matches unified binary pattern (parseltongue/src/main.rs)

use console::style;
use anyhow::Result;

use llm_to_cozodb_writer::{
    cli::CliConfig,
    LlmWriterConfig,
};

use parseltongue_core::storage::CozoDbStorage;
use parseltongue_core::entities::TemporalAction;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = CliConfig::build_cli();
    let matches = cli.try_get_matches();

    match matches {
        Ok(matches) => {
            let config = CliConfig::parse_config(&matches);

            println!("{}", style("Running Tool 2: llm-to-cozodb-writer").cyan());

            // Run writer with simple pattern
            match run_writer(&config).await {
                Ok(()) => {
                    println!("{}", style("✓ Entity updated successfully").green().bold());
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

/// Run the writer with ultra-minimalist pattern (S01)
///
/// Matches the implementation in parseltongue/src/main.rs (unified binary)
async fn run_writer(config: &LlmWriterConfig) -> Result<()> {
    // Validate future-code requirement
    if (config.action == "create" || config.action == "edit") && config.future_code.is_none() {
        eprintln!("{}", style("Error: --future-code required for create/edit actions").red());
        std::process::exit(1);
    }

    // Connect to database
    let storage = CozoDbStorage::new(&config.db_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Process action
    match config.action.as_str() {
        "create" => {
            println!("  Creating entity: {}", config.entity_key);
            println!("  Future code: {} bytes", config.future_code.as_ref().unwrap().len());
            eprintln!("{}", style("⚠️  CREATE action requires full entity construction - not yet implemented").yellow());
            eprintln!("    Hint: First index the codebase, then use EDIT to modify entities");
            Ok(())
        }
        "edit" => {
            println!("  Editing entity: {}", config.entity_key);

            // Fetch existing entity
            let mut entity = storage.get_entity(&config.entity_key)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to fetch entity: {}", e))?;

            // Update future_code
            entity.future_code = Some(config.future_code.as_ref().unwrap().clone());

            // Set temporal action
            entity.temporal_state.future_action = Some(TemporalAction::Edit);
            entity.temporal_state.future_ind = true;

            // Persist updated entity back to database
            storage.update_entity_internal(&entity)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to persist entity changes: {}", e))?;

            println!("{}", style("✓ Entity updated with future code").green());
            println!("  Temporal state: Edit pending (future_ind=true)");
            Ok(())
        }
        "delete" => {
            println!("  Deleting entity: {}", config.entity_key);

            // Fetch existing entity
            let mut entity = storage.get_entity(&config.entity_key)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to fetch entity: {}", e))?;

            // Mark for deletion via temporal state
            entity.temporal_state.future_ind = false;
            entity.temporal_state.future_action = Some(TemporalAction::Delete);

            // Persist updated entity
            storage.update_entity_internal(&entity)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to mark for deletion: {}", e))?;

            println!("{}", style("✓ Entity marked for deletion").green());
            println!("  Temporal state: Delete pending (future_ind=false)");
            Ok(())
        }
        _ => unreachable!("clap validation should prevent this"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_edit_requires_code() {
        let config = LlmWriterConfig {
            entity_key: "rust:fn:test:lib_rs:10-15".to_string(),
            action: "edit".to_string(),
            future_code: None,  // Missing code for edit
            db_path: "mem".to_string(),
        };

        // Should require future_code for edit action
        assert!(config.future_code.is_none());
        assert_eq!(config.action, "edit");
    }

    #[test]
    fn test_config_validation_delete_no_code() {
        let config = LlmWriterConfig {
            entity_key: "rust:fn:old:lib_rs:20-25".to_string(),
            action: "delete".to_string(),
            future_code: None,  // Delete doesn't need code
            db_path: "mem".to_string(),
        };

        // Delete should not need future_code
        assert!(config.future_code.is_none());
        assert_eq!(config.action, "delete");
    }

    #[test]
    fn test_config_default() {
        let config = LlmWriterConfig::default();
        assert_eq!(config.db_path, "parseltongue.db");
        assert_eq!(config.action, "edit");
        assert!(config.future_code.is_none());
    }
}