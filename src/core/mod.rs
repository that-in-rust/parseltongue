// Level 4: Core Module
// - Manages core runtime functionality
// - Coordinates worker pools and tasks
// - Handles shutdown signals

pub mod runtime;

// Re-export commonly used types
pub use runtime::worker::WorkerPool;
pub use runtime::shutdown::ShutdownManager;