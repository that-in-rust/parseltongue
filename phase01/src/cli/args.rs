//! CLI Argument Handling
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Argument Processing
//! - ArgumentProcessor (processes arguments)
//! - ConfigBuilder     (builds configuration)
//! - Validator         (validates arguments)
//! 
//! Level 3: Argument Types
//! - InputArgs        (input file arguments)
//! - OutputArgs       (output configuration)
//! - RuntimeArgs      (runtime settings)
//! 
//! Level 2: Argument Implementation
//! - ArgParser        (argument parsing)
//! - ArgValidator     (validation logic)
//! - ConfigMapper     (config mapping)
//! 
//! Level 1 (Base): Core Argument Types
//! - Args            (argument structure)
//! - ArgError        (argument errors)
//! - ValidationRule  (validation rules)

use std::path::PathBuf;
use clap::Parser;
use crate::core::{error::Result, types::*};

// ===== Level 1: Core Argument Types =====
// Design Choice: Using clap for argument parsing

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Input ZIP file path
    #[clap(short = 'i', long = "input-zip")]
    pub input_path: PathBuf,

    /// Output directory path
    #[clap(short = 'o', long = "output-dir")]
    pub output_dir: PathBuf,

    /// Number of worker threads
    #[clap(short = 'w', long = "workers", default_value = "4")]
    pub workers: usize,

    /// Buffer size in bytes
    #[clap(short = 'b', long = "buffer-size", default_value = "8192")]
    pub buffer_size: usize,

    /// Shutdown timeout in seconds
    #[clap(short = 's', long = "shutdown-timeout", default_value = "30")]
    pub shutdown_timeout: u64,

    /// Enable verbose output
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}

// ===== Level 2: Argument Implementation =====
// Design Choice: Using validation traits

impl Args {
    /// Validates command line arguments
    pub fn validate(&self) -> Result<()> {
        // Validate input path
        if !self.input_path.exists() {
            return Err(Error::InvalidPath(self.input_path.clone()));
        }

        // Validate worker count
        if self.workers == 0 {
            return Err(Error::ResourceLimit("Worker count must be > 0".into()));
        }

        // Validate buffer size
        if self.buffer_size == 0 {
            return Err(Error::ResourceLimit("Buffer size must be > 0".into()));
        }

        Ok(())
    }

    /// Converts to runtime configuration
    pub fn into_config(self) -> RuntimeConfig {
        RuntimeConfig {
            worker_config: WorkerConfig {
                thread_count: self.workers,
                queue_capacity: 1000,
                stack_size: 3 * 1024 * 1024,
            },
            resource_limits: ResourceLimits {
                max_tasks: self.workers * 2,
                max_memory: self.buffer_size * self.workers,
                max_connections: self.workers,
            },
            shutdown_config: ShutdownConfig {
                timeout: std::time::Duration::from_secs(self.shutdown_timeout),
                force_after_timeout: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_arg_validation() {
        let temp_dir = TempDir::new().unwrap();
        let input_file = temp_dir.path().join("test.zip");
        std::fs::write(&input_file, b"test").unwrap();

        let args = Args {
            input_path: input_file,
            output_dir: temp_dir.path().to_path_buf(),
            workers: 4,
            buffer_size: 8192,
            shutdown_timeout: 30,
            verbose: false,
        };

        assert!(args.validate().is_ok());
    }
}

