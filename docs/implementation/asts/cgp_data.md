# The `cgp_data` AST stack

The `cgp_data` stack is the small family of AST types that every extensible-data derive parses into before the shared codegen takes over: `ItemCgpData`, `ItemCgpRecord`, `ItemCgpVariant`, and the field-tag types (`Symbol`, `Index`, `FieldName`, `HasFieldBound`) that name each field. Unlike the [`cgp_component` stack](cgp_component.md), there is no multi-stage `preprocess → eval → to_items` transform here: each of the eight data derives parses its input into one of these types and then calls a `to_*` method that composes free codegen helpers into a `Vec<syn::Item>`. This document covers the types; the per-derive entrypoint documents ([`derive_has_field`](../entrypoints/derive_has_field.md), [`derive_has_fields`](../entrypoints/derive_has_fields.md), [`derive_cgp_data`](../entrypoints/derive_cgp_data.md), [`derive_build_field`](../entrypoints/derive_build_field.md), [`derive_extract_field`](../entrypoints/derive_extract_field.md), [`derive_from_variant`](../entrypoints/derive_from_variant.md)) cover what each generated item looks like.

The organizing idea is that a *record* is a struct and a *variant* is an enum, and the two shapes drive disjoint codegen: a record produces field getters, the `HasFields` product, and the incremental builder; a variant produces the `HasFields` sum, the `FromVariant` constructors, and the incremental extractor. `ItemCgpRecord` and `ItemCgpVariant` are the two shape-specific types, `ItemCgpData` is the union that dispatches on shape, and the field-tag types are shared by both because every field — named struct field, tuple field, or enum variant — is addressed by the same kind of type-level tag.

## `ItemCgpData`

`ItemCgpData` is the shape-dispatching wrapper that backs `#[derive(CgpData)]`. It is an enum of `Record(ItemCgpRecord)` or `Variant(ItemCgpVariant)`, and its `Parse` impl parses a `syn::Item` and routes a `struct` to the record arm and an `enum` to the variant arm, rejecting anything else with "expect body to be either a struct or enum". Its only method, `to_items`, forwards to the wrapped `ItemCgpRecord::to_items` or `ItemCgpVariant::to_items`, so `CgpData` on a struct emits exactly what `CgpRecord` emits and `CgpData` on an enum exactly what `CgpVariant` emits.

## `ItemCgpRecord`

`ItemCgpRecord` wraps a `syn::ItemStruct` and owns all struct-shape codegen. It is a thin struct — just `item_struct` — that exposes one method per slice of the record output plus a `to_items` that concatenates them:

- `to_has_field_impls` — the per-field `HasField`/`HasFieldMut` getters, via `derive_has_field_impls_from_struct`. This is what `#[derive(HasField)]` calls.
- `to_has_fields_impls` — the five representation impls (`HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, `ToFieldsRef`), via `derive_has_fields_impls_from_struct`. This is the struct path of `#[derive(HasFields)]`.
- `to_build_field_items` — the incremental builder: the `__Partial{Name}` struct and its trait impls. This is what `#[derive(BuildField)]` calls.
- `to_items` — the full `#[derive(CgpRecord)]`/`#[derive(CgpData)]`-on-struct output: the getters, then the representation impls, then the builder items, in that order.

The builder method names the partial companion struct `__Partial{ContextName}` and composes the helpers in the [`derive_builder/`](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_builder/) submodule in a fixed order (builder struct, `HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, then the per-field `UpdateField` and `HasField` impls). `#[derive(HasField)]`, `#[derive(HasFields)]`, `#[derive(BuildField)]`, `#[derive(CgpRecord)]`, and the struct path of `#[derive(CgpData)]` all construct an `ItemCgpRecord` and call one of these methods, which is why they never disagree about the shape they emit.

## `ItemCgpVariant`

`ItemCgpVariant` wraps a `syn::ItemEnum` and owns all enum-shape codegen, mirroring `ItemCgpRecord`:

- `to_has_fields_impls` — the five representation impls over a *sum* rather than a product, via `derive_has_fields_impls_from_enum`. This is the enum path of `#[derive(HasFields)]`.
- `to_from_variant_impls` — the per-variant `FromVariant` constructors, via `derive_from_variant_from_enum`. This is what `#[derive(FromVariant)]` calls.
- `to_extract_field_items` — the incremental extractor: the `__Partial{Name}` and `__PartialRef{Name}` enums and their trait impls. This is what `#[derive(ExtractField)]` calls.
- `to_items` — the full `#[derive(CgpVariant)]`/`#[derive(CgpData)]`-on-enum output: the representation impls, then the `FromVariant` constructors, then the extractor items.

The extractor method names two partial companion enums, `__Partial{ContextName}` (owned) and `__PartialRef{ContextName}` (borrowed), and drives most extractor helpers twice — once for each — passing a `bool` that selects the borrowed form. That is why the borrowed extractor mirrors the owned one and both stay in sync with the owned/ref pair. `#[derive(HasFields)]` on an enum, `#[derive(FromVariant)]`, `#[derive(ExtractField)]`, `#[derive(CgpVariant)]`, and the enum path of `#[derive(CgpData)]` all construct an `ItemCgpVariant` and call one of these methods.

## `Symbol`, `Index`, and `FieldName`

The field-tag types decide how a field is named at the type level, and every data derive routes its field identifiers through them. A named field or an enum variant is keyed by a `Symbol` — a type-level string — and a tuple-struct field is keyed by an `Index` — a type-level natural number of its position.

`Symbol` holds the field's identifier as a `String` and, on `ToTokens`, emits the full type-level spelling `Symbol<N, Chars<…, Nil>>` where `N` is the character count and the `Chars` cons-list carries the characters. So the field name `foo` expands to `Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>`; the leading length works around the absence of const generics over strings. `Index` holds a `usize` position and emits `Index<N>`. `FieldName` is the enum that unifies the two — `Ident(Symbol)` or `Index(Index)` — so a helper that walks a struct's fields does not care whether they are named or positional; it converts each `Member` to a `FieldName` and lets `ToTokens` pick the right spelling.

```rust
// a named field                     a tuple field
Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>    Index<0>
```

`HasFieldBound` is the small companion that renders a `HasField`/`HasFieldMut` bound (`HasField<Tag, Value = T>`), used where the codegen needs to write such a bound as a `where`-clause fragment rather than as a full impl.

## Tests

- The stack has no parser-rejection tests of its own in `cgp-macro-tests`; the shape errors it raises (a non-struct/non-enum for `CgpData`, a non-struct for the record derives, a non-enum for the variant derives, and a non-single-field variant for the extractor and `FromVariant`) surface at `syn::parse2` or in the codegen helpers and are exercised only implicitly.
- The stage transforms are exercised end-to-end by the expansion snapshots indexed in the entrypoint documents' Snapshots sections — the [`snapshot_derive_has_field`](../entrypoints/derive_has_field.md#snapshots), [`snapshot_derive_has_fields`](../entrypoints/derive_has_fields.md#snapshots), and [`snapshot_derive_cgp_data`](../entrypoints/derive_cgp_data.md#snapshots) families.

## Source

- The stack lives in [crates/macros/cgp-macro-core/src/types/cgp_data/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/): `ItemCgpData` in `item.rs`, `ItemCgpRecord` in `record.rs`, and `ItemCgpVariant` in `variant.rs`.
- The record codegen helpers are under `derive_has_field.rs`, `derive_has_fields/`, and `derive_builder/`; the variant codegen helpers under `derive_has_fields/`, `derive_from_variant.rs`, and `derive_extractor/`.
- The field-tag types are in [crates/macros/cgp-macro-core/src/types/field/](../../../crates/macros/cgp-macro-core/src/types/field/): `Symbol` in `symbol.rs`, `Index` in `index.rs`, `FieldName` in `field_name.rs`, and `HasFieldBound` in `has_field_bound.rs`.
- The runtime traits these impls satisfy are defined in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/).
