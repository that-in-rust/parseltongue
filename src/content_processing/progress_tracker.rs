use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;

use crate::content_processing::types::*;

/// Tracks progress of content processing with chunk identifiers and line ranges
/// 
/// # Contract
/// - Maintains progress state for all chunks in a processing session
/// - Provides atomic updates to chunk status
/// - Persists progress to disk for recovery
/// - Generates progress reports and statistics
pub struct ProgressTracker {
    session_id: SessionId,
    progress_file: PathBuf,
    chunk_progress: HashMap<ChunkId, ChunkProgress>,
}

impl ProgressTracker {
    /// Creates a new progress tracker for a processing session
    /// 
    /// # Arguments
    /// * `session` - Processing session to track
    /// 
    /// # Returns
    /// * `Result<Self>` - New progress tracker or error if initialization fails
    pub fn new(session: &ProcessingSession) -> Result<Self> {
        let progress_file = session.output_directory.join("progress").join("progress.json");
        
        let mut tracker = Self {
            session_id: session.id,
            progress_file,
            chunk_progress: HashMap::new(),
        };
        
        // Try to load existing progress
        if let Err(_) = tracker.load_progress() {
            // If loading fails, start with empty progress
            tracker.save_progress()?;
        }
        
        Ok(tracker)
    }
    
    /// Registers a new chunk for progress tracking
    /// 
    /// # Arguments
    /// * `chunk` - Content chunk to track
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn register_chunk(&mut self, chunk: &ContentChunk) -> Result<()> {
        let progress = ChunkProgress::new(chunk);
        self.chunk_progress.insert(chunk.id, progress);
        self.save_progress()?;
        Ok(())
    }
    
    /// Updates the status of a chunk
    /// 
    /// # Arguments
    /// * `chunk_id` - ID of the chunk to update
    /// * `status` - New status for the chunk
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if chunk not found
    pub fn update_chunk_status(&mut self, chunk_id: ChunkId, status: ChunkStatus) -> Result<()> {
        let progress = self.chunk_progress.get_mut(&chunk_id)
            .ok_or(ContentProcessingError::ChunkNotFound { chunk_id })?;
        
        progress.update_status(status);
        self.save_progress()?;
        Ok(())
    }
    
    /// Gets the current status of a chunk
    /// 
    /// # Arguments
    /// * `chunk_id` - ID of the chunk to query
    /// 
    /// # Returns
    /// * `Result<ChunkStatus>` - Current status or error if chunk not found
    pub fn get_chunk_status(&self, chunk_id: ChunkId) -> Result<ChunkStatus> {
        let progress = self.chunk_progress.get(&chunk_id)
            .ok_or(ContentProcessingError::ChunkNotFound { chunk_id })?;
        
        Ok(progress.status.clone())
    }
    
    /// Gets progress information for a specific chunk
    /// 
    /// # Arguments
    /// * `chunk_id` - ID of the chunk to query
    /// 
    /// # Returns
    /// * `Result<&ChunkProgress>` - Progress information or error if chunk not found
    pub fn get_chunk_progress(&self, chunk_id: ChunkId) -> Result<&ChunkProgress> {
        self.chunk_progress.get(&chunk_id)
            .ok_or(ContentProcessingError::ChunkNotFound { chunk_id })
    }
    
    /// Gets all chunks with a specific status
    /// 
    /// # Arguments
    /// * `status` - Status to filter by
    /// 
    /// # Returns
    /// * `Vec<&ChunkProgress>` - List of chunks with the specified status
    pub fn get_chunks_by_status(&self, status: &ChunkStatus) -> Vec<&ChunkProgress> {
        self.chunk_progress.values()
            .filter(|progress| &progress.status == status)
            .collect()
    }
    
    /// Generates a progress summary for the session
    /// 
    /// # Returns
    /// * `ProgressSummary` - Summary of processing progress
    pub fn get_progress_summary(&self) -> ProgressSummary {
        let total_chunks = self.chunk_progress.len();
        let mut status_counts = HashMap::new();
        
        for progress in self.chunk_progress.values() {
            let status_key = match &progress.status {
                ChunkStatus::Pending => "pending",
                ChunkStatus::Processing => "processing", 
                ChunkStatus::Completed => "completed",
                ChunkStatus::Failed { .. } => "failed",
            };
            
            *status_counts.entry(status_key).or_insert(0) += 1;
        }
        
        let completed = status_counts.get("completed").unwrap_or(&0);
        let failed = status_counts.get("failed").unwrap_or(&0);
        let processing = status_counts.get("processing").unwrap_or(&0);
        let pending = status_counts.get("pending").unwrap_or(&0);
        
        let completion_percentage = if total_chunks > 0 {
            (*completed as f64 / total_chunks as f64) * 100.0
        } else {
            0.0
        };
        
        ProgressSummary {
            session_id: self.session_id,
            total_chunks,
            completed_chunks: *completed,
            failed_chunks: *failed,
            processing_chunks: *processing,
            pending_chunks: *pending,
            completion_percentage,
            last_updated: Utc::now(),
        }
    }
    
    /// Gets all chunk progress entries
    /// 
    /// # Returns
    /// * `Vec<&ChunkProgress>` - All chunk progress entries
    pub fn get_all_progress(&self) -> Vec<&ChunkProgress> {
        self.chunk_progress.values().collect()
    }
    
    /// Resets a failed chunk back to pending status
    /// 
    /// # Arguments
    /// * `chunk_id` - ID of the chunk to reset
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if chunk not found
    pub fn reset_chunk(&mut self, chunk_id: ChunkId) -> Result<()> {
        self.update_chunk_status(chunk_id, ChunkStatus::Pending)
    }
    
    /// Marks all processing chunks as failed (useful for recovery after crashes)
    /// 
    /// # Returns
    /// * `Result<usize>` - Number of chunks that were reset
    pub fn reset_processing_chunks(&mut self) -> Result<usize> {
        let processing_chunks: Vec<ChunkId> = self.chunk_progress.values()
            .filter(|p| matches!(p.status, ChunkStatus::Processing))
            .map(|p| p.chunk_id)
            .collect();
        
        let count = processing_chunks.len();
        
        for chunk_id in processing_chunks {
            self.update_chunk_status(chunk_id, ChunkStatus::Failed { 
                error: "Processing interrupted".to_string() 
            })?;
        }
        
        Ok(count)
    }
    
    /// Saves progress to disk
    fn save_progress(&self) -> Result<()> {
        let progress_data = ProgressData {
            session_id: self.session_id,
            chunk_progress: self.chunk_progress.clone(),
            last_saved: Utc::now(),
        };
        
        let json = serde_json::to_string_pretty(&progress_data)?;
        fs::write(&self.progress_file, json)?;
        Ok(())
    }
    
    /// Loads progress from disk
    fn load_progress(&mut self) -> Result<()> {
        if !self.progress_file.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.progress_file)?;
        let progress_data: ProgressData = serde_json::from_str(&content)?;
        
        // Verify session ID matches
        if progress_data.session_id != self.session_id {
            return Err(ContentProcessingError::SessionNotFound { 
                session_id: self.session_id 
            });
        }
        
        self.chunk_progress = progress_data.chunk_progress;
        Ok(())
    }
}

/// Summary of processing progress
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgressSummary {
    pub session_id: SessionId,
    pub total_chunks: usize,
    pub completed_chunks: usize,
    pub failed_chunks: usize,
    pub processing_chunks: usize,
    pub pending_chunks: usize,
    pub completion_percentage: f64,
    pub last_updated: chrono::DateTime<Utc>,
}

/// Internal structure for persisting progress data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ProgressData {
    session_id: SessionId,
    chunk_progress: HashMap<ChunkId, ChunkProgress>,
    last_saved: chrono::DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;
    
    fn create_test_session() -> ProcessingSession {
        let temp_dir = TempDir::new().unwrap();
        ProcessingSession::new(
            "test_session".to_string(),
            vec![PathBuf::from("test.md")],
            temp_dir.path().to_path_buf(),
            None,
        )
    }
    
    fn create_test_chunk() -> ContentChunk {
        ContentChunk::new(
            PathBuf::from("test.md"),
            LineRange::new(1, 300),
            "test content".to_string(),
            None,
            Some(LineRange::new(281, 300)),
        )
    }
    
    #[test]
    fn test_progress_tracker_creation() {
        let session = create_test_session();
        
        // Create progress directory
        std::fs::create_dir_all(session.output_directory.join("progress")).unwrap();
        
        let tracker = ProgressTracker::new(&session).unwrap();
        assert_eq!(tracker.session_id, session.id);
        assert!(tracker.chunk_progress.is_empty());
    }
    
    #[test]
    fn test_register_and_track_chunk() {
        let session = create_test_session();
        std::fs::create_dir_all(session.output_directory.join("progress")).unwrap();
        
        let mut tracker = ProgressTracker::new(&session).unwrap();
        let chunk = create_test_chunk();
        
        // Register chunk
        tracker.register_chunk(&chunk).unwrap();
        
        // Verify chunk is registered with pending status
        let status = tracker.get_chunk_status(chunk.id).unwrap();
        assert_eq!(status, ChunkStatus::Pending);
        
        // Update status to processing
        tracker.update_chunk_status(chunk.id, ChunkStatus::Processing).unwrap();
        let status = tracker.get_chunk_status(chunk.id).unwrap();
        assert_eq!(status, ChunkStatus::Processing);
        
        // Update status to completed
        tracker.update_chunk_status(chunk.id, ChunkStatus::Completed).unwrap();
        let status = tracker.get_chunk_status(chunk.id).unwrap();
        assert_eq!(status, ChunkStatus::Completed);
    }
    
    #[test]
    fn test_progress_summary() {
        let session = create_test_session();
        std::fs::create_dir_all(session.output_directory.join("progress")).unwrap();
        
        let mut tracker = ProgressTracker::new(&session).unwrap();
        
        // Register multiple chunks with different statuses
        let chunk1 = create_test_chunk();
        let chunk2 = create_test_chunk();
        let chunk3 = create_test_chunk();
        
        tracker.register_chunk(&chunk1).unwrap();
        tracker.register_chunk(&chunk2).unwrap();
        tracker.register_chunk(&chunk3).unwrap();
        
        tracker.update_chunk_status(chunk1.id, ChunkStatus::Completed).unwrap();
        tracker.update_chunk_status(chunk2.id, ChunkStatus::Processing).unwrap();
        // chunk3 remains pending
        
        let summary = tracker.get_progress_summary();
        assert_eq!(summary.total_chunks, 3);
        assert_eq!(summary.completed_chunks, 1);
        assert_eq!(summary.processing_chunks, 1);
        assert_eq!(summary.pending_chunks, 1);
        assert_eq!(summary.failed_chunks, 0);
        assert!((summary.completion_percentage - 33.33333333333333).abs() < 0.0001);
    }
    
    #[test]
    fn test_get_chunks_by_status() {
        let session = create_test_session();
        std::fs::create_dir_all(session.output_directory.join("progress")).unwrap();
        
        let mut tracker = ProgressTracker::new(&session).unwrap();
        
        let chunk1 = create_test_chunk();
        let chunk2 = create_test_chunk();
        
        tracker.register_chunk(&chunk1).unwrap();
        tracker.register_chunk(&chunk2).unwrap();
        
        tracker.update_chunk_status(chunk1.id, ChunkStatus::Completed).unwrap();
        // chunk2 remains pending
        
        let pending_chunks = tracker.get_chunks_by_status(&ChunkStatus::Pending);
        assert_eq!(pending_chunks.len(), 1);
        assert_eq!(pending_chunks[0].chunk_id, chunk2.id);
        
        let completed_chunks = tracker.get_chunks_by_status(&ChunkStatus::Completed);
        assert_eq!(completed_chunks.len(), 1);
        assert_eq!(completed_chunks[0].chunk_id, chunk1.id);
    }
    
    #[test]
    fn test_reset_processing_chunks() {
        let session = create_test_session();
        std::fs::create_dir_all(session.output_directory.join("progress")).unwrap();
        
        let mut tracker = ProgressTracker::new(&session).unwrap();
        
        let chunk1 = create_test_chunk();
        let chunk2 = create_test_chunk();
        
        tracker.register_chunk(&chunk1).unwrap();
        tracker.register_chunk(&chunk2).unwrap();
        
        tracker.update_chunk_status(chunk1.id, ChunkStatus::Processing).unwrap();
        tracker.update_chunk_status(chunk2.id, ChunkStatus::Processing).unwrap();
        
        let reset_count = tracker.reset_processing_chunks().unwrap();
        assert_eq!(reset_count, 2);
        
        // Both chunks should now be failed
        let status1 = tracker.get_chunk_status(chunk1.id).unwrap();
        let status2 = tracker.get_chunk_status(chunk2.id).unwrap();
        
        assert!(matches!(status1, ChunkStatus::Failed { .. }));
        assert!(matches!(status2, ChunkStatus::Failed { .. }));
    }
}