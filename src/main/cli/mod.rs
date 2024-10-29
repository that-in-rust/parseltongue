//! CLI Module - Pyramidal Structure
//! Layer 1: Module Exports
//! Layer 2: Common Types
//! Layer 3: CLI Coordination
//! Layer 4: Error Handling
//! Layer 5: Helper Functions

// Layer 1: Module Organization
pub mod args;
pub mod config;

pub use args::Args;
pub use config::{Config, ConfigBuilder};

use anyhow::Result;
use tracing::info;

// Layer 2: CLI Coordination
pub struct CliManager {
    args: Args,
    config: Config,
}

// Layer 3: Implementation
impl CliManager {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let config = args.into_config()?;
        
        Ok(Self { args, config })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    // Layer 4: Validation
    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        info!("CLI configuration validated successfully");
        Ok(())
    }
}

// Layer 5: Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cli_manager() {
        let manager = CliManager::new();
        assert!(manager.is_ok());
    }
}
