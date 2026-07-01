//! Entrypoint for the `getters` concept.
//!
//! Covers CGP's getter macros: `#[cgp_auto_getter]` (a blanket getter impl over
//! `HasField`, with the field taken from the method name) and `#[cgp_getter]`
//! (a full component whose source field can be chosen by wiring through
//! `UseField`). This concept owns the canonical macro-expansion snapshots for
//! both, exercising the return-type shorthands (`&str`, `&[u8]`, `Option<&T>`,
//! `MRef`, clone-on-`Copy`), associated getter types, generic getters, non-`self`
//! getters, and abstract-type integration.
//!
//! See docs/reference/macros/cgp_getter.md and
//! docs/reference/macros/cgp_auto_getter.md.
#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

pub mod getters;
