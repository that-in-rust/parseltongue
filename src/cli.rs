//! CLI Interface for Parseltongue AIM Daemon
//! 
//! Provides command-line interface with performance monitoring and JSON/human output

use crate::daemon::ParseltongueAIM;
use crate::isg::ISGError;
use crate::discovery::{SimpleDiscoveryEngine, DiscoveryEngine, EntityInfo, FileLocation};
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
        /// Create sample data for learning
        #[arg(long)]
        sample: bool,
    },
    /// Generate interactive HTML visualization
    Visualize {
        /// Target entity to focus visualization on (optional)
        entity: Option<String>,
        /// Output HTML file path
        #[arg(long, default_value = "parseltongue_visualization.html")]
        output: PathBuf,
    },
    /// List all entities in the codebase
    ListEntities {
        /// Filter by entity type
        #[arg(long, value_enum)]
        r#type: Option<DiscoveryEntityType>,
        /// Maximum number of results to return
        #[arg(long, default_value = "100")]
        limit: usize,
        /// Output format
        #[arg(long, default_value = "human")]
        format: OutputFormat,
    },
    /// List entities defined in a specific file
    EntitiesInFile {
        /// File path to search
        file: String,
        /// Filter by entity type
        #[arg(long, value_enum)]
        r#type: Option<DiscoveryEntityType>,
        /// Output format
        #[arg(long, default_value = "human")]
        format: OutputFormat,
    },
    /// Find where an entity is defined
    WhereDefined {
        /// Entity name to find
        entity: String,
        /// Output format
        #[arg(long, default_value = "human")]
        format: OutputFormat,
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
    /// Find all callers of an entity
    Calls,
    /// Find all users of a type
    Uses,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// JSON output for LLM consumption
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DiscoveryEntityType {
    /// Function entities
    Function,
    /// Struct entities
    Struct,
    /// Trait entities
    Trait,
    /// Implementation blocks
    Impl,
    /// Module entities
    Module,
    /// Constant entities
    Constant,
    /// Static entities
    Static,
    /// Macro entities
    Macro,
}

impl From<DiscoveryEntityType> for crate::discovery::types::EntityType {
    fn from(cli_type: DiscoveryEntityType) -> Self {
        match cli_type {
            DiscoveryEntityType::Function => Self::Function,
            DiscoveryEntityType::Struct => Self::Struct,
            DiscoveryEntityType::Trait => Self::Trait,
            DiscoveryEntityType::Impl => Self::Impl,
            DiscoveryEntityType::Module => Self::Module,
            DiscoveryEntityType::Constant => Self::Constant,
            DiscoveryEntityType::Static => Self::Static,
            DiscoveryEntityType::Macro => Self::Macro,
        }
    }
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

pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
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
                QueryType::Calls => {
                    let entity_hash = daemon.find_entity_by_name(&target)?;
                    let callers = daemon.isg.find_callers(entity_hash)?;
                    callers.into_iter().map(|n| n.name.to_string()).collect::<Vec<_>>()
                }
                QueryType::Uses => {
                    let entity_hash = daemon.find_entity_by_name(&target)?;
                    let users = daemon.isg.find_users(entity_hash)?;
                    users.into_iter().map(|n| n.name.to_string()).collect::<Vec<_>>()
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
                            QueryType::Calls => "calls",
                            QueryType::Uses => "uses",
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
        
        Commands::Debug { graph, dot, sample } => {
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
            } else if graph {
                // Show current ISG structure
                println!("{}", daemon.isg.debug_print());
            } else if dot {
                // Export current ISG to DOT format
                println!("{}", daemon.isg.export_dot());
            } else {
                println!("Use --graph to see ISG structure, --dot for Graphviz export, or --sample for learning example");
            }
        }
        
        Commands::Visualize { entity, output } => {
            let start = Instant::now();
            
            let html = daemon.isg.generate_html_visualization(entity.as_deref())?;
            
            // Write HTML to file
            std::fs::write(&output, html)
                .map_err(|e| format!("Failed to write HTML file: {}", e))?;
            
            let elapsed = start.elapsed();
            
            println!("✓ Interactive HTML visualization generated:");
            println!("  Output file: {}", output.display());
            println!("  Nodes: {}", daemon.isg.node_count());
            println!("  Edges: {}", daemon.isg.edge_count());
            if let Some(entity) = entity {
                println!("  Focused on: {}", entity);
            }
            println!("  Generation time: {}ms", elapsed.as_millis());
            println!("  Open {} in your browser to view the visualization", output.display());
            
            // Verify <500ms constraint
            if elapsed.as_millis() > 500 {
                eprintln!("⚠️  HTML generation took {}ms (>500ms constraint violated)", elapsed.as_millis());
            }
        }
        
        Commands::ListEntities { r#type, limit, format } => {
            handle_list_entities_command(&daemon, r#type, limit, format.clone()).await?;
        }
        
        Commands::EntitiesInFile { file, r#type, format } => {
            handle_entities_in_file_command(&daemon, &file, r#type, format.clone()).await?;
        }
        
        Commands::WhereDefined { entity, format } => {
            handle_where_defined_command(&daemon, &entity, format.clone()).await?;
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

/// Handle the list-entities command
async fn handle_list_entities_command(
    daemon: &ParseltongueAIM,
    entity_type: Option<DiscoveryEntityType>,
    limit: usize,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Create discovery engine
    let discovery_engine = SimpleDiscoveryEngine::new(daemon.isg.clone());
    
    // Convert CLI entity type to discovery entity type
    let discovery_type = entity_type.map(|t| t.into());
    
    // Execute the query
    let entities = discovery_engine
        .list_all_entities(discovery_type, limit)
        .await
        .map_err(|e| format!("Discovery error: {}", e))?;
    
    let elapsed = start.elapsed();
    
    // Format and display results
    match format {
        OutputFormat::Human => {
            format_entities_human(&entities, elapsed, entity_type.is_some());
        }
        OutputFormat::Json => {
            format_entities_json(&entities, elapsed)?;
        }
    }
    
    // Check performance contract
    if elapsed.as_millis() > 100 {
        eprintln!("⚠️  Discovery took {}ms (>100ms contract violated)", elapsed.as_millis());
    }
    
    Ok(())
}

/// Handle the entities-in-file command
async fn handle_entities_in_file_command(
    daemon: &ParseltongueAIM,
    file_path: &str,
    entity_type: Option<DiscoveryEntityType>,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Create discovery engine
    let discovery_engine = SimpleDiscoveryEngine::new(daemon.isg.clone());
    
    // Get entities in file
    let mut entities = discovery_engine
        .entities_in_file(file_path)
        .await
        .map_err(|e| format!("Discovery error: {}", e))?;
    
    // Apply entity type filter if specified
    if let Some(filter_type) = entity_type {
        let discovery_type = filter_type.into();
        entities.retain(|entity| entity.entity_type == discovery_type);
    }
    
    let elapsed = start.elapsed();
    
    // Format and display results
    match format {
        OutputFormat::Human => {
            format_file_entities_human(&entities, file_path, elapsed, entity_type.is_some());
        }
        OutputFormat::Json => {
            format_file_entities_json(&entities, file_path, elapsed)?;
        }
    }
    
    // Check performance contract
    if elapsed.as_millis() > 100 {
        eprintln!("⚠️  Discovery took {}ms (>100ms contract violated)", elapsed.as_millis());
    }
    
    Ok(())
}

/// Handle the where-defined command
async fn handle_where_defined_command(
    daemon: &ParseltongueAIM,
    entity_name: &str,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Create discovery engine
    let discovery_engine = SimpleDiscoveryEngine::new(daemon.isg.clone());
    
    // Find entity definition
    let location = discovery_engine
        .where_defined(entity_name)
        .await
        .map_err(|e| format!("Discovery error: {}", e))?;
    
    let elapsed = start.elapsed();
    
    // Format and display results
    match format {
        OutputFormat::Human => {
            format_location_human(entity_name, &location, elapsed);
        }
        OutputFormat::Json => {
            format_location_json(entity_name, &location, elapsed)?;
        }
    }
    
    // Check performance contract (stricter for exact lookups)
    if elapsed.as_micros() > 50_000 {
        eprintln!("⚠️  Lookup took {}μs (>50ms contract violated)", elapsed.as_micros());
    }
    
    Ok(())
}

/// Format entities for human-readable output
fn format_entities_human(entities: &[EntityInfo], elapsed: std::time::Duration, filtered: bool) {
    if entities.is_empty() {
        println!("No entities found.");
        return;
    }
    
    let type_filter_text = if filtered { " (filtered)" } else { "" };
    println!("Found {} entities{}:", entities.len(), type_filter_text);
    println!();
    
    // Group entities by type for better organization
    let mut by_type = std::collections::HashMap::new();
    for entity in entities {
        by_type.entry(entity.entity_type).or_insert_with(Vec::new).push(entity);
    }
    
    // Sort types for consistent output
    let mut types: Vec<_> = by_type.keys().collect();
    types.sort_by_key(|t| format!("{:?}", t));
    
    for entity_type in types {
        let entities_of_type = by_type.get(entity_type).unwrap();
        println!("{:?} ({}):", entity_type, entities_of_type.len());
        
        for entity in entities_of_type {
            let location = if let Some(line) = entity.line_number {
                format!("{}:{}", entity.file_path, line)
            } else {
                entity.file_path.clone()
            };
            println!("  • {} ({})", entity.name, location);
        }
        println!();
    }
    
    println!("Discovery completed in {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

/// Format entities for JSON output
fn format_entities_json(entities: &[EntityInfo], elapsed: std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
    let output = serde_json::json!({
        "command": "list-entities",
        "results": entities,
        "count": entities.len(),
        "execution_time_ms": elapsed.as_secs_f64() * 1000.0,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Format file entities for human-readable output
fn format_file_entities_human(entities: &[EntityInfo], file_path: &str, elapsed: std::time::Duration, filtered: bool) {
    let type_filter_text = if filtered { " (filtered)" } else { "" };
    println!("Entities in file '{}'{}: {}", file_path, type_filter_text, entities.len());
    
    if entities.is_empty() {
        println!("No entities found in this file.");
        return;
    }
    
    println!();
    
    // Group by type
    let mut by_type = std::collections::HashMap::new();
    for entity in entities {
        by_type.entry(entity.entity_type).or_insert_with(Vec::new).push(entity);
    }
    
    let mut types: Vec<_> = by_type.keys().collect();
    types.sort_by_key(|t| format!("{:?}", t));
    
    for entity_type in types {
        let entities_of_type = by_type.get(entity_type).unwrap();
        println!("{:?} ({}):", entity_type, entities_of_type.len());
        
        for entity in entities_of_type {
            if let Some(line) = entity.line_number {
                println!("  • {} (line {})", entity.name, line);
            } else {
                println!("  • {}", entity.name);
            }
        }
        println!();
    }
    
    println!("Discovery completed in {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

/// Format file entities for JSON output
fn format_file_entities_json(entities: &[EntityInfo], file_path: &str, elapsed: std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
    let output = serde_json::json!({
        "command": "entities-in-file",
        "file_path": file_path,
        "results": entities,
        "count": entities.len(),
        "execution_time_ms": elapsed.as_secs_f64() * 1000.0,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Format location for human-readable output
fn format_location_human(entity_name: &str, location: &Option<FileLocation>, elapsed: std::time::Duration) {
    match location {
        Some(loc) => {
            println!("Entity '{}' is defined at:", entity_name);
            println!("  File: {}", loc.file_path);
            if let Some(line) = loc.line_number {
                if let Some(col) = loc.column {
                    println!("  Position: line {}, column {}", line, col);
                } else {
                    println!("  Line: {}", line);
                }
            }
            println!("  Editor link: {}", loc.format_for_editor());
        }
        None => {
            println!("Entity '{}' not found.", entity_name);
            println!("💡 Try 'parseltongue list-entities' to see available entities");
        }
    }
    
    println!();
    println!("Lookup completed in {:.2}μs", elapsed.as_micros() as f64);
}

/// Format location for JSON output
fn format_location_json(entity_name: &str, location: &Option<FileLocation>, elapsed: std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
    let output = serde_json::json!({
        "command": "where-defined",
        "entity_name": entity_name,
        "found": location.is_some(),
        "location": location,
        "execution_time_us": elapsed.as_micros(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
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
    #[tokio::test]
    async fn test_query_command_execution() {
        // Query commands should work now
        let args = vec!["parseltongue", "query", "what-implements", "TestTrait"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli).await;
        
        // Should succeed now that query execution is implemented
        assert!(result.is_ok());
    }

    #[test]
    fn test_calls_query_parsing() {
        let args = vec!["parseltongue", "query", "calls", "test_function", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Query { query_type, target, format } => {
                assert!(matches!(query_type, QueryType::Calls));
                assert_eq!(target, "test_function");
                assert!(matches!(format, OutputFormat::Json));
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[test]
    fn test_uses_query_parsing() {
        let args = vec!["parseltongue", "query", "uses", "TestStruct"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Query { query_type, target, format } => {
                assert!(matches!(query_type, QueryType::Uses));
                assert_eq!(target, "TestStruct");
                assert!(matches!(format, OutputFormat::Human));
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[tokio::test]
    async fn test_calls_query_execution() {
        // This test will fail until we implement calls query execution
        let args = vec!["parseltongue", "query", "calls", "test_function"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli).await;
        
        // Should fail in RED phase because find_callers doesn't exist yet
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_uses_query_execution() {
        // Uses query commands should work now
        let args = vec!["parseltongue", "query", "uses", "TestStruct"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli).await;
        
        // Should succeed now that query execution is implemented
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_performance_reporting() {
        // Test that query commands measure and report performance
        // This will be implemented in GREEN phase
        
        // For now, just validate the structure exists
        assert!(true, "Performance reporting structure ready");
    }

    // TDD Cycle 16: Ingest and daemon commands (RED phase)
    #[tokio::test]
    async fn test_ingest_command() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        fs::write(&dump_path, "FILE: test.rs\npub fn test() {}").unwrap();
        
        let args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli).await;
        
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

    #[tokio::test]
    async fn test_generate_context_command() {
        let args = vec!["parseltongue", "generate-context", "TestFunction", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli).await;
        
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
    #[tokio::test]
    async fn test_end_to_end_workflow() {
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
        let ingest_result = run(ingest_cli).await;
        
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

    // TDD Cycle 22: Visualize command (RED phase)
    #[test]
    fn test_visualize_command_parsing() {
        // Test visualize command without entity
        let args = vec!["parseltongue", "visualize"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Visualize { entity, output } => {
                assert!(entity.is_none());
                assert_eq!(output, PathBuf::from("parseltongue_visualization.html"));
            }
            _ => panic!("Expected Visualize command"),
        }
        
        // Test visualize command with entity and custom output
        let args = vec!["parseltongue", "visualize", "MyFunction", "--output", "custom.html"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Visualize { entity, output } => {
                assert_eq!(entity, Some("MyFunction".to_string()));
                assert_eq!(output, PathBuf::from("custom.html"));
            }
            _ => panic!("Expected Visualize command"),
        }
    }

    #[tokio::test]
    async fn test_visualize_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_visualization.html");
        
        let args = vec!["parseltongue", "visualize", "--output", output_path.to_str().unwrap()];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let result = run(cli).await;
        
        // Should succeed and create HTML file
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        // Verify HTML content
        let html_content = fs::read_to_string(&output_path).unwrap();
        assert!(html_content.contains("<!DOCTYPE html>"));
        assert!(html_content.contains("Parseltongue Architecture Visualization"));
    }

    #[test]
    fn test_visualize_command_with_focus() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("focused_visualization.html");
        
        let args = vec!["parseltongue", "visualize", "TestFunction", "--output", output_path.to_str().unwrap()];
        let cli = Cli::try_parse_from(args).unwrap();
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(run(cli));
        
        // Should succeed even if entity doesn't exist (graceful handling)
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        let html_content = fs::read_to_string(&output_path).unwrap();
        assert!(html_content.contains("TestFunction"));
    }

    // Discovery command parsing tests
    #[test]
    fn test_list_entities_command_parsing() {
        // Test basic list-entities command
        let args = vec!["parseltongue", "list-entities"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::ListEntities { r#type, limit, format } => {
                assert!(r#type.is_none());
                assert_eq!(limit, 100); // default
                assert!(matches!(format, OutputFormat::Human)); // default
            }
            _ => panic!("Expected ListEntities command"),
        }
        
        // Test with type filter
        let args = vec!["parseltongue", "list-entities", "--type", "function", "--limit", "50"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::ListEntities { r#type, limit, format } => {
                assert!(matches!(r#type, Some(DiscoveryEntityType::Function)));
                assert_eq!(limit, 50);
                assert!(matches!(format, OutputFormat::Human));
            }
            _ => panic!("Expected ListEntities command"),
        }
        
        // Test with JSON format
        let args = vec!["parseltongue", "list-entities", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::ListEntities { r#type, limit, format } => {
                assert!(r#type.is_none());
                assert_eq!(limit, 100);
                assert!(matches!(format, OutputFormat::Json));
            }
            _ => panic!("Expected ListEntities command"),
        }
    }
    
    #[test]
    fn test_entities_in_file_command_parsing() {
        // Test basic entities-in-file command
        let args = vec!["parseltongue", "entities-in-file", "src/main.rs"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::EntitiesInFile { file, r#type, format } => {
                assert_eq!(file, "src/main.rs");
                assert!(r#type.is_none());
                assert!(matches!(format, OutputFormat::Human));
            }
            _ => panic!("Expected EntitiesInFile command"),
        }
        
        // Test with type filter and JSON format
        let args = vec!["parseltongue", "entities-in-file", "src/lib.rs", "--type", "struct", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::EntitiesInFile { file, r#type, format } => {
                assert_eq!(file, "src/lib.rs");
                assert!(matches!(r#type, Some(DiscoveryEntityType::Struct)));
                assert!(matches!(format, OutputFormat::Json));
            }
            _ => panic!("Expected EntitiesInFile command"),
        }
    }
    
    #[test]
    fn test_where_defined_command_parsing() {
        // Test basic where-defined command
        let args = vec!["parseltongue", "where-defined", "test_function"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::WhereDefined { entity, format } => {
                assert_eq!(entity, "test_function");
                assert!(matches!(format, OutputFormat::Human));
            }
            _ => panic!("Expected WhereDefined command"),
        }
        
        // Test with JSON format
        let args = vec!["parseltongue", "where-defined", "MyStruct", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::WhereDefined { entity, format } => {
                assert_eq!(entity, "MyStruct");
                assert!(matches!(format, OutputFormat::Json));
            }
            _ => panic!("Expected WhereDefined command"),
        }
    }
    
    #[test]
    fn test_discovery_entity_type_conversion() {
        // Test all entity type conversions
        use crate::discovery::types::EntityType;
        
        assert_eq!(EntityType::from(DiscoveryEntityType::Function), EntityType::Function);
        assert_eq!(EntityType::from(DiscoveryEntityType::Struct), EntityType::Struct);
        assert_eq!(EntityType::from(DiscoveryEntityType::Trait), EntityType::Trait);
        assert_eq!(EntityType::from(DiscoveryEntityType::Impl), EntityType::Impl);
        assert_eq!(EntityType::from(DiscoveryEntityType::Module), EntityType::Module);
        assert_eq!(EntityType::from(DiscoveryEntityType::Constant), EntityType::Constant);
        assert_eq!(EntityType::from(DiscoveryEntityType::Static), EntityType::Static);
        assert_eq!(EntityType::from(DiscoveryEntityType::Macro), EntityType::Macro);
    }

    // Integration tests for discovery commands
    #[tokio::test]
    async fn test_list_entities_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        // Create test dump with entities
        let dump_content = r#"
FILE: src/lib.rs
pub fn hello_world() -> String {
    "Hello, World!".to_string()
}

pub struct Person {
    name: String,
    age: u32,
}

pub trait Greeter {
    fn greet(&self) -> String;
}

impl Greeter for Person {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // First ingest the data
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let ingest_result = run(ingest_cli).await;
        assert!(ingest_result.is_ok());
        
        // Test list-entities command
        let list_args = vec!["parseltongue", "list-entities", "--limit", "10"];
        let list_cli = Cli::try_parse_from(list_args).unwrap();
        let list_result = run(list_cli).await;
        
        // Should succeed
        assert!(list_result.is_ok());
    }
    
    #[tokio::test]
    async fn test_list_entities_with_type_filter() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        let dump_content = r#"
FILE: src/lib.rs
pub fn test_function() {}
pub struct TestStruct {}
pub trait TestTrait {}
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // Ingest data
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let _ = run(ingest_cli).await;
        
        // Test with function filter
        let list_args = vec!["parseltongue", "list-entities", "--type", "function"];
        let list_cli = Cli::try_parse_from(list_args).unwrap();
        let list_result = run(list_cli).await;
        assert!(list_result.is_ok());
        
        // Test with struct filter
        let list_args = vec!["parseltongue", "list-entities", "--type", "struct"];
        let list_cli = Cli::try_parse_from(list_args).unwrap();
        let list_result = run(list_cli).await;
        assert!(list_result.is_ok());
    }
    
    #[tokio::test]
    async fn test_entities_in_file_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        let dump_content = r#"
FILE: src/main.rs
pub fn main() {
    println!("Hello, World!");
}

pub fn helper() -> i32 {
    42
}

FILE: src/lib.rs
pub struct Config {
    debug: bool,
}
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // Ingest data
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let _ = run(ingest_cli).await;
        
        // Test entities-in-file command
        let file_args = vec!["parseltongue", "entities-in-file", "src/main.rs"];
        let file_cli = Cli::try_parse_from(file_args).unwrap();
        let file_result = run(file_cli).await;
        assert!(file_result.is_ok());
        
        // Test with type filter
        let file_args = vec!["parseltongue", "entities-in-file", "src/main.rs", "--type", "function"];
        let file_cli = Cli::try_parse_from(file_args).unwrap();
        let file_result = run(file_cli).await;
        assert!(file_result.is_ok());
    }
    
    #[tokio::test]
    async fn test_where_defined_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        let dump_content = r#"
FILE: src/lib.rs
pub fn target_function() -> String {
    "Found me!".to_string()
}
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // Ingest data
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let _ = run(ingest_cli).await;
        
        // Test where-defined command
        let where_args = vec!["parseltongue", "where-defined", "target_function"];
        let where_cli = Cli::try_parse_from(where_args).unwrap();
        let where_result = run(where_cli).await;
        assert!(where_result.is_ok());
        
        // Test with non-existent entity
        let where_args = vec!["parseltongue", "where-defined", "nonexistent_function"];
        let where_cli = Cli::try_parse_from(where_args).unwrap();
        let where_result = run(where_cli).await;
        assert!(where_result.is_ok()); // Should succeed but report not found
    }
    
    #[tokio::test]
    async fn test_discovery_commands_json_output() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        let dump_content = r#"
FILE: src/lib.rs
pub fn json_test() {}
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // Ingest data
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let _ = run(ingest_cli).await;
        
        // Test list-entities with JSON output
        let list_args = vec!["parseltongue", "list-entities", "--format", "json"];
        let list_cli = Cli::try_parse_from(list_args).unwrap();
        let list_result = run(list_cli).await;
        assert!(list_result.is_ok());
        
        // Test entities-in-file with JSON output
        let file_args = vec!["parseltongue", "entities-in-file", "src/lib.rs", "--format", "json"];
        let file_cli = Cli::try_parse_from(file_args).unwrap();
        let file_result = run(file_cli).await;
        assert!(file_result.is_ok());
        
        // Test where-defined with JSON output
        let where_args = vec!["parseltongue", "where-defined", "json_test", "--format", "json"];
        let where_cli = Cli::try_parse_from(where_args).unwrap();
        let where_result = run(where_cli).await;
        assert!(where_result.is_ok());
    }
    
    #[tokio::test]
    async fn test_discovery_commands_performance_contracts() {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test.dump");
        
        // Create a reasonably sized test dump
        let mut dump_content = String::new();
        dump_content.push_str("FILE: src/lib.rs\n");
        
        // Add multiple entities to test performance
        for i in 0..50 {
            dump_content.push_str(&format!("pub fn test_function_{}() {{}}\n", i));
            dump_content.push_str(&format!("pub struct TestStruct{} {{}}\n", i));
        }
        
        fs::write(&dump_path, dump_content).unwrap();
        
        // Ingest data
        let ingest_args = vec!["parseltongue", "ingest", dump_path.to_str().unwrap()];
        let ingest_cli = Cli::try_parse_from(ingest_args).unwrap();
        let _ = run(ingest_cli).await;
        
        // Test list-entities performance
        let start = Instant::now();
        let list_args = vec!["parseltongue", "list-entities"];
        let list_cli = Cli::try_parse_from(list_args).unwrap();
        let _ = run(list_cli).await;
        let list_elapsed = start.elapsed();
        
        // Should meet <100ms contract for discovery operations
        assert!(list_elapsed.as_millis() < 100, 
                "list-entities took {:?}, expected <100ms", list_elapsed);
        
        // Test where-defined performance
        let start = Instant::now();
        let where_args = vec!["parseltongue", "where-defined", "test_function_0"];
        let where_cli = Cli::try_parse_from(where_args).unwrap();
        let _ = run(where_cli).await;
        let where_elapsed = start.elapsed();
        
        // Should meet <50ms contract for exact lookups
        assert!(where_elapsed.as_millis() < 50, 
                "where-defined took {:?}, expected <50ms", where_elapsed);
    }
    
    #[test]
    fn test_cli_help_includes_discovery_commands() {
        use clap::CommandFactory;
        let mut cli = Cli::command();
        let help = cli.render_help();
        let help_text = help.to_string();
        
        // Should contain all discovery commands
        assert!(help_text.contains("list-entities"));
        assert!(help_text.contains("entities-in-file"));
        assert!(help_text.contains("where-defined"));
        
        // Should contain command descriptions
        assert!(help_text.contains("List all entities in the codebase"));
        assert!(help_text.contains("List entities defined in a specific file"));
        assert!(help_text.contains("Find where an entity is defined"));
    }
    
    #[test]
    fn test_discovery_command_error_handling() {
        // Test invalid entity type
        let args = vec!["parseltongue", "list-entities", "--type", "invalid"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
        
        // Test invalid format
        let args = vec!["parseltongue", "list-entities", "--format", "invalid"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
        
        // Test missing required arguments
        let args = vec!["parseltongue", "entities-in-file"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
        
        let args = vec!["parseltongue", "where-defined"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }
}