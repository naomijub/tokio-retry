use std::iter::Iterator;

use tokio::time::Duration;

/// A retry strategy driven by the fibonacci series.
///
/// Each retry uses a delay which is the sum of the two previous delays.
///
/// Depending on the problem at hand, a fibonacci retry strategy might
/// perform better and lead to better throughput than the `ExponentialBackoff`
/// strategy.
///
/// See [A Performance Comparison of Different Backoff Algorithms under Different Rebroadcast Probabilities for MANETs.](https://www.researchgate.net/profile/Saher-Manaseer/publication/255672213_A_Performance_Comparison_of_Different_Backoff_Algorithms_under_Different_Rebroadcast_Probabilities_for_MANET's/links/542d40220cf29bbc126d2378/A-Performance-Comparison-of-Different-Backoff-Algorithms-under-Different-Rebroadcast-Probabilities-for-MANETs.pdf)
/// for more details.
#[derive(Debug, Clone)]
pub struct FibonacciBackoff {
    current: u64,
    next: u64,
    factor: u64,
    max_delay: Option<Duration>,
}

impl FibonacciBackoff {
    /// Constructs a new fibonacci back-off strategy,
    /// given a base duration in milliseconds.
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self {
            current: millis,
            next: millis,
            factor: 1u64,
            max_delay: None,
        }
    }

    /// A multiplicative factor that will be applied to the retry delay.
    ///
    /// For example, using a factor of `1000` will make each delay in units of seconds.
    ///
    /// Default factor is `1`.
    #[must_use]
    pub const fn factor(mut self, factor: u64) -> Self {
        self.factor = factor;
        self
    }

    /// Apply a maximum delay. No single retry delay will be longer than this `Duration`.
    #[must_use]
    pub const fn max_delay(mut self, duration: Duration) -> Self {
        self.max_delay = Some(duration);
        self
    }

    /// Apply a maximum delay. No single retry delay will be longer than this `Duration::from_millis`.
    #[must_use]
    pub const fn max_delay_millis(mut self, duration: u64) -> Self {
        self.max_delay = Some(Duration::from_millis(duration));
        self
    }
}

impl Iterator for FibonacciBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Duration> {
        // set delay duration by applying factor
        let duration = self
            .current
            .checked_mul(self.factor)
            .map_or_else(|| Duration::from_millis(u64::MAX), Duration::from_millis);

        // check if we reached max delay
        if let Some(ref max_delay) = self.max_delay
            && duration > *max_delay
        {
            #[cfg(feature = "tracing")]
            tracing::warn!("`max_delay` for strategy reached");
            return Some(*max_delay);
        }

        if let Some(next_next) = self.current.checked_add(self.next) {
            self.current = self.next;
            self.next = next_next;
        } else {
            self.current = self.next;
            self.next = u64::MAX;
        }

        Some(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_the_fibonacci_series_starting_at_10() {
        let mut iter = FibonacciBackoff::from_millis(10);
        assert_eq!(iter.next(), Some(Duration::from_millis(10)));
        assert_eq!(iter.next(), Some(Duration::from_millis(10)));
        assert_eq!(iter.next(), Some(Duration::from_millis(20)));
        assert_eq!(iter.next(), Some(Duration::from_millis(30)));
        assert_eq!(iter.next(), Some(Duration::from_millis(50)));
        assert_eq!(iter.next(), Some(Duration::from_millis(80)));
    }

    #[test]
    fn saturates_at_maximum_value() {
        let mut iter = FibonacciBackoff::from_millis(u64::MAX);
        assert_eq!(iter.next(), Some(Duration::from_millis(u64::MAX)));
        assert_eq!(iter.next(), Some(Duration::from_millis(u64::MAX)));
    }

    #[test]
    fn stops_increasing_at_max_delay() {
        let mut iter = FibonacciBackoff::from_millis(10).max_delay(Duration::from_millis(50));
        assert_eq!(iter.next(), Some(Duration::from_millis(10)));
        assert_eq!(iter.next(), Some(Duration::from_millis(10)));
        assert_eq!(iter.next(), Some(Duration::from_millis(20)));
        assert_eq!(iter.next(), Some(Duration::from_millis(30)));
        assert_eq!(iter.next(), Some(Duration::from_millis(50)));
        assert_eq!(iter.next(), Some(Duration::from_millis(50)));
    }

    #[test]
    fn returns_max_when_max_less_than_base() {
        let mut iter = FibonacciBackoff::from_millis(20).max_delay(Duration::from_millis(10));

        assert_eq!(iter.next(), Some(Duration::from_millis(10)));
        assert_eq!(iter.next(), Some(Duration::from_millis(10)));
    }

    #[test]
    fn can_use_factor_to_get_seconds() {
        let factor = 1000;
        let mut s = FibonacciBackoff::from_millis(1).factor(factor);

        assert_eq!(s.next(), Some(Duration::from_secs(1)));
        assert_eq!(s.next(), Some(Duration::from_secs(1)));
        assert_eq!(s.next(), Some(Duration::from_secs(2)));
    }
}
