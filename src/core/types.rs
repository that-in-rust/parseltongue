// Level 4: Core Type Definitions
// - Defines fundamental types used across application
// - Implements key traits and conversions
// - Provides type safety guarantees
// - Enables efficient data handling

use std::path::PathBuf;
use bytes::Bytes;

// Level 3: Configuration Types
#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub worker_threads: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: std::time::Duration,
}

// Level 2: Data Types
#[derive(Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub data: Bytes,
    pub size: u64,
    pub crc32: u32,
}

// Level 1: Status Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Complete,
    Failed,
} 