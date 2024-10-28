//! Metrics Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Metrics Orchestration
//! - MetricsManager    (coordinates all metrics)
//! - MetricsRegistry   (global metrics store)
//! - MetricsExporter   (exports metrics)
//! 
//! Level 3: Metrics Types
//! - RuntimeMetrics    (runtime stats)
//! - StorageMetrics    (storage stats)
//! - ProcessingMetrics (processing stats)
//! 
//! Level 2: Metrics Implementation
//! - MetricsCollector  (collects metrics)
//! - MetricsReporter   (reports metrics)
//! - MetricsFormatter  (formats metrics)
//! 
//! Level 1 (Base): Core Metrics Types
//! - MetricsConfig    (metrics config)
//! - MetricsValue     (value types)
//! - MetricsError     (metrics errors)

pub mod collect;
pub mod report;

use std::sync::Arc;
use tokio::sync::RwLock;
use metrics::{Counter, Gauge, Histogram};
use crate::core::error::Result;
use std::path::PathBuf;
use std::path::Path;
use chrono::Utc;
use serde_json;

// ===== Level 1: Core Metrics Types =====
// Design Choice: Using metrics crate for standardization

/// Metrics configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Collection interval
    pub interval: std::time::Duration,
    /// Export format
    pub format: MetricsFormat,
    /// Output directory (owned PathBuf needed for config)
    pub output_dir: PathBuf,
}

/// Metrics format options
#[derive(Debug, Clone, Copy)]
pub enum MetricsFormat {
    Json,
    Prometheus,
    OpenTelemetry,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: std::time::Duration::from_secs(1),
            format: MetricsFormat::Json,
            output_dir: std::path::PathBuf::from("metrics"),
        }
    }
}

// ===== Level 2: Metrics Implementation =====
// Design Choice: Using RwLock for thread-safe metrics

/// Metrics manager implementation
pub struct MetricsManager {
    /// Metrics configuration
    config: MetricsConfig,
    /// Metrics registry
    registry: Arc<RwLock<MetricsRegistry>>,
    /// Metrics collector
    collector: Arc<collect::MetricsCollector>,
    /// Metrics reporter
    reporter: Arc<report::MetricsReporter>,
}

impl MetricsManager {
    /// Creates new metrics manager
    pub fn new(config: MetricsConfig) -> Self {
        let registry = Arc::new(RwLock::new(MetricsRegistry::new()));
        let collector = Arc::new(collect::MetricsCollector::new(registry.clone()));
        let reporter = Arc::new(report::MetricsReporter::new(registry.clone(), config.clone()));

        Self {
            config,
            registry,
            collector,
            reporter,
        }
    }

    /// Starts metrics collection and reporting
    pub async fn start(&self) -> Result<()> {
        if self.config.enabled {
            self.collector.start().await?;
            self.reporter.start().await?;
        }
        Ok(())
    }

    /// Stops metrics collection and reporting
    pub async fn stop(&self) -> Result<()> {
        if self.config.enabled {
            self.collector.stop().await?;
            self.reporter.stop().await?;
        }
        Ok(())
    }

    /// Records a counter metric
    pub async fn record_counter(&self, name: &str, value: u64) -> Result<()> {
        if self.config.enabled {
            let mut registry = self.registry.write().await;
            registry.record_counter(name, value);
        }
        Ok(())
    }

    /// Records a gauge metric
    pub async fn record_gauge(&self, name: &str, value: f64) -> Result<()> {
        if self.config.enabled {
            let mut registry = self.registry.write().await;
            registry.record_gauge(name, value);
        }
        Ok(())
    }

    /// Records a histogram metric
    pub async fn record_histogram(&self, name: &str, value: f64) -> Result<()> {
        if self.config.enabled {
            let mut registry = self.registry.write().await;
            registry.record_histogram(name, value);
        }
        Ok(())
    }

    /// Writes metrics to path
    pub async fn write_metrics(&self, path: impl AsRef<Path>) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let path = path.as_ref();
        let registry = self.registry.read().await;

        match self.config.format {
            MetricsFormat::Json => {
                let json = serde_json::to_value(&*registry)?;
                tokio::fs::write(path, serde_json::to_string_pretty(&json)?).await?;
            }
            MetricsFormat::Prometheus => {
                let output = self.format_prometheus(&registry)?;
                tokio::fs::write(path, output).await?;
            }
            MetricsFormat::OpenTelemetry => {
                // OpenTelemetry implementation
            }
        }

        Ok(())
    }

    /// Gets metrics file path
    fn get_metrics_path(&self) -> PathBuf {
        self.config.output_dir.join(format!(
            "metrics_{}.{}",
            Utc::now().format("%Y%m%d_%H%M%S"),
            match self.config.format {
                MetricsFormat::Json => "json",
                MetricsFormat::Prometheus => "prom",
                MetricsFormat::OpenTelemetry => "otlp",
            }
        ))
    }
}

// ===== Level 3: Metrics Types =====
// Design Choice: Using separate metric groups

/// Metrics registry implementation
#[derive(Debug)]
pub struct MetricsRegistry {
    /// Runtime metrics
    runtime: RuntimeMetrics,
    /// Storage metrics
    storage: StorageMetrics,
    /// Processing metrics
    processing: ProcessingMetrics,
}

impl MetricsRegistry {
    fn new() -> Self {
        Self {
            runtime: RuntimeMetrics::new(),
            storage: StorageMetrics::new(),
            processing: ProcessingMetrics::new(),
        }
    }

    fn record_counter(&mut self, name: &str, value: u64) {
        match name {
            s if s.starts_with("runtime") => self.runtime.record_counter(s, value),
            s if s.starts_with("storage") => self.storage.record_counter(s, value),
            s if s.starts_with("processing") => self.processing.record_counter(s, value),
            _ => {}
        }
    }

    fn record_gauge(&mut self, name: &str, value: f64) {
        match name {
            s if s.starts_with("runtime") => self.runtime.record_gauge(s, value),
            s if s.starts_with("storage") => self.storage.record_gauge(s, value),
            s if s.starts_with("processing") => self.processing.record_gauge(s, value),
            _ => {}
        }
    }

    fn record_histogram(&mut self, name: &str, value: f64) {
        match name {
            s if s.starts_with("runtime") => self.runtime.record_histogram(s, value),
            s if s.starts_with("storage") => self.storage.record_histogram(s, value),
            s if s.starts_with("processing") => self.processing.record_histogram(s, value),
            _ => {}
        }
    }
}

// ===== Level 4: Metrics Orchestration =====
// Design Choice: Using separate metric groups

/// Runtime metrics collection
#[derive(Debug)]
pub struct RuntimeMetrics {
    pub tasks_created: Counter,
    pub tasks_completed: Counter,
    pub active_tasks: Gauge,
    pub task_duration: Histogram,
}

impl RuntimeMetrics {
    fn new() -> Self {
        Self {
            tasks_created: Counter::new(),
            tasks_completed: Counter::new(),
            active_tasks: Gauge::new(),
            task_duration: Histogram::new(),
        }
    }

    fn record_counter(&mut self, name: &str, value: u64) {
        match name {
            "runtime.tasks_created" => self.tasks_created.increment(value),
            "runtime.tasks_completed" => self.tasks_completed.increment(value),
            _ => {}
        }
    }

    fn record_gauge(&mut self, name: &str, value: f64) {
        if name == "runtime.active_tasks" {
            self.active_tasks.set(value);
        }
    }

    fn record_histogram(&mut self, name: &str, value: f64) {
        if name == "runtime.task_duration" {
            self.task_duration.record(value);
        }
    }
}

/// Storage metrics collection
#[derive(Debug)]
pub struct StorageMetrics {
    pub bytes_written: Counter,
    pub bytes_read: Counter,
    pub active_connections: Gauge,
    pub operation_duration: Histogram,
}

impl StorageMetrics {
    fn new() -> Self {
        Self {
            bytes_written: Counter::new(),
            bytes_read: Counter::new(),
            active_connections: Gauge::new(),
            operation_duration: Histogram::new(),
        }
    }

    fn record_counter(&mut self, name: &str, value: u64) {
        match name {
            "storage.bytes_written" => self.bytes_written.increment(value),
            "storage.bytes_read" => self.bytes_read.increment(value),
            _ => {}
        }
    }

    fn record_gauge(&mut self, name: &str, value: f64) {
        if name == "storage.active_connections" {
            self.active_connections.set(value);
        }
    }

    fn record_histogram(&mut self, name: &str, value: f64) {
        if name == "storage.operation_duration" {
            self.operation_duration.record(value);
        }
    }
}

/// Processing metrics collection
#[derive(Debug)]
pub struct ProcessingMetrics {
    pub entries_processed: Counter,
    pub bytes_processed: Counter,
    pub active_processors: Gauge,
    pub processing_duration: Histogram,
}

impl ProcessingMetrics {
    fn new() -> Self {
        Self {
            entries_processed: Counter::new(),
            bytes_processed: Counter::new(),
            active_processors: Gauge::new(),
            processing_duration: Histogram::new(),
        }
    }

    fn record_counter(&mut self, name: &str, value: u64) {
        match name {
            "processing.entries_processed" => self.entries_processed.increment(value),
            "processing.bytes_processed" => self.bytes_processed.increment(value),
            _ => {}
        }
    }

    fn record_gauge(&mut self, name: &str, value: f64) {
        if name == "processing.active_processors" {
            self.active_processors.set(value);
        }
    }

    fn record_histogram(&mut self, name: &str, value: f64) {
        if name == "processing.processing_duration" {
            self.processing_duration.record(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_metrics_manager() {
        let temp_dir = TempDir::new().unwrap();
        
        let config = MetricsConfig {
            enabled: true,
            interval: Duration::from_secs(1),
            format: MetricsFormat::Json,
            output_dir: temp_dir.path().to_path_buf(),
        };

        let manager = MetricsManager::new(config);
        
        assert!(manager.start().await.is_ok());
        
        // Test recording metrics
        manager.record_counter("runtime.tasks_created", 1).await.unwrap();
        manager.record_gauge("storage.active_connections", 2.0).await.unwrap();
        manager.record_histogram("processing.processing_duration", 100.0).await.unwrap();
        
        assert!(manager.stop().await.is_ok());
    }
}
