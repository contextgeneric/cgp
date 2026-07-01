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
//! See docs/reference/derives/derive_cgp_data.md,
//! docs/reference/derives/derive_has_fields.md,
//! docs/reference/traits/has_builder.md, and
//! docs/concepts/extensible-records.md.
#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

pub mod extensible_records;
