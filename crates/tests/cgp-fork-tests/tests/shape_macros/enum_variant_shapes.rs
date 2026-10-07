//! Each `Enum!` variant's payload is encoded by the `Struct!` rules for its
//! fields, so the four variant shapes map as `#[derive(HasFields)]` maps them: a
//! unit variant carries `Nil`, one positional field carries its bare type,
//! several positional fields carry an `Index`-keyed product, and named fields
//! carry a `Symbol!`-keyed product.
//!
//! The equivalences below follow from that rule, and cargo-cgp relies on them
//! when it prints a variant list: it chooses the shortest of several spellings
//! that are all the same type.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/enum.md.

use cgp_fork::prelude::*;

use crate::shape_macros::assertions::{assert_same_type, assert_type_ne};

#[test]
fn each_variant_shape_has_its_payload() {
    assert_same_type::<
        Enum! {
            Empty,
            Circle(u32),
            Rectangle(u32, u32),
            Triangle { base: u32, height: u32 },
        },
        Sum![
            Field<Symbol!("Empty"), Nil>,
            Field<Symbol!("Circle"), u32>,
            Field<Symbol!("Rectangle"), Product![Field<Index<0>, u32>, Field<Index<1>, u32>]>,
            Field<
                Symbol!("Triangle"),
                Product![Field<Symbol!("base"), u32>, Field<Symbol!("height"), u32>],
            >,
        ],
    >();
}

#[test]
fn a_unit_variant_is_any_empty_payload() {
    assert_same_type::<Enum! { V }, Enum! { V() }>();
    assert_same_type::<Enum! { V }, Enum! { V {} }>();
    assert_same_type::<Enum! { V }, Enum! { V(Nil) }>();
}

#[test]
fn a_variant_body_is_a_struct_shape_payload() {
    assert_same_type::<Enum! { V { a: u32 } }, Enum! { V(Struct! { a: u32 }) }>();
    assert_same_type::<Enum! { V(u8, u16) }, Enum! { V(Struct!(u8, u16)) }>();
    assert_same_type::<Enum! { V(u64) }, Enum! { V(Struct!(u64)) }>();
}

#[test]
fn positional_and_named_payloads_differ() {
    assert_type_ne::<Enum! { V(u64) }, Enum! { V { value: u64 } }>();
}
