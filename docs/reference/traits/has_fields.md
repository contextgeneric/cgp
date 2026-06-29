# `HasFields`

`HasFields` is the consumer trait that exposes a type's complete field structure as a single type-level shape — a [`Product`](../macros/product.md) for a struct, a [`Sum`](../macros/sum.md) for an enum — with `HasFieldsRef` giving the borrowed view and `ToFields`, `FromFields`, and `ToFieldsRef` providing the round-trip conversions between a concrete value and that shape.

## Purpose

`HasFields` exists to describe an entire type's shape as one type rather than one entry per field. Where [`HasField<Tag>`](has_field.md) answers "give me *this* field by name," `HasFields` answers "describe *all* the fields at once": it produces a single associated `Fields` type that is the type-level list of every field, each tagged with its name. This aggregate view is what lets generic code fold over a context's shape uniformly — serialization, builders, conversions, and the extensible-record and extensible-variant machinery all operate on the `Fields` type rather than reaching for fields one at a time.

The distinction from `HasField` is the whole point. `HasField` is indexed access through many small impls, used for dependency injection where a provider wants one specific value. `HasFields` is the structural mirror — a single impl whose `Fields` type enumerates the full shape, used when an algorithm must process the type as a record or as a tagged union. The two are complementary, and a type that wants both indexed access and structural processing derives both with [`#[derive(HasField, HasFields)]`](../derives/derive_has_fields.md).

The conversion traits make that structural view a two-way door. `ToFields` turns an owned value into its `Fields` shape, `FromFields` rebuilds the value from the shape, and `ToFieldsRef` borrows the value as a shape of references. Together they let generic code decompose a concrete type into its anonymous structural form, transform it, and reconstruct the concrete type.

## Definition

The two core traits each carry a single associated type and nothing else. `HasFields` names the owned shape, and `HasFieldsRef` names the borrowed shape, parameterized by a lifetime:

```rust
pub trait HasFields {
    type Fields;
}

pub trait HasFieldsRef {
    type FieldsRef<'a>
    where
        Self: 'a;
}
```

For a struct, `Fields` is a [`Product`](../macros/product.md) — a `Cons`/`Nil` chain of [`Field<Tag, Value>`](../types/field.md) entries, each value carrying its type-level name tag. For an enum, `Fields` is a [`Sum`](../macros/sum.md) — an `Either`/`Void` chain whose arms are `Field<Tag, ...>` entries naming each variant and carrying that variant's own fields as a nested product. Named fields and variants are tagged by [`Symbol!`](../macros/symbol.md); tuple fields by `Index<N>`. `FieldsRef<'a>` is the same shape with each value borrowed for `'a`.

The three conversion traits each supertrait one of the two shape traits and add a single method. `ToFields` and `FromFields` build on `HasFields`, while `ToFieldsRef` builds on `HasFieldsRef`:

```rust
pub trait ToFields: HasFields {
    fn to_fields(self) -> Self::Fields;
}

pub trait FromFields: HasFields {
    fn from_fields(fields: Self::Fields) -> Self;
}

pub trait ToFieldsRef: HasFieldsRef {
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a;
}
```

`to_fields` consumes the value to produce the owned product or sum, `from_fields` consumes the shape to reconstruct the value, and `to_fields_ref` borrows the value to produce a shape of references — the non-consuming counterpart used when the original value must be kept.

## Behavior

All five impls come from [`#[derive(HasFields)]`](../derives/derive_has_fields.md); the trait module defines only the bare trait shapes. The derive dispatches on whether the type is a struct or an enum: a struct produces a `Product!` of its fields, an enum produces a `Sum!` of its variants, and a single-field tuple struct (a newtype) is treated specially so its `Fields` is the inner type directly rather than a one-element product.

The conversions are mechanical inverses of one another. `to_fields` folds the concrete value into a `Cons` chain (or an `Either` arm for an enum), `from_fields` pattern-matches that chain back into the concrete fields, and `to_fields_ref` builds the same `Cons` chain over borrows. Because `from_fields` and `to_fields` round-trip through the identical `Fields` type, generic code can decompose a value, operate on the structural form, and rebuild the value with the guarantee that the shapes line up by construction.

## Examples

Deriving `HasFields` lets generic code treat any type as a record without naming its concrete type, and a value can be round-tripped through its `Fields` shape:

```rust
use cgp::prelude::*;

#[derive(HasField, HasFields)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

let config = Config { host: "localhost".into(), port: 8080 };

// ToFields: Config -> Product![Field<Symbol!("host"), String>, Field<Symbol!("port"), u16>]
let fields = config.to_fields();

// FromFields: the product -> back to Config
let config_again = Config::from_fields(fields);
```

For an enum the `Fields` shape is a sum instead of a product. A `Shape` enum with `Circle { radius: f64 }` and `Rectangle { width: f64, height: f64 }` variants produces a `Fields` of `Either<Field<Symbol!("Circle"), Product![...]>, Either<Field<Symbol!("Rectangle"), Product![...]>, Void>>`, and the same `to_fields`/`from_fields` pair moves a `Shape` value in and out of that sum. Generic algorithms bound `Context: HasFields` (or `ToFields`/`FromFields`) and process `Context::Fields` structurally, which is how the extensible-data machinery operates over arbitrary contexts.

## Related constructs

`HasFields` is the structural counterpart to [`HasField`](has_field.md): `HasField` gives indexed, single-field access for dependency injection, while `HasFields` gives the whole-shape view. All five impls are generated by [`#[derive(HasFields)]`](../derives/derive_has_fields.md). The `Fields` shape is built from [`Product`](../macros/product.md) for structs and [`Sum`](../macros/sum.md) for enums, with each entry a [`Field<Tag, Value>`](../types/field.md) tagged by [`Symbol!`](../macros/symbol.md).

## Source

The trait definitions are in [crates/core/cgp-field/src/traits/has_fields.rs](../../../crates/core/cgp-field/src/traits/has_fields.rs) (`HasFields`, `HasFieldsRef`), [crates/core/cgp-field/src/traits/to_fields.rs](../../../crates/core/cgp-field/src/traits/to_fields.rs) (`ToFields`, `ToFieldsRef`), and [crates/core/cgp-field/src/traits/from_fields.rs](../../../crates/core/cgp-field/src/traits/from_fields.rs) (`FromFields`). The `Field`, `Cons`/`Nil`, `Either`/`Void` building blocks live under [crates/core/cgp-field/src/types/](../../../crates/core/cgp-field/src/types/). The derive that emits the impls is in [crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/); expansion snapshots are in [crates/tests/cgp-macro-tests](../../../crates/tests/cgp-macro-tests).
