use universal_time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let start = Instant::now();
    let now = SystemTime::now();
    let elapsed = start.elapsed();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();

    println!("elapsed = {:?}", elapsed);
    println!("since epoch = {:?}", since_epoch);
}
