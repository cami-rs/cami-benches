use core::time::Duration;
use criterion::{BenchmarkId, Criterion};

/// Speed up. Why? This functionality is simple. It should warm up (flood the caches), and show a
/// benefit, fast.
pub fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(1000))
}
