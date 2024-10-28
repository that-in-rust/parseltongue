//! Tokio Console Metrics Integration
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Console Integration
//! - ConsoleManager    (manages console)
//!   ├── Metric export
//!   ├── Event handling
//!   └── Display updates
//! 
//! Level 3: Metric Types
//! - RuntimeMetrics    (runtime stats)
//! - TaskMetrics      (task stats)
//! - ResourceMetrics  (resource stats)
//! 
//! Level 2: Implementation
//! - MetricCollector  (collects metrics)
//! - EventHandler     (handles events)
//! - DisplayUpdater   (updates display)
//! 
//! Level 1 (Base): Core Types
//! - ConsoleConfig    (console config)
//! - ConsoleEvent     (event types)
//! - ConsoleError     (error types)

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use metrics::{Counter, Gauge, Histogram};
use crate::core::error::Result;
use super::{MetricsRegistry, MetricsConfig};

// Design Choice: Using tokio-console for runtime insights
pub struct ConsoleManager {
    registry: Arc<RwLock<MetricsRegistry>>,
    config: ConsoleConfig,
    metrics: ConsoleMetrics,
}

impl ConsoleManager {
    pub fn new(registry: Arc<RwLock<MetricsRegistry>>, config: ConsoleConfig) -> Self {
        Self {
            registry,
            config,
            metrics: ConsoleMetrics::new(),
        }
    }

    pub async fn start(&self) -> Result<()> {
        // Initialize tokio-console
        console_subscriber::init();
        Ok(())
    }

    pub async fn record_event(&self, event: ConsoleEvent) -> Result<()> {
        match event {
            ConsoleEvent::TaskSpawned => self.metrics.tasks_spawned.increment(1),
            ConsoleEvent::TaskCompleted => self.metrics.tasks_completed.increment(1),
            ConsoleEvent::ResourceUsage(usage) => self.metrics.resource_usage.set(usage),
        }
        Ok(())
    }
}

// Design Choice: Using separate types for metrics
#[derive(Debug)]
struct ConsoleMetrics {
    tasks_spawned: Counter,
    tasks_completed: Counter,
    resource_usage: Gauge,
    event_latency: Histogram,
}

impl ConsoleMetrics {
    fn new() -> Self {
        Self {
            tasks_spawned: Counter::new(),
            tasks_completed: Counter::new(),
            resource_usage: Gauge::new(),
            event_latency: Histogram::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleConfig {
    pub enabled: bool,
    pub update_interval: Duration,
    pub max_events: usize,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            update_interval: Duration::from_millis(100),
            max_events: 1000,
        }
    }
}

#[derive(Debug)]
pub enum ConsoleEvent {
    TaskSpawned,
    TaskCompleted,
    ResourceUsage(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_console_manager() {
        let registry = Arc::new(RwLock::new(MetricsRegistry::new()));
        let config = ConsoleConfig::default();
        let console = ConsoleManager::new(registry, config);

        assert!(console.start().await.is_ok());
        assert!(console.record_event(ConsoleEvent::TaskSpawned).await.is_ok());
        
        sleep(Duration::from_millis(100)).await;
        
        assert!(console.record_event(ConsoleEvent::TaskCompleted).await.is_ok());
    }
}
