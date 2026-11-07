#![no_std]
//! This crate re-exports CGP's base layer with the abstract-type layer added.
//!
//! See [`HasType`](https://contextgeneric.dev/docs/reference/components/has_type).
#![warn(missing_docs)]

/// The prelude, re-exported as `cgp_base_extra::macro_prelude`.
pub mod macro_prelude;

pub use cgp_base::{component, types};
