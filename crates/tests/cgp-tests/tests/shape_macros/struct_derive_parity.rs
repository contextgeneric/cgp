//! `Struct!` is the same type `#[derive(HasFields)]` gives as `Fields` for the
//! same body, because the macro runs the derive's own encoder.
//!
//! The `assert_struct_parity!` macro below takes one body, declares a struct
//! with `#[derive(HasFields)]` from it, and asserts that its `Fields` equals
//! `Struct!` over the same tokens, so the two are compared on identical input.
//! Generic structs and the borrowed `FieldsRef` shape are checked by hand,
//! since their generics cannot be written inside a `Struct!` body.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use cgp::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

macro_rules! assert_struct_parity {
    ($name:ident { $($body:tt)* }) => {{
        #[derive(HasFields)]
        struct $name { $($body)* }

        assert_same_type::<<$name as HasFields>::Fields, Struct! { $($body)* }>();
    }};
    ($name:ident ( $($body:tt)* )) => {{
        #[derive(HasFields)]
        struct $name ( $($body)* );

        assert_same_type::<<$name as HasFields>::Fields, Struct!( $($body)* )>();
    }};
}

#[test]
fn named_bodies_match_the_derive() {
    assert_struct_parity!(Person {
        name: String,
        age: u8
    });
    assert_struct_parity!(Single { value: u64 });
    assert_struct_parity!(Trailing { a: u8, b: u16 });
    assert_struct_parity!(Raw {
        r#type: u8,
        r#match: u16
    });
    assert_struct_parity!(EmptyNamed {});
}

#[test]
fn tuple_bodies_match_the_derive() {
    assert_struct_parity!(Pair(u64, String));
    assert_struct_parity!(Triple(u8, u16, u32));
    assert_struct_parity!(Newtype(u64));
    assert_struct_parity!(NewtypeTrailing(u64,));
    assert_struct_parity!(Paths(core::marker::PhantomData<u8>, ::core::primitive::u8));
    assert_struct_parity!(EmptyTuple());
}

#[derive(HasFields)]
pub struct Unit;

#[test]
fn a_unit_struct_matches_an_empty_body() {
    assert_same_type::<<Unit as HasFields>::Fields, Struct! {}>();
}

#[derive(HasFields)]
pub struct Wrapper<T> {
    pub value: T,
    pub count: usize,
}

#[derive(HasFields)]
pub struct GenericPair<A, B>(pub A, pub B);

fn generic_structs_match<A, B>() {
    assert_same_type::<<Wrapper<A> as HasFields>::Fields, Struct! { value: A, count: usize }>();
    assert_same_type::<<GenericPair<A, B> as HasFields>::Fields, Struct!(A, B)>();
}

#[test]
fn generic_structs_match_the_derive() {
    generic_structs_match::<u8, String>();
}

#[derive(HasFields)]
pub struct Borrowed<'a> {
    pub text: &'a str,
    pub count: u32,
}

// `FieldsRef<'b>` borrows each field for `'b`, so a field that is already a
// reference gains a second layer.
fn borrowed_shapes_match<'a: 'b, 'b, T: 'b>() {
    assert_same_type::<
        <Wrapper<T> as HasFieldsRef>::FieldsRef<'b>,
        Struct! { value: &'b T, count: &'b usize },
    >();

    assert_same_type::<
        <Borrowed<'a> as HasFieldsRef>::FieldsRef<'b>,
        Struct! { text: &'b &'a str, count: &'b u32 },
    >();
}

#[test]
fn borrowed_shapes_match_the_derive() {
    borrowed_shapes_match::<'static, 'static, u8>();
}
