//! `#[cgp_fn]` on a function generic over a type parameter `Scalar`.
//!
//! The generic parameter moves onto the generated trait (`RectangleArea<Scalar>`)
//! and impl, its `where` clause becomes an impl-side dependency, and the
//! `#[implicit]` arguments are pulled from the context's `width`/`height` fields.
//! This is the reference snapshot for a `#[cgp_fn]` carrying a type parameter.
//!
//! See docs/reference/macros/cgp_fn.md.

use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn rectangle_area<Scalar>(
        &self,
        #[implicit] width: Scalar,
        #[implicit] height: Scalar,
    ) -> Scalar
    where
        Scalar: Mul<Output = Scalar> + Copy,
    {
        width * height
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea<Scalar> {
            fn rectangle_area(&self) -> Scalar;
        }
        impl<__Context__, Scalar> RectangleArea<Scalar> for __Context__
        where
            Scalar: Mul<Output = Scalar> + Copy,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = Scalar,
                >,
        {
            fn rectangle_area(&self) -> Scalar {
                let width: Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'h',
                                    Chars<
                                        'e',
                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone();
                width * height
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

pub trait CheckRectangle: RectangleArea<f32> {}
impl CheckRectangle for Rectangle {}
