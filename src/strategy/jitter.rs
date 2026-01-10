use rand::distr::uniform::SampleRange;
use tokio::time::Duration;

/// defines `jitter` based on specific duration
#[must_use]
pub fn jitter(duration: Duration) -> Duration {
    duration.mul_f64(rand::random::<f64>() + 0.5)
}

/// defines `jitter` based on explicit bounds
pub fn jitter_with_bounds(min: f64, max: f64) -> impl Fn(Duration) -> Duration {
    move |x| x.mul_f64(rand::random::<f64>().mul_add(max - min, min))
}

/// defines `jitter` based on range
pub fn jitter_range<R: SampleRange<u32>>(r: R) -> impl Fn(Duration) -> Duration {
    let range = rand::random_range(r);
    move |x| x * range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitter() {
        let jitter = jitter(Duration::from_millis(100));
        assert!(jitter.as_millis() >= 50);
        assert!(jitter.as_millis() <= 150);
        assert!(jitter.as_millis() != 100);
    }

    #[test]
    fn test_jitter_with_bounds() {
        let jitter = jitter_with_bounds(0.01, 0.1)(Duration::from_millis(100));
        assert!(jitter.as_millis() >= 1);
        assert!(jitter.as_millis() <= 10);
        assert!(jitter.as_millis() != 100);

        let jitter = jitter_with_bounds(0.1, 0.2)(Duration::from_millis(100));
        assert!(jitter.as_millis() >= 10);
        assert!(jitter.as_millis() <= 20);
        assert!(jitter.as_millis() != 100);

        let jitter = jitter_with_bounds(0.5, 0.6)(Duration::from_millis(100));
        assert!(jitter.as_millis() >= 50);
        assert!(jitter.as_millis() <= 60);
        assert!(jitter.as_millis() != 100);
    }

    #[test]
    fn test_jitter_range() {
        let jitter = jitter_range(0..1)(Duration::from_millis(100));
        assert!(jitter.as_millis() <= 100);
        assert!(jitter.as_millis() != 100);
    }
}
