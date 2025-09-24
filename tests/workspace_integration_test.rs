//! Integration tests for workspace state management system
//! 
//! Tests the complete workspace functionality including:
//! - WorkspaceManager for persistent analysis sessions
//! - AnalysisSession tracking with timestamps and automatic latest linking
//! - Workspace cleanup commands and stale analysis detection
//! - Workspace isolation and state persistence

use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;
use chrono::{DateTime, Utc};
use serde_json;
use std::collections::HashMap;

// Import the workspace manager directly
use parseltongue::discovery::{WorkspaceManager, AnalysisSession, WorkspaceError};

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
    assert_eq!(parsed_session.entities_discovered, session.entities_discovered);
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
    
    manager.store_workflow_result("onboard", &test_data).await.unwrap();
    
    // Test retrieving workflow result
    let retrieved: Option<HashMap<String, i32>> = manager
        .get_cached_result("onboard")
        .await
        .unwrap();
    
    assert!(retrieved.is_some());
    let retrieved_data = retrieved.unwrap();
    assert_eq!(retrieved_data.get("entities"), Some(&42));
    assert_eq!(retrieved_data.get("files"), Some(&15));
    assert_eq!(retrieved_data.get("analysis_time_ms"), Some(&1250));
    
    // Test retrieving non-existent workflow result
    let nonexistent: Option<HashMap<String, i32>> = manager
        .get_cached_result("nonexistent")
        .await
        .unwrap();
    
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
    
    manager1.store_workflow_result("test", &data1).await.unwrap();
    manager2.store_workflow_result("test", &data2).await.unwrap();
    
    // Verify isolation - each workspace should have its own data
    let retrieved1: Option<HashMap<String, i32>> = manager1
        .get_cached_result("test")
        .await
        .unwrap();
    let retrieved2: Option<HashMap<String, i32>> = manager2
        .get_cached_result("test")
        .await
        .unwrap();
    
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
async fn test_workspace_complex_workflow_data() {
    let (mut manager, _temp_dir) = create_test_workspace().await;
    
    // Create session
    let _session = manager.get_or_create_session(false).await.unwrap();
    
    // Test storing complex workflow data
    let onboard_result = serde_json::json!({
        "architecture_html_path": "./parseltongue_workspace/analysis_20241201_120000/architecture.html",
        "route_table": [
            {"path": "/api/users", "handler": "UserHandler", "methods": ["GET", "POST"]},
            {"path": "/api/messages", "handler": "MessageHandler", "methods": ["GET", "POST", "DELETE"]}
        ],
        "key_contexts": [
            {"entity": "UserService", "file": "src/services/user.rs", "line": 15},
            {"entity": "MessageService", "file": "src/services/message.rs", "line": 23}
        ],
        "next_steps": [
            "Review architecture.html for system overview",
            "Examine UserService for authentication patterns",
            "Check MessageService for data flow patterns"
        ]
    });
    
    manager.store_workflow_result("onboard", &onboard_result).await.unwrap();
    
    // Test storing feature planning result
    let feature_result = serde_json::json!({
        "impact_scope": {
            "total_entities": 15,
            "production_files": 8,
            "test_files": 7,
            "risk_level": "Medium"
        },
        "change_checklist": [
            "Update UserService.authenticate method",
            "Add new endpoint in UserHandler",
            "Update user model validation"
        ],
        "test_recommendations": [
            "Add unit tests for new authentication logic",
            "Update integration tests for user endpoints",
            "Add security tests for authentication flow"
        ]
    });
    
    manager.store_workflow_result("feature-start", &feature_result).await.unwrap();
    
    // Retrieve and verify both results
    let retrieved_onboard: Option<serde_json::Value> = manager
        .get_cached_result("onboard")
        .await
        .unwrap();
    
    let retrieved_feature: Option<serde_json::Value> = manager
        .get_cached_result("feature-start")
        .await
        .unwrap();
    
    assert!(retrieved_onboard.is_some());
    assert!(retrieved_feature.is_some());
    
    let onboard_data = retrieved_onboard.unwrap();
    let feature_data = retrieved_feature.unwrap();
    
    // Verify onboard data
    assert_eq!(onboard_data["route_table"].as_array().unwrap().len(), 2);
    assert_eq!(onboard_data["key_contexts"].as_array().unwrap().len(), 2);
    assert_eq!(onboard_data["next_steps"].as_array().unwrap().len(), 3);
    
    // Verify feature data
    assert_eq!(feature_data["impact_scope"]["total_entities"], 15);
    assert_eq!(feature_data["impact_scope"]["risk_level"], "Medium");
    assert_eq!(feature_data["change_checklist"].as_array().unwrap().len(), 3);
    assert_eq!(feature_data["test_recommendations"].as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn test_workspace_error_handling() {
    let (mut manager, _temp_dir) = create_test_workspace().await;
    
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
    let result: Result<Option<HashMap<String, i32>>, WorkspaceError> = manager
        .get_cached_result("test")
        .await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        WorkspaceError::WorkspaceCorrupted { reason } => {
            assert_eq!(reason, "No active session");
        }
        _ => panic!("Expected WorkspaceCorrupted error"),
    }
}

#[tokio::test]
async fn test_workspace_automatic_latest_linking() {
    let (mut manager, _temp_dir) = create_test_workspace().await;
    
    // Create multiple sessions with delays to ensure different timestamps
    let session1 = manager.get_or_create_session(false).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    let session2 = manager.get_or_create_session(true).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    let session3 = manager.get_or_create_session(true).await.unwrap();
    
    // Latest should always be the most recently created
    let latest = manager.get_latest_session().await.unwrap();
    assert!(latest.is_some());
    assert_eq!(latest.unwrap().session_id, session3.session_id);
    
    // Verify sessions are sorted by timestamp (newest first)
    let sessions = manager.list_sessions().await.unwrap();
    assert_eq!(sessions.len(), 3);
    
    // First session should be the newest (session3)
    assert_eq!(sessions[0].session_id, session3.session_id);
    
    // Verify timestamps are in descending order
    for i in 1..sessions.len() {
        assert!(sessions[i-1].timestamp >= sessions[i].timestamp);
    }
}