// Level 4: Command Line Interface Parsing
// - Defines CLI arguments
// - Parses arguments into Config
// - Validates inputs

use clap::{Arg, Command};
use crate::config::Config;
use crate::error::{Result, Error};

pub fn parse_args() -> Result<Config> {
    // Level 3: Define CLI arguments
    let matches = Command::new("parseltongue")
        .version("0.1.0")
        .author("twitter.com/amuldotexe")
        .about("High-performance ZIP file processor with async I/O")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("ZIP_FILE")
            .help("Input ZIP file to process")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT_DIR")
            .help("Output directory for processed data")
            .required(true))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose logging"))
        .arg(Arg::new("workers")
            .short('w')
            .long("workers")
            .value_name("NUM")
            .help("Number of worker threads")
            .required(false))
        .get_matches();

    // Level 3: Parse and validate inputs
    let input_zip = matches.value_of("input").unwrap().into();
    let output_dir = matches.value_of("output").unwrap().into();
    let verbose = matches.is_present("verbose");
    let workers = matches.value_of("workers")
        .map(|s| s.parse::<usize>())
        .transpose()
        .map_err(|e| Error::Config(format!("Invalid workers value: {}", e)))?
        .unwrap_or_else(num_cpus::get);

    // Level 3: Construct Config
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