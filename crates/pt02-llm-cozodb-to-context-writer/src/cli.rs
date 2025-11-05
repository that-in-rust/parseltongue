//! Command-line interface for pt02-llm-cozodb-to-context-writer.
//!
//! # CLI Architecture: Level-Based Progressive Disclosure
//!
//! **Level 0**: Pure edge list (--where only, NO --include-code)
//! **Level 1**: Node-centric + ISG + Temporal (--include-code required)
//! **Level 2**: + Type system (--include-code required)
//!
//! ## Datalog WHERE Clause Syntax
//!
//! **CRITICAL**: All `--where` clauses use **Datalog syntax** (CozoDB native), NOT SQL!
//!
//! | SQL (WRONG) | Datalog (CORRECT) |
//! |-------------|-------------------|
//! | `x = 5 AND y = 10` | `x = 5, y = 10` |
//! | `x = 5 OR y = 10` | `x = 5; y = 10` |
//! | `x == 5` | `x = 5` |
//!
//! ## Examples
//!
//! ```bash
//! # Level 0: Pure edge list
//! pt02-level00 --where "ALL"
//!
//! # Level 1: Entity export, signatures only
//! pt02-level01 --include-code 0 --where "is_public = true, entity_type = 'fn'"
//!
//! # Level 2: With type system, full code
//! pt02-level02 --include-code 1 --where "entity_type = 'fn', is_async = true"
//! ```

use clap::Parser;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

use crate::models::ExportConfig;

/// PT02: Export entity graphs from CozoDB to JSON
///
/// Supports 3 export levels with progressive disclosure:
/// - Level 0: Pure edge list (minimal - ~2-5K tokens)
/// - Level 1: Node-centric + ISG + Temporal (~30K tokens)
/// - Level 2: + Type system essentials (~60K tokens)
#[derive(Parser, Debug)]
#[command(name = "pt02-llm-cozodb-to-context-writer")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Export level (0=edges, 1=entities+ISG, 2=+types)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=2))]
    pub level: u8,

    /// Include current_code field: 0=signatures only, 1=with code
    ///
    /// MANDATORY for Level 1-2 (N/A for Level 0)
    ///
    /// Cost impact:
    /// - 0 (signatures): ~5-60K tokens depending on level
    /// - 1 (with code): ~500-700K tokens (100× more expensive)
    #[arg(
        long,
        value_parser = clap::value_parser!(u8).range(0..=1),
        required_if_eq("level", "1"),
        required_if_eq("level", "2")
    )]
    pub include_code: Option<u8>,

    /// Datalog WHERE clause or "ALL" (MANDATORY)
    ///
    /// IMPORTANT: Uses Datalog syntax (CozoDB native), NOT SQL!
    ///
    /// Examples:
    ///   --where "ALL"
    ///   --where "is_public = true, entity_type = 'fn'"
    ///   --where "line_number > 100, line_number < 500"
    ///
    /// Datalog syntax:
    ///   - AND: Use comma (,)     NOT &&
    ///   - OR: Use semicolon (;)  NOT ||
    ///   - Equality: Use =        NOT ==
    #[arg(long)]
    pub where_clause: String,

    /// Output JSON file path
    ///
    /// Defaults: ISGLevel00.json, ISGLevel01.json, ISGLevel02.json
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Database file path
    #[arg(long, default_value = "parseltongue.db")]
    pub db: String,

    /// Output format: json (default) or toon (40% token reduction)
    ///
    /// TOON format provides 41.9% token savings for LLM consumption.
    /// - json: Standard JSON format (compatible with all tools)
    /// - toon: Tab-delimited format optimized for LLMs
    ///
    /// Example: --format toon --toon-delimiter tab
    #[arg(long, default_value = "json")]
    pub format: String,

    /// TOON delimiter: tab (recommended), comma, pipe
    ///
    /// Only applicable when --format toon is used.
    /// Tab delimiter provides best LLM parsing performance.
    #[arg(long, default_value = "tab")]
    pub toon_delimiter: String,

    /// Verbose output (show progress, token estimates)
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    /// Validate CLI arguments and create ExportConfig
    ///
    /// # Validation Rules
    ///
    /// 1. Level 0: Must NOT have --include-code (edges only)
    /// 2. Level 1-2: Must HAVE --include-code (entities need code flag)
    /// 3. WHERE clause: Must be non-empty string
    ///
    /// # Returns
    ///
    /// `ExportConfig` ready for export operations
    ///
    /// # Errors
    ///
    /// - Level 0 with --include-code: "Level 0 exports edges only, --include-code not applicable"
    /// - Level 1-2 without --include-code: "Level N requires --include-code [0|1]"
    /// - Empty WHERE clause: "WHERE clause cannot be empty"
    pub fn validate(&self) -> Result<ExportConfig> {
        // Validate Level 0: Should NOT have include_code
        if self.level == 0 && self.include_code.is_some() {
            return Err(anyhow!(
                "Level 0 exports edges only (from_key, to_key, edge_type). \
                 --include-code flag not applicable. \
                 Remove --include-code for Level 0."
            ));
        }

        // Validate Level 1-2: Must HAVE include_code
        if self.level > 0 && self.include_code.is_none() {
            return Err(anyhow!(
                "Level {} exports entities which may include code. \
                 --include-code [0|1] is MANDATORY. \
                 Use 0 for signatures only (cheap), 1 for full code (expensive).",
                self.level
            ));
        }

        // Validate WHERE clause non-empty
        if self.where_clause.trim().is_empty() {
            return Err(anyhow!(
                "WHERE clause cannot be empty. \
                 Use --where \"ALL\" to export all entities/edges."
            ));
        }

        // Validate format
        let format_lower = self.format.to_lowercase();
        if format_lower != "json" && format_lower != "toon" {
            return Err(anyhow!(
                "Invalid format '{}'. Must be 'json' or 'toon'.",
                self.format
            ));
        }

        // Validate TOON delimiter
        let delimiter_lower = self.toon_delimiter.to_lowercase();
        if format_lower == "toon" && !["tab", "comma", "pipe"].contains(&delimiter_lower.as_str()) {
            return Err(anyhow!(
                "Invalid TOON delimiter '{}'. Must be 'tab', 'comma', or 'pipe'.",
                self.toon_delimiter
            ));
        }

        // Build config
        let extension = if format_lower == "toon" { "toon" } else { "json" };

        Ok(ExportConfig {
            level: self.level,
            include_code: self.include_code.map(|v| v == 1).unwrap_or(false),
            where_filter: self.where_clause.clone(),
            output_path: self.output.clone().unwrap_or_else(|| {
                PathBuf::from(format!("ISGLevel{:02}.{}", self.level, extension))
            }),
            // v0.9.0: Dual outputs for code/test separation (None for general CLI)
            code_output_path: None,
            tests_output_path: None,
            db_path: self.db.clone(),
        })
    }

    /// Get TOON delimiter from CLI argument
    pub fn get_toon_delimiter(&self) -> crate::ToonDelimiter {
        match self.toon_delimiter.to_lowercase().as_str() {
            "comma" => crate::ToonDelimiter::Comma,
            "pipe" => crate::ToonDelimiter::Pipe,
            _ => crate::ToonDelimiter::Tab, // Default to tab
        }
    }

    /// Check if TOON format is selected
    pub fn is_toon_format(&self) -> bool {
        self.format.to_lowercase() == "toon"
    }

    /// Print verbose output if enabled
    pub fn verbose_print(&self, message: &str) {
        if self.verbose {
            eprintln!("[PT02] {}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level0_minimal() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "0",
            "--where-clause", "ALL",
        ]);

        assert_eq!(cli.level, 0);
        assert!(cli.include_code.is_none());
        assert_eq!(cli.where_clause, "ALL");

        let config = cli.validate();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.level, 0);
        assert_eq!(config.include_code, false);
    }

    #[test]
    fn test_level0_with_include_code_fails() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "0",
            "--include-code", "0",
            "--where-clause", "ALL",
        ]);

        let result = cli.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Level 0 exports edges only"));
    }

    #[test]
    fn test_level1_without_include_code_fails() {
        // This should fail at clap parsing level due to required_if_eq
        let result = Cli::try_parse_from(&[
            "pt02",
            "--level", "1",
            "--where-clause", "ALL",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn test_level1_signatures_only() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "1",
            "--include-code", "0",
            "--where-clause", "ALL",
        ]);

        let config = cli.validate().unwrap();
        assert_eq!(config.level, 1);
        assert_eq!(config.include_code, false);  // 0 = signatures only
        assert_eq!(config.where_filter, "ALL");
    }

    #[test]
    fn test_level1_with_code() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "1",
            "--include-code", "1",
            "--where-clause", "future_action != null",
        ]);

        let config = cli.validate().unwrap();
        assert_eq!(config.level, 1);
        assert_eq!(config.include_code, true);  // 1 = with code
        assert_eq!(config.where_filter, "future_action != null");
    }

    #[test]
    fn test_level2_with_complex_where() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "2",
            "--include-code", "0",
            "--where-clause", "is_public = true, entity_type = 'fn', is_async = true",
        ]);

        let config = cli.validate().unwrap();
        assert_eq!(config.level, 2);
        assert_eq!(config.include_code, false);
        assert!(config.where_filter.contains("is_public = true"));
        assert!(config.where_filter.contains("entity_type = 'fn'"));
    }

    #[test]
    fn test_output_path_default() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "1",
            "--include-code", "0",
            "--where-clause", "ALL",
        ]);

        let config = cli.validate().unwrap();
        assert_eq!(config.output_path, PathBuf::from("ISGLevel01.json"));
    }

    #[test]
    fn test_output_path_custom() {
        let cli = Cli::parse_from(&[
            "pt02",
            "--level", "1",
            "--include-code", "0",
            "--where-clause", "ALL",
            "--output", "custom.json",
        ]);

        let config = cli.validate().unwrap();
        assert_eq!(config.output_path, PathBuf::from("custom.json"));
    }

    #[test]
    fn test_empty_where_clause_fails() {
        let cli = Cli {
            level: 1,
            include_code: Some(0),
            where_clause: "".to_string(),  // Empty!
            output: None,
            db: "test.db".to_string(),
            verbose: false,
        };

        let result = cli.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("WHERE clause cannot be empty"));
    }
}
