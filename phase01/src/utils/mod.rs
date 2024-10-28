//! Utility Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Utility Coordination
//! - UtilityManager    (coordinates utilities)
//! - MetricsAggregator (aggregates utility metrics)
//! - ResourceTracker   (tracks resource usage)
//! 
//! Level 3: Resource Management
//! - BufferManager     (manages buffers)
//! - ResourceManager   (manages resources)
//! - CleanupManager    (manages cleanup)
//! 
//! Level 2: Utility Traits
//! - AsyncResource     (async resource trait)
//! - BufferPool       (buffer pool trait)
//! - Cleanup          (cleanup trait)
//! 
//! Level 1 (Base): Core Utility Types
//! - UtilityConfig    (utility configuration)
//! - ResourceMetrics  (resource metrics)
//! - UtilityError    (utility errors)

pub mod resource;
pub mod buffer;
pub mod cleanup;

// Re-export main types
pub use resource::ResourceManager;
pub use buffer::BufferPool;
pub use cleanup::CleanupGuard;
