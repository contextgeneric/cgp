//! One unit test per file. Each file is self-contained: it defines its own
//! components, providers, and context types at module scope so that the
//! type-level wiring of one test never leaks into another.

// Generic-parameter variants of `#[cgp_fn]` (this concept owns their snapshots):
// a function generic over a type parameter, and one whose impl generics are
// declared with `#[impl_generics(...)]`.
pub mod fn_generic_param;
pub mod fn_impl_generics;

// Generic-parameter variants of `#[cgp_component]` (this concept owns their
// snapshots): a component with a lifetime and type parameter, and components
// carrying const generics (a plain const and a const of an abstract type).
pub mod component_const;
pub mod component_generic_const;
pub mod component_lifetime;
