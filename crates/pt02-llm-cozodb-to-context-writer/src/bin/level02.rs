//! PT02 Level 2: Entity + ISG + Temporal + Type System Exporter
//!
//! # Usage
//!
//! ```bash
//! # Export all entities with type information (signatures only)
//! pt02-level02 --include-code 0 --where "ALL" --output typed_entities.json
//!
//! # Export async functions with full code
//! pt02-level02 --include-code 1 --where "is_async = true" --output async_fns.json
//!
//! # Export public API surface
//! pt02-level02 --include-code 0 --where "is_public = true" --output api.json
//!
//! # Find unsafe code
//! pt02-level02 --include-code 0 --where "is_unsafe = true" --output unsafe.json
//! ```
//!
//! ## Level 2 Design
//!
//! - **Output**: Entities with ISG + temporal + type system
//! - **Token estimate**: ~60K tokens (signatures), ~500K+ (with code)
//! - **Use case**: Type-safe refactoring, API analysis, safety audits
//! - **Type fields**: return_type, param_types, is_async, is_unsafe, trait_impls, etc.

use anyhow::Result;
use clap::Parser;
use pt02_llm_cozodb_to_context_writer::{
    exporters::Level2Exporter,
    export_trait::LevelExporter,
    models::ExportConfig,
};
use std::path::PathBuf;

/// PT02 Level 2: Export entities with type system information
#[derive(Parser, Debug)]
#[command(name = "pt02-level02")]
#[command(version, about = "Export entities with type system (Level 2)", long_about = None)]
struct Cli {
    /// Include current_code field: 0=signatures only, 1=with code
    ///
    /// MANDATORY for Level 2
    ///
    /// Cost impact:
    /// - 0 (signatures): ~60K tokens (cheap)
    /// - 1 (with code): ~500-700K tokens (expensive - 100× more)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=1))]
    include_code: u8,

    /// Datalog WHERE clause or "ALL" (MANDATORY)
    ///
    /// Examples:
    ///   --where "ALL"
    ///   --where "is_public = true"
    ///   --where "is_async = true"
    ///   --where "is_unsafe = true"
    ///   --where "entity_type = 'fn', is_public = true"
    ///
    /// Datalog syntax:
    ///   - AND: Use comma (,)     NOT &&
    ///   - OR: Use semicolon (;)  NOT ||
    ///   - Equality: Use =        NOT ==
    #[arg(long)]
    where_clause: String,

    /// Output JSON file path
    ///
    /// Default: ISGLevel02.json
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Database file path
    #[arg(long, default_value = "parseltongue.db")]
    db: String,

    /// Verbose output (show progress, token estimates)
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    fn validate(&self) -> Result<ExportConfig> {
        // Validate WHERE clause non-empty
        if self.where_clause.trim().is_empty() {
            return Err(anyhow::anyhow!(
                "WHERE clause cannot be empty. Use --where \"ALL\" to export all entities."
            ));
        }

        // Build config
        Ok(ExportConfig {
            level: 2,
            include_code: self.include_code == 1,
            where_filter: self.where_clause.clone(),
            output_path: self.output.clone().unwrap_or_else(|| {
                PathBuf::from("ISGLevel02.json")
            }),
            // v0.9.0: Dual outputs for code/test separation (None for level02)
            code_output_path: None,
            tests_output_path: None,
            db_path: self.db.clone(),
        })
    }

    fn verbose_print(&self, message: &str) {
        if self.verbose {
            eprintln!("[PT02-L2] {}", message);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Validate and get configuration
    let config = cli.validate()?;

    // Verbose logging
    cli.verbose_print("Starting PT02 Level 2 export (Entity + ISG + Temporal + Type System)");
    cli.verbose_print(&format!("Database: {}", config.db_path));
    cli.verbose_print(&format!("Output: {:?}", config.output_path));
    cli.verbose_print(&format!("WHERE filter: {}", config.where_filter));
    cli.verbose_print(&format!(
        "Include code: {}",
        if config.include_code { "YES (expensive)" } else { "NO (cheap)" }
    ));

    // Create exporter
    let exporter = Level2Exporter::new();
    let base_tokens = exporter.estimated_tokens();
    let estimated = if config.include_code {
        base_tokens * 15  // Rough estimate: code adds 15x tokens (slightly less than L1 due to more metadata)
    } else {
        base_tokens
    };
    cli.verbose_print(&format!("Estimated tokens: ~{}", estimated));

    // TODO: Connect to real CozoDB and export
    println!("PT02 Level 2 binary created successfully!");
    println!("TODO: Connect to CozoDB at {}", config.db_path);
    println!("TODO: Export entities with type info to {:?}", config.output_path);

    Ok(())
}
