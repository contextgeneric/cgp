//! One unit test per file. Each file is self-contained: it defines its own
//! traits, providers, and context types at module scope so that the type-level
//! wiring of one test never leaks into another.
//!
//! The exception is `types`, a small shared fixture (the `Foo`/`Bar`/`FooBar`
//! enum) that the shape tests dispatch over. It is declared here and referenced
//! by siblings via `super::types`.

// Shared fixture: the `Foo`/`Bar`/`FooBar` enum the shape tests route over.
pub mod types;

// Method-shape coverage: one shape per file. Each defines per-variant impls and
// dispatches them over an extensible-data enum.
pub mod generics;
pub mod multi_args;
pub mod multi_args_owned_self;
pub mod multi_args_ref;
pub mod multi_methods;
pub mod self_mut_only;
pub mod self_only;
pub mod self_ref_only;
pub mod self_ref_return_explicit_ref;
pub mod self_ref_return_implicit_ref;
pub mod shape;

// Where the trait lives and what it declares: a trait used across modules, and a
// method with a default body.
pub mod cross_module;
pub mod default_method;

// Combined with `#[async_trait]`: the async shapes.
pub mod async_generics;
pub mod async_multi_args;
pub mod async_multi_args_owned_self;
pub mod async_multi_args_ref;
pub mod async_self_mut_only;
pub mod async_self_only;
pub mod async_self_ref_only;

// The names and items the macro generates: the per-variant computer wired by
// name, imported consumer traits that would make an unqualified matcher call
// ambiguous, a module item sharing a method's name, a method named with a raw
// identifier, and a module without the prelude, where only qualified paths resolve.
pub mod computer_by_name;
pub mod consumer_traits_in_scope;
pub mod method_name_in_scope;
pub mod raw_method_name;
pub mod without_prelude;
