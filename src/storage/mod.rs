// Level 4: Storage Module Organization
// - Manages database operations
// - Coordinates storage components
// - Handles persistence
// - Provides metrics collection

pub mod db;
pub mod batch;
pub mod index;

pub use db::Database;
pub use batch::BatchProcessor;
pub use index::StorageIndex;

// Re-export key types
pub use rocksdb::Options as DbOptions; 