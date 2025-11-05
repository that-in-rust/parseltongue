//! Wrapper Binary: pt07-visual-analytics-terminal
//!
//! Orchestrates all visualization binaries and runs them in sequence.
//! This is the main entry point spawned by pt01 after code ingestion.
//!
//! ## Usage
//! ```bash
//! pt07-visual-analytics-terminal --db parseltongue.db
//! pt07-visual-analytics-terminal --db parseltongue.db --include-tests
//! ```
//!
//! ## Configurable Defaults
//! The DEFAULT_VISUALIZATIONS const defines which visualizations run.
//! Add/remove/reorder visualizations by editing this array.

use anyhow::{Context, Result};
use clap::Parser;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Parser, Debug)]
#[command(name = "pt07-visual-analytics-terminal")]
#[command(about = "Run all visual analytics on CozoDB after code ingestion")]
struct Args {
    /// Path to CozoDB database file
    #[arg(long)]
    db: String,

    /// Include test entities (default: implementation-only)
    #[arg(long, default_value_t = false)]
    include_tests: bool,
}

/// Configurable default visualizations
///
/// Each entry is the binary name (without path).
/// Add/remove/reorder to customize which visualizations run by default.
const DEFAULT_VISUALIZATIONS: &[&str] = &[
    "pt07-render-entity-count-bar-chart",
    "pt07-render-dependency-cycle-warning-list",
];

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    eprintln!("🚀 Running Parseltongue Visual Analytics...\n");

    // Build common arguments for all visualization binaries
    let mut common_args = vec!["--db".to_string(), args.db.clone()];
    if args.include_tests {
        common_args.push("--include-tests".to_string());
    }

    // Run each visualization in sequence
    for (idx, viz_name) in DEFAULT_VISUALIZATIONS.iter().enumerate() {
        eprintln!("[{}/{}] Running {}...", idx + 1, DEFAULT_VISUALIZATIONS.len(), viz_name);

        // Spawn visualization binary
        let output = Command::new(viz_name)
            .args(&common_args)
            .stdout(Stdio::inherit())  // Inherit stdout to show visualization
            .stderr(Stdio::inherit())  // Inherit stderr to show save confirmations
            .output()
            .await
            .context(format!("Failed to execute {}", viz_name))?;

        if !output.status.success() {
            eprintln!("⚠️  Warning: {} exited with status {}", viz_name, output.status);
        }

        eprintln!(); // Blank line between visualizations
    }

    eprintln!("✅ All visualizations complete!");
    eprintln!("📊 Check timestamped .txt files in current directory for results.");

    Ok(())
}
