//! `Struct!` with a tuple body expands to a `Product!` of `Field` entries keyed
//! by `Index<N>`, mirroring a tuple struct's `HasFields` shape.
//!
//! The derive's newtype rule applies: a body with exactly one positional field
//! is that field's type, with no `Field` wrapper and no list. A trailing comma
//! does not change this, so `Struct!(T,)` is `T` too, unlike the tuple type
//! `(T,)`. The form is read from the entries rather than the delimiter, so this
//! file also pins that path types such as `core::marker::PhantomData<u8>`,
//! whose `::` begins with a `:`, are read as positional fields.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

#[test]
fn positional_fields_expand_to_an_index_keyed_product() {
    assert_same_type::<Struct!(u64, String), Product![Field<Index<0>, u64>, Field<Index<1>, String>]>(
    );

    assert_same_type::<
        Struct!(u8, u16, u32),
        Product![
            Field<Index<0>, u8>,
            Field<Index<1>, u16>,
            Field<Index<2>, u32>
        ],
    >();
}

#[test]
fn an_empty_tuple_body_is_nil() {
    assert_same_type::<Struct!(), Nil>();
}

#[test]
fn one_positional_field_is_its_bare_type() {
    assert_same_type::<Struct!(u64), u64>();
    assert_same_type::<Struct!(u64,), u64>();
}

#[test]
fn path_types_are_positional_fields() {
    assert_same_type::<
        Struct!(core::marker::PhantomData<u8>, ::core::primitive::u8),
        Product![Field<Index<0>, PhantomData<u8>>, Field<Index<1>, u8>],
    >();

    assert_same_type::<Struct!(u8, bool), Product![Field<Index<0>, u8>, Field<Index<1>, bool>]>();
}

#[test]
fn trait_object_and_function_types_are_positional_fields() {
    assert_same_type::<
        Struct!(Box<dyn Fn(u8) -> u8>, &'static dyn Debug),
        Product![
            Field<Index<0>, Box<dyn Fn(u8) -> u8>>,
            Field<Index<1>, &'static dyn Debug>
        ],
    >();
}

pub struct Holder;

impl Holder {
    // `Self` is a keyword rather than a field name, so it starts a positional
    // field.
    fn self_is_a_positional_field() {
        assert_same_type::<Struct!(Self, u8), Product![Field<Index<0>, Holder>, Field<Index<1>, u8>]>(
        );
    }
}

#[test]
fn self_is_a_positional_field() {
    Holder::self_is_a_positional_field();
}
