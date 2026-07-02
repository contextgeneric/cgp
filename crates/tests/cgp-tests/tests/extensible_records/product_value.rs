//! The value-level `product!` macro: building a `Cons`/`Nil` value whose type is
//! the matching `Product!`.
//!
//! `product!` parses its items as expressions, not types, so an element may be any
//! expression — a literal, a method call, an arithmetic expression — and not just a
//! path that happens to also parse as a type. This test pins that the value it
//! builds is exactly the nested `Cons(..)`/`Nil` constructor form, that its type is
//! the corresponding `Product!`, and that the empty and trailing-comma forms work.
//!
//! See docs/implementation/entrypoints/product.md and
//! docs/reference/macros/product.md.

use cgp::prelude::*;

#[test]
fn test_product_value() {
    // Elements are expressions: a literal, a method call, and an arithmetic
    // expression — none of which parse as a type.
    let row: Product![u32, String, bool] = product![2 + 3, "hi".to_owned(), true];

    assert_eq!(row, Cons(5, Cons("hi".to_owned(), Cons(true, Nil))));

    // The nested spine is addressable field-by-field through the tuple-struct
    // fields of `Cons`.
    assert_eq!(row.0, 5);
    assert_eq!(row.1.0, "hi");
    assert!(row.1.1.0);
}

#[test]
fn test_empty_product_value() {
    // An empty `product![]` is `Nil`, the value of the empty `Product![]`.
    let empty: Product![] = product![];
    assert_eq!(empty, Nil);
}

#[test]
fn test_product_value_trailing_comma() {
    let row: Product![u32, u32] = product![1, 2,];
    assert_eq!(row, Cons(1, Cons(2, Nil)));
}
