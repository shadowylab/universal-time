//! Cross-platform time primitives.
//!
//! Provides `Instant` and `SystemTime` plus traits for injecting custom
//! platform clocks through a global time context.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use core::time::Duration;

mod global;
mod instant;
mod system;

pub use self::global::*;
pub use self::instant::*;
pub use self::system::*;
