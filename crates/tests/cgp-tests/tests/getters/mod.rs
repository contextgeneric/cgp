//! One unit test per file. Each file is self-contained: it defines its own
//! getter traits, providers, and context types at module scope so that the
//! type-level wiring of one test never leaks into another.

// `#[cgp_auto_getter]` snapshots (this concept owns the macro's expansion): the
// distinct return-type shapes and trait shapes the auto getter supports.
pub mod assoc_type_auto_getter;
pub mod assoc_type_self_referential_auto;
pub mod auto_getter_generic;
pub mod clone_auto;
pub mod mref_auto;
pub mod non_self_auto;
pub mod option_auto;
pub mod slice_auto;
pub mod string_auto;

// `#[cgp_getter]` snapshots (this concept owns the macro's expansion): the full
// getter component, its return-type shapes, and its `provider`/`name` overrides.
pub mod assoc_type_getter;
pub mod assoc_type_self_referential;
pub mod clone;
pub mod mref;
pub mod non_self;
pub mod option;
pub mod slice;
pub mod string;
pub mod string_custom_name;
pub mod string_custom_spec;

// Getters that name an abstract type imported from another component, via either
// `#[extend(...)]` + `Self::Scalar` or `#[use_type(...)]` + bare `Scalar`.
pub mod abstract_type_extend;
pub mod abstract_type_use_type;
