//! `Struct!` with a named-field body expands to a `Product!` of `Field` entries
//! keyed by `Symbol!`, in declaration order.
//!
//! Besides the plain expansion, this pins the corner cases of the named form: an
//! empty body is `Nil`, a single named field stays a one-element list (only a
//! single *positional* field is unwrapped), a raw identifier is tagged by its
//! logical name, and field types that contain commas, parentheses, or brackets
//! are kept whole.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use std::collections::HashMap;

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

#[test]
fn named_fields_expand_to_a_symbol_keyed_product() {
    assert_same_type::<
        Struct! { name: String, age: u8 },
        Product![Field<Symbol!("name"), String>, Field<Symbol!("age"), u8>],
    >();

    assert_same_type::<
        Struct! { name: String, age: u8 },
        Cons<Field<Symbol!("name"), String>, Cons<Field<Symbol!("age"), u8>, Nil>>,
    >();
}

#[test]
fn an_empty_body_is_nil() {
    assert_same_type::<Struct! {}, Nil>();
}

#[test]
fn a_trailing_comma_is_accepted() {
    assert_same_type::<Struct! { a: u8, b: u16, }, Struct! { a: u8, b: u16 }>();
}

#[test]
fn a_single_named_field_stays_a_list() {
    assert_same_type::<Struct! { value: u64 }, Product![Field<Symbol!("value"), u64>]>();
}

#[test]
fn a_raw_identifier_is_tagged_by_its_logical_name() {
    assert_same_type::<
        Struct! { r#type: u8, r#match: u16 },
        Product![Field<Symbol!("type"), u8>, Field<Symbol!("match"), u16>],
    >();
}

#[test]
fn a_shape_nests_inside_a_field() {
    assert_same_type::<
        Struct! { inner: Struct! { x: u8 } },
        Product![Field<Symbol!("inner"), Product![Field<Symbol!("x"), u8>]>],
    >();
}

#[test]
fn compound_field_types_are_kept_whole() {
    assert_same_type::<
        Struct! {
            map: HashMap<String, u32>,
            callback: fn(u8, u16) -> u8,
            pair: (u8, u16),
            bytes: [u8; 4],
            text: &'static str,
        },
        Product![
            Field<Symbol!("map"), HashMap<String, u32>>,
            Field<Symbol!("callback"), fn(u8, u16) -> u8>,
            Field<Symbol!("pair"), (u8, u16)>,
            Field<Symbol!("bytes"), [u8; 4]>,
            Field<Symbol!("text"), &'static str>,
        ],
    >();
}

pub trait HasAssoc {
    type Assoc;
}

impl HasAssoc for u8 {
    type Assoc = u16;
}

// Generic parameters and associated-type projections pass through as field
// types unchanged.
fn generic_field_types<T: HasAssoc>() {
    assert_same_type::<
        Struct! { plain: T, assoc: <T as HasAssoc>::Assoc },
        Product![Field<Symbol!("plain"), T>, Field<Symbol!("assoc"), T::Assoc>],
    >();
}

#[test]
fn generic_field_types_pass_through() {
    generic_field_types::<u8>();
}
