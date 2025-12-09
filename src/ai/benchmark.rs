/// Performance benchmarking utilities for AI evaluation
/// 
/// This module provides tools to measure and track performance
/// improvements from optimization efforts.

use std::time::{Instant, Duration};

/// Performance metrics for evaluation operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformanceMetrics {
    /// Total time spent in operation
    pub total_duration: Duration,
    /// Number of operations performed
    pub operations: usize,
    /// Average time per operation
    pub avg_per_op: Duration,
    /// Minimum time for any single operation
    pub min_time: Duration,
    /// Maximum time for any single operation
    pub max_time: Duration,
}

impl PerformanceMetrics {
    /// Create empty metrics
    pub fn new() -> Self {
        PerformanceMetrics {
            total_duration: Duration::ZERO,
            operations: 0,
            avg_per_op: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        }
    }

    /// Add a measurement to the metrics
    pub fn record(&mut self, duration: Duration) {
        self.total_duration += duration;
        self.operations += 1;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);
        self.avg_per_op = self.total_duration / self.operations as u32;
    }

    /// Get average time in microseconds
    pub fn avg_micros(&self) -> u128 {
        self.avg_per_op.as_micros()
    }

    /// Get average time in milliseconds
    pub fn avg_millis(&self) -> f64 {
        self.avg_per_op.as_secs_f64() * 1000.0
    }

    /// Get throughput (operations per second)
    pub fn throughput(&self) -> f64 {
        if self.total_duration.is_zero() {
            0.0
        } else {
            self.operations as f64 / self.total_duration.as_secs_f64()
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer for measuring operation duration
pub struct Timer {
    start: Instant,
}

impl Timer {
    /// Create and start a new timer
    pub fn start() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    /// Elapsed time since timer creation
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Elapsed time in microseconds
    pub fn elapsed_micros(&self) -> u128 {
        self.elapsed().as_micros()
    }

    /// Elapsed time in milliseconds
    pub fn elapsed_millis(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }
}

/// Benchmark result comparing two implementations
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub baseline_metrics: PerformanceMetrics,
    pub optimized_metrics: PerformanceMetrics,
}

impl BenchmarkResult {
    /// Calculate speedup factor
    pub fn speedup(&self) -> f64 {
        if self.optimized_metrics.avg_per_op.is_zero() {
            0.0
        } else {
            self.baseline_metrics.avg_per_op.as_secs_f64()
                / self.optimized_metrics.avg_per_op.as_secs_f64()
        }
    }

    /// Calculate time saved per operation
    pub fn time_saved_per_op(&self) -> Duration {
        self.baseline_metrics
            .avg_per_op
            .saturating_sub(self.optimized_metrics.avg_per_op)
    }

    /// Calculate percentage improvement
    pub fn improvement_percent(&self) -> f64 {
        let speedup = self.speedup();
        if speedup == 0.0 {
            0.0
        } else {
            ((speedup - 1.0) / speedup) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_metrics_new() {
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.operations, 0);
        assert_eq!(metrics.total_duration, Duration::ZERO);
    }

    #[test]
    fn test_performance_metrics_record() {
        let mut metrics = PerformanceMetrics::new();
        metrics.record(Duration::from_millis(10));

        assert_eq!(metrics.operations, 1);
        assert!(metrics.total_duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_performance_metrics_multiple_records() {
        let mut metrics = PerformanceMetrics::new();
        metrics.record(Duration::from_millis(10));
        metrics.record(Duration::from_millis(20));
        metrics.record(Duration::from_millis(30));

        assert_eq!(metrics.operations, 3);
        assert_eq!(metrics.min_time, Duration::from_millis(10));
        assert_eq!(metrics.max_time, Duration::from_millis(30));
    }

    #[test]
    fn test_performance_metrics_throughput() {
        let mut metrics = PerformanceMetrics::new();
        // Record 10 operations taking 1 second total
        for _ in 0..10 {
            metrics.record(Duration::from_millis(100));
        }

        let throughput = metrics.throughput();
        assert!(throughput > 0.0);
    }

    #[test]
    fn test_timer_elapsed() {
        let timer = Timer::start();
        thread::sleep(Duration::from_millis(10));

        let elapsed = timer.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_timer_elapsed_millis() {
        let timer = Timer::start();
        thread::sleep(Duration::from_millis(5));

        let elapsed_ms = timer.elapsed_millis();
        assert!(elapsed_ms >= 5.0);
    }

    #[test]
    fn test_benchmark_result_speedup() {
        let mut baseline = PerformanceMetrics::new();
        baseline.record(Duration::from_millis(100));

        let mut optimized = PerformanceMetrics::new();
        optimized.record(Duration::from_millis(50));

        let result = BenchmarkResult {
            baseline_metrics: baseline,
            optimized_metrics: optimized,
        };

        let speedup = result.speedup();
        assert!(speedup >= 1.9 && speedup <= 2.1); // Approximately 2x
    }

    #[test]
    fn test_benchmark_result_improvement_percent() {
        let mut baseline = PerformanceMetrics::new();
        baseline.record(Duration::from_millis(100));

        let mut optimized = PerformanceMetrics::new();
        optimized.record(Duration::from_millis(50));

        let result = BenchmarkResult {
            baseline_metrics: baseline,
            optimized_metrics: optimized,
        };

        let improvement = result.improvement_percent();
        assert!(improvement >= 49.0 && improvement <= 51.0); // Approximately 50%
    }

    #[test]
    fn test_benchmark_result_time_saved() {
        let mut baseline = PerformanceMetrics::new();
        baseline.record(Duration::from_millis(100));

        let mut optimized = PerformanceMetrics::new();
        optimized.record(Duration::from_millis(30));

        let result = BenchmarkResult {
            baseline_metrics: baseline,
            optimized_metrics: optimized,
        };

        let saved = result.time_saved_per_op();
        assert!(saved >= Duration::from_millis(69) && saved <= Duration::from_millis(71));
    }
}
