// Level 4: Metrics Export
// - Implements Prometheus metrics export
// - Manages metric collection
// - Provides HTTP endpoint

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::net::SocketAddr;
use tokio::sync::oneshot;
use crate::error::Result;

pub struct MetricsExporter {
    handle: PrometheusHandle,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl MetricsExporter {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        let builder = PrometheusBuilder::new();
        let handle = builder.install_recorder()?;
        
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        
        tokio::spawn(async move {
            let server = hyper::Server::bind(&addr)
                .serve(handle.clone());
            
            tokio::select! {
                _ = server => {},
                _ = shutdown_rx => {},
            }
        });

        Ok(Self {
            handle,
            shutdown_tx: Some(shutdown_tx),
        })
    }

    pub fn record_task_duration(&self, duration: f64) {
        metrics::histogram!("task_duration_seconds", duration);
    }

    pub fn record_error_count(&self) {
        metrics::counter!("error_count").increment(1);
    }
}

impl Drop for MetricsExporter {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
} 