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
    #[inline]
    pub fn now() -> Self {
        if let Some(context) = crate::global::global_time_context() {
            if let Some(now) = context.system_time() {
                return now;
            }
        }

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
            all(feature = "std", all(target_family = "wasm", target_os = "unknown"))
        ))]
        {
            crate::global::panic_missing_system_time()
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
