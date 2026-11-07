//! This crate holds the shared core of CGP's procedural macros: the AST types, parsers, visitors,
//! and code-generation helpers the macro crates build on.
//!
//! See [`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component) and [`#[cgp_impl]`](https://contextgeneric.dev/docs/reference/macros/cgp_impl) for the macros it builds.

pub mod exports;
pub mod functions;
pub mod macros;
pub mod traits;
pub mod types;
pub mod vendor;
pub mod visitors;
