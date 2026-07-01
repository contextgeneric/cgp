# `#[cgp_provider]` — implementation

`#[cgp_provider]` takes a provider-trait impl written directly on a provider struct, passes it through unchanged, and derives the matching [`IsProviderFor`](../../reference/traits/is_provider_for.md) impl from the same `where` clause so the provider's dependencies can never drift out of sync. This document covers how that derivation works internally; for the accepted syntax and the full expansion, read the reference document [reference/macros/cgp_provider.md](../../reference/macros/cgp_provider.md).

## Entry point

The macro is driven by the `cgp_provider` function in [cgp-macro-lib/src/cgp_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_provider.rs). It parses the item into a `syn::ItemImpl` and the attribute into a `ProviderArgs` (an optional `new` keyword and an optional component-type override), then lowers the impl and emits the result:

```rust
let item = ItemCgpProvider { args, item_impl };
let lowered = item.lower()?;
```

The sibling [`#[cgp_new_provider]`](cgp_new_provider.md) shares this whole stack: its `cgp_new_provider` entry parses the same `ProviderArgs`, forces `args.new = Some(...)`, and then runs the identical `ItemCgpProvider::lower`. The two macros differ only in whether the provider struct is declared. A malformed attribute is rejected while parsing `ProviderArgs`, and a non-`impl` item fails at `syn::parse2::<ItemImpl>`.

## Pipeline

The macro has a single lowering stage, `ItemCgpProvider::lower`, which produces a `LoweredCgpProvider` bundling three items; the [`cgp_provider` AST stack](../asts/cgp_provider.md) documents those types in full. Within that one stage the work splits three ways:

- deriving the **component type** — defaulting to the provider trait's name with a `Component` suffix, or using the attribute's explicit override;
- deriving the **`IsProviderFor` impl** — cloning the provider impl, stripping its body and associated types, and swapping the trait for `IsProviderFor<Component, Context, Params>`;
- deriving the **provider struct** — emitted only when `new` is set.

## Generated items

The macro emits the provider impl (verbatim), the derived `IsProviderFor` impl, and — for `#[cgp_new_provider]` or `#[cgp_impl(new …)]` — the provider struct, in that order. The one piece of real synthesis is the `IsProviderFor` impl.

Its trait arguments are split out of the provider trait's own arguments: the first is the component type, the second is the provider trait's **leading** type argument (the context), and the third is a tuple of every **remaining** type parameter. A lifetime among the remaining arguments is lifted into `Life<'a>` in that tuple. So a multi-parameter provider trait yields a grouped `Params` tuple:

```rust
// provider impl
impl<Context, Code, Input> ComputerRef<Context, Code, Input> for FirstNameToString
where Context: HasField<Symbol!("first_name"), Value: Display> { /* … */ }

// derived alongside it
impl<Context, Code, Input>
    IsProviderFor<ComputerRefComponent, Context, (Code, Input)> for FirstNameToString
where Context: HasField<Symbol!("first_name"), Value: Display> {}
```

The derived impl keeps the original generic parameters and `where` clause verbatim, so it holds under exactly the conditions the provider impl holds. Its body and associated types are cleared, and its `defaultness`/`unsafety` are dropped.

## Behavior and corner cases

The **component type defaults** from the provider trait's identifier: `component_type()` reads the trait path, takes its identifier, and appends `Component`, so implementing `AreaCalculator` targets `AreaCalculatorComponent`. The optional attribute argument overrides this with an explicit type — the only thing the attribute changes.

The **context is the first type argument** of the provider trait, and it must be present. `ProviderImplArgs::from_generic_args` walks the trait's arguments in order, takes the first *type* argument as the context, and collects the rest into the `Params` tuple; lifetimes always go into the tuple (lifted to `Life<'a>`) regardless of position. A provider trait path with no type argument at all is rejected with a spanned error.

The **`new` keyword controls the struct**, whose shape is read from the impl's `Self` type by `to_provider_struct`. A plain provider name yields a unit `pub struct Name;`. A generic provider yields a struct with a `PhantomData` field binding its parameters — a lifetime parameter is bound via `Life<'a>` inside the `PhantomData` tuple:

```rust
// #[cgp_new_provider] impl<Context, Code, InCode> Runner<Context, Code> for SpawnAndRun<InCode>
pub struct SpawnAndRun<InCode>(pub ::core::marker::PhantomData<(InCode)>);
```

The **provider-name rewrite in the generics** keeps the `IsProviderFor` bounds honest: `replace_provider_in_generics` rewrites any `Provider: SomeTrait<Context, …>` bound in the `where` clause into an `IsProviderFor<SomeComponent, Context, (…)>` bound, so a higher-order provider's inner-provider dependency surfaces in error messages as an `IsProviderFor` obligation rather than a raw provider-trait bound.

## Known issues

A **const argument in the provider trait's arguments** is rejected with a spanned error rather than supported: `ProviderImplArgs::from_generic_args` returns "const arguments are not supported in provider impl trait arguments" if a `const` appears among the trait's type arguments. A const generic on the *provider struct* is fine — it flows through untouched — so this limits only const parameters that sit in the provider trait's own argument list.

## Tests

`#[cgp_provider]` is exercised both by expansion snapshots (see [`#[cgp_component]`](cgp_component.md#snapshots) for the `snapshot_cgp_provider!` index, which lives with the component feature) and by direct behavioral use:

- [generic_components/component_const.rs](../../../crates/tests/cgp-tests/tests/generic_components/component_const.rs) snapshots a const-generic provider `UseConstant<CONSTANT>`, with the const on the struct flowing through to the `IsProviderFor` impl unchanged.
- [generic_components/component_generic_const.rs](../../../crates/tests/cgp-tests/tests/generic_components/component_generic_const.rs) snapshots the same const-generic provider carrying an impl-side dependency that ties the const's type to the context's abstract `Unit`.
- [generic_components/component_lifetime.rs](../../../crates/tests/cgp-tests/tests/generic_components/component_lifetime.rs) snapshots a lifetime-carrying `UseField` provider, showing the lifetime lifted into `Life<'a>` in the `Params` tuple.
- [dispatching/compose.rs](../../../crates/tests/cgp-tests/tests/dispatching/compose.rs) and [async_and_send/spawn.rs](../../../crates/tests/cgp-tests/tests/async_and_send/spawn.rs) exercise `#[cgp_provider]` and `#[cgp_new_provider]` directly in real wiring.

## Source

- Entry points: `cgp_provider` in [cgp-macro-lib/src/cgp_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_provider.rs) and `cgp_new_provider` in [cgp-macro-lib/src/cgp_new_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_new_provider.rs).
- Shared lowering and its AST types: [cgp-macro-core/src/types/cgp_provider/](../../../crates/macros/cgp-macro-core/src/types/cgp_provider/), documented in [asts/cgp_provider.md](../asts/cgp_provider.md).
- `IsProviderFor` derivation: [cgp-macro-core/src/types/provider_impl.rs](../../../crates/macros/cgp-macro-core/src/types/provider_impl.rs).
- Provider-name-to-`IsProviderFor` rewrite: the [`replace_provider` visitor](../../../crates/macros/cgp-macro-core/src/visitors/replace_provider.rs).
- This macro is the handoff target of [`#[cgp_impl]`](cgp_impl.md).
