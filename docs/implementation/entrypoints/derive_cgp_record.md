# `#[derive(CgpRecord)]` — implementation

`#[derive(CgpRecord)]` is `#[derive(CgpData)]` restricted to structs: it emits the full extensible-record machinery — the per-field getters, the representation impls, and the incremental builder — and refuses non-struct input. This document covers the entry point and its shared codegen; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_cgp_record.md](../../reference/derives/derive_cgp_record.md).

## Entry point

The macro is driven by the `derive_cgp_record` function in [cgp-macro-lib/src/cgp_record.rs](../../../crates/macros/cgp-macro-lib/src/cgp_record.rs). It parses the input into an `ItemCgpRecord` and calls `to_items`:

```rust
let record: ItemCgpRecord = parse2(body)?;
let items = record.to_items()?;
```

`ItemCgpRecord::Parse` parses a `syn::ItemStruct`, so applying the derive to an enum or other item fails at parse time — this is the only behavioral difference from `#[derive(CgpData)]` on a struct, which reaches the same `to_items` through `ItemCgpData`.

## Pipeline and generated items

There is no multi-stage transform. `ItemCgpRecord::to_items` concatenates three slices in a fixed order — the per-field `HasField`/`HasFieldMut` getters, the five representation impls (`HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, `ToFieldsRef`), and the incremental builder items (`__Partial{Name}`, `HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, then per-field `UpdateField` and `HasField`). These are exactly the outputs of [`#[derive(HasField)]`](derive_has_field.md), [`#[derive(HasFields)]`](derive_has_fields.md), and [`#[derive(BuildField)]`](derive_build_field.md) respectively; this document does not repeat their item shapes.

The record's corner cases — `Symbol!` versus `Index<N>` tagging, the newtype `HasFields` special case, and generic threading — are the same as those building blocks describe, because `CgpRecord` runs the same helpers. The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpRecord` and its methods, and [`derive_cgp_data`](derive_cgp_data.md) covers the umbrella derive this is a restriction of.

## Tests

`#[derive(CgpRecord)]` has no snapshot macro of its own; its expansion is identical to the record path of `#[derive(CgpData)]` and is pinned by the `snapshot_derive_cgp_data!` snapshots indexed in [derive_cgp_data.md's Snapshots section](derive_cgp_data.md#snapshots).

- The behavioral record tests in [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/) — notably [record_build_from.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_build_from.rs) and [record_build_with_handlers.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_build_with_handlers.rs) — exercise the builder that this derive produces.

## Source

- Entry point: `derive_cgp_record` in [cgp-macro-lib/src/cgp_record.rs](../../../crates/macros/cgp-macro-lib/src/cgp_record.rs).
- Codegen: `ItemCgpRecord::to_items` in [cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs), which composes `derive_has_field_impls_from_struct`, `derive_has_fields_impls_from_struct`, and the [derive_builder/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_builder/) helpers; the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The runtime traits live in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/).
