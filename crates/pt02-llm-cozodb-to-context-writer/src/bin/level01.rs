//! PT02 Level 1: Entity + ISG + Temporal Exporter
//!
//! # Usage
//!
//! ```bash
//! # Export all entities (signatures only)
//! pt02-level01 --include-code 0 --where "ALL" --output entities.json
//!
//! # Export public functions with full code
//! pt02-level01 --include-code 1 --where "is_public = true, entity_type = 'fn'" --output public_fns.json
//!
//! # Export entities with planned changes (temporal)
//! pt02-level01 --include-code 0 --where "future_action != null" --output changes.json
//! ```
//!
//! ## Level 1 Design
//!
//! - **Output**: Entities with ISG + temporal state
//! - **Token estimate**: ~30K tokens (signatures), ~500K+ (with code)
//! - **Use case**: Code understanding, refactoring planning
//! - **--include-code flag**: 0 = signatures only (cheap), 1 = full code (expensive)

use anyhow::Result;
use clap::Parser;
use pt02_llm_cozodb_to_context_writer::{
    exporters::{Level1Exporter, ToonLevel1Exporter},
    export_trait::LevelExporter,
    models::ExportConfig,
};
use std::path::PathBuf;

/// PT02 Level 1: Export entities with ISG + temporal state
#[derive(Parser, Debug)]
#[command(name = "pt02-level01")]
#[command(version, about = "Export entities with ISG + temporal (Level 1)", long_about = None)]
struct Cli {
    /// Include current_code field: 0=signatures only, 1=with code
    ///
    /// MANDATORY for Level 1
    ///
    /// Cost impact:
    /// - 0 (signatures): ~30K tokens (cheap)
    /// - 1 (with code): ~500-700K tokens (expensive - 100× more)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=1))]
    include_code: u8,

    /// Datalog WHERE clause or "ALL" (MANDATORY)
    ///
    /// Examples:
    ///   --where "ALL"
    ///   --where "is_public = true, entity_type = 'fn'"
    ///   --where "future_action != null"
    ///
    /// Datalog syntax:
    ///   - AND: Use comma (,)     NOT &&
    ///   - OR: Use semicolon (;)  NOT ||
    ///   - Equality: Use =        NOT ==
    #[arg(long)]
    where_clause: String,

    /// Output JSON file path
    ///
    /// Default: ISGLevel01.json
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Database file path
    #[arg(long, default_value = "parseltongue.db")]
    db: String,

    /// Output format: json (default) or toon (41.9% token reduction)
    ///
    /// TOON format provides 41.9% token savings for LLM consumption.
    /// - json: Standard JSON format (compatible with all tools)
    /// - toon: Tab-delimited format optimized for LLMs
    #[arg(long, default_value = "json")]
    format: String,

    /// TOON delimiter: tab (recommended), comma, pipe
    ///
    /// Only applicable when --format toon is used.
    #[arg(long, default_value = "tab")]
    toon_delimiter: String,

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

        // Validate format
        let format_lower = self.format.to_lowercase();
        if format_lower != "json" && format_lower != "toon" {
            return Err(anyhow::anyhow!(
                "Invalid format '{}'. Must be 'json' or 'toon'.",
                self.format
            ));
        }

        // Build config
        let extension = if format_lower == "toon" { "toon" } else { "json" };

        Ok(ExportConfig {
            level: 1,
            include_code: self.include_code == 1,
            where_filter: self.where_clause.clone(),
            output_path: self.output.clone().unwrap_or_else(|| {
                PathBuf::from(format!("ISGLevel01.{}", extension))
            }),
            // v0.9.0: Dual outputs for code/test separation (None for level01)
            code_output_path: None,
            tests_output_path: None,
            db_path: self.db.clone(),
        })
    }

    fn is_toon_format(&self) -> bool {
        self.format.to_lowercase() == "toon"
    }

    fn get_toon_delimiter(&self) -> pt02_llm_cozodb_to_context_writer::ToonDelimiter {
        use pt02_llm_cozodb_to_context_writer::ToonDelimiter;
        match self.toon_delimiter.to_lowercase().as_str() {
            "comma" => ToonDelimiter::Comma,
            "pipe" => ToonDelimiter::Pipe,
            _ => ToonDelimiter::Tab,
        }
    }

    fn verbose_print(&self, message: &str) {
        if self.verbose {
            eprintln!("[PT02-L1] {}", message);
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
    cli.verbose_print("Starting PT02 Level 1 export (Entity + ISG + Temporal)");
    cli.verbose_print(&format!("Database: {}", config.db_path));
    cli.verbose_print(&format!("Output: {:?}", config.output_path));
    cli.verbose_print(&format!("WHERE filter: {}", config.where_filter));
    cli.verbose_print(&format!(
        "Include code: {}",
        if config.include_code { "YES (expensive)" } else { "NO (cheap)" }
    ));

    // Connect to CozoDB
    cli.verbose_print("Connecting to CozoDB...");
    let db = pt02_llm_cozodb_to_context_writer::CozoDbAdapter::connect(&config.db_path).await?;
    cli.verbose_print("✅ Connected to CozoDB");

    // Create format-specific exporter and export
    if cli.is_toon_format() {
        // TOON format (41.9% token reduction)
        let delimiter = cli.get_toon_delimiter();
        let exporter = ToonLevel1Exporter::new(delimiter);

        let base_tokens = exporter.estimated_tokens();
        let estimated = if config.include_code {
            base_tokens * 20  // Code adds ~20× tokens
        } else {
            base_tokens
        };

        cli.verbose_print(&format!("Format: TOON (41.9% more efficient than JSON)"));
        cli.verbose_print(&format!("Delimiter: {:?}", delimiter));
        cli.verbose_print(&format!("Estimated tokens: ~{} (vs ~{} JSON)", estimated, estimated * 2));

        // Execute export
        cli.verbose_print("Exporting entities to TOON format...");
        let output = exporter.export(&db, &config).await?;

        println!("✅ PT02 Level 1 TOON export completed!");
        println!("   Output: {:?}", config.output_path);
        println!("   Entities exported: {}", output.export_metadata.total_entities.unwrap_or(0));
        println!("   Token savings: 41.9% vs JSON");
    } else {
        // JSON format (default)
        let exporter = Level1Exporter::new();

        let base_tokens = exporter.estimated_tokens();
        let estimated = if config.include_code {
            base_tokens * 20
        } else {
            base_tokens
        };

        cli.verbose_print(&format!("Format: JSON (standard)"));
        cli.verbose_print(&format!("Estimated tokens: ~{}", estimated));

        // Execute export
        cli.verbose_print("Exporting entities to JSON format...");
        let output = exporter.export(&db, &config).await?;

        println!("✅ PT02 Level 1 JSON export completed!");
        println!("   Output: {:?}", config.output_path);
        println!("   Entities exported: {}", output.export_metadata.total_entities.unwrap_or(0));
    }

    Ok(())
}
