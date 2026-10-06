//! A `Struct!` or `Enum!` written by another macro receives its fields as
//! forwarded fragments. A `$ty:ty` fragment arrives wrapped in a token group
//! with no visible delimiter, so this pins that the named and tuple forms are
//! still read correctly from such input.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use core::marker::PhantomData;

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

macro_rules! named_shape {
    ($($name:ident : $ty:ty),* $(,)?) => {
        Struct! { $($name: $ty),* }
    };
}

macro_rules! tuple_shape {
    ($($ty:ty),* $(,)?) => {
        Struct!( $($ty),* )
    };
}

macro_rules! variant_shape {
    ($($variant:ident ( $ty:ty )),* $(,)?) => {
        Enum! { $($variant($ty)),* }
    };
}

#[test]
fn forwarded_named_fields_stay_named() {
    assert_same_type::<
        named_shape!(width: f64, height: Vec<u8>),
        Product![Field<Symbol!("width"), f64>, Field<Symbol!("height"), Vec<u8>>],
    >();
}

#[test]
fn forwarded_types_stay_positional() {
    assert_same_type::<
        tuple_shape!(u8, core::marker::PhantomData<u8>),
        Product![Field<Index<0>, u8>, Field<Index<1>, PhantomData<u8>>],
    >();

    assert_same_type::<tuple_shape!(Vec<u8>), Vec<u8>>();
}

#[test]
fn forwarded_variants_keep_their_payloads() {
    assert_same_type::<
        variant_shape!(Circle(f64), Square(core::marker::PhantomData<u8>)),
        Sum![Field<Symbol!("Circle"), f64>, Field<Symbol!("Square"), PhantomData<u8>>],
    >();
}
