use std::path::PathBuf;
use clap::{Args, Subcommand};
use crate::discovery::{WorkspaceManager, WorkspaceError};
use chrono::Utc;
use serde_json;

/// Workspace management commands
#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    /// Workspace root directory (defaults to ./parseltongue_workspace)
    #[arg(long, default_value = "./parseltongue_workspace")]
    pub workspace_root: PathBuf,
    
    #[command(subcommand)]
    pub command: WorkspaceCommand,
}

#[derive(Debug, Subcommand)]
pub enum WorkspaceCommand {
    /// Create or get current analysis session
    Session {
        /// Force refresh - create new session even if current exists
        #[arg(long)]
        force_refresh: bool,
    },
    /// List all analysis sessions
    List,
    /// Clean up stale analysis sessions
    Cleanup {
        /// Maximum age in hours for sessions to keep
        #[arg(long, default_value = "168")] // 7 days default
        max_age_hours: u64,
    },
    /// Show workspace status and latest session info
    Status,
    /// Store workflow result for caching
    Store {
        /// Workflow type identifier
        workflow_type: String,
        /// JSON data to store
        data: String,
    },
    /// Retrieve cached workflow result
    Get {
        /// Workflow type identifier
        workflow_type: String,
    },
}

pub async fn handle_workspace_command(args: WorkspaceArgs) -> Result<(), WorkspaceError> {
    let mut manager = WorkspaceManager::new(args.workspace_root);
    
    match args.command {
        WorkspaceCommand::Session { force_refresh } => {
            let session = manager.get_or_create_session(force_refresh).await?;
            println!("Active session: {}", session.session_id);
            println!("Path: {}", session.analysis_path.display());
            println!("Created: {}", session.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("Last updated: {}", session.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("Entities discovered: {}", session.entities_discovered);
        }
        
        WorkspaceCommand::List => {
            let sessions = manager.list_sessions().await?;
            if sessions.is_empty() {
                println!("No analysis sessions found.");
            } else {
                println!("Analysis sessions ({} total):", sessions.len());
                for session in sessions {
                    let age_hours = (Utc::now() - session.last_updated).num_hours();
                    println!("  {} ({}h ago) - {} entities", 
                        session.session_id, 
                        age_hours,
                        session.entities_discovered
                    );
                }
            }
        }
        
        WorkspaceCommand::Cleanup { max_age_hours } => {
            let cleaned = manager.cleanup_stale_sessions(max_age_hours).await?;
            if cleaned.is_empty() {
                println!("No stale sessions found to clean up.");
            } else {
                println!("Cleaned up {} stale sessions:", cleaned.len());
                for session_id in cleaned {
                    println!("  Removed: {}", session_id);
                }
            }
        }
        
        WorkspaceCommand::Status => {
            let sessions = manager.list_sessions().await?;
            let latest = manager.get_latest_session().await?;
            
            println!("Workspace Status:");
            println!("  Root: {}", manager.workspace_root().display());
            println!("  Total sessions: {}", sessions.len());
            
            if let Some(latest_session) = latest {
                let age_hours = (Utc::now() - latest_session.last_updated).num_hours();
                let is_stale = manager.is_analysis_stale(&latest_session, 24);
                
                println!("  Latest session: {}", latest_session.session_id);
                println!("  Age: {}h ({})", age_hours, if is_stale { "stale" } else { "fresh" });
                println!("  Entities: {}", latest_session.entities_discovered);
            } else {
                println!("  No sessions found");
            }
        }
        
        WorkspaceCommand::Store { workflow_type, data } => {
            // Parse JSON data
            let json_value: serde_json::Value = serde_json::from_str(&data)
                .map_err(|e| WorkspaceError::Serialization(e))?;
            
            manager.store_workflow_result(&workflow_type, &json_value).await?;
            println!("Stored workflow result for: {}", workflow_type);
        }
        
        WorkspaceCommand::Get { workflow_type } => {
            let result: Option<serde_json::Value> = manager
                .get_cached_result(&workflow_type)
                .await?;
            
            match result {
                Some(data) => {
                    println!("Cached result for {}:", workflow_type);
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                None => {
                    println!("No cached result found for: {}", workflow_type);
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;
    use std::collections::HashMap;

    async fn create_test_manager() -> (WorkspaceManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let workspace_root = temp_dir.path().join("test_workspace");
        fs::create_dir_all(&workspace_root).await.unwrap();
        
        let manager = WorkspaceManager::new(workspace_root);
        (manager, temp_dir)
    }

    #[tokio::test]
    async fn test_workspace_cli_session_creation() {
        let (mut manager, _temp_dir) = create_test_manager().await;
        
        // Test session creation
        let session = manager.get_or_create_session(false).await.unwrap();
        assert!(!session.session_id.is_empty());
        assert!(session.analysis_path.exists());
    }

    #[tokio::test]
    async fn test_workspace_cli_store_and_retrieve() {
        let (mut manager, _temp_dir) = create_test_manager().await;
        
        // Create session first
        let _session = manager.get_or_create_session(false).await.unwrap();
        
        // Test storing and retrieving workflow results
        let test_data = serde_json::json!({
            "entities": 42,
            "files": 15,
            "timestamp": "2024-01-01T12:00:00Z"
        });
        
        manager.store_workflow_result("test_workflow", &test_data).await.unwrap();
        
        let retrieved: Option<serde_json::Value> = manager
            .get_cached_result("test_workflow")
            .await
            .unwrap();
        
        assert!(retrieved.is_some());
        let retrieved_data = retrieved.unwrap();
        assert_eq!(retrieved_data["entities"], 42);
        assert_eq!(retrieved_data["files"], 15);
    }

    #[tokio::test]
    async fn test_workspace_cli_cleanup() {
        let (mut manager, _temp_dir) = create_test_manager().await;
        
        // Create a current session
        let current_session = manager.get_or_create_session(false).await.unwrap();
        
        // Simulate old session
        let old_timestamp = Utc::now() - chrono::Duration::hours(25);
        let old_session_id = format!("analysis_{}", old_timestamp.format("%Y%m%d_%H%M%S"));
        let old_session_path = manager.workspace_root().join(&old_session_id);
        fs::create_dir_all(&old_session_path).await.unwrap();
        
        // Create old session metadata
        let old_session = crate::discovery::AnalysisSession {
            timestamp: old_timestamp,
            session_id: old_session_id.clone(),
            analysis_path: old_session_path.clone(),
            entities_discovered: 100,
            last_updated: old_timestamp,
        };
        
        let metadata_path = old_session_path.join("session.json");
        let metadata_json = serde_json::to_string_pretty(&old_session).unwrap();
        fs::write(&metadata_path, metadata_json).await.unwrap();
        
        // Test cleanup
        let cleaned = manager.cleanup_stale_sessions(24).await.unwrap();
        assert_eq!(cleaned.len(), 1);
        assert_eq!(cleaned[0], old_session_id);
        assert!(!old_session_path.exists());
        assert!(current_session.analysis_path.exists());
    }

    #[tokio::test]
    async fn test_workspace_cli_list_sessions() {
        let (mut manager, _temp_dir) = create_test_manager().await;
        
        // Create multiple sessions
        let session1 = manager.get_or_create_session(false).await.unwrap();
        let session2 = manager.get_or_create_session(true).await.unwrap();
        
        let sessions = manager.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 2);
        
        let session_ids: Vec<&String> = sessions.iter().map(|s| &s.session_id).collect();
        assert!(session_ids.contains(&&session1.session_id));
        assert!(session_ids.contains(&&session2.session_id));
    }

    #[tokio::test]
    async fn test_workspace_cli_status() {
        let (mut manager, _temp_dir) = create_test_manager().await;
        
        // Initially no sessions
        let latest = manager.get_latest_session().await.unwrap();
        assert!(latest.is_none());
        
        // Create session
        let session = manager.get_or_create_session(false).await.unwrap();
        
        // Check status
        let latest = manager.get_latest_session().await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().session_id, session.session_id);
        
        let is_stale = manager.is_analysis_stale(&session, 24);
        assert!(!is_stale); // Should be fresh
    }
}