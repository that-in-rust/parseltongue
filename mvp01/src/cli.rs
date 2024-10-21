use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Config {
    /// Path to the input ZIP file
    pub input_zip: PathBuf,
    /// Path to the output directory
    pub output_dir: PathBuf,
}
