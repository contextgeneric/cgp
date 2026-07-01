# `#[cgp_type]` — implementation

`#[cgp_type]` builds an abstract-type component by running the `#[cgp_component]` pipeline over a trait carrying one associated type, then appending the two provider impls (`UseType` and `WithProvider`) that let a context choose the concrete type through wiring. This document covers how that works internally; for the accepted syntax and the full expansion, read the reference document [reference/macros/cgp_type.md](../../reference/macros/cgp_type.md).

## Entry point

The macro is driven by the `cgp_type` function in [cgp-macro-lib/src/cgp_type.rs](../../../crates/macros/cgp-macro-lib/src/cgp_type.rs). It follows the canonical entry-point shape but with one preparatory step unique to this macro: after parsing the attribute into `CgpComponentRawArgs` and the item into a `syn::ItemTrait`, it extracts the trait's single associated type and, when the user gave no provider name, defaults `provider_ident` to `{Type}TypeProvider` — keyed off the *associated type's* identifier, not the trait's. It then feeds the args and trait into the shared `#[cgp_component]` pipeline and wraps the result in an `ItemCgpType` for the extra codegen.

```rust
if raw_args.provider_ident.is_none() {
    raw_args.provider_ident = Some(Ident::new(
        &format!("{}TypeProvider", item_type.ident),
        item_type.ident.span(),
    ));
}

let evaluated = item_cgp_component.preprocess()?.eval()?;
let item_cgp_type = ItemCgpType { item_component: evaluated };
let items = item_cgp_type.to_items()?;
```

Three failures surface here: a malformed attribute is rejected while parsing `CgpComponentRawArgs`; a non-trait item fails at `syn::parse2::<ItemTrait>`; and a trait whose body is not exactly one plain (non-generic, `where`-free) associated type is rejected by `extract_item_type_from_trait`.

## Pipeline

`#[cgp_type]` reuses the `#[cgp_component]` pipeline for the component itself and adds a final rendering step of its own. The two shared stages, `preprocess` and `eval`, are exactly the ones documented for the [`cgp_component` entrypoint](cgp_component.md) and are owned by the [`cgp_component` AST stack](../asts/cgp_component.md); this macro does not re-implement them.

- **preprocess → eval** run the standard `#[cgp_component]` derivation over the associated-type trait, producing an `EvaluatedCgpComponent` — the consumer trait, provider trait, both blanket impls, and the component marker.
- **to_items** is where `#[cgp_type]` diverges: `ItemCgpType::to_items` first calls the evaluated component's own `to_items` (emitting the five core items plus the `UseContext` and `RedirectLookup` provider impls), then appends the `UseType` and `WithProvider` provider impls. The [`cgp_type` AST stack](../asts/cgp_type.md) documents `ItemCgpType`.

## Generated items

The macro emits the entire `#[cgp_component]` output for the trait — described in the [`cgp_component` entrypoint document](cgp_component.md), except that every blanket impl forwards an *associated type* rather than a method body — followed by two abstract-type provider impls. Each of the two extra impls is paired with a matching `IsProviderFor` impl carrying the same bounds, produced through the shared [`ItemProviderImpl`/`ItemProviderImpls`](../asts/cgp_type.md) machinery.

The first extra impl is the `UseType` blanket impl, the heart of the macro. It implements the provider trait for `UseType<Type>` by setting the abstract associated type to the free generic parameter, so wiring a component to `UseType<f64>` supplies `f64` as the type with no bespoke provider:

```rust
impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar> {
    type Scalar = Scalar;
}
```

The second is a `WithProvider` bridge that adapts the built-in `TypeProvider` machinery into this component, so a `#[cgp_type]` component can be backed by any generic `TypeProvider`:

```rust
impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
    for WithProvider<__Provider__>
where
    __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
{
    type Scalar = Scalar;
}
```

## Behavior and corner cases

A **bound on the associated type** is threaded not only into the provider trait (that comes for free from the shared component pipeline) but also onto both extra impls. `to_item_provider_impls` reads the bound with [`get_bounds_and_replace_self_assoc_type`](../../../crates/macros/cgp-macro-core/src/visitors/self_assoc_type.rs), which rewrites any `Self::Scalar` inside the bound to the free `Scalar` parameter, then adds it as a `where Scalar: <bound>` predicate on both the `UseType` and `WithProvider` impls. A self-referential bound such as `type Scalar: Mul<Output = Self::Scalar> + Clone` therefore emits `where Scalar: Mul<Output = Scalar> + Clone`.

**Generic parameters** on the trait are handled entirely by the shared component pipeline and then reused: `to_item_provider_impls` clones the provider trait's generics and inserts the associated-type name (and, for `WithProvider`, `__Provider__`) as leading impl parameters, so a `?Sized` or otherwise-bounded parameter carries through onto the extra impls unchanged.

The **provider-name default** is the one behavior `#[cgp_type]` adds ahead of the pipeline: an omitted provider name becomes `{Type}TypeProvider` (from the associated type), where a bare `#[cgp_component]` would instead reject a missing name. A supplied name — `#[cgp_type(ProvideFooType)]` — overrides this exactly as it does for `#[cgp_component]`.

## Snapshots

Every `snapshot_cgp_type!` invocation across the suite is indexed here; the canonical variants live in the `abstract_types` target, with one `UseDelegate`-focused variant owned by the dispatch target:

- [abstract_types/cgp_type_macro.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_macro.rs) — the canonical plain expansion (one associated type, default `ScalarTypeProvider` name), showing the full component output plus the `UseType` and `WithProvider` impls.
- [abstract_types/cgp_type_bounded.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_bounded.rs) — the associated type bounded by another abstract-type component (`type Types: HasScalarType`), with the bound propagating onto the `UseType` and `WithProvider` impls.
- [abstract_types/cgp_type_self_referential.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_self_referential.rs) — a self-referential bound (`type Scalar: Mul<Output = Self::Scalar> + Clone`), where `Self::Scalar` is rewritten to the free parameter in the extra impls' `where` clause.
- [abstract_types/cgp_type_unsized.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_unsized.rs) — a `#[cgp_type(ProvideFooType)]` overriding the default name together with a `?Sized` generic parameter threaded through every generated item.
- [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) — a multi-parameter `#[cgp_type]` with `#[derive_delegate]` attributes, pinning the `UseDelegate` dispatch-provider impls; owned by the dispatch concept rather than repeated in the abstract-types target.

No snapshot yet captures the keyed argument form (`#[cgp_type { provider: …, context: … }]`) distinct from the bare-name override.

## Tests

The behavioral tests confirm the generated wiring and the `UseType` route work:

- [abstract_types/cgp_type_bounded.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_bounded.rs) wires a context whose abstract type is itself an abstract-type component and checks the bound is enforced.
- [abstract_types/cgp_type_self_referential.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_self_referential.rs) wires a self-referentially bounded type through `delegate_components!` and passes its check.
- [abstract_types/cgp_type_unsized.rs](../../../crates/tests/cgp-tests/tests/abstract_types/cgp_type_unsized.rs) exercises a `?Sized` abstract type together with a dependent getter.

## Source

- Entry point: `cgp_type` in [cgp-macro-lib/src/cgp_type.rs](../../../crates/macros/cgp-macro-lib/src/cgp_type.rs).
- Extra codegen: [cgp-macro-core/src/types/cgp_type/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_type/item.rs), documented in [asts/cgp_type.md](../asts/cgp_type.md).
- Shared component pipeline it wraps: [cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/), documented in [entrypoints/cgp_component.md](cgp_component.md).
- Associated-type-bound rewriting: [`get_bounds_and_replace_self_assoc_type`](../../../crates/macros/cgp-macro-core/src/visitors/self_assoc_type.rs).
- Paired provider/`IsProviderFor` impls: `ItemProviderImpl` in [cgp-macro-core/src/types/provider_impl.rs](../../../crates/macros/cgp-macro-core/src/types/provider_impl.rs).
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
