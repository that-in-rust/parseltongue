//! Command-line interface for parseltongue-01.
//!
//! # CLI Architecture
//!
//! This crate has two CLI modes:
//!
//! 1. **Unified Binary** (production): Defined in `parseltongue/src/main.rs`
//!    - Usage: `parseltongue folder-to-cozodb-streamer <directory> [--db <path>] [--verbose] [--quiet]`
//!    - `<directory>` is a required positional argument
//!
//! 2. **Standalone Binary** (development): Defined in this file
//!    - Same CLI as unified binary (for consistency)
//!    - Internal fields (max_file_size, include_patterns, etc.) use hardcoded defaults
//!
//! ## Philosophy (S01 Ultra-Minimalist)
//!
//! Following ultra-minimalist principles:
//! - NO unused arguments (removed: --parsing-library, --chunking, --max-size, --include, --exclude)
//! - NO configuration complexity
//! - Hardcoded sensible defaults matching unified binary

use clap::{Arg, Command};
use std::path::PathBuf;

use crate::StreamerConfig;

/// CLI configuration builder
pub struct CliConfig;

impl CliConfig {
    /// Build CLI application
    pub fn build_cli() -> Command {
        Command::new("parseltongue-01")
            .version("0.7.0")
            .author("Parseltongue Team")
            .about("Tool 01: folder-to-cozoDB-streamer")
            .long_about(
                "Ultra-minimalist streaming tool that reads code files from a directory,\n\
                generates ISGL1 keys using tree-sitter, and stores them in CozoDB.\n\
                \n\
                Following TDD-first principles with executable specifications.",
            )
            .arg(
                Arg::new("directory")
                    .help("Directory to index")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("database")
                    .long("db")
                    .value_name("PATH")
                    .help("Database connection string (use 'mem' for in-memory)")
                    .default_value("mem"),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enable verbose output")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .help("Suppress output except errors")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("verbose"),
            )
    }

    /// Parse CLI arguments into StreamerConfig
    ///
    /// Uses hardcoded defaults for internal fields (matching unified binary behavior):
    /// - max_file_size: 100MB (ultra-minimalist: let tree-sitter decide what to parse)
    /// - include_patterns: ALL files (tree-sitter handles unsupported files gracefully)
    /// - exclude_patterns: Common build/dependency dirs only
    /// - parsing_library: "tree-sitter"
    /// - chunking: "ISGL1"
    pub fn parse_config(matches: &clap::ArgMatches) -> StreamerConfig {
        StreamerConfig {
            root_dir: PathBuf::from(matches.get_one::<String>("directory").unwrap()),
            db_path: matches.get_one::<String>("database").unwrap().clone(),
            // Hardcoded defaults (S01 ultra-minimalist - NO artificial limits)
            max_file_size: 100 * 1024 * 1024,  // 100MB - let tree-sitter decide
            include_patterns: vec!["*".to_string()],  // ALL files - tree-sitter handles it
            exclude_patterns: vec![
                "target".to_string(),      // Rust build
                "node_modules".to_string(), // Node.js dependencies
                ".git".to_string(),        // Git metadata
                "build".to_string(),       // Generic build dir
                "dist".to_string(),        // Distribution files
                "__pycache__".to_string(), // Python cache
                ".venv".to_string(),       // Python virtual env
                "venv".to_string(),        // Python virtual env
            ],
            parsing_library: "tree-sitter".to_string(),
            chunking: "ISGL1".to_string(),
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
        println!("parseltongue-01 version 0.7.0");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_config_parsing() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-01",
            "/test/dir",  // Positional argument (matches unified binary)
            "--db",
            "test.db",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        // Check CLI arguments
        assert_eq!(config.root_dir, PathBuf::from("/test/dir"));
        assert_eq!(config.db_path, "test.db");

        // Check hardcoded defaults (S01 ultra-minimalist - NO artificial limits)
        assert_eq!(config.max_file_size, 100 * 1024 * 1024);  // 100MB
        assert_eq!(config.include_patterns, vec!["*".to_string()]);  // ALL files
        assert!(config.exclude_patterns.contains(&"target".to_string()));
        assert!(config.exclude_patterns.contains(&"node_modules".to_string()));
        assert_eq!(config.parsing_library, "tree-sitter");
        assert_eq!(config.chunking, "ISGL1");
    }

    #[test]
    fn test_default_config() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-01",
            ".",  // Directory is now required (positional argument)
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        // Check CLI defaults
        assert_eq!(config.root_dir, PathBuf::from("."));
        assert_eq!(config.db_path, "mem");

        // Check hardcoded defaults (S01 ultra-minimalist - NO artificial limits)
        assert_eq!(config.max_file_size, 100 * 1024 * 1024);  // 100MB
        assert_eq!(config.include_patterns, vec!["*".to_string()]);  // ALL files
        assert!(config.exclude_patterns.contains(&"target".to_string()));
        assert!(config.exclude_patterns.contains(&"node_modules".to_string()));
        assert_eq!(config.parsing_library, "tree-sitter");
        assert_eq!(config.chunking, "ISGL1");
    }

    #[test]
    fn test_prd_command_format() {
        // Test ultra-minimalist CLI (S01 principle)
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "folder-to-cozoDB-streamer",
            "./src",  // Positional argument (matches unified binary)
            "--db",
            "./parseltongue.db",
        ]);

        assert!(matches.is_ok(), "Ultra-minimalist command should be valid");
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        // Check CLI arguments
        assert_eq!(config.root_dir, PathBuf::from("./src"));
        assert_eq!(config.db_path, "./parseltongue.db");

        // Check hardcoded defaults (S01 ultra-minimalist)
        assert_eq!(config.parsing_library, "tree-sitter");
        assert_eq!(config.chunking, "ISGL1");
    }
}