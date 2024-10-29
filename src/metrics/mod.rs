// Level 4: Metrics Module Organization
// - Manages metrics collection
// - Coordinates reporting
// - Handles aggregation
// - Provides visualization

pub mod collect;
pub mod report;
pub mod export;
pub mod progress;

pub use collect::MetricsCollector;
pub use report::MetricsReporter;
pub use progress::ProgressTracker; 