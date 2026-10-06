//! `Enum!` expands to a `Sum!` of `Field` entries, one per variant, keyed by the
//! variant name as a `Symbol!` and ending in `Void`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/enum.md.

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

#[test]
fn variants_expand_to_a_symbol_keyed_sum() {
    assert_same_type::<
        Enum! { Circle(f64), Square(f64) },
        Sum![Field<Symbol!("Circle"), f64>, Field<Symbol!("Square"), f64>],
    >();

    assert_same_type::<
        Enum! { Circle(f64), Square(f64) },
        Either<Field<Symbol!("Circle"), f64>, Either<Field<Symbol!("Square"), f64>, Void>>,
    >();
}

#[test]
fn an_empty_body_is_void() {
    assert_same_type::<Enum! {}, Void>();
}

#[test]
fn a_trailing_comma_is_accepted() {
    assert_same_type::<Enum! { A(u8), B(u16), }, Enum! { A(u8), B(u16) }>();
}

#[test]
fn a_raw_variant_name_is_tagged_by_its_logical_name() {
    assert_same_type::<Enum! { r#match(u8) }, Sum![Field<Symbol!("match"), u8>]>();
}
