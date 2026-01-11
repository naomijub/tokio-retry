use std::time::Duration;

/// Linear backoff strategy that increases delay by a constant amount each retry
///
/// > If `increment` is not defined then it will be equal to `initial`.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use tokio_retry2::strategy::LinearBackoff;
///
/// // Start at 100ms, increase by 200ms each retry
/// let mut strategy = LinearBackoff::from_millis(100)
///     .increment_millis(200)
///     .take(5);
///
/// // Produces delays: 100ms, 300ms, 500ms, 700ms, 900ms
/// assert_eq!(strategy.next(), Some(Duration::from_millis(100)));
/// assert_eq!(strategy.next(), Some(Duration::from_millis(300)));
/// assert_eq!(strategy.next(), Some(Duration::from_millis(500)));
/// assert_eq!(strategy.next(), Some(Duration::from_millis(700)));
/// assert_eq!(strategy.next(), Some(Duration::from_millis(900)));
/// assert_eq!(strategy.next(), None);
/// ```
///
/// ## Without defined increment:
/// ```
/// use std::time::Duration;
/// use tokio_retry2::strategy::LinearBackoff;
///
/// // Start at 100ms, increase by 200ms each retry
/// let mut strategy = LinearBackoff::from_millis(100)
///     .take(3);
///
/// // Produces delays: 100ms, 200ms, 300ms,
/// assert_eq!(strategy.next(), Some(Duration::from_millis(100)));
/// assert_eq!(strategy.next(), Some(Duration::from_millis(200)));
/// assert_eq!(strategy.next(), Some(Duration::from_millis(300)));
/// assert_eq!(strategy.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct LinearBackoff {
    initial: Duration,
    increment: Duration,
    current_attempt: u64,
    max_delay: Option<Duration>,
}

impl LinearBackoff {
    /// Create a new linear backoff starting at the given duration
    #[must_use]
    pub const fn new(initial: Duration) -> Self {
        Self {
            initial,
            increment: initial, // Default increment is same as initial
            current_attempt: 0,
            max_delay: None,
        }
    }

    /// Create a linear backoff from milliseconds
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self::new(Duration::from_millis(millis))
    }

    /// Create a linear backoff from seconds
    #[must_use]
    pub const fn from_secs(secs: u64) -> Self {
        Self::new(Duration::from_secs(secs))
    }

    /// Set the increment to add on each retry
    #[must_use]
    pub const fn increment(mut self, increment: Duration) -> Self {
        self.increment = increment;
        self
    }

    /// Set the increment in milliseconds
    #[must_use]
    pub const fn increment_millis(mut self, millis: u64) -> Self {
        self.increment = Duration::from_millis(millis);
        self
    }

    /// Set the increment in seconds
    #[must_use]
    pub const fn increment_secs(mut self, secs: u64) -> Self {
        self.increment = Duration::from_secs(secs);
        self
    }

    /// Set a maximum delay cap
    #[must_use]
    pub const fn max_delay(mut self, max: Duration) -> Self {
        self.max_delay = Some(max);
        self
    }

    /// Set a maximum delay in milliseconds
    #[must_use]
    pub const fn max_delay_millis(mut self, millis: u64) -> Self {
        self.max_delay = Some(Duration::from_millis(millis));
        self
    }
}

impl Iterator for LinearBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::cast_possible_truncation, reason = "Verified overflow")]
        let current_attempt = if self.current_attempt > u64::from(u32::MAX) {
            u32::MAX
        } else {
            self.current_attempt as u32
        };

        let delay = self
            .initial
            .saturating_add(self.increment.saturating_mul(current_attempt));

        let delay = self.max_delay.map_or(delay, |max| delay.min(max));

        self.current_attempt += 1;
        Some(delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_linear() {
        let mut s = LinearBackoff::new(Duration::from_millis(123));

        assert_eq!(s.next(), Some(Duration::from_millis(123)));
        assert_eq!(s.next(), Some(Duration::from_millis(246)));
        assert_eq!(s.next(), Some(Duration::from_millis(369)));
    }

    #[test]
    fn returns_linear_max_delay() {
        let mut s = LinearBackoff::from_millis(123).max_delay_millis(300);

        assert_eq!(s.next(), Some(Duration::from_millis(123)));
        assert_eq!(s.next(), Some(Duration::from_millis(246)));
        assert_eq!(s.next(), Some(Duration::from_millis(300)));
    }

    #[test]
    fn returns_linear_with_increment() {
        let mut s = LinearBackoff::new(Duration::from_millis(123)).increment_millis(20);

        assert_eq!(s.next(), Some(Duration::from_millis(123)));
        assert_eq!(s.next(), Some(Duration::from_millis(143)));
        assert_eq!(s.next(), Some(Duration::from_millis(163)));
    }

    #[test]
    fn returns_linear_with_increment_secs() {
        let mut s = LinearBackoff::new(Duration::from_secs(1)).increment_secs(2);

        assert_eq!(s.next(), Some(Duration::from_secs(1)));
        assert_eq!(s.next(), Some(Duration::from_secs(3)));
        assert_eq!(s.next(), Some(Duration::from_secs(5)));
    }

    #[test]
    fn returns_linear_max_delay_duration() {
        let mut s = LinearBackoff::new(Duration::from_millis(100))
            .increment(Duration::from_millis(70))
            .max_delay(Duration::from_millis(200));

        assert_eq!(s.next(), Some(Duration::from_millis(100)));
        assert_eq!(s.next(), Some(Duration::from_millis(170)));
        assert_eq!(s.next(), Some(Duration::from_millis(200)));
    }
}
