//! One unit test per file. Each file is self-contained: it defines its own
//! traits, providers, and context types at module scope so that the type-level
//! wiring of one test never leaks into another.
//!
//! The exception is `types`, a small shared fixture (the `Foo`/`Bar`/`FooBar`
//! enum) that the `#[cgp_auto_dispatch]` shape tests dispatch over. It is
//! declared here and referenced by siblings via `super::types`.

// Shared fixture: the `Foo`/`Bar`/`FooBar` enum the auto-dispatch tests route over.
pub mod types;

// `#[cgp_auto_dispatch]` shape coverage: one method-shape per file. Each defines
// per-variant impls and dispatches them over an extensible-data enum. Two files cover
// the generated names instead: a module item sharing a method's name, and a method
// named with a raw identifier.
pub mod auto_dispatch_generics;
pub mod auto_dispatch_method_name_in_scope;
pub mod auto_dispatch_multi_args;
pub mod auto_dispatch_multi_args_owned_self;
pub mod auto_dispatch_multi_args_ref;
pub mod auto_dispatch_multi_methods;
pub mod auto_dispatch_raw_method_name;
pub mod auto_dispatch_self_mut_only;
pub mod auto_dispatch_self_only;
pub mod auto_dispatch_self_ref_only;
pub mod auto_dispatch_self_ref_return_explicit_ref;
pub mod auto_dispatch_self_ref_return_implicit_ref;
pub mod auto_dispatch_shape;

// `#[cgp_auto_dispatch]` combined with `#[async_trait]` — the async shapes.
pub mod auto_dispatch_async_generics;
pub mod auto_dispatch_async_multi_args;
pub mod auto_dispatch_async_multi_args_owned_self;
pub mod auto_dispatch_async_multi_args_ref;
pub mod auto_dispatch_async_self_mut_only;
pub mod auto_dispatch_async_self_only;
pub mod auto_dispatch_async_self_ref_only;

// `#[cgp_auto_dispatch]` beside imported `CanCompute`/`CanComputeAsync`.
pub mod auto_dispatch_consumer_traits_in_scope;

// The `UseDelegate` dispatch provider and the `UseDelegate`-table form of
// `delegate_components!` (this concept owns those snapshots).
pub mod use_delegate_getter;

// Composing handler/computer providers.
pub mod compose;

// Table structs declared by `delegate_components!` that carry a `const` parameter:
// a const-generic `new` target and a nested inner table with a const generic list.
pub mod const_generic_tables;
