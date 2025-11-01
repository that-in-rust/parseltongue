//! Command-line interface for pt02-llm-cozodb-to-context-writer.
//!
//! # CLI Architecture (S01 Ultra-Minimalist)
//!
//! Following ultra-minimalist principles:
//! - NO LLM configuration (LLM runs externally)
//! - NO optimization settings (just dump the data)
//! - 3 arguments total: --output, --db, --filter
//!
//! ## Examples
//!
//! ```bash
//! # Export all entities
//! pt02-llm-cozodb-to-context-writer --output context.json
//!
//! # Export only changed entities
//! pt02-llm-cozodb-to-context-writer --output changes.json --filter changed
//!
//! # Export with specific database
//! pt02-llm-cozodb-to-context-writer --output out.json --db rocksdb:analysis.db
//! ```

use clap::{Arg, Command};

/// CLI configuration builder
pub struct CliConfig;

impl CliConfig {
    /// Build CLI application (S01: Ultra-minimalist - only 3 args)
    pub fn build_cli() -> Command {
        Command::new("pt02-llm-cozodb-to-context-writer")
            .version("0.8.1")
            .author("Parseltongue Team")
            .about("Tool 02: Export entity graphs from CozoDB to JSON")
            .long_about(
                "Ultra-minimalist database export tool.\n\
                \n\
                Exports CodeGraph entities from CozoDB to JSON format.\n\
                No LLM calls, no HTTP requests, just simple DB-to-JSON export.\n\
                \n\
                Examples:\n  \
                pt02-llm-cozodb-to-context-writer --output context.json\n  \
                pt02-llm-cozodb-to-context-writer --output changes.json --filter changed\n  \
                pt02-llm-cozodb-to-context-writer --output out.json --db rocksdb:analysis.db\n\
                ",
            )
            .arg(
                Arg::new("output")
                    .long("output")
                    .short('o')
                    .value_name("PATH")
                    .help("Output JSON file path")
                    .required(true),
            )
            .arg(
                Arg::new("db")
                    .long("db")
                    .value_name("PATH")
                    .help("Database file path")
                    .default_value("parseltongue.db"),
            )
            .arg(
                Arg::new("filter")
                    .long("filter")
                    .value_name("FILTER")
                    .help("Entity filter: all, changed, or current")
                    .value_parser(["all", "changed", "current"])
                    .default_value("all"),
            )
    }
}
