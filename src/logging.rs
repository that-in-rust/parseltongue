// Level 4: Asynchronous Logging Configuration
// - Sets up logging with tracing and tracing_subscriber
// - Configures log file rotation and verbosity levels

use tracing_subscriber::EnvFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::path::Path;
use crate::config::Config;

pub fn init(config: &Config) -> Result<()> {
    // Level 3: Determine log level
    let env_filter = if config.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    // Level 2: Set up log file appender
    let file_appender = RollingFileAppender::new(Rotation::DAILY, &config.output_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Level 1: Initialize the subscriber
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_thread_names(true)
        .init();

    Ok(())
} 