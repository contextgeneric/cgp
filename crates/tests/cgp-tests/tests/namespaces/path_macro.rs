//! The bare `Path!` macro, written directly rather than embedded in a namespace
//! entry or `#[prefix]` attribute. Each `assert_same_type` call compiles only if
//! the two types are identical, pinning the segment classification: a lowercase
//! segment becomes a `Symbol!`, a capitalized segment stays a type, a primitive
//! name such as `u32` stays the primitive (each of the seventeen is asserted) while a near miss
//! such as `f` or `u2` becomes a symbol, and the chain ends in `Nil`.
//!
//! See cgp-knowledge-base/cgp/reference/macros/path.md.

use core::marker::PhantomData;

use cgp::prelude::*;

pub struct ErrorRaiserComponent;

fn assert_same_type<T: ?Sized>(_: PhantomData<T>, _: PhantomData<T>) {}

macro_rules! assert_primitive_segments {
    ($($primitive:ident),*) => {
        $(
            assert_same_type(
                PhantomData::<Path!(@$primitive)>,
                PhantomData::<PathCons<$primitive, Nil>>,
            );
        )*
    };
}

#[test]
fn test_path_macro_segments() {
    assert_same_type(
        PhantomData::<Path!(@app.error.ErrorRaiserComponent)>,
        PhantomData::<
            PathCons<
                Symbol!("app"),
                PathCons<Symbol!("error"), PathCons<ErrorRaiserComponent, Nil>>,
            >,
        >,
    );

    assert_same_type(
        PhantomData::<Path!(@ErrorRaiserComponent)>,
        PhantomData::<PathCons<ErrorRaiserComponent, Nil>>,
    );

    assert_same_type(
        PhantomData::<Path!(@app.u32.bool)>,
        PhantomData::<PathCons<Symbol!("app"), PathCons<u32, PathCons<bool, Nil>>>>,
    );

    // Only real primitive names stay types: `f`, `u2`, and `i7` look like
    // primitives but are none, so each becomes a symbol.
    assert_same_type(
        PhantomData::<Path!(@f.u2.i7.f64)>,
        PhantomData::<
            PathCons<
                Symbol!("f"),
                PathCons<Symbol!("u2"), PathCons<Symbol!("i7"), PathCons<f64, Nil>>>,
            >,
        >,
    );

    // Every primitive name stays the primitive type, `str` included.
    assert_primitive_segments!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, bool, str
    );
}
