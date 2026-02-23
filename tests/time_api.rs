use std::sync::OnceLock;
use std::time::Duration;

use universal_time::{
    global_time_context, set_global_time_context, GlobalTimeContextAlreadySet, Instant,
    MonotonicClock, SystemTime, TimeContext, WallClock, UNIX_EPOCH,
};

struct TestContext;

static TEST_CONTEXT: TestContext = TestContext;
static TEST_START: OnceLock<std::time::Instant> = OnceLock::new();
static INSTALL_RESULT: OnceLock<Result<(), GlobalTimeContextAlreadySet>> = OnceLock::new();

impl WallClock for TestContext {
    fn system_time(&self) -> Option<SystemTime> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?;
        Some(SystemTime::from_unix_duration(now))
    }
}

impl MonotonicClock for TestContext {
    fn instant(&self) -> Option<Instant> {
        let ticks = TEST_START.get_or_init(std::time::Instant::now).elapsed();
        Some(Instant::from_ticks(ticks))
    }
}

fn install_test_context() {
    let _ = INSTALL_RESULT.get_or_init(|| set_global_time_context(&TEST_CONTEXT));
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn test_context_is_send_sync() {
    assert_send_sync::<TestContext>();
}

#[test]
fn time_context_requires_send_sync() {
    fn assert_time_context<T: TimeContext + Send + Sync>() {}
    assert_time_context::<TestContext>();
}

#[test]
fn unix_epoch_is_zero() {
    assert_eq!(UNIX_EPOCH.as_unix_duration(), Duration::ZERO);
}

#[test]
fn system_time_roundtrip_unix_duration() {
    let duration = Duration::from_secs(123) + Duration::from_nanos(456);
    let time = SystemTime::from_unix_duration(duration);
    assert_eq!(time.as_unix_duration(), duration);
}

#[test]
fn system_time_duration_since_forward() {
    let earlier = SystemTime::from_unix_duration(Duration::from_secs(10));
    let later = SystemTime::from_unix_duration(Duration::from_secs(12));
    assert_eq!(later.duration_since(earlier), Ok(Duration::from_secs(2)));
}

#[test]
fn system_time_duration_since_backward() {
    let earlier = SystemTime::from_unix_duration(Duration::from_secs(10));
    let later = SystemTime::from_unix_duration(Duration::from_secs(12));
    assert_eq!(earlier.duration_since(later), Err(Duration::from_secs(2)));
}

#[test]
fn instant_roundtrip_ticks() {
    let ticks = Duration::from_millis(42);
    let instant = Instant::from_ticks(ticks);
    assert_eq!(instant.to_ticks(), ticks);
}

#[test]
fn instant_duration_since_saturates_at_zero() {
    let earlier = Instant::from_ticks(Duration::from_secs(10));
    let later = Instant::from_ticks(Duration::from_secs(3));
    assert_eq!(later.duration_since(earlier), Duration::ZERO);
}

#[test]
fn instant_checked_duration_since_some() {
    let earlier = Instant::from_ticks(Duration::from_secs(3));
    let later = Instant::from_ticks(Duration::from_secs(10));
    assert_eq!(
        later.checked_duration_since(earlier),
        Some(Duration::from_secs(7))
    );
}

#[test]
fn instant_checked_duration_since_none() {
    let earlier = Instant::from_ticks(Duration::from_secs(10));
    let later = Instant::from_ticks(Duration::from_secs(3));
    assert_eq!(later.checked_duration_since(earlier), None);
}

#[test]
fn instant_checked_add_and_sub_roundtrip() {
    let start = Instant::from_ticks(Duration::from_secs(5));
    let delta = Duration::from_secs(2);
    let end = start.checked_add(delta).expect("must not overflow");
    assert_eq!(end.to_ticks(), Duration::from_secs(7));
    assert_eq!(end.checked_sub(delta), Some(start));
}

#[test]
fn instant_checked_add_overflow_returns_none() {
    let start = Instant::from_ticks(Duration::MAX);
    assert_eq!(start.checked_add(Duration::from_nanos(1)), None);
}

#[test]
fn instant_checked_sub_underflow_returns_none() {
    let start = Instant::from_ticks(Duration::ZERO);
    assert_eq!(start.checked_sub(Duration::from_nanos(1)), None);
}

#[test]
fn instant_add_operator_works() {
    let start = Instant::from_ticks(Duration::from_secs(5));
    let end = start + Duration::from_secs(2);
    assert_eq!(end.to_ticks(), Duration::from_secs(7));
}

#[test]
#[should_panic(expected = "overflow while adding Duration to Instant")]
fn instant_add_operator_panics_on_overflow() {
    let _ = Instant::from_ticks(Duration::MAX) + Duration::from_nanos(1);
}

#[test]
fn instant_sub_operator_works() {
    let end = Instant::from_ticks(Duration::from_secs(7));
    let start = end - Duration::from_secs(2);
    assert_eq!(start.to_ticks(), Duration::from_secs(5));
}

#[test]
#[should_panic(expected = "underflow while subtracting Duration from Instant")]
fn instant_sub_operator_panics_on_underflow() {
    let _ = Instant::from_ticks(Duration::ZERO) - Duration::from_nanos(1);
}

#[test]
fn instant_sub_instant_operator_is_saturating() {
    let a = Instant::from_ticks(Duration::from_secs(9));
    let b = Instant::from_ticks(Duration::from_secs(4));
    assert_eq!(a - b, Duration::from_secs(5));
    assert_eq!(b - a, Duration::ZERO);
}

#[test]
fn global_context_is_available_after_install() {
    install_test_context();
    assert!(global_time_context().is_some());
}

#[test]
fn global_context_can_only_be_set_once() {
    install_test_context();
    assert_eq!(
        set_global_time_context(&TEST_CONTEXT),
        Err(GlobalTimeContextAlreadySet)
    );
}

#[test]
fn instant_now_is_monotonic() {
    install_test_context();
    let first = Instant::now();
    let second = Instant::now();
    assert!(second >= first);
}

#[test]
fn elapsed_is_non_negative() {
    install_test_context();
    let start = Instant::now();
    assert!(start.elapsed() >= Duration::ZERO);
}

#[test]
fn system_time_now_is_after_unix_epoch() {
    install_test_context();
    let now = SystemTime::now();
    assert!(now.duration_since(UNIX_EPOCH).is_ok());
}
