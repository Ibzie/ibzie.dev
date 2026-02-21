//! Sanity configuration

use serde::Deserialize;

/// Configuration for sanity scoring
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SanityConfig {
    /// Whether sanity checking is enabled
    pub enabled: bool,
    /// Block merges below this score
    pub block_merge_below: i32,
    pub thresholds: Thresholds,
    pub weights: Weights,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Thresholds {
    /// Max lines per file
    pub max_file_lines: usize,
    /// Max changes per commit (insertions + deletions)
    pub max_changes_per_commit: usize,
    /// Minimum test ratio (0.0 - 1.0)
    pub min_test_ratio: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Weights {
    pub large_file: i32,
    pub missing_docstring: i32,
    pub large_commit: i32,
    pub missing_tests: i32,
    pub readme_updated: i32,
    pub unwrap_usage: i32,
    pub console_log: i32,
}

impl Default for SanityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            block_merge_below: 50,
            thresholds: Thresholds::default(),
            weights: Weights::default(),
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            max_file_lines: 500,
            max_changes_per_commit: 500,
            min_test_ratio: 0.0,
        }
    }
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            large_file: -10,
            missing_docstring: -5,
            large_commit: -3,
            missing_tests: -15,
            readme_updated: 5,
            unwrap_usage: -4,
            console_log: -3,
        }
    }
}

/// Load config from TOML string, defaults on error
pub fn load_config(toml_str: &str) -> Result<SanityConfig, toml::de::Error> {
    toml::from_str(toml_str)
}
