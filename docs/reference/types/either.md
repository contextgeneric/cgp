# `Either` and `Void`

`Either<Head, Tail>` and `Void` are the two cells of CGP's sum spine — a recursive, right-nested type-level choice that generic code walks to handle any enum's variants one branch at a time.

## Purpose

`Either` and `Void` exist to represent a choice among several types as a single type, so that the variants of an enum can be reasoned about generically. Where the product spine ([`Cons`/`Nil`](./cons.md)) holds a value for *every* element at once, the sum spine holds a value for exactly *one* of its branches — a tagged union, or *anonymous sum type*. By branching at each step and terminating with an uninhabited marker, CGP builds a coproduct whose structure a provider can walk without knowing the concrete enum it came from.

The sum spine is what makes structural, variant-by-variant operations possible across every enum uniformly. Because an enum's variants are exposed as one sum type through [`HasFields`](../traits/has_fields.md), a provider written once to recurse over the `Either`/`Void` branches can match, dispatch on, or construct *any* enum's variants. This is the basis for CGP's extensible-variant machinery: each variant is reached by walking the nested branches rather than by writing a hand-rolled `match` against a fixed enum.

`Either` and `Void` are the building blocks; the [`Sum!`](../macros/sum.md) macro is how a programmer writes a chain of them. `Sum![A, B, C]` is sugar for the right-nested `Either` chain terminated by `Void`. The branches are most often [`Field`](field.md) entries pairing a variant name with its payload, so an enum's shape becomes a `Sum!` of `Field` branches over this spine.

## Definition

`Either` is a two-case enum selecting the head or deferring to the rest of the chain, and `Void` is an empty enum that can never be constructed:

```rust
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Either<Head, Tail> {
    Left(Head),
    Right(Tail),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Void {}
```

The `Head` parameter is the type of the current branch and `Tail` is the rest of the chain — itself another `Either` or, at the end, `Void`. The `Left(Head)` case carries a value of the head type; the `Right(Tail)` case carries a value belonging somewhere further down the chain. `Void` has no variants, so it has no values: it is uninhabited. Both types derive `Eq`, `PartialEq`, `Debug`, and `Clone`, so a sum of values that implement these traits inherits them.

## Behavior

A sum of any width is an `Either` chain ending in `Void`, nested to the right. The type `Sum![A, B, C]` is `Either<A, Either<B, Either<C, Void>>>`, and the empty `Sum![]` is just `Void`. A value selects exactly one branch by how deep it sits: `Left(a)` is an `A`, `Right(Left(b))` is a `B`, and `Right(Right(Left(c)))` is a `C`. Reaching the `Void` position would mean the value matched none of the listed branches — which is impossible, because `Void` has no values, so the chain is closed off at its end.

Generic code consumes the sum by recursing on its two cases, mirroring how it folds the product spine but branching instead of pairing. A `Left` is handled directly as the head; a `Right` defers to a trait impl on the `Tail`, recursing until a `Left` is found. The base case is the `Void` terminator, and here the difference from the product spine matters: a product ends in [`Nil`](cons.md), a constructible empty record, but a sum ends in the *uninhabited* `Void`, because an empty choice has no value to pick. `Void` is functionally the never type, used specifically to mark the end of a sum.

The uninhabitedness of `Void` is load-bearing for the extractor machinery, where [`FinalizeExtract`](../traits/extract_field.md) relies on it to discharge the impossible remainder. After an extractor has tried every variant of a sum and matched none, the leftover value has type `Void` — a value that cannot exist. `FinalizeExtract for Void` turns that into any required type with an empty `match self {}`, since there are no cases to handle, so a fully-handled sum extraction is total at compile time with no unreachable runtime branch. A constructible terminator like `Nil` could not be discharged this way; only an uninhabited type lets the compiler accept the empty match.

## Examples

The sum spine appears most visibly as the `Fields` of an enum that derives [`HasFields`](../derives/derive_has_fields.md), where the [`Sum!`](../macros/sum.md) sugar hides the `Either`/`Void` chain:

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
//     // i.e. Either<Field<Symbol!("Circle"), f64>,
//     //          Either<Field<Symbol!("Rectangle"), _>, Void>>
// }
```

A standalone sum type can also be written directly, and a value picks one branch by its nesting depth:

```rust
use cgp::prelude::*;

type Token = Sum![u32, String, bool];
// Token == Either<u32, Either<String, Either<bool, Void>>>

let t: Token = Either::Right(Either::Left("hi".to_string())); // the String branch
```

## Related constructs

`Either`/`Void` are the sum (choice-like) spine; their product (record-like) counterpart is [`Cons`/`Nil`](./cons.md), which shares the same right-nested shape but pairs a head with the rest at each step and terminates in the constructible `Nil` rather than the uninhabited `Void`. A chain of these cells is written with the [`Sum!`](../macros/sum.md) macro, and its branches are usually [`Field`](field.md) entries whose tags come from [`Symbol!`](../macros/symbol.md). The whole chain is what [`#[derive(HasFields)]`](../derives/derive_has_fields.md) assigns to an enum and what [`HasFields`](../traits/has_fields.md) exposes, and `Void`'s uninhabitedness is what the `FinalizeExtract` part of the [extractor family](../traits/extract_field.md) depends on to close a total variant match.

## Source

- `Either<Head, Tail>` and `Void` are both defined in [crates/core/cgp-field/src/types/sum.rs](../../../crates/core/cgp-field/src/types/sum.rs).
- The `Sum!` macro that folds element types onto this spine is the `SumType` construct in [crates/macros/cgp-macro-core/src/types/sum.rs](../../../crates/macros/cgp-macro-core/src/types/sum.rs).
- `FinalizeExtract for Void`, which discharges the uninhabited remainder of a variant extraction, lives in [crates/core/cgp-field/src/traits/extract_field.rs](../../../crates/core/cgp-field/src/traits/extract_field.rs), and the enum `HasFields` derive that emits a `Sum!` of `Field` branches is under [crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/sum.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/sum.rs).
