#![doc = include_str!("../README.md")]
#![no_std]
#![allow(mixed_script_confusables)]
#![warn(missing_docs)]

/// The core names users write, re-exported through `cgp::prelude`.
pub mod prelude;

/// Every core name the macros' generated code refers to, re-exported through `cgp::macro_prelude`.
/// Only the macros use this module; import from `cgp::prelude` or the defining module instead.
pub mod macro_prelude;
#[doc(inline)]
pub use {
    cgp_async_macro::async_trait, cgp_base as base, cgp_component as component, cgp_error as error,
    cgp_field as field, cgp_macro as macros, cgp_type as types,
};
