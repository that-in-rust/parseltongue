// Metrics collection for runtime statistics
//
// This module provides functionality to start and stop metrics collection.
// The design includes:
//
// - Initializing metrics exporters and collectors.
// - Periodically recording metrics to files.

use crate::output::OutputDirs;
use tokio::task::JoinHandle;
use tracing::{info, error};
use tokio_metrics::{TaskMonitor, TaskLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref METRICS_HANDLE: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
}

pub async fn start_collection(output_dirs: &OutputDirs) {
    // Initialize the task monitor.
    let (layer, collector) = TaskLayer::new();
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    // Spawn a task to collect metrics periodically.
    let metrics_file = output_dirs.metrics_dir.join("task-metrics.json");
    let handle = tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let stats = collector.snapshot();
            // Write metrics to file.
            if let Err(e) = tokio::fs::write(&metrics_file, serde_json::to_string(&stats).unwrap()).await {
                error!("Failed to write metrics: {}", e);
            }
        }
    });

    // Store the handle for graceful shutdown.
    *METRICS_HANDLE.lock().unwrap() = Some(handle);
}

pub async fn shutdown() {
    // Abort the metrics collection task.
    if let Some(handle) = METRICS_HANDLE.lock().unwrap().take() {
        handle.abort();
        if let Err(e) = handle.await {
            error!("Metrics collection task error: {}", e);
        }
    }
    info!("Metrics collection stopped.");
} 