use core::ops::{Add, Sub};
use core::time::Duration;

/// Monotonic clock reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    ticks: Duration,
}

impl Instant {
    /// Creates an `Instant` from monotonic ticks.
    #[inline]
    pub const fn from_ticks(ticks: Duration) -> Self {
        Self { ticks }
    }

    /// Returns monotonic ticks for this instant.
    #[inline]
    pub const fn to_ticks(self) -> Duration {
        self.ticks
    }

    /// Returns an instant corresponding to "now".
    ///
    /// # Platform behavior
    ///
    /// - With `std` feature on supported platforms: uses `std::time::Instant`
    /// - Without `std` or on WASM unknown: uses the provider defined via `define_time_provider!` macro
    ///
    /// If no provider is defined on platforms without std, you'll get a **link error** at compile time.
    #[inline]
    pub fn now() -> Self {
        #[cfg(all(
            feature = "std",
            not(all(target_family = "wasm", target_os = "unknown"))
        ))]
        {
            Self::from_ticks(std_now_ticks())
        }

        #[cfg(any(
            not(feature = "std"),
            all(feature = "std", target_family = "wasm", target_os = "unknown")
        ))]
        {
            crate::global::get_time_provider().instant()
        }
    }

    /// Returns the amount of time elapsed since this instant.
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    /// Returns the duration since another instant, saturating at zero.
    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.ticks.saturating_sub(earlier.ticks)
    }

    /// Returns `Some(duration)` if `self` is not earlier than `other`.
    #[inline]
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        self.ticks.checked_sub(earlier.ticks)
    }

    /// Returns `Some(instant)` if adding the duration does not overflow.
    #[inline]
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        self.ticks.checked_add(duration).map(Self::from_ticks)
    }

    /// Returns `Some(instant)` if subtracting the duration does not underflow.
    #[inline]
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.ticks.checked_sub(duration).map(Self::from_ticks)
    }
}

#[cfg(all(
    feature = "std",
    not(all(target_family = "wasm", target_os = "unknown"))
))]
fn std_now_ticks() -> Duration {
    use std::sync::OnceLock;

    static START: OnceLock<std::time::Instant> = OnceLock::new();

    START.get_or_init(std::time::Instant::now).elapsed()
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        self.checked_add(other)
            .expect("overflow while adding Duration to Instant")
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        self.checked_sub(other)
            .expect("underflow while subtracting Duration from Instant")
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_ticks() {
        let ticks = Duration::from_millis(42);
        let instant = Instant::from_ticks(ticks);
        assert_eq!(instant.to_ticks(), ticks);
    }

    #[test]
    fn duration_since_saturates_at_zero() {
        let earlier = Instant::from_ticks(Duration::from_secs(10));
        let later = Instant::from_ticks(Duration::from_secs(3));
        assert_eq!(later.duration_since(earlier), Duration::ZERO);
    }

    #[test]
    fn checked_duration_since_some() {
        let earlier = Instant::from_ticks(Duration::from_secs(3));
        let later = Instant::from_ticks(Duration::from_secs(10));
        assert_eq!(
            later.checked_duration_since(earlier),
            Some(Duration::from_secs(7))
        );
    }

    #[test]
    fn checked_duration_since_none() {
        let earlier = Instant::from_ticks(Duration::from_secs(10));
        let later = Instant::from_ticks(Duration::from_secs(3));
        assert_eq!(later.checked_duration_since(earlier), None);
    }

    #[test]
    fn checked_add_and_sub_roundtrip() {
        let start = Instant::from_ticks(Duration::from_secs(5));
        let delta = Duration::from_secs(2);
        let end = start.checked_add(delta).expect("must not overflow");
        assert_eq!(end.to_ticks(), Duration::from_secs(7));
        assert_eq!(end.checked_sub(delta), Some(start));
    }

    #[test]
    fn checked_add_overflow_returns_none() {
        let start = Instant::from_ticks(Duration::MAX);
        assert_eq!(start.checked_add(Duration::from_nanos(1)), None);
    }

    #[test]
    fn checked_sub_underflow_returns_none() {
        let start = Instant::from_ticks(Duration::ZERO);
        assert_eq!(start.checked_sub(Duration::from_nanos(1)), None);
    }

    #[test]
    fn add_operator_works() {
        let start = Instant::from_ticks(Duration::from_secs(5));
        let end = start + Duration::from_secs(2);
        assert_eq!(end.to_ticks(), Duration::from_secs(7));
    }

    #[test]
    #[should_panic(expected = "overflow while adding Duration to Instant")]
    fn add_operator_panics_on_overflow() {
        let _ = Instant::from_ticks(Duration::MAX) + Duration::from_nanos(1);
    }

    #[test]
    fn sub_operator_works() {
        let end = Instant::from_ticks(Duration::from_secs(7));
        let start = end - Duration::from_secs(2);
        assert_eq!(start.to_ticks(), Duration::from_secs(5));
    }

    #[test]
    #[should_panic(expected = "underflow while subtracting Duration from Instant")]
    fn sub_operator_panics_on_underflow() {
        let _ = Instant::from_ticks(Duration::ZERO) - Duration::from_nanos(1);
    }

    #[test]
    fn sub_instant_operator_is_saturating() {
        let a = Instant::from_ticks(Duration::from_secs(9));
        let b = Instant::from_ticks(Duration::from_secs(4));
        assert_eq!(a - b, Duration::from_secs(5));
        assert_eq!(b - a, Duration::ZERO);
    }

    #[test]
    #[cfg(all(
        feature = "std",
        not(all(target_family = "wasm", target_os = "unknown"))
    ))]
    fn now_is_monotonic() {
        let first = Instant::now();
        let second = Instant::now();
        assert!(second >= first);
    }

    #[test]
    #[cfg(all(
        feature = "std",
        not(all(target_family = "wasm", target_os = "unknown"))
    ))]
    fn elapsed_is_non_negative() {
        let start = Instant::now();
        assert!(start.elapsed() >= Duration::ZERO);
    }
}
