# CGP Implementation Reference

This directory documents the *internals* of the CGP macros — how each macro is implemented in [crates/macros/cgp-macro-core](../../crates/macros/cgp-macro-core) and [crates/macros/cgp-macro-lib](../../crates/macros/cgp-macro-lib), including corner-case behavior, known limitations and bugs, and the test suite that exercises each construct. It is the documentation an agent reviewing or maintaining the macro source reads first: it records the current state of the code in one place so an agent can pick up a construct's implementation from where the last one left off. The authoring rules, document templates, and the synchronization rule that binds these documents to the code live in [CLAUDE.md](CLAUDE.md).

These documents complement the [construct reference](../reference/README.md) rather than repeating it. The reference explains what each construct does for a *user* — its accepted syntax and the code it expands to — and points only at library source. The implementation documents explain how the macro *produces* that behavior, and they are the sole home for every pointer into the [test suite](../../crates/tests): behavioral tests, failure cases, and macro-expansion snapshots all index from here. A reference document links to its implementation counterpart to elaborate a corner case; an implementation document links back to the reference for the user-facing semantics.

## Directory layout

The tree is organized by the kind of source construct each document describes, so an agent has an obvious place to start for "the macro entry point", "the AST type behind a stage", "a helper function", or "an internal macro". A new document goes in the matching subdirectory and registers itself in the catalog below in the same change.

The [entrypoints/](entrypoints/) directory holds one document per CGP macro — the top-level procedural macro a programmer invokes — describing its `cgp-macro-lib` entry function, the transform pipeline it drives, the items it emits, its corner cases, known issues, tests, and snapshots. The [asts/](asts/) directory holds one document per evaluation stack of AST constructs — the `cgp-macro-core` types implementing `Parse` or `ToTokens`, or serving as an intermediate representation — with the types of one pipeline grouped into a single document. The [functions/](functions/) directory holds the standalone helper functions, split into [functions/parse/](functions/parse/) for parsing helpers and [functions/derive/](functions/derive/) for code-synthesis helpers. The [macros/](macros/) directory holds the internal `macro_rules!` macros the implementation is written in, such as `parse_internal!` and `define_keyword!`.

## Catalog

The catalog is the index of implementation documents. The build-out is proceeding one construct at a time; this section registers what is documented and lists what is still pending, so the next agent can see both the shape of the finished tree and where to continue. When you add a document, move its entry from the pending list to the documented list in the same change.

### Entrypoints — [entrypoints/](entrypoints/)

Documented so far:

- [`#[cgp_component]`](entrypoints/cgp_component.md) — the foundational component-definition macro and its `preprocess → eval → to_items` pipeline.

Pending, one document each — the remaining `cgp-macro-lib` macros (`#[cgp_impl]`, `#[cgp_provider]`, `#[cgp_new_provider]`, `#[cgp_fn]`, `#[cgp_type]`, `#[cgp_getter]`, `#[cgp_auto_getter]`, `#[blanket_trait]`, `delegate_components!`, `check_components!`, `delegate_and_check_components!`, `#[cgp_namespace]`, `Symbol!`, `Product!`, `Sum!`, `Path!`, and the data derives `#[derive(HasField)]`, `HasFields`, `CgpData`, `CgpRecord`, `CgpVariant`, `BuildField`, `ExtractField`, `FromVariant`), the `cgp-extra-macro-lib` macros (`#[cgp_computer]`, `#[cgp_producer]`, `#[cgp_auto_dispatch]`), `#[async_trait]` from `cgp-async-macro`, and the `snapshot_*!` family from [cgp-macro-test-util](../../crates/macros/cgp-macro-test-util).

### AST stacks — [asts/](asts/)

Documented so far:

- [The `cgp_component` AST stack](asts/cgp_component.md) — `CgpComponentArgs`, `ItemCgpComponent`, `PreprocessedCgpComponent`, and `EvaluatedCgpComponent`.

Pending — one document per remaining evaluation stack, grouped by the macro that owns it: the `cgp_impl`, `cgp_provider`, `cgp_fn`, `cgp_type`, `cgp_getter`, `delegate_component`, `check_components`, `namespace`, `cgp_data`, `product`, and `sum` stacks, plus the shared building-block AST types (`attributes/`, `generics/`, `field/`, `getter/`, `implicits/`, `ident/`, `path/`, `keyword`).

### Functions — [functions/](functions/)

Documented so far:

- [Delegated-impl synthesis](functions/derive/delegated_impls.md) — `trait_items_to_delegated_impl_items` and `provider_trait_to_impl_items`, the forwarding-impl machinery.
- [`parse_is_provider_params`](functions/parse/is_provider_params.md) — building the `IsProviderFor` params tuple from trait generics.

Pending — the remaining helpers in `cgp-macro-core/src/functions`: identifier case conversion (`camel_case`/`snake_case`), generics merging, field/getter/implicit-argument parsing, and `strip`.

### Internal macros — [macros/](macros/)

Documented so far:

- [`parse_internal!`](macros/parse_internal.md) — build a `syn` node from quoted tokens with a descriptive parse error.
- [`define_keyword!`](macros/define_keyword.md) — declare a custom-keyword marker type implementing `IsKeyword`.

Pending — the `export_construct(s)!` family in `cgp-macro-core/src/macros` that backs the hygienic `exports.rs` markers.
