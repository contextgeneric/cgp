//! Entrypoint for tests of the attributes the extensible-data derives copy onto their generated
//! companion types.
//!
//! The builder's `__Partial{Name}` struct and the extractor's `__Partial{Name}` and
//! `__PartialRef{Name}` enums clone the input's fields or variants. A helper attribute of another
//! derive on one of them (`#[default]`, `#[serde(…)]`) would fail to resolve on a companion that
//! lacks that derive, so the companions keep only `#[cfg]` and documentation. These call the derive
//! entrypoints directly, since a test crate cannot compile a helper attribute without the derive
//! that declares it.
#![allow(dead_code)]

pub mod companion_attrs;
