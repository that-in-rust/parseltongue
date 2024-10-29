// Level 4: Runtime Metrics Collection
// - Integrates tokio-metrics for runtime statistics
// - Records metrics to files periodically

use tokio_metrics::RuntimeMonitor;
use tokio::time::{self, Duration, Interval};
use crate::output::OutputDirs;
use std::fs::File;
use std::io::Write;
use crate::error::Result;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MONITOR: Mutex<Option<RuntimeMonitor>> = Mutex::new(None);
}

pub async fn start_collection(output_dirs: &OutputDirs) {
    let monitor = RuntimeMonitor::new();

    {
        let mut mon = MONITOR.lock().unwrap();
        *mon = Some(monitor.clone());
    }

    let metrics_path = output_dirs.metrics_path().join("runtime_metrics.log");
    let file = File::create(metrics_path).expect("Failed to create metrics file");
    let mut interval = time::interval(Duration::from_secs(5));

    tokio::spawn(async move {
        let mut file = file;
        loop {
            interval.tick().await;
            let metrics = monitor.current_thread_metrics();
            if let Err(e) = writeln!(file, "{:?}", metrics) {
                tracing::error!("Failed to write metrics: {:?}", e);
            }
        }
    });
}

pub async fn shutdown() -> Result<()> {
    let mut mon = MONITOR.lock().unwrap();
    *mon = None;
    Ok(())
} 