//! The bare `Path!` macro, written directly rather than embedded in a namespace
//! entry or `#[prefix]` attribute. Each `assert_same_type` call compiles only if
//! the two types are identical, pinning the segment classification: a lowercase
//! segment becomes a `Symbol!`, a capitalized segment stays a type, a primitive
//! name such as `u32` stays the primitive, and the chain ends in `Nil`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/path.md.

use core::marker::PhantomData;

use cgp_fork::prelude::*;

pub struct ErrorRaiserComponent;

fn assert_same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

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
}
