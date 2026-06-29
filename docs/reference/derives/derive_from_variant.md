# `#[derive(FromVariant)]`

`#[derive(FromVariant)]` derives just the variant-construction machinery for an enum: one `FromVariant` impl per variant, so the enum can be built generically from any single variant addressed by name.

## Purpose

`#[derive(FromVariant)]` lets generic code construct an enum from one of its variants without naming the concrete variant constructor. It is the smallest building block of the extensible-variant family — the *construction* counterpart to the extractor's deconstruction. Where a plain `Shape::Circle(circle)` hard-codes both the enum and the variant, `FromVariant` lets a generic provider write `Shape::from_variant(tag, value)` with the variant chosen by a type-level `Symbol!` tag, so the same code can target any variant of any enum that derives it.

The derive exists as a standalone because variant construction is sometimes wanted on its own — for example, by the casts and dispatchers that build a value into a target enum after matching. Most code, however, reaches for [`#[derive(CgpVariant)]`](derive_cgp_variant.md) or [`#[derive(CgpData)]`](derive_cgp_data.md), which include these impls alongside the extractor and the field representation. This is the simplest derive in the family: it generates no companion types and no presence tracking, only a direct constructor per variant.

## Syntax

The derive is applied to an enum and takes no arguments:

```rust
use cgp::prelude::*;

#[derive(FromVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

Each variant's name becomes a type-level string `Symbol!` used as the variant's `Tag`, and its payload type becomes the constructor's value type. Every variant must carry exactly one unnamed payload — a single-field tuple variant such as `Circle(Circle)`; a fieldless, multi-field, or struct-style variant is a compile error. Generic parameters on the enum are carried onto the generated impls. The derive emits exactly the `FromVariant` impls that the variant path of [`#[derive(CgpData)]`](derive_cgp_data.md) emits — that slice in isolation.

## Expansion

`#[derive(FromVariant)]` expands into one [`FromVariant`](../traits/from_variant.md) impl per variant and nothing else. The symbols below are abbreviated as `Symbol!("Name")` in place of the full `Symbol<Len, Chars<...>>` form. Starting from:

```rust
#[derive(FromVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

the derive emits a constructor keyed by each variant's name symbol, with the payload type as the associated `Value`:

```rust
impl FromVariant<Symbol!("Circle")> for Shape {
    type Value = Circle;
    fn from_variant(_tag: PhantomData<Symbol!("Circle")>, value: Self::Value) -> Self {
        Self::Circle(value)
    }
}

impl FromVariant<Symbol!("Rectangle")> for Shape {
    type Value = Rectangle;
    fn from_variant(_tag: PhantomData<Symbol!("Rectangle")>, value: Self::Value) -> Self {
        Self::Rectangle(value)
    }
}
```

The `FromVariant` trait is defined in the field crate; the derive only supplies the per-variant impls. There is no partial type, no `MapType` marker, and no state tracking — each impl simply wraps the value in its variant. The `Tag` is the variant name's type-level string, and the `PhantomData<Tag>` argument exists solely to let the caller pick which variant to build when several `FromVariant` impls are in scope.

## Examples

`FromVariant` lets a value be lifted into an enum generically by naming the variant with a tag:

```rust
use cgp::prelude::*;

#[derive(FromVariant)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn wrap_circle(circle: Circle) -> Shape {
    Shape::from_variant(PhantomData::<Symbol!("Circle")>, circle)
}
```

The call is equivalent to `Shape::Circle(circle)`, but the variant is selected by the type-level tag, so generic code that is parameterized over the tag can construct whichever variant it was asked for.

## Related constructs

`#[derive(FromVariant)]` is one slice of the variant output of [`#[derive(CgpData)]`](derive_cgp_data.md) and [`#[derive(CgpVariant)]`](derive_cgp_variant.md); those derives include it alongside the [`#[derive(ExtractField)]`](derive_extract_field.md) extractor and the [`#[derive(HasFields)]`](derive_has_fields.md) representation traits. Its closest relative is [`ExtractField`](../traits/extract_field.md), the reverse operation that takes a variant out rather than putting one in. For structs, the analogous field-setting building block is [`#[derive(BuildField)]`](derive_build_field.md). The generated constructors correspond to the arms of the enum's [`sum`](../macros/sum.md) representation ([`Either`/`Void`](../types/either.md)).

## Known issues

The derive only accepts enums whose every variant is a single-field tuple variant. A fieldless variant like `Empty`, a multi-field variant like `Pair(A, B)`, or a struct-style variant like `Named { x: A }` causes the macro to fail with the error "Expected variant to contain exactly one unnamed field." There is no way to opt a variant out of the requirement, so an enum that mixes shapes cannot derive the constructor at all.

## Source

The derive entry point is `derive_from_variant` in [crates/macros/cgp-macro-lib/src/derive_from_variant.rs](../../../crates/macros/cgp-macro-lib/src/derive_from_variant.rs), which builds an `ItemCgpVariant` and calls `to_from_variant_impls()`. That method, in [crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs), delegates to `derive_from_variant_from_enum` in [crates/macros/cgp-macro-core/src/types/cgp_data/derive_from_variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_from_variant.rs). The `FromVariant` trait is in [crates/core/cgp-field/src/traits/from_variant.rs](../../../crates/core/cgp-field/src/traits/from_variant.rs). Tests are in [crates/tests/cgp-tests/tests/extensible_data_tests/variants/](../../../crates/tests/cgp-tests/tests/extensible_data_tests/variants/).
