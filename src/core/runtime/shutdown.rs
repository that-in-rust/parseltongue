// Level 4: Shutdown Coordination
// - Manages graceful shutdown process
// - Coordinates subsystem shutdown
// - Handles timeout management
// - Collects shutdown metrics

use tokio::sync::broadcast;
use std::time::Duration;

// Level 3: Shutdown Manager
pub struct ShutdownManager {
    shutdown_tx: broadcast::Sender<()>,
    timeout: Duration,
}

impl ShutdownManager {
    // Level 2: Lifecycle Management
    pub fn new() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            shutdown_tx,
            timeout: Duration::from_secs(30),
        }
    }

    // Level 1: Control Operations
    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }

    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }
} 