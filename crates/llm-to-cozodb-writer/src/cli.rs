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

use clap::{Arg, Command};

use crate::LlmWriterConfig;

/// CLI configuration builder
pub struct CliConfig;

impl CliConfig {
    /// Build CLI application
    pub fn build_cli() -> Command {
        Command::new("parseltongue-02")
            .version("0.7.1")
            .author("Parseltongue Team")
            .about("Tool 02: LLM-to-cozoDB-writer")
            .long_about(
                "Ultra-minimalist tool for writing temporal code changes to CozoDB.\n\
                \n\
                Examples:\n  \
                llm-to-cozodb-writer --entity \"rust:fn:hello:lib_rs:4-6\" --action edit --future-code 'pub fn hello() {}'\n  \
                llm-to-cozodb-writer --entity \"rust:fn:new_func:lib_rs:10-15\" --action create --future-code 'pub fn new_func() {}'\n  \
                llm-to-cozodb-writer --entity \"rust:fn:old_func:lib_rs:20-25\" --action delete --db rocksdb:demo.db",
            )
            .arg(
                Arg::new("entity")
                    .long("entity")
                    .value_name("ISGL1_KEY")
                    .help("ISGL1 key of entity (e.g., 'rust:fn:hello:lib_rs:4-6')")
                    .required(true),
            )
            .arg(
                Arg::new("action")
                    .long("action")
                    .value_name("ACTION")
                    .help("Temporal action type")
                    .value_parser(["create", "edit", "delete"])
                    .required(true),
            )
            .arg(
                Arg::new("future-code")
                    .long("future-code")
                    .value_name("CODE")
                    .help("Future code content (required for create/edit actions)"),
            )
            .arg(
                Arg::new("database")
                    .long("db")
                    .value_name("PATH")
                    .help("Database file path")
                    .default_value("parseltongue.db"),
            )
    }

    /// Parse CLI arguments into LlmWriterConfig
    ///
    /// Maps CLI arguments to config structure with hardcoded defaults for internal fields.
    pub fn parse_config(matches: &clap::ArgMatches) -> LlmWriterConfig {
        LlmWriterConfig {
            entity_key: matches.get_one::<String>("entity").unwrap().clone(),
            action: matches.get_one::<String>("action").unwrap().clone(),
            future_code: matches.get_one::<String>("future-code").cloned(),
            db_path: matches.get_one::<String>("database").unwrap().clone(),
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
            "--entity",
            "rust:fn:hello:lib_rs:4-6",
            "--action",
            "edit",
            "--future-code",
            "pub fn hello() -> &'static str { \"Hello!\" }",
            "--db",
            "test.db",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.entity_key, "rust:fn:hello:lib_rs:4-6");
        assert_eq!(config.action, "edit");
        assert_eq!(
            config.future_code,
            Some("pub fn hello() -> &'static str { \"Hello!\" }".to_string())
        );
        assert_eq!(config.db_path, "test.db");
    }

    #[test]
    fn test_default_config() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-02",
            "--entity",
            "rust:fn:test:lib_rs:10-15",
            "--action",
            "create",
            "--future-code",
            "pub fn test() {}",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.entity_key, "rust:fn:test:lib_rs:10-15");
        assert_eq!(config.action, "create");
        assert_eq!(config.future_code, Some("pub fn test() {}".to_string()));
        assert_eq!(config.db_path, "parseltongue.db"); // Default value
    }

    #[test]
    fn test_delete_action_without_code() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-02",
            "--entity",
            "rust:fn:old_func:lib_rs:20-25",
            "--action",
            "delete",
            "--db",
            "test.db",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.entity_key, "rust:fn:old_func:lib_rs:20-25");
        assert_eq!(config.action, "delete");
        assert_eq!(config.future_code, None); // No code needed for delete
        assert_eq!(config.db_path, "test.db");
    }
}