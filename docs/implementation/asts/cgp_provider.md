# The `cgp_provider` AST stack

The `cgp_provider` stack is the sequence of AST types that both [`#[cgp_provider]`](../entrypoints/cgp_provider.md) and [`#[cgp_new_provider]`](../entrypoints/cgp_new_provider.md) parse into and lower through — the two macros share this stack entirely and differ only in whether the `new` keyword is set. The argument type `ProviderArgs` and a `syn::ItemImpl` become an `ItemCgpProvider`, whose single `lower` step derives the `IsProviderFor` impl (via `ItemProviderImpl`) and the provider struct (via `EmptyStruct`), packaging all three items into a `LoweredCgpProvider` that renders them. The argument-splitting helper `ProviderImplArgs` supports the `IsProviderFor` derivation. The [`#[cgp_provider]` entrypoint](../entrypoints/cgp_provider.md) covers what the stage produces; this document covers the types.

## `ProviderArgs`

`ProviderArgs` is the parsed attribute argument, shared by both macros: an optional `new` keyword and an optional component-type override.

```rust
pub struct ProviderArgs {
    pub new: Option<Keyword<New>>,
    pub component_type: Option<Type>,
}
```

`#[cgp_provider]` parses it as written; `#[cgp_new_provider]` parses it and then forces `new` to `Some`. When [`#[cgp_impl]`](cgp_impl.md#implargs) lowers to this stack, it constructs a `ProviderArgs` from its own `ImplArgs`, copying the `new` and `component_type` fields.

## `ItemCgpProvider`

`ItemCgpProvider` is the input stage — the args and the provider-trait impl. Its `lower` step drives the whole macro in one pass, delegating to three helpers on itself:

- `component_type` derives the component: it reads the provider trait's identifier and appends `Component` (so `AreaCalculator` → `AreaCalculatorComponent`), unless the attribute supplied an explicit override.
- `ItemProviderImpl::to_is_provider_for_impl` derives the `IsProviderFor` impl.
- `to_provider_struct` derives the provider struct, returning `None` when `new` is unset.

It packages the original `item_impl` (emitted verbatim), the derived `IsProviderFor` impl, and the optional struct into a `LoweredCgpProvider`.

## `ItemProviderImpl` and the `IsProviderFor` derivation

`ItemProviderImpl` pairs a component type with a provider impl and derives the `IsProviderFor` marker impl from it. `to_is_provider_for_impl` clones the provider impl, clears its body, associated types, attributes, `defaultness`, and `unsafety`, and swaps the trait for `IsProviderFor<Component, Context, (Params)>` — keeping the original generic parameters and `where` clause so the marker holds under exactly the same conditions:

```rust
// from  impl<Context, Code, Input> ComputerRef<Context, Code, Input> for FirstNameToString where …
// to    impl<Context, Code, Input> IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
//           for FirstNameToString where …  {}
```

The trait arguments come from `ProviderImplArgs::from_generic_args`, and the derivation ends by running [`replace_provider_in_generics`](../../../crates/macros/cgp-macro-core/src/visitors/replace_provider.rs) with a map from the provider identifier to the component type, which rewrites a `Provider: SomeTrait<Context, …>` `where`-bound into an `IsProviderFor<…>` bound so a higher-order provider's inner-provider dependency shows up as an `IsProviderFor` obligation.

## `ProviderImplArgs`

`ProviderImplArgs` splits a provider trait's generic arguments into the context type and the `Params` tuple. Walking the arguments in order, it takes the first *type* argument as the context and collects the rest as `Params`; a lifetime always goes into `Params` (its `ToTokens` lifts it to `Life<'a>`) regardless of position, and a `const` argument is rejected with a spanned error. A trait path with no type argument at all is an error, since there is no context to place in the leading position.

## `LoweredCgpProvider`

`LoweredCgpProvider` is the output stage — a bag of the three emitted items. Its `ToTokens` renders them in order: the provider impl, the `IsProviderFor` impl, then the provider struct (which renders to nothing when `None`).

## `EmptyStruct`

`EmptyStruct` is the provider struct, emitted only when `new` is set. `to_provider_struct` reads the shape from the impl's `Self` type: a plain name yields a unit `pub struct Name;`, while a generic provider yields a struct whose single `PhantomData` field binds every parameter — a lifetime parameter is bound as `Life<'a>` inside the `PhantomData` tuple so the struct stays covariant and `'static`-friendly.

```rust
// generic provider Self type SpawnAndRun<InCode>
pub struct SpawnAndRun<InCode>(pub ::core::marker::PhantomData<(InCode)>);
```

## Tests

- The stage transforms are exercised by the `snapshot_cgp_provider!` snapshots and the behavioral tests indexed in the [`#[cgp_provider]` entrypoint document](../entrypoints/cgp_provider.md); `#[cgp_new_provider]`'s direct coverage is indexed in [its entrypoint document](../entrypoints/cgp_new_provider.md).

## Source

- The stack lives in [cgp-macro-core/src/types/cgp_provider/](../../../crates/macros/cgp-macro-core/src/types/cgp_provider/): `ProviderArgs` in `args.rs`, `ItemCgpProvider` and its helpers in `item.rs`, `LoweredCgpProvider` in `lower.rs`, and `ProviderImplArgs` in `provider_impl_args.rs`.
- The `IsProviderFor` derivation (`ItemProviderImpl`) is in [cgp-macro-core/src/types/provider_impl.rs](../../../crates/macros/cgp-macro-core/src/types/provider_impl.rs), the provider struct (`EmptyStruct`) in [cgp-macro-core/src/types/empty_struct.rs](../../../crates/macros/cgp-macro-core/src/types/empty_struct.rs), and the provider-name rewrite in [cgp-macro-core/src/visitors/replace_provider.rs](../../../crates/macros/cgp-macro-core/src/visitors/replace_provider.rs).
- The consumer-style stack that hands off to this one is documented in [asts/cgp_impl.md](cgp_impl.md).
