//! Error Handling Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Error Context & Recovery
//! - ErrorExt trait      (adds context to errors)
//! - CircuitBreaker      (prevents cascading failures)
//! - Metrics integration (tracks error patterns)
//! 
//! Level 3: Error Categories
//! - IO/System Errors    (file, network)
//! - Domain Errors       (ZIP, DB operations)
//! - Resource Errors     (pools, limits)
//! - Runtime Errors      (async, cancellation)
//! 
//! Level 2: Error State Management
//! - CircuitState        (breaker state machine)
//! - Error propagation   (From implementations)
//! - Failure tracking    (metrics collection)
//! 
//! Level 1 (Base): Core Error Types
//! - Error enum          (comprehensive error types)
//! - Result type         (error type alias)
//! - Basic error traits  (Error, Display)

use std::{io, path::PathBuf};
use thiserror::Error;
use tokio::time::error::Elapsed;
use std::sync::Arc;
use metrics::{Counter, Gauge};
use std::time::Duration;
use tokio::sync::Mutex;

/// Core error types for the ZIP processing system
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Database error: {0}")]
    Database(#[from] sled::Error),

    #[error("Encoding error: {0}")]
    Encoding(#[from] std::string::FromUtf8Error),

    #[error("Operation timed out after {0:?}")]
    Timeout(Duration, #[source] Box<Elapsed>),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    #[error("Task cancelled")]
    Cancelled,

    #[error("Shutdown in progress")]
    Shutdown,

    // Additional error types for async operations
    #[error("Buffer pool exhausted")]
    BufferPoolExhausted,

    #[error("Connection pool exhausted")]
    ConnectionPoolExhausted,

    #[error("Invalid ZIP entry: {0}")]
    InvalidZipEntry(String),

    #[error("Metrics error: {0}")]
    MetricsError(String),
}

/// Result type alias for our error type
pub type Result<T> = std::result::Result<T, Error>;

/// Circuit breaker state machine for handling cascading failures
#[derive(Debug, Clone, Copy)]
enum CircuitState {
    /// Normal operation, accepting requests
    Closed,
    /// Not accepting requests until specified time
    Open { until: std::time::Instant },
    /// Testing if service has recovered
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures in async operations.
/// Implements the circuit breaker pattern with metrics tracking.
pub struct CircuitBreaker {
    name: String,
    failures: Counter,
    state: Arc<Mutex<CircuitState>>,
    threshold: u64,
    reset_after: Duration,
    last_failure: Gauge,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker with specified parameters
    pub fn new(name: &str, threshold: u64, reset_after: Duration) -> Self {
        let failures = metrics::counter!("circuit_breaker_failures", "name" => name.to_string());
        let last_failure = metrics::gauge!("circuit_breaker_last_failure", "name" => name.to_string());
        
        Self {
            name: name.to_string(),
            failures,
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            threshold,
            reset_after,
            last_failure,
        }
    }

    /// Executes an async operation with circuit breaker protection
    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut state = self.state.lock().await;

        match *state {
            CircuitState::Open { until } if until > std::time::Instant::now() => {
                tracing::warn!(breaker = self.name, "Circuit breaker open");
                return Err(Error::CircuitBreakerOpen(self.name.clone()));
            }
            CircuitState::Open { .. } => {
                tracing::info!(breaker = self.name, "Circuit breaker entering half-open state");
                *state = CircuitState::HalfOpen;
            }
            _ => {}
        }

        let result = f().await;

        match result {
            Ok(value) => {
                if matches!(*state, CircuitState::HalfOpen) {
                    tracing::info!(breaker = self.name, "Circuit breaker reset");
                    *state = CircuitState::Closed;
                    self.failures.reset();
                }
                Ok(value)
            }
            Err(e) => {
                self.failures.increment(1);
                self.last_failure.set(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as f64);

                if self.failures.get() >= self.threshold {
                    tracing::error!(
                        breaker = self.name,
                        failures = self.failures.get(),
                        "Circuit breaker tripped"
                    );
                    *state = CircuitState::Open {
                        until: std::time::Instant::now() + self.reset_after,
                    };
                }
                Err(e)
            }
        }
    }

    /// Returns current failure count
    pub fn failure_count(&self) -> u64 {
        self.failures.get()
    }

    /// Manually reset the circuit breaker
    pub async fn reset(&self) {
        let mut state = self.state.lock().await;
        *state = CircuitState::Closed;
        self.failures.reset();
    }
}

/// Extension trait for adding context to errors
pub trait ErrorExt<T> {
    fn with_context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}

impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn with_context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let err: Error = e.into();
            tracing::error!(?err, context = %context, "Operation failed");
            err
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_circuit_breaker_basic_operation() {
        let breaker = CircuitBreaker::new("test", 3, Duration::from_millis(100));

        // Test successful calls
        let result = breaker.call(|| async { Ok::<_, Error>(42) }).await;
        assert!(result.is_ok());

        // Test failure threshold
        for _ in 0..3 {
            let _ = breaker.call(|| async { 
                Err(Error::ResourceLimit("test".to_string())) 
            }).await;
        }

        // Circuit should be open now
        let result = breaker.call(|| async { Ok::<_, Error>(42) }).await;
        assert!(matches!(result, Err(Error::CircuitBreakerOpen(_))));

        // Wait for reset
        sleep(Duration::from_millis(150)).await;

        // Circuit should be half-open and accept calls
        let result = breaker.call(|| async { Ok::<_, Error>(42) }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_error_context() {
        let result: Result<()> = Err(Error::ResourceLimit("memory".to_string()))
            .with_context("Failed to allocate buffer");
        
        assert!(matches!(result, Err(Error::ResourceLimit(_))));
    }

    #[tokio::test]
    async fn test_circuit_breaker_metrics() {
        let breaker = CircuitBreaker::new("test_metrics", 2, Duration::from_millis(50));
        
        // Cause some failures
        for _ in 0..2 {
            let _ = breaker.call(|| async { 
                Err(Error::ResourceLimit("test".to_string())) 
            }).await;
        }

        assert_eq!(breaker.failure_count(), 2);
        
        // Reset and verify metrics are cleared
        breaker.reset().await;
        assert_eq!(breaker.failure_count(), 0);
    }
}
