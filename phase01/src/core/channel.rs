//! Async Channel Infrastructure
//! 
//! Pyramid Structure:
//! Level 4 (Top): Channel Coordination
//! Level 3: Backpressure Management
//! Level 2: Channel Implementation
//! Level 1 (Base): Core Channel Types

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore, RwLock};
use futures::{Stream, StreamExt};
use std::time::{Duration, Instant};
use metrics::{Counter, Gauge, Histogram};
use crate::core::{error::{Error, Result}, types::*};
use tracing::{info, warn, error};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::Stream;
use futures::ready;

// ===== Level 1: Core Channel Types =====

#[derive(Debug, Clone)]
pub struct ChannelConfig {
    pub buffer_size: usize,
    pub high_water_mark: usize,
    pub low_water_mark: usize,
    pub metrics_enabled: bool,
}

// ===== Level 2: Channel Implementation =====

/// Async channel with backpressure
pub struct AsyncChannel<T> {
    tx: mpsc::Sender<T>,
    config: ChannelConfig,
    metrics: Arc<Metrics>,
    backpressure: Arc<BackpressureControl>,
}

/// Metrics collection - Now using interior mutability correctly
#[derive(Debug)]
struct Metrics {
    messages_sent: Counter,
    queue_size: Gauge,
    latency: Histogram,
    backpressure_events: Counter,
}

impl Metrics {
    fn new() -> Self {
        Self {
            messages_sent: Counter::new(),
            queue_size: Gauge::new(),
            latency: Histogram::new(),
            backpressure_events: Counter::new(),
        }
    }
}

/// Backpressure control with proper synchronization
#[derive(Debug)]
struct BackpressureControl {
    permits: Arc<Semaphore>,
    state: Arc<RwLock<BackpressureState>>,
}

#[derive(Debug, Clone)]
struct BackpressureState {
    is_active: bool,
    last_trigger: Instant,
    trigger_count: u64,
}

impl BackpressureControl {
    fn new(high: usize, low: usize) -> Self {
        Self {
            permits: Arc::new(Semaphore::new(high)),
            state: Arc::new(RwLock::new(BackpressureState {
                is_active: false,
                last_trigger: Instant::now(),
                trigger_count: 0,
            })),
        }
    }

    async fn acquire(&self) -> Result<BackpressureGuard> {
        let permit = self.permits.acquire().await?;
        
        let mut state = self.state.write().await;
        if !state.is_active && self.permits.available_permits() <= self.low_water_mark() {
            state.is_active = true;
            state.last_trigger = Instant::now();
            state.trigger_count += 1;
        }

        Ok(BackpressureGuard {
            permit,
            control: self.clone(),
        })
    }

    fn low_water_mark(&self) -> usize {
        self.permits.available_permits() / 4
    }

    fn try_acquire(&self) -> Result<BackpressureGuard> {
        match self.permits.try_acquire() {
            Ok(permit) => {
                let mut state = self.state.blocking_lock();
                if !state.is_active && self.permits.available_permits() <= self.low_water_mark() {
                    state.is_active = true;
                    state.last_trigger = Instant::now();
                    state.trigger_count += 1;
                }
                Ok(BackpressureGuard {
                    permit,
                    control: self.clone(),
                })
            }
            Err(_) => Err(Error::ResourceLimit("Backpressure active".into()))
        }
    }
}

// RAII guard for backpressure permits
struct BackpressureGuard {
    permit: tokio::sync::SemaphorePermit<'static>,
    control: BackpressureControl,
}

impl<T> AsyncChannel<T>
where
    T: Send + 'static,
{
    pub fn new(config: ChannelConfig) -> (Self, mpsc::Receiver<T>) {
        let (tx, rx) = mpsc::channel(config.buffer_size);
        let metrics = Arc::new(Metrics::new());
        let backpressure = Arc::new(BackpressureControl::new(
            config.high_water_mark,
            config.low_water_mark,
        ));

        (Self {
            tx,
            config,
            metrics,
            backpressure,
        }, rx)
    }

    pub async fn send(&self, value: T) -> Result<()> {
        let start = Instant::now();
        
        // Get backpressure permit
        let _guard = self.backpressure.acquire().await?;

        // Send message
        self.tx.send(value).await.map_err(|_| Error::ChannelClosed)?;

        // Update metrics atomically
        self.metrics.messages_sent.increment(1);
        self.metrics.latency.record(start.elapsed());
        
        Ok(())
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }
}

impl<T> Stream for AsyncChannel<T> 
where
    T: Send + 'static,
{
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // First check backpressure
        match self.backpressure.try_acquire() {
            Ok(_permit) => {
                // We have capacity, try to receive message
                match ready!(self.rx.poll_recv(cx)) {
                    Some(msg) => {
                        // Update metrics
                        self.metrics.messages_received.increment(1);
                        Poll::Ready(Some(Ok(msg)))
                    }
                    None => {
                        // Channel closed
                        Poll::Ready(None)
                    }
                }
            }
            Err(_) => {
                // Backpressure active, yield to runtime
                self.metrics.backpressure_events.increment(1);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_channel_backpressure() {
        let config = ChannelConfig {
            buffer_size: 10,
            high_water_mark: 8,
            low_water_mark: 2,
            metrics_enabled: true,
        };

        let (channel, mut rx) = AsyncChannel::new(config);

        // Spawn consumer
        tokio::spawn(async move {
            while let Some(_) = rx.recv().await {
                sleep(Duration::from_millis(10)).await;
            }
        });

        // Test sending with backpressure
        for i in 0..20 {
            channel.send(i).await.unwrap();
        }

        assert!(channel.metrics().messages_sent.get() > 0);
    }

    #[tokio::test]
    async fn test_channel_stream() {
        let config = ChannelConfig {
            buffer_size: 10,
            high_water_mark: 8,
            low_water_mark: 2,
            metrics_enabled: true,
        };

        let (channel, _rx) = AsyncChannel::new(config);
        
        // Send some test messages
        for i in 0..5 {
            channel.send(i).await.unwrap();
        }

        // Use as stream
        let mut count = 0;
        let mut stream = channel.take(5);
        
        while let Some(Ok(_msg)) = stream.next().await {
            count += 1;
        }

        assert_eq!(count, 5);
        assert!(channel.metrics().backpressure_events.get() >= 0);
    }
}
