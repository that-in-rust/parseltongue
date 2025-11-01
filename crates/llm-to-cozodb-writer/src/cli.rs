//! Command-line interface for parseltongue-02.

use clap::{Arg, Command};
use std::path::PathBuf;

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
                "Ultra-minimalist LLM communication tool that reads ISGL1 keys from CozoDB,\n\
                generates code changes using LLM reasoning, and writes them back as temporal\n\
                changes. Following TDD-first principles with executable specifications.",
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
                Arg::new("endpoint")
                    .short('e')
                    .long("endpoint")
                    .value_name("URL")
                    .help("LLM API endpoint")
                    .default_value("https://api.openai.com/v1/chat/completions"),
            )
            .arg(
                Arg::new("api-key")
                    .short('k')
                    .long("api-key")
                    .value_name("KEY")
                    .help("LLM API key (or set OPENAI_API_KEY env var)")
            )
            .arg(
                Arg::new("model")
                    .short('m')
                    .long("model")
                    .value_name("MODEL")
                    .help("LLM model to use")
                    .default_value("gpt-4"),
            )
            .arg(
                Arg::new("max-tokens")
                    .short('t')
                    .long("max-tokens")
                    .value_name("TOKENS")
                    .help("Maximum tokens per request")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("4096"),
            )
            .arg(
                Arg::new("temperature")
                    .short('T')
                    .long("temperature")
                    .value_name("TEMP")
                    .help("Temperature for LLM generation (0.0-1.0)")
                    .value_parser(clap::value_parser!(f32))
                    .default_value("0.7"),
            )
            .arg(
                Arg::new("query")
                    .short('q')
                    .long("query")
                    .value_name("SQL")
                    .help("Query to select entities for processing")
                    .default_value("SELECT * FROM CodeEntity WHERE temporal_state = 'current' LIMIT 10"),
            )
            .arg(
                Arg::new("batch-size")
                    .short('s')
                    .long("batch-size")
                    .value_name("SIZE")
                    .help("Batch size for processing")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("5"),
            )
            .arg(
                Arg::new("dry-run")
                    .short('d')
                    .long("dry-run")
                    .help("Generate changes but don't apply them")
                    .action(clap::ArgAction::SetTrue),
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
                    .short('Q')
                    .long("quiet")
                    .help("Suppress output except errors")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("verbose"),
            )
    }

    /// Parse CLI arguments into LlmWriterConfig
    pub fn parse_config(matches: &clap::ArgMatches) -> LlmWriterConfig {
        LlmWriterConfig {
            db_path: matches.get_one::<String>("database").unwrap().clone(),
            llm_endpoint: matches.get_one::<String>("endpoint").unwrap().clone(),
            llm_api_key: matches
                .get_one::<String>("api-key")
                .cloned()
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .unwrap_or_default(),
            model: matches.get_one::<String>("model").unwrap().clone(),
            max_tokens: *matches.get_one::<usize>("max-tokens").unwrap(),
            temperature: *matches.get_one::<f32>("temperature").unwrap(),
            query_filter: matches.get_one::<String>("query").unwrap().clone(),
            batch_size: *matches.get_one::<usize>("batch-size").unwrap(),
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
            "--db",
            "test.db",
            "--endpoint",
            "https://api.example.com/v1/chat",
            "--api-key",
            "test-key-123",
            "--model",
            "gpt-3.5-turbo",
            "--max-tokens",
            "2048",
            "--temperature",
            "0.5",
            "--batch-size",
            "3",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.db_path, "test.db");
        assert_eq!(config.llm_endpoint, "https://api.example.com/v1/chat");
        assert_eq!(config.llm_api_key, "test-key-123");
        assert_eq!(config.model, "gpt-3.5-turbo");
        assert_eq!(config.max_tokens, 2048);
        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.batch_size, 3);
    }

    #[test]
    fn test_default_config() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&["parseltongue-02"]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.db_path, "parseltongue.db");
        assert_eq!(config.llm_endpoint, "https://api.openai.com/v1/chat/completions");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.max_tokens, 4096);
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.batch_size, 5);
    }

    #[test]
    fn test_environment_variable_api_key() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-02",
            "--api-key",
            "env-key-456",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.llm_api_key, "env-key-456");
    }
}