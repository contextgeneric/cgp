//! Acceptable failure: a getter (or `#[implicit]` argument) typed `Option<&[T]>`
//! combines the `Option<&T>` and `&[T]` shorthands, a combination CGP does not
//! provide magic for. The shared `parse_field_type` lowers it literally by the
//! `Option<&T>` rule — reading an `Option<T>` field where `T` is the slice `[u8]`
//! — so the generated `HasField` bound names the unsized `Option<[u8]>`, which
//! `rustc` rejects (`[u8]` has no statically known size). CGP is working as
//! designed: the `Option` and slice shorthands ease the common single-shape
//! cases, and an unsupported combination is deferred to the compiler rather than
//! given a bespoke rule. The same boundary holds for `#[cgp_getter]` and for a
//! `#[cgp_fn]` `#[implicit]` argument, since all three share `parse_field_type`.
//!
//! See docs/errors/lowering/ill-formed-generated-type.md; the parser detail is in
//! docs/implementation/entrypoints/cgp_auto_getter.md (Behavior and corner cases).

use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasItems {
    fn items(&self) -> Option<&[u8]>;
}

fn main() {}
