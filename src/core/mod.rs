// Level 4: Core Module Organization
// - Manages fundamental types and traits
// - Coordinates runtime components
// - Handles error propagation
// - Provides core utilities

pub mod error;
pub mod channel;
pub mod runtime;
pub mod types;

// Re-export commonly used types
pub use error::{Error, Result};
pub use runtime::RuntimeManager;
pub use types::Config;