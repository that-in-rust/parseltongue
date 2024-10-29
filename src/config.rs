// Level 4: Application Configuration
// - Holds configuration values
// - Provides methods for creation and validation

use std::path::PathBuf;
use crate::error::{Result, Error};

pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
    pub verbose: bool,
    pub workers: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: u64,
}

impl Config {
    // Level 3: Create Config from paths
    pub fn from_paths(input_zip: &str, output_dir: &str) -> Result<Self> {
        let input_zip = PathBuf::from(input_zip);
        let output_dir = PathBuf::from(output_dir);

        if !input_zip.is_file() {
            return Err(Error::Config(format!("Input ZIP file not found: {:?}", input_zip)));
        }

        Ok(Config {
            input_zip,
            output_dir,
            verbose: false,
            workers: num_cpus::get(),
            buffer_size: 8192,
            shutdown_timeout: 5,
        })
    }
} 