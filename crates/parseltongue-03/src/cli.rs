//! Command-line interface for parseltongue-03.

use clap::{Arg, Command};
use std::path::PathBuf;

use crate::ContextWriterConfig;

/// CLI configuration builder
pub struct CliConfig;

impl CliConfig {
    /// Build CLI application
    pub fn build_cli() -> Command {
        Command::new("parseltongue-03")
            .version("0.7.0")
            .author("Parseltongue Team")
            .about("Tool 03: LLM-cozoDB-to-context-writer")
            .long_about(
                "Ultra-minimalist context optimization tool that reads entity graphs from CozoDB,\\n\\\n                generates optimized CodeGraphContext.json files using LLM reasoning, and writes\\n\\\n                them for consumption by other tools. Following TDD-first principles with\\n\\\n                executable specifications.",
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
                    .help("LLM API key (or set OPENAI_API_KEY env var)"),
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
                    .default_value("8192"),
            )
            .arg(
                Arg::new("temperature")
                    .short('T')
                    .long("temperature")
                    .value_name("TEMP")
                    .help("Temperature for LLM generation (0.0-1.0)")
                    .value_parser(clap::value_parser!(f32))
                    .default_value("0.3"),
            )
            .arg(
                Arg::new("query")
                    .short('q')
                    .long("query")
                    .value_name("QUERY")
                    .help("Query to select entity graph for context generation")
                    .default_value("MATCH (e:Entity)-[r:RELATED_TO]->(n:Entity) RETURN e, r, n LIMIT 100"),
            )
            .arg(
                Arg::new("max-context-tokens")
                    .short('c')
                    .long("max-context-tokens")
                    .value_name("TOKENS")
                    .help("Maximum context size in tokens")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("128000"),
            )
            .arg(
                Arg::new("relevance-threshold")
                    .short('r')
                    .long("relevance-threshold")
                    .value_name("THRESHOLD")
                    .help("Relevance threshold for entity inclusion (0.0-1.0)")
                    .value_parser(clap::value_parser!(f32))
                    .default_value("0.7"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("PATH")
                    .help("Output directory for context files")
                    .default_value("./contexts"),
            )
            .arg(
                Arg::new("context-id")
                    .short('i')
                    .long("context-id")
                    .value_name("ID")
                    .help("Custom context ID (auto-generated if not provided)"),
            )
            .arg(
                Arg::new("focus-areas")
                    .short('f')
                    .long("focus-areas")
                    .value_name("AREAS")
                    .help("Comma-separated focus areas for optimization")
                    .default_value("core_types,implementations"),
            )
            .arg(
                Arg::new("optimization-goals")
                    .short('g')
                    .long("optimization-goals")
                    .value_name("GOALS")
                    .help("Comma-separated optimization goals")
                    .default_value("minimize_size,maximize_relevance,preserve_connectivity"),
            )
            .arg(
                Arg::new("dry-run")
                    .short('d')
                    .long("dry-run")
                    .help("Generate context but don't write to file")
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

    /// Parse CLI arguments into ContextWriterConfig
    pub fn parse_config(matches: &clap::ArgMatches) -> ContextWriterConfig {
        ContextWriterConfig {
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
            entity_query: matches.get_one::<String>("query").unwrap().clone(),
            max_context_tokens: *matches.get_one::<usize>("max-context-tokens").unwrap(),
            relevance_threshold: *matches.get_one::<f32>("relevance-threshold").unwrap(),
            output_dir: matches.get_one::<String>("output").unwrap().clone(),
        }
    }

    /// Parse focus areas from CLI argument
    pub fn parse_focus_areas(areas_str: &str) -> Vec<String> {
        areas_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Parse optimization goals from CLI argument
    pub fn parse_optimization_goals(goals_str: &str) -> Vec<crate::llm_client::OptimizationGoal> {
        goals_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .map(|goal| match goal.as_str() {
                "minimize_size" => crate::llm_client::OptimizationGoal::MinimizeSize,
                "maximize_relevance" => crate::llm_client::OptimizationGoal::MaximizeRelevance,
                "preserve_connectivity" => crate::llm_client::OptimizationGoal::PreserveConnectivity,
                "focus_on_types" => crate::llm_client::OptimizationGoal::FocusOnTypes,
                "focus_on_functions" => crate::llm_client::OptimizationGoal::FocusOnFunctions,
                "balance_complexity" => crate::llm_client::OptimizationGoal::BalanceComplexity,
                _ => crate::llm_client::OptimizationGoal::MaximizeRelevance, // default fallback
            })
            .collect()
    }

    /// Generate output file path
    pub fn generate_output_path(output_dir: &str, context_id: &str) -> String {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        format!("{}/context_{}_{}.json", output_dir, context_id, timestamp)
    }

    /// Print usage information
    pub fn print_usage() {
        let mut cli = Self::build_cli();
        cli.print_help().unwrap();
        println!();
    }

    /// Print version information
    pub fn print_version() {
        println!("parseltongue-03 version 0.7.0");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_config_parsing() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-03",
            "--db",
            "test.db",
            "--endpoint",
            "https://api.example.com/v1/chat",
            "--api-key",
            "test-key-123",
            "--model",
            "gpt-3.5-turbo",
            "--max-tokens",
            "4096",
            "--temperature",
            "0.2",
            "--max-context-tokens",
            "64000",
            "--relevance-threshold",
            "0.8",
            "--output",
            "./test_contexts",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.db_path, "test.db");
        assert_eq!(config.llm_endpoint, "https://api.example.com/v1/chat");
        assert_eq!(config.llm_api_key, "test-key-123");
        assert_eq!(config.model, "gpt-3.5-turbo");
        assert_eq!(config.max_tokens, 4096);
        assert_eq!(config.temperature, 0.2);
        assert_eq!(config.max_context_tokens, 64000);
        assert_eq!(config.relevance_threshold, 0.8);
        assert_eq!(config.output_dir, "./test_contexts");
    }

    #[test]
    fn test_focus_areas_parsing() {
        let areas = CliConfig::parse_focus_areas("core_types, implementations, tests");
        assert_eq!(areas.len(), 3);
        assert_eq!(areas[0], "core_types");
        assert_eq!(areas[1], "implementations");
        assert_eq!(areas[2], "tests");

        let empty_areas = CliConfig::parse_focus_areas("");
        assert_eq!(empty_areas.len(), 0);
    }

    #[test]
    fn test_optimization_goals_parsing() {
        let goals = CliConfig::parse_optimization_goals("minimize_size,maximize_relevance,preserve_connectivity");
        assert_eq!(goals.len(), 3);
        assert!(matches!(goals[0], crate::llm_client::OptimizationGoal::MinimizeSize));
        assert!(matches!(goals[1], crate::llm_client::OptimizationGoal::MaximizeRelevance));
        assert!(matches!(goals[2], crate::llm_client::OptimizationGoal::PreserveConnectivity));

        let invalid_goals = CliConfig::parse_optimization_goals("invalid_goal,another_invalid");
        assert_eq!(invalid_goals.len(), 2);
        // Should fall back to MaximizeRelevance for invalid goals
    }

    #[test]
    fn test_output_path_generation() {
        let context_id = "test_context_123";
        let output_path = CliConfig::generate_output_path("./contexts", context_id);

        assert!(output_path.starts_with("./contexts/context_test_context_123_"));
        assert!(output_path.ends_with(".json"));
    }

    #[test]
    fn test_default_config() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&["parseltongue-03"]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.db_path, "parseltongue.db");
        assert_eq!(config.llm_endpoint, "https://api.openai.com/v1/chat/completions");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.max_tokens, 8192);
        assert_eq!(config.temperature, 0.3);
        assert_eq!(config.max_context_tokens, 128000);
        assert_eq!(config.relevance_threshold, 0.7);
        assert_eq!(config.output_dir, "./contexts");
    }

    #[test]
    fn test_environment_variable_api_key() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "parseltongue-03",
            "--api-key",
            "env-key-456",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();

        let config = CliConfig::parse_config(&matches);
        assert_eq!(config.llm_api_key, "env-key-456");
    }
}