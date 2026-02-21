//! Metric trait and types

use crate::config::SanityConfig;
use git_core::DiffSummary;

/// Input for metric evaluation
pub struct MetricInput<'a> {
    pub diff: &'a DiffSummary,
    pub config: &'a SanityConfig,
}

/// Result of metric evaluation
pub struct MetricResult {
    pub name: &'static str,
    pub delta: i32,
    pub detail: String,
}

/// Trait for implementing metrics
pub trait Metric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult;
}
