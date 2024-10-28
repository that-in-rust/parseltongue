//! Metrics Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Metrics Orchestration
//! - MetricsManager    (coordinates metrics)
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
use metrics::{Counter, Gauge, Histogram};
use tokio::sync::RwLock;
use crate::core::error::Result;

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
}

/// Metrics format options
#[derive(Debug, Clone, Copy)]
pub enum MetricsFormat {
    Json,
    Prometheus,
    OpenTelemetry,
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

    /// Starts metrics collection
    pub async fn start(&self) -> Result<()> {
        if self.config.enabled {
            self.collector.start().await?;
            self.reporter.start().await?;
        }
        Ok(())
    }

    /// Stops metrics collection
    pub async fn stop(&self) -> Result<()> {
        if self.config.enabled {
            self.collector.stop().await?;
            self.reporter.stop().await?;
        }
        Ok(())
    }
}

// ===== Level 3: Metrics Types =====
// Design Choice: Using builder pattern for metrics

/// Metrics registry implementation
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
}

// ===== Level 4: Metrics Orchestration =====
// Design Choice: Using separate metric groups

/// Runtime metrics collection
#[derive(Debug, Default)]
pub struct RuntimeMetrics {
    pub tasks_created: Counter,
    pub tasks_completed: Counter,
    pub active_tasks: Gauge,
    pub task_duration: Histogram,
}

/// Storage metrics collection
#[derive(Debug, Default)]
pub struct StorageMetrics {
    pub bytes_written: Counter,
    pub bytes_read: Counter,
    pub active_connections: Gauge,
    pub operation_duration: Histogram,
}

/// Processing metrics collection
#[derive(Debug, Default)]
pub struct ProcessingMetrics {
    pub entries_processed: Counter,
    pub bytes_processed: Counter,
    pub active_processors: Gauge,
    pub processing_duration: Histogram,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_metrics_manager() {
        let config = MetricsConfig {
            enabled: true,
            interval: Duration::from_secs(1),
            format: MetricsFormat::Json,
        };

        let manager = MetricsManager::new(config);
        
        assert!(manager.start().await.is_ok());
        assert!(manager.stop().await.is_ok());
    }
}
