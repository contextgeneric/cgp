//! This crate holds the shared core of CGP's procedural macros: the AST types, parsers, visitors,
//! and code-generation helpers the macro crates build on.
//!
//! See [`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component) and [`#[cgp_impl]`](https://contextgeneric.dev/docs/reference/macros/cgp_impl) for the macros it builds.
#![warn(missing_docs)]

/// The re-exports the macro crates use.
pub mod exports;
/// The shared parsing and code-generation helpers.
pub mod functions;
/// The helper macros the AST types use.
pub mod macros;
/// The shared traits the AST types implement.
pub mod traits;
/// The AST types for the macros.
pub mod types;
/// The vendored third-party helpers.
pub mod vendor;
/// The visitors that rewrite the parsed items.
pub mod visitors;
