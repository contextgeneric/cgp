//! This crate re-exports CGP's base layer: the type-level primitives, the component machinery, and
//! the macros.
//!
//! See [`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component), [`Symbol!`](https://contextgeneric.dev/docs/reference/macros/symbol), and [`Cons`](https://contextgeneric.dev/docs/reference/types/cons).
#![no_std]
#![warn(missing_docs)]

/// The prelude, re-exported as `cgp_base::macro_prelude`.
pub mod macro_prelude;

pub use cgp_base_types::*;
pub use cgp_component as component;
