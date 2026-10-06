//! `Enum!` is the same type `#[derive(HasFields)]` gives as `Fields` for the
//! same body, because the macro runs the derive's own encoder.
//!
//! The `assert_enum_parity!` macro below takes one body, declares an enum with
//! `#[derive(HasFields)]` from it, and asserts that its `Fields` equals `Enum!`
//! over the same tokens. Generic enums and the borrowed `FieldsRef` shape are
//! checked by hand.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/enum.md.

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

macro_rules! assert_enum_parity {
    ($name:ident { $($body:tt)* }) => {{
        // The raw-identifier case declares lowercase variants.
        #[allow(non_camel_case_types)]
        #[derive(HasFields)]
        enum $name { $($body)* }

        assert_same_type::<<$name as HasFields>::Fields, Enum! { $($body)* }>();
    }};
}

#[test]
fn enum_bodies_match_the_derive() {
    assert_enum_parity!(Shape {
        Empty,
        Circle(u32),
        Rectangle(u32, u32),
        Triangle { base: u32, height: u32 },
    });
    assert_enum_parity!(Single { Only(String) });
    assert_enum_parity!(Raw { r#match(u8), r#type { r#in: u16 } });
    assert_enum_parity!(EmptyPayloads { NoParens, Parens(), Braces {} });
    assert_enum_parity!(Never {});
}

#[derive(HasFields)]
pub enum Choice<L, R> {
    Left(L),
    Right(R),
}

#[derive(HasFields)]
pub enum Text<'a> {
    Borrowed(&'a str),
    Owned(String),
}

fn generic_enums_match<'a: 'b, 'b, L: 'b, R: 'b>() {
    assert_same_type::<<Choice<L, R> as HasFields>::Fields, Enum! { Left(L), Right(R) }>();

    assert_same_type::<
        <Choice<L, R> as HasFieldsRef>::FieldsRef<'b>,
        Enum! { Left(&'b L), Right(&'b R) },
    >();

    assert_same_type::<
        <Text<'a> as HasFieldsRef>::FieldsRef<'b>,
        Enum! { Borrowed(&'b &'a str), Owned(&'b String) },
    >();
}

#[test]
fn generic_enums_match_the_derive() {
    generic_enums_match::<'static, 'static, u8, String>();
}
