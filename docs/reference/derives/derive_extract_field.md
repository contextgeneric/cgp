# `#[derive(ExtractField)]`

`#[derive(ExtractField)]` derives just the incremental-extractor machinery for an enum: partial companion enums plus the `HasExtractor`, `HasExtractorRef`, `HasExtractorMut`, `PartialData`, `FinalizeExtract`, and `ExtractField` impls that let the enum be matched one variant at a time.

## Purpose

`#[derive(ExtractField)]` gives an enum a type-checked, variant-by-variant extractor without the rest of the extensible-data machinery. It is the building block that supplies the *deconstruction* half of a variant: converting a value to an extractor, pulling one variant out at a time, and carrying the still-unmatched variants forward in a *remainder* whose type shrinks with each attempt. It exists as a standalone derive for cases where you want the extractor alone — though most code reaches for [`#[derive(CgpVariant)]`](derive_cgp_variant.md) or [`#[derive(CgpData)]`](derive_cgp_data.md), which include this output.

The extractor's distinguishing property is that remaining possibilities are tracked in the type. The derive generates *partial* companion enums whose type parameters record, per variant, whether that variant is still possible or has been ruled out. Each failed extraction returns a remainder with one more variant marked impossible; once all are impossible, the remainder is an empty type that can be discharged unconditionally. This is how a chain of extractions becomes a provably exhaustive match without a wildcard arm.

The capability is exposed through `ExtractField`, generated per variant, together with `HasExtractor` (and its borrowed and mutable forms) to obtain an extractor, and `FinalizeExtract` to discharge the empty remainder.

## Syntax

The derive is applied to an enum and takes no arguments:

```rust
use cgp::prelude::*;

#[derive(ExtractField)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

Each variant's name becomes a type-level string `Symbol!` used as the variant's `Tag`, and its payload type becomes its value type. Every variant must carry exactly one unnamed payload — a single-field tuple variant such as `Circle(Circle)`; a fieldless, multi-field, or struct-style variant is a compile error. Generic parameters on the enum are carried onto the generated impls. The derive emits the same extractor impls that the variant path of [`#[derive(CgpData)]`](derive_cgp_data.md) emits — it is that slice in isolation, with no `HasFields` representation traits and no [`FromVariant`](../traits/from_variant.md) constructors.

## Expansion

`#[derive(ExtractField)]` expands into two partial companion enums and the traits that drive them. The symbols below are abbreviated as `Symbol!("Name")` in place of the full `Symbol<Len, Chars<...>>` form. Starting from:

```rust
#[derive(ExtractField)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

it first emits the owned partial enum `__PartialShape` and the borrowed partial enum `__PartialRefShape`. Each variant's payload is wrapped in a `MapType` marker, where `IsPresent` keeps the payload and `IsVoid` maps it to the empty [`Void`](../types/either.md) type; the borrowed form adds a `MapTypeRef` parameter that selects shared or mutable references:

```rust
pub enum __PartialShape<__F0__: MapType, __F1__: MapType> {
    Circle(<__F0__ as MapType>::Map<Circle>),
    Rectangle(<__F1__ as MapType>::Map<Rectangle>),
}

pub enum __PartialRefShape<'__a__, __R__: MapTypeRef, __F0__: MapType, __F1__: MapType> {
    Circle(<__F0__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Circle>>),
    Rectangle(<__F1__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Rectangle>>),
}
```

It then emits `PartialData` (both partial enums target `Shape`) and the extractor accessors. `HasExtractor` yields an owned extractor with every variant present; `HasExtractorRef` and `HasExtractorMut` yield borrowed extractors over `__PartialRefShape` with `IsRef`/`IsMut`:

```rust
impl HasExtractor for Shape {
    type Extractor = __PartialShape<IsPresent, IsPresent>;
    fn to_extractor(self) -> Self::Extractor {
        match self {
            Self::Circle(value) => __PartialShape::Circle(value),
            Self::Rectangle(value) => __PartialShape::Rectangle(value),
        }
    }
    fn from_extractor(extractor: Self::Extractor) -> Self {
        match extractor {
            __PartialShape::Circle(value) => Self::Circle(value),
            __PartialShape::Rectangle(value) => Self::Rectangle(value),
        }
    }
}

impl HasExtractorRef for Shape {
    type ExtractorRef<'a> = __PartialRefShape<'a, IsRef, IsPresent, IsPresent> where Self: 'a;
    fn extractor_ref(&self) -> Self::ExtractorRef<'_> { /* ... */ }
}
// plus HasExtractorMut over __PartialRefShape<'a, IsMut, IsPresent, IsPresent>
```

It emits a `FinalizeExtract` impl for the all-`IsVoid` configuration of each partial enum. Because that configuration is uninhabited, `finalize_extract` can return any type by matching on the empty value:

```rust
impl FinalizeExtract for __PartialShape<IsVoid, IsVoid> {
    fn finalize_extract<__T__>(self) -> __T__ { match self {} }
}
// plus the borrowed __PartialRefShape<'a, __R__, IsVoid, IsVoid>
```

Finally it emits, per variant, an `ExtractField` impl available only when that variant's marker is `IsPresent`. Calling it returns `Ok(value)` if the runtime value is that variant, or `Err(remainder)` where the remainder has that variant flipped to `IsVoid`:

```rust
impl<__F1__: MapType> ExtractField<Symbol!("Circle")> for __PartialShape<IsPresent, __F1__> {
    type Value = Circle;
    type Remainder = __PartialShape<IsVoid, __F1__>;
    fn extract_field(self, _: PhantomData<Symbol!("Circle")>) -> Result<Circle, Self::Remainder> {
        match self {
            __PartialShape::Circle(value) => Ok(value),
            __PartialShape::Rectangle(value) => Err(__PartialShape::Rectangle(value)),
        }
    }
}
// plus ExtractField for "Rectangle", and the borrowed variants over __PartialRefShape
```

The `FinalizeExtract` trait itself is defined in the field crate (with blanket impls for `Void` and `Infallible`); the derive supplies the all-void impl on the partial enums. The companion `FinalizeExtractResult` trait, also in the field crate, is what `finalize_extract_result` calls to collapse an `Ok`/empty-`Err` result into the value.

The key takeaway is that `to_extractor` yields `__PartialShape<IsPresent, IsPresent>`, each failed `extract_field` returns a remainder with one more `IsVoid`, and at `__PartialShape<IsVoid, IsVoid>` the value is uninhabited — so once every variant has been tried, the compiler knows the match is exhaustive.

## Examples

The extractor is driven through `to_extractor`, a chain of `extract_field` calls, and `finalize_extract_result` to discharge the impossible remainder:

```rust
use cgp::prelude::*;
use cgp::core::field::traits::FinalizeExtractResult;

#[derive(ExtractField)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn area(shape: Shape) -> f64 {
    match shape.to_extractor().extract_field(PhantomData::<Symbol!("Circle")>) {
        Ok(circle) => core::f64::consts::PI * circle.radius * circle.radius,
        Err(remainder) => {
            let rect = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();   // remainder is now empty; cannot fail
            rect.width * rect.height
        }
    }
}
```

After the second extraction the remainder type has both markers `IsVoid`, so `finalize_extract_result` is accepted with no wildcard arm.

## Related constructs

`#[derive(ExtractField)]` is one slice of the variant output of [`#[derive(CgpData)]`](derive_cgp_data.md) and [`#[derive(CgpVariant)]`](derive_cgp_variant.md); those derives include it alongside the [`#[derive(FromVariant)]`](derive_from_variant.md) constructors and [`#[derive(HasFields)]`](derive_has_fields.md) representation traits. Its struct analogue is [`#[derive(BuildField)]`](derive_build_field.md), the incremental builder. The capability it generates is the [`ExtractField`](../traits/extract_field.md) trait. The generated code stores variants in [`sum`](../macros/sum.md)-shaped partial enums ([`Either`/`Void`](../types/either.md)) and switches on the [`MapType`](../traits/map_type.md) markers `IsPresent`/`IsVoid`.

## Known issues

The derive only accepts enums whose every variant is a single-field tuple variant. A fieldless variant like `Empty`, a multi-field variant like `Pair(A, B)`, or a struct-style variant like `Named { x: A }` causes the macro to fail with the error "Expected variant to contain exactly one unnamed field." There is no way to opt a variant out of the requirement, so an enum that mixes shapes cannot derive the extractor at all.

## Source

- Entry point: `derive_extract_field` in [crates/macros/cgp-macro-lib/src/derive_extract_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_extract_field.rs), which builds an `ItemCgpVariant` and calls `to_extract_field_items()`.
- Codegen: that method, in [crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs), composes the helpers in the [`derive_extractor/`](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_extractor/) submodule (`extractor_enum.rs`, `has_extractor_impl.rs`, `partial_data.rs`, `finalize_extract_impl.rs`, `extract_field_impls.rs`).
- Runtime traits: `ExtractField`, `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`, `FinalizeExtract`, and `FinalizeExtractResult` in [crates/core/cgp-field/src/traits/extract_field.rs](../../../crates/core/cgp-field/src/traits/extract_field.rs), `PartialData` in `partial_data.rs`, and the `MapType`/`MapTypeRef` markers in `crates/core/cgp-field/src/impls/`.
- Internal walkthrough (the extractor helpers, the corner-case handling, and the index of tests and expansion snapshots): [implementation/entrypoints/derive_extract_field.md](../../implementation/entrypoints/derive_extract_field.md).
