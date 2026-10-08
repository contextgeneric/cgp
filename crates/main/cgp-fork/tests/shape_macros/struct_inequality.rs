//! Shapes that look alike are distinct types. Field order is part of the type,
//! a positional field and a named field are keyed differently, a tuple body is
//! not the plain list of its types, and only a single *positional* field is
//! unwrapped to its bare type.
//!
//! The compiler cannot assert that two types differ, so these compare `TypeId`s
//! at run time.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/struct.md.

use cgp_fork::prelude::*;

use crate::shape_macros::assertions::assert_type_ne;

#[test]
fn field_order_is_part_of_the_type() {
    assert_type_ne::<Struct! { a: u8, b: u16 }, Struct! { b: u16, a: u8 }>();
    assert_type_ne::<Struct!(u8, u16), Struct!(u16, u8)>();
}

#[test]
fn positional_and_named_fields_are_keyed_differently() {
    assert_type_ne::<Struct!(u8, u16), Struct! { a: u8, b: u16 }>();
    assert_type_ne::<Struct!(u64), Struct! { value: u64 }>();
}

#[test]
fn a_tuple_body_is_not_the_plain_list_of_its_types() {
    assert_type_ne::<Struct!(u8, u16), Product![u8, u16]>();
}

#[test]
fn only_a_single_positional_field_is_unwrapped() {
    assert_type_ne::<Struct! { value: u64 }, u64>();
    assert_type_ne::<Struct!(u64), Product![u64]>();
}
