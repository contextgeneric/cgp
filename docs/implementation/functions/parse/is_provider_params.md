# `parse_is_provider_params`

`parse_is_provider_params` converts a trait's generic parameters into the tuple of types that fills the `Params` position of an [`IsProviderFor`](../../../reference/traits/is_provider_for.md) bound. Every provider trait carries an `IsProviderFor<Component, Context, (Params)>` supertrait, and this function computes the `(Params)` part from the consumer trait's generics, so the marker records exactly the extra parameters a component takes beyond its context.

The transformation is a straightforward per-parameter mapping, but it normalizes the parameters into type form because the params tuple is a tuple of *types*. A type parameter passes through as itself: `T` becomes `T`. A lifetime parameter is lifted into a type through the `Life` wrapper, because a bare lifetime cannot appear as a tuple element: `'a` becomes `Life<'a>`. The parameters are first passed through `TypeGenerics` so that bounds and defaults are stripped and only the parameter names remain, then each is rendered with `parse_internal!`. The result is a `Punctuated<Type, Comma>` that the caller wraps in parentheses.

## Behavior and corner cases

Lifetimes are preserved in the params tuple even though they are dropped from the redirected lookup path. `parse_is_provider_params` emits `Life<'a>` for a lifetime, so `HasReference<'a, T>` yields the tuple `(Life<'a>, T)`; the separate `generic_params_to_path` helper used by the `RedirectLookup` impl keeps only type parameters, which is why a lifetime appears in `IsProviderFor` but not in the `ConcatPath` path. Holding both facts together is necessary to read the lifetime-component snapshot correctly.

## Known issues

A const generic parameter triggers a panic. The `GenericParam::Const` arm is `unimplemented!("const generic parameters are not yet supported in CGP traits")`, so any component-defining macro that reaches this function with a const parameter aborts expansion with a panic rather than returning a spanned `syn::Error`. The correct behavior would be a clean rejection or genuine const-parameter support. This is the root cause of the const-generic limitation recorded in [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md); the same panic affects every macro that builds a provider trait through this helper.

## Tests

The function is covered indirectly through the expansion snapshots that pin the `IsProviderFor` params tuple.

- The empty `()` case in [basic_delegation/component_macro.rs](../../../../crates/tests/cgp-tests/tests/basic_delegation/component_macro.rs).
- The `(Life<'a>, T)` case in [generic_components/component_lifetime.rs](../../../../crates/tests/cgp-tests/tests/generic_components/component_lifetime.rs).
- The const-generic panic has no test yet and is a candidate failure case for `cgp-macro-tests`.

## Source

- The function lives in [cgp-macro-core/src/functions/is_provider_params.rs](../../../../crates/macros/cgp-macro-core/src/functions/is_provider_params.rs).
- It is called by the provider-trait and blanket-impl builders in [cgp-macro-core/src/types/cgp_component/preprocessed/](../../../../crates/macros/cgp-macro-core/src/types/cgp_component/preprocessed/); the `Life` wrapper it emits is documented in [reference/types/life.md](../../../reference/types/life.md).
