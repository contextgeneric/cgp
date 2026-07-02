# `parse_is_provider_params`

`parse_is_provider_params` converts a trait's generic parameters into the tuple of types that fills the `Params` position of an [`IsProviderFor`](../../../reference/traits/is_provider_for.md) bound. Every provider trait carries an `IsProviderFor<Component, Context, (Params)>` supertrait, and this function computes the `(Params)` part from the consumer trait's generics, so the marker records exactly the extra parameters a component takes beyond its context.

The transformation is a straightforward per-parameter mapping over the trait's generics, emitting one type per parameter because the params tuple is a tuple of *types*. A type parameter passes through by name: `T` becomes `T`. A lifetime parameter is lifted into a type through the `Life` wrapper, because a bare lifetime cannot appear as a tuple element: `'a` becomes `Life<'a>`. Only the parameter's name matters, so any bounds or defaults on it are dropped. Each element is rendered with `parse_internal!`, and the result is a `Punctuated<Type, Comma>` that the caller wraps in parentheses.

## Behavior and corner cases

Lifetimes are preserved in the params tuple even though they are dropped from the redirected lookup path. `parse_is_provider_params` emits `Life<'a>` for a lifetime, so `HasReference<'a, T>` yields the tuple `(Life<'a>, T)`; the separate `generic_params_to_path` helper used by the `RedirectLookup` impl keeps only type parameters, which is why a lifetime appears in `IsProviderFor` but not in the `ConcatPath` path. Holding both facts together is necessary to read the lifetime-component snapshot correctly.

A const generic parameter is rejected with a spanned `syn::Error`. A const value cannot appear in the params tuple (which is a tuple of types) and CGP's type-based wiring cannot key on it, so the `GenericParam::Const` arm returns an error rather than emitting a tuple element. Because every macro that builds a provider trait routes through this helper, the rejection applies uniformly to `#[cgp_component]`, `#[cgp_type]`, and `#[cgp_getter]`; the user-facing consequence is recorded in [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md).

## Tests

The params tuple is pinned by the expansion snapshots, and the const-parameter rejection has its own failure case.

- The empty `()` case in [basic_delegation/component_macro.rs](../../../../crates/tests/cgp-tests/tests/basic_delegation/component_macro.rs).
- The `(Life<'a>, T)` case in [generic_components/component_lifetime.rs](../../../../crates/tests/cgp-tests/tests/generic_components/component_lifetime.rs).
- The const-parameter rejection in [cgp-macro-tests/tests/parser_rejections/cgp_component.rs](../../../../crates/tests/cgp-macro-tests/tests/parser_rejections/cgp_component.rs).

## Source

- The function lives in [cgp-macro-core/src/functions/is_provider_params.rs](../../../../crates/macros/cgp-macro-core/src/functions/is_provider_params.rs).
- It is called by the provider-trait and blanket-impl builders in [cgp-macro-core/src/types/cgp_component/preprocessed/](../../../../crates/macros/cgp-macro-core/src/types/cgp_component/preprocessed/); the `Life` wrapper it emits is documented in [reference/types/life.md](../../../reference/types/life.md).
