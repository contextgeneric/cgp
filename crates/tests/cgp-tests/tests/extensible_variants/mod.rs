//! One unit test per file. Each file is self-contained: it defines its own
//! enums, providers, and context types at module scope so that the type-level
//! wiring of one test never leaks into another.

// The `Sum!` type-level sum list that represents an enum's variants.
pub mod sum_macro;

// `#[derive(HasFields)]` snapshots for enums (this concept owns the enum
// expansion of the derive): the plain field list of an enum, and the generic
// variant.
pub mod has_fields_enum;
pub mod has_fields_enum_generic;

// `#[derive(CgpData)]` snapshots for enums (this concept owns the variant
// expansion of the derive): the full extractor/extractor-ref machinery for a
// concrete enum, a generic enum, and an enum whose variants carry struct
// payloads.
pub mod derive_cgp_data;
pub mod derive_cgp_data_generic;
pub mod derive_cgp_data_shape;

// Dispatching an extensible-variant input to per-variant handlers (the derives
// here are plain scaffolding — the dispatch combinators are owned elsewhere).
pub mod shape_dispatch;
pub mod shape_dispatch_ref;
pub mod variant_dispatch;
