//! Metrics Module - Pyramidal Structure
//! Layer 1: Public Interface
//! Layer 2: Metrics Collection
//! Layer 3: Metrics Processing
//! Layer 4: Storage & Export
//! Layer 5: Cleanup & Resource Management

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, mpsc};
use anyhow::Result;
use serde::{Serialize, Deserialize};

pub mod console;
pub mod task;

// Consider adding:
pub mod tracing; // For structured logging with tokio trace contexts

// Layer 1: Core Types
#[derive(Clone)]
pub struct MetricsManager {
    inner: Arc<MetricsInner>,
}

struct MetricsInner {
    metrics: RwLock<Metrics>,
    tx: mpsc::Sender<MetricEvent>,
}

// Layer 2: Metric Types
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metrics {
    start_time: Option<Instant>,
    processed_files: usize,
    total_bytes: u64,
    errors: usize,
    operation_durations: Vec<Duration>,
}

#[derive(Debug)]
enum MetricEvent {
    FileProcessed { size: u64, duration: Duration },
    Error { context: String },
    Complete,
}

// Layer 3: Implementation
impl MetricsManager {
    pub fn new() -> (Self, mpsc::Receiver<MetricEvent>) {
        let (tx, rx) = mpsc::channel(1000);
        let manager = Self {
            inner: Arc::new(MetricsInner {
                metrics: RwLock::new(Metrics::default()),
                tx,
            }),
        };
        (manager, rx)
    }

    // Layer 4: Metric Collection
    pub async fn record_file(&self, size: u64, duration: Duration) -> Result<()> {
        self.inner.tx.send(MetricEvent::FileProcessed { 
            size, 
            duration 
        }).await?;
        
        let mut metrics = self.inner.metrics.write().await;
        metrics.processed_files += 1;
        metrics.total_bytes += size;
        metrics.operation_durations.push(duration);
        
        Ok(())
    }

    pub async fn record_error(&self, context: String) -> Result<()> {
        self.inner.tx.send(MetricEvent::Error { context }).await?;
        
        let mut metrics = self.inner.metrics.write().await;
        metrics.errors += 1;
        
        Ok(())
    }

    // Layer 5: Metrics Export
    pub async fn export_metrics(&self) -> Result<String> {
        let metrics = self.inner.metrics.read().await;
        Ok(serde_json::to_string_pretty(&*metrics)?)
    }
}

// Error Types
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {
    #[error("Failed to record metric: {0}")]
    RecordError(String),
    
    #[error("Failed to export metrics: {0}")]
    ExportError(String),
}

pub use console::ConsoleMetrics;
pub use task::TaskMetrics;
