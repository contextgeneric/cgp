//! Entrypoint for the `namespaces` concept.
//!
//! Covers CGP's namespace feature: reusable, inheritable wiring tables built
//! with `cgp_namespace!`, components attached to a namespace with `#[prefix(...)]`,
//! the `namespace`/`open` statements inside `delegate_components!`, the
//! `RedirectLookup` provider that re-routes a lookup along a type-level `Path!`,
//! the `DefaultNamespace` trait for default provider resolution, and namespace
//! inheritance (`cgp_namespace! { new Child: Parent { .. } }`, `DefaultImpls1`,
//! and the `for <..> in ..` default-impl loop).
//!
//! This concept owns the canonical macro-expansion snapshots for `cgp_namespace!`
//! and for the namespace/`open`/`#[prefix]`/`@`-path forms of
//! `delegate_components!`; incidental uses of other macros are written plainly,
//! since their expansion is pinned in their owning target.
//!
//! See docs/concepts/namespaces.md, docs/reference/macros/cgp_namespace.md,
//! docs/reference/macros/delegate_components.md,
//! docs/reference/providers/redirect_lookup.md, and
//! docs/reference/traits/default_namespace.md.
#![allow(dead_code)]

pub mod namespaces;
