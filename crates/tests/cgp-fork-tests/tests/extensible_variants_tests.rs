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
//! See cgp-knowledge-base/cgp/concepts/extensible-variants.md,
//! cgp-knowledge-base/cgp/reference/derives/derive_cgp_data.md,
//! cgp-knowledge-base/cgp/reference/derives/derive_from_variant.md,
//! cgp-knowledge-base/cgp/reference/derives/derive_extract_field.md, and
//! cgp-knowledge-base/cgp/reference/macros/sum.md.
#![allow(dead_code)]

pub mod extensible_variants;
