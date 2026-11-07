#![no_std]
//! This crate re-exports CGP's base layer with the abstract-type layer added.
//!
//! See [`HasType`](https://contextgeneric.dev/docs/reference/components/has_type).

pub mod macro_prelude;

pub use cgp_base::{component, types};
