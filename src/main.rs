//! CLI Binary - Pyramidal Structure
//! Layer 1: CLI Interface & Arguments
//! Layer 2: Application Setup
//! Layer 3: Core Processing
//! Layer 4: Error Handling
//! Layer 5: Support Functions

use anyhow::{Context, Result};
use tracing::{error, info, Level};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

use parseltongue::main::Application;

// Layer 1: Main Entry Point
#[tokio::main]
async fn main() -> Result<()> {
    // Layer 2: Setup
    setup_logging()
        .context("Failed to initialize logging")?;
    
    info!("Starting ZIP processor v{}", parseltongue::VERSION);

    // Layer 3: Application Lifecycle
    let app = Application::new().await
        .context("Failed to initialize application")?;

    // Layer 4: Error Handling
    if let Err(e) = app.run().await {
        app.handle_error(e.clone()).await;
        error!("Application error: {:#}", e);
        app.shutdown().await?;
        std::process::exit(1);
    }

    // Layer 5: Cleanup
    app.shutdown().await
        .context("Failed to shutdown cleanly")?;

    info!("Processing completed successfully");
    Ok(())
}

// Support Functions
fn setup_logging() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .try_init()
        .context("Failed to initialize logging subscriber")?;

    Ok(())
}
