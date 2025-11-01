//! Command-line interface for parseltongue-02.
//!
//! # CLI Architecture
//!
//! This crate has two CLI modes:
//!
//! 1. **Unified Binary** (production): Defined in `parseltongue/src/main.rs`
//!    - Usage: `parseltongue llm-to-cozodb-writer --entity <key> --action <create|edit|delete> [--future-code <code>] [--db <path>]`
//!    - `--entity` and `--action` are required arguments
//!
//! 2. **Standalone Binary** (development): Defined in this file
//!    - Same CLI as unified binary (for consistency)
//!
//! ## Philosophy (S01 Ultra-Minimalist)
//!
//! Following ultra-minimalist principles:
//! - NO automatic LLM calls (LLM runs externally, passes changes via CLI)
//! - NO batch processing (process one entity at a time)
//! - NO dry-run mode (trust the input)
//! - Direct temporal state updates only
//!
//! ## Examples
//!
//! ```bash
//! # Edit an existing function
//! llm-to-cozodb-writer \
//!   --entity "rust:fn:hello:greeter_src_lib_rs:4-6" \
//!   --action edit \
//!   --future-code 'pub fn hello() -> &'static str { "Hello!" }' \
//!   --db rocksdb:demo.db
//!
//! # Create a new function
//! llm-to-cozodb-writer \
//!   --entity "rust:fn:goodbye:greeter_src_lib_rs:8-10" \
//!   --action create \
//!   --future-code 'pub fn goodbye() -> &'static str { "Goodbye!" }'
//!
//! # Delete a function
//! llm-to-cozodb-writer \
//!   --entity "rust:fn:old_func:lib_rs:20-25" \
//!   --action delete \
//!   --db rocksdb:demo.db
//! ```

use clap::{Arg, ArgGroup, Command};

use crate::{
    AdvancedQueryConfig, EntityAction, InterfaceMode, LlmWriterConfig, SimpleUpdateConfig,
};

/// CLI configuration builder
pub struct CliConfig;

impl CliConfig {
    /// Build CLI application with Progressive Disclosure pattern
    ///
    /// Two interfaces:
    /// 1. Simple (80% use cases): --entity --action --future-code
    /// 2. Advanced (20% power users): --query
    pub fn build_cli() -> Command {
        Command::new("parseltongue-02")
            .version("0.7.1")
            .author("Parseltongue Team")
            .about("Tool 02: LLM-to-cozoDB-writer")
            .long_about(
                "Ultra-minimalist temporal state writer for CozoDB.\n\
                \n\
                Two interfaces:\n\
                \n\
                1. Simple Interface (80% use cases):\n  \
                llm-to-cozodb-writer --entity <KEY> --action <create|edit|delete> --future-code <CODE>\n\
                \n\
                2. Advanced Interface (20% power users):\n  \
                llm-to-cozodb-writer --query \"?[...] := [[...]] :put CodeGraph {...}\"\n\
                ",
            )
            // Simple interface arguments
            .arg(
                Arg::new("entity")
                    .long("entity")
                    .value_name("ISGL1_KEY")
                    .help("ISGL1 entity key to modify")
                    .conflicts_with("query"),
            )
            .arg(
                Arg::new("action")
                    .long("action")
                    .value_name("ACTION")
                    .help("Action to perform: create, edit, or delete")
                    .value_parser(["create", "edit", "delete"])
                    .conflicts_with("query"),
            )
            .arg(
                Arg::new("future-code")
                    .long("future-code")
                    .value_name("CODE")
                    .help("Future code content (required for create/edit)")
                    .conflicts_with("query"),
            )
            // Advanced interface arguments
            .arg(
                Arg::new("query")
                    .long("query")
                    .value_name("DATALOG")
                    .help("Raw Datalog query to execute")
                    .conflicts_with("entity"),
            )
            // Common argument
            .arg(
                Arg::new("database")
                    .long("db")
                    .value_name("PATH")
                    .help("Database file path")
                    .default_value("parseltongue.db"),
            )
            // Mutual exclusion groups
            .group(
                ArgGroup::new("interface")
                    .args(&["query", "entity"])
                    .required(true),
            )
    }

    /// Parse CLI arguments into LlmWriterConfig (S01 ultra-minimalist)
    ///
    /// Deprecated: Use parse_interface_mode() instead
    pub fn parse_config(matches: &clap::ArgMatches) -> LlmWriterConfig {
        LlmWriterConfig {
            query: matches.get_one::<String>("query").unwrap().clone(),
            db_path: matches.get_one::<String>("database").unwrap().clone(),
        }
    }

    /// Parse CLI arguments into InterfaceMode (Progressive Disclosure pattern)
    ///
    /// Determines whether user is using Simple or Advanced interface
    pub fn parse_interface_mode(matches: &clap::ArgMatches) -> InterfaceMode {
        let db_path = matches
            .get_one::<String>("database")
            .unwrap()
            .clone();

        // Check which interface mode
        if let Some(entity_key) = matches.get_one::<String>("entity") {
            // Simple Interface
            let action_str = matches.get_one::<String>("action").unwrap();
            let action = match action_str.as_str() {
                "create" => EntityAction::Create,
                "edit" => EntityAction::Edit,
                "delete" => EntityAction::Delete,
                _ => unreachable!("clap validates this"),
            };

            let future_code = matches.get_one::<String>("future-code").cloned();

            InterfaceMode::Simple(SimpleUpdateConfig {
                entity_key: entity_key.clone(),
                action,
                future_code,
                db_path,
            })
        } else {
            // Advanced Interface
            let query = matches.get_one::<String>("query").unwrap().clone();
            InterfaceMode::Advanced(AdvancedQueryConfig { query, db_path })
        }
    }

    /// Print usage information
    pub fn print_usage() {
        let mut cli = Self::build_cli();
        cli.print_help().unwrap();
        println!();
    }

    /// Print version information
    pub fn print_version() {
        println!("parseltongue-02 version 0.7.1");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_config_parsing() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-02",
            "--query",
            "?[a] := [[1]]",
            "--db",
            "test.db",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.query, "?[a] := [[1]]");
        assert_eq!(config.db_path, "test.db");
    }

    #[test]
    fn test_default_db_path() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-02",
            "--query",
            "?[b] := [[2]]",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.query, "?[b] := [[2]]");
        assert_eq!(config.db_path, "parseltongue.db"); // Default value
    }
}