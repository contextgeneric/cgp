# `#[derive(CgpVariant)]`

`#[derive(CgpVariant)]` derives the extensible-data machinery for an enum: the type-level field representation, variant constructors, and an incremental extractor, so the enum can be enumerated, built from a single variant, and matched one variant at a time.

## Purpose

`#[derive(CgpVariant)]` makes an enum into an *extensible variant* — a sum of named variants that generic CGP code can address, construct, and take apart one variant at a time. A plain Rust enum is opaque to generic code: there is no way to refer to "the `Circle` variant" or "an enum with these variants" through a type parameter. This derive exposes the enum's variants as type-level data so that generic providers — variant dispatchers, the `upcast`/`downcast` casts, match handlers — can operate over any enum uniformly.

The derive is the enum-specific face of [`#[derive(CgpData)]`](derive_cgp_data.md). When `CgpData` is applied to an enum it emits exactly what `CgpVariant` emits; the two are the same code path. Use `CgpVariant` when the type is always an enum and you want the name to say so. Using `CgpVariant` on a struct is a type error, since it parses its input as an enum.

The defining capability a variant gains is incremental extraction. Beyond constructing the enum from any single variant, the derive generates a *partial* companion enum that tracks, in its type parameters, which variants are still possible. Generic code can convert a value to its extractor and pull one variant out; on a match the value is returned, and on a miss the *remainder* — a partial enum with that variant marked impossible — is returned for the next attempt. When every variant has been ruled out the remainder becomes an empty type that can be discharged, so an exhaustive match is proven at the type level.

## Syntax

The derive is applied to an enum and takes no arguments:

```rust
use cgp::prelude::*;

#[derive(CgpVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

Each variant is expected to carry a single payload (a newtype variant); its name becomes a type-level string `Symbol!` used as the variant's `Tag`, and its payload type becomes the variant's value type. Generic parameters on the enum are carried onto the generated impls. The derive accepts the same enums that [`#[derive(CgpData)]`](derive_cgp_data.md) accepts for the variant path; the only difference is that `CgpVariant` refuses non-enum inputs outright.

## Expansion

`#[derive(CgpVariant)]` expands into three groups of impls: the representation traits, the variant constructors, and the extractor machinery. The symbols below are abbreviated as `Symbol!("Name")` in place of the full `Symbol<Len, Chars<...>>` spelling the macro actually emits. Starting from:

```rust
#[derive(CgpVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

the derive first emits the representation traits, exposing the enum as a type-level sum of named [`Field`](../types/field.md) entries terminated by [`Void`](../types/either.md), with whole-value conversions — the [`#[derive(HasFields)]`](derive_has_fields.md) output for enums:

```rust
impl HasFields for Shape {
    type Fields = Either<
        Field<Symbol!("Circle"), Circle>,
        Either<Field<Symbol!("Rectangle"), Rectangle>, Void>,
    >;
}

impl FromFields for Shape { /* match each Either arm back to a variant */ }
// plus ToFields, HasFieldsRef, ToFieldsRef
```

Second, it emits one [`FromVariant`](../traits/from_variant.md) impl per variant, so the enum can be constructed generically from any single variant by name:

```rust
impl FromVariant<Symbol!("Circle")> for Shape {
    type Value = Circle;
    fn from_variant(_: PhantomData<Symbol!("Circle")>, value: Circle) -> Self { Self::Circle(value) }
}
// plus FromVariant<Symbol!("Rectangle")>
```

Third, it emits the extractor machinery — the [`#[derive(ExtractField)]`](derive_extract_field.md) output. This centers on two partial companion enums: `__PartialShape` for owned extraction and `__PartialRefShape` for borrowed extraction. Each arm's payload is wrapped in a `MapType` marker, so a variant can be present (`IsPresent`) or ruled out (`IsVoid`):

```rust
pub enum __PartialShape<__F0__: MapType, __F1__: MapType> {
    Circle(<__F0__ as MapType>::Map<Circle>),
    Rectangle(<__F1__ as MapType>::Map<Rectangle>),
}

impl HasExtractor for Shape {
    type Extractor = __PartialShape<IsPresent, IsPresent>;     // all variants still possible
    fn to_extractor(self) -> Self::Extractor { /* map each variant across */ }
    fn from_extractor(extractor: Self::Extractor) -> Self { /* map back */ }
}

impl FinalizeExtract for __PartialShape<IsVoid, IsVoid> {       // no variants left -> empty
    fn finalize_extract<__T__>(self) -> __T__ { match self {} }
}
```

It then emits, per variant, an `ExtractField` impl available only when that variant's marker is `IsPresent`; calling it returns `Ok(value)` if the runtime value is that variant, or `Err(remainder)` where the remainder has that variant's marker flipped to `IsVoid`. The borrowed `__PartialRefShape` enum carries an extra `MapTypeRef` parameter and backs `HasExtractorRef`/`HasExtractorMut`. The full per-variant detail is documented in [`#[derive(ExtractField)]`](derive_extract_field.md).

The single most important fact about the expansion is that *possibility is encoded in the type*. `to_extractor` yields `__PartialShape<IsPresent, IsPresent>`; each failed `extract_field` returns a remainder with one more `IsVoid`; and once every marker is `IsVoid` the value inhabits `FinalizeExtract`, which can produce any type because the enum is uninhabited. This is how a chain of `extract_field` calls becomes a provably exhaustive match. The partial type names are the reserved `__Partial{Name}` and `__PartialRef{Name}`.

## Examples

A variant is most useful when matching generically with a guaranteed-exhaustive fallthrough. Because `Shape` derives the variant machinery, the match can be expressed as a chain of `extract_field` calls, with `finalize_extract_result` discharging the impossible remainder:

```rust
use cgp::prelude::*;
use cgp::core::field::traits::FinalizeExtractResult;

#[derive(CgpVariant)]
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
                .finalize_extract_result();   // remainder is now empty; this cannot fail
            rect.width * rect.height
        }
    }
}
```

Because each `extract_field` narrows the remainder type, the compiler knows after the second extraction that no variants remain, so `finalize_extract_result` is accepted without a wildcard arm.

## Related constructs

`#[derive(CgpVariant)]` is the enum restriction of [`#[derive(CgpData)]`](derive_cgp_data.md), which dispatches to this same path; [`#[derive(CgpRecord)]`](derive_cgp_record.md) is the struct counterpart. Its output decomposes into [`#[derive(HasFields)]`](derive_has_fields.md) (the representation traits), [`#[derive(FromVariant)]`](derive_from_variant.md) (the variant constructors), and [`#[derive(ExtractField)]`](derive_extract_field.md) (the incremental extractor) — derive those individually when you need only one slice. The generated types reference [`Field`](../types/field.md), the [`sum`](../macros/sum.md) type-level list (`Either`/`Void`), and the `MapType` markers `IsPresent`/`IsVoid`.

## Source

- Entry point: `derive_cgp_variant` in [crates/macros/cgp-macro-lib/src/cgp_variant.rs](../../../crates/macros/cgp-macro-lib/src/cgp_variant.rs), which parses an `ItemCgpVariant` and calls `to_items()`.
- Variant codegen: [crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs), which composes `derive_has_fields_impls_from_enum`, `derive_from_variant_from_enum`, and the extractor helpers in the `derive_extractor/` submodule.
- Runtime traits: `HasExtractor`, `HasExtractorRef`, `HasExtractorMut`, `ExtractField`, `FinalizeExtract`, `FromVariant`, `HasFields`, `FromFields`, `ToFields` in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/).
- Internal walkthrough (the codegen it composes, the corner-case handling, and the index of tests and expansion snapshots): [implementation/entrypoints/derive_cgp_variant.md](../../implementation/entrypoints/derive_cgp_variant.md).
