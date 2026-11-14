#![no_std]
//! This crate re-exports CGP's base layer with the abstract-type layer added.
//!
//! See [`HasType`](https://contextgeneric.dev/docs/reference/components/has_type).
#![warn(missing_docs)]

/// The extra base-layer names the macros' generated code refers to.
/// Only the macros use this module; import from `cgp::prelude` or the defining module instead.
pub mod macro_prelude;

pub use cgp_base::{component, types};
