# `Cons` and `Nil`

`Cons<Head, Tail>` and `Nil` are the two cells of CGP's product spine — a recursive, right-nested type-level list that generic code folds over to handle any struct's fields one at a time.

## Purpose

`Cons` and `Nil` exist to represent an ordered sequence of types as a single type, so that a collection of fields can be reasoned about generically. A plain Rust tuple holds several things at once but cannot be taken apart by generic code element by element; a recursive list can. By pairing a head with the rest of the list and terminating with an empty marker, CGP builds an *anonymous product type* — a record-shaped value whose structure a provider can walk without knowing the concrete struct it came from.

The product spine is what makes structural, field-by-field operations possible across every struct uniformly. Because a struct's fields are exposed as one list type through [`HasFields`](../traits/has_fields.md), a provider written once to recurse over `Cons`/`Nil` can iterate, transform, read, or rebuild *any* struct's fields. Each step handles the `Head`, then recurses into the `Tail`, until it reaches `Nil` and stops. This recursion over a fixed two-case shape — a pair or the empty list — is the mechanism behind builders, extractors, and field mappers alike.

`Cons` and `Nil` are the constructible building blocks; the [`Product!`](../macros/product.md) macro is how a programmer writes a list of them. `Product![A, B, C]` is sugar for the right-nested `Cons` chain, and the value macro `product![a, b, c]` builds a matching value. The list elements are most often [`Field`](field.md) entries pairing a name with a value, so a struct's layout becomes a `Product!` of `Field` cells over this same spine.

## Definition

`Cons` is a tuple struct holding the first element and the rest of the list, and `Nil` is a unit struct marking the end:

```rust
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Cons<Head, Tail>(pub Head, pub Tail);

#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Nil;
```

The `Head` parameter is the first element's type and the `Tail` parameter is the rest of the list — itself another `Cons` or, at the end, `Nil`. Both positional fields are public, so `Cons(head, tail)` constructs a cell and `.0`/`.1` reach its parts. `Nil` carries no data; used as a `Tail` it terminates the chain, and used on its own it is the empty list. Both derive `Eq`, `PartialEq`, `Clone`, `Default`, and `Debug`, so a list of values that themselves implement these traits inherits them structurally — equality compares head-to-head down the chain, and `Default` yields the all-defaults list.

## Behavior

A list of any length is a `Cons` chain ending in `Nil`, nested to the right. The type `Product![A, B, C]` is `Cons<A, Cons<B, Cons<C, Nil>>>`, and the empty `Product![]` is just `Nil`. The corresponding value is built with the tuple-struct constructor, `Cons(a, Cons(b, Cons(c, Nil)))`, so a `product!` value is an ordinary owned value whose type is exactly what `Product!` produces over the same elements' types. Because `Cons` and `Nil` are real structs, nothing about the list is virtual or boxed; the whole structure is flat data laid out by nesting.

Generic code consumes the spine by recursing on its two cases. A trait implemented for `Nil` supplies the base case — the empty list — and a blanket impl for `Cons<Head, Tail>` supplies the recursive step, typically constraining `Tail` to implement the same trait so the recursion bottoms out at `Nil`. This pairing of a `Nil` impl with a `Cons<Head, Tail>` impl is the standard shape for any operation that folds over a product, and it is how the field machinery processes a struct of arbitrary width without per-field code.

## Examples

The product spine appears most visibly as the `Fields` of a struct that derives [`HasFields`](../derives/derive_has_fields.md), where the [`Product!`](../macros/product.md) sugar hides the `Cons`/`Nil` chain:

```rust
use cgp::prelude::*;

#[derive(HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// generated:
// impl HasFields for Person {
//     type Fields = Product![
//         Field<Symbol!("name"), String>,
//         Field<Symbol!("age"), u8>,
//     ];
//     // i.e. Cons<Field<Symbol!("name"), String>,
//     //          Cons<Field<Symbol!("age"), u8>, Nil>>
// }
```

A standalone list type and a matching value can also be written directly, which expands to the nested `Cons` form:

```rust
use cgp::prelude::*;

type Row = Product![u32, String, bool];
let row: Row = product![1, "hi".to_string(), true];
// Row == Cons<u32, Cons<String, Cons<bool, Nil>>>
// row == Cons(1, Cons("hi".to_string(), Cons(true, Nil)))
```

## Related constructs

`Cons`/`Nil` are the product (record-like) spine; their sum (choice-like) counterpart is [`Either`/`Void`](./either.md), which shares the same right-nested shape but branches at each step and terminates in the uninhabited `Void` rather than the constructible `Nil`. A list of these cells is written with the [`Product!`](../macros/product.md) macro, and its elements are usually [`Field`](field.md) entries whose tags come from [`Symbol!`](../macros/symbol.md) or [`Index`](index.md). The whole list is what [`#[derive(HasFields)]`](../derives/derive_has_fields.md) assigns to a struct and what [`HasFields`](../traits/has_fields.md) exposes. The `Chars` list inside [`Symbol!`](../macros/symbol.md) is a specialized version of this same spine, with a `const char` head in place of a type.

## Source

`Cons<Head, Tail>` is defined in [crates/core/cgp-base-types/src/types/cons.rs](../../../crates/core/cgp-base-types/src/types/cons.rs) and `Nil` in [crates/core/cgp-base-types/src/types/nil.rs](../../../crates/core/cgp-base-types/src/types/nil.rs). The `Product!`/`product!` macros that fold elements onto this spine are driven by the constructs under [crates/macros/cgp-macro-core/src/types/product/](../../../crates/macros/cgp-macro-core/src/types/product/), and the `HasFields` machinery that recurses over it lives in [crates/core/cgp-field/src/traits/has_fields.rs](../../../crates/core/cgp-field/src/traits/has_fields.rs).
