//! Command-line interface for parseltongue-01.

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
                    .short('d')
                    .long("dir")
                    .value_name("DIR")
                    .help("Root directory to scan for code files")
                    .default_value("."),
            )
            .arg(
                Arg::new("database")
                    .short('b')
                    .long("db")
                    .value_name("PATH")
                    .help("Database file path")
                    .default_value("parseltongue.db"),
            )
            .arg(
                Arg::new("max-size")
                    .short('s')
                    .long("max-size")
                    .value_name("BYTES")
                    .help("Maximum file size to process")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("1048576"),
            )
            .arg(
                Arg::new("include")
                    .short('i')
                    .long("include")
                    .value_name("PATTERN")
                    .help("File patterns to include (can be used multiple times)")
                    .action(clap::ArgAction::Append)
                    .default_values(&["**/*.rs", "**/*.py"]),
            )
            .arg(
                Arg::new("exclude")
                    .short('e')
                    .long("exclude")
                    .value_name("PATTERN")
                    .help("File patterns to exclude (can be used multiple times)")
                    .action(clap::ArgAction::Append)
                    .default_values(&["**/target/**", "**/node_modules/**"]),
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
    pub fn parse_config(matches: &clap::ArgMatches) -> StreamerConfig {
        StreamerConfig {
            root_dir: PathBuf::from(matches.get_one::<String>("directory").unwrap()),
            db_path: matches.get_one::<String>("database").unwrap().clone(),
            max_file_size: *matches.get_one::<usize>("max-size").unwrap(),
            include_patterns: matches
                .get_many::<String>("include")
                .unwrap_or_default()
                .cloned()
                .collect(),
            exclude_patterns: matches
                .get_many::<String>("exclude")
                .unwrap_or_default()
                .cloned()
                .collect(),
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
            "--dir",
            "/test/dir",
            "--db",
            "test.db",
            "--max-size",
            "2048000",
            "--include",
            "**/*.js",
            "--exclude",
            "**/test/**",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.root_dir, PathBuf::from("/test/dir"));
        assert_eq!(config.db_path, "test.db");
        assert_eq!(config.max_file_size, 2048000);
        assert!(config.include_patterns.contains(&"**/*.js".to_string()));
        assert!(config.exclude_patterns.contains(&"**/test/**".to_string()));
    }

    #[test]
    fn test_default_config() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&["parseltongue-01"]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.root_dir, PathBuf::from("."));
        assert_eq!(config.db_path, "parseltongue.db");
        assert_eq!(config.max_file_size, 1048576);
        assert_eq!(config.include_patterns.len(), 2);
        assert_eq!(config.exclude_patterns.len(), 2);
    }
}