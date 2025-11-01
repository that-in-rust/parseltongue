//! Parseltongue: Unified CLI toolkit for code analysis and modification
//!
//! This binary provides 6 subcommands that dispatch to the individual tools:
//! - pt01-folder-to-cozodb-streamer (Tool 1: Ingest)
//! - pt02-llm-cozodb-to-context-writer (Tool 2: Read)
//! - pt03-llm-to-cozodb-writer (Tool 3: Edit/Write)
//! - pt04-syntax-preflight-validator (Tool 4: Validate)
//! - pt05-llm-cozodb-to-diff-writer (Tool 5: Diff)
//! - pt06-cozodb-make-future-code-current (Tool 6: Reset)

use clap::{Arg, ArgMatches, Command};
use console::style;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::collections::HashMap;

// Import traits to enable trait methods
use pt01_folder_to_cozodb_streamer::streamer::FileStreamer;

// Import core types for entity construction
use parseltongue_core::entities::{
    CodeEntity, TemporalState, InterfaceSignature, EntityType, Visibility,
    LineRange, Language, LanguageSpecificSignature, RustSignature,
    TddClassification, EntityClass, TestabilityLevel, ComplexityLevel, RiskLevel,
    EntityMetadata,
};

/// Build a new CodeEntity for CREATE action
///
/// # Functional Composition Pattern (S01 Philosophy)
/// Pure function that constructs CodeEntity from minimal inputs:
/// - ISGL1 key parsing (filepath-filename-interface)
/// - Auto-generation of required fields with sensible defaults
/// - Temporal state initialization for CREATE (0,1,Create)
///
/// # Arguments
/// * `isgl1_key` - Entity key in format: "filepath-filename-interface" or full ISGL1 format
/// * `future_code` - Code content for the new entity
///
/// # Returns
/// * `Result<CodeEntity>` - Constructed entity ready for insertion
fn build_create_entity(isgl1_key: &str, future_code: String) -> Result<CodeEntity> {
    // Parse file path and entity name from ISGL1 key
    let (file_path, entity_name, language) = parse_isgl1_key_components(isgl1_key)?;

    // Calculate hash before consuming future_code
    let content_hash = calculate_hash(&future_code);
    let now = chrono::Utc::now();

    // Construct entity using functional composition
    let entity = CodeEntity {
        isgl1_key: isgl1_key.to_string(),
        temporal_state: TemporalState::create(),
        interface_signature: InterfaceSignature {
            entity_type: EntityType::Function, // Default to Function
            name: entity_name,
            visibility: Visibility::Public,
            file_path,
            line_range: LineRange::new(1, 1)?, // Placeholder until actual insertion
            module_path: vec![],
            documentation: None,
            language_specific: build_default_language_signature(language),
        },
        current_code: None, // CREATE action means no current code
        future_code: Some(future_code),
        tdd_classification: TddClassification {
            entity_class: EntityClass::CodeImplementation,
            testability: TestabilityLevel::Medium,
            complexity: ComplexityLevel::Simple,
            dependencies: 0,
            test_coverage_estimate: 0.0,
            critical_path: false,
            change_risk: RiskLevel::Low,
        },
        lsp_metadata: None,
        metadata: EntityMetadata {
            created_at: now,
            modified_at: now,
            content_hash,
            additional: HashMap::new(),
        },
    };

    Ok(entity)
}

/// Parse ISGL1 key into components (pure function)
fn parse_isgl1_key_components(key: &str) -> Result<(PathBuf, String, Language)> {
    // Support both simple "filepath-filename-interface" and full ISGL1 formats
    let parts: Vec<&str> = key.split(':').collect();

    let (file_str, name) = if parts.len() >= 4 {
        // Full format: "rust:fn:name:file_path:start-end"
        (parts[3], parts[2].to_string())
    } else {
        // Simple format: "filepath-filename-interface" - parse backwards
        let segments: Vec<&str> = key.rsplitn(3, '-').collect();
        if segments.len() < 2 {
            anyhow::bail!("Invalid ISGL1 key format. Expected 'filepath-filename-interface' or 'lang:type:name:path:range'");
        }
        let interface = segments[0];
        let filepath = segments[2];
        (filepath, interface.to_string())
    };

    let file_path = PathBuf::from(file_str);

    // Infer language from file extension
    let language = Language::from_file_path(&file_path)
        .unwrap_or(Language::Rust); // Default to Rust if cannot infer

    Ok((file_path, name, language))
}

/// Build default language-specific signature (pure function)
fn build_default_language_signature(language: Language) -> LanguageSpecificSignature {
    match language {
        Language::Rust => LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec![],
            trait_impl: None,
        }),
        _ => LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec![],
            trait_impl: None,
        }), // Default to Rust signature for now
    }
}

/// Calculate content hash (pure function)
fn calculate_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("pt01-folder-to-cozodb-streamer", sub_matches)) => {
            run_folder_to_cozodb_streamer(sub_matches).await
        }
        Some(("pt02-llm-cozodb-to-context-writer", sub_matches)) => {
            run_llm_cozodb_to_context_writer(sub_matches).await
        }
        Some(("pt03-llm-to-cozodb-writer", sub_matches)) => {
            run_llm_to_cozodb_writer(sub_matches).await
        }
        Some(("pt04-syntax-preflight-validator", sub_matches)) => {
            run_rust_preflight_code_simulator(sub_matches).await
        }
        Some(("pt05-llm-cozodb-to-diff-writer", sub_matches)) => {
            run_llm_cozodb_to_diff_writer(sub_matches).await
        }
        Some(("pt06-cozodb-make-future-code-current", sub_matches)) => {
            run_cozodb_make_future_code_current(sub_matches).await
        }
        _ => {
            println!("{}", style("Parseltongue CLI Toolkit").blue().bold());
            println!("{}", style("Ultra-minimalist code analysis and modification toolkit").blue());
            println!();
            println!("Use --help for more information");
            println!();
            println!("Available commands:");
            println!("  pt01-folder-to-cozodb-streamer       - Index codebase into CozoDB (Tool 1: Ingest)");
            println!("  pt02-llm-cozodb-to-context-writer    - Generate context from CozoDB (Tool 2: Read)");
            println!("  pt03-llm-to-cozodb-writer            - Write LLM changes to temporal state (Tool 3: Edit)");
            println!("  pt04-syntax-preflight-validator      - Validate syntax of proposed changes (Tool 4: Validate)");
            println!("  pt05-llm-cozodb-to-diff-writer       - Generate CodeDiff.json (Tool 5: Diff)");
            println!("  pt06-cozodb-make-future-code-current - Reset database state (Tool 6: Reset)");
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
            Command::new("pt01-folder-to-cozodb-streamer")
                .about("Tool 1: Stream folder contents to CozoDB with ISGL1 keys")
                .long_about(
                    "Examples:\n  \
                    parseltongue pt01-folder-to-cozodb-streamer .            # Index current directory\n  \
                    parseltongue pt01-folder-to-cozodb-streamer ./src --db rocksdb:analysis.db --verbose"
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
            Command::new("pt02-llm-cozodb-to-context-writer")
                .about("Tool 2: Generate context JSON from CozoDB for LLM consumption")
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
            Command::new("pt03-llm-to-cozodb-writer")
                .about("Tool 3: Write LLM-proposed changes to temporal state")
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
            Command::new("pt04-syntax-preflight-validator")
                .about("Tool 4: Validate syntax of proposed changes")
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
            Command::new("pt05-llm-cozodb-to-diff-writer")
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
            Command::new("pt06-cozodb-make-future-code-current")
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

    println!("{}", style("Running Tool 3: pt03-llm-to-cozodb-writer").cyan());

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
            let future_code_content = future_code.unwrap().clone();
            println!("  Future code: {} bytes", future_code_content.len());

            // Build new entity using functional composition
            let entity = build_create_entity(&entity_key, future_code_content)
                .with_context(|| format!("Failed to construct entity from key: {}", entity_key))?;

            // Persist to database
            storage.insert_entity(&entity)
                .await
                .with_context(|| "Failed to insert new entity into database")?;

            println!("{}", style("✓ Entity created successfully").green());
            println!("  Temporal state: Create pending (current_ind=false, future_ind=true)");
            println!("  Entity type: {:?}", entity.interface_signature.entity_type);
            println!("  File path: {}", entity.interface_signature.file_path.display());
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

    println!("{}", style("Running Tool 2: pt02-llm-cozodb-to-context-writer").cyan());
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

    println!("{}", style("Running Tool 4: pt04-syntax-preflight-validator").cyan());
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

    println!("{}", style("Running Tool 5: pt05-llm-cozodb-to-diff-writer").cyan());
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

    println!("{}", style("Running Tool 6: pt06-cozodb-make-future-code-current").cyan());
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
    println!("  Run: parseltongue pt01-folder-to-cozodb-streamer {} --db {}", project, db);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_builds() {
        let cli = build_cli();
        // Verify all subcommands are present with pt01-pt06 prefixes (PRDv2)
        let subcommands: Vec<&str> = cli.get_subcommands().map(|cmd| cmd.get_name()).collect();
        assert!(subcommands.contains(&"pt01-folder-to-cozodb-streamer"));
        assert!(subcommands.contains(&"pt02-llm-cozodb-to-context-writer"));
        assert!(subcommands.contains(&"pt03-llm-to-cozodb-writer"));
        assert!(subcommands.contains(&"pt04-syntax-preflight-validator"));
        assert!(subcommands.contains(&"pt05-llm-cozodb-to-diff-writer"));
        assert!(subcommands.contains(&"pt06-cozodb-make-future-code-current"));
    }
}
