//! Parseltongue: Unified CLI toolkit for code analysis and modification
//!
//! This binary provides 6 subcommands that dispatch to the individual tools:
//! - index:  folder-to-cozodb-streamer (Tool 1)
//! - write:  llm-to-cozodb-writer (Tool 2)
//! - read:   llm-cozodb-to-context-writer (Tool 3)
//! - check:  rust-preflight-code-simulator (Tool 4)
//! - diff:   llm-cozodb-to-diff-writer (Tool 5)
//! - reset:  cozodb-make-future-code-current (Tool 6)

use clap::{Arg, ArgMatches, Command};
use console::style;
use anyhow::Result;

// Import traits to enable trait methods
use folder_to_cozodb_streamer::streamer::FileStreamer;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("folder-to-cozodb-streamer", sub_matches)) => {
            run_folder_to_cozodb_streamer(sub_matches).await
        }
        Some(("llm-to-cozodb-writer", sub_matches)) => {
            run_llm_to_cozodb_writer(sub_matches).await
        }
        Some(("llm-cozodb-to-context-writer", sub_matches)) => {
            run_llm_cozodb_to_context_writer(sub_matches).await
        }
        Some(("rust-preflight-code-simulator", sub_matches)) => {
            run_rust_preflight_code_simulator(sub_matches).await
        }
        Some(("llm-cozodb-to-diff-writer", sub_matches)) => {
            run_llm_cozodb_to_diff_writer(sub_matches).await
        }
        Some(("cozodb-make-future-code-current", sub_matches)) => {
            run_cozodb_make_future_code_current(sub_matches).await
        }
        _ => {
            println!("{}", style("Parseltongue CLI Toolkit").blue().bold());
            println!("{}", style("Ultra-minimalist code analysis and modification toolkit").blue());
            println!();
            println!("Use --help for more information");
            println!();
            println!("Available commands:");
            println!("  folder-to-cozodb-streamer        - Index codebase into CozoDB (Tool 1)");
            println!("  llm-to-cozodb-writer             - Write LLM changes to temporal state (Tool 2)");
            println!("  llm-cozodb-to-context-writer     - Generate context from CozoDB (Tool 3)");
            println!("  rust-preflight-code-simulator    - Validate syntax of proposed changes (Tool 4)");
            println!("  llm-cozodb-to-diff-writer        - Generate CodeDiff.json (Tool 5)");
            println!("  cozodb-make-future-code-current  - Reset database state (Tool 6)");
            Ok(())
        }
    }
}

fn build_cli() -> Command {
    Command::new("parseltongue")
        .version("1.0.0")
        .author("Parseltongue Team")
        .about("Ultra-minimalist CLI toolkit for code analysis and modification")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("folder-to-cozodb-streamer")
                .about("Tool 1: Stream folder contents to CozoDB with ISGL1 keys")
                .arg(
                    Arg::new("directory")
                        .help("Directory to index")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("db")
                        .long("db")
                        .help("Database file path")
                        .default_value("parseltongue.db"),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Enable verbose output")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("quiet")
                        .long("quiet")
                        .short('q')
                        .help("Suppress output")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("llm-to-cozodb-writer")
                .about("Tool 2: Write LLM-proposed changes to temporal state")
                .arg(
                    Arg::new("entity")
                        .long("entity")
                        .help("ISGL1 key of entity")
                        .required(true),
                )
                .arg(
                    Arg::new("action")
                        .long("action")
                        .help("Action type: create, edit, or delete")
                        .value_parser(["create", "edit", "delete"])
                        .required(true),
                )
                .arg(
                    Arg::new("future-code")
                        .long("future-code")
                        .help("Future code content (required for create/edit)"),
                )
                .arg(
                    Arg::new("db")
                        .long("db")
                        .help("Database file path")
                        .default_value("parseltongue.db"),
                ),
        )
        .subcommand(
            Command::new("llm-cozodb-to-context-writer")
                .about("Tool 3: Generate context JSON from CozoDB for LLM consumption")
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Output JSON file")
                        .required(true),
                )
                .arg(
                    Arg::new("db")
                        .long("db")
                        .help("Database file path")
                        .default_value("parseltongue.db"),
                )
                .arg(
                    Arg::new("filter")
                        .long("filter")
                        .help("Filter: all, changed, or current")
                        .value_parser(["all", "changed", "current"])
                        .default_value("all"),
                ),
        )
        .subcommand(
            Command::new("rust-preflight-code-simulator")
                .about("Tool 4: Validate syntax and simulate code execution")
                .arg(
                    Arg::new("db")
                        .long("db")
                        .help("Database file path")
                        .default_value("parseltongue.db"),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Show detailed errors")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("llm-cozodb-to-diff-writer")
                .about("Tool 5: Generate CodeDiff.json from temporal state")
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Output JSON file")
                        .required(true),
                )
                .arg(
                    Arg::new("db")
                        .long("db")
                        .help("Database file path")
                        .default_value("parseltongue.db"),
                ),
        )
        .subcommand(
            Command::new("cozodb-make-future-code-current")
                .about("Tool 6: Make future code current and reset temporal state")
                .arg(
                    Arg::new("project")
                        .long("project")
                        .help("Project root directory")
                        .required(true),
                )
                .arg(
                    Arg::new("db")
                        .long("db")
                        .help("Database file path")
                        .default_value("parseltongue.db"),
                ),
        )
}

async fn run_folder_to_cozodb_streamer(matches: &ArgMatches) -> Result<()> {
    let directory = matches.get_one::<String>("directory").unwrap();
    let db = matches.get_one::<String>("db").unwrap();
    let verbose = matches.get_flag("verbose");
    let quiet = matches.get_flag("quiet");

    println!("{}", style("Running Tool 1: folder-to-cozodb-streamer").cyan());

    // Create config
    let config = folder_to_cozodb_streamer::StreamerConfig {
        root_dir: std::path::PathBuf::from(directory),
        db_path: db.clone(),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],  // Simplified pattern that works with current matcher
        exclude_patterns: vec!["target".to_string()],  // Simplified exclude pattern
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    // Create and run streamer
    let streamer = folder_to_cozodb_streamer::ToolFactory::create_streamer(config.clone()).await?;
    let result = streamer.stream_directory().await?;

    if !quiet {
        println!("{}", style("✓ Indexing completed").green().bold());
        println!("  Files processed: {}", result.processed_files);
        println!("  Entities created: {}", result.entities_created);
        if verbose {
            println!("  Duration: {:?}", result.duration);
        }
    }

    Ok(())
}

async fn run_llm_to_cozodb_writer(matches: &ArgMatches) -> Result<()> {
    use parseltongue_core::storage::CozoDbStorage;
    use parseltongue_core::entities::{TemporalAction, TemporalState};

    let entity_key = matches.get_one::<String>("entity").unwrap();
    let action = matches.get_one::<String>("action").unwrap();
    let future_code = matches.get_one::<String>("future-code");
    let db = matches.get_one::<String>("db").unwrap();

    println!("{}", style("Running Tool 2: llm-to-cozodb-writer").cyan());

    // Validate future-code requirement
    if (action == "create" || action == "edit") && future_code.is_none() {
        eprintln!("{}", style("Error: --future-code required for create/edit actions").red());
        std::process::exit(1);
    }

    // Connect to database
    let storage = CozoDbStorage::new(db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Process action
    match action.as_str() {
        "create" => {
            println!("  Creating entity: {}", entity_key);
            println!("  Future code: {} bytes", future_code.unwrap().len());
            // For MVP: creating new entity requires full entity data structure
            // This is a simplified implementation - in practice, you'd parse the ISGL1 key
            // and construct a proper CodeEntity
            eprintln!("{}", style("⚠️  CREATE action requires full entity construction - not yet implemented").yellow());
            eprintln!("    Hint: First index the codebase, then use EDIT to modify entities");
        }
        "edit" => {
            println!("  Editing entity: {}", entity_key);

            // Fetch existing entity
            let mut entity = storage.get_entity(entity_key)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to fetch entity: {}", e))?;

            // Update future_code
            entity.future_code = Some(future_code.unwrap().clone());

            // Set temporal action
            entity.temporal_state.future_action = Some(TemporalAction::Edit);
            entity.temporal_state.future_ind = true;

            // Persist updated entity back to database
            storage.update_entity_internal(&entity)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to persist entity changes: {}", e))?;

            println!("{}", style("✓ Entity updated with future code").green());
            println!("  Temporal state: Edit pending (future_ind=true)");
        }
        "delete" => {
            println!("  Deleting entity: {}", entity_key);

            // Fetch existing entity
            let mut entity = storage.get_entity(entity_key)
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
        }
        _ => unreachable!("clap validation should prevent this"),
    }

    Ok(())
}

async fn run_llm_cozodb_to_context_writer(matches: &ArgMatches) -> Result<()> {
    use parseltongue_core::storage::CozoDbStorage;
    use std::io::Write;

    let output = matches.get_one::<String>("output").unwrap();
    let db = matches.get_one::<String>("db").unwrap();
    let filter = matches.get_one::<String>("filter").unwrap();

    println!("{}", style("Running Tool 3: llm-cozodb-to-context-writer").cyan());
    println!("  Database: {}", db);
    println!("  Filter: {}", filter);
    println!("  Output: {}", output);

    // Connect to database
    let storage = CozoDbStorage::new(db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Fetch entities based on filter
    let entities = match filter.as_str() {
        "all" => storage.get_all_entities().await?,
        "changed" => storage.get_changed_entities().await?,
        "current" => {
            // For MVP: "current" means entities where current_ind=true
            storage.get_all_entities().await?
                .into_iter()
                .filter(|e| e.temporal_state.current_ind)
                .collect()
        }
        _ => unreachable!("clap validation should prevent this"),
    };

    println!("  Found {} entities", entities.len());

    // Write to JSON file
    let json = serde_json::to_string_pretty(&entities)
        .map_err(|e| anyhow::anyhow!("Failed to serialize entities: {}", e))?;

    let mut file = std::fs::File::create(output)
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))?;

    file.write_all(json.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;

    println!("{}", style("✓ Context JSON written").green());
    println!("  Output file: {}", output);
    println!("  Entities exported: {}", entities.len());

    Ok(())
}

async fn run_rust_preflight_code_simulator(matches: &ArgMatches) -> Result<()> {
    let db = matches.get_one::<String>("db").unwrap();
    let verbose = matches.get_flag("verbose");

    println!("{}", style("Running Tool 4: rust-preflight-code-simulator").cyan());

    // TODO: Call rust-preflight-code-simulator library function
    println!("  Database: {}", db);
    if verbose {
        println!("  Verbose: enabled");
    }

    println!("{}", style("✓ Validation passed (placeholder)").green());
    println!("⚠️  Tool 4 integration pending - see issue tracker");

    Ok(())
}

async fn run_llm_cozodb_to_diff_writer(matches: &ArgMatches) -> Result<()> {
    let output = matches.get_one::<String>("output").unwrap();
    let db = matches.get_one::<String>("db").unwrap();

    println!("{}", style("Running Tool 5: llm-cozodb-to-diff-writer").cyan());

    // TODO: Call llm-cozodb-to-diff-writer library function
    println!("  Database: {}", db);
    println!("  Output: {}", output);

    println!("{}", style("✓ Diff generated (placeholder)").green());
    println!("⚠️  Tool 5 integration pending - see issue tracker");

    Ok(())
}

async fn run_cozodb_make_future_code_current(matches: &ArgMatches) -> Result<()> {
    let project = matches.get_one::<String>("project").unwrap();
    let db = matches.get_one::<String>("db").unwrap();

    println!("{}", style("Running Tool 6: cozodb-make-future-code-current").cyan());

    // TODO: Call cozodb-make-future-code-current library function
    println!("  Project: {}", project);
    println!("  Database: {}", db);

    println!("{}", style("✓ Reset completed (placeholder)").green());
    println!("⚠️  Tool 6 integration pending - see issue tracker");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_builds() {
        let cli = build_cli();
        // Verify all subcommands are present with crate names
        let subcommands: Vec<&str> = cli.get_subcommands().map(|cmd| cmd.get_name()).collect();
        assert!(subcommands.contains(&"folder-to-cozodb-streamer"));
        assert!(subcommands.contains(&"llm-to-cozodb-writer"));
        assert!(subcommands.contains(&"llm-cozodb-to-context-writer"));
        assert!(subcommands.contains(&"rust-preflight-code-simulator"));
        assert!(subcommands.contains(&"llm-cozodb-to-diff-writer"));
        assert!(subcommands.contains(&"cozodb-make-future-code-current"));
    }
}
