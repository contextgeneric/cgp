# Optional and defaulted field extensions

The optional-field extensions are the traits and providers that let a record be finalized even when some fields were never set, by filling the gaps from `Default` or treating them as optional, rather than requiring every field to be present.

## Purpose

These extensions solve the problem that the core builder family is deliberately strict: a partial record can be turned back into its concrete struct only once every field has been set, because [`FinalizeBuild`](has_builder.md) is implemented solely on the all-`IsPresent` configuration of the partial type. That strictness is exactly what catches a missing field at compile time, but it is too rigid for records where some fields have sensible defaults or are genuinely allowed to be absent. The traits in `cgp-field-extra` relax it in two controlled ways — filling unset fields with `Default::default()`, and tracking each field as an `Option` so absence is a runtime condition rather than a compile error — while reusing the same `UpdateField`-driven machinery underneath.

The whole layer is built by composing three core mechanisms rather than inventing new ones. It reuses [`UpdateField`](has_builder.md) to move individual fields between states, [`TransformMapFields`](map_type.md) to re-wrap every field of a partial record at once, and the [`MapType`](map_type.md) markers `IsPresent`, `IsNothing`, and `IsOptional` to name what state each field is in. The extensions are therefore best understood as a small set of `TransformMap` natural transformations plus the entry-point traits that drive them, layered on top of the builder and extractor traits in core.

## Definition

The layer splits into two complementary capabilities — defaulted finalization and optional fields — that share the same underlying transform pattern. Defaulted finalization fills any unset field from `Default` so a record can be completed without setting everything; the optional-field traits convert a builder so every field becomes an `Option`, allow those optional slots to be set and replaced individually, and finalize either by requiring presence with an error or by defaulting. The `TransformMap` markers `TransformMapDefault` and `TransformOptional` carry the actual per-field conversions, and the entry-point traits `CanBuildWithDefault`, `CanFinalizeWithDefault`, `HasOptionalBuilder`, `ToOptional`, `SetOptional`, and `FinalizeOptional` expose them as usable operations.

### `TransformMapDefault` — filling unset fields from `Default`

`TransformMapDefault` is the [`TransformMap`](map_type.md) natural transformation that re-wraps every field into `IsPresent`, supplying `Default::default()` wherever a field has no value. It is a zero-sized marker with one impl per source marker, each describing how a field in that state becomes a present value:

```rust
pub struct TransformMapDefault;

impl<T> TransformMap<IsPresent, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(value: T) -> T { value }
}

impl<T: Default> TransformMap<IsNothing, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(_value: ()) -> T { T::default() }
}

impl<T: Default> TransformMap<IsOptional, IsPresent, T> for TransformMapDefault {
    fn transform_mapped(value: Option<T>) -> T { value.unwrap_or_default() }
}
```

A field that is already present passes through unchanged; a field that is `IsNothing` (never set) becomes its type's default; and a field that is `IsOptional` becomes the contained value or, when empty, the default. Because all three target `IsPresent`, applying this transform across a record leaves every field present, which is precisely the configuration `FinalizeBuild` accepts.

### `CanFinalizeWithDefault` — finalize, defaulting whatever is unset

`CanFinalizeWithDefault` finalizes a partial record into its target struct, defaulting any field that is not present. It is implemented for any partial value whose fields can be transformed by `TransformMapDefault` into the all-present configuration that `FinalizeBuild` then consumes:

```rust
pub trait CanFinalizeWithDefault {
    type Output;
    fn finalize_with_default(self) -> Self::Output;
}

impl<Builder, Output> CanFinalizeWithDefault for Builder
where
    Builder: TransformMapFields<TransformMapDefault, IsPresent>,
    Builder::Output: FinalizeBuild<Target = Output>,
{
    type Output = Output;
    fn finalize_with_default(self) -> Output {
        self.transform_map_fields().finalize_build()
    }
}
```

The body is the layer's core motion: `transform_map_fields` walks the record and applies `TransformMapDefault` to every field, producing an all-`IsPresent` partial value, and `finalize_build` turns that into the concrete struct. The strict presence check still applies — but it always succeeds, because the transform guarantees presence before `finalize_build` is reached.

### `CanBuildWithDefault` — build from a source, defaulting the rest

`CanBuildWithDefault<Source>` constructs the target struct by copying whatever fields a `Source` record shares with it and defaulting every remaining field. It chains the core [`CanBuildFrom`](has_builder.md) copy step into a defaulted finalize:

```rust
pub trait CanBuildWithDefault<Source> {
    fn build_with_default(source: Source) -> Self;
}

impl<Source, Target, Builder> CanBuildWithDefault<Source> for Target
where
    Target: HasBuilder<Builder = Builder>,
    Builder: CanBuildFrom<Source>,
    Builder::Output: CanFinalizeWithDefault<Output = Target>,
{
    fn build_with_default(source: Source) -> Target {
        Target::builder().build_from(source).finalize_with_default()
    }
}
```

The pipeline reads top to bottom: start an empty builder for the target with [`HasBuilder`](has_builder.md), copy across every field the source and target have in common with `build_from`, then finalize with defaults for the fields the source did not supply. This is the field-level "widening cast" — turning a `Point2d` into a `Point3d` whose extra `z` is `0`, for instance — without naming any field explicitly.

### `ToOptional` and `TransformOptional` — re-wrap every field as `Option`

`ToOptional` converts a partial record so every field is wrapped in `IsOptional`, and `TransformOptional` is the `TransformMap` that performs the per-field conversion. Where `TransformMapDefault` targets `IsPresent`, `TransformOptional` targets `IsOptional`, mapping a present value to `Some` and an absent field to `None`:

```rust
pub struct TransformOptional;

impl<T> TransformMap<IsPresent, IsOptional, T> for TransformOptional {
    fn transform_mapped(value: T) -> Option<T> { Some(value) }
}

impl<T> TransformMap<IsNothing, IsOptional, T> for TransformOptional {
    fn transform_mapped(_value: ()) -> Option<T> { None }
}

pub trait ToOptional {
    type Output;
    fn to_optional(self) -> Self::Output;
}

impl<Context> ToOptional for Context
where
    Context: TransformMapFields<TransformOptional, IsOptional>,
{
    type Output = Context::Output;
    fn to_optional(self) -> Self::Output { self.transform_map_fields() }
}
```

After `to_optional`, the partial type's every field marker is `IsOptional`, so each field's storage is an `Option`. A field already set becomes `Some`, an unset field becomes `None`, and from then on every field can be assigned or reassigned freely, because an `IsOptional` slot can always be overwritten.

### `HasOptionalBuilder` — start an all-optional builder

`HasOptionalBuilder` is the entry point that hands back a fresh builder in which every field is already optional. It composes `HasBuilder::builder()` with `ToOptional`, so the resulting builder starts with every field `None`:

```rust
pub trait HasOptionalBuilder {
    type Builder;
    fn optional_builder() -> Self::Builder;
}

impl<Context, Builder> HasOptionalBuilder for Context
where
    Context: HasBuilder,
    Context::Builder: ToOptional<Output = Builder>,
{
    type Builder = Builder;
    fn optional_builder() -> Self::Builder { Self::builder().to_optional() }
}
```

This is the usual starting point for the optional-field workflow: `Context::optional_builder()` gives a builder whose fields can be set in any order and any number of times, deferring the decision about which fields must ultimately be present until finalization.

### `SetOptional` — set or replace an optional field

`SetOptional<Tag>` sets the value of an optional field, optionally returning whatever value it replaced. It is implemented for any context whose `Tag` field is in the `IsOptional` state and stays there after the update:

```rust
pub trait SetOptional<Tag> {
    type Value;
    fn set(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self;
    fn set_optional(
        self,
        _tag: PhantomData<Tag>,
        value: Self::Value,
    ) -> (Option<Self::Value>, Self);
}

impl<Context, Tag> SetOptional<Tag> for Context
where
    Context: UpdateField<Tag, IsOptional, Mapper = IsOptional, Output = Context>,
{
    type Value = Context::Value;
    fn set(self, tag, value) -> Self { self.set_optional(tag, value).1 }
    fn set_optional(self, tag, value) -> (Option<Self::Value>, Self) {
        self.update_field(tag, Some(value))
    }
}
```

Both methods reduce to a single `UpdateField` call that writes `Some(value)` into the `IsOptional` slot. The crucial detail is that the field's marker is `IsOptional` both before and after — the `Mapper = IsOptional, Output = Context` bounds keep the builder's type unchanged — so a field can be set repeatedly, and `set_optional` returns the previous `Option` while `set` discards it. This is what makes the optional builder freely mutable, in contrast to the core `build_field` that consumes an absent slot exactly once.

### `FinalizeOptional` — finalize, erroring on a genuinely missing field

`FinalizeOptional` finalizes an optional builder into its concrete struct, succeeding only if every field actually holds a value and returning an error naming the first missing field otherwise. Unlike `CanFinalizeWithDefault`, it does not substitute defaults — absence is a recoverable runtime error rather than a silent fill:

```rust
pub trait FinalizeOptional: PartialData {
    fn finalize_optional(self) -> Result<Self::Target, &'static str>;
}
```

The implementation walks the target's [`HasFields`](has_fields.md) spine field by field. For each field it pulls the `Option` out of the `IsOptional` slot with `UpdateField`, and if the value is present it writes it back as `IsPresent` with `BuildField`; if the value is `None` it returns `Err(Tag::VALUE)` — the field's name as a static string — without finalizing. Only when every field has yielded a value does it call `finalize_build` on the now all-present partial value and return `Ok`. The error type `&'static str` is the missing field's own name, so a caller learns exactly which field was left unset.

## Behavior

Two finalization strategies sit on top of the same optional builder, differing only in how they treat a field that was never set. After `optional_builder` and a series of `set` calls, calling `finalize_with_default` completes the record by defaulting every unset field, whereas calling `finalize_optional` completes it only if nothing is missing and otherwise reports the missing field by name. The choice is made at the finalize call site, not when the builder is created, so the same builder value can be finalized either way depending on whether absence should be tolerated.

The defaulted path and the optional path reuse the identical `transform_map_fields` recursion with different markers, which is why their behavior is so symmetric. `CanFinalizeWithDefault` drives `TransformMapDefault` toward `IsPresent`; `ToOptional` drives `TransformOptional` toward `IsOptional`. Both walk the same field spine, both rebuild the partial type one field at a time through `UpdateField`, and neither changes a value's runtime layout beyond wrapping or unwrapping an `Option` or substituting a default. The strict, all-present `FinalizeBuild` from core remains the only way a partial value becomes a concrete struct; these extensions simply guarantee the all-present configuration is reached before it is invoked.

## Examples

The optional-field workflow starts an all-optional builder, sets fields freely, and finalizes with one of the two strategies. Setting a field that already holds a value reports the replaced value, and finalizing with defaults fills whatever was never set:

```rust
use cgp::prelude::*;
use cgp::extra::field::impls::{
    CanFinalizeWithDefault, FinalizeOptional, HasOptionalBuilder, SetOptional,
};

#[derive(CgpData)]
pub struct Context {
    pub foo: String,
    pub bar: u64,
}

// Every field present: finalize_optional succeeds.
let builder = Context::optional_builder()
    .set(PhantomData::<Symbol!("foo")>, "foo".to_owned())
    .set(PhantomData::<Symbol!("bar")>, 42);

let (replaced, builder) =
    builder.set_optional(PhantomData::<Symbol!("foo")>, "bar".to_owned());
assert_eq!(replaced, Some("foo".to_owned()));   // the previous value comes back

let context = builder.finalize_optional().unwrap();
assert_eq!(context.foo, "bar");
assert_eq!(context.bar, 42);

// bar left unset: finalize_with_default fills it from Default.
let context = Context::optional_builder()
    .set(PhantomData::<Symbol!("foo")>, "foo".to_owned())
    .finalize_with_default();
assert_eq!(context.foo, "foo");
assert_eq!(context.bar, 0);
```

Had the second case used `finalize_optional` instead, it would have returned `Err("bar")` because `bar` was never set, rather than defaulting it to `0`.

The defaulted-build path constructs a wider record from a narrower one in a single call, defaulting the fields the source lacks:

```rust
use cgp::prelude::*;
use cgp::extra::field::impls::CanBuildWithDefault;

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point2d { x: u64, y: u64 }

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point3d { x: u64, y: u64, z: u64 }

let point_2d = Point2d { x: 1, y: 2 };
let point_3d = Point3d::build_with_default(point_2d);
assert_eq!(point_3d, Point3d { x: 1, y: 2, z: 0 });   // z defaulted
```

`build_with_default` copies `x` and `y` from the source through `build_from`, then defaults the unmatched `z` to `0` during the defaulted finalize.

## Related constructs

These extensions layer directly on the core builder family in [`HasBuilder`](has_builder.md): `CanBuildWithDefault` chains its `CanBuildFrom` copy step and `HasBuilder` entry point, and every finalize path ultimately calls `FinalizeBuild`. They are driven by the [`MapType`](map_type.md) machinery — `TransformMapDefault` and `TransformOptional` are `TransformMap` natural transformations, and `TransformMapFields` is the recursion that applies them across a whole record, re-marking each field's `IsPresent`/`IsNothing`/`IsOptional` state. `SetOptional` and `FinalizeOptional` reduce to the same `UpdateField` primitive used by the core `BuildField` and `TakeField`. The partial types these operate on are generated by [`#[derive(BuildField)]`](../derives/derive_build_field.md) (also exposed through `#[derive(CgpData)]`). The enum-side counterpart that takes fields apart variant by variant is [`ExtractField`](extract_field.md).

## Source

The extensions are defined in `cgp-field-extra`: `CanBuildWithDefault`, `CanFinalizeWithDefault`, and `TransformMapDefault` in [crates/extra/cgp-field-extra/src/impls/build_default.rs](../../../crates/extra/cgp-field-extra/src/impls/build_default.rs); `FinalizeOptional` in [crates/extra/cgp-field-extra/src/impls/finalize_optional.rs](../../../crates/extra/cgp-field-extra/src/impls/finalize_optional.rs); `SetOptional` in [crates/extra/cgp-field-extra/src/impls/set_optional.rs](../../../crates/extra/cgp-field-extra/src/impls/set_optional.rs); and `HasOptionalBuilder`, `ToOptional`, and `TransformOptional` in [crates/extra/cgp-field-extra/src/impls/to_optional.rs](../../../crates/extra/cgp-field-extra/src/impls/to_optional.rs). They build on the core traits in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/) (`UpdateField`, `BuildField`, `FinalizeBuild`, `PartialData`, `HasFields`, `TransformMap`, `TransformMapFields`) and the `MapType` markers in [crates/core/cgp-field/src/impls/map_type.rs](../../../crates/core/cgp-field/src/impls/map_type.rs). Behavioral tests are in [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/).
