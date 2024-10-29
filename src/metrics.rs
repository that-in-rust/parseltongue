// Level 4: Runtime Metrics Collection
// - Integrates tokio-metrics for runtime statistics
// - Records metrics to files
// - Manages worker metrics

use tokio_metrics::TaskMonitor;
use tokio::time::{self, Duration};
use crate::output::OutputDirs;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use parking_lot::Mutex as PLMutex;

#[derive(Debug)]
struct WorkerStat {
    worker_id: usize,
    task_count: u64,
    error_count: u64,
    total_duration: Duration,
}

pub struct WorkerMetrics {
    metrics: PLMutex<Vec<WorkerStat>>,
}

impl WorkerMetrics {
    pub fn new() -> Self {
        Self {
            metrics: PLMutex::new(Vec::new()),
        }
    }

    pub fn record_task(&self, worker_id: usize, duration: Duration) {
        let mut metrics = self.metrics.lock();
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
        let mut metrics = self.metrics.lock();
        if let Some(stat) = metrics.iter_mut().find(|s| s.worker_id == worker_id) {
            stat.error_count += 1;
        }
    }
}

static MONITOR: Lazy<Mutex<Option<TaskMonitor>>> = Lazy::new(|| Mutex::new(None));

pub async fn start_collection(output_dirs: &OutputDirs) {
    let monitor = TaskMonitor::new();
    {
        let mut mon = MONITOR.lock().unwrap();
        *mon = Some(monitor);
    }

    let metrics_path = output_dirs.metrics_path().join("runtime_metrics.csv");
    let mut file = File::create(metrics_path).expect("Failed to create metrics file");

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            if let Some(mon) = MONITOR.lock().unwrap().as_ref() {
                let stats = mon.cumulative();
                let _ = writeln!(file, "{:?}", stats);
            } else {
                break;
            }
        }
    });
}

pub async fn shutdown() {
    let mut mon = MONITOR.lock().unwrap();
    *mon = None;
} 