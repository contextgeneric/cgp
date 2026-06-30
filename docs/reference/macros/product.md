# `Product!` and `product!`

`Product![A, B, C]` is the type macro that builds a type-level list — a heterogeneous list of types encoded entirely in the type system — and the lowercase `product![a, b, c]` is its value-level counterpart that builds an actual value of that type.

## Purpose

`Product!` exists to represent an ordered sequence of types as a single type, so that a collection of fields can be reasoned about generically. CGP uses this to describe the *shape* of a struct: the list of its fields, in order, as one type. A type-level list is sometimes called an anonymous product type, because like a tuple it holds several things at once, but unlike a tuple it is built from a recursive `Cons`/`Nil` spine that generic code can take apart one element at a time.

The list is what makes structural, field-by-field operations possible. Because the fields of a struct are exposed as a single list type through [`HasFields`](../traits/has_fields.md), a provider can be written once to iterate, transform, or rebuild *any* struct's fields without knowing the concrete struct, by recursing over the `Cons`/`Nil` structure. A plain tuple cannot be decomposed this way in generic code; the recursive list can.

`Product!` and `product!` are two halves of the same idea, split across the type and value levels. `Product!` produces a *type* and is used in type position — associated types, bounds, `type` aliases. `product!` produces a *value* of a matching type and is used in expression position. The uppercase/lowercase convention mirrors Rust's own split between, say, a struct type and a struct literal.

## Syntax

Both macros take a comma-separated list of elements and may be empty. `Product!` takes types and is used wherever a type is expected; `product!` takes expressions and is used wherever a value is expected:

```rust
Product![u32, String, bool]   // a type
product![1u32, "hi".to_string(), true]   // a value of that type
Product![]   // the empty list type
```

The element lists line up positionally, so the value built by `product!` has the type built by `Product!` over the corresponding element types.

## Syntax Grammar

The two macros take a possibly-empty, comma-separated list — of types for `Product!` and of expressions for `product!`:

```ebnf
ProductInput -> ( Type ( `,` Type )* `,`? )?

ProductExpr  -> ( Expression ( `,` Expression )* `,`? )?
```

`ProductInput` is the grammar of the type macro `Product!`, used in type position; `ProductExpr` is the grammar of the value macro `product!`, used in expression position. `Type` and `Expression` are the Rust grammar's productions, and both lists may be empty (`Product![]`, `product![]`) or carry a trailing comma. The element lists line up positionally, so a `product!` value has the type the corresponding `Product!` builds.

## Expansion

`Product!` expands to a right-nested chain of `Cons`, terminated by `Nil`. The three-element list desugars as follows:

```rust
// before
Product![A, B, C]
```

```rust
// after
Cons<A, Cons<B, Cons<C, Nil>>>
```

The two building blocks are defined in `cgp-base-types`. `Cons<Head, Tail>` is a pair holding the first element and the rest of the list as `Cons<Head, Tail>(pub Head, pub Tail)`; chaining it through `Tail` and terminating with the empty `Nil` struct produces a list of any length. An empty `Product![]` is simply `Nil`. The macro constructs the chain by folding the elements from right to left onto `Nil`.

The value macro `product!` expands the same way but produces a value rather than a type, using the tuple-struct constructor of `Cons`:

```rust
// before
product![a, b, c]
```

```rust
// after
Cons(a, Cons(b, Cons(c, Nil)))
```

Because `Cons` is a real tuple struct and `Nil` a real unit struct, the value built by `product!` is an ordinary owned value whose type is exactly what `Product!` builds over the same elements' types.

## Examples

The most common appearance of `Product!` is as the `Fields` of a struct that derives [`HasFields`](../derives/derive_has_fields.md), where each element is a `Field<Tag, Value>` pairing a field name with its type:

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
// }
```

The field names here are type-level strings — see [`Symbol!`](symbol.md) — so the whole `Product!` is a fully type-level description of `Person`'s layout. Generic code can then walk that list to build, read, or transform a `Person` without being written against `Person` specifically.

A standalone list and a matching value can also be written directly:

```rust
type Row = Product![u32, String, bool];
let row: Row = product![1, "hi".to_string(), true];
```

## Related constructs

`Product!` is the product (record-like) counterpart to [`Sum!`](sum.md), which builds the coproduct used for enum variants; the two share the same right-nested shape but `Sum!` terminates in `Void` rather than `Nil`. The list elements are most often [`Field`](../types/field.md) entries whose tags are [`Symbol!`](symbol.md) field names or [`Index`](../types/index.md) positions. The list type as a whole is what [`#[derive(HasFields)]`](../derives/derive_has_fields.md) assigns to a struct, building on the per-field [`#[derive(HasField)]`](../derives/derive_has_field.md). The `Chars` list inside `Symbol!` is a specialized version of this same `Cons`/`Nil` structure.

## Source

The macro entry points are `Product` and `product` in [crates/macros/cgp-macro-lib/src/product.rs](../../../crates/macros/cgp-macro-lib/src/product.rs). The type form is the `ProductType` construct in [crates/macros/cgp-macro-core/src/types/product/product_type.rs](../../../crates/macros/cgp-macro-core/src/types/product/product_type.rs), whose `eval` right-folds the elements with `Cons` onto `Nil`; the value form is `ProductExpr` in [crates/macros/cgp-macro-core/src/types/product/product_expr.rs](../../../crates/macros/cgp-macro-core/src/types/product/product_expr.rs), which does the same fold using the `Cons(..)` constructor. The runtime types are defined in [crates/core/cgp-base-types/src/types/cons.rs](../../../crates/core/cgp-base-types/src/types/cons.rs) (`Cons<Head, Tail>`) and [crates/core/cgp-base-types/src/types/nil.rs](../../../crates/core/cgp-base-types/src/types/nil.rs) (`Nil`).
