//! Main entry point for Tool 5: CozoDB State Reset
//!
//! This is the primary executable for the cozoDB-make-future-code-current tool.

use anyhow::Result;
use clap::Parser;
use miette::{IntoDiagnostic, WrapErr};

use parseltongue_06::cli::Tool5Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Tool5Cli::parse();

    // Execute the CLI command
    cli.execute().await
        .wrap_err("Failed to execute Tool 5 command")?;

    Ok(())
}