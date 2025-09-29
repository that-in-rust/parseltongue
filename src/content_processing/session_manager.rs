use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

use crate::content_processing::types::*;

/// Manages processing sessions and directory structure
/// 
/// # Contract
/// - Creates organized directory structure for processing sessions
/// - Maintains session metadata and configuration
/// - Provides session lifecycle management
pub struct ProcessingSessionManager {
    base_directory: PathBuf,
}

impl ProcessingSessionManager {
    /// Creates a new session manager with the specified base directory
    /// 
    /// # Arguments
    /// * `base_directory` - Root directory for all processing sessions
    /// 
    /// # Returns
    /// * `Result<Self>` - New session manager or error if directory creation fails
    pub fn new(base_directory: impl Into<PathBuf>) -> Result<Self> {
        let base_dir = base_directory.into();
        
        // Create base directory if it doesn't exist
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)
                .map_err(|_| ContentProcessingError::DirectoryCreationFailed { 
                    path: base_dir.clone() 
                })?;
        }
        
        Ok(Self {
            base_directory: base_dir,
        })
    }
    
    /// Creates a new processing session with organized directory structure
    /// 
    /// # Arguments
    /// * `name` - Human-readable name for the session
    /// * `source_files` - List of source files to process
    /// * `config` - Optional segmentation configuration
    /// 
    /// # Returns
    /// * `Result<ProcessingSession>` - New session with created directory structure
    pub fn create_session(
        &self,
        name: String,
        source_files: Vec<PathBuf>,
        config: Option<SegmentationConfig>,
    ) -> Result<ProcessingSession> {
        // Create session directory structure
        let session_dir = self.base_directory.join(&name);
        self.create_session_directories(&session_dir)?;
        
        // Create session metadata
        let session = ProcessingSession::new(
            name,
            source_files,
            session_dir,
            config,
        );
        
        // Save session metadata
        self.save_session_metadata(&session)?;
        
        Ok(session)
    }
    
    /// Loads an existing processing session
    /// 
    /// # Arguments
    /// * `session_name` - Name of the session to load
    /// 
    /// # Returns
    /// * `Result<ProcessingSession>` - Loaded session or error if not found
    pub fn load_session(&self, session_name: &str) -> Result<ProcessingSession> {
        let session_dir = self.base_directory.join(session_name);
        let metadata_file = session_dir.join("session_metadata.json");
        
        if !metadata_file.exists() {
            return Err(ContentProcessingError::FileNotFound { 
                path: metadata_file 
            });
        }
        
        let metadata_content = fs::read_to_string(&metadata_file)?;
        let session: ProcessingSession = serde_json::from_str(&metadata_content)?;
        
        Ok(session)
    }
    
    /// Lists all available processing sessions
    /// 
    /// # Returns
    /// * `Result<Vec<String>>` - List of session names
    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let mut sessions = Vec::new();
        
        if !self.base_directory.exists() {
            return Ok(sessions);
        }
        
        for entry in fs::read_dir(&self.base_directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Check if it's a valid session directory
                    let metadata_file = path.join("session_metadata.json");
                    if metadata_file.exists() {
                        sessions.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(sessions)
    }
    
    /// Updates session metadata
    /// 
    /// # Arguments
    /// * `session` - Session to update
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn update_session(&self, mut session: ProcessingSession) -> Result<ProcessingSession> {
        session.updated_at = Utc::now();
        self.save_session_metadata(&session)?;
        Ok(session)
    }
    
    /// Creates the standard directory structure for a processing session
    /// 
    /// Directory structure:
    /// ```
    /// session_name/
    /// ├── chunks/              # Individual content chunks
    /// ├── progress/            # Progress tracking files
    /// ├── analysis/            # Analysis results
    /// ├── insights/            # Extracted insights
    /// │   ├── user_journeys/   # User journey extractions
    /// │   ├── technical/       # Technical insights
    /// │   └── strategic/       # Strategic themes
    /// ├── synthesis/           # Cross-reference and synthesis
    /// └── output/              # Final documents
    /// ```
    fn create_session_directories(&self, session_dir: &Path) -> Result<()> {
        let directories = [
            "chunks",
            "progress", 
            "analysis",
            "insights",
            "insights/user_journeys",
            "insights/technical",
            "insights/strategic",
            "synthesis",
            "output",
        ];
        
        for dir in &directories {
            let dir_path = session_dir.join(dir);
            fs::create_dir_all(&dir_path)
                .map_err(|_| ContentProcessingError::DirectoryCreationFailed { 
                    path: dir_path 
                })?;
        }
        
        Ok(())
    }
    
    /// Saves session metadata to disk
    fn save_session_metadata(&self, session: &ProcessingSession) -> Result<()> {
        let metadata_file = session.output_directory.join("session_metadata.json");
        let metadata_json = serde_json::to_string_pretty(session)?;
        fs::write(&metadata_file, metadata_json)?;
        Ok(())
    }
    
    /// Gets the path for a specific session subdirectory
    pub fn get_session_path(&self, session: &ProcessingSession, subdir: &str) -> PathBuf {
        session.output_directory.join(subdir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_session_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProcessingSessionManager::new(temp_dir.path()).unwrap();
        
        assert!(temp_dir.path().exists());
    }
    
    #[test]
    fn test_create_session() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProcessingSessionManager::new(temp_dir.path()).unwrap();
        
        let source_files = vec![
            PathBuf::from("DTNote01.md"),
            PathBuf::from("DTNote02.md"),
        ];
        
        let session = manager.create_session(
            "test_session".to_string(),
            source_files.clone(),
            None,
        ).unwrap();
        
        assert_eq!(session.name, "test_session");
        assert_eq!(session.source_files, source_files);
        assert_eq!(session.config.chunk_size, 300);
        assert_eq!(session.config.overlap_size, 20);
        
        // Verify directory structure was created
        let session_dir = temp_dir.path().join("test_session");
        assert!(session_dir.exists());
        assert!(session_dir.join("chunks").exists());
        assert!(session_dir.join("progress").exists());
        assert!(session_dir.join("analysis").exists());
        assert!(session_dir.join("insights").exists());
        assert!(session_dir.join("insights/user_journeys").exists());
        assert!(session_dir.join("insights/technical").exists());
        assert!(session_dir.join("insights/strategic").exists());
        assert!(session_dir.join("synthesis").exists());
        assert!(session_dir.join("output").exists());
        assert!(session_dir.join("session_metadata.json").exists());
    }
    
    #[test]
    fn test_load_session() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProcessingSessionManager::new(temp_dir.path()).unwrap();
        
        let source_files = vec![PathBuf::from("DTNote01.md")];
        let original_session = manager.create_session(
            "load_test".to_string(),
            source_files,
            None,
        ).unwrap();
        
        let loaded_session = manager.load_session("load_test").unwrap();
        
        assert_eq!(original_session.id, loaded_session.id);
        assert_eq!(original_session.name, loaded_session.name);
        assert_eq!(original_session.source_files, loaded_session.source_files);
    }
    
    #[test]
    fn test_list_sessions() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProcessingSessionManager::new(temp_dir.path()).unwrap();
        
        // Initially no sessions
        let sessions = manager.list_sessions().unwrap();
        assert!(sessions.is_empty());
        
        // Create some sessions
        manager.create_session("session1".to_string(), vec![], None).unwrap();
        manager.create_session("session2".to_string(), vec![], None).unwrap();
        
        let sessions = manager.list_sessions().unwrap();
        assert_eq!(sessions.len(), 2);
        assert!(sessions.contains(&"session1".to_string()));
        assert!(sessions.contains(&"session2".to_string()));
    }
}