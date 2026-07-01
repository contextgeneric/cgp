# `MapType`

`MapType` is the type-level marker trait whose associated `Map<T>` describes how a single field's value type is wrapped, so that builders and extractors can track each field's presence at the type level rather than at runtime.

## Purpose

`MapType` exists to make "what state is this field in?" a question the compiler answers, not the program. CGP's extensible-record machinery represents a half-built struct or a partially-extracted enum as a single type whose every field is independently wrapped: a field that is present carries its value, a field that has been consumed carries nothing, and a field that may or may not be there carries an `Option`. Rather than hard-code those three shapes, CGP parameterizes each field by a `MapType` marker and reads the field's actual storage type out of that marker's `Map<T>`. The marker is the field's state, expressed as a type.

This is what lets the builder family in [`HasBuilder`](has_builder.md) and the extractor family in [`ExtractField`](extract_field.md) move a record through intermediate states without ever changing the runtime representation of a value. A builder starts every field as "nothing" and flips each one to "present" as it is filled; an extractor starts every variant as "present" and flips each one to "void" as it is consumed. Because the flip is a change of type parameter, the compiler tracks exactly which fields are filled and refuses to finalize a record with a missing field. `MapType` is the vocabulary for those per-field states.

The companion trait [`MapTypeRef`](#maptyperef) does the same job for borrowed views, where the wrapping additionally introduces a lifetime — a reference, a mutable reference, or an owned value.

## Definition

`MapType` is a trait with a single generic associated type. The implementor is a zero-sized marker, and `Map<T>` names the storage type that the marker assigns to a field whose value type is `T`:

```rust
pub trait MapType {
    type Map<T>;
}
```

The four standard markers in `cgp-field` cover the states a field passes through. `IsPresent` is the identity wrapping — the field holds its value directly. `IsNothing` erases the value to the unit type, marking the field as absent. `IsVoid` maps to the uninhabited [`Void`](../types/either.md) type, marking a variant that can never be reached. `IsOptional` wraps the value in `Option<T>`, marking a field that is optionally present:

```rust
impl MapType for IsPresent { type Map<T> = T; }
impl MapType for IsNothing { type Map<T> = (); }
impl MapType for IsVoid    { type Map<T> = Void; }
impl MapType for IsOptional { type Map<T> = Option<T>; }
```

## `MapTypeRef`

`MapTypeRef` is the borrowed-view counterpart, used when a record is viewed by reference rather than by value. Its `Map<'a, T>` additionally threads a lifetime, so the marker decides not just whether the value is present but whether it is borrowed shared, borrowed mutably, or owned:

```rust
pub trait MapTypeRef {
    type Map<'a, T: 'a>: 'a;
}
```

The three standard markers correspond to the three ways of holding a value behind a lifetime. `IsRef` maps to `&'a T`, `IsMut` to `&'a mut T`, and `IsOwned` to a plain owned `T`:

```rust
impl MapTypeRef for IsRef   { type Map<'a, T: 'a> = &'a T; }
impl MapTypeRef for IsMut   { type Map<'a, T: 'a> = &'a mut T; }
impl MapTypeRef for IsOwned { type Map<'a, T: 'a> = T; }
```

A borrowed extractor combines the two families: the partial type carries one outer `MapTypeRef` marker (`IsRef` for `extractor_ref`, `IsMut` for `extractor_mut`) shared across all fields, and one `MapType` marker per field. A field's storage type is then `MapType::Map<MapTypeRef::Map<'a, T>>` — the per-field presence marker applied to the borrowed value type.

## `TransformMap` and `TransformMapFields`

`TransformMap` is a value-level natural transformation that converts a field from one `MapType` wrapping to another. Where `MapType` only names storage types, `TransformMap<M1, M2, T>` carries the function that turns an `M1::Map<T>` value into an `M2::Map<T>` value:

```rust
pub trait TransformMap<M1: MapType, M2: MapType, T> {
    fn transform_mapped(value: M1::Map<T>) -> M2::Map<T>;
}
```

`TransformMapFields` lifts such a transform across every field of a whole partial record. It is implemented for any context that is a [`PartialData`](has_builder.md), and it walks the target's [`HasFields`](has_fields.md) product field by field, applying the `Transform` to re-wrap each field from its current marker into `TargetMap`:

```rust
pub trait TransformMapFields<Transform, TargetMap> {
    type Output;

    fn transform_map_fields(self) -> Self::Output;
}
```

The recursion is the load-bearing part. For each `Field<Tag, Value>` in the spine, `TransformMapFields` uses [`UpdateField`](has_builder.md) twice: it first takes the field out (replacing its marker with `IsNothing`), reads its current marker, applies `Transform::transform_mapped` to convert the value to the `TargetMap` wrapping, then writes it back under `TargetMap`. The result type therefore has every field re-marked to `TargetMap`, with the values transformed accordingly.

## Examples

These markers are mostly seen through the generated partial types of [`#[derive(CgpData)]`](../derives/derive_cgp_data.md), but the transform family is directly useful. A common application fills in default values for absent fields by transforming every field's marker to `IsPresent`, supplying `Default::default()` wherever a field was `IsNothing` or an empty `Option`:

```rust
use cgp::prelude::*;

pub struct FillDefaults;

impl<T> TransformMap<IsPresent, IsPresent, T> for FillDefaults {
    fn transform_mapped(value: T) -> T {
        value
    }
}

impl<T: Default> TransformMap<IsNothing, IsPresent, T> for FillDefaults {
    fn transform_mapped(_value: ()) -> T {
        T::default()
    }
}

impl<T: Default> TransformMap<IsOptional, IsPresent, T> for FillDefaults {
    fn transform_mapped(value: Option<T>) -> T {
        value.unwrap_or_default()
    }
}
```

Applied through `transform_map_fields`, this turns a partially-built record — where some fields are present, some absent, and some optional — into a fully present builder whose missing fields have been filled with their defaults, ready to finalize.

## Related constructs

`MapType` is the per-field state vocabulary that [`HasBuilder`](has_builder.md) and its `PartialData`/`UpdateField` family use to track which fields are filled, and that [`ExtractField`](extract_field.md) uses to track which variants are still reachable. The `IsPresent`/`IsNothing`/`IsVoid`/`IsOptional` markers wrap field values; `IsVoid` maps to the uninhabited [`Void`](../types/either.md), the terminator of the [`Either`](../types/either.md) sum spine. The borrowed markers `IsRef`/`IsMut`/`IsOwned` of `MapTypeRef` feed the reference extractors. [`MapFields`](product_ops.md) applies a single `MapType` marker uniformly to every entry of a [`Cons`](../types/cons.md) product or `Either` sum, producing the partial-type field lists these markers populate. The whole scheme is generated by [`#[derive(CgpData)]`](../derives/derive_cgp_data.md). The [optional-field traits](optional_fields.md) build on this vocabulary with the `TransformMapDefault` and `TransformOptional` markers, which fill or wrap fields when finalizing a partially-built record. The conceptual overviews that show these markers tracking field and variant state are [extensible records](../../concepts/extensible-records.md) and [extensible variants](../../concepts/extensible-variants.md).

## Source

`MapType` is defined in [crates/core/cgp-field/src/traits/map_type.rs](../../../crates/core/cgp-field/src/traits/map_type.rs) and `MapTypeRef` in [crates/core/cgp-field/src/traits/map_type_ref.rs](../../../crates/core/cgp-field/src/traits/map_type_ref.rs). The standard markers are in [crates/core/cgp-field/src/impls/map_type.rs](../../../crates/core/cgp-field/src/impls/map_type.rs) (`IsPresent`, `IsNothing`, `IsVoid`, `IsOptional`) and [crates/core/cgp-field/src/impls/map_type_ref.rs](../../../crates/core/cgp-field/src/impls/map_type_ref.rs) (`IsRef`, `IsMut`, `IsOwned`). `TransformMap` and `TransformMapFields` are in [crates/core/cgp-field/src/traits/transform_map.rs](../../../crates/core/cgp-field/src/traits/transform_map.rs). The default-filling transform built on this machinery lives in [crates/extra/cgp-field-extra/src/impls/build_default.rs](../../../crates/extra/cgp-field-extra/src/impls/build_default.rs). The generated partial types that carry these markers appear in the extensible-data tests under [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/) and [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/).
