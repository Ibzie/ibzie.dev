//! Score calculation

use crate::config::SanityConfig;
use crate::metrics::{Metric, MetricInput, MetricResult};
use git_core::DiffSummary;
use regex::Regex;

/// Sanity report
pub struct SanityReport {
    pub score: i32,
    pub raw_score: i32,
    pub passed: bool,
    pub metrics: Vec<MetricResult>,
}

/// Large file metric - penalizes files over threshold
pub struct LargeFileMetric;

impl Metric for LargeFileMetric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult {
        let threshold = input.config.thresholds.max_file_lines;
        let weight = input.config.weights.large_file;

        let mut count = 0;
        for file in &input.diff.files_changed {
            // Estimate lines from insertions + deletions
            let lines = file.insertions + file.deletions;
            if lines > threshold {
                count += 1;
            }
        }

        if count > 0 {
            MetricResult {
                name: "large_file",
                delta: weight * count,
                detail: format!("{} file(s) exceed {} lines", count, threshold),
            }
        } else {
            MetricResult {
                name: "large_file",
                delta: 0,
                detail: "No files exceed line limit".to_string(),
            }
        }
    }
}

/// Large commit metric - penalizes commits with many changes
pub struct LargeCommitMetric;

impl Metric for LargeCommitMetric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult {
        let threshold = input.config.thresholds.max_changes_per_commit;
        let weight = input.config.weights.large_commit;

        // Changes = insertions + deletions
        let changes = input.diff.insertions + input.diff.deletions;

        if changes > threshold {
            // Penalize for every chunk of 500 changes
            // 650 changes = 1 penalty, 1100 = 2, 1500 = 3, etc.
            let penalty_count = (changes - threshold + 500) / 500;
            let delta = weight * penalty_count as i32;

            MetricResult {
                name: "large_commit",
                delta,
                detail: format!(
                    "Commit has {} changes ({} penalties of -{})",
                    changes, penalty_count, -weight
                ),
            }
        } else {
            MetricResult {
                name: "large_commit",
                delta: 0,
                detail: "Commit size is acceptable".to_string(),
            }
        }
    }
}

/// README bonus metric
pub struct ReadmeMetric;

impl Metric for ReadmeMetric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult {
        let weight = input.config.weights.readme_updated;

        let has_readme = input.diff.files_changed.iter().any(|f| {
            f.new_path
                .as_ref()
                .map(|p| p.to_uppercase().starts_with("README"))
                .unwrap_or(false)
        });

        if has_readme {
            MetricResult {
                name: "readme_updated",
                delta: weight,
                detail: "README file updated (+5)".to_string(),
            }
        } else {
            MetricResult {
                name: "readme_updated",
                delta: 0,
                detail: "No README updated".to_string(),
            }
        }
    }
}

/// Missing tests metric
pub struct TestCoverageMetric;

impl Metric for TestCoverageMetric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult {
        let min_ratio = input.config.thresholds.min_test_ratio;
        let weight = input.config.weights.missing_tests;

        if min_ratio <= 0.0 {
            return MetricResult {
                name: "test_coverage",
                delta: 0,
                detail: "Test ratio check disabled".to_string(),
            };
        }

        let mut test_lines = 0;
        let mut source_lines = 0;

        for file in &input.diff.files_changed {
            let path = file.new_path.clone().unwrap_or_default();
            let lines = file.insertions;

            if path.contains("test") || path.ends_with("_test.rs") || path.ends_with(".test.js") {
                test_lines += lines;
            } else if path.ends_with(".rs")
                || path.ends_with(".js")
                || path.ends_with(".ts")
                || path.ends_with(".py")
            {
                source_lines += lines;
            }
        }

        if source_lines == 0 {
            return MetricResult {
                name: "test_coverage",
                delta: 0,
                detail: "No source files changed".to_string(),
            };
        }

        let ratio = test_lines as f32 / source_lines as f32;

        if ratio < min_ratio {
            MetricResult {
                name: "test_coverage",
                delta: weight,
                detail: format!(
                    "Test ratio {:.1}% below minimum {:.1}%",
                    ratio * 100.0,
                    min_ratio * 100.0
                ),
            }
        } else {
            MetricResult {
                name: "test_coverage",
                delta: 0,
                detail: format!("Test ratio {:.1}% acceptable", ratio * 100.0),
            }
        }
    }
}

/// Rust unwrap usage metric
pub struct RustUnwrapMetric;

impl Metric for RustUnwrapMetric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult {
        let weight = input.config.weights.unwrap_usage;

        let unwrap_re = Regex::new(r"\bunwrap\(\)").unwrap();
        let mut count = 0;

        for file in &input.diff.files_changed {
            if file
                .new_path
                .as_ref()
                .map(|p| p.ends_with(".rs"))
                .unwrap_or(false)
            {
                if let Some(patch) = &file.patch {
                    for line in patch.lines() {
                        if unwrap_re.is_match(line) {
                            count += 1;
                        }
                    }
                }
            }
        }

        if count > 0 {
            MetricResult {
                name: "rust_unwrap",
                delta: weight * count,
                detail: format!("{} unwrap() call(s) found", count),
            }
        } else {
            MetricResult {
                name: "rust_unwrap",
                delta: 0,
                detail: "No unwrap() calls found".to_string(),
            }
        }
    }
}

/// JavaScript console.log metric
pub struct JsConsoleLogMetric;

impl Metric for JsConsoleLogMetric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult {
        let weight = input.config.weights.console_log;

        let console_re = Regex::new(r"console\.(log|debug|info|warn|error)").unwrap();
        let mut count = 0;

        for file in &input.diff.files_changed {
            if file
                .new_path
                .as_ref()
                .map(|p| p.ends_with(".js") || p.ends_with(".ts"))
                .unwrap_or(false)
            {
                if let Some(patch) = &file.patch {
                    for line in patch.lines() {
                        if console_re.is_match(line) {
                            count += 1;
                        }
                    }
                }
            }
        }

        if count > 0 {
            MetricResult {
                name: "js_console",
                delta: weight * count,
                detail: format!("{} console.* call(s) found", count),
            }
        } else {
            MetricResult {
                name: "js_console",
                delta: 0,
                detail: "No console.* calls found".to_string(),
            }
        }
    }
}

/// Analyze diff and produce sanity report
pub fn analyze(diff: &DiffSummary, config: &SanityConfig) -> SanityReport {
    let input = MetricInput { diff, config };

    let metrics: Vec<Box<dyn Metric>> = vec![
        Box::new(LargeFileMetric),
        Box::new(LargeCommitMetric),
        Box::new(ReadmeMetric),
        Box::new(TestCoverageMetric),
        Box::new(RustUnwrapMetric),
        Box::new(JsConsoleLogMetric),
    ];

    let mut raw_score = 100;
    let mut results = Vec::new();

    for metric in metrics {
        let result = metric.evaluate(&input);
        raw_score += result.delta;
        results.push(result);
    }

    let score = raw_score.clamp(0, 100);
    let passed = score >= config.block_merge_below;

    SanityReport {
        score,
        raw_score,
        passed,
        metrics: results,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_config;
    use git_core::{DiffSummary, FileDiff, FileStatus};

    #[test]
    fn test_large_commit_penalty_tiers() {
        // 650 changes = 1 penalty
        let diff = DiffSummary {
            files_changed: vec![],
            insertions: 650,
            deletions: 0,
        };

        let config = SanityConfig {
            thresholds: crate::config::Thresholds {
                max_changes_per_commit: 500,
                ..Default::default()
            },
            ..Default::default()
        };

        let report = analyze(&diff, &config);
        // weight is -3, so delta = -3 * 1 = -3
        assert_eq!(report.raw_score, 97);
    }

    #[test]
    fn test_large_commit_two_penalties() {
        // 1100 changes = 2 penalties
        let diff = DiffSummary {
            files_changed: vec![],
            insertions: 1100,
            deletions: 0,
        };

        let config = SanityConfig {
            thresholds: crate::config::Thresholds {
                max_changes_per_commit: 500,
                ..Default::default()
            },
            ..Default::default()
        };

        let report = analyze(&diff, &config);
        // weight is -3, so delta = -3 * 2 = -6
        assert_eq!(report.raw_score, 94);
    }

    #[test]
    fn test_large_commit_three_penalties() {
        // 1500 changes = 3 penalties
        let diff = DiffSummary {
            files_changed: vec![],
            insertions: 1500,
            deletions: 0,
        };

        let config = SanityConfig {
            thresholds: crate::config::Thresholds {
                max_changes_per_commit: 500,
                ..Default::default()
            },
            ..Default::default()
        };

        let report = analyze(&diff, &config);
        // weight is -3, so delta = -3 * 3 = -9
        assert_eq!(report.raw_score, 91);
    }

    #[test]
    fn test_empty_diff_scores_100() {
        let diff = DiffSummary {
            files_changed: vec![],
            insertions: 0,
            deletions: 0,
        };
        let report = analyze(&diff, &SanityConfig::default());
        assert_eq!(report.score, 100);
        assert!(report.passed);
    }

    #[test]
    fn test_score_clamped_at_zero() {
        let mut diff = DiffSummary {
            files_changed: vec![],
            insertions: 1000,
            deletions: 1000,
        };

        for i in 0..20 {
            diff.files_changed.push(FileDiff {
                old_path: Some(format!("file{}.rs", i)),
                new_path: Some(format!("file{}.rs", i)),
                status: FileStatus::Modified,
                insertions: 600,
                deletions: 0,
                patch: None,
            });
        }

        let report = analyze(&diff, &SanityConfig::default());
        assert_eq!(report.score, 0);
        assert!(report.raw_score < 0);
    }

    #[test]
    fn test_readme_bonus() {
        let diff = DiffSummary {
            files_changed: vec![FileDiff {
                old_path: None,
                new_path: Some("README.md".to_string()),
                status: FileStatus::Added,
                insertions: 10,
                deletions: 0,
                patch: None,
            }],
            insertions: 10,
            deletions: 0,
        };

        let report = analyze(&diff, &SanityConfig::default());
        assert!(report.raw_score > 100);
    }

    #[test]
    fn test_passed_below_threshold() {
        let diff = DiffSummary {
            files_changed: vec![],
            insertions: 0,
            deletions: 0,
        };
        let mut config = SanityConfig::default();
        config.block_merge_below = 50;

        let report = analyze(&diff, &config);
        assert!(report.passed);
    }

    #[test]
    fn test_load_config_defaults() {
        let config = load_config("").unwrap();
        assert!(config.enabled);
        assert_eq!(config.block_merge_below, 50);
    }

    #[test]
    fn test_load_config_partial_override() {
        let toml = r#"
            block_merge_below = 30
            [weights]
            large_file = -20
        "#;
        let config = load_config(toml).unwrap();
        assert_eq!(config.block_merge_below, 30);
        assert_eq!(config.weights.large_file, -20);
        assert_eq!(config.weights.unwrap_usage, -4);
    }
}
