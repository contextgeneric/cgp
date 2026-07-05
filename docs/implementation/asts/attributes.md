# The attribute-modifier AST stack

The attribute modifiers — `#[uses]`, `#[use_type]`, `#[use_provider]`, `#[extend]`, `#[extend_where]`, `#[derive_delegate]`, and `#[default_impl]` — are not standalone macros. Each is an `#[…]` attribute that a host macro (`#[cgp_component]`, `#[cgp_impl]`, or `#[cgp_fn]`) strips off its input, parses into an AST type, and folds into the code it generates. This document covers each modifier's AST type, what it parses from, and what it injects into its host's output; for the user-facing syntax and expansion of each, read the reference documents under [reference/](../../reference/README.md) (in the `attributes/` subdirectory), and for how the hosts drive them see [entrypoints/cgp_component.md](../entrypoints/cgp_component.md), [entrypoints/cgp_impl.md](../entrypoints/cgp_impl.md), and [entrypoints/cgp_fn.md](../entrypoints/cgp_fn.md).

The modifiers do not parse themselves out of the token stream on their own; a host collects them first. `#[cgp_component]` gathers them into a `CgpComponentAttributes`, `#[cgp_impl]` into a `CgpImplAttributes`, and `#[cgp_fn]` into a `FunctionAttributes`. Each collector walks the item's attribute list, matches the leading identifier (`uses`, `use_type`, …), parses that attribute's arguments into the corresponding AST type, and passes any unrecognized attribute through untouched onto the generated code. Which modifiers a host accepts differs — `#[derive_delegate]` and `#[prefix]` are only meaningful on a component, `#[default_impl]` only on a provider impl — so a given modifier appears in only the collectors of the hosts that consume it.

## `#[uses]`

`#[uses(TraitA, TraitB<Param>)]` imports `Self` trait bounds, reading like a `use` statement. It parses into `UsesAttributes`, which holds a `Vec<TypeParamBound>` — one bound per imported capability. Each entry is a full `syn::TypeParamBound`, so any bound a `where` clause accepts is allowed, including an associated-type-equality binding (`HasErrorType<Error = AppError>`), a higher-ranked bound, or a lifetime bound; the plain `Trait<Params>` form is the idiomatic one. Its `to_type_param_bounds` collects the bounds, and the host appends them to the generated impl's `where` clause (on `Self`), where they become impl-side dependencies. On `#[cgp_fn]` the bounds are parsed straight into `FunctionAttributes` (a `Vec<TypeParamBound>` field mirroring `#[extend]`); on `#[cgp_impl]` into the `UsesAttributes` held by `CgpImplAttributes`. The bounds land only on the impl, never on the consumer trait, which is what keeps the dependency hidden from callers.

## `#[use_type]`

`#[use_type(HasErrorType.Error)]` imports an abstract type: it rewrites the bare alias everywhere and adds the owning trait as a bound. It parses into a `UseTypeAttribute` per spec, collected into a `UseTypeAttributes`. Each spec captures a context type (defaulting to `Self`, or an explicit `@Context.` foreign context), the owning trait path, and one or more type idents (with optional `as` alias and `=` equality). Both the context and the trait parse as a `PathWithTypeArgs`, and a `.` (not `::`) separates the context, trait, and associated type — so the trait may be a full path or carry generic arguments (`HasFooType<X>.Foo`) with its own `::` staying inside the path, while the `.` unambiguously marks where the associated type begins. Application is a two-phase transform. First, the `SubstituteAbstractType` visitor rewrites every bare, single-segment, argument-free use of the alias into the fully-qualified associated type:

```rust
// #[use_type(HasErrorType.Error)]  turns a bare `Error` into:
<Self as HasErrorType>::Error
```

Then the host adds the trait: on a `#[cgp_component]` trait, `transform_item_trait` pushes the trait path onto the consumer trait's supertraits (only for `Self`-context specs); on an impl, `transform_item_impl` derives the `where` predicates — `context_type: trait_path` — and extends the impl's `where` clause. Both transforms first call `forbid_duplicate_aliases`, which rejects any two imports resolving to the same identifier or alias, comparing every pair across all specs and within a single braced list, so the check applies uniformly to components, impls, and functions. The predicate derivation additionally resolves `as` aliases, `=` equalities, and cross-spec equalities. The visitor is applied in reverse spec order so an earlier spec's substitution can rewrite an identifier a later spec's substitution introduced — this is what makes a nested foreign import such as `HasTypes.Types, @Types.HasScalarType.Scalar` compose into `<<Self as HasTypes>::Types as HasScalarType>::Scalar`. A `=` equality is rejected outright on a `#[cgp_component]` trait, since a component definition cannot pin an abstract type to a concrete one.

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

`#[extend_where(Bound)]` adds `where` predicates to a generated trait definition, and is `#[cgp_fn]`-only. It parses into the `extend_where` field of `FunctionAttributes` — a `Vec<WherePredicate>` — and its predicates are added to both the trait and the impl `where` clauses. Where `#[uses]` adds a bound to `Self` on the impl alone, `#[extend_where]` is the way to make a bound part of the generated *trait*, and it takes a full predicate — a bound on any type, not only `Self`, including associated-type-equality constraints.

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

**The provider's `where` clause is deliberately dropped from this impl.** `to_item_impl` receives the provider impl's generics *after* `#[implicit]`/`#[uses]`/`#[use_type]`/`#[use_provider]` have pushed their `Self`-keyed impl-side bounds into it — a provider with `#[use_type(HasErrorType.Error)]`, for instance, arrives carrying `where Self: HasErrorType`. Those bounds belong on the provider's own impl and its `IsProviderFor`, never on this registration impl, whose only job is `type Delegate = Provider`. The registration impl's `Self` is the path key (`PathCons<..>`), so a retained `Self: HasErrorType` would demand `PathCons<..>: HasErrorType` — a bound that never holds — and silently break every context that joins the namespace. `to_item_impl` therefore clears `generics.where_clause` before splitting, keeping only the parameters that name the key and provider plus the `__Components__` table. (A provider whose *type* is generic, and whose parameter appears only in the `Delegate` associated type, would leave that parameter unconstrained, so a per-component default is written for a concrete provider.)

## Tests

The behavioral and snapshot tests that exercise each modifier are listed per attribute below; test and snapshot pointers for a construct live only in these implementation documents.

- **`#[uses]`** — [impl_side_dependencies/fn_uses.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses.rs) pins the `#[cgp_fn]` form and [impl_side_dependencies/impl_uses.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/impl_uses.rs) the `#[cgp_impl]` form; [generic_components/fn_impl_generics.rs](../../../crates/tests/cgp-tests/tests/generic_components/fn_impl_generics.rs) exercises it alongside generic parameters. The associated-type-equality bound (`HasErrorType<Error = ...>`) that `#[uses]` also accepts is pinned on the `#[cgp_fn]` form in [impl_side_dependencies/fn_uses_associated_type.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses_associated_type.rs) and exercised end-to-end on the `#[cgp_impl]` form in [impl_side_dependencies/impl_uses_associated_type.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/impl_uses_associated_type.rs).
- **`#[use_type]`** — [abstract_types/use_type_component.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_component.rs) covers the `#[cgp_component]` supertrait form and [use_type_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_foreign.rs) the `@` foreign form on a component; [abstract_types/use_type_fn_alias.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_alias.rs), [use_type_fn_equality.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality.rs), and [use_type_fn_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign.rs) cover the alias, equality, and foreign-context (`@`) forms; [use_type_fn_equality_cross_trait.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality_cross_trait.rs), [use_type_fn_foreign_equality.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality.rs), and [use_type_fn_foreign_equality_cross_trait.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality_cross_trait.rs) cover cross-spec and nested-foreign equality; [use_type_generic_param.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_generic_param.rs) covers an alias that collides with a generic parameter, [use_type_path_qualified.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_path_qualified.rs) the path-qualified trait form the `.` separator enables, and [implicit_arguments/cgp_fn_multi_and_use_type.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_multi_and_use_type.rs) the generic-argument trait form (`HasFooType<X>.Foo`). Rejections are pinned in [parser_rejections/use_type.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/use_type.rs): a `=` equality on a component, and a duplicate identifier or alias across specs, within one braced list, and on a component.
- **`#[use_provider]`** — [higher_order_providers/use_provider_fn.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_fn.rs) pins the `#[cgp_fn]` form and [higher_order_providers/use_provider_impl.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_impl.rs) the `#[cgp_impl]` form; [higher_order_providers/scaled_area.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/scaled_area.rs) wires a full higher-order provider through it.
- **`#[extend]`** — [impl_side_dependencies/fn_extend.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_extend.rs) pins the `#[cgp_fn]` supertrait form; [abstract_types/extend_component.rs](../../../crates/tests/cgp-tests/tests/abstract_types/extend_component.rs) and [abstract_types/use_type_fn_extend.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_extend.rs) exercise it on a component and alongside `#[use_type]`; [getters/abstract_type_extend.rs](../../../crates/tests/cgp-tests/tests/getters/abstract_type_extend.rs) uses it with a getter.
- **`#[extend_where]`** — [abstract_types/use_type_fn_nested_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_nested_foreign.rs) exercises it alongside `#[use_type]` on a `#[cgp_fn]`.
- **`#[derive_delegate]`** — [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) wires a component defined with `#[derive_delegate]` through a `UseDelegate` table.
- **`#[default_impl]`** — [namespaces/default_impls.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls.rs) pins the emitted namespace-default impl (`snapshot_cgp_impl!`), and [namespaces/default_impls_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls_wiring.rs) checks a context picks up the default. [namespaces/default_impl_use_type.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impl_use_type.rs) pins that the registration impl carries no `where` clause when the provider has a `#[use_type]` dependency, and resolves such a provider through a context that joins the namespace; the cross-crate orphan restriction on a prefixed component's default is pinned in [acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs).

## Source

- The modifiers live in [cgp-macro-core/src/types/attributes/](../../../crates/macros/cgp-macro-core/src/types/attributes/): `uses.rs` (`UsesAttributes`), the `use_type/` submodule (`UseTypeAttribute`, per-type entries in `ident.rs`, the two-phase transform in `attributes.rs`, and predicate derivation in `type_predicates.rs`), the `use_provider/` submodule (`UseProviderAttribute` and its bound completion), the `derive_delegate/` submodule (`DeriveDelegateAttribute::to_provider_impl`), and the `default_impl/` submodule (`DefaultImplAttribute::to_item_impl`).
- `#[extend]`/`#[extend_where]` are fields of `FunctionAttributes` in `function.rs`.
- The host collectors are `CgpComponentAttributes` in `cgp_component_attributes.rs`, `CgpImplAttributes` in `cgp_impl_attributes.rs`, and `FunctionAttributes` in `function.rs`.
- The abstract-type substitution is the `SubstituteAbstractType` visitor in [cgp-macro-core/src/visitors/substitute_abstract_type.rs](../../../crates/macros/cgp-macro-core/src/visitors/substitute_abstract_type.rs), and the `#[derive_delegate]` forwarding bodies come from the [delegated-impl helpers](../functions/derive/delegated_impls.md).
