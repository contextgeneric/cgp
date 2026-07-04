//! Problematic failure: the extractor codegen does not handle an enum with no
//! variants. `#[derive(ExtractField)]` (and therefore `#[derive(CgpVariant)]`
//! and `#[derive(CgpData)]`) on a variantless enum still emits the borrowed
//! partial enum `__PartialRef…<'__a__, __R__: MapTypeRef>`, whose lifetime and
//! `MapTypeRef` parameters no variant uses (`E0392`), and emits
//! `extractor_ref`/`extractor_mut` bodies that are `match self {}` over a
//! reference to the uninhabited enum, which the exhaustiveness checker rejects
//! because a reference to an empty type is not itself considered empty (`E0004`).
//!
//! An empty enum is uninhabited and so can never hold a value to extract, making
//! the machinery degenerate; the macro should either reject it at expansion time
//! with a spanned error or emit an extractor that compiles, rather than emitting
//! code that fails downstream. `#[derive(FromVariant)]` has no such problem — it
//! emits zero impls for a variantless enum.
//!
//! See the `## Known issues` section of
//! docs/implementation/entrypoints/derive_extract_field.md.

use cgp::prelude::*;

#[derive(ExtractField)]
pub enum Never {}

fn main() {}
