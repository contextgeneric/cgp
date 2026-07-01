# `FromVariant`

`FromVariant<Tag>` constructs an enum from a single named variant, with the variant chosen by a type-level tag rather than by writing the concrete variant constructor.

## Purpose

`FromVariant` solves the problem of building an enum value in generic code that does not — and cannot — name the concrete variant constructor. Writing `Shape::Circle(circle)` hard-codes both the enum and the variant, so it only works where both are known statically. `FromVariant` instead selects the variant with a type-level [`Symbol!`](../macros/symbol.md) tag, so a provider parameterized over the tag can construct whichever variant it was asked to build, on whatever enum implements the trait for that tag. It is the construction counterpart to the extractor's deconstruction: where [`ExtractField`](extract_field.md) takes one variant *out* of a value, `FromVariant` puts one variant *in*.

This is the smallest trait of the extensible-variant family. It carries no partial companion type and no presence tracking — there is no state to track when building a single variant — so it is just a direct, tag-selected constructor. It is implemented for an enum by [`#[derive(FromVariant)]`](../derives/derive_from_variant.md), which emits one impl per variant, and it is the construction half that the casts and dispatchers reach for after a match has decided which variant to build.

## Definition

`FromVariant<Tag>` is parameterized by the variant's tag and exposes the variant's payload type as an associated `Value`:

```rust
pub trait FromVariant<Tag> {
    type Value;
    fn from_variant(_tag: PhantomData<Tag>, value: Self::Value) -> Self;
}
```

`Tag` is the variant's name as a type-level string `Symbol!`, `Value` is that variant's payload type, and `from_variant` wraps a payload into the enum as the chosen variant. The `PhantomData<Tag>` argument carries no data; its sole job is to let the caller select which variant to build when several `FromVariant` impls — one per variant — are in scope on the same enum. The trait is implemented once per variant, each impl fixing its own `Tag` and `Value`, so the choice of impl *is* the choice of variant.

## Behavior

Each `FromVariant` impl is a thin wrapper that maps a payload to its variant. The derive emits, for every single-payload variant, an impl keyed by that variant's name symbol with the payload as `Value`, whose `from_variant` simply returns `Self::Variant(value)`. There is no intermediate type, no `MapType` marker, and no validation beyond the type system's own check that the supplied `value` matches the variant's payload type. Because the impls are distinguished only by their `Tag` type parameter, resolving a `from_variant` call comes down to which `Symbol!` the caller names in the `PhantomData` argument — the compiler picks the matching impl and inlines it to the corresponding constructor.

For an enum `Shape { Circle(Circle), Rectangle(Rectangle) }`, the derive produces:

```rust
impl FromVariant<Symbol!("Circle")> for Shape {
    type Value = Circle;
    fn from_variant(_tag: PhantomData<Symbol!("Circle")>, value: Circle) -> Self {
        Self::Circle(value)
    }
}

impl FromVariant<Symbol!("Rectangle")> for Shape {
    type Value = Rectangle;
    fn from_variant(_tag: PhantomData<Symbol!("Rectangle")>, value: Rectangle) -> Self {
        Self::Rectangle(value)
    }
}
```

A call to `Shape::from_variant(PhantomData::<Symbol!("Circle")>, circle)` resolves to the first impl and is exactly `Shape::Circle(circle)`. The benefit appears only in generic code: a function that is itself parameterized over a tag `Tag` with a `C: FromVariant<Tag>` bound can build the variant named by `Tag` without ever mentioning a concrete variant.

## Examples

`FromVariant` lifts a value into an enum by naming the variant with a tag, which a concrete call site can do directly and generic code can do over an abstract `Tag`:

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

The call is equivalent to `Shape::Circle(circle)`, but because the variant is selected by the type-level tag, the same construction pattern works inside code that is generic over the tag and the enum.

## Related constructs

The derive that generates the per-variant impls is [`#[derive(FromVariant)]`](../derives/derive_from_variant.md), whose doc shows the exact expanded code; it is also folded into [`#[derive(CgpVariant)]`](../derives/derive_cgp_variant.md) and [`#[derive(CgpData)]`](../derives/derive_cgp_data.md). The reverse operation is [`ExtractField`](extract_field.md), which deconstructs an enum variant by variant rather than constructing one. The variant tag is a [`Symbol!`](../macros/symbol.md) type-level string, the same kind of tag that keys the field traits. For structs, the analogous field-setting building block is the builder family in [`has_builder`](has_builder.md). The conceptual overview that frames variant construction is [extensible variants](../../concepts/extensible-variants.md), worked through in the [expression interpreter](../../examples/expression-interpreter.md) example, where upcasting a small local enum relies on `FromVariant` to rebuild each variant into the target.

## Source

- The `FromVariant` trait is defined in [crates/core/cgp-field/src/traits/from_variant.rs](../../../crates/core/cgp-field/src/traits/from_variant.rs).
- The per-variant impls are generated by `derive_from_variant` in [crates/macros/cgp-macro-lib/src/derive_from_variant.rs](../../../crates/macros/cgp-macro-lib/src/derive_from_variant.rs), driving `derive_from_variant_from_enum` in [crates/macros/cgp-macro-core/src/types/cgp_data/derive_from_variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_from_variant.rs).
- For how it is generated and the index of tests, see the implementation document [implementation/entrypoints/derive_from_variant.md](../../implementation/entrypoints/derive_from_variant.md).
