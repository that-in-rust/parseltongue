//! Metrics Collection Implementation
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Collection Orchestration
//! - CollectionManager (manages collection)
//! - CollectionMetrics (tracks collection)
//! - CollectionTasks   (collection tasks)
//! 
//! Level 3: Collection Types
//! - SystemCollector   (system metrics)
//! - RuntimeCollector  (runtime metrics)
//! - CustomCollector   (custom metrics)
//! 
//! Level 2: Collection Implementation
//! - AsyncCollector    (async collection)
//! - CollectionState   (collection state)
//! - MetricsBuffer     (metrics buffer)
//! 
//! Level 1 (Base): Core Collection Types
//! - CollectorConfig   (collector config)
//! - CollectionResult  (result types)
//! - CollectionError   (collection errors)

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use metrics::{Counter, Gauge, Histogram};
use crate::core::error::Result;
use super::{MetricsRegistry, MetricsConfig};

// ===== Level 1: Core Collection Types =====
// Design Choice: Using async collection for efficiency

/// Collector configuration
#[derive(Debug, Clone)]
pub struct CollectorConfig {
    /// Collection interval
    pub interval: Duration,
    /// Buffer size
    pub buffer_size: usize,
    /// Enable system metrics
    pub system_metrics: bool,
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            buffer_size: 1000,
            system_metrics: true,
        }
    }
}

// ===== Level 2: Collection Implementation =====
// Design Choice: Using interval-based collection

/// Metrics collector implementation
pub struct MetricsCollector {
    /// Metrics registry
    registry: Arc<RwLock<MetricsRegistry>>,
    /// Collection task handle
    task: RwLock<Option<tokio::task::JoinHandle<()>>>,
    /// Collector metrics
    metrics: CollectorMetrics,
    /// Collector configuration
    config: CollectorConfig,
}

impl MetricsCollector {
    /// Creates new metrics collector
    pub fn new(registry: Arc<RwLock<MetricsRegistry>>) -> Self {
        Self {
            registry,
            task: RwLock::new(None),
            metrics: CollectorMetrics::new(),
            config: CollectorConfig::default(),
        }
    }

    /// Starts metrics collection
    pub async fn start(&self) -> Result<()> {
        let mut task = self.task.write().await;
        if task.is_some() {
            return Ok(());
        }

        let registry = self.registry.clone();
        let metrics = self.metrics.clone();
        let config = self.config.clone();

        *task = Some(tokio::spawn(async move {
            let mut interval = interval(config.interval);
            
            loop {
                interval.tick().await;
                metrics.collections.increment(1);
                
                if let Err(e) = Self::collect_metrics(&registry).await {
                    metrics.collection_errors.increment(1);
                    tracing::error!("Metrics collection error: {}", e);
                }
            }
        }));

        Ok(())
    }

    /// Stops metrics collection
    pub async fn stop(&self) -> Result<()> {
        let mut task = self.task.write().await;
        if let Some(handle) = task.take() {
            handle.abort();
        }
        Ok(())
    }

    // ===== Level 3: Collection Types =====
    // Design Choice: Using separate collectors for different metrics

    /// Collects all metrics
    async fn collect_metrics(registry: &Arc<RwLock<MetricsRegistry>>) -> Result<()> {
        let mut registry = registry.write().await;
        
        // Collect system metrics
        Self::collect_system_metrics(&mut registry)?;
        
        // Collect runtime metrics
        Self::collect_runtime_metrics(&mut registry)?;
        
        // Collect custom metrics
        Self::collect_custom_metrics(&mut registry)?;
        
        Ok(())
    }

    /// Collects system metrics
    fn collect_system_metrics(registry: &mut MetricsRegistry) -> Result<()> {
        let sys_info = sys_info::System::new();
        
        // CPU usage
        if let Ok(cpu) = sys_info.cpu_load_aggregate() {
            registry.record_gauge("system.cpu_usage", cpu.user * 100.0);
        }

        // Memory usage
        if let Ok(mem) = sys_info.memory() {
            let used_mem = (mem.total - mem.free) as f64;
            registry.record_gauge("system.memory_usage", used_mem);
        }

        Ok(())
    }

    /// Collects runtime metrics
    fn collect_runtime_metrics(registry: &mut MetricsRegistry) -> Result<()> {
        // Thread count
        registry.record_gauge("runtime.thread_count", 
            std::thread::available_parallelism()?.get() as f64);

        Ok(())
    }

    /// Collects custom metrics
    fn collect_custom_metrics(registry: &mut MetricsRegistry) -> Result<()> {
        // Custom metrics collection
        Ok(())
    }
}

// ===== Level 4: Collection Orchestration =====
// Design Choice: Using metrics for self-monitoring

/// Collector metrics
#[derive(Debug, Clone)]
struct CollectorMetrics {
    collections: Counter,
    collection_errors: Counter,
    collection_duration: Histogram,
}

impl CollectorMetrics {
    fn new() -> Self {
        Self {
            collections: Counter::new(),
            collection_errors: Counter::new(),
            collection_duration: Histogram::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_collector() {
        let registry = Arc::new(RwLock::new(MetricsRegistry::new()));
        let collector = MetricsCollector::new(registry);
        
        assert!(collector.start().await.is_ok());
        
        // Wait for some collections
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        assert!(collector.stop().await.is_ok());
    }
}
