//! CLI Metrics Display
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Metrics Display
//! - MetricsManager    (manages metrics display)
//! - DisplayFormatter  (formats metrics)
//! - OutputManager     (manages output)
//! 
//! Level 3: Metric Types
//! - PerformanceMetrics (performance stats)
//! - ResourceMetrics    (resource usage)
//! - OperationMetrics   (operation stats)
//! 
//! Level 2: Display Implementation
//! - MetricsRenderer   (renders metrics)
//! - UpdateManager     (manages updates)
//! - FormatManager     (manages formatting)
//! 
//! Level 1 (Base): Core Metrics Types
//! - MetricsConfig    (metrics configuration)
//! - MetricsFormat    (format configuration)
//! - DisplayError     (display errors)

use std::sync::Arc;
use tokio::sync::RwLock;
use metrics::{Counter, Gauge, Histogram};
use colored::*;
use crate::core::error::Result;

// ===== Level 1: Core Metrics Types =====
// Design Choice: Using metrics crate for collection

/// Metrics display configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Enable metrics display
    pub show_metrics: bool,
    /// Update interval
    pub update_interval: std::time::Duration,
    /// Output format
    pub format: MetricsFormat,
}

/// Metrics format options
#[derive(Debug, Clone, Copy)]
pub enum MetricsFormat {
    Plain,
    Colored,
    Json,
}

impl Default for MetricsFormat {
    fn default() -> Self {
        Self::Colored
    }
}

// ===== Level 2: Display Implementation =====
// Design Choice: Using colored output

/// Metrics display implementation
pub struct MetricsDisplay {
    /// Display configuration
    config: MetricsConfig,
    /// Performance metrics
    performance: Arc<RwLock<PerformanceMetrics>>,
    /// Resource metrics
    resources: Arc<RwLock<ResourceMetrics>>,
    /// Operation metrics
    operations: Arc<RwLock<OperationMetrics>>,
}

impl MetricsDisplay {
    /// Creates new metrics display
    pub fn new(format: MetricsFormat) -> Self {
        let config = MetricsConfig {
            show_metrics: true,
            update_interval: std::time::Duration::from_secs(1),
            format,
        };

        Self {
            config,
            performance: Arc::new(RwLock::new(PerformanceMetrics::new())),
            resources: Arc::new(RwLock::new(ResourceMetrics::new())),
            operations: Arc::new(RwLock::new(OperationMetrics::new())),
        }
    }

    /// Starts metrics display
    pub async fn start(&self) -> Result<()> {
        if !self.config.show_metrics {
            return Ok(());
        }

        self.clear_screen()?;
        self.draw_header()?;
        
        Ok(())
    }

    /// Stops metrics display
    pub async fn stop(&self) -> Result<()> {
        if !self.config.show_metrics {
            return Ok(());
        }

        self.draw_summary().await?;
        
        Ok(())
    }

    // ===== Level 3: Metric Types =====
    // Design Choice: Using separate metric groups

    /// Updates performance metrics
    pub async fn update_performance(&self, metrics: PerformanceMetrics) -> Result<()> {
        if !self.config.show_metrics {
            return Ok(());
        }

        let mut perf = self.performance.write().await;
        *perf = metrics;
        self.draw_performance(&perf)?;
        
        Ok(())
    }

    /// Updates resource metrics
    pub async fn update_resources(&self, metrics: ResourceMetrics) -> Result<()> {
        if !self.config.show_metrics {
            return Ok(());
        }

        let mut res = self.resources.write().await;
        *res = metrics;
        self.draw_resources(&res)?;
        
        Ok(())
    }

    // ===== Level 4: Display Formatting =====
    // Design Choice: Using terminal control sequences

    /// Clears the screen
    fn clear_screen(&self) -> Result<()> {
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    /// Draws the header
    fn draw_header(&self) -> Result<()> {
        match self.config.format {
            MetricsFormat::Plain => println!("=== Performance Metrics ==="),
            MetricsFormat::Colored => println!("{}", "=== Performance Metrics ===".blue().bold()),
            MetricsFormat::Json => println!("{{\"type\": \"header\"}}"),
        }
        Ok(())
    }

    /// Draws performance metrics
    fn draw_performance(&self, metrics: &PerformanceMetrics) -> Result<()> {
        match self.config.format {
            MetricsFormat::Plain => {
                println!("Tasks: {}", metrics.tasks.get());
                println!("Throughput: {} MB/s", metrics.throughput.get());
            }
            MetricsFormat::Colored => {
                println!("{}: {}", "Tasks".green(), metrics.tasks.get());
                println!("{}: {} MB/s", "Throughput".green(), metrics.throughput.get());
            }
            MetricsFormat::Json => {
                println!("{{\"tasks\": {}, \"throughput\": {}}}", 
                    metrics.tasks.get(), metrics.throughput.get());
            }
        }
        Ok(())
    }

    /// Draws resource metrics
    fn draw_resources(&self, metrics: &ResourceMetrics) -> Result<()> {
        match self.config.format {
            MetricsFormat::Plain => {
                println!("Memory: {} MB", metrics.memory.get());
                println!("CPU: {}%", metrics.cpu.get());
            }
            MetricsFormat::Colored => {
                println!("{}: {} MB", "Memory".yellow(), metrics.memory.get());
                println!("{}: {}%", "CPU".yellow(), metrics.cpu.get());
            }
            MetricsFormat::Json => {
                println!("{{\"memory\": {}, \"cpu\": {}}}", 
                    metrics.memory.get(), metrics.cpu.get());
            }
        }
        Ok(())
    }

    /// Draws summary
    async fn draw_summary(&self) -> Result<()> {
        let perf = self.performance.read().await;
        let res = self.resources.read().await;
        
        match self.config.format {
            MetricsFormat::Plain => {
                println!("\n=== Summary ===");
                println!("Total tasks: {}", perf.tasks.get());
                println!("Peak memory: {} MB", res.memory.get());
            }
            MetricsFormat::Colored => {
                println!("\n{}", "=== Summary ===".blue().bold());
                println!("{}: {}", "Total tasks".green(), perf.tasks.get());
                println!("{}: {} MB", "Peak memory".yellow(), res.memory.get());
            }
            MetricsFormat::Json => {
                println!("{{\"summary\": {{\"tasks\": {}, \"peak_memory\": {}}}}}", 
                    perf.tasks.get(), res.memory.get());
            }
        }
        Ok(())
    }
}

/// Performance metrics collection
#[derive(Debug)]
struct PerformanceMetrics {
    tasks: Counter,
    throughput: Gauge,
    latency: Histogram,
}

impl PerformanceMetrics {
    fn new() -> Self {
        Self {
            tasks: Counter::new(),
            throughput: Gauge::new(),
            latency: Histogram::new(),
        }
    }
}

/// Resource metrics collection
#[derive(Debug)]
struct ResourceMetrics {
    memory: Gauge,
    cpu: Gauge,
    disk: Gauge,
}

impl ResourceMetrics {
    fn new() -> Self {
        Self {
            memory: Gauge::new(),
            cpu: Gauge::new(),
            disk: Gauge::new(),
        }
    }
}

/// Operation metrics collection
#[derive(Debug)]
struct OperationMetrics {
    operations: Counter,
    errors: Counter,
    duration: Histogram,
}

impl OperationMetrics {
    fn new() -> Self {
        Self {
            operations: Counter::new(),
            errors: Counter::new(),
            duration: Histogram::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_display() {
        let display = MetricsDisplay::new(MetricsFormat::Plain);
        
        assert!(display.start().await.is_ok());
        
        let perf = PerformanceMetrics::new();
        assert!(display.update_performance(perf).await.is_ok());
        
        let res = ResourceMetrics::new();
        assert!(display.update_resources(res).await.is_ok());
        
        assert!(display.stop().await.is_ok());
    }
}
