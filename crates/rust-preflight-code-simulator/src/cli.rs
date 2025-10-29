use clap::Parser;
use std::path::PathBuf;

/// Rust preflight code validation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Code snippet to validate (alternative to --file)
    #[arg(long, conflicts_with = "file")]
    pub code_snippet: Option<String>,

    /// File containing code to validate (alternative to --code-snippet)
    #[arg(long, conflicts_with = "code_snippet")]
    pub file: Option<PathBuf>,

    /// Type of validation to perform
    #[arg(long, default_value = "all")]
    pub validation_type: ValidationTypeArg,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format (json or text)
    #[arg(long, default_value = "text")]
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ValidationTypeArg {
    /// Run all validations
    All,
    /// Syntax validation only
    Syntax,
    /// Type validation only
    Type,
    /// Borrow checker validation only
    BorrowChecker,
    /// Compilation validation only
    Compilation,
    /// Test validation only
    Test,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text output
    Text,
    /// JSON output for machine parsing
    Json,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Validate that required arguments are present
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.code_snippet.is_none() && self.file.is_none() {
            anyhow::bail!("Either --code-snippet or --file must be provided");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_validation_requires_input() {
        // Create CLI without code snippet or file
        let cli = Cli {
            code_snippet: None,
            file: None,
            validation_type: ValidationTypeArg::All,
            verbose: false,
            output_format: OutputFormat::Text,
        };

        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_cli_validation_with_code_snippet() {
        let cli = Cli {
            code_snippet: Some("fn main() {}".to_string()),
            file: None,
            validation_type: ValidationTypeArg::Syntax,
            verbose: false,
            output_format: OutputFormat::Json,
        };

        assert!(cli.validate().is_ok());
    }
}
