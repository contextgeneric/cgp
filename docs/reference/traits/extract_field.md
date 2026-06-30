# `ExtractField` and the extractor trait family

The extractor family is the set of traits that let an enum be matched one variant at a time, with the still-possible variants tracked in the type so that a chain of extractions becomes a provably exhaustive match without a wildcard arm.

## Purpose

The extractor family solves the problem of deconstructing a sum type generically and exhaustively. A `match` on a concrete enum names every variant in one place; the extractor instead pulls variants out one at a time, and each failed attempt narrows the set of remaining possibilities. The narrowing happens in the type: a failed extraction hands back a *remainder* whose type has one more variant marked impossible, and once every variant has been ruled out the remainder is an uninhabited type that can be discharged unconditionally. This is how generic code can match an arbitrary enum, variant by variant, and have the compiler confirm the match is complete.

The family pairs an accessor that turns a value into an extractor with a per-variant extraction operation and a finalize step for the empty remainder. `HasExtractor` and its borrowed forms obtain the extractor; `ExtractField<Tag>` tries one variant and returns either the payload or the shrunken remainder; `FinalizeExtract` discharges the remainder once it is uninhabited. The traits live in the field crate and are implemented for an enum by [`#[derive(ExtractField)]`](../derives/derive_extract_field.md), which generates the partial companion enums they operate on.

## Definition

The family consists of three accessor traits, the extraction operation, and two finalize traits. The accessors turn a value into an extractor in one of three ownership modes — owned, shared-reference, and mutable-reference:

```rust
pub trait HasExtractor {
    type Extractor;
    fn to_extractor(self) -> Self::Extractor;
    fn from_extractor(extractor: Self::Extractor) -> Self;
}

pub trait HasExtractorRef {
    type ExtractorRef<'a> where Self: 'a;
    fn extractor_ref(&self) -> Self::ExtractorRef<'_>;
}

pub trait HasExtractorMut {
    type ExtractorMut<'a> where Self: 'a;
    fn extractor_mut(&mut self) -> Self::ExtractorMut<'_>;
}
```

`ExtractField<Tag>` is the extraction operation. It attempts to read the variant named by `Tag` out of the extractor, returning `Ok(value)` if the runtime value is that variant or `Err(remainder)` if it is not, where the remainder is the same extractor with that one variant ruled out:

```rust
pub trait ExtractField<Tag> {
    type Value;
    type Remainder;
    fn extract_field(self, _tag: PhantomData<Tag>) -> Result<Self::Value, Self::Remainder>;
}
```

`FinalizeExtract` discharges a remainder that has become uninhabited. Its method returns *any* type, which is sound precisely because there is no value to return it from — it is implemented for the empty [`Void`](../types/either.md) type and for the standard-library `Infallible`, both by matching on the empty value:

```rust
pub trait FinalizeExtract {
    fn finalize_extract<T>(self) -> T;
}

impl FinalizeExtract for Void {
    fn finalize_extract<T>(self) -> T { match self {} }
}

impl FinalizeExtract for Infallible {
    fn finalize_extract<T>(self) -> T { match self {} }
}
```

`FinalizeExtractResult` is the convenience wrapper that collapses the `Result` produced by the last extraction. It is implemented for any `Result<T, E>` whose error half implements `FinalizeExtract`, returning the `Ok` value and discharging the `Err`:

```rust
pub trait FinalizeExtractResult {
    type Output;
    fn finalize_extract_result(self) -> Self::Output;
}

impl<T, E> FinalizeExtractResult for Result<T, E>
where E: FinalizeExtract {
    type Output = T;
    fn finalize_extract_result(self) -> T {
        match self {
            Ok(value) => value,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}
```

## Behavior

Extraction proceeds variant by variant against a shrinking remainder until that remainder is uninhabited. The derive generates a partial companion enum with one [`MapType`](map_type.md) parameter per variant; the marker in each position decides whether that variant's payload is present (`IsPresent`) or has been mapped to the empty `Void` type (`IsVoid`). `to_extractor` starts the chain at the all-`IsPresent` configuration, where every variant is still possible. Each `extract_field` call is available only when the requested variant's marker is `IsPresent`; it matches on the value, returning the payload if it is that variant, or returning the remainder with that one marker flipped to `IsVoid` if it is not.

Exhaustiveness is proven at the type level rather than by a wildcard. As variants are ruled out, the remainder's markers turn to `IsVoid` one by one, and `IsVoid` maps each payload slot to `Void`. When every marker is `IsVoid`, every arm of the partial enum holds a `Void`, so the whole type is uninhabited. The derive supplies a `FinalizeExtract` impl on exactly that all-`IsVoid` configuration, and that impl is sound only because the value cannot exist — `match self {}` has no arms to write. A caller therefore reaches `finalize_extract` (directly, or through `finalize_extract_result` on the last `Result`) only after trying every variant, and the compiler accepts the discharge with no wildcard arm.

The three accessor traits differ only in ownership. `HasExtractor` consumes the value and yields an owned extractor whose payloads are owned; the `from_extractor` method reverses `to_extractor` for an unmatched value. `HasExtractorRef` and `HasExtractorMut` borrow the value and yield a borrowed extractor over the same partial enum, carrying a `MapTypeRef` marker (`IsRef` or `IsMut`) that maps each payload slot to a shared or mutable reference, so a value can be matched without being moved.

## Examples

The family is normally driven through `to_extractor`, a chain of `extract_field` calls, and `finalize_extract_result` to discharge the impossible remainder at the end:

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
            // remainder now has Circle ruled out (IsVoid)
            let rect = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();   // remainder is now uninhabited; cannot fail
            rect.width * rect.height
        }
    }
}
```

After the second extraction the remainder has both markers `IsVoid`, so its type is uninhabited and `finalize_extract_result` is accepted with no wildcard arm. Adding a third variant to `Shape` would make this function fail to compile until the new variant is handled, because the remainder after two extractions would no longer be uninhabited.

## Related constructs

The enum-side derive that generates the partial enums and all these impls is [`#[derive(ExtractField)]`](../derives/derive_extract_field.md), whose doc shows the exact expanded code. The presence markers `IsPresent` and `IsVoid` are [`MapType`](map_type.md) implementations, and the empty remainder bottoms out in the [`Void`](../types/either.md) type that anchors the sum spine. The construction counterpart, which puts a variant *into* an enum rather than taking one out, is [`FromVariant`](from_variant.md). The struct analogue of the whole family is the builder family in [`has_builder`](has_builder.md), which assembles a record field by field instead of deconstructing a variant. The conceptual overview that frames this family is [extensible variants](../../concepts/extensible-variants.md), worked through in the [expression interpreter](../../examples/expression-interpreter.md) example.

## Source

The traits — `ExtractField`, `HasExtractor`, `HasExtractorRef`, `HasExtractorMut`, `FinalizeExtract`, and `FinalizeExtractResult` — are all defined in [crates/core/cgp-field/src/traits/extract_field.rs](../../../crates/core/cgp-field/src/traits/extract_field.rs), with `PartialData` in [partial_data.rs](../../../crates/core/cgp-field/src/traits/partial_data.rs). The `MapType` markers are in [crates/core/cgp-field/src/impls/map_type.rs](../../../crates/core/cgp-field/src/impls/map_type.rs) and the `MapTypeRef` markers in [map_type_ref.rs](../../../crates/core/cgp-field/src/impls/map_type_ref.rs). Tests are in [crates/tests/cgp-tests/tests/extensible_data_tests/variants/](../../../crates/tests/cgp-tests/tests/extensible_data_tests/variants/).
