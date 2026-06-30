# Casting: `CanUpcast`, `CanDowncast`, `CanDowncastFields`, `CanBuildFrom`

`CanUpcast`, `CanDowncast`, `CanDowncastFields`, and `CanBuildFrom` are the structural conversion traits that move a value between two CGP data types by their shared fields or variants — widening an enum, narrowing it, or assembling a struct from the fields of others — entirely by matching type-level names.

## Purpose

These traits exist so that two types sharing a subset of named fields or variants can be converted into one another generically, without any hand-written `From`/`TryFrom` impl. CGP represents every record as a product of named fields and every enum as a sum of named variants (via [`#[derive(HasFields)]`](../derives/derive_has_fields.md)), and once a type's shape is a type-level list, conversion becomes a matter of routing each named entry to the matching slot in the target. The four traits cover the directions that routing can take: enlarging a sum, shrinking a sum, and growing a record out of smaller records.

Upcast and downcast are the two directions of variant conversion. An *upcast* takes a value of a "narrow" enum whose variants are a subset of a "wide" enum's, and lifts it into the wide enum — always succeeds, because every variant of the source has a home in the target. A *downcast* goes the other way: it tries to narrow a wide enum into a smaller one, succeeding only if the value's current variant exists in the target, and otherwise handing back a remainder so the caller can keep trying other targets. `CanBuildFrom` is the record counterpart: it assembles a target by copying the fields it shares with a source, leaving the rest of the target to be filled in separately.

## Definition

`CanUpcast<Target>` consumes a value and produces the wider `Target` directly, since an upcast cannot fail. The `PhantomData<Target>` argument only names the target type for inference:

```rust
pub trait CanUpcast<Target> {
    fn upcast(self, _tag: PhantomData<Target>) -> Target;
}
```

`CanDowncast<Target>` is fallible. It returns `Result<Target, Self::Remainder>`, where the `Remainder` is the source's extractor with the attempted variants removed — what is left to try if this narrowing did not match:

```rust
pub trait CanDowncast<Target> {
    type Remainder;

    fn downcast(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder>;
}
```

`CanDowncastFields<Target>` is the same operation one level down, implemented directly on an extractor (a partial-sum type) rather than on the original enum. It is what you call on a `Remainder` returned by a previous `downcast` to attempt the next target without re-wrapping:

```rust
pub trait CanDowncastFields<Target> {
    type Remainder;

    fn downcast_fields(self, _tag: PhantomData<Target>) -> Result<Target, Self::Remainder>;
}
```

`CanBuildFrom<Source>` assembles a target builder by drawing fields from a source. It is implemented for a builder and consumes a `Source`, moving every field the two share out of the source and into the builder, returning the updated builder as `Output`:

```rust
pub trait CanBuildFrom<Source> {
    type Output;

    fn build_from(self, source: Source) -> Self::Output;
}
```

## Behavior

The variant conversions are driven by a shared recursion over the target's sum of variants. `CanUpcast` is implemented for any context that has both a [`HasFields`](has_fields.md) shape and a [`HasExtractor`](extract_field.md): it turns the source into its extractor, then walks the source's own field list, extracting each variant from the extractor and reconstructing it into the target via [`FromVariant`](from_variant.md). Because every source variant is guaranteed to exist in a wider target, the walk is total and the remaining extractor is uninhabited, finalized away with [`FinalizeExtract`](extract_field.md). The result is the source value re-tagged as the target enum.

`CanDowncast` and `CanDowncastFields` recurse over the *target's* variants instead. For each `Field<Tag, Value>` in `Target::Fields`, the implementation uses [`ExtractField`](extract_field.md) to try pulling that variant out of the source extractor: on success it rebuilds the target with `FromVariant` and returns `Ok`; on failure it threads the shrunken remainder into the next variant's attempt. If no target variant matches, the terminal `Void` impl returns the whole remainder as `Err`. The difference between the two traits is only the starting point — `CanDowncast` first calls `to_extractor` on the original enum, while `CanDowncastFields` operates on an extractor it is already given, which is exactly the `Remainder` from a prior `downcast`. This is why downcasting against several candidate targets in turn chains a `downcast` followed by `downcast_fields` calls on each remainder.

`CanBuildFrom` recurses over the source's field product. For each `Field<Tag, Value>` the source exposes, it uses [`TakeField`](has_builder.md) to remove that field's value from the source and [`BuildField`](has_builder.md) to write it into the target builder, threading both the shrinking source and the growing builder through the recursion. When the source's fields are exhausted, the target builder is returned. The target need not be complete after one `build_from`: a builder can absorb fields from several sources in sequence, and only then be finalized.

## Examples

Upcasting and downcasting let independently-defined enums interconvert by their common variants. Given a wide enum and a narrow one that share variant names, conversion is name-driven and needs no manual impl:

```rust
use cgp::prelude::*;
use core::marker::PhantomData;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBar {
    Foo(u64),
    Bar(String),
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBarBaz {
    Foo(u64),
    Bar(String),
    Baz(bool),
}

// upcast: always succeeds, every FooBar variant exists in FooBarBaz
let wide = FooBar::Foo(1).upcast(PhantomData::<FooBarBaz>);
assert_eq!(wide, FooBarBaz::Foo(1));

// downcast: succeeds for shared variants, fails for Baz
assert_eq!(FooBarBaz::Bar("hi".into()).downcast(PhantomData::<FooBar>).ok(), Some(FooBar::Bar("hi".into())));
assert_eq!(FooBarBaz::Baz(true).downcast(PhantomData::<FooBar>).ok(), None);
```

`CanBuildFrom` assembles one struct from several smaller ones by copying their fields into a builder before finalizing:

```rust
#[derive(CgpData)] pub struct FooBar { pub foo: u64, pub bar: String }
#[derive(CgpData)] pub struct Baz { pub baz: bool }
#[derive(CgpData)] pub struct FooBarBaz { pub foo: u64, pub bar: String, pub baz: bool }

let combined: FooBarBaz = FooBarBaz::builder()
    .build_from(FooBar { foo: 1, bar: "bar".into() })
    .build_from(Baz { baz: true })
    .finalize_build();
```

## Related constructs

The casting traits sit on top of the extensible-data primitives: variant casts route through [`ExtractField`](extract_field.md), [`FromVariant`](from_variant.md), and [`FinalizeExtract`](extract_field.md) over the [`Either`](../types/either.md)/`Void` sum spine, while `CanBuildFrom` routes through [`HasBuilder`](has_builder.md)'s `TakeField`/`BuildField` over the [`Cons`](../types/cons.md)/`Nil` product spine. All four depend on the type's [`HasFields`](has_fields.md) shape, which is what [`#[derive(CgpData)]`](../derives/derive_cgp_data.md) generates along with the builders and extractors these conversions consume. The names matched during casting are the [`Symbol!`](../macros/symbol.md) tags on each field or variant. The conceptual overviews that frame these conversions are [extensible records](../../concepts/extensible-records.md) (for `CanBuildFrom`) and [extensible variants](../../concepts/extensible-variants.md) (for upcasting and downcasting); `build_from` is used to assemble a context in the [application builder](../../examples/application-builder.md) example, `CanUpcast` to construct partial enums in the [expression interpreter](../../examples/expression-interpreter.md) example, and both `upcast` and `downcast` to convert between sibling shape enums in the [extensible shapes](../../examples/extensible-shapes.md) example.

## Source

`CanUpcast`, `CanDowncast`, and `CanDowncastFields`, together with the internal `FieldsExtractor` recursion that drives them, are defined in [crates/core/cgp-field/src/impls/cast.rs](../../../crates/core/cgp-field/src/impls/cast.rs). `CanBuildFrom` and its internal `FieldsBuilder` recursion are in [crates/core/cgp-field/src/impls/build_from.rs](../../../crates/core/cgp-field/src/impls/build_from.rs). The underlying extractor and builder traits are under [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/) (`extract_field.rs`, `from_variant.rs`, `has_builder.rs`). End-to-end tests of upcasting, downcasting, and building-from live in the extensible-data suites under [crates/tests/cgp-tests/tests/extensible_data_tests/](../../../crates/tests/cgp-tests/tests/extensible_data_tests/).
