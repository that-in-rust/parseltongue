//! Minimal test for workspace manager functionality
//! Tests only the core workspace management without dependencies on other modules

use chrono::Utc;
use tempfile::TempDir;
use tokio::fs;

use std::collections::HashMap;

// Directly include the workspace manager code for testing
mod workspace_manager {
    use chrono::{DateTime, Utc};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use std::path::PathBuf;
    use thiserror::Error;
    use tokio::fs;

    /// Persistent analysis workspace for iterative discovery
    #[derive(Debug)]
    pub struct WorkspaceManager {
        workspace_root: PathBuf,
        current_analysis: Option<AnalysisSession>,
    }

    /// Analysis session tracking with timestamps and metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnalysisSession {
        pub timestamp: DateTime<Utc>,
        pub session_id: String,
        pub analysis_path: PathBuf,
        pub entities_discovered: usize,
        pub last_updated: DateTime<Utc>,
    }

    /// Workspace management errors
    #[derive(Error, Debug)]
    #[allow(dead_code)]
    pub enum WorkspaceError {
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),

        #[error("Serialization error: {0}")]
        Serialization(#[from] serde_json::Error),

        #[error("Session not found: {session_id}")]
        SessionNotFound { session_id: String },

        #[error("Workspace corrupted: {reason}")]
        WorkspaceCorrupted { reason: String },

        #[error("Analysis stale: last updated {last_updated}, threshold {threshold_hours} hours")]
        AnalysisStale {
            last_updated: DateTime<Utc>,
            threshold_hours: u64,
        },
    }

    impl WorkspaceManager {
        /// Create a new workspace manager
        pub fn new(workspace_root: PathBuf) -> Self {
            Self {
                workspace_root,
                current_analysis: None,
            }
        }

        /// Get the workspace root path
        pub fn workspace_root(&self) -> &PathBuf {
            &self.workspace_root
        }

        /// Create or reuse analysis session
        pub async fn get_or_create_session(
            &mut self,
            force_refresh: bool,
        ) -> Result<AnalysisSession, WorkspaceError> {
            // Check if we should reuse existing session
            if !force_refresh {
                if let Some(ref current) = self.current_analysis {
                    if current.analysis_path.exists() {
                        return Ok(current.clone());
                    }
                }
            }

            // Create new session
            let timestamp = Utc::now();
            let session_id = format!("analysis_{}", timestamp.format("%Y%m%d_%H%M%S_%3f"));
            let analysis_path = self.workspace_root.join(&session_id);

            // Create session directory
            fs::create_dir_all(&analysis_path).await?;

            let session = AnalysisSession {
                timestamp,
                session_id,
                analysis_path,
                entities_discovered: 0,
                last_updated: timestamp,
            };

            // Save session metadata
            let metadata_path = session.analysis_path.join("session.json");
            let metadata_json = serde_json::to_string_pretty(&session)?;
            fs::write(&metadata_path, metadata_json).await?;

            // Update current session
            self.current_analysis = Some(session.clone());

            Ok(session)
        }

        /// Store workflow results for reuse
        pub async fn store_workflow_result<T: Serialize>(
            &self,
            workflow_type: &str,
            result: &T,
        ) -> Result<(), WorkspaceError> {
            let current_session = self.current_analysis.as_ref().ok_or_else(|| {
                WorkspaceError::WorkspaceCorrupted {
                    reason: "No active session".to_string(),
                }
            })?;

            let workflow_path = current_session.analysis_path.join("workflows");
            fs::create_dir_all(&workflow_path).await?;

            let result_file = workflow_path.join(format!("{}.json", workflow_type));
            let result_json = serde_json::to_string_pretty(result)?;
            fs::write(&result_file, result_json).await?;

            Ok(())
        }

        /// Retrieve cached workflow results
        pub async fn get_cached_result<T: DeserializeOwned>(
            &self,
            workflow_type: &str,
        ) -> Result<Option<T>, WorkspaceError> {
            let current_session = self.current_analysis.as_ref().ok_or_else(|| {
                WorkspaceError::WorkspaceCorrupted {
                    reason: "No active session".to_string(),
                }
            })?;

            let result_file = current_session
                .analysis_path
                .join("workflows")
                .join(format!("{}.json", workflow_type));

            if !result_file.exists() {
                return Ok(None);
            }

            let result_json = fs::read_to_string(&result_file).await?;
            let result: T = serde_json::from_str(&result_json)?;
            Ok(Some(result))
        }

        /// Clean up old analysis sessions
        pub async fn cleanup_stale_sessions(
            &self,
            max_age_hours: u64,
        ) -> Result<Vec<String>, WorkspaceError> {
            let mut cleaned_sessions = Vec::new();
            let threshold = Utc::now() - chrono::Duration::hours(max_age_hours as i64);

            let mut entries = fs::read_dir(&self.workspace_root).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                let session_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if !session_name.starts_with("analysis_") {
                    continue;
                }

                // Try to read session metadata
                let metadata_path = path.join("session.json");
                if let Ok(metadata_json) = fs::read_to_string(&metadata_path).await {
                    if let Ok(session) = serde_json::from_str::<AnalysisSession>(&metadata_json) {
                        if session.last_updated < threshold {
                            // Remove the entire session directory
                            fs::remove_dir_all(&path).await?;
                            cleaned_sessions.push(session.session_id);
                        }
                    }
                }
            }

            Ok(cleaned_sessions)
        }

        /// List all analysis sessions
        pub async fn list_sessions(&self) -> Result<Vec<AnalysisSession>, WorkspaceError> {
            let mut sessions = Vec::new();

            let mut entries = fs::read_dir(&self.workspace_root).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                let session_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if !session_name.starts_with("analysis_") {
                    continue;
                }

                // Try to read session metadata
                let metadata_path = path.join("session.json");
                if let Ok(metadata_json) = fs::read_to_string(&metadata_path).await {
                    if let Ok(session) = serde_json::from_str::<AnalysisSession>(&metadata_json) {
                        sessions.push(session);
                    }
                }
            }

            // Sort by timestamp (newest first)
            sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

            Ok(sessions)
        }

        /// Get the latest analysis session
        pub async fn get_latest_session(&self) -> Result<Option<AnalysisSession>, WorkspaceError> {
            let sessions = self.list_sessions().await?;
            Ok(sessions.into_iter().next())
        }

        /// Check if analysis is stale
        pub fn is_analysis_stale(&self, session: &AnalysisSession, threshold_hours: u64) -> bool {
            let threshold = chrono::Duration::hours(threshold_hours as i64);
            Utc::now() - session.last_updated > threshold
        }
    }
}

use workspace_manager::{AnalysisSession, WorkspaceError, WorkspaceManager};

async fn create_test_workspace() -> (WorkspaceManager, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path().join("parseltongue_workspace");
    fs::create_dir_all(&workspace_root).await.unwrap();

    let manager = WorkspaceManager::new(workspace_root);
    (manager, temp_dir)
}

#[tokio::test]
async fn test_workspace_session_creation_and_persistence() {
    let (mut manager, _temp_dir) = create_test_workspace().await;

    // Test creating a new session
    let session = manager.get_or_create_session(false).await.unwrap();

    // Verify session properties
    assert!(!session.session_id.is_empty());
    assert!(session.analysis_path.exists());
    assert_eq!(session.entities_discovered, 0);
    assert!(session.timestamp <= Utc::now());
    assert!(session.last_updated <= Utc::now());

    // Verify session metadata file exists
    let metadata_path = session.analysis_path.join("session.json");
    assert!(metadata_path.exists());

    // Verify metadata content
    let metadata_content = fs::read_to_string(&metadata_path).await.unwrap();
    let parsed_session: AnalysisSession = serde_json::from_str(&metadata_content).unwrap();
    assert_eq!(parsed_session.session_id, session.session_id);
    assert_eq!(
        parsed_session.entities_discovered,
        session.entities_discovered
    );
}

#[tokio::test]
async fn test_workspace_session_reuse() {
    let (mut manager, _temp_dir) = create_test_workspace().await;

    // Create first session
    let session1 = manager.get_or_create_session(false).await.unwrap();
    let session1_id = session1.session_id.clone();

    // Get session again without force refresh - should reuse
    let session2 = manager.get_or_create_session(false).await.unwrap();

    assert_eq!(session1_id, session2.session_id);
    assert_eq!(session1.analysis_path, session2.analysis_path);

    // Force refresh should create new session
    let session3 = manager.get_or_create_session(true).await.unwrap();

    assert_ne!(session1_id, session3.session_id);
    assert_ne!(session1.analysis_path, session3.analysis_path);
}

#[tokio::test]
async fn test_workspace_workflow_result_storage_and_retrieval() {
    let (mut manager, _temp_dir) = create_test_workspace().await;

    // Create session first
    let _session = manager.get_or_create_session(false).await.unwrap();

    // Test storing workflow result
    let test_data = HashMap::from([
        ("entities".to_string(), 42),
        ("files".to_string(), 15),
        ("analysis_time_ms".to_string(), 1250),
    ]);

    manager
        .store_workflow_result("onboard", &test_data)
        .await
        .unwrap();

    // Test retrieving workflow result
    let retrieved: Option<HashMap<String, i32>> =
        manager.get_cached_result("onboard").await.unwrap();

    assert!(retrieved.is_some());
    let retrieved_data = retrieved.unwrap();
    assert_eq!(retrieved_data.get("entities"), Some(&42));
    assert_eq!(retrieved_data.get("files"), Some(&15));
    assert_eq!(retrieved_data.get("analysis_time_ms"), Some(&1250));

    // Test retrieving non-existent workflow result
    let nonexistent: Option<HashMap<String, i32>> =
        manager.get_cached_result("nonexistent").await.unwrap();

    assert!(nonexistent.is_none());
}

#[tokio::test]
async fn test_workspace_stale_analysis_detection_and_cleanup() {
    let (mut manager, _temp_dir) = create_test_workspace().await;

    // Create a current session
    let current_session = manager.get_or_create_session(false).await.unwrap();

    // Simulate old session by creating directory manually
    let old_timestamp = Utc::now() - chrono::Duration::hours(25);
    let old_session_id = format!("analysis_{}", old_timestamp.format("%Y%m%d_%H%M%S_%3f"));
    let old_session_path = manager.workspace_root().join(&old_session_id);
    fs::create_dir_all(&old_session_path).await.unwrap();

    // Create old session metadata file
    let old_session = AnalysisSession {
        timestamp: old_timestamp,
        session_id: old_session_id.clone(),
        analysis_path: old_session_path.clone(),
        entities_discovered: 100,
        last_updated: old_timestamp,
    };

    let metadata_path = old_session_path.join("session.json");
    let metadata_json = serde_json::to_string_pretty(&old_session).unwrap();
    fs::write(&metadata_path, metadata_json).await.unwrap();

    // Test stale detection
    assert!(manager.is_analysis_stale(&old_session, 24));
    assert!(!manager.is_analysis_stale(&current_session, 24));

    // Test cleanup of stale sessions
    let cleaned_sessions = manager.cleanup_stale_sessions(24).await.unwrap();

    assert_eq!(cleaned_sessions.len(), 1);
    assert_eq!(cleaned_sessions[0], old_session_id);
    assert!(!old_session_path.exists());

    // Current session should still exist
    assert!(current_session.analysis_path.exists());
}

#[tokio::test]
async fn test_workspace_session_listing_and_latest() {
    let (mut manager, _temp_dir) = create_test_workspace().await;

    // Initially no sessions
    let sessions = manager.list_sessions().await.unwrap();
    assert!(sessions.is_empty());

    let latest = manager.get_latest_session().await.unwrap();
    assert!(latest.is_none());

    // Create multiple sessions
    let session1 = manager.get_or_create_session(false).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let session2 = manager.get_or_create_session(true).await.unwrap();

    // Test listing sessions
    let sessions = manager.list_sessions().await.unwrap();
    assert_eq!(sessions.len(), 2);

    let session_ids: Vec<&String> = sessions.iter().map(|s| &s.session_id).collect();
    assert!(session_ids.contains(&&session1.session_id));
    assert!(session_ids.contains(&&session2.session_id));

    // Test getting latest session (should be session2 as it was created later)
    let latest = manager.get_latest_session().await.unwrap();
    assert!(latest.is_some());
    assert_eq!(latest.unwrap().session_id, session2.session_id);
}

#[tokio::test]
async fn test_workspace_isolation() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();

    let workspace1 = temp_dir1.path().join("parseltongue_workspace");
    let workspace2 = temp_dir2.path().join("parseltongue_workspace");

    fs::create_dir_all(&workspace1).await.unwrap();
    fs::create_dir_all(&workspace2).await.unwrap();

    let mut manager1 = WorkspaceManager::new(workspace1);
    let mut manager2 = WorkspaceManager::new(workspace2);

    // Create sessions in both workspaces
    let session1 = manager1.get_or_create_session(false).await.unwrap();
    let session2 = manager2.get_or_create_session(false).await.unwrap();

    // Store different data in each workspace
    let data1 = HashMap::from([("workspace".to_string(), 1)]);
    let data2 = HashMap::from([("workspace".to_string(), 2)]);

    manager1
        .store_workflow_result("test", &data1)
        .await
        .unwrap();
    manager2
        .store_workflow_result("test", &data2)
        .await
        .unwrap();

    // Verify isolation - each workspace should have its own data
    let retrieved1: Option<HashMap<String, i32>> =
        manager1.get_cached_result("test").await.unwrap();
    let retrieved2: Option<HashMap<String, i32>> =
        manager2.get_cached_result("test").await.unwrap();

    assert_eq!(retrieved1.unwrap().get("workspace"), Some(&1));
    assert_eq!(retrieved2.unwrap().get("workspace"), Some(&2));

    // Sessions should be different
    assert_ne!(session1.session_id, session2.session_id);
    assert_ne!(session1.analysis_path, session2.analysis_path);

    // Each workspace should only see its own sessions
    let sessions1 = manager1.list_sessions().await.unwrap();
    let sessions2 = manager2.list_sessions().await.unwrap();

    assert_eq!(sessions1.len(), 1);
    assert_eq!(sessions2.len(), 1);
    assert_eq!(sessions1[0].session_id, session1.session_id);
    assert_eq!(sessions2[0].session_id, session2.session_id);
}

#[tokio::test]
async fn test_workspace_error_handling() {
    let (manager, _temp_dir) = create_test_workspace().await;

    // Test error when trying to store without active session
    let test_data = HashMap::from([("test".to_string(), 1)]);
    let result = manager.store_workflow_result("test", &test_data).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        WorkspaceError::WorkspaceCorrupted { reason } => {
            assert_eq!(reason, "No active session");
        }
        _ => panic!("Expected WorkspaceCorrupted error"),
    }

    // Test error when trying to retrieve without active session
    let result: Result<Option<HashMap<String, i32>>, WorkspaceError> =
        manager.get_cached_result("test").await;

    assert!(result.is_err());
    match result.unwrap_err() {
        WorkspaceError::WorkspaceCorrupted { reason } => {
            assert_eq!(reason, "No active session");
        }
        _ => panic!("Expected WorkspaceCorrupted error"),
    }
}
