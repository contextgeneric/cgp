//! One unit test per file. Each file is self-contained: it defines its own
//! components, providers, and context types at module scope so that the
//! type-level wiring of one test never leaks into another.

// The macro-expansion snapshots that this concept owns: the canonical
// `#[cgp_component]`, `#[cgp_impl]`, and `delegate_components!` output.
pub mod component_macro;
pub mod delegate_array_key;
pub mod delegate_components_macro;
pub mod delegate_generic_table;
pub mod provider_macro;

// Behavioral wiring: a consumer trait becomes usable once its component is wired.
pub mod consumer_delegate_generic;
pub mod consumer_delegate_getter;
pub mod default_methods;
pub mod impl_self;

// `delegate_components!` shape variants (compile-time checks only).
pub mod delegate_generic_nested_value;
pub mod delegate_nested_use_delegate;
pub mod delegate_new_array_key;
pub mod delegate_new_generic_struct;
pub mod delegate_new_struct;
