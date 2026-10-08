//! The delimiter a `Struct!` is invoked with does not matter: a proc macro
//! never sees it, so the named or tuple form is read from the entries. The
//! conventional spelling is braces for named fields and parentheses for a tuple
//! body, and that is the form cargo-cgp prints, but each body gives the same
//! type under any delimiter.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/struct.md.

use cgp_fork::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

#[test]
fn a_named_body_is_the_same_type_under_any_delimiter() {
    assert_same_type::<Struct! { a: u8, b: u16 }, Struct!(a: u8, b: u16)>();
    assert_same_type::<Struct! { a: u8, b: u16 }, Struct![a: u8, b: u16]>();
}

#[test]
fn a_tuple_body_is_the_same_type_under_any_delimiter() {
    assert_same_type::<Struct!(u8, bool), Struct! { u8, bool }>();
    assert_same_type::<Struct!(u8, bool), Struct![u8, bool]>();
}
