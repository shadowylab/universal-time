# universal-time

Cross-platform time primitives with **compile-time guarantees** — no runtime panics!

This library provides `Instant` (monotonic time) and `SystemTime` (wall-clock time) that work consistently across all platforms with zero runtime overhead.

## Why?

This library **fails at link time** if you try to build without a time provider on platforms that need one. This means:

- ✅ **Zero runtime panics** from missing time sources
- ✅ **Compile-time verification** that time is available
- ✅ **Single consistent API** across all platforms
- ✅ **No overhead** – compiles away to platform calls

## Quick Start

### With `std` (default)

Works automatically with `std::time`:

```rust,no_run
use universal_time::{Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let start = Instant::now();
    let elapsed = start.elapsed();

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    println!("Elapsed: {:?}", elapsed);
    println!("Since Unix epoch: {:?}", since_epoch);
}
```

### Without `std` (embedded, WASM unknown)

Define a custom time provider using the `define_time_provider!` macro:

```rust,ignore
# use core::time::Duration;
# use universal_time::{define_time_provider, Instant, SystemTime, WallClock, MonotonicClock};
struct MyTimeProvider;

impl WallClock for MyTimeProvider {
    fn system_time(&self) -> SystemTime {
        // Your platform-specific implementation
        # SystemTime::from_unix_duration(Duration::from_secs(0))
    }
}

impl MonotonicClock for MyTimeProvider {
    fn instant(&self) -> Instant {
        // Your platform-specific implementation
        # Instant::from_ticks(Duration::from_secs(0))
    }
}

define_time_provider!(MyTimeProvider);
# fn main() {
#    // Now Instant::now() and SystemTime::now() work!
#     let _now = Instant::now();
# }
```

## How It Works

The library uses **extern symbols** to enforce time provider availability at **link time**:

| Platform                             | Behavior                               |
|--------------------------------------|----------------------------------------|
| Linux/macOS/Windows with `std`       | Uses `std::time` automatically         |
| `no_std` and `wasm*-unknown-unknown` | Requires `define_time_provider!` macro |
| Other WASM targets with `std`        | Uses `std::time` automatically         |

**Without a provider on no_std**, you get a clear link error:
```text
error: undefined reference to '__universal_time_provider'
```

**Duplicate provider?** Link error: "duplicate symbol" – catches configuration mistakes at compile time!

## Features

- `std` (enabled by default) - Uses `std::time` on supported platforms

## Changelog

All notable changes to this library are documented in the [CHANGELOG.md](CHANGELOG.md).

## License

This project is distributed under the MIT software license – see the [LICENSE](./LICENSE) file for details
