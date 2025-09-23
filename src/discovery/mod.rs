//! Discovery Infrastructure for Parseltongue v2
//! 
//! Discovery-first architectural intelligence tool that transforms entity discovery
//! from a 5+ minute bottleneck to a <30 second interactive experience.
//! 
//! Core components:
//! - String interning system for memory-efficient file path storage
//! - DiscoveryEngine trait for entity exploration
//! - Discovery indexes for fast entity listing and filtering

pub mod string_interning;
pub mod engine;
pub mod types;
pub mod error;
pub mod enhanced_isg_node;

// Re-export core types for convenience
pub use string_interning::{FileId, FileInterner};
pub use engine::DiscoveryEngine;
pub use types::{EntityInfo, FileLocation, DiscoveryQuery, DiscoveryResult};
pub use error::{DiscoveryError, DiscoveryResult as Result};
pub use enhanced_isg_node::{EnhancedIsgNode, NodeConverter};