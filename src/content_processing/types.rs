use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Unique identifier for a processing session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for SessionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<SessionId> for Uuid {
    fn from(id: SessionId) -> Self {
        id.0
    }
}

/// Unique identifier for a content chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkId(pub Uuid);

impl ChunkId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for ChunkId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ChunkId> for Uuid {
    fn from(id: ChunkId) -> Self {
        id.0
    }
}

/// Represents a range of lines in a source file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
}

impl LineRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Represents a chunk of content from a source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub id: ChunkId,
    pub source_file: PathBuf,
    pub line_range: LineRange,
    pub content: String,
    pub overlap_start: Option<LineRange>,
    pub overlap_end: Option<LineRange>,
    pub created_at: DateTime<Utc>,
}

impl ContentChunk {
    pub fn new(
        source_file: PathBuf,
        line_range: LineRange,
        content: String,
        overlap_start: Option<LineRange>,
        overlap_end: Option<LineRange>,
    ) -> Self {
        Self {
            id: ChunkId::new(),
            source_file,
            line_range,
            content,
            overlap_start,
            overlap_end,
            created_at: Utc::now(),
        }
    }
}

/// Processing status for a chunk
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkStatus {
    Pending,
    Processing,
    Completed,
    Failed { error: String },
}

/// Progress information for a chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkProgress {
    pub chunk_id: ChunkId,
    pub source_file: PathBuf,
    pub line_range: LineRange,
    pub status: ChunkStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChunkProgress {
    pub fn new(chunk: &ContentChunk) -> Self {
        let now = Utc::now();
        Self {
            chunk_id: chunk.id,
            source_file: chunk.source_file.clone(),
            line_range: chunk.line_range.clone(),
            status: ChunkStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_status(&mut self, status: ChunkStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

/// Configuration for content segmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentationConfig {
    pub chunk_size: usize,
    pub overlap_size: usize,
}

impl Default for SegmentationConfig {
    fn default() -> Self {
        Self {
            chunk_size: 300,  // 300 lines per chunk as per requirements
            overlap_size: 20, // 20 lines overlap as per requirements
        }
    }
}

/// Processing session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingSession {
    pub id: SessionId,
    pub name: String,
    pub source_files: Vec<PathBuf>,
    pub output_directory: PathBuf,
    pub config: SegmentationConfig,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProcessingSession {
    pub fn new(
        name: String,
        source_files: Vec<PathBuf>,
        output_directory: PathBuf,
        config: Option<SegmentationConfig>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: SessionId::new(),
            name,
            source_files,
            output_directory,
            config: config.unwrap_or_default(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Error types for content processing
#[derive(Debug, thiserror::Error)]
pub enum ContentProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid line range: start={start}, end={end}")]
    InvalidLineRange { start: usize, end: usize },
    
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    
    #[error("Session not found: {session_id:?}")]
    SessionNotFound { session_id: SessionId },
    
    #[error("Chunk not found: {chunk_id:?}")]
    ChunkNotFound { chunk_id: ChunkId },
    
    #[error("Directory creation failed: {path}")]
    DirectoryCreationFailed { path: PathBuf },
}

pub type Result<T> = std::result::Result<T, ContentProcessingError>;