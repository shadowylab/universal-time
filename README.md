# universal-time

Cross-platform time primitives for Rust that can run in any envuironment.

## Why

`universal-time` gives you a single API for:

- `Instant` for monotonic elapsed-time measurements
- `SystemTime` for wall-clock timestamps
- Trait-based clock injection for platforms without built-in time access

## Quick Start

```rust,no_run
use universal_time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let start = Instant::now();
    let now = SystemTime::now();
    let elapsed = start.elapsed();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();

    println!("elapsed = {:?}", elapsed);
    println!("since epoch = {:?}", since_epoch);
}
```

For more examples, check out the [examples](examples) directory.

## Panic Behavior

In `no_std` mode, and in `std` mode on `wasm32-unknown-unknown`, both
`Instant::now()` and `SystemTime::now()` panic when:

- no global context has been installed, or
- installed context returns `None` for that clock type

This is intentional so missing time sources fail fast instead of silently returning fake timestamps.

## Concurrency Notes

- `std`: global context uses `OnceLock`
- `no_std` with atomics: global context uses lock-free once initialization
- `no_std` without atomics: fallback expects single-threaded startup initialization
