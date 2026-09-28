//! Output formatting and verbosity management for CLI
//!
//! This module handles output formatting, verbosity levels, and metrics display
//! for the TEA GPG Wallet CLI tool.

use std::time::Duration;

use colored::*;
use libtea_gpg_wallet::monitoring::OperationMetrics;
use serde_json;

/// Verbosity levels for CLI output
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerbosityLevel {
    /// No additional output
    Normal = 0,
    /// Basic performance metrics
    Verbose = 1,
    /// Detailed performance metrics and debugging info
    Debug = 2,
}

impl VerbosityLevel {
    /// Parse verbosity level from command line flags
    pub fn from_flags(verbose: bool, debug: bool) -> Self {
        if debug {
            Self::Debug
        } else if verbose {
            Self::Verbose
        } else {
            Self::Normal
        }
    }

    /// Check if verbose output is enabled
    pub fn is_verbose(&self) -> bool {
        *self as u8 >= 1
    }

    /// Check if debug output is enabled
    pub fn is_debug(&self) -> bool {
        *self as u8 >= 2
    }
}

/// Output formatter for CLI results
pub struct OutputFormatter {
    /// Current verbosity level
    verbosity: VerbosityLevel,
    /// JSON output mode
    json_mode: bool,
}

impl OutputFormatter {
    /// Create new output formatter
    pub fn new(verbosity: VerbosityLevel, json_mode: bool) -> Self {
        Self {
            verbosity,
            json_mode,
        }
    }

    /// Print operation progress with timing
    pub fn print_operation_progress(&self, operation: &str, elapsed: Duration) {
        if !self.verbosity.is_verbose() {
            return;
        }

        let elapsed_ms = elapsed.as_millis();
        println!("[{}ms] {}...", elapsed_ms, operation.blue());
    }

    /// Print operation completion with metrics
    pub fn print_operation_complete(&self, operation: &str, metrics: Option<&OperationMetrics>) {
        if self.json_mode {
            self.print_json_metrics(operation, metrics);
        } else {
            self.print_human_metrics(operation, metrics);
        }
    }

    /// Print human-readable metrics
    fn print_human_metrics(&self, operation: &str, metrics: Option<&OperationMetrics>) {
        if let Some(metrics) = metrics {
            if let Some(duration) = metrics.duration {
                let duration_ms = duration.as_millis();
                println!("✓ {} completed in {}ms", operation.green(), duration_ms);

                if self.verbosity.is_debug() {
                    if let Some(gas_cost) = metrics.gas_cost {
                        println!("  Gas cost: {} wei", gas_cost.to_string().yellow());
                    }
                    if let Some(latency) = metrics.network_latency {
                        println!(
                            "  Network latency: {}ms",
                            latency.as_millis().to_string().yellow()
                        );
                    }
                    if metrics.retry_count > 0 {
                        println!("  Retries: {}", metrics.retry_count.to_string().yellow());
                    }
                }
            }
        } else {
            println!("✓ {} completed", operation.green());
        }
    }

    /// Print JSON metrics
    fn print_json_metrics(&self, operation: &str, metrics: Option<&OperationMetrics>) {
        if let Some(metrics) = metrics {
            let json_data = serde_json::json!({
                "operation": operation,
                "duration_ms": metrics.duration.map(|d| d.as_millis()),
                "gas_cost": metrics.gas_cost,
                "network_latency_ms": metrics.network_latency.map(|l| l.as_millis()),
                "retry_count": metrics.retry_count
            });
            println!("{}", json_data);
        }
    }

    /// Print error message with appropriate formatting
    pub fn print_error(&self, error: &str) {
        if self.json_mode {
            let json_data = serde_json::json!({
                "error": error,
                "success": false
            });
            eprintln!("{}", json_data);
        } else {
            eprintln!("{} {}", "Error:".red().bold(), error);
        }
    }

    /// Print success message
    pub fn print_success(&self, message: &str) {
        if self.json_mode {
            let json_data = serde_json::json!({
                "message": message,
                "success": true
            });
            println!("{}", json_data);
        } else {
            println!("{} {}", "✓".green(), message.green());
        }
    }

    /// Print info message
    pub fn print_info(&self, message: &str) {
        if !self.json_mode {
            println!("{} {}", "ℹ".blue(), message.blue());
        }
    }

    /// Print warning message
    pub fn print_warning(&self, message: &str) {
        if !self.json_mode {
            println!("{} {}", "⚠".yellow(), message.yellow());
        }
    }
}
