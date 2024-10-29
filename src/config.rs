// Level 4: Application Configuration
// - Stores all configuration options
// - Validates inputs and provides defaults
// - Organizes configuration into logical sections

use clap::ArgMatches;
use std::path::PathBuf;
use crate::error::{Result, Error};

// Level 3: Configuration Struct
pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
    pub verbose: bool,
    pub workers: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: u64,
}

impl Config {
    // Level 2: Create Config from CLI matches
    pub fn from_matches(matches: &ArgMatches) -> Result<Self> {
        // Level 1: Extract and validate arguments
        let input_zip = PathBuf::from(matches.get_one::<String>("input_zip").unwrap());
        let output_dir = PathBuf::from(matches.get_one::<String>("output_dir").unwrap());

        if !input_zip.is_file() {
            return Err(Error::Config(format!("Input ZIP file not found: {:?}", input_zip)));
        }

        let verbose = matches.get_flag("verbose");
        let workers = matches.get_one::<String>("workers").unwrap().parse::<usize>()?;
        let buffer_size = matches.get_one::<String>("buffer_size").unwrap().parse::<usize>()?;
        let shutdown_timeout = matches.get_one::<String>("shutdown_timeout").unwrap().parse::<u64>()?;

        Ok(Config {
            input_zip,
            output_dir,
            verbose,
            workers,
            buffer_size,
            shutdown_timeout,
        })
    }

    // Level 2: Create Config from input and output paths
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