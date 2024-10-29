// Level 4: Metrics Collection
// - Manages runtime metrics
// - Handles metric recording
// - Coordinates metric export

use tokio_metrics::TaskMonitor;
use tokio::time::{self, Duration};
use crate::output::OutputDirs;
use std::fs::File;
use std::io::Write;
use parking_lot::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct WorkerMetrics {
    metrics: Mutex<Vec<WorkerStat>>,
}

#[derive(Debug)]
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

pub fn init(output_dirs: &OutputDirs) {
    let metrics_path = output_dirs.metrics_path().join("task-metrics.json");
    let mut file = File::create(metrics_path).expect("Failed to create metrics file");

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            if let Some(mon) = MONITOR.lock().as_ref() {
                let stats = mon.cumulative();
                let _ = writeln!(file, "{:?}", stats);
            } else {
                break;
            }
        }
    });
}

pub async fn shutdown() {
    let mut mon = MONITOR.lock();
    *mon = None;
} 