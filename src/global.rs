use crate::{Instant, SystemTime};

/// Source of wall-clock timestamps.
pub trait WallClock {
    /// Returns the current wall-clock time.
    fn system_time(&self) -> SystemTime;
}

/// Source of monotonic instants.
pub trait MonotonicClock {
    /// Returns the current monotonic instant.
    fn instant(&self) -> Instant;
}

/// A full time context that can provide wall-clock and monotonic time.
pub trait TimeProvider: WallClock + MonotonicClock + Sync {}

impl<T> TimeProvider for T where T: WallClock + MonotonicClock + Sync {}

// On platforms without std, users must provide the time provider symbol via define_time_provider! macro
extern "Rust" {
    #[link_name = "__universal_time_provider"]
    static TIME_PROVIDER: &'static dyn TimeProvider;
}

#[inline(always)]
pub(crate) fn get_time_provider() -> &'static dyn TimeProvider {
    unsafe { TIME_PROVIDER }
}

/// Macro to define a custom time provider for no_std platforms.
///
/// This macro must be called in your binary crate when using this library on no_std
/// or WASM unknown targets. Pass a static instance of your time provider.
///
/// # Example
///
/// ```
/// use core::time::Duration;
///
/// use universal_time::{define_time_provider, Instant, MonotonicClock, SystemTime, WallClock};
///
/// struct MyTimeProvider;
///
/// impl WallClock for MyTimeProvider {
///     fn system_time(&self) -> SystemTime {
///         SystemTime::from_unix_duration(Duration::from_secs(0))
///     }
/// }
///
/// impl MonotonicClock for MyTimeProvider {
///     fn instant(&self) -> Instant {
///         Instant::from_ticks(Duration::from_secs(0))
///     }
/// }
///
/// define_time_provider!(MyTimeProvider);
/// ```
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        not(feature = "std"),
        all(feature = "std", target_family = "wasm", target_os = "unknown")
    )))
)]
#[macro_export]
macro_rules! define_time_provider {
    ($provider_instance:expr) => {
        #[export_name = "__universal_time_provider"]
        static __UNIVERSAL_TIME_PROVIDER: &dyn $crate::TimeProvider = &$provider_instance;
    };
}
