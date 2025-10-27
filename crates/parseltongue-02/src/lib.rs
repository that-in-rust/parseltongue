//! Tool 1: folder-to-cozoDB-streamer
//!
//! Following TDD-first principle - tests first, implementation second
//! Implements folder ingestion with tree-sitter parsing and CozoDB storage

pub mod chunking;
pub mod discovery;
pub mod error;
pub mod parser;
pub mod storage;
pub mod streamer;

// Re-export key components for convenience
pub use chunking::{Chunk, ChunkStrategy, Chunker};
pub use discovery::{FileDiscovery, RustFileFilter};
pub use error::{ToolError, ToolResult};
pub use parser::{ParseResult, TreeSitterRustParser};
pub use storage::{CozoDBConnection, IngestionResult};
pub use streamer::{FolderToCozoDBStreamer, StreamConfig};
