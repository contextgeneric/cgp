//! Entrypoint for the `abstract_types` concept.
//!
//! Covers CGP's abstract-type feature end to end: the `#[cgp_type]` macro (an
//! abstract-type component whose expansion additionally emits the `UseType`
//! blanket impl), the `UseType<T>` provider that fixes the type by wiring, and
//! the `#[use_type(...)]` attribute that imports an abstract type into a
//! `#[cgp_component]`/`#[cgp_impl]`/`#[cgp_fn]` and rewrites the bare alias
//! (`Scalar`, `Error`) to its fully-qualified associated-type form. This concept
//! owns the canonical `#[cgp_type]` macro-expansion snapshots and the
//! abstract-type-rewriting snapshots for `#[use_type]`.
//!
//! See docs/reference/macros/cgp_type.md, docs/reference/attributes/use_type.md,
//! docs/reference/providers/use_type.md, and docs/concepts/abstract-types.md.
#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

pub mod abstract_types;
