# `#[derive_delegate]` — the AST stack

`#[derive_delegate(UseDelegate<Shape>)]` on a `#[cgp_component]` trait generates a dispatcher provider impl, so the component can be wired to a `UseDelegate` table that dispatches on a generic parameter. It is a modifier attribute collected by the component host; this page covers its AST types and the impl it builds, and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and expansion, read the reference document [reference/attributes/derive_delegate.md](../../../reference/attributes/derive_delegate.md).

## `DeriveDelegateAttribute`

The attribute parses into a `DeriveDelegateAttribute` — a `wrapper` identifier (`UseDelegate`) and its angle-bracketed key, held as a `Punctuated<Ident, Comma>` of `params`. The parser reads the wrapper identifier, a `<`, then either a single identifier or a parenthesized tuple of identifiers, then a `>`. An empty parenthesized tuple is rejected with a spanned "expect non-empty tuple list of identifiers in use_delegate_spec" error, so `UseDelegate<()>` cannot slip through as a keyless dispatcher.

## `DeriveDelegateAttributes`

`DeriveDelegateAttributes` is the thin collection wrapper — a `Vec<DeriveDelegateAttribute>` — that `CgpComponentAttributes` fills, one entry per `#[derive_delegate]` attribute on the trait. The host emits one dispatcher impl per entry alongside the component's standard provider impls, during `to_items`.

## `to_provider_impl` — the generated impl

`DeriveDelegateAttribute::to_provider_impl(provider_trait)` builds one impl of the provider trait for `Wrapper<__Components__>` that forwards each method to a delegate looked up through `DelegateComponent`. It clones the provider trait's own generics and appends two synthetic parameters — `__Components__` (the table type) and `__Delegate__` (the resolved delegate) — then adds two `where` bounds: the table lookup that resolves the key to a delegate, and the delegate's own provider-trait bound. Each trait method is forwarded through the shared [delegated-impl helpers](../../functions/derive/delegated_impls.md), so the dispatcher's bodies read as `<__Delegate__>::method(context, …)`:

```rust
impl<__Context__, __Components__, __Delegate__> AreaCalculator<__Context__>
    for UseDelegate<__Components__>
where
    __Components__: DelegateComponent<(Shape), Delegate = __Delegate__>,
    __Delegate__: AreaCalculator<__Context__>,
{ /* each method forwards to __Delegate__ */ }
```

The key in the `DelegateComponent<(…)>` lookup is the parenthesized `params` the attribute parsed, so a single-identifier key becomes `(Shape)` and a tuple key `(A, B)`. The impl keeps the component's own generics ahead of the two synthetic parameters, and reuses the provider trait's type generics (via `split_for_impl`) for both the delegate bound and the forwarded projections.

## Behavior and corner cases

**`#[derive_delegate]` is a legacy form for user code, but not dead code.** The `open` dispatch statement is preferred for new components, yet CGP's own error and handler families still *define* components with `#[derive_delegate]`, so the dispatcher impl this attribute generates remains in active use across the library.

**The synthetic parameters use the reserved double-underscore form.** `__Components__` and `__Delegate__` are constructed with `Span::call_site()` and the double-underscore convention so they cannot clash with a user's own type parameters or with the component's generics that precede them.

## Tests

- [dispatching/use_delegate_getter.rs](../../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) wires a component defined with `#[derive_delegate]` through a `UseDelegate` table.

The `UseDelegate` impl a `#[derive_delegate]` attribute adds to a bare component has no dedicated expansion snapshot yet; it is exercised through the error and handler families instead (noted as a missing snapshot in [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md)).

## Source

- The `derive_delegate/` submodule in [cgp-macro-core/src/types/attributes/derive_delegate/](../../../../crates/macros/cgp-macro-core/src/types/attributes/derive_delegate/): `attribute.rs` holds `DeriveDelegateAttribute`, its parser, and `to_provider_impl`; `attributes.rs` holds the `DeriveDelegateAttributes` collection.
- The forwarding method bodies come from the [delegated-impl helpers](../../functions/derive/delegated_impls.md).
- The host that drives it: [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md).
