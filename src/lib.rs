#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use core::time::Duration;

#[cfg(any(
    not(feature = "std"),
    all(feature = "std", target_family = "wasm", target_os = "unknown")
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        not(feature = "std"),
        all(feature = "std", target_family = "wasm", target_os = "unknown")
    )))
)]
mod global;
mod instant;
mod system;

#[cfg(any(
    not(feature = "std"),
    all(feature = "std", target_family = "wasm", target_os = "unknown")
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        not(feature = "std"),
        all(feature = "std", target_family = "wasm", target_os = "unknown")
    )))
)]
pub use self::global::*;
pub use self::instant::*;
pub use self::system::*;
