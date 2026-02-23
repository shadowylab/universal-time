use core::time::Duration;

use universal_time::{
    set_global_time_context, Instant, MonotonicClock, SystemTime, WallClock, UNIX_EPOCH,
};

struct BoardClock;

impl WallClock for BoardClock {
    fn system_time(&self) -> Option<SystemTime> {
        // Replace with your RTC / SNTP source.
        let unix_secs: u64 = rtc_unix_seconds();
        Some(SystemTime::from_unix_duration(Duration::from_secs(
            unix_secs,
        )))
    }
}

impl MonotonicClock for BoardClock {
    fn instant(&self) -> Option<Instant> {
        // Replace with your monotonic timer ticks.
        let millis: u64 = board_millis_since_boot();
        Some(Instant::from_ticks(Duration::from_millis(millis)))
    }
}

fn rtc_unix_seconds() -> u64 {
    // platform-specific implementation
    0
}

fn board_millis_since_boot() -> u64 {
    // platform-specific implementation
    0
}

static CLOCK: BoardClock = BoardClock;

fn main() {
    set_global_time_context(&CLOCK).unwrap();

    let start = Instant::now();
    let now = SystemTime::now();
    let elapsed = start.elapsed();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();

    println!("elapsed = {:?}", elapsed);
    println!("since epoch = {:?}", since_epoch);
}
