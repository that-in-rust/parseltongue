//! Core Module Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Module Coordination
//! - Public exports
//! - Type re-exports
//! - Common functionality
//! 
//! Level 3: Error Management
//! - Error types
//! - Result types
//! - Error conversion
//! 
//! Level 2: Type System
//! - Common types
//! - Type aliases
//! - Shared traits
//! 
//! Level 1 (Base): Core Infrastructure
//! - Basic types
//! - Constants
//! - Utilities

// Design Choice: Using explicit module structure
pub mod error;
pub mod types;

// Design Choice: Re-exporting common types
pub use error::{Error, Result};
pub use types::*;

// Design Choice: Common type aliases
pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxedFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;

// Design Choice: Core constants
pub const DEFAULT_BUFFER_SIZE: usize = 64 * 1024; // 64KB
pub const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
