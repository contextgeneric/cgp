# The `cgp_component` AST stack

The `cgp_component` stack is the sequence of AST types that `#[cgp_component]` parses into and transforms through — the argument types, then the three pipeline stages `ItemCgpComponent`, `PreprocessedCgpComponent`, and `EvaluatedCgpComponent`. Each stage is a plain struct holding the data the next stage needs, and each transform is a method on one stage returning the next. This document describes the types; the [entrypoint document](../entrypoints/cgp_component.md) describes how the macro drives them, and the [reference document](../../reference/macros/cgp_component.md) describes the user-facing result.

The stack flows in one direction: `CgpComponentArgs` and a `syn::ItemTrait` combine into `ItemCgpComponent`, which `preprocess`es into `PreprocessedCgpComponent`, which `eval`s into `EvaluatedCgpComponent`, which finally `to_items` renders into a `Vec<syn::Item>`. Only the argument types implement `Parse`; the stage types are constructed programmatically and carry the transform methods rather than a `ToTokens` impl.

## `CgpComponentRawArgs` and `CgpComponentArgs`

`CgpComponentArgs` is the parsed, defaulted form of the attribute argument, and `CgpComponentRawArgs` is the intermediate that captures exactly what the user wrote before defaults are applied. The split exists so parsing and defaulting are separate concerns: `CgpComponentRawArgs` holds three `Option`s (`context_ident`, `provider_ident`, `component_name`), and `CgpComponentArgs` holds the same three fields resolved to concrete values.

`CgpComponentRawArgs::parse` (in `args/raw.rs`) accepts the two attribute forms. When the input is a single token followed by end-of-input (`peek2(End)`), it is the bare-identifier form and that token is the `provider_ident`. Otherwise it parses a comma-separated sequence of `key: value` pairs, matching the key string against `name`, `context`, and `provider`, rejecting a duplicate key with "duplicate key is not allowed" and an unknown key with "unknown key {key}". The `name` value parses as an [`IdentWithTypeGenerics`](../asts/cgp_component.md) so a component name may carry generic parameters, while `context` and `provider` parse as bare `Ident`s.

`CgpComponentArgs` is produced from the raw form through `TryFrom` (in `args/component_args.rs`), which applies the defaults the reference documents describe: `provider_ident` is required and errors with "`provider_ident` key must be given" when absent, `context_ident` defaults to `Ident::new("__Context__", …)`, and `component_name` defaults to the provider identifier with a `Component` suffix. `CgpComponentArgs` implements `Parse` by parsing a `CgpComponentRawArgs` and delegating to this `TryFrom`, so the entry function can `syn::parse2` the attribute directly into the defaulted form.

## `ItemCgpComponent`

`ItemCgpComponent` is the raw input stage — the parsed attribute and the parsed trait, before any CGP attributes are stripped. It holds `args: CgpComponentArgs` and `item_trait: syn::ItemTrait`, and the entry function constructs it directly from the two `syn::parse2` results.

Its one method, `preprocess`, calls `CgpComponentAttributes::preprocess` on the trait to split the CGP modifier attributes off the plain trait, then returns a `PreprocessedCgpComponent` carrying the cloned args, the cleaned trait, and the parsed attributes. It generates no code; it only normalizes the trait so later stages see a plain `syn::ItemTrait` and a separate, structured record of the attributes that modify the output.

## `PreprocessedCgpComponent`

`PreprocessedCgpComponent` is the stage that owns the core derivation. It holds `args`, the preprocessed `item_trait`, and `attributes: CgpComponentAttributes`, and it carries the methods that build the provider trait, the two blanket impls, and the component struct.

Its `eval` method orchestrates the derivation: it calls `to_component_struct` for the `EmptyStruct` marker, `to_provider_trait_and_blanket_impl` for the provider trait paired with its blanket impl, and `to_consumer_item_impl` for the consumer blanket impl, then packages all of these with the original consumer trait, the attributes, and the args into an `EvaluatedCgpComponent`. The individual builders are `to_provider_trait` (the provider trait, with `self`/`Self` rewritten and supertraits lowered to context `where` predicates), `to_provider_trait_and_blanket_impl` (which reuses `to_provider_trait` and adds the `__Provider__` blanket impl), `to_consumer_item_impl` (the `__Context__` consumer impl), and `to_component_struct` (the marker). The [entrypoint document's Generated items section](../entrypoints/cgp_component.md) describes precisely what each builder emits; the key structural point is that the provider trait is built once and shared, so the trait and its blanket impl cannot disagree.

## `EvaluatedCgpComponent`

`EvaluatedCgpComponent` is the final stage — a bag of all the derived items plus the context needed to render the standard provider impls. Its fields are the `component_struct` (an `EmptyStruct`), the `consumer_trait`, `consumer_impl`, `provider_trait`, and `provider_impl`, plus the `args` and `attributes` carried through for the provider-impl builders.

Its `to_items` method fixes the emission order — consumer trait, consumer impl, provider trait, provider impl, component struct — then appends the provider impls from `to_item_impls`. `to_item_impls` gathers the provider impls (`to_provider_impls`) and the prefix impls (`to_prefix_impls`). `to_provider_impls` always emits the `UseContext` impl (`to_use_context_impl`) and the `RedirectLookup` impl (`to_redirect_lookup_impl`), then one `UseDelegate` impl per `#[derive_delegate]` attribute (`to_use_delegate_impls`); `to_prefix_impls` emits one namespace impl per `#[prefix]` attribute. The provider impls are collected as `ItemProviderImpl`s (each pairing the impl with its component type) and rendered through `ItemProviderImpls::to_item_impls`.

## Tests

The argument parser's rejection of a non-trait item is pinned in [cgp-macro-tests/tests/parser_rejections/cgp_component.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/cgp_component.rs). The stage transforms are exercised end-to-end through the expansion snapshots indexed in the [entrypoint document's Snapshots section](../entrypoints/cgp_component.md) — the plain, supertrait/default-method, lifetime, and namespace variants each pin the output of the full `preprocess → eval → to_items` sequence.

## Source

The stack lives in [cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/): the argument types in `args/`, `ItemCgpComponent` in `item.rs`, `PreprocessedCgpComponent` and its builders in `preprocessed/`, and `EvaluatedCgpComponent` and the provider-impl builders in `evaluated/`. The `EmptyStruct`, `ItemProviderImpl`, and `CgpComponentAttributes` helper types live in sibling `types/` modules; the visitors that rewrite `self`/`Self` live in [cgp-macro-core/src/visitors/](../../../crates/macros/cgp-macro-core/src/visitors/).
