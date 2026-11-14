//! This crate re-exports CGP's base layer: the type-level primitives, the component machinery, and
//! the macros.
//!
//! See [`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component), [`Symbol!`](https://contextgeneric.dev/docs/reference/macros/symbol), and [`Cons`](https://contextgeneric.dev/docs/reference/types/cons).
#![no_std]
#![warn(missing_docs)]

/// The base-layer names the macros' generated code refers to.
/// Only the macros use this module; import from `cgp::prelude` or the defining module instead.
pub mod macro_prelude;

pub use cgp_base_types::*;
pub use cgp_component as component;
