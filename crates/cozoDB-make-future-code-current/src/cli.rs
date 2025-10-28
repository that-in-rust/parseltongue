//! CLI interface for Tool 5: CozoDB State Reset
//!
//! Provides command-line interface for state reset operations with
//! configurable options and Git integration.

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;

use super::{CozoDBMakeFutureCodeCurrent, Tool5Error, Tool5Result};

/// Tool 5 CLI configuration
#[derive(Debug, Clone, Parser)]
#[command(name = "cozoDB-make-future-code-current")]
#[command(about = "Reset CozoDB state after successful code changes")]
#[command(long_about = """
Tool 5: CozoDB Make Future Code Current

This tool provides simplified state reset functionality that:
1. Backs up current metadata to timestamped MD files (Git-integrated)
2. Re-triggers Tool 1 to re-ingest current file state
3. Resets all current/future flags appropriately

The simplified approach prioritizes speed and reliability over complex
reconciliation logic, leveraging existing Tool 1 functionality.
""")]
pub struct Tool5Cli {
    /// Path to the Rust project directory
    #[arg(short, long, default_value = ".")]
    pub project_path: PathBuf,

    /// Backup directory path (default: .parseltongue/metadata-backups)
    #[arg(short, long, default_value = ".parseltongue/metadata-backups")]
    pub backup_dir: Option<PathBuf>,

    /// Reset strategy to use
    #[arg(short, long, default_value = "simple")]
    pub reset_strategy: ResetStrategy,

    /// Skip metadata backup
    #[arg(long)]
    pub skip_backup: bool,

    /// Enable Git integration
    #[arg(long)]
    pub git_integration: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Reset strategy options
#[derive(Debug, Clone, Parser)]
pub enum ResetStrategy {
    /// Simple re-ingestion approach (default)
    Simple,
    /// Hybrid strategy with metadata preservation
    Hybrid,
    /// Complex reconciliation (not implemented yet)
    Complex,
}

/// Available subcommands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Reset the database state
    Reset {
        /// Force reset without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// List available metadata backups
    ListBackups,
    /// Show current database statistics
    Stats,
    /// Validate project structure
    Validate,
}

/// Tool 5 configuration
#[derive(Debug, Clone)]
pub struct Tool5Config {
    pub project_path: PathBuf,
    pub backup_dir: PathBuf,
    pub reset_strategy: ResetStrategy,
    pub skip_backup: bool,
    pub git_integration: bool,
    pub verbose: bool,
}

impl Tool5Config {
    /// Create configuration from CLI arguments
    pub fn from_cli(cli: Tool5Cli) -> Self {
        let backup_dir = cli.backup_dir
            .unwrap_or_else(|| cli.project_path.join(".parseltongue").join("metadata-backups"));

        Self {
            project_path: cli.project_path,
            backup_dir,
            reset_strategy: cli.reset_strategy,
            skip_backup: cli.skip_backup,
            git_integration: cli.git_integration,
            verbose: cli.verbose,
        }
    }
}

impl Tool5Cli {
    /// Execute the CLI command
    pub async fn execute(self) -> Tool5Result<()> {
        match self.command {
            Some(Commands::Reset { force }) => {
                self.handle_reset(force).await
            }
            Some(Commands::ListBackups) => {
                self.handle_list_backups().await
            }
            Some(Commands::Stats) => {
                self.handle_stats().await
            }
            Some(Commands::Validate) => {
                self.handle_validate().await
            }
            None => {
                // Default behavior: reset state
                self.handle_reset(false).await
            }
        }
    }

    /// Handle reset command
    async fn handle_reset(&self, force: bool) -> Tool5Result<()> {
        let config = Tool5Config::from_cli(self.clone());

        // Validate project structure first
        self.validate_project_structure(&config.project_path)?;

        // Ask for confirmation unless force flag is used
        if !force {
            self.confirm_reset_operation(&config)?;
        }

        // Create Tool 5 instance
        let tool5 = CozoDBMakeFutureCodeCurrent::new(config.project_path.clone())?;

        // Execute state reset
        let stats = tool5.reset_state().await?;

        // Git integration if enabled
        if config.git_integration {
            self.handle_git_integration(&config, &stats).await?;
        }

        // Display results
        self.display_reset_results(&config, &stats);

        Ok(())
    }

    /// Handle list backups command
    async fn handle_list_backups(&self) -> Tool5Result<()> {
        let config = Tool5Config::from_cli(self.clone());

        // Create backup manager to list backups
        let backup_manager = super::MetadataBackupManager::new(config.project_path.clone());
        let backups = backup_manager.list_backups().await?;

        if backups.is_empty() {
            println!("No metadata backups found in: {}", backup_manager.backup_dir().display());
        } else {
            println!("Available metadata backups in: {}", backup_manager.backup_dir().display());
            for backup in backups {
                println!("  📁 {}", backup.display());
            }
        }

        Ok(())
    }

    /// Handle stats command
    async fn handle_stats(&self) -> Tool5Result<()> {
        let config = Tool5Config::from_cli(self.clone());

        // Create Tool 5 instance to get stats
        let tool5 = CozoDBMakeFutureCodeCurrent::new(config.project_path.clone())?;
        let stats = tool5.get_stats().await?;

        self.display_database_stats(&stats);

        Ok(())
    }

    /// Handle validate command
    async fn handle_validate(&self) -> Tool5Result<()> {
        let config = Tool5Config::from_cli(self.clone());

        println!("🔍 Validating project structure...");
        self.validate_project_structure(&config.project_path)?;

        println!("✅ Project structure is valid");
        println!("📁 Project path: {}", config.project_path.display());
        println!("📁 Backup directory: {}", config.backup_dir.display());
        println!("🔄 Reset strategy: {:?}", config.reset_strategy);

        Ok(())
    }

    /// Validate project structure
    fn validate_project_structure(&self, project_path: &PathBuf) -> Tool5Result<()> {
        // Check if project path exists
        if !project_path.exists() {
            return Err(Tool5Error::project_path_not_found(project_path.clone()));
        }

        // Check if it's a Rust project (has Cargo.toml)
        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(Tool5Error::parseltongue_project_not_found(project_path.clone()));
        }

        // Check if it has .parseltongue directory
        let parseltongue_dir = project_path.join(".parseltongue");
        if !parseltongue_dir.exists() {
            // Create it if it doesn't exist
            std::fs::create_dir_all(&parseltongue_dir)
                .map_err(|e| Tool5Error::file_system(format!("Failed to create .parseltongue directory: {}", e)))?;
            println!("📁 Created .parseltongue directory");
        }

        Ok(())
    }

    /// Confirm reset operation with user
    fn confirm_reset_operation(&self, config: &Tool5Config) -> Tool5Result<()> {
        println!("🔄 About to reset CozoDB state for project: {}", config.project_path.display());
        println!("📁 Backup directory: {}", config.backup_dir.display());
        println!("🔄 Reset strategy: {:?}", config.reset_strategy);

        if config.skip_backup {
            println!("⚠️  WARNING: Skipping metadata backup!");
        }

        println!();
        print!("Do you want to continue? [y/N]: ");
        use std::io::{self, Write};
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(()),
            _ => Err(Tool5Error::validation("Operation cancelled by user".to_string())),
        }
    }

    /// Handle Git integration
    async fn handle_git_integration(&self, config: &Tool5Config, stats: &super::ResetStats) -> Tool5Result<()> {
        println!("🔄 Git integration enabled (not yet implemented)");
        println!("📁 Would commit backup: {}", stats.backup_path.display());

        // TODO: Implement actual Git operations
        // - Check if we're in a Git repository
        // - Check for uncommitted changes
        // - Commit backup files if needed
        // - Create proper commit message

        Ok(())
    }

    /// Display reset operation results
    fn display_reset_results(&self, config: &Tool5Config, stats: &super::ResetStats) {
        println!();
        println!("✅ State reset completed successfully!");
        println!("📊 Results:");
        println!("   📁 Backup location: {}", stats.backup_path.display());
        println!("   📄 Files processed: {}", stats.files_processed);
        println!("   🧩 Chunks processed: {}", stats.chunks_processed);
        println!("   🔗 Relationships processed: {}", stats.relationships_processed);
        println!("   ⏰ Completed at: {}", stats.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));

        if config.verbose {
            println!();
            println!("🔧 Configuration:");
            println!("   📁 Project path: {}", config.project_path.display());
            println!("   📁 Backup directory: {}", config.backup_dir.display());
            println!("   🔄 Reset strategy: {:?}", config.reset_strategy);
            println!("   📋 Skip backup: {}", config.skip_backup);
            println!("   🔄 Git integration: {}", config.git_integration);
        }
    }

    /// Display database statistics
    fn display_database_stats(&self, stats: &parseltongue_02::storage::DatabaseStats) {
        println!();
        println!("📊 Database Statistics:");
        println!("   🧩 Total chunks: {}", stats.total_chunks);
        println!("   🔗 Total relationships: {}", stats.total_relationships);
        println!("   📁 Unique files: {}", stats.file_count);
        println!("   ⏰ Last updated: {}", stats.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));
    }
}