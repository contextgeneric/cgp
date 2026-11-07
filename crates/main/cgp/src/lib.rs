#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[doc(inline)]
pub use {cgp_core as core, cgp_extra as extra};

/// The prelude, re-exported as `cgp::prelude` and `cgp::macro_prelude`.
pub mod prelude;

pub use prelude as macro_prelude;
