//! One unit test per file. Each file is self-contained: it defines its own
//! structs, enums, and contexts at module scope so that the type-level items of
//! one test never leak into another.

// The type-equality assertions the tests below share.
pub mod assertions;

// `Struct!`: the named and tuple forms, the encoding rules, and the parity with
// `#[derive(HasFields)]` on the same body.
pub mod struct_delimiter_agnostic;
pub mod struct_derive_parity;
pub mod struct_hygiene;
pub mod struct_in_signatures;
pub mod struct_inequality;
pub mod struct_named;
pub mod struct_round_trip;
pub mod struct_through_macro_rules;
pub mod struct_tuple;

// `Enum!`: the variant shapes, their equivalences with `Struct!` payloads, and
// the parity with `#[derive(HasFields)]` on the same body.
pub mod enum_basic;
pub mod enum_derive_parity;
pub mod enum_round_trip;
pub mod enum_variant_shapes;
pub mod enum_with_variant_derives;

// A shape used as an `open` dispatch key in `delegate_components!`.
pub mod open_dispatch_key;
