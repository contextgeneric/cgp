//! Entrypoint for the `implicit_arguments` concept.
//!
//! Covers `#[implicit]` arguments — parameters removed from a `#[cgp_fn]` or
//! `#[cgp_impl]` signature and read from the context's fields via `HasField`.
//! This concept owns the `#[cgp_fn]` expansion snapshots (showing the generated
//! `HasField` bounds and field reads), and exercises implicit args in `#[cgp_impl]`
//! providers, including the implicit context parameter.
//!
//! See docs/reference/attributes/implicit.md, docs/reference/macros/cgp_fn.md,
//! and docs/concepts/implicit-arguments.md.
#![allow(dead_code)]

pub mod implicit_arguments;
