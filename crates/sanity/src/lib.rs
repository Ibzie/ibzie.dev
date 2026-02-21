//! Sanity - PR sanity scoring engine

pub mod analyzers;
pub mod config;
pub mod error;
pub mod metrics;
pub mod score;

pub use config::{load_config, SanityConfig};
pub use metrics::{Metric, MetricInput, MetricResult};
pub use score::{analyze, SanityReport};
