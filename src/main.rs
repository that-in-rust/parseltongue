//! Parseltongue AIM Daemon - Main CLI Entry Point

use clap::Parser;
use parseltongue::cli::Cli;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = parseltongue::cli::run(cli).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}