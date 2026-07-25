//! Entrypoint for the `extensible_records` concept.
//!
//! Covers CGP's extensible-record (struct) data machinery: `#[derive(CgpData)]`,
//! which derives the full record spine — `HasField`/`HasFieldMut`, `HasFields`,
//! the `From`/`To` field-list conversions, and the extensible *builder* pattern
//! (`HasBuilder`, `BuildField`, `build_from`, `finalize_build`) via a generated
//! `__Partial…` type — and `#[derive(HasFields)]`, which derives only the field
//! list. This concept owns the canonical macro-expansion snapshots for both
//! derives on structs, and exercises named, tuple, generic, optional, and cast
//! variations.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_cgp_data.md,
//! cgp-knowledge-base/cgp/reference/derives/derive_has_fields.md,
//! cgp-knowledge-base/cgp/reference/traits/has_builder.md, and
//! cgp-knowledge-base/cgp/concepts/extensible-records.md.
#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

pub mod extensible_records;
