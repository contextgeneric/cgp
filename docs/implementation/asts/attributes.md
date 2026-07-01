# The attribute-modifier AST stack

The attribute modifiers — `#[uses]`, `#[use_type]`, `#[use_provider]`, `#[extend]`, `#[extend_where]`, `#[derive_delegate]`, and `#[default_impl]` — are not standalone macros. Each is an `#[…]` attribute that a host macro (`#[cgp_component]`, `#[cgp_impl]`, or `#[cgp_fn]`) strips off its input, parses into an AST type, and folds into the code it generates. This document covers each modifier's AST type, what it parses from, and what it injects into its host's output; for the user-facing syntax and expansion of each, read the reference documents under [reference/](../../reference/README.md) (in the `attributes/` subdirectory), and for how the hosts drive them see [entrypoints/cgp_component.md](../entrypoints/cgp_component.md), [entrypoints/cgp_impl.md](../entrypoints/cgp_impl.md), and [entrypoints/cgp_fn.md](../entrypoints/cgp_fn.md).

The modifiers do not parse themselves out of the token stream on their own; a host collects them first. `#[cgp_component]` gathers them into a `CgpComponentAttributes`, `#[cgp_impl]` into a `CgpImplAttributes`, and `#[cgp_fn]` into a `FunctionAttributes`. Each collector walks the item's attribute list, matches the leading identifier (`uses`, `use_type`, …), parses that attribute's arguments into the corresponding AST type, and passes any unrecognized attribute through untouched onto the generated code. Which modifiers a host accepts differs — `#[derive_delegate]` and `#[prefix]` are only meaningful on a component, `#[default_impl]` only on a provider impl — so a given modifier appears in only the collectors of the hosts that consume it.

## `#[uses]`

`#[uses(TraitA, TraitB<Param>)]` imports `Self` trait bounds, reading like a `use` statement. It parses into `UsesAttributes`, which holds a `Vec<PathWithTypeArgs>` — one path per imported trait, accepting only the plain `Trait<Params>` form and no associated-type equality. Its `to_type_param_bounds` turns each path directly into a `TypeParamBound`, and the host appends those bounds to the generated impl's `where` clause (on `Self`), where they become impl-side dependencies. On `#[cgp_fn]` the same parsing runs through `FunctionAttributes`; on `#[cgp_impl]` through `CgpImplAttributes`. The bounds land only on the impl, never on the consumer trait, which is what keeps the dependency hidden from callers.

## `#[use_type]`

`#[use_type(HasErrorType::Error)]` imports an abstract type: it rewrites the bare alias everywhere and adds the owning trait as a bound. It parses into a `UseTypeAttribute` per spec, collected into a `UseTypeAttributes`. Each spec captures a context type (defaulting to `Self`, or an explicit `@Context::` foreign context), the owning trait path, and one or more type idents (with optional `as` alias and `=` equality). Application is a two-phase transform. First, the `SubstituteAbstractType` visitor rewrites every bare, single-segment, argument-free use of the alias into the fully-qualified associated type:

```rust
// #[use_type(HasErrorType::Error)]  turns a bare `Error` into:
<Self as HasErrorType>::Error
```

Then the host adds the trait: on a `#[cgp_component]` trait, `transform_item_trait` pushes the trait path onto the consumer trait's supertraits (only for `Self`-context specs); on an impl, `transform_item_impl` derives the `where` predicates — `context_type: trait_path` — and extends the impl's `where` clause. The predicate derivation also resolves `as` aliases, `=` equalities, and cross-spec equalities, and rejects two specs sharing one alias. The visitor is applied in reverse spec order so earlier specs can shadow later ones. A `=` equality is rejected outright on a `#[cgp_component]` trait, since a component definition cannot pin an abstract type to a concrete one. The single-identifier-head parse of an `@Context::` prefix is deliberate: it keeps `IdentWithTypeArgs` rather than a greedy path parser, which would otherwise swallow the trailing `::Trait::Type`.

## `#[use_provider]`

`#[use_provider(Inner: AreaCalculator)]` completes an inner provider's bound for a higher-order provider. It parses into a `UseProviderAttribute` — a provider type, a colon, and a `+`-separated list of provider-trait paths — collected into a `UseProviderAttributes`. The one thing it does is finish each bound by inserting the context type as the leading generic argument, so the user's `: AreaCalculator` becomes `AreaCalculator<Self>`, and move the completed bound into the impl's `where` clause on the provider parameter:

```rust
// #[use_provider(Inner: AreaCalculator)]  becomes the where-predicate:
Inner: AreaCalculator<Self>
```

The context type is inserted at index 0 of the trait's angle-bracketed arguments, so a bound that already carries parameters keeps them after the context. On `#[cgp_impl]` the collection is `CgpImplAttributes`; on `#[cgp_fn]`, `FunctionAttributes`. There is no call-site rewriting — the body still calls the provider explicitly with the associated-function form.

## `#[extend]`

`#[extend(Trait)]` adds *supertrait* bounds to a generated trait. On `#[cgp_fn]` it is parsed into the `extend` field of `FunctionAttributes` — a `Vec<TypeParamBound>` — and its bounds are pushed onto both the generated trait's supertraits and the impl's `where` clause, because it is the only way to add a supertrait when a `#[cgp_fn]`'s `where` clauses are reserved for impl-side dependencies. On `#[cgp_component]` it is parsed by `CgpComponentAttributes` and its bounds are appended to the consumer trait's supertraits during `preprocess`, where it is the preferred way to add a non-type capability supertrait (an abstract-type supertrait should instead use `#[use_type]`, which adds the bound *and* rewrites the type).

## `#[extend_where]`

`#[extend_where(Bound)]` adds `where` predicates to a generated trait definition, and is `#[cgp_fn]`-only. It parses into the `extend_where` field of `FunctionAttributes` — a `Vec<WherePredicate>` — and its predicates are added to both the trait and the impl `where` clauses. It is the escape hatch for the bounds `#[uses]` cannot spell, chiefly associated-type-equality constraints.

## `#[derive_delegate]`

`#[derive_delegate(UseDelegate<Shape>)]` (on a `#[cgp_component]` trait) generates a dispatcher provider impl so the component can be wired to a `UseDelegate` table. It parses into a `DeriveDelegateAttribute` — a wrapper identifier (`UseDelegate`) and its angle-bracketed key, which is either a single identifier or a non-empty parenthesized tuple — collected into a `DeriveDelegateAttributes`. Its `to_provider_impl` builds one impl of the provider trait for `Wrapper<__Components__>` that forwards each method to a delegate looked up through `DelegateComponent`. The impl carries two synthetic generics and two `where` bounds — the table lookup and the delegate's provider-trait bound — and forwards each trait method through the shared delegated-impl helpers:

```rust
impl<__Context__, __Components__, __Delegate__> AreaCalculator<__Context__>
    for UseDelegate<__Components__>
where
    __Components__: DelegateComponent<(Shape), Delegate = __Delegate__>,
    __Delegate__: AreaCalculator<__Context__>,
{ /* each method forwards to __Delegate__ */ }
```

The host (`#[cgp_component]`) collects it in `CgpComponentAttributes` and emits one such impl per `#[derive_delegate]` attribute alongside the component's standard provider impls. It is a legacy form for user code — `open` dispatch is preferred — but CGP's own error and handler families still define components with it.

## `#[default_impl]`

`#[default_impl(@test.ShowImplComponent.u32 in ExtendedNamespace)]` (on a `#[cgp_impl]` provider) registers the provider as a namespace's default for one path. It parses into a `DefaultImplAttribute` — a key type (a path or type), the `in` keyword, and the namespace path — collected into a `DefaultImplAttributes`. Its `to_item_impl` emits one impl of the namespace's lookup trait, keyed on the given path type, whose `Delegate` associated type is the provider being defined:

```rust
// #[default_impl(@test.ShowImplComponent.u32 in ExtendedNamespace)] on provider ShowU32:
impl<__Components__> ExtendedNamespace<__Components__>
for PathCons<Symbol!("test"), PathCons<ShowImplComponent, PathCons<u32, Nil>>>
{
    type Delegate = ShowU32;
}
```

The namespace path gains a trailing `__Components__` type argument and the impl generics gain a matching `__Components__` parameter, so the default is generic over any table the namespace is queried through. The host (`#[cgp_impl]`) collects it in `CgpImplAttributes` and emits one such impl per attribute after the provider impl, using the provider's own generics and provider type.

## Tests

The behavioral and snapshot tests that exercise each modifier are listed per attribute below; test and snapshot pointers for a construct live only in these implementation documents.

- **`#[uses]`** — [impl_side_dependencies/fn_uses.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses.rs) pins the `#[cgp_fn]` form and [impl_side_dependencies/impl_uses.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/impl_uses.rs) the `#[cgp_impl]` form; [generic_components/fn_impl_generics.rs](../../../crates/tests/cgp-tests/tests/generic_components/fn_impl_generics.rs) exercises it alongside generic parameters.
- **`#[use_type]`** — [abstract_types/use_type_component.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_component.rs) covers the `#[cgp_component]` supertrait form; [abstract_types/use_type_fn_alias.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_alias.rs), [use_type_fn_equality.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality.rs), and [use_type_fn_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign.rs) cover the alias, equality, and foreign-context (`@`) forms; [use_type_fn_equality_cross_trait.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality_cross_trait.rs) and [use_type_fn_foreign_equality_cross_trait.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality_cross_trait.rs) cover cross-spec equality; [use_type_generic_param.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_generic_param.rs) covers a generic-parameter abstract type.
- **`#[use_provider]`** — [higher_order_providers/use_provider_fn.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_fn.rs) pins the `#[cgp_fn]` form and [higher_order_providers/use_provider_impl.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_impl.rs) the `#[cgp_impl]` form; [higher_order_providers/scaled_area.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/scaled_area.rs) wires a full higher-order provider through it.
- **`#[extend]`** — [impl_side_dependencies/fn_extend.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_extend.rs) pins the `#[cgp_fn]` supertrait form; [abstract_types/extend_component.rs](../../../crates/tests/cgp-tests/tests/abstract_types/extend_component.rs) and [abstract_types/use_type_fn_extend.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_extend.rs) exercise it on a component and alongside `#[use_type]`; [getters/abstract_type_extend.rs](../../../crates/tests/cgp-tests/tests/getters/abstract_type_extend.rs) uses it with a getter.
- **`#[extend_where]`** — [abstract_types/use_type_fn_nested_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_nested_foreign.rs) exercises it alongside `#[use_type]` on a `#[cgp_fn]`.
- **`#[derive_delegate]`** — [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) wires a component defined with `#[derive_delegate]` through a `UseDelegate` table.
- **`#[default_impl]`** — [namespaces/default_impls.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls.rs) pins the emitted namespace-default impl (`snapshot_cgp_impl!`), and [namespaces/default_impls_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls_wiring.rs) checks a context picks up the default.

## Source

- The modifiers live in [cgp-macro-core/src/types/attributes/](../../../crates/macros/cgp-macro-core/src/types/attributes/): `uses.rs` (`UsesAttributes`), the `use_type/` submodule (`UseTypeAttribute`, per-type entries in `ident.rs`, the two-phase transform in `attributes.rs`, and predicate derivation in `type_predicates.rs`), the `use_provider/` submodule (`UseProviderAttribute` and its bound completion), the `derive_delegate/` submodule (`DeriveDelegateAttribute::to_provider_impl`), and the `default_impl/` submodule (`DefaultImplAttribute::to_item_impl`).
- `#[extend]`/`#[extend_where]` are fields of `FunctionAttributes` in `function.rs`.
- The host collectors are `CgpComponentAttributes` in `cgp_component_attributes.rs`, `CgpImplAttributes` in `cgp_impl_attributes.rs`, and `FunctionAttributes` in `function.rs`.
- The abstract-type substitution is the `SubstituteAbstractType` visitor in [cgp-macro-core/src/visitors/substitute_abstract_type.rs](../../../crates/macros/cgp-macro-core/src/visitors/substitute_abstract_type.rs), and the `#[derive_delegate]` forwarding bodies come from the [delegated-impl helpers](../functions/derive/delegated_impls.md).
