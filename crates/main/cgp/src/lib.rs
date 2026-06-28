#![no_std]
#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use {cgp_core as core, cgp_extra as extra};

pub mod prelude;

pub use prelude as macro_prelude;
