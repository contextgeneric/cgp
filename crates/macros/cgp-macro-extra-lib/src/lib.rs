//! This crate implements CGP's extra procedural macros, which lower through `cgp-macro-core`'s AST
//! types.
//!
//! See [`#[cgp_computer]`](https://contextgeneric.dev/docs/reference/macros/cgp_computer), [`#[cgp_producer]`](https://contextgeneric.dev/docs/reference/macros/cgp_producer), and [`#[cgp_auto_dispatch]`](https://contextgeneric.dev/docs/reference/macros/cgp_auto_dispatch).
#![warn(missing_docs)]

mod cgp_auto_dispatch;
mod cgp_computer;
mod cgp_producer;
mod handler_fn;

pub use cgp_auto_dispatch::*;
pub use cgp_computer::*;
pub use cgp_producer::*;
