// Level 4: Graceful Shutdown Management
// - Provides mechanisms to signal and coordinate shutdown across tasks
// - Uses broadcast channels to notify tasks

use tokio::sync::broadcast;

pub struct ShutdownManager {
    shutdown_signal: broadcast::Sender<()>,
}

impl ShutdownManager {
    pub fn new() -> Self {
        let (shutdown_signal, _) = broadcast::channel(1);
        Self { shutdown_signal }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.shutdown_signal.subscribe()
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_signal.send(());
    }
} 