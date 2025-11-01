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
        Some(("index", sub_matches)) => {
            run_index(sub_matches).await
        }
        Some(("write", sub_matches)) => {
            run_write(sub_matches).await
        }
        Some(("read", sub_matches)) => {
            run_read(sub_matches).await
        }
        Some(("check", sub_matches)) => {
            run_check(sub_matches).await
        }
        Some(("diff", sub_matches)) => {
            run_diff(sub_matches).await
        }
        Some(("reset", sub_matches)) => {
            run_reset(sub_matches).await
        }
        _ => {
            println!("{}", style("Parseltongue CLI Toolkit").blue().bold());
            println!("{}", style("Ultra-minimalist code analysis and modification toolkit").blue());
            println!();
            println!("Use --help for more information");
            println!();
            println!("Available commands:");
            println!("  index   - Index codebase into CozoDB (Tool 1)");
            println!("  write   - Write LLM changes to temporal state (Tool 2)");
            println!("  read    - Generate context from CozoDB (Tool 3)");
            println!("  check   - Validate syntax of proposed changes (Tool 4)");
            println!("  diff    - Generate CodeDiff.json (Tool 5)");
            println!("  reset   - Reset database state (Tool 6)");
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
            Command::new("index")
                .about("Index codebase into CozoDB (Tool 1: folder-to-cozodb-streamer)")
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
            Command::new("write")
                .about("Write LLM changes to CozoDB (Tool 2: llm-to-cozodb-writer)")
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
            Command::new("read")
                .about("Generate context from CozoDB (Tool 3: llm-cozodb-to-context-writer)")
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
            Command::new("check")
                .about("Validate syntax (Tool 4: rust-preflight-code-simulator)")
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
            Command::new("diff")
                .about("Generate CodeDiff.json (Tool 5: llm-cozodb-to-diff-writer)")
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
            Command::new("reset")
                .about("Reset database state (Tool 6: cozodb-make-future-code-current)")
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

async fn run_index(matches: &ArgMatches) -> Result<()> {
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

async fn run_write(matches: &ArgMatches) -> Result<()> {
    let entity = matches.get_one::<String>("entity").unwrap();
    let action = matches.get_one::<String>("action").unwrap();
    let future_code = matches.get_one::<String>("future-code");
    let db = matches.get_one::<String>("db").unwrap();

    println!("{}", style("Running Tool 2: llm-to-cozodb-writer").cyan());

    // Validate future-code requirement
    if (action == "create" || action == "edit") && future_code.is_none() {
        eprintln!("{}", style("Error: --future-code required for create/edit actions").red());
        std::process::exit(1);
    }

    // TODO: Call llm-to-cozodb-writer library function
    // For now, print what would be done
    println!("  Entity: {}", entity);
    println!("  Action: {}", action);
    if let Some(code) = future_code {
        println!("  Future code: {} bytes", code.len());
    }
    println!("  Database: {}", db);

    println!("{}", style("✓ Write completed (placeholder)").green());
    println!("⚠️  Tool 2 integration pending - see issue tracker");

    Ok(())
}

async fn run_read(matches: &ArgMatches) -> Result<()> {
    let output = matches.get_one::<String>("output").unwrap();
    let db = matches.get_one::<String>("db").unwrap();
    let filter = matches.get_one::<String>("filter").unwrap();

    println!("{}", style("Running Tool 3: llm-cozodb-to-context-writer").cyan());

    // TODO: Call llm-cozodb-to-context-writer library function
    println!("  Database: {}", db);
    println!("  Filter: {}", filter);
    println!("  Output: {}", output);

    println!("{}", style("✓ Context generated (placeholder)").green());
    println!("⚠️  Tool 3 integration pending - see issue tracker");

    Ok(())
}

async fn run_check(matches: &ArgMatches) -> Result<()> {
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

async fn run_diff(matches: &ArgMatches) -> Result<()> {
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

async fn run_reset(matches: &ArgMatches) -> Result<()> {
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
        // Verify all subcommands are present
        let subcommands: Vec<&str> = cli.get_subcommands().map(|cmd| cmd.get_name()).collect();
        assert!(subcommands.contains(&"index"));
        assert!(subcommands.contains(&"write"));
        assert!(subcommands.contains(&"read"));
        assert!(subcommands.contains(&"check"));
        assert!(subcommands.contains(&"diff"));
        assert!(subcommands.contains(&"reset"));
    }
}
