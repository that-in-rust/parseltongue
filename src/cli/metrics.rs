// Level 4: CLI Metrics Integration
// - Manages metrics display
// - Handles real-time updates
// - Implements custom formatters
// - Provides user feedback

use metrics::{Counter, Gauge, Histogram};
use std::sync::Arc;
use tokio::sync::watch;
use crate::core::error::Result;

// Level 3: Metrics Display
pub struct MetricsDisplay {
    processed: Counter,
    throughput: Gauge,
    latency: Histogram,
    update_tx: watch::Sender<MetricsUpdate>,
}

// Level 2: Update Types
#[derive(Clone, Debug)]
pub struct MetricsUpdate {
    files_processed: u64,
    bytes_processed: u64,
    current_throughput: f64,
    avg_latency_ms: f64,
}

impl MetricsDisplay {
    // Level 1: Display Operations
    pub async fn update(&self, update: MetricsUpdate) -> Result<()> {
        self.processed.increment(update.files_processed as u64);
        self.throughput.set(update.current_throughput);
        self.latency.record(update.avg_latency_ms);
        self.update_tx.send(update)
            .map_err(|_| crate::core::error::Error::Channel("Metrics update failed".into()))
    }
} 