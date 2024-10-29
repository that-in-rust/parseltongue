// Level 4: Asynchronous Logging Configuration
// - Initializes logging systems
// - Configures console and file logging
// - Integrates structured logging with trace contexts

use tracing_subscriber::{fmt, EnvFilter};
use crate::config::Config;
use crate::error::Result;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::path::Path;

pub fn init(config: &Config, log_dir: &Path) -> Result<()> {
    let env_filter = if config.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_thread_names(true)
        .init();

    Ok(())
} 