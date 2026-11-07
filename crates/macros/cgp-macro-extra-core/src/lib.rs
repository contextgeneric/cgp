//! This crate holds the shared core of CGP's extra procedural macros: their AST types, parsers, and
//! visitors.
//!
//! See [`#[cgp_computer]`](https://contextgeneric.dev/docs/reference/macros/cgp_computer) and [`#[cgp_auto_dispatch]`](https://contextgeneric.dev/docs/reference/macros/cgp_auto_dispatch) for the macros it builds.
#![warn(missing_docs)]

/// The re-exports the macro crates use.
pub mod exports;
/// The shared parsing and code-generation helpers.
pub mod functions;
/// The AST types for the extra macros.
pub mod types;
/// The visitors that rewrite the parsed items.
pub mod visitors;
