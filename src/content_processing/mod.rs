use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

pub mod user_journey_extractor;
pub mod strategic_theme_organizer;
pub mod cross_reference_synthesizer;

/// Unique identifier for content chunks
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

/// Line range for content chunks
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

/// Content chunk for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub id: ChunkId,
    pub content: String,
    pub line_range: LineRange,
    pub source_file: String,
    pub created_at: DateTime<Utc>,
}

impl ContentChunk {
    pub fn new(content: String, line_range: LineRange, source_file: String) -> Self {
        Self {
            id: ChunkId::new(),
            content,
            line_range,
            source_file,
            created_at: Utc::now(),
        }
    }
}

/// Content processing errors
#[derive(Error, Debug)]
pub enum ContentProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid content chunk: {reason}")]
    InvalidChunk { reason: String },
    
    #[error("Analysis failed: {reason}")]
    AnalysisFailed { reason: String },
    
    #[error("Extraction failed for {entity_type}: {reason}")]
    ExtractionFailed { entity_type: String, reason: String },
    
    #[error("Validation failed: {reason}")]
    ValidationFailed { reason: String },
}

/// Result type for content processing operations
pub type Result<T> = std::result::Result<T, ContentProcessingError>;

/// Journey candidate for extraction
#[derive(Debug, Clone)]
pub struct JourneyCandidate {
    pub start_line: usize,
    pub end_line: usize,
    pub content_lines: Vec<String>,
    pub persona_hints: Vec<user_journey_extractor::DeveloperPersona>,
    pub workflow_hints: Vec<user_journey_extractor::WorkflowType>,
}