//! This crate holds the shared core of CGP's extra procedural macros: their AST types, parsers, and
//! visitors.
//!
//! See [`#[cgp_computer]`](https://contextgeneric.dev/docs/reference/macros/cgp_computer) and [`#[cgp_auto_dispatch]`](https://contextgeneric.dev/docs/reference/macros/cgp_auto_dispatch) for the macros it builds.

pub mod exports;
pub mod functions;
pub mod types;
pub mod visitors;
