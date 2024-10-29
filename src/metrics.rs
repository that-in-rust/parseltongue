// Level 4: Runtime Metrics Collection
// - Integrates tokio-metrics for runtime statistics
// - Records metrics to files
// - Manages worker metrics

use tokio_metrics::RuntimeMonitor;
use tokio::time::{self, Duration};
use crate::output::OutputDirs;
use std::fs::File;
use std::io::Write;
use crate::error::Result;
use std::sync::{Arc, Mutex};

pub struct WorkerMetrics {
    metrics: Mutex<Vec<WorkerStat>>,
}

struct WorkerStat {
    worker_id: usize,
    task_count: u64,
    error_count: u64,
    total_duration: Duration,
}

impl WorkerMetrics {
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(Vec::new()),
        }
    }

    pub fn record_task(&self, worker_id: usize, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(stat) = metrics.iter_mut().find(|s| s.worker_id == worker_id) {
            stat.task_count += 1;
            stat.total_duration += duration;
        } else {
            metrics.push(WorkerStat {
                worker_id,
                task_count: 1,
                error_count: 0,
                total_duration: duration,
            });
        }
    }

    pub fn record_error(&self, worker_id: usize) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(stat) = metrics.iter_mut().find(|s| s.worker_id == worker_id) {
            stat.error_count += 1;
        } else {
            metrics.push(WorkerStat {
                worker_id,
                task_count: 0,
                error_count: 1,
                total_duration: Duration::default(),
            });
        }
    }
}

static MONITOR: once_cell::sync::Lazy<Mutex<Option<RuntimeMonitor>>> = once_cell::sync::Lazy::new(|| Mutex::new(None));

pub async fn start_collection(output_dirs: &OutputDirs) {
    let mut monitor = RuntimeMonitor::new();
    {
        let mut mon = MONITOR.lock().unwrap();
        *mon = Some(monitor.clone());
    }

    let metrics_path = output_dirs.metrics_path().join("runtime_metrics.csv");
    let mut file = File::create(metrics_path).expect("Failed to create metrics file");

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let stats = monitor.collect();
            let _ = writeln!(file, "{:?}", stats);
        }
    });
}

pub async fn shutdown() -> Result<()> {
    let mut mon = MONITOR.lock().unwrap();
    *mon = None;
    Ok(())
} 