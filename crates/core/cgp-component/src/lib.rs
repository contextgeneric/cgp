#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

/// Re-exports the component machinery that the macros expand to.
/// Only the macros use this module; import from `cgp::prelude` or the defining module instead.
pub mod macro_prelude;

mod namespaces;
mod providers;
mod traits;

pub use namespaces::*;
pub use providers::*;
pub use traits::*;
