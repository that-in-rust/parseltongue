//! CLI Interface for Parseltongue AIM Daemon
//! 
//! Provides command-line interface with performance monitoring and JSON/human output

use crate::daemon::ParseltongueAIM;
use crate::isg::ISGError;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "parseltongue")]
#[command(about = "Rust-only architectural intelligence daemon")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ingest code dump with FILE: markers
    Ingest {
        /// Path to code dump file
        file: PathBuf,
    },
    /// Start daemon monitoring .rs files
    Daemon {
        /// Directory to watch recursively
        #[arg(long)]
        watch: PathBuf,
    },
    /// Execute graph queries
    Query {
        /// Query type
        #[arg(value_enum)]
        query_type: QueryType,
        /// Target entity name
        target: String,
        /// Output format
        #[arg(long, default_value = "human")]
        format: OutputFormat,
    },
    /// Generate LLM context for entity
    GenerateContext {
        /// Entity name
        entity: String,
        /// Output format
        #[arg(long, default_value = "human")]
        format: OutputFormat,
    },
}

#[derive(Clone, ValueEnum)]
pub enum QueryType {
    /// Find all implementors of a trait
    WhatImplements,
    /// Calculate blast radius from entity
    BlastRadius,
    /// Find circular dependencies
    FindCycles,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// JSON output for LLM consumption
    Json,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LlmContext {
    pub target: crate::isg::NodeData,
    pub dependencies: Vec<crate::isg::NodeData>,
    pub callers: Vec<crate::isg::NodeData>,
}

impl LlmContext {
    pub fn format_human(&self) -> String {
        format!(
            "Entity: {} ({:?})\nSignature: {}\nFile: {}:{}\n\nDependencies ({}):\n{}\n\nCallers ({}):\n{}",
            self.target.name,
            self.target.kind,
            self.target.signature,
            self.target.file_path,
            self.target.line,
            self.dependencies.len(),
            self.dependencies.iter()
                .map(|d| format!("  - {} ({}): {}", d.name, d.file_path, d.signature))
                .collect::<Vec<_>>()
                .join("\n"),
            self.callers.len(),
            self.callers.iter()
                .map(|c| format!("  - {} ({}): {}", c.name, c.file_path, c.signature))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement in GREEN phase
    Err("Not implemented".into())
}

/// Generate LLM context with 2-hop dependency analysis
pub fn generate_context(daemon: &ParseltongueAIM, entity_name: &str, format: OutputFormat) -> Result<String, ISGError> {
    // TODO: Implement in GREEN phase
    Err(ISGError::NodeNotFound(crate::isg::SigHash(0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    // TDD Cycle 14: CLI parsing (RED phase)
    #[test]
    fn test_cli_parsing() {
        // Test ingest command
        let args = vec!["parseltongue", "ingest", "test.dump"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Ingest { file } => {
                assert_eq!(file, PathBuf::from("test.dump"));
            }
            _ => panic!("Expected Ingest command"),
        }
        
        // Test daemon command
        let args = vec!["parseltongue", "daemon", "--watch", "/path/to/watch"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Daemon { watch } => {
                assert_eq!(watch, PathBuf::from("/path/to/watch"));
            }
            _ => panic!("Expected Daemon command"),
        }
        
        // Test query command
        let args = vec!["parseltongue", "query", "what-implements", "TestTrait", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Query { query_type, target, format } => {
                assert!(matches!(query_type, QueryType::WhatImplements));
                assert_eq!(target, "TestTrait");
                assert!(matches!(format, OutputFormat::Json));
            }
            _ => panic!("Expected Query command"),
        }
        
        // Test generate-context command
        let args = vec!["parseltongue", "generate-context", "MyFunction"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::GenerateContext { entity, format } => {
                assert_eq!(entity, "MyFunction");
                assert!(matches!(format, OutputFormat::Human));
            }
            _ => panic!("Expected GenerateContext command"),
        }
    }

    #[test]
    fn test_cli_help_output() {
        use clap::CommandFactory;
        let cli = Cli::command();
        let help = cli.render_help();
        
        // Should contain all required commands
        assert!(help.to_string().contains("ingest"));
        assert!(help.to_string().contains("daemon"));
        assert!(help.to_string().contains("query"));
        assert!(help.to_string().contains("generate-context"));
    }

    // TDD Cycle 15: Query command execution (RED phase)
    #[test]
    fn test_query_command_execution() {
        // This test will fail until we implement query execution
        let args = vec!["parseltongue", "query", "what-implements", "TestTrait"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli);
        
        // Should fail in RED phase
        assert!(result.is_err());
    }

    #[test]
    fn test_query_performance_reporting() {
        // Test that query commands measure and report performance
        // This will be implemented in GREEN phase
        
        // For now, just validate the structure exists
        assert!(true, "Performance reporting structure ready");
    }

    // TDD Cycle 16: Ingest and daemon commands (RED phase)
    #[test]
    fn test_ingest_command() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        fs::write(&dump_path, "FILE: test.rs\npub fn test() {}").unwrap();
        
        let args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli);
        
        // Should fail in RED phase
        assert!(result.is_err());
    }

    #[test]
    fn test_daemon_command() {
        let temp_dir = TempDir::new().unwrap();
        
        let args = vec!["parseltongue", "daemon", "--watch", temp_dir.path().to_str().unwrap()];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli);
        
        // Should fail in RED phase
        assert!(result.is_err());
    }

    // TDD Cycle 17: LLM context generation (RED phase)
    #[test]
    fn test_generate_context_human() {
        let daemon = ParseltongueAIM::new();
        
        let result = generate_context(&daemon, "test_function", OutputFormat::Human);
        
        // Should fail in RED phase
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_context_json() {
        let daemon = ParseltongueAIM::new();
        
        let result = generate_context(&daemon, "test_function", OutputFormat::Json);
        
        // Should fail in RED phase
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_context_command() {
        let args = vec!["parseltongue", "generate-context", "TestFunction", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli);
        
        // Should fail in RED phase
        assert!(result.is_err());
    }

    // TDD Cycle 18: LLM context formatting (RED phase)
    #[test]
    fn test_llm_context_format_human() {
        use crate::isg::{NodeData, NodeKind, SigHash};
        use std::sync::Arc;
        
        let target = NodeData {
            hash: SigHash(1),
            kind: NodeKind::Function,
            name: Arc::from("test_function"),
            signature: Arc::from("fn test_function() -> i32"),
            file_path: Arc::from("test.rs"),
            line: 10,
        };
        
        let context = LlmContext {
            target,
            dependencies: Vec::new(),
            callers: Vec::new(),
        };
        
        let formatted = context.format_human();
        
        assert!(formatted.contains("test_function"));
        assert!(formatted.contains("Function"));
        assert!(formatted.contains("test.rs:10"));
        assert!(formatted.contains("Dependencies (0)"));
        assert!(formatted.contains("Callers (0)"));
    }

    #[test]
    fn test_llm_context_json_serialization() {
        use crate::isg::{NodeData, NodeKind, SigHash};
        use std::sync::Arc;
        
        let target = NodeData {
            hash: SigHash(1),
            kind: NodeKind::Function,
            name: Arc::from("test_function"),
            signature: Arc::from("fn test_function() -> i32"),
            file_path: Arc::from("test.rs"),
            line: 10,
        };
        
        let context = LlmContext {
            target,
            dependencies: Vec::new(),
            callers: Vec::new(),
        };
        
        let json = serde_json::to_string_pretty(&context).unwrap();
        
        assert!(json.contains("test_function"));
        assert!(json.contains("Function"));
        assert!(json.contains("dependencies"));
        assert!(json.contains("callers"));
    }

    // TDD Cycle 19: End-to-end workflow (RED phase)
    #[test]
    fn test_end_to_end_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        // Create test dump
        let dump_content = r#"
FILE: src/lib.rs
pub fn hello() -> String {
    "Hello".to_string()
}

pub trait Greeter {
    fn greet(&self) -> String;
}

pub struct Person {
    name: String,
}

impl Greeter for Person {
    fn greet(&self) -> String {
        format!("Hello, {}", self.name)
    }
}
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // Test complete workflow: ingest → query → context
        
        // 1. Ingest
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let ingest_result = run(ingest_cli);
        
        // Should fail in RED phase
        assert!(ingest_result.is_err());
        
        // TODO: Continue with query and context generation once ingest works
    }

    #[test]
    fn test_performance_requirements_met() {
        // This test validates all performance requirements are met
        // Will be implemented in GREEN phase
        
        // Performance targets:
        // - Code dump ingestion: <5s for 2.1MB
        // - File updates: <12ms
        // - Simple queries: <500μs
        // - Complex queries: <1ms
        // - Persistence: <500ms
        
        assert!(true, "Performance requirements test structure ready");
    }
}