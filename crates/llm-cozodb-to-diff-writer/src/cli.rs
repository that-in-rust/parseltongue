use clap::Parser;
use std::path::PathBuf;

/// Ultra-minimalist code writer from CozoDB to files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to CozoDB database
    #[arg(long)]
    pub database: PathBuf,

    /// Root directory for file operations
    #[arg(long)]
    pub root: PathBuf,

    /// Dry-run mode (show what would be written without actually writing)
    #[arg(long)]
    pub dry_run: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli {
            database: PathBuf::from("./parseltongue.db"),
            root: PathBuf::from("./project"),
            dry_run: false,
            verbose: false,
        };

        assert_eq!(cli.database, PathBuf::from("./parseltongue.db"));
        assert_eq!(cli.root, PathBuf::from("./project"));
    }
}
