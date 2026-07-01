//! Entrypoint for the `generic_components` concept.
//!
//! Covers components and functions that carry generic parameters: type
//! parameters, lifetimes, and const generics. This concept owns the
//! generic-parameter variants of the `#[cgp_component]` and `#[cgp_fn]`
//! macro-expansion snapshots — how a leading `__Context__` parameter is threaded
//! alongside user generics, how lifetimes are lifted into `Life<'a>` in the
//! `IsProviderFor` params tuple, and how const generics flow onto the generated
//! provider trait and its providers.
//!
//! See docs/reference/macros/cgp_component.md, docs/reference/macros/cgp_fn.md,
//! docs/reference/types/life.md, and docs/concepts/higher-order-providers.md.
#![allow(dead_code)]

pub mod generic_components;
