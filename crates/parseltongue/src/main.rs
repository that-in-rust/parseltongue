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
use pt01_folder_to_cozodb_streamer::streamer::FileStreamer;

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
                .long_about(
                    "Examples:\n  \
                    parseltongue folder-to-cozodb-streamer .            # Index current directory\n  \
                    parseltongue folder-to-cozodb-streamer ./src --db rocksdb:analysis.db --verbose"
                )
                .arg(
                    Arg::new("directory")
                        .help("Directory to index [default: current directory]")
                        .default_value(".")
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

    // Create config (S01 ultra-minimalist: let tree-sitter decide what to parse)
    let config = pt01_folder_to_cozodb_streamer::StreamerConfig {
        root_dir: std::path::PathBuf::from(directory),
        db_path: db.clone(),
        max_file_size: 100 * 1024 * 1024,  // 100MB - no artificial limits
        include_patterns: vec!["*".to_string()],  // ALL files - tree-sitter handles it
        exclude_patterns: vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build".to_string(),
            "dist".to_string(),
            "__pycache__".to_string(),
            ".venv".to_string(),
            "venv".to_string(),
        ],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    // Create and run streamer
    let streamer = pt01_folder_to_cozodb_streamer::ToolFactory::create_streamer(config.clone()).await?;
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
    use parseltongue_core::entities::TemporalAction;

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
    use parseltongue_core::storage::CozoDbStorage;
    use pt04_syntax_preflight_validator::SimpleSyntaxValidator;

    let db = matches.get_one::<String>("db").unwrap();
    let verbose = matches.get_flag("verbose");

    println!("{}", style("Running Tool 4: rust-preflight-code-simulator").cyan());
    println!("  Database: {}", db);

    // Connect to database
    let storage = CozoDbStorage::new(db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Fetch changed entities (those with future_action set)
    let entities = storage.get_changed_entities().await?;

    if entities.is_empty() {
        println!("{}", style("ℹ No entities with pending changes found").yellow());
        return Ok(());
    }

    println!("  Validating {} changed entities...", entities.len());

    // Create syntax validator
    let mut validator = SimpleSyntaxValidator::new()
        .map_err(|e| anyhow::anyhow!("Failed to create validator: {}", e))?;

    let mut total_validated = 0;
    let mut total_errors = 0;
    let mut validation_details = Vec::new();

    // Validate each entity's future_code
    for entity in &entities {
        if let Some(future_code) = &entity.future_code {
            total_validated += 1;

            let result = validator.validate_syntax(future_code)
                .map_err(|e| anyhow::anyhow!("Validation failed for {}: {}", entity.isgl1_key, e))?;

            if !result.is_valid {
                total_errors += 1;

                if verbose {
                    eprintln!("{} {}", style("✗").red(), entity.isgl1_key);
                    for error in &result.errors {
                        eprintln!("  {}", style(error).red());
                    }
                }

                validation_details.push((entity.isgl1_key.clone(), result.errors));
            } else if verbose {
                println!("{} {}", style("✓").green(), entity.isgl1_key);
            }
        }
    }

    // Print summary
    println!();
    if total_errors == 0 {
        println!("{}", style("✓ All syntax validations passed").green().bold());
        println!("  Entities validated: {}", total_validated);
    } else {
        eprintln!("{}", style("✗ Syntax validation failed").red().bold());
        eprintln!("  Entities validated: {}", total_validated);
        eprintln!("  Entities with errors: {}", total_errors);

        if !verbose {
            eprintln!();
            eprintln!("Failed entities:");
            for (key, errors) in &validation_details {
                eprintln!("  {} {}", style("✗").red(), key);
                for error in errors {
                    eprintln!("    {}", error);
                }
            }
        }

        return Err(anyhow::anyhow!("Syntax validation failed for {} entities", total_errors));
    }

    Ok(())
}

async fn run_llm_cozodb_to_diff_writer(matches: &ArgMatches) -> Result<()> {
    use parseltongue_core::storage::CozoDbStorage;
    use pt05_llm_cozodb_to_diff_writer::DiffGenerator;
    use std::sync::Arc;

    let output = matches.get_one::<String>("output").unwrap();
    let db = matches.get_one::<String>("db").unwrap();

    println!("{}", style("Running Tool 5: llm-cozodb-to-diff-writer").cyan());
    println!("  Database: {}", db);
    println!("  Output: {}", output);

    // Connect to database
    let storage = Arc::new(
        CozoDbStorage::new(db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?
    );

    // Create diff generator with dependency injection
    let generator = DiffGenerator::new(storage);

    // Generate CodeDiff from changed entities
    let diff = generator.generate_diff()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to generate diff: {}", e))?;

    if diff.changes.is_empty() {
        println!("{}", style("ℹ No changes found in database").yellow());
        return Ok(());
    }

    // Serialize to JSON
    let json = diff.to_json_pretty()
        .map_err(|e| anyhow::anyhow!("Failed to serialize diff to JSON: {}", e))?;

    // Write to file
    std::fs::write(output, json)
        .map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;

    println!("{}", style("✓ CodeDiff.json generated").green());
    println!("  Output file: {}", output);
    println!("  Changes included: {}", diff.changes.len());

    // Print summary by operation
    let mut creates = 0;
    let mut edits = 0;
    let mut deletes = 0;
    for change in &diff.changes {
        match change.operation {
            pt05_llm_cozodb_to_diff_writer::Operation::Create => creates += 1,
            pt05_llm_cozodb_to_diff_writer::Operation::Edit => edits += 1,
            pt05_llm_cozodb_to_diff_writer::Operation::Delete => deletes += 1,
        }
    }
    println!("    Creates: {}", creates);
    println!("    Edits: {}", edits);
    println!("    Deletes: {}", deletes);

    Ok(())
}

async fn run_cozodb_make_future_code_current(matches: &ArgMatches) -> Result<()> {
    use parseltongue_core::storage::CozoDbStorage;
    use pt06_cozodb_make_future_code_current::StateResetManager;
    use std::path::Path;

    let project = matches.get_one::<String>("project").unwrap();
    let db = matches.get_one::<String>("db").unwrap();

    println!("{}", style("Running Tool 6: cozodb-make-future-code-current").cyan());
    println!("  Project: {}", project);
    println!("  Database: {}", db);

    // Connect to database
    let storage = CozoDbStorage::new(db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Create state reset manager
    let reset_manager = StateResetManager::new(storage);

    // Reset database state (delete all entities, recreate schema)
    let result = reset_manager.reset(Path::new(project))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to reset database state: {}", e))?;

    println!("{}", style("✓ Database reset completed").green().bold());
    println!("  Entities deleted: {}", result.entities_deleted);
    println!("  Schema recreated: {}", if result.schema_recreated { "yes" } else { "no" });
    println!();
    println!("{}", style("Next step: Re-index the codebase").cyan());
    println!("  Run: parseltongue folder-to-cozodb-streamer {} --db {}", project, db);

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
