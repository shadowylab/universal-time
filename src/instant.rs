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
    #[inline]
    pub fn now() -> Self {
        if let Some(context) = crate::global::global_time_context() {
            if let Some(now) = context.instant() {
                return now;
            }
        }

        #[cfg(all(
            feature = "std",
            not(all(target_family = "wasm", target_os = "unknown"))
        ))]
        {
            Self::from_ticks(std_now_ticks())
        }

        #[cfg(any(
            not(feature = "std"),
            all(feature = "std", all(target_family = "wasm", target_os = "unknown"))
        ))]
        {
            crate::global::panic_missing_instant()
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
