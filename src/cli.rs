// Level 4: Command Line Interface Parsing
// - Defines CLI arguments
// - Parses arguments into Config
// - Validates inputs

use clap::{Arg, Command};
use crate::cli::args::Args;
use crate::core::types::Config;
use crate::core::error::{Result, Error};

pub fn parse_args() -> Result<Config> {
    // Level 3: Define command-line arguments
    let matches = Command::new("parseltongue")
        .version("0.1.0")
        .author("twitter.com/amuldotexe")
        .about("High-performance ZIP file processor with async I/O")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("ZIP_FILE")
                .help("Input ZIP file to process")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_DIR")
                .help("Output directory for processed data")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose logging"),
        )
        .arg(
            Arg::new("workers")
                .short('w')
                .long("workers")
                .value_name("NUM")
                .help("Number of worker threads"),
        )
        .get_matches();

    // Level 2: Parse arguments into Config fields
    let input_zip = matches.get_one::<String>("input").unwrap().into();
    let output_dir = matches.get_one::<String>("output").unwrap().into();
    let verbose = matches.get_flag("verbose");
    let workers = matches
        .get_one::<String>("workers")
        .map(|s| s.parse::<usize>())
        .transpose()
        .map_err(|e| Error::Config(format!("Invalid workers value: {}", e)))?
        .unwrap_or_else(num_cpus::get);

    // Level 1: Construct Config
    let config = Config {
        input_zip,
        output_dir,
        verbose,
        workers,
        buffer_size: 8192,
        shutdown_timeout: 5,
    };

    Ok(config)
} 