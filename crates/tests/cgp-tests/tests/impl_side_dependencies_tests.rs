//! Entrypoint for the `impl_side_dependencies` concept.
//!
//! Covers the capabilities a provider or `#[cgp_fn]` function requires but the
//! consumer trait does not itself expose — CGP's form of dependency injection.
//! These impl-side dependencies are declared with `#[uses(...)]` (import `Self`
//! trait bounds, read like a `use` statement) and `#[extend(...)]` (add a
//! *supertrait* bound to the generated trait). This concept owns the `#[cgp_fn]`
//! snapshots that show how `#[uses]`/`#[extend]` land on the generated trait and
//! impl.
//!
//! See cgp-knowledge-base/cgp/concepts/impl-side-dependencies.md,
//! cgp-knowledge-base/cgp/reference/attributes/uses.md,
//! cgp-knowledge-base/cgp/reference/attributes/extend.md, and
//! cgp-knowledge-base/cgp/reference/attributes/extend_where.md.
#![allow(dead_code)]

pub mod impl_side_dependencies;
