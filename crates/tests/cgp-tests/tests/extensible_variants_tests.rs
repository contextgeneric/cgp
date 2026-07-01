//! Entrypoint for the `extensible_variants` concept.
//!
//! Covers CGP's extensible *variant* (enum) data: `#[derive(CgpData)]` and
//! `#[derive(HasFields)]` applied to enums, the extractor family that
//! deconstructs a value variant-by-variant (`HasExtractor`/`ExtractField`/
//! `FinalizeExtract`), the `FromVariant` constructor, the structural casts
//! (`CanUpcast`/`CanDowncast`) between enum shapes, dispatching an
//! extensible-variant input to per-variant handlers, and the `Sum!` type-level
//! sum list that underlies all of it.
//!
//! The dual concept for structs is `extensible_records`; `field_access` owns
//! the `#[derive(HasField)]` struct derive.
//!
//! See docs/concepts/extensible-variants.md, docs/reference/derives/derive_cgp_data.md,
//! docs/reference/derives/derive_from_variant.md, docs/reference/derives/derive_extract_field.md,
//! and docs/reference/macros/sum.md.
#![allow(dead_code)]

pub mod extensible_variants;
