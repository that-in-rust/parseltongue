//! CLI Interface for Parseltongue AIM Daemon
//! 
//! Provides command-line interface with performance monitoring and JSON/human output

use crate::daemon::ParseltongueAIM;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;
use chrono::Utc;

#[derive(Parser)]
#[command(name = "parseltongue")]
#[command(about = "Multi-language architectural intelligence daemon")]
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
        /// Language for parsing (rust, python, etc.)
        #[arg(long, default_value = "rust")]
        language: String,
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
    /// Export ISG diagram to Mermaid Markdown
    Export {
        /// Output file path (optional, auto-generated if not provided)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Export ISG diagram to WASM visualization
    ExportWasm {
        /// Output directory (optional, creates 'wasm_output' if not provided)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Layout algorithm to use
        #[arg(long, default_value = "breadthfirst")]
        layout: String,
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
        Commands::Ingest { file, language } => {
            if !file.exists() {
                return Err(format!("File not found: {}", file.display()).into());
            }

            // Validate language
            if language != "rust" && language != "python" {
                return Err(format!("Unsupported language: {}. Supported: rust, python", language).into());
            }

            let start = Instant::now();
            let stats = daemon.ingest_code_dump(&file)?;
            let elapsed = start.elapsed();

            println!("✓ Ingestion complete (language: {}):", language);
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

        Commands::Export { output } => {
            let start = Instant::now();
            let output_path = match output {
                Some(path) => path,
                None => {
                    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
                    PathBuf::from(format!("ISG_Architecture_{}", timestamp))
                }
            };
            let mermaid_content = crate::mermaid_export::export_isg_to_mermaid(&daemon.isg);

      let elapsed = start.elapsed();

      // Write MD file with extension
      let md_path = output_path.with_extension("md");
      std::fs::write(&md_path, mermaid_content)?;

      println!("✓ Mermaid export completed:");
      println!("  MD:   {} (GitHub compatible)", md_path.display());
      println!("  Nodes: {}", daemon.isg.node_count());
      println!("  Edges: {}", daemon.isg.edge_count());
      println!("  Time: {:.2}s", elapsed.as_secs_f64());

      // Save snapshot for persistence
      if let Err(e) = daemon.save_snapshot(snapshot_path) {
          eprintln!("⚠️  Could not save snapshot: {}", e);
      }

      println!("✓ File created successfully");
        }

        Commands::ExportWasm { output, layout } => {
            let start = Instant::now();
            let output_dir = match output {
                Some(path) => path,
                None => PathBuf::from("wasm_output"),
            };

            // Create output directory if it doesn't exist
            std::fs::create_dir_all(&output_dir)?;

            // Serialize ISG to JSON
            let isg_json = serde_json::to_string_pretty(&daemon.isg)?;

            // Write ISG JSON file
            let isg_path = output_dir.join("isg_data.json");
            std::fs::write(&isg_path, isg_json)?;

            // Generate WASM visualization files
            let wasm_content = crate::wasm_renderer::generate_wasm_visualization(&daemon.isg, &layout)?;

            // Write WASM HTML file
            let html_path = output_dir.join("visualization.html");
            std::fs::write(&html_path, wasm_content)?;

            let elapsed = start.elapsed();

            println!("✓ WASM export completed:");
            println!("  Output directory: {}", output_dir.display());
            println!("  ISG JSON: {}", isg_path.display());
            println!("  HTML Visualization: {}", html_path.display());
            println!("  Layout algorithm: {}", layout);
            println!("  Nodes: {}", daemon.isg.node_count());
            println!("  Edges: {}", daemon.isg.edge_count());
            println!("  Time: {:.2}s", elapsed.as_secs_f64());

            // Save snapshot for persistence
            if let Err(e) = daemon.save_snapshot(snapshot_path) {
                eprintln!("⚠️  Could not save snapshot: {}", e);
            }

            println!("✓ Open {} in your browser to view the visualization", html_path.display());
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
                println!("=== CURRENT ISG STRUCTURE ===\n");
                println!("{}", daemon.isg.debug_print());
            } else if dot {
                // Export to DOT format for Graphviz
                let dot_content = daemon.isg.export_dot();
                println!("=== DOT FORMAT (for Graphviz) ===");
                println!("Copy this to a .dot file and run: dot -Tpng graph.dot -o graph.png\n");
                println!("{}", dot_content);
            } else if mermaid {
                // Export to Mermaid format for GitHub
                let mermaid_content = crate::mermaid_export::export_isg_to_mermaid(&daemon.isg);
                println!("=== MERMAID FORMAT (for GitHub) ===");
                println!("Copy this to a .md file and view in GitHub:\n");
                println!("{}", mermaid_content);
            } else {
                // Show usage
                println!("Debug commands require --graph, --dot, --mermaid, or --sample flag");
            }
        }
    }
    Ok(())
}

/// Generate context for LLM consumption
fn generate_context(daemon: &ParseltongueAIM, entity: &str, format: OutputFormat) -> Result<String, Box<dyn std::error::Error>> {
    // Find the entity in the ISG
    if let Ok(entity_hash) = daemon.find_entity_by_name(entity) {
        let dependencies = daemon.get_dependencies(entity_hash);
        let callers = daemon.get_callers(entity_hash);

        let context = LlmContext {
            target: daemon.get_entity_data(entity_hash)?,
            dependencies,
            callers,
        };

        match format {
            OutputFormat::Human => Ok(format!("Entity: {}\nDependencies: {}\nCallers: {}",
                entity, context.dependencies.len(), context.callers.len())),
            OutputFormat::Json => Ok(serde_json::to_string_pretty(&context)?),
        }
    } else {
        Err(format!("Entity '{}' not found", entity).into())
    }
}
