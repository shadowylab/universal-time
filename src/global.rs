#[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
use core::cell::UnsafeCell;
#[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
use core::sync::atomic::{AtomicU8, Ordering};

use crate::{Instant, SystemTime};

/// Source of wall-clock timestamps.
pub trait WallClock {
    /// Returns the current wall-clock time, or `None` if not available.
    fn system_time(&self) -> Option<SystemTime>;
}

/// Source of monotonic instants.
pub trait MonotonicClock {
    /// Returns the current monotonic instant, or `None` if not available.
    fn instant(&self) -> Option<Instant>;
}

/// A full time context that can provide wall-clock and monotonic time.
pub trait TimeContext: WallClock + MonotonicClock + Send + Sync {}

impl<T> TimeContext for T where T: WallClock + MonotonicClock + Send + Sync {}

/// Error returned when attempting to set the global time context more than once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalTimeContextAlreadySet;

#[cfg(feature = "std")]
static GLOBAL_TIME_CONTEXT: std::sync::OnceLock<&'static dyn TimeContext> =
    std::sync::OnceLock::new();

#[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
struct NoStdGlobalTimeContext {
    state: AtomicU8,
    value: UnsafeCell<Option<&'static dyn TimeContext>>,
}

#[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
unsafe impl Sync for NoStdGlobalTimeContext {}

#[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
impl NoStdGlobalTimeContext {
    const UNINITIALIZED: u8 = 0;
    const INITIALIZING: u8 = 1;
    const READY: u8 = 2;

    const fn new() -> Self {
        Self {
            state: AtomicU8::new(Self::UNINITIALIZED),
            value: UnsafeCell::new(None),
        }
    }

    fn set(&self, context: &'static dyn TimeContext) -> Result<(), GlobalTimeContextAlreadySet> {
        match self.state.compare_exchange(
            Self::UNINITIALIZED,
            Self::INITIALIZING,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                unsafe {
                    *self.value.get() = Some(context);
                }
                self.state.store(Self::READY, Ordering::Release);
                Ok(())
            }
            Err(_) => {
                while self.state.load(Ordering::Acquire) == Self::INITIALIZING {
                    core::hint::spin_loop();
                }
                Err(GlobalTimeContextAlreadySet)
            }
        }
    }

    fn get(&self) -> Option<&'static dyn TimeContext> {
        let mut state = self.state.load(Ordering::Acquire);
        while state == Self::INITIALIZING {
            core::hint::spin_loop();
            state = self.state.load(Ordering::Acquire);
        }

        if state == Self::READY {
            unsafe { *self.value.get() }
        } else {
            None
        }
    }
}

#[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
static GLOBAL_TIME_CONTEXT: NoStdGlobalTimeContext = NoStdGlobalTimeContext::new();

#[cfg(all(not(feature = "std"), not(target_has_atomic = "8")))]
static mut GLOBAL_TIME_CONTEXT: Option<&'static dyn TimeContext> = None;

/// Installs the global time context.
///
/// This can be called only once for the process lifetime.
pub fn set_global_time_context(
    context: &'static dyn TimeContext,
) -> Result<(), GlobalTimeContextAlreadySet> {
    #[cfg(feature = "std")]
    {
        GLOBAL_TIME_CONTEXT
            .set(context)
            .map_err(|_| GlobalTimeContextAlreadySet)
    }

    #[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
    {
        GLOBAL_TIME_CONTEXT.set(context)
    }

    #[cfg(all(not(feature = "std"), not(target_has_atomic = "8")))]
    {
        // Fallback for targets without atomics. Call during single-threaded startup
        // before concurrency begins.
        unsafe {
            let current = core::ptr::read(core::ptr::addr_of!(GLOBAL_TIME_CONTEXT));
            if current.is_some() {
                Err(GlobalTimeContextAlreadySet)
            } else {
                core::ptr::write(core::ptr::addr_of_mut!(GLOBAL_TIME_CONTEXT), Some(context));
                Ok(())
            }
        }
    }
}

/// Returns the globally configured time context if one was installed.
pub fn global_time_context() -> Option<&'static dyn TimeContext> {
    #[cfg(feature = "std")]
    {
        GLOBAL_TIME_CONTEXT.get().copied()
    }

    #[cfg(all(not(feature = "std"), target_has_atomic = "8"))]
    {
        GLOBAL_TIME_CONTEXT.get()
    }

    #[cfg(all(not(feature = "std"), not(target_has_atomic = "8")))]
    {
        // Fallback for targets without atomics. See synchronization note in
        // `set_global_time_context`.
        unsafe { core::ptr::read(core::ptr::addr_of!(GLOBAL_TIME_CONTEXT)) }
    }
}

#[cfg(any(
    not(feature = "std"),
    all(feature = "std", target_family = "wasm", target_os = "unknown")
))]
pub(crate) fn panic_missing_system_time() -> ! {
    panic!(
        "no wall-clock time source is available; install one with universal_time::set_global_time_context()"
    )
}

#[cfg(any(
    not(feature = "std"),
    all(feature = "std", target_family = "wasm", target_os = "unknown")
))]
pub(crate) fn panic_missing_instant() -> ! {
    panic!(
        "no monotonic clock is available; install one with universal_time::set_global_time_context()"
    )
}
