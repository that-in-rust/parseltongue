// Level 4: Metrics Reporting
// - Implements metrics export
// - Handles Prometheus format
// - Manages reporting intervals
// - Provides aggregation

use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use std::net::SocketAddr;
use crate::core::error::Result;

// Level 3: Reporter Configuration
pub struct MetricsReporter {
    builder: PrometheusBuilder,
    address: SocketAddr,
}

impl MetricsReporter {
    // Level 2: Reporter Setup
    pub fn new(addr: SocketAddr) -> Self {
        let builder = PrometheusBuilder::new()
            .with_http_listener(addr)
            .add_global_label("service", "parseltongue");

        Self {
            builder,
            address: addr,
        }
    }

    // Level 1: Reporting Operations
    pub async fn start(self) -> Result<()> {
        self.builder
            .install()
            .map_err(|e| crate::core::error::Error::Processing {
                msg: format!("Failed to start metrics reporter: {}", e)
            })?;
        Ok(())
    }
} 