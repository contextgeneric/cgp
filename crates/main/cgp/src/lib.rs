#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[doc(inline)]
pub use {cgp_core as core, cgp_extra as extra};

/// The names users write, imported with `use cgp::prelude::*`.
pub mod prelude;

/// Every name the macros' generated code refers to, as `::cgp::macro_prelude::Name`.
pub mod macro_prelude;
