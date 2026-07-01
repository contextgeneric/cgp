# The `cgp_type` AST stack

The `cgp_type` stack is thin: `#[cgp_type]` reuses the whole [`cgp_component` AST stack](cgp_component.md) to derive the component and adds a single wrapper type, `ItemCgpType`, that carries the finished `EvaluatedCgpComponent` and appends the two abstract-type provider impls. There is no bespoke argument type — the attribute is parsed as `CgpComponentArgs`, with the provider-name default patched in the [entrypoint function](../entrypoints/cgp_type.md) before the pipeline runs. This document covers `ItemCgpType` and the `ItemProviderImpl`/`ItemProviderImpls` helpers it renders through; the [entrypoint document](../entrypoints/cgp_type.md) covers what each item produces.

## `ItemCgpType`

`ItemCgpType` is the final rendering stage. It holds a single field, the `EvaluatedCgpComponent` produced by the shared `preprocess → eval` pipeline, and exists only to emit the extra impls on top of the standard component output. Its `to_items` first calls the wrapped component's own `to_items` (the five core items plus the `UseContext` and `RedirectLookup` provider impls) and then extends that vector with the abstract-type impls.

The extra impls are built by `to_item_provider_impls`, which reads the component's args, provider trait, and single associated type (via `extract_item_type_from_trait`, which also validates that the trait body is exactly one non-generic associated type) and produces two `ItemProviderImpl`s: the `UseType<Type>` impl and the `WithProvider<__Provider__>` impl. It clones the provider trait's generics for each, inserts the associated-type name (and `__Provider__` for the `WithProvider` case) as leading impl parameters, and moves any associated-type bound onto both impls with `Self::<Type>` rewritten to the free parameter. The shapes of the two impls are shown in the [entrypoint document](../entrypoints/cgp_type.md).

## `ItemProviderImpl` and `ItemProviderImpls`

`ItemProviderImpl` pairs one provider `syn::ItemImpl` with the component-marker type it implements, and its role in this stack is to attach the matching `IsProviderFor` impl: `to_item_impls` emits the provider impl and a companion `IsProviderFor<Component, Context, Params>` impl carrying the same generics and `where` clause, so the two can never disagree. `ItemProviderImpls` is just the collection of them, and its `to_item_impls` flattens the pairs into the `Vec<ItemImpl>` that `ItemCgpType::to_items` folds into the output. This pairing helper is shared with other macros that emit provider impls, so `#[cgp_type]` gets the `IsProviderFor` bookkeeping for free.

## Tests

- The stage is exercised end-to-end by the expansion snapshots indexed in the [entrypoint document's Snapshots section](../entrypoints/cgp_type.md); the trait-shape rejection in `extract_item_type_from_trait` has no dedicated `cgp-macro-tests` failure case yet.

## Source

- `ItemCgpType` and `extract_item_type_from_trait` live in [cgp-macro-core/src/types/cgp_type/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_type/item.rs).
- The `ItemProviderImpl`/`ItemProviderImpls` helpers are in [cgp-macro-core/src/types/provider_impl.rs](../../../crates/macros/cgp-macro-core/src/types/provider_impl.rs).
- The associated-type-bound rewriting is done by [`get_bounds_and_replace_self_assoc_type`](../../../crates/macros/cgp-macro-core/src/visitors/self_assoc_type.rs), and the shared component types are in [cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/), documented in [asts/cgp_component.md](cgp_component.md).
