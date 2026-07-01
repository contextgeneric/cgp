# `#[derive(CgpData)]` — implementation

`#[derive(CgpData)]` is the umbrella extensible-data derive: applied to a struct it emits the full record machinery, applied to an enum the full variant machinery, dispatching on shape and reusing the same codegen as the shape-specific `#[derive(CgpRecord)]` and `#[derive(CgpVariant)]`. This document covers how that composition works; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_cgp_data.md](../../reference/derives/derive_cgp_data.md).

## Entry point

The macro is driven by the `derive_cgp_data` function in [cgp-macro-lib/src/cgp_data.rs](../../../crates/macros/cgp-macro-lib/src/cgp_data.rs). It parses the input into an `ItemCgpData` and calls `to_items`:

```rust
let data: ItemCgpData = parse2(body)?;
let items = data.to_items()?;
```

`ItemCgpData` is an enum of `Record(ItemCgpRecord)` or `Variant(ItemCgpVariant)`; its `Parse` impl routes a `struct` to the record arm and an `enum` to the variant arm, rejecting anything else with "expect body to be either a struct or enum". Its `to_items` forwards to the wrapped type's `to_items`, so `CgpData` on a struct emits exactly what `#[derive(CgpRecord)]` emits and `CgpData` on an enum exactly what `#[derive(CgpVariant)]` emits — the two shape-specific derives are `CgpData` restricted to one shape. See the [`cgp_data` AST stack](../asts/cgp_data.md) for those types.

## Pipeline

There is no multi-stage transform; the composition happens inside the two `to_items` methods.

- **`ItemCgpRecord::to_items`** (the struct path) concatenates three slices in order: the per-field `HasField`/`HasFieldMut` getters (the [`#[derive(HasField)]`](derive_has_field.md) output), the five representation impls (the [`#[derive(HasFields)]`](derive_has_fields.md) output), and the incremental builder items (the [`#[derive(BuildField)]`](derive_build_field.md) output).
- **`ItemCgpVariant::to_items`** (the enum path) concatenates three slices in order: the five representation impls over a sum (the enum path of [`#[derive(HasFields)]`](derive_has_fields.md)), the per-variant `FromVariant` constructors (the [`#[derive(FromVariant)]`](derive_from_variant.md) output), and the incremental extractor items (the [`#[derive(ExtractField)]`](derive_extract_field.md) output).

Each slice is documented in the entrypoint linked beside it; because `CgpData` reuses those exact methods, this document does not repeat the item shapes and points to them instead.

## Generated items

For a struct, the fixed emission order is: the `HasField`/`HasFieldMut` getters per field, then `HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, `ToFieldsRef`, then the builder block — the `__Partial{Name}` struct, `HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, then the per-field `UpdateField` and `HasField` impls. For an enum: `HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, `ToFieldsRef`, then one `FromVariant` per variant, then the extractor block — the `__Partial{Name}` and `__PartialRef{Name}` enums, `PartialData` for each, `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`, `FinalizeExtract` for each, then the per-variant `ExtractField` impls for both.

The two views this composes are the *representation* view (the `HasFields` product or sum, convertible with `FromFields`/`ToFields`) and the *incremental* view (the `__Partial…` companion type that tracks per-field presence or per-variant possibility in its type parameters). The reserved companion names are `__Partial{Name}` and, for the borrowed extractor, `__PartialRef{Name}`. The full item shapes live in the building-block entrypoint documents linked above.

## Behavior and corner cases

Field tagging follows the same rule as the whole family: a named struct field or an enum variant is keyed by [`Symbol!`](../../reference/macros/symbol.md), and a tuple-struct field by [`Index<N>`](../../reference/types/index.md), so a tuple-struct record exposes `Field<Index<0>, _>` entries and `UpdateField<Index<0>, _>` impls rather than symbol tags. The type's generic parameters and `where` clause are threaded onto every generated impl and onto the `__Partial…` companion types.

The shape-specific corner cases are inherited from the building blocks rather than introduced here: a single-field tuple struct is special-cased in the `HasFields` product (see [`derive_has_fields`](derive_has_fields.md)), and an enum whose variants are not each single-unnamed-field tuple variants fails in the extractor and `FromVariant` codegen (see [`derive_extract_field`](derive_extract_field.md) and [`derive_from_variant`](derive_from_variant.md)). `CgpData` on such an enum therefore fails the same way, because it runs the same helpers.

## Snapshots

Every `snapshot_derive_cgp_data!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint. The record expansion is owned by the `extensible_records` target and the variant expansion by `extensible_variants`:

- [extensible_records/record_derive.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_derive.rs) — the canonical named-field record expansion (getters, representation, builder).
- [extensible_records/person_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/person_record.rs) — multi-character field names, pinning the full `Symbol<N, Chars<…>>` spine with its length prefix.
- [extensible_records/tuple_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/tuple_record.rs) — a tuple struct, the whole record spine keyed by `Index<N>`.
- [extensible_records/generic_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/generic_record.rs) — a generic record with a `where` clause threaded through every impl.
- [extensible_records/optional_builder.rs](../../../crates/tests/cgp-tests/tests/extensible_records/optional_builder.rs) — the record expansion under the optional-builder runtime path.
- [extensible_records/point_cast.rs](../../../crates/tests/cgp-tests/tests/extensible_records/point_cast.rs) — the record expansion behind a structural record cast with `build_with_default`.
- [extensible_variants/derive_cgp_data.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/derive_cgp_data.rs) — the canonical concrete-enum variant expansion (representation, `FromVariant`, extractor).
- [extensible_variants/derive_cgp_data_generic.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/derive_cgp_data_generic.rs) — a generic enum, generics lifted onto the `__Partial*` extractor enums, with upcast/downcast.
- [extensible_variants/derive_cgp_data_shape.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/derive_cgp_data_shape.rs) — an enum with struct payloads and multi-character variant names.

## Tests

The snapshot tests above also carry runtime assertions that exercise the composed machinery:

- `person_record.rs` builds an `Employee` from a `Person` via the builder.
- `optional_builder.rs` drives the optional builder (`set`/`finalize_optional`/`finalize_with_default`).
- `point_cast.rs` casts a smaller record up into a larger one.
- The `derive_cgp_data*` variant snapshots run the extractor and the upcast/downcast casts.
- The neighboring [record_build_from.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_build_from.rs), [record_build_with_handlers.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_build_with_handlers.rs), [shape_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/shape_dispatch.rs), [shape_dispatch_ref.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/shape_dispatch_ref.rs), and [variant_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/variant_dispatch.rs) exercise the builder and dispatch behaviors on `CgpData` types without pinning a snapshot.

## Source

- Entry point: `derive_cgp_data` in [cgp-macro-lib/src/cgp_data.rs](../../../crates/macros/cgp-macro-lib/src/cgp_data.rs).
- Shape dispatch: `ItemCgpData` in [cgp-macro-core/src/types/cgp_data/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/item.rs), documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The record path is `ItemCgpRecord::to_items` in `record.rs` (see [`derive_cgp_record`](derive_cgp_record.md)) and the variant path `ItemCgpVariant::to_items` in `variant.rs` (see [`derive_cgp_variant`](derive_cgp_variant.md)), both under [cgp-macro-core/src/types/cgp_data/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/).
- The runtime traits live in [crates/core/cgp-field/src/](../../../crates/core/cgp-field/src/).
