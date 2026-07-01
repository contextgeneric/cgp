//! One unit test per file. Each file is self-contained: it defines its own
//! record types, builders, and context wiring at module scope so that the
//! type-level machinery of one test never leaks into another.

// `#[derive(CgpData)]` on structs (this concept owns the derive's expansion):
// the full record spine — field access, field lists, and the extensible builder.
pub mod generic_record;
pub mod optional_builder;
pub mod person_record;
pub mod point_cast;
pub mod record_derive;
pub mod tuple_record;

// Behavioral record building: assembling a record from other records and from
// handler pipelines. These reuse `#[derive(CgpData)]` as plain scaffolding —
// the derive expansion is already pinned by `record_derive`.
pub mod record_build_from;
pub mod record_build_with_handlers;

// `#[derive(HasFields)]` on structs (this concept owns the derive's expansion):
// deriving only the field list, across named/tuple/generic/lifetime shapes.
pub mod struct_generic;
pub mod struct_generic_lifetime;
pub mod struct_single_named_field;
pub mod struct_single_unnamed_field;
pub mod struct_tuple_fields;
pub mod struct_two_named_fields;
