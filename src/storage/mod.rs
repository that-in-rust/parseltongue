// Level 4: Storage Module
// - Coordinates database operations
// - Manages persistence layer
// - Handles data lifecycle

mod db;
pub use db::Database;

// Re-export key types
pub use rocksdb::Options as DbOptions; 