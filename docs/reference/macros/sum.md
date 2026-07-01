# `Sum!`

`Sum![A, B, C]` is the type macro that builds a type-level sum type — a coproduct encoded in the type system whose value is exactly one of the listed types — used by CGP to represent the variants of an enum the way [`Product!`](product.md) represents the fields of a struct.

## Purpose

`Sum!` exists to represent a choice among several types as a single type, so that the variants of an enum can be reasoned about generically. Where a [`Product!`](product.md) list holds a value for *every* element at once (a record), a `Sum!` holds a value for exactly *one* element (a tagged union). It is sometimes called an anonymous sum type or coproduct, and it is the structural mirror image of the product: both are right-nested chains over the same kind of spine, but the sum branches at each step instead of pairing.

The sum is what makes structural, variant-by-variant operations possible. Because an enum's variants are exposed as a single sum type through [`HasFields`](../traits/has_fields.md), a provider can be written once to match, dispatch on, or construct *any* enum's variants without knowing the concrete enum, by recursing over the nested branch structure. This is the basis for CGP's extensible-variant machinery, where each variant is handled by walking the chain rather than by writing a hand-rolled `match` against a fixed enum.

`Sum!` is the variant-level analogue of `Product!`, and the two are used together. A struct's fields desugar to a `Product!`; an enum's variants desugar to a `Sum!` of the same `Field<Tag, Value>` entries. Knowing one shape tells you the other.

## Syntax

The macro takes a comma-separated list of types and may be empty. It is used wherever a type is expected:

```rust
Sum![u32, String, bool]
Sum![]   // the empty sum
```

Each listed type is one possible variant of the sum; a value of the sum type carries exactly one of them.

## Syntax Grammar

The input to `Sum!` is a possibly-empty, comma-separated list of types:

```ebnf
SumInput -> ( Type ( `,` Type )* `,`? )?
```

`Type` is the Rust grammar's type production, and the list may be empty (`Sum![]`) or carry a trailing comma. The macro is used in type position, and each listed type is one possible variant of the sum.

## Expansion

`Sum!` expands to a right-nested chain of `Either`, terminated by `Void`. The three-element sum desugars as follows:

```rust
// before
Sum![A, B, C]
```

```rust
// after — readable form
Either<A, Either<B, Either<C, Void>>>
```

The two building blocks are defined in `cgp-field` and differ from the product spine in being branching rather than pairing. `Either<Head, Tail>` is the sum cell, an enum with two cases — `Left(Head)` selects the head type, and `Right(Tail)` defers to the rest of the chain — so a value of `Either<A, Either<B, Either<C, Void>>>` is `Left` for an `A`, `Right(Left(..))` for a `B`, and `Right(Right(Left(..)))` for a `C`. The terminator is `Void`, an empty enum that can never be constructed, which closes the chain off: reaching the `Void` position would mean the value matched none of the listed types, which is impossible. An empty `Sum![]` is therefore just `Void`, a type with no values.

The choice of `Void` rather than `Nil` is the key difference from [`Product!`](product.md). A product terminates in `Nil` because an empty record is a valid, constructible value (the unit-like `Nil`); a sum terminates in `Void` because an empty choice is *uninhabited* — there is no value to pick. `Void` is functionally the never type, used here specifically to mark the end of a sum. The macro constructs the chain by folding the element types from right to left onto `Void`.

## Examples

The primary appearance of `Sum!` is as the `Fields` of an enum that derives [`HasFields`](../derives/derive_has_fields.md), where each branch is a `Field<Tag, Value>` pairing a variant name with its payload:

```rust
use cgp::prelude::*;

#[derive(HasFields)]
pub enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

// generated (schematically):
// impl HasFields for Shape {
//     type Fields = Sum![
//         Field<Symbol!("Circle"), f64>,
//         Field<Symbol!("Rectangle"), Product![
//             Field<Symbol!("width"), f64>,
//             Field<Symbol!("height"), f64>,
//         ]>,
//     ];
// }
```

The variant names are type-level strings — see [`Symbol!`](symbol.md) — and a struct-like variant nests a [`Product!`](product.md) of its own fields, so an enum's full shape is a `Sum!` of variants whose payloads may themselves be `Product!` records. Generic code walks the `Sum!` to dispatch on which variant a value holds, and walks any nested `Product!` to reach that variant's fields.

A standalone sum type can also be written directly:

```rust
type Token = Sum![u32, String, bool];
```

## Related constructs

`Sum!` is the coproduct counterpart to [`Product!`](product.md): the two share a right-nested shape, but `Sum!` branches with [`Either`](../types/either.md) and terminates in the uninhabited `Void`, while `Product!` pairs with [`Cons`](../types/cons.md) and terminates in `Nil`. Its branches are typically [`Field`](../types/field.md) entries whose tags are [`Symbol!`](symbol.md) variant names. The sum type as a whole is what [`#[derive(HasFields)]`](../derives/derive_has_fields.md) assigns to an enum, and it underpins the extensible-variant derives [`#[derive(CgpVariant)]`](../derives/derive_cgp_variant.md) and [`#[derive(FromVariant)]`](../derives/derive_from_variant.md), which build and consume individual `Either` branches. For struct fields, the per-field tags are produced by [`#[derive(HasField)]`](../derives/derive_has_field.md).

## Source

- Entry point: `Sum` in [crates/macros/cgp-macro-lib/src/sum.rs](../../../crates/macros/cgp-macro-lib/src/sum.rs), forwarding to the `SumType` construct in [crates/macros/cgp-macro-core/src/types/sum.rs](../../../crates/macros/cgp-macro-core/src/types/sum.rs), whose `eval` right-folds the element types with `Either` onto `Void`.
- Runtime types: `Either<Head, Tail>` and the uninhabited `Void`, both defined in [crates/core/cgp-field/src/types/sum.rs](../../../crates/core/cgp-field/src/types/sum.rs).
- Enum `HasFields` derive that emits a `Sum!` of `Field<Symbol!("..."), _>` branches: [crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/sum.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/sum.rs).
- Internal walkthrough (the parse-and-`eval` pipeline, the right-fold onto `Void`, and the index of tests): [implementation/entrypoints/sum.md](../../implementation/entrypoints/sum.md).
