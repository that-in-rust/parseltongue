//! Common Type Definitions
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Type API
//! - Public type exports
//! - Type conversions
//! - Type utilities
//! 
//! Level 3: Domain Types
//! - Task types
//! - Resource types
//! - Configuration types
//! 
//! Level 2: Type Implementation
//! - Type conversion
//! - Type validation
//! - Type formatting
//! 
//! Level 1 (Base): Basic Types
//! - Primitive types
//! - Type aliases
//! - Common traits

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use bytes::Bytes;

// Design Choice: Using strong typing for domain concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetadata {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub priority: TaskPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    High,
    Normal,
    Low,
}

// Design Choice: Using newtype pattern for strong typing
#[derive(Debug, Clone)]
pub struct ResourceId(pub String);

#[derive(Debug, Clone)]
pub struct TaskId(pub String);

// Design Choice: Using type aliases for common types
pub type Buffer = Bytes;
pub type Path = PathBuf;
