//! Entrypoint for the `blanket_traits` concept.
//!
//! Covers the `#[blanket_trait]` macro, which turns a trait with supertrait
//! bounds (and optional default methods / associated types) into an extension
//! trait plus a blanket impl that applies to every context satisfying those
//! bounds. This concept OWNS the canonical `#[blanket_trait]` expansion
//! snapshots.
//!
//! See docs/reference/macros/blanket_trait.md.
#![allow(dead_code)]

pub mod blanket_traits;
