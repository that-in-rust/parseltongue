// Level 4: Graceful Shutdown Management
// - Provides mechanisms to signal and coordinate shutdown across tasks
// - Uses broadcast channels to notify tasks

use tokio::sync::broadcast;

pub struct ShutdownManager {
    shutdown_signal: broadcast::Sender<()>,
}

impl ShutdownManager {
    // Level 3: Initialize the shutdown manager
    pub fn new() -> Self {
        let (shutdown_signal, _) = broadcast::channel(1);
        Self { shutdown_signal }
    }

    // Level 2: Subscribe to shutdown notifications
    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.shutdown_signal.subscribe()
    }

    // Level 1: Trigger a shutdown signal
    pub fn shutdown(&self) {
        let _ = self.shutdown_signal.send(());
    }
} 