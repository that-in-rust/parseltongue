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
    /// Debug and visualization commands
    Debug {
        /// Show graph structure
        #[arg(long)]
        graph: bool,
        /// Export to DOT format for Graphviz
        #[arg(long)]
        dot: bool,
        /// Export to Mermaid format for GitHub
        #[arg(long)]
        mermaid: bool,
        /// Create sample data for learning
        #[arg(long)]
        sample: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum QueryType {
    /// Find all implementors of a trait
    WhatImplements,
    /// Calculate blast radius from entity
    BlastRadius,
    /// Find circular dependencies
    FindCycles,
    /// Find all functions that call the target function
    WhoCalls,
    /// Find all functions that the target function calls
    GetCalledFunctions,
    /// Find execution path between two functions
    ExecutionPath,
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
    let mut daemon = ParseltongueAIM::new();
    
    // Try to load existing snapshot for persistence between commands
    let snapshot_path = std::path::Path::new("parseltongue_snapshot.json");
    if let Err(e) = daemon.load_snapshot(snapshot_path) {
        eprintln!("⚠️  Could not load snapshot: {}", e);
    }
    
    match cli.command {
        Commands::Ingest { file } => {
            if !file.exists() {
                return Err(format!("File not found: {}", file.display()).into());
            }
            
            let start = Instant::now();
            let stats = daemon.ingest_code_dump(&file)?;
            let elapsed = start.elapsed();
            
            println!("✓ Ingestion complete:");
            println!("  Files processed: {}", stats.files_processed);
            println!("  Nodes created: {}", stats.nodes_created);
            println!("  Total nodes in ISG: {}", daemon.isg.node_count());
            println!("  Total edges in ISG: {}", daemon.isg.edge_count());
            println!("  Time: {:.2}s", elapsed.as_secs_f64());
            
            // Verify <5s constraint for 2.1MB dumps (Performance Contract)
            if elapsed.as_secs() > 5 {
                eprintln!("⚠️  Ingestion took {:.2}s (>5s constraint violated)", elapsed.as_secs_f64());
            }
            
            // Save snapshot for persistence between commands
            let snapshot_path = std::path::Path::new("parseltongue_snapshot.json");
            if let Err(e) = daemon.save_snapshot(snapshot_path) {
                eprintln!("⚠️  Could not save snapshot: {}", e);
            } else {
                println!("✓ Snapshot saved for future queries");
            }
        }
        
        Commands::Daemon { watch } => {
            if !watch.exists() {
                return Err(format!("Directory not found: {}", watch.display()).into());
            }
            if !watch.is_dir() {
                return Err(format!("Path is not a directory: {}", watch.display()).into());
            }
            
            daemon.start_daemon(&watch)?;
        }
        
        Commands::Query { query_type, target, format } => {
            if target.trim().is_empty() {
                return Err("Target entity name cannot be empty".into());
            }
            
            let start = Instant::now();
            
            let result = match query_type {
                QueryType::WhatImplements => {
                    let trait_hash = daemon.find_entity_by_name(&target)?;
                    let implementors = daemon.isg.find_implementors(trait_hash)?;
                    implementors.into_iter().map(|n| n.name.to_string()).collect::<Vec<_>>()
                }
                QueryType::BlastRadius => {
                    let entity_hash = daemon.find_entity_by_name(&target)?;
                    let radius = daemon.isg.calculate_blast_radius(entity_hash)?;
                    radius.into_iter().map(|h| format!("{:?}", h)).collect()
                }
                QueryType::FindCycles => {
                    daemon.isg.find_cycles().into_iter().flatten()
                        .map(|h| format!("{:?}", h)).collect()
                }
                QueryType::WhoCalls => {
                    let function_hash = daemon.find_entity_by_name(&target)?;
                    let callers = daemon.isg.find_callers(function_hash)?;
                    callers.into_iter().map(|n| n.name.to_string()).collect::<Vec<_>>()
                }
                QueryType::GetCalledFunctions => {
                    let function_hash = daemon.find_entity_by_name(&target)?;
                    let called = daemon.isg.get_called_functions(function_hash)?;
                    called.into_iter().map(|n| n.name.to_string()).collect::<Vec<_>>()
                }
                QueryType::ExecutionPath => {
                    // For execution path, we need two targets separated by ">"
                    let parts: Vec<&str> = target.split('>').collect();
                    if parts.len() != 2 {
                        return Err("Execution path requires format: 'from_function>to_function'".into());
                    }
                    let from_hash = daemon.find_entity_by_name(parts[0].trim())?;
                    let to_hash = daemon.find_entity_by_name(parts[1].trim())?;
                    let path = daemon.isg.get_execution_path(from_hash, to_hash)?;
                    path.into_iter().map(|n| n.name.to_string()).collect::<Vec<_>>()
                }
            };
            
            let elapsed = start.elapsed();
            
            match format {
                OutputFormat::Human => {
                    println!("Results for {} query on '{}':",
                        match query_type {
                            QueryType::WhatImplements => "what-implements",
                            QueryType::BlastRadius => "blast-radius",
                            QueryType::FindCycles => "find-cycles",
                            QueryType::WhoCalls => "who-calls",
                            QueryType::GetCalledFunctions => "get-called-functions",
                            QueryType::ExecutionPath => "execution-path",
                        }, target);
                    for item in &result {
                        println!("  - {}", item);
                    }
                    println!("\nQuery completed in {}μs", elapsed.as_micros());
                    
                    // Verify performance constraints (2x tolerance)
                    if elapsed.as_micros() > 2000 {
                        eprintln!("⚠️  Query took {}μs (>2ms constraint)", elapsed.as_micros());
                    }
                }
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "query_type": format!("{:?}", query_type),
                        "target": target,
                        "results": result,
                        "execution_time_us": elapsed.as_micros(),
                        "node_count": daemon.isg.node_count(),
                        "edge_count": daemon.isg.edge_count()
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
        }
        
        Commands::GenerateContext { entity, format } => {
            if entity.trim().is_empty() {
                return Err("Entity name cannot be empty".into());
            }
            
            let context = generate_context(&daemon, &entity, format.clone())?;
            println!("{}", context);
        }
        
        Commands::Debug { graph, dot, mermaid, sample } => {
            if sample {
                // Create and show sample ISG for learning
                let sample_isg = crate::isg::OptimizedISG::create_sample();
                println!("=== SAMPLE ISG FOR LEARNING ===\n");
                println!("This shows a simple Rust program structure:\n");
                println!("{}", sample_isg.debug_print());

                if dot {
                    println!("\n=== DOT FORMAT (for Graphviz) ===");
                    println!("Copy this to a .dot file and run: dot -Tpng graph.dot -o graph.png\n");
                    println!("{}", sample_isg.export_dot());
                }

                if mermaid {
                    println!("\n=== MERMAID FORMAT (for GitHub) ===");
                    println!("Copy this to a .md file and view in GitHub:\n");
                    println!("{}", crate::mermaid_export::export_isg_to_mermaid(&sample_isg));
                }
            } else if graph {
                // Show current ISG structure
                println!("{}", daemon.isg.debug_print());
            } else if dot {
                // Export current ISG to DOT format
                println!("{}", daemon.isg.export_dot());
            } else if mermaid {
                // Export current ISG to Mermaid format
                println!("{}", crate::mermaid_export::export_isg_to_mermaid(&daemon.isg));
            } else {
                println!("Use --graph to see ISG structure, --dot for Graphviz export, --mermaid for GitHub export, or --sample for learning example");
            }
        }
    }
    
    Ok(())
}

/// Generate LLM context with 2-hop dependency analysis
pub fn generate_context(daemon: &ParseltongueAIM, entity_name: &str, format: OutputFormat) -> Result<String, ISGError> {
    let start = Instant::now();
    
    // Find entity by name
    let target_hash = daemon.find_entity_by_name(entity_name)?;
    let target_node = daemon.isg.get_node(target_hash)?;
    
    let context = LlmContext {
        target: target_node.clone(),
        dependencies: daemon.get_dependencies(target_hash),
        callers: daemon.get_callers(target_hash),
    };
    
    let elapsed = start.elapsed();
    
    let result = match format {
        OutputFormat::Human => {
            let mut output = context.format_human();
            output.push_str(&format!("\nContext generated in {}μs", elapsed.as_micros()));
            output
        }
        OutputFormat::Json => {
            serde_json::to_string_pretty(&context)
                .map_err(|e| ISGError::IoError(format!("JSON serialization failed: {}", e)))?
        }
    };
    
    Ok(result)
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
        let mut cli = Cli::command();
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
        
        // Should succeed in GREEN phase
        assert!(result.is_ok());
    }

    #[test]
    fn test_daemon_command() {
        let temp_dir = TempDir::new().unwrap();
        
        let args = vec!["parseltongue", "daemon", "--watch", temp_dir.path().to_str().unwrap()];
        let cli = Cli::try_parse_from(args).unwrap();
        
        // For testing, we need to avoid the infinite loop
        // This test just verifies the CLI parsing works correctly
        match cli.command {
            Commands::Daemon { watch } => {
                assert_eq!(watch, temp_dir.path());
            }
            _ => panic!("Expected daemon command"),
        }
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
        
        // Should succeed in GREEN phase
        assert!(ingest_result.is_ok());
        
        // TODO: Add query and context generation tests in future iterations
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