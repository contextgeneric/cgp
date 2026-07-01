# Delegated-impl synthesis

The delegated-impl functions turn a trait's items into the body of an impl that forwards every method, associated type, and constant to a chosen delegate type. They are the shared machinery behind the forwarding impls that `#[cgp_component]` and its relatives emit — the consumer blanket impl, the provider blanket impl, the `UseContext` impl, and the `RedirectLookup` impl all route their calls through the same delegate type by way of these functions, so the forwarding shape is written once and reused everywhere.

The core idea is a single delegate type supplied by the caller: given a trait and a `delegate_type` expression, each generated impl item calls the delegate's version of that trait item. A method `fn area(context: &Context) -> f64` becomes `fn area(context: &Context) -> f64 { <delegate>::area(context) }`; an associated type becomes `type Output = <delegate as Trait>::Output;`; an associated const becomes `const N: usize = <delegate as Trait>::N;`. Which concrete type fills `<delegate>` is the caller's choice, and that is what distinguishes the four impls that use these functions.

## `trait_items_to_delegated_impl_items`

`trait_items_to_delegated_impl_items` maps a slice of `syn::TraitItem` to the `Vec<syn::ImplItem>` that forwards each one to the delegate, and is the entry point most callers use. It takes the trait items, the `delegate_type` to forward to, and the `provider_trait_path` used to qualify associated-type and const projections, and applies `trait_item_to_delegated_impl_items` to each item.

`trait_item_to_delegated_impl_items` handles the three supported item kinds and rejects the rest. A **function** is forwarded by `signature_to_delegated_impl_item_fn`, which builds a method body that calls the delegate with the same arguments. An **associated type** becomes `type Name<generics> = <delegate as provider_trait_path>::Name<generics>;`, qualified through the provider trait path so the projection is unambiguous. An **associated const** becomes `const Name<generics> = <delegate as provider_trait_path>::Name<generics>;`, copying the const's type and attributes. Any other trait item — a macro invocation, for example — produces a `syn::Error` reading "unsupported trait item", so the macro fails cleanly rather than dropping the item silently.

## `provider_trait_to_impl_items`

`provider_trait_to_impl_items` is the convenience wrapper for the common case of forwarding a *provider* trait's own items to a delegate. It takes the provider `ItemTrait` and a `delegate_type`, reconstructs the provider trait path from the trait's identifier and its generics (via `split_for_impl`), and calls `trait_items_to_delegated_impl_items` with that path. The provider blanket impl and the `RedirectLookup` impl use it because both forward the provider trait to a `<… as DelegateComponent<…>>::Delegate` type; the consumer blanket impl and the `UseContext` impl call `trait_items_to_delegated_impl_items` directly instead, because they forward *across* traits (consumer to provider, or provider to consumer) and so must supply a different trait path than the trait being implemented.

## Behavior and corner cases

The forwarding qualifies associated types and consts through the supplied trait path, not through the impl's own trait, which matters when the source and target traits differ. When the consumer blanket impl forwards to the provider trait, an associated type is projected as `<__Context__ as ProviderTrait>::Output`, because the value lives on the provider side; passing the wrong path here would produce an impl that does not type-check. Method forwarding, by contrast, needs no trait qualification — it calls the delegate's inherent-looking associated function `<delegate>::method(args)` and relies on the delegate's trait bound in the impl's `where` clause to resolve it.

## Tests

These functions have no dedicated unit test; they are covered through the `#[cgp_component]` expansion snapshots, which pin the forwarding bodies of all four impls.

- The plain case in [basic_delegation/component_macro.rs](../../../../crates/tests/cgp-tests/tests/basic_delegation/component_macro.rs) shows method forwarding through `<__Provider__ as DelegateComponent<…>>::Delegate::foo(…)` and through `UseContext`/`RedirectLookup`.
- The default-method case in [basic_delegation/default_methods.rs](../../../../crates/tests/cgp-tests/tests/basic_delegation/default_methods.rs) confirms a forwarded call resolves to a default body.
- Associated-type and const forwarding are pinned by the `#[cgp_type]` snapshots in the `abstract_types` target.

## Source

- The functions live in [cgp-macro-core/src/functions/delegated_impls/](../../../../crates/macros/cgp-macro-core/src/functions/delegated_impls/): `trait_items.rs` holds `trait_items_to_delegated_impl_items` and the per-item dispatch, `provider_trait.rs` holds `provider_trait_to_impl_items`, `signature.rs` holds the method-forwarding builder `signature_to_delegated_impl_item_fn`, and `item_type.rs` holds `trait_to_impl_item_type`.
- The callers are documented in [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md) and the [cgp_component AST stack](../../asts/cgp_component.md).
