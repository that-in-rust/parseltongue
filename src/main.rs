// Level 4: Application Entry Point
// - CLI initialization
// - Runtime setup
// - Error handling
// - Metrics setup

use parseltongue::{
    cli::args::Args,
    core::{error::Result, runtime::RuntimeManager},
    metrics::report::MetricsReporter,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Level 3: Setup
    let args = Args::parse_args();
    let config = args.into_config()?;
    
    // Level 2: Runtime Init
    let runtime = RuntimeManager::new(config.worker_threads);
    let metrics = MetricsReporter::new("0.0.0.0:9000".parse().unwrap());
    
    // Level 1: Execution
    metrics.start().await?;
    runtime.run(config).await?;
    
    Ok(())
} 