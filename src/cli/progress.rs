//! Progress Display Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Progress Coordination
//! - Progress bar management
//! - Style configuration
//! - Update coordination
//! 
//! Level 3: Display Management
//! - Progress updates
//! - Message formatting
//! - Style application
//! 
//! Level 2: Progress Implementation
//! - Bar rendering
//! - Update handling
//! - Cleanup handling
//! 
//! Level 1 (Base): Core Types
//! - Progress types
//! - Style types
//! - Message types

use std::sync::Arc;
use indicatif::{ProgressBar as InternalProgressBar, ProgressStyle};

// Design Choice: Using wrapper for progress bar
#[derive(Clone)]
pub struct ProgressBar {
    inner: Arc<InternalProgressBar>,
}

impl ProgressBar {
    pub fn new() -> Self {
        let inner = InternalProgressBar::new_spinner();
        inner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn set_message(&self, msg: impl Into<String>) {
        self.inner.set_message(msg.into());
    }

    pub fn finish_with_message(&self, msg: impl Into<String>) {
        self.inner.finish_with_message(msg.into());
    }

    pub fn inc(&self, delta: u64) {
        self.inner.inc(delta);
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}
