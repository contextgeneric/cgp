//! Entrypoint for the `field_access` concept.
//!
//! Covers tag-keyed field access: the `HasField`/`HasFieldMut` traits and the
//! `#[derive(HasField)]` derive that generates their impls from a struct's
//! fields, together with the type-level tags those impls are keyed by —
//! `Symbol!("name")` for named fields and `Index<N>` for tuple fields. This
//! concept owns the canonical `#[derive(HasField)]` macro-expansion snapshots,
//! exercising named fields, tuple (positional) fields, lifetime-carrying fields,
//! and nested/chained field access, plus the runtime behavior of the `Symbol!`
//! and `Index` tags themselves (`Display` and `StaticString`).
//!
//! See docs/reference/traits/has_field.md,
//! docs/reference/derives/derive_has_field.md,
//! docs/reference/macros/symbol.md and docs/reference/types/index.md.
#![allow(dead_code)]

pub mod field_access;
