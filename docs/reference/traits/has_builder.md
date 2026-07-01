# `HasBuilder` and the builder trait family

The builder family is the set of traits that let a record be assembled one field at a time, with field presence tracked at the type level so that a value can be finalized only once every field is set.

## Purpose

The builder family solves the problem of constructing a record incrementally and generically, where the fields are not all known at the same place in the code and the construction must still be checked at compile time. A plain struct literal requires every field to be supplied at once; these traits instead start from an empty *partial* value and add fields to it one by one, with each addition advancing a type-level record of which fields are present. The payoff is that finalizing an incomplete value is a compile error, not a runtime panic — the trait that turns a partial value back into the concrete struct is implemented only for the fully-present configuration.

The family is built around a single primitive, `UpdateField`, that changes one field's storage from one state to another. Everything else is either a wrapper over that primitive (`BuildField` sets an absent field present, `TakeField` removes a present field) or an entry/exit point for the whole process (`HasBuilder`/`IntoBuilder` start a build, `FinalizeBuild` ends one). The traits live in the field crate and are implemented for a struct by [`#[derive(BuildField)]`](../derives/derive_build_field.md), which also generates the partial companion type they operate on.

## Definition

The family divides into entry points, the update primitive, two wrappers over it, and the finalize step. The entry points obtain a partial value to build into. `HasBuilder` produces an empty one and `IntoBuilder` produces a fully-present one from an existing value:

```rust
pub trait HasBuilder {
    type Builder;
    fn builder() -> Self::Builder;
}

pub trait IntoBuilder {
    type Builder;
    fn into_builder(self) -> Self::Builder;
}
```

`UpdateField<Tag, M>` is the primitive every field operation reduces to. It changes the field named by `Tag` from its current marker (`Mapper`) to the new marker `M`, both of which are [`MapType`](map_type.md) markers, and returns the field's old value alongside the rebuilt partial value:

```rust
pub trait UpdateField<Tag, M: MapType> {
    type Value;
    type Mapper: MapType;                 // the field's marker before the update
    type Output;                          // the partial value with the field now in state M
    fn update_field(
        self,
        _tag: PhantomData<Tag>,
        value: M::Map<Self::Value>,
    ) -> (<Self::Mapper as MapType>::Map<Self::Value>, Self::Output);
}
```

The `M::Map<Self::Value>` argument and the `Mapper::Map<Self::Value>` first return component are the field's value as it is *stored* under each marker: `IsPresent` stores the value itself, `IsNothing` stores `()`. So updating an absent field to present takes the real value in and returns `()` as the old value; the reverse takes `()` in and returns the real value.

`BuildField<Tag>` and `TakeField<Tag>` are the two directions of that transition, each defined once in the field crate as a blanket impl over `UpdateField`. `BuildField` is the `IsNothing → IsPresent` direction — set a currently-absent field — and `TakeField` is the `IsPresent → IsNothing` direction — remove a currently-present field:

```rust
pub trait BuildField<Tag> {
    type Value;
    type Output;
    fn build_field(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self::Output;
}

impl<Context, Tag> BuildField<Tag> for Context
where
    Context: UpdateField<Tag, IsPresent, Mapper = IsNothing>,
{ /* build_field = self.update_field(tag, value).1 */ }

pub trait TakeField<Tag> {
    type Value;
    type Remainder;
    fn take_field(self, _tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder);
}

impl<Context, Tag> TakeField<Tag> for Context
where
    Context: UpdateField<Tag, IsNothing, Mapper = IsPresent>,
{ /* take_field = self.update_field(tag, ()) */ }
```

`PartialData` records which concrete struct a partial value targets, and `FinalizeBuild` — a subtrait of `PartialData` — turns the partial value back into that struct:

```rust
pub trait PartialData {
    type Target;
}

pub trait FinalizeBuild: PartialData {
    fn finalize_build(self) -> Self::Target;
}
```

## Behavior

A build is a sequence of `UpdateField`-driven state changes that only finalizes at the all-present configuration. The entry point fixes the starting state: `HasBuilder::builder()` returns the partial type with every field marker `IsNothing` (an empty value where each field is stored as `()`), while `IntoBuilder::into_builder(self)` returns it with every marker `IsPresent` (a full value carrying the real fields). Each `build_field` call flips one marker from `IsNothing` to `IsPresent` by calling the generated `update_field` and keeping only its `Output`; each `take_field` does the reverse and also hands back the removed value.

Presence lives entirely in the type. The derive generates the partial struct with one `MapType` parameter per field, and the marker in each position decides whether that field's slot holds the value (`IsPresent`), holds `()` (`IsNothing`), or holds the empty `Void` type (`IsVoid`). Because `BuildField` requires `Mapper = IsNothing` and `TakeField` requires `Mapper = IsPresent`, the compiler rejects building a field that is already set or taking one that is absent. The derive also emits a [`HasField`](has_field.md) impl on the partial type gated on `IsPresent`, so a field that has been set can be read back out of a still-incomplete value.

Finalizing is what makes the tracking load-bearing. The derive provides exactly one `FinalizeBuild` impl, on the all-`IsPresent` configuration of the partial type, so `finalize_build` is in scope only when every field is present; calling it on a partial value with any `IsNothing` field fails to compile. `PartialData::Target` is implemented for *every* configuration and names the struct being built, which is how generic builder code knows the destination type before the build is complete.

## Examples

The family is normally driven through `builder()`, a series of `build_field` calls, and `finalize_build`, with `build_from` (from the field crate's `CanBuildFrom`, itself layered on `BuildField`) copying every shared field from another record in one step:

```rust
use cgp::prelude::*;
use cgp::core::field::impls::CanBuildFrom;

#[derive(BuildField)]
pub struct FooBar { pub foo: u64, pub bar: String }

#[derive(BuildField)]
pub struct FooBarBaz { pub foo: u64, pub bar: String, pub baz: bool }

fn extend(foo_bar: FooBar) -> FooBarBaz {
    FooBarBaz::builder()                                   // all IsNothing
        .build_from(foo_bar)                              // foo, bar now IsPresent
        .build_field(PhantomData::<Symbol!("baz")>, true) // baz now IsPresent
        .finalize_build()                                 // only the all-present impl applies
}
```

Each line changes the partial type, and the `finalize_build` on the last line type-checks only because every marker has reached `IsPresent` by that point. Reordering the steps so that `finalize_build` ran before `baz` was set would be a compile error rather than a runtime failure.

## Related constructs

The struct-side derive that generates the partial type and all these impls is [`#[derive(BuildField)]`](../derives/derive_build_field.md), whose doc shows the exact expanded code. The presence markers `IsPresent`, `IsNothing`, and `IsVoid` are [`MapType`](map_type.md) implementations, and the partial type's `MapType` parameters are what record per-field state. Fields already set on a partial value are read back through [`HasField`](has_field.md). The enum counterparts to this family are the extractor traits in [`extract_field`](extract_field.md), which deconstruct a value variant by variant, and [`FromVariant`](from_variant.md), which constructs an enum from a single variant. The conceptual overview that ties this family into the [extensible builder pattern](../../concepts/extensible-records.md) is in [extensible records](../../concepts/extensible-records.md), worked through in the [application builder](../../examples/application-builder.md) example.

## Source

The traits are defined in [crates/core/cgp-field/src/traits/has_builder.rs](../../../crates/core/cgp-field/src/traits/has_builder.rs) (`HasBuilder`, `IntoBuilder`), [build_field.rs](../../../crates/core/cgp-field/src/traits/build_field.rs) (`BuildField`, `FinalizeBuild`), [update_field.rs](../../../crates/core/cgp-field/src/traits/update_field.rs) (`UpdateField`), [take_field.rs](../../../crates/core/cgp-field/src/traits/take_field.rs) (`TakeField`), and [partial_data.rs](../../../crates/core/cgp-field/src/traits/partial_data.rs) (`PartialData`). The `MapType` markers are in [crates/core/cgp-field/src/impls/map_type.rs](../../../crates/core/cgp-field/src/impls/map_type.rs), and `CanBuildFrom` in [crates/core/cgp-field/src/impls/build_from.rs](../../../crates/core/cgp-field/src/impls/build_from.rs). Tests are in [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/).
