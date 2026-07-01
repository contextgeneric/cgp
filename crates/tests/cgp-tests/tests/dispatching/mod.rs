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
// per-variant impls and dispatches them over an extensible-data enum.
pub mod auto_dispatch_generics;
pub mod auto_dispatch_multi_args;
pub mod auto_dispatch_multi_args_owned_self;
pub mod auto_dispatch_multi_args_ref;
pub mod auto_dispatch_multi_methods;
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

// The `UseDelegate` dispatch provider and the `UseDelegate`-table form of
// `delegate_components!` (this concept owns those snapshots).
pub mod use_delegate_getter;

// Composing handler/computer providers.
pub mod compose;
