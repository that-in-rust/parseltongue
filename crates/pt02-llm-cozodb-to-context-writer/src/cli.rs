//! Command-line interface for pt02-llm-cozodb-to-context-writer.
//!
//! # CLI Architecture (S01 Ultra-Minimalist + Dual Interface)
//!
//! **Dual Interface Design:**
//! - Simple Mode: `--include-current-code 0|1` + `--where "filter"` (95% of use cases)
//! - Advanced Mode: `--query "full datalog"` (5% of use cases, overrides simple mode)
//!
//! ## Examples
//!
//! ```bash
//! # Simple mode - composed query
//! pt02-llm-cozodb-to-context-writer-export-all-entities-json \
//!   --output context.json --db demo.db \
//!   --include-current-code 0 --where "ALL"
//!
//! # Advanced mode - raw Datalog
//! pt02-llm-cozodb-to-context-writer-export-all-entities-json \
//!   --output context.json --db demo.db \
//!   --query "?[isgl1_key, interface_signature] := *CodeGraph{isgl1_key, interface_signature}"
//! ```

use clap::{Arg, ArgGroup, Command};

/// CLI configuration builder
pub struct CliConfig;

impl CliConfig {
    /// Build CLI application with dual interface (Simple + Advanced)
    pub fn build_cli() -> Command {
        Command::new("pt02-llm-cozodb-to-context-writer")
            .version("0.8.2")
            .author("Parseltongue Team")
            .about("Tool 02: Export entity graphs from CozoDB to JSON")
            .long_about(
                "Ultra-minimalist database export tool with dual interface.\n\
                \n\
                SIMPLE MODE (95% of use cases):\n  \
                --include-current-code 0|1  (token optimization)\n  \
                --where 'filter'            (Datalog filter fragment)\n\
                \n\
                ADVANCED MODE (5% of use cases):\n  \
                --query 'full Datalog'      (complete query override)\n\
                \n\
                Examples:\n  \
                # Simple: Export all, signatures only (cheap)\n  \
                pt02-export-all-entities-json -o ctx.json --include-current-code 0 --where 'ALL'\n\
                \n  \
                # Simple: Export changed, with code (expensive)\n  \
                pt02-export-all-entities-json -o ctx.json --include-current-code 1 \\\n  \
                  --where 'future_action != null'\n\
                \n  \
                # Advanced: Custom query\n  \
                pt02-export-all-entities-json -o ctx.json \\\n  \
                  --query '?[isgl1_key, interface_signature] := *CodeGraph{isgl1_key, interface_signature}'\n\
                ",
            )
            // Common arguments (required for all modes)
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
            // Simple mode arguments (compose query for user)
            .arg(
                Arg::new("include-current-code")
                    .long("include-current-code")
                    .value_name("0|1")
                    .help("Include current_code field (0=signatures only [cheap], 1=with code [expensive])")
                    .value_parser(["0", "1"])
                    .conflicts_with("query"),
            )
            .arg(
                Arg::new("where")
                    .long("where")
                    .value_name("FILTER")
                    .help("Datalog WHERE filter fragment (default: 'ALL')")
                    .default_value("ALL")
                    .conflicts_with("query"),
            )
            // Advanced mode argument (full Datalog override)
            .arg(
                Arg::new("query")
                    .long("query")
                    .value_name("DATALOG")
                    .help("Full Datalog query (OVERRIDES --include-current-code and --where)")
                    .conflicts_with_all(["include-current-code", "where"]),
            )
            // Mutually exclusive groups
            .group(
                ArgGroup::new("interface_mode")
                    .args(["query", "include-current-code"])
                    .required(true)
                    .multiple(false),
            )
    }

    /// Parse interface mode from CLI arguments
    ///
    /// Returns: (query_string, use_advanced_mode)
    /// - Simple mode: Returns composed query + false
    /// - Advanced mode: Returns user query + true
    pub fn parse_interface_mode(matches: &clap::ArgMatches) -> (String, bool) {
        if let Some(query) = matches.get_one::<String>("query") {
            // Advanced mode: Use raw Datalog query
            (query.clone(), true)
        } else {
            // Simple mode: Compose query from include-current-code + where
            let include_code = matches
                .get_one::<String>("include-current-code")
                .map(|s| s == "1")
                .unwrap_or(false);

            let where_clause = matches
                .get_one::<String>("where")
                .map(|s| s.as_str())
                .unwrap_or("ALL");

            let query = crate::query_builder::build_export_query(include_code, where_clause);
            (query, false)
        }
    }

    /// Check if current_code should be included in output
    ///
    /// In simple mode: Based on --include-current-code flag
    /// In advanced mode: Parse query to detect current_code column (heuristic)
    pub fn should_include_code(matches: &clap::ArgMatches) -> bool {
        if let Some(query) = matches.get_one::<String>("query") {
            // Advanced mode: Check if query includes current_code
            query.contains("current_code")
        } else {
            // Simple mode: Use explicit flag
            matches
                .get_one::<String>("include-current-code")
                .map(|s| s == "1")
                .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mode_minimal() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "pt02",
            "--output", "test.json",
            "--include-current-code", "0",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();
        let (query, is_advanced) = CliConfig::parse_interface_mode(&matches);

        assert!(!is_advanced);
        assert!(query.contains("isgl1_key"));
        assert!(!query.contains("current_code"));
    }

    #[test]
    fn test_simple_mode_with_code() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "pt02",
            "--output", "test.json",
            "--include-current-code", "1",
            "--where", "future_action != null",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();
        let (query, is_advanced) = CliConfig::parse_interface_mode(&matches);

        assert!(!is_advanced);
        assert!(query.contains("current_code"));
        assert!(query.contains("future_action != null"));
    }

    #[test]
    fn test_advanced_mode() {
        let cli = CliConfig::build_cli();
        let matches = cli.try_get_matches_from(&[
            "pt02",
            "--output", "test.json",
            "--query", "?[isgl1_key] := *CodeGraph{isgl1_key}",
        ]);

        assert!(matches.is_ok());
        let matches = matches.unwrap();
        let (query, is_advanced) = CliConfig::parse_interface_mode(&matches);

        assert!(is_advanced);
        assert_eq!(query, "?[isgl1_key] := *CodeGraph{isgl1_key}");
    }

    #[test]
    fn test_mutual_exclusion() {
        let cli = CliConfig::build_cli();
        let result = cli.try_get_matches_from(&[
            "pt02",
            "--output", "test.json",
            "--include-current-code", "0",
            "--query", "?[isgl1_key] := *CodeGraph{isgl1_key}",
        ]);

        assert!(result.is_err());  // Should conflict
    }
}
