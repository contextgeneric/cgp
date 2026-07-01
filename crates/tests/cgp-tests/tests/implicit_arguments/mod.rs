//! One unit test per file, each self-contained at module scope.

// `#[cgp_fn]` expansion snapshots (this concept owns them): how `#[implicit]`
// arguments become `HasField` bounds and field reads.
pub mod cgp_fn_calling_fn;
pub mod cgp_fn_custom_trait_name;
pub mod cgp_fn_greet;
pub mod cgp_fn_multi_and_use_type;
pub mod cgp_fn_mutable;

// `#[implicit]` arguments inside `#[cgp_impl]` providers, and the implicit
// context parameter.
pub mod cgp_impl_implicit;
pub mod cgp_impl_implicit_generic;
pub mod implicit_context;
