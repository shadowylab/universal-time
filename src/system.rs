use core::time::Duration;

/// Wall clock time represented as a duration since the Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime {
    since_unix_epoch: Duration,
}

/// Unix epoch (January 1, 1970).
pub const UNIX_EPOCH: SystemTime = SystemTime::from_unix_duration(Duration::ZERO);

impl SystemTime {
    /// Creates a `SystemTime` from a duration since Unix epoch.
    #[inline]
    pub const fn from_unix_duration(since_unix_epoch: Duration) -> Self {
        Self { since_unix_epoch }
    }

    /// Returns this timestamp as duration since Unix epoch.
    #[inline]
    pub const fn as_unix_duration(self) -> Duration {
        self.since_unix_epoch
    }

    /// Returns the current system time.
    ///
    /// # Platform behavior
    ///
    /// - With `std` feature on supported platforms: uses `std::time::SystemTime`
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
            let now = std::time::SystemTime::now();
            let since_unix_epoch = now
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::ZERO);
            Self::from_unix_duration(since_unix_epoch)
        }

        #[cfg(any(
            not(feature = "std"),
            all(feature = "std", target_family = "wasm", target_os = "unknown")
        ))]
        {
            crate::global::get_time_provider().system_time()
        }
    }

    /// Returns the duration since another `SystemTime`.
    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, Duration> {
        if self.since_unix_epoch >= earlier.since_unix_epoch {
            Ok(self.since_unix_epoch - earlier.since_unix_epoch)
        } else {
            Err(earlier.since_unix_epoch - self.since_unix_epoch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_epoch_is_zero() {
        assert_eq!(UNIX_EPOCH.as_unix_duration(), Duration::ZERO);
    }

    #[test]
    fn roundtrip_unix_duration() {
        let duration = Duration::from_secs(123) + Duration::from_nanos(456);
        let time = SystemTime::from_unix_duration(duration);
        assert_eq!(time.as_unix_duration(), duration);
    }

    #[test]
    fn duration_since_forward() {
        let earlier = SystemTime::from_unix_duration(Duration::from_secs(10));
        let later = SystemTime::from_unix_duration(Duration::from_secs(12));
        assert_eq!(later.duration_since(earlier), Ok(Duration::from_secs(2)));
    }

    #[test]
    fn duration_since_backward() {
        let earlier = SystemTime::from_unix_duration(Duration::from_secs(10));
        let later = SystemTime::from_unix_duration(Duration::from_secs(12));
        assert_eq!(earlier.duration_since(later), Err(Duration::from_secs(2)));
    }

    #[test]
    #[cfg(all(
        feature = "std",
        not(all(target_family = "wasm", target_os = "unknown"))
    ))]
    fn now_is_after_unix_epoch() {
        let now = SystemTime::now();
        assert!(now.duration_since(UNIX_EPOCH).is_ok());
    }
}
