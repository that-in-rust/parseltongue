// Level 4: Command Line Interface Parsing
// - Defines CLI arguments
// - Validates input
// - Constructs Config struct

use clap::{Arg, Command};
use crate::config::Config;
use crate::error::Result;

// Level 3: Parse CLI arguments into Config
pub fn parse_args() -> Result<Config> {
    // Level 2: Define CLI arguments using Clap
    let matches = Command::new("parseltongue")
        .version("0.1.0")
        .about("High-performance ZIP file processor with async I/O")
        .arg(Arg::new("input_zip")
            .short('i')
            .long("input-zip")
            .value_name("FILE")
            .help("Absolute path to source ZIP file")
            .required(true))
        .arg(Arg::new("output_dir")
            .short('o')
            .long("output-dir")
            .value_name("DIR")
            .help("Absolute path for base output directory")
            .required(true))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(clap::ArgAction::SetTrue)
            .help("Enable verbose logging"))
        .arg(Arg::new("workers")
            .short('w')
            .long("workers")
            .value_name("NUM")
            .help("Number of worker threads")
            .default_value("4"))
        .arg(Arg::new("buffer_size")
            .short('b')
            .long("buffer-size")
            .value_name("BYTES")
            .help("Streaming buffer size")
            .default_value("8192"))
        .arg(Arg::new("shutdown_timeout")
            .short('s')
            .long("shutdown-timeout")
            .value_name("SECONDS")
            .help("Graceful shutdown timeout")
            .default_value("5"))
        .get_matches();

    // Level 2: Construct Config from matches
    Config::from_matches(&matches)
} 