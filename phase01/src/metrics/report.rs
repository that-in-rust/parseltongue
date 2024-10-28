//! Metrics Reporting Implementation
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Report Orchestration
//! - ReportManager     (manages reporting)
//! - ReportMetrics     (tracks reporting)
//! - ExportManager     (manages exports)
//! 
//! Level 3: Report Types
//! - JsonReporter      (JSON format)
//! - PrometheusReporter (Prometheus format)
//! - OpenTelemetryReporter (OpenTelemetry)
//! 
//! Level 2: Report Implementation
//! - AsyncReporter     (async reporting)
//! - ReportState       (report state)
//! - ReportBuffer      (report buffer)
//! 
//! Level 1 (Base): Core Report Types
//! - ReporterConfig    (reporter config)
//! - ReportResult      (result types)
//! - ReportError       (report errors)

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use metrics::{Counter, Gauge, Histogram};
use serde_json::Value;
use crate::core::error::Result;
use super::{MetricsRegistry, MetricsConfig, MetricsFormat};

// ===== Level 1: Core Report Types =====
// Design Choice: Using serde for serialization

/// Reporter configuration
#[derive(Debug, Clone)]
pub struct ReporterConfig {
    /// Report interval
    pub interval: Duration,
    /// Output format
    pub format: MetricsFormat,
    /// Output path
    pub output_path: std::path::PathBuf,
}

impl Default for ReporterConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(10),
            format: MetricsFormat::Json,
            output_path: std::path::PathBuf::from("metrics"),
        }
    }
}

// ===== Level 2: Report Implementation =====
// Design Choice: Using async reporting for efficiency

/// Metrics reporter implementation
pub struct MetricsReporter {
    /// Metrics registry
    registry: Arc<RwLock<MetricsRegistry>>,
    /// Reporter configuration
    config: MetricsConfig,
    /// Report task handle
    task: RwLock<Option<tokio::task::JoinHandle<()>>>,
    /// Reporter metrics
    metrics: ReporterMetrics,
}

impl MetricsReporter {
    /// Creates new metrics reporter
    pub fn new(registry: Arc<RwLock<MetricsRegistry>>, config: MetricsConfig) -> Self {
        Self {
            registry,
            config,
            task: RwLock::new(None),
            metrics: ReporterMetrics::new(),
        }
    }

    /// Starts metrics reporting
    pub async fn start(&self) -> Result<()> {
        let mut task = self.task.write().await;
        if task.is_some() {
            return Ok(());
        }

        let registry = self.registry.clone();
        let config = self.config.clone();
        let metrics = self.metrics.clone();

        *task = Some(tokio::spawn(async move {
            let mut interval = interval(config.interval);
            
            loop {
                interval.tick().await;
                metrics.reports.increment(1);
                
                if let Err(e) = Self::report_metrics(&registry, &config).await {
                    metrics.report_errors.increment(1);
                    tracing::error!("Metrics reporting error: {}", e);
                }
            }
        }));

        Ok(())
    }

    /// Stops metrics reporting
    pub async fn stop(&self) -> Result<()> {
        let mut task = self.task.write().await;
        if let Some(handle) = task.take() {
            handle.abort();
        }
        Ok(())
    }

    // ===== Level 3: Report Types =====
    // Design Choice: Using trait-based reporters

    /// Reports metrics in configured format
    async fn report_metrics(registry: &Arc<RwLock<MetricsRegistry>>, config: &MetricsConfig) -> Result<()> {
        let registry = registry.read().await;
        let output_path = config.output_dir.as_path();
        
        match config.format {
            MetricsFormat::Json => Self::report_json(&registry, output_path).await,
            MetricsFormat::Prometheus => Self::report_prometheus(&registry, output_path).await,
            MetricsFormat::OpenTelemetry => Self::report_opentelemetry(&registry).await,
        }
    }

    /// Reports metrics in JSON format
    async fn report_json(registry: &MetricsRegistry, output_dir: impl AsRef<Path>) -> Result<()> {
        let output_dir = output_dir.as_ref();
        let json = serde_json::to_value(registry)?;
        let path = output_dir.join(format!(
            "metrics_{}.json",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ));
        
        tokio::fs::create_dir_all(output_dir).await?;
        tokio::fs::write(path, serde_json::to_string_pretty(&json)?).await?;
        
        Ok(())
    }

    /// Reports metrics in Prometheus format
    async fn report_prometheus(registry: &MetricsRegistry, output_dir: impl AsRef<Path>) -> Result<()> {
        let output_dir = output_dir.as_ref();
        let mut output = String::new();
        
        // Format runtime metrics
        output.push_str(&format!("# Runtime Metrics\n"));
        output.push_str(&format!(
            "runtime_tasks_created {}\n",
            registry.runtime.tasks_created.get()
        ));
        
        // Write to file
        let path = output_dir.join(format!(
            "metrics_{}.prom",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ));
        
        tokio::fs::create_dir_all(output_dir).await?;
        tokio::fs::write(path, output).await?;
        
        Ok(())
    }

    /// Reports metrics in OpenTelemetry format
    async fn report_opentelemetry(registry: &MetricsRegistry) -> Result<()> {
        // OpenTelemetry implementation would go here
        Ok(())
    }
}

// ===== Level 4: Report Orchestration =====
// Design Choice: Using metrics for self-monitoring

/// Reporter metrics
#[derive(Debug, Clone)]
struct ReporterMetrics {
    reports: Counter,
    report_errors: Counter,
    report_duration: Histogram,
}

impl ReporterMetrics {
    fn new() -> Self {
        Self {
            reports: Counter::new(),
            report_errors: Counter::new(),
            report_duration: Histogram::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_metrics_reporter() {
        let temp_dir = TempDir::new().unwrap();
        let registry = Arc::new(RwLock::new(MetricsRegistry::new()));
        
        let config = MetricsConfig {
            enabled: true,
            interval: Duration::from_secs(1),
            format: MetricsFormat::Json,
            output_dir: temp_dir.path().to_path_buf(),
        };

        let reporter = MetricsReporter::new(registry, config);
        
        assert!(reporter.start().await.is_ok());
        
        // Wait for some reports
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        assert!(reporter.stop().await.is_ok());
        
        // Check if files were created
        let files = std::fs::read_dir(temp_dir.path()).unwrap();
        assert!(files.count() > 0);
    }
}
