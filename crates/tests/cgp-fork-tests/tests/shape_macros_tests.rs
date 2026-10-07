//! Entrypoint for the `shape_macros` concept.
//!
//! Covers the `Struct!` and `Enum!` type-level shape macros, which build the
//! `HasFields` shape of a struct or enum from the body of its declaration. Most
//! tests are type equalities: against the hand-written `Product!`/`Sum!` list,
//! and against the `Fields` that `#[derive(HasFields)]` gives the same body.
//! Neither macro has a snapshot, since each emits a single type.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/struct.md,
//! cgp-knowledge-base-fork/cgp/reference/macros/enum.md, and
//! cgp-knowledge-base-fork/cgp/implementation/entrypoints/struct.md.
#![allow(dead_code)]
// Clippy measures a shape by its expansion, where each field name is a deep
// `Symbol<N, Chars<…>>` chain, so `type_complexity` fires on ordinary shapes.
#![allow(clippy::type_complexity)]

pub mod shape_macros;
