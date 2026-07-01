# `#[derive(CgpVariant)]` — implementation

`#[derive(CgpVariant)]` is `#[derive(CgpData)]` restricted to enums: it emits the full extensible-variant machinery — the representation impls, the `FromVariant` constructors, and the incremental extractor — and refuses non-enum input. This document covers the entry point and its shared codegen; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_cgp_variant.md](../../reference/derives/derive_cgp_variant.md).

## Entry point

The macro is driven by the `derive_cgp_variant` function in [cgp-macro-lib/src/cgp_variant.rs](../../../crates/macros/cgp-macro-lib/src/cgp_variant.rs). It parses the input into an `ItemCgpVariant` and calls `to_items`:

```rust
let variant: ItemCgpVariant = parse2(body)?;
let items = variant.to_items()?;
```

`ItemCgpVariant::Parse` parses a `syn::ItemEnum`, so applying the derive to a struct or other item fails at parse time — the only behavioral difference from `#[derive(CgpData)]` on an enum, which reaches the same `to_items` through `ItemCgpData`.

## Pipeline and generated items

There is no multi-stage transform. `ItemCgpVariant::to_items` concatenates three slices in a fixed order — the five representation impls over a *sum* (`HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, `ToFieldsRef`), the per-variant `FromVariant` constructors, and the incremental extractor items (the `__Partial{Name}` and `__PartialRef{Name}` enums, `PartialData` for each, `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`, `FinalizeExtract` for each, then the per-variant `ExtractField` impls). These are exactly the outputs of the enum path of [`#[derive(HasFields)]`](derive_has_fields.md), [`#[derive(FromVariant)]`](derive_from_variant.md), and [`#[derive(ExtractField)]`](derive_extract_field.md) respectively; this document does not repeat their item shapes.

The variant corner cases are inherited from those building blocks: variant names are keyed by [`Symbol!`](../../reference/macros/symbol.md), generics are threaded onto every impl and onto the partial enums, and every variant must be a single-unnamed-field tuple variant or the extractor and `FromVariant` codegen fail (see [`derive_extract_field`](derive_extract_field.md)). The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpVariant` and its methods, and [`derive_cgp_data`](derive_cgp_data.md) covers the umbrella derive this is a restriction of.

## Tests

`#[derive(CgpVariant)]` has no snapshot macro of its own; its expansion is identical to the enum path of `#[derive(CgpData)]` and is pinned by the `snapshot_derive_cgp_data!` snapshots indexed in [derive_cgp_data.md's Snapshots section](derive_cgp_data.md#snapshots).

- The behavioral variant tests in [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/) — notably [shape_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/shape_dispatch.rs), [shape_dispatch_ref.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/shape_dispatch_ref.rs), and [variant_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/variant_dispatch.rs) — exercise the extractor and constructors this derive produces.

## Source

- Entry point: `derive_cgp_variant` in [cgp-macro-lib/src/cgp_variant.rs](../../../crates/macros/cgp-macro-lib/src/cgp_variant.rs).
- Codegen: `ItemCgpVariant::to_items` in [cgp-macro-core/src/types/cgp_data/variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs), which composes `derive_has_fields_impls_from_enum`, `derive_from_variant_from_enum`, and the [derive_extractor/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_extractor/) helpers; the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The runtime traits live in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/).
