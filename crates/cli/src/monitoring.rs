//! CLI-specific performance monitoring integration
//!
//! This module provides CLI-specific performance monitoring functionality,
//! including integration with the library monitoring and output formatting.

use libtea_gpg_wallet::monitoring::{
    OperationMetrics,
    PerformanceMonitor,
};

/// CLI performance monitor with verbosity support
#[derive(Debug)]
pub struct CliPerformanceMonitor {
    /// Base performance monitor
    monitor: PerformanceMonitor,
    /// Verbosity level (0=none, 1=basic, 2=detailed)
    verbosity: u8,
}

impl CliPerformanceMonitor {
    /// Create new CLI performance monitor
    pub fn new(verbosity: u8) -> Self {
        Self {
            monitor: PerformanceMonitor::new(),
            verbosity,
        }
    }

    /// Start tracking an operation
    pub fn start_operation(&mut self, operation_name: String) {
        if self.verbosity > 0 {
            self.monitor.start_operation(operation_name);
        }
    }

    /// Complete an operation
    pub fn complete_operation(&mut self, operation_name: &str) -> Option<OperationMetrics> {
        if self.verbosity > 0 {
            self.monitor.complete_operation(operation_name)
        } else {
            None
        }
    }

    /// Get metrics for an active operation
    pub fn get_active_metrics(&mut self, operation_name: &str) -> Option<&mut OperationMetrics> {
        if self.verbosity > 0 {
            self.monitor.get_active_metrics(operation_name)
        } else {
            None
        }
    }

    /// Print performance summary if verbosity is enabled
    pub fn print_summary(&self) {
        if self.verbosity == 0 {
            return;
        }

        let completed = self.monitor.get_completed_operations();
        if completed.is_empty() {
            return;
        }

        println!("\nPerformance Summary:");
        println!("==================");

        for operation in completed {
            if let Some(duration) = operation.duration {
                let duration_ms = duration.as_millis();
                println!("{}: {}ms", operation.operation_name, duration_ms);

                if self.verbosity > 1 {
                    if let Some(gas_cost) = operation.gas_cost {
                        println!("  Gas cost: {} wei", gas_cost);
                    }
                    if let Some(latency) = operation.network_latency {
                        println!("  Network latency: {}ms", latency.as_millis());
                    }
                    if operation.retry_count > 0 {
                        println!("  Retries: {}", operation.retry_count);
                    }
                }
            }
        }

        if let Some(avg_duration) = self.monitor.average_duration() {
            println!("Average operation time: {}ms", avg_duration.as_millis());
        }
    }
}
