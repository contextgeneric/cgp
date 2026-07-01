# CGP Implementation Reference

This directory documents the *internals* of the CGP macros — how each macro is implemented in [crates/macros/cgp-macro-core](../../crates/macros/cgp-macro-core) and [crates/macros/cgp-macro-lib](../../crates/macros/cgp-macro-lib), including corner-case behavior, known limitations and bugs, and the test suite that exercises each construct. It is the documentation an agent reviewing or maintaining the macro source reads first: it records the current state of the code in one place so an agent can pick up a construct's implementation from where the last one left off. The authoring rules, document templates, and the synchronization rule that binds these documents to the code live in [CLAUDE.md](CLAUDE.md).

These documents complement the [construct reference](../reference/README.md) rather than repeating it. The reference explains what each construct does for a *user* — its accepted syntax and the code it expands to — and points only at library source. The implementation documents explain how the macro *produces* that behavior, and they are the sole home for every pointer into the [test suite](../../crates/tests): behavioral tests, failure cases, and macro-expansion snapshots all index from here. A reference document links to its implementation counterpart to elaborate a corner case; an implementation document links back to the reference for the user-facing semantics.

## Directory layout

The tree is organized by the kind of source construct each document describes, so an agent has an obvious place to start for "the macro entry point", "the AST type behind a stage", "a helper function", or "an internal macro". A new document goes in the matching subdirectory and registers itself in the catalog below in the same change.

The [entrypoints/](entrypoints/) directory holds one document per CGP macro — the top-level procedural macro a programmer invokes — describing its `cgp-macro-lib` entry function, the transform pipeline it drives, the items it emits, its corner cases, known issues, tests, and snapshots. The [asts/](asts/) directory holds one document per evaluation stack of AST constructs — the `cgp-macro-core` types implementing `Parse` or `ToTokens`, or serving as an intermediate representation — with the types of one pipeline grouped into a single document. The [functions/](functions/) directory holds the standalone helper functions, split into [functions/parse/](functions/parse/) for parsing helpers and [functions/derive/](functions/derive/) for code-synthesis helpers. The [macros/](macros/) directory holds the internal `macro_rules!` macros the implementation is written in, such as `parse_internal!` and `define_keyword!`.

## Catalog

This section is the index of implementation documents. When you add a document, register it here in the same change.

### Entrypoints — [entrypoints/](entrypoints/)

The component and provider macros:

- [`#[cgp_component]`](entrypoints/cgp_component.md) — the foundational component-definition macro and its `preprocess → eval → to_items` pipeline.
- [`#[cgp_impl]`](entrypoints/cgp_impl.md) — lowers consumer-style syntax into a provider impl and hands it to `#[cgp_provider]`.
- [`#[cgp_provider]`](entrypoints/cgp_provider.md) — passes a provider-trait impl through and derives its `IsProviderFor` impl.
- [`#[cgp_new_provider]`](entrypoints/cgp_new_provider.md) — `#[cgp_provider]` with the provider struct also declared.

Functions and getters:

- [`#[cgp_fn]`](entrypoints/cgp_fn.md) — a single-implementation capability as a blanket-impl trait, with `#[implicit]` argument lowering.
- [`#[cgp_getter]`](entrypoints/cgp_getter.md) — a getter component wired through CGP, adding `UseField`/`UseFields` provider impls.
- [`#[cgp_auto_getter]`](entrypoints/cgp_auto_getter.md) — a getter as a blanket impl over `HasField`.

Abstract types and blanket traits:

- [`#[cgp_type]`](entrypoints/cgp_type.md) — an abstract-type component, reusing the `#[cgp_component]` pipeline and adding `UseType`.
- [`#[blanket_trait]`](entrypoints/blanket_trait.md) — a blanket impl generated from a trait with default methods.

Wiring and checking:

- [`delegate_components!`](entrypoints/delegate_components.md) — the context wiring table and its mapping/statement grammar.
- [`check_components!`](entrypoints/check_components.md) — compile-time wiring assertions.
- [`delegate_and_check_components!`](entrypoints/delegate_and_check_components.md) — wire and check in one macro.
- [`cgp_namespace!`](entrypoints/cgp_namespace.md) — reusable, inheritable wiring tables via `RedirectLookup`.

Type-level construction macros:

- [`Symbol!`](entrypoints/symbol.md), [`Product!`](entrypoints/product.md), [`Sum!`](entrypoints/sum.md), [`Path!`](entrypoints/path.md) — the type-level string, list, sum, and path macros.

Data derives:

- [`#[derive(HasField)]`](entrypoints/derive_has_field.md), [`#[derive(HasFields)]`](entrypoints/derive_has_fields.md) — field-access derives.
- [`#[derive(CgpData)]`](entrypoints/derive_cgp_data.md), [`#[derive(CgpRecord)]`](entrypoints/derive_cgp_record.md), [`#[derive(CgpVariant)]`](entrypoints/derive_cgp_variant.md) — the extensible-data derives.
- [`#[derive(BuildField)]`](entrypoints/derive_build_field.md), [`#[derive(ExtractField)]`](entrypoints/derive_extract_field.md), [`#[derive(FromVariant)]`](entrypoints/derive_from_variant.md) — builder/extractor/variant support.

Handlers and other extra macros:

- [`#[cgp_computer]`](entrypoints/cgp_computer.md), [`#[cgp_producer]`](entrypoints/cgp_producer.md) — define `Computer`/`Producer` providers from functions.
- [`#[cgp_auto_dispatch]`](entrypoints/cgp_auto_dispatch.md) — generate a dispatching handler.
- [`#[async_trait]`](entrypoints/async_trait.md) — rewrite trait `async fn` to `-> impl Future`.
- [The `snapshot_*!` family](entrypoints/snapshot_macros.md) — the `cgp-macro-test-util` macros that pin macro expansions as `insta` snapshots.

### AST stacks — [asts/](asts/)

One document per evaluation stack, grouped by the macro that owns it:

- [cgp_component](asts/cgp_component.md), [cgp_impl](asts/cgp_impl.md), [cgp_provider](asts/cgp_provider.md), [cgp_type](asts/cgp_type.md), [cgp_fn](asts/cgp_fn.md), [cgp_getter](asts/cgp_getter.md), [blanket_trait](asts/blanket_trait.md).
- [delegate_component](asts/delegate_component.md), [check_components](asts/check_components.md), [namespace](asts/namespace.md).
- [cgp_data](asts/cgp_data.md) — the shared extensible-data derive stack.
- [product](asts/product.md), [sum](asts/sum.md), [path](asts/path.md), [symbol](asts/symbol.md) — the type-level construction stacks.
- [attributes](asts/attributes.md) — the modifier-attribute AST types (`#[uses]`, `#[use_type]`, `#[use_provider]`, `#[extend]`, `#[extend_where]`, `#[derive_delegate]`, `#[default_impl]`).

### Functions — [functions/](functions/)

The cross-cutting helper functions; construct-specific parse/derive helpers are documented inside the owning macro's entrypoint or AST document.

- [Delegated-impl synthesis](functions/derive/delegated_impls.md) — the forwarding-impl machinery shared by the component impls.
- [`parse_is_provider_params`](functions/parse/is_provider_params.md) — building the `IsProviderFor` params tuple from trait generics.
- [`merge_generics`](functions/derive/generics.md) — combining two `Generics` into one.
- [Identifier case conversion](functions/derive/idents.md) — the PascalCase/snake_case/reserved-name helpers.

### Internal macros — [macros/](macros/)

- [`parse_internal!`](macros/parse_internal.md) — build a `syn` node from quoted tokens with a descriptive parse error.
- [`define_keyword!`](macros/define_keyword.md) — declare a custom-keyword marker type implementing `IsKeyword`.
- [`export_construct!` / `export_constructs!`](macros/export_constructs.md) — declare the hygienic markers backing `exports.rs`.
