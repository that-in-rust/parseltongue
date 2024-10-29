// Level 4: Metrics Collection
// - Implements metrics gathering
// - Handles async collection
// - Manages metric types
// - Provides aggregation points

use metrics::{Counter, Gauge, Histogram};
use std::sync::Arc;
use tokio::sync::RwLock;

// Level 3: Metric Types
pub struct MetricsCollector {
    processed_files: Counter,
    active_workers: Gauge,
    processing_time: Histogram,
    error_count: Counter,
}

impl MetricsCollector {
    // Level 2: Collection Operations
    pub fn record_processed(&self) {
        self.processed_files.increment(1);
    }

    pub fn update_workers(&self, count: i64) {
        self.active_workers.set(count as f64);
    }

    // Level 1: Time Tracking
    pub fn record_duration(&self, duration_ms: u64) {
        self.processing_time.record(duration_ms as f64);
    }
} 