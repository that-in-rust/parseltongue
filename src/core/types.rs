// Level 4: Core Types
// - Configuration
// - Shared types
// - Common traits

use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
    pub workers: usize,
    pub verbose: bool,
    pub buffer_size: usize,
    pub shutdown_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_zip: PathBuf::new(),
            output_dir: PathBuf::new(),
            workers: num_cpus::get(),
            verbose: false,
            buffer_size: 8192,
            shutdown_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeMetrics {
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub memory_usage: usize,
} 