// Level 4: Core Types
// - Defines fundamental data structures
// - Manages configuration
// - Provides type safety
// - Enables extensibility

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
    pub verbose: bool,
    pub workers: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: u64,
}

#[derive(Debug, Clone)]
pub struct RuntimeMetrics {
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub memory_usage: usize,
} 