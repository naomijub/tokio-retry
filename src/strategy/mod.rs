mod exponential_backoff;
mod exponential_factor_backoff;
mod fibonacci_backoff;
mod fixed_interval;
#[cfg(feature = "jitter")]
mod jitter;
mod max_interval;

#[cfg(feature = "jitter")]
pub use self::jitter::{jitter, jitter_range};
pub use self::{
    exponential_backoff::ExponentialBackoff,
    exponential_factor_backoff::ExponentialFactorBackoff,
    fibonacci_backoff::FibonacciBackoff,
    fixed_interval::FixedInterval,
    max_interval::{MaxInterval, MaxIntervalIterator},
};
