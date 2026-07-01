# The `cgp_component` AST stack

The `cgp_component` stack is the sequence of AST types that `#[cgp_component]` parses into and transforms through: the argument types, then the three pipeline stages `ItemCgpComponent`, `PreprocessedCgpComponent`, and `EvaluatedCgpComponent`. Each stage is a plain struct holding what the next stage needs, and the data flows in one direction — args plus a `syn::ItemTrait` become `ItemCgpComponent`, which `preprocess`es, then `eval`s, then `to_items` renders to a `Vec<syn::Item>`. The [entrypoint document](../entrypoints/cgp_component.md) covers what each stage produces; this document covers the types.

## `CgpComponentRawArgs` and `CgpComponentArgs`

The attribute argument is parsed in two steps so that parsing and defaulting stay separate. `CgpComponentRawArgs` captures exactly what the user wrote as three `Option`s; `CgpComponentArgs` is the same three fields resolved to concrete values.

`CgpComponentRawArgs` accepts either a bare provider identifier or a comma-separated `key: value` list over the keys `name`, `context`, and `provider`, rejecting a duplicate or unknown key:

```rust
#[cgp_component(AreaCalculator)]                       // bare form
#[cgp_component { provider: AreaCalculator, context: Cx }]  // keyed form
```

`CgpComponentArgs` is produced from the raw form by a `TryFrom` that applies the defaults: `provider` is required, `context` defaults to `__Context__`, and `name` defaults to the provider identifier with a `Component` suffix. Its `Parse` impl just parses the raw form and runs that conversion, so the entry function can parse the attribute straight into the defaulted type.

## `ItemCgpComponent`

`ItemCgpComponent` is the raw input stage — the parsed args and trait before any CGP attributes are stripped. Its `preprocess` step splits the CGP modifier attributes off the trait and hands the cleaned trait, the args, and the parsed attributes to the next stage, so later stages see a plain `syn::ItemTrait` alongside a structured record of the attributes that modify the output.

## `PreprocessedCgpComponent`

`PreprocessedCgpComponent` owns the core derivation. It holds the args, the preprocessed trait, and the attributes, and its `eval` step derives the provider trait, the two blanket impls, and the component marker struct, packaging them into the final stage. The one structural point worth knowing is that the provider trait is built once and shared with its blanket impl, so the trait and the impl cannot disagree; the shapes they produce are described in the [entrypoint document](../entrypoints/cgp_component.md).

## `EvaluatedCgpComponent`

`EvaluatedCgpComponent` is the final stage — a bag of all the derived items plus the args and attributes needed to render the standard provider impls. Its `to_items` step emits the five core items in fixed order and then appends the provider impls: always the `UseContext` and `RedirectLookup` impls, plus one `UseDelegate` impl per `#[derive_delegate]` attribute and one prefix impl per `#[prefix]` attribute.

## Tests

- [cgp-macro-tests/tests/parser_rejections/cgp_component.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/cgp_component.rs) pins the argument/trait parser's rejection of a non-trait item.
- The stage transforms are exercised end-to-end by the expansion snapshots indexed in the [entrypoint document's Snapshots section](../entrypoints/cgp_component.md).

## Source

- The stack lives in [cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/): the argument types in `args/`, `ItemCgpComponent` in `item.rs`, `PreprocessedCgpComponent` in `preprocessed/`, and `EvaluatedCgpComponent` in `evaluated/`.
- The `self`/`Self` rewriting is done by the visitors in [cgp-macro-core/src/visitors/](../../../crates/macros/cgp-macro-core/src/visitors/).
