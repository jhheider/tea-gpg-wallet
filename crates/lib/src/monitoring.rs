//! Performance monitoring for the TEA GPG Wallet CLI Tool
//!
//! This module provides performance monitoring capabilities including operation timing,
//! memory usage tracking, gas cost calculation, network latency measurement, and retry counting.

use std::{
    collections::HashMap,
    time::{
        Duration,
        Instant,
    },
};

/// Performance metrics for a single operation
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    /// Operation start time
    pub start_time: Instant,
    /// Operation duration
    pub duration: Option<Duration>,
    /// Peak memory usage in bytes
    pub peak_memory: Option<usize>,
    /// Gas cost in wei
    pub gas_cost: Option<u64>,
    /// Network latency in milliseconds
    pub network_latency: Option<Duration>,
    /// Number of retries attempted
    pub retry_count: u32,
    /// Operation name/identifier
    pub operation_name: String,
}

impl OperationMetrics {
    /// Create new metrics for an operation
    pub fn new(operation_name: String) -> Self {
        Self {
            start_time: Instant::now(),
            duration: None,
            peak_memory: None,
            gas_cost: None,
            network_latency: None,
            retry_count: 0,
            operation_name,
        }
    }

    /// Mark operation as complete and record duration
    pub fn complete(&mut self) {
        self.duration = Some(self.start_time.elapsed());
    }

    /// Record gas cost
    pub fn record_gas_cost(&mut self, gas_cost: u64) {
        self.gas_cost = Some(gas_cost);
    }

    /// Record network latency
    pub fn record_network_latency(&mut self, latency: Duration) {
        self.network_latency = Some(latency);
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}

/// Performance monitor for tracking multiple operations
#[derive(Debug)]
pub struct PerformanceMonitor {
    /// Current active operations
    active_operations: HashMap<String, OperationMetrics>,
    /// Completed operations history
    completed_operations: Vec<OperationMetrics>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            active_operations: HashMap::new(),
            completed_operations: Vec::new(),
        }
    }

    /// Start tracking an operation
    pub fn start_operation(&mut self, operation_name: String) {
        let metrics = OperationMetrics::new(operation_name.clone());
        self.active_operations.insert(operation_name, metrics);
    }

    /// Complete an operation and move to history
    pub fn complete_operation(&mut self, operation_name: &str) -> Option<OperationMetrics> {
        if let Some(mut metrics) = self.active_operations.remove(operation_name) {
            metrics.complete();
            self.completed_operations.push(metrics.clone());
            Some(metrics)
        } else {
            None
        }
    }

    /// Get metrics for an active operation
    pub fn get_active_metrics(&mut self, operation_name: &str) -> Option<&mut OperationMetrics> {
        self.active_operations.get_mut(operation_name)
    }

    /// Get all completed operations
    pub fn get_completed_operations(&self) -> &[OperationMetrics] {
        &self.completed_operations
    }

    /// Get total operations count
    pub fn total_operations(&self) -> usize {
        self.completed_operations.len()
    }

    /// Get average operation duration
    pub fn average_duration(&self) -> Option<Duration> {
        if self.completed_operations.is_empty() {
            return None;
        }

        let total_duration: Duration = self
            .completed_operations
            .iter()
            .filter_map(|op| op.duration)
            .sum();

        let count = self
            .completed_operations
            .iter()
            .filter(|op| op.duration.is_some())
            .count();

        if count > 0 {
            Some(Duration::from_nanos(
                total_duration.as_nanos() as u64 / count as u64,
            ))
        } else {
            None
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
