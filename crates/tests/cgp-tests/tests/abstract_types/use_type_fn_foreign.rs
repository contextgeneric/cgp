//! `#[use_type]` importing an abstract type from a *foreign generic parameter* in
//! a `#[cgp_fn]`: `#[use_type(HasScalarType.Scalar in Types)]`.
//!
//! The function is generic over a plain `Types` (declared *without* a
//! `HasScalarType` bound), and the `in Types` suffix resolves `Scalar` against
//! that parameter, rewriting the bare alias to `<Types as HasScalarType>::Scalar`
//! throughout. The foreign import supplies the `Types: HasScalarType` bound on
//! the generated trait itself — the regression guard for that bound being
//! dropped — so the trait becomes `RectangleArea<Types> where Types: HasScalarType`.
//! `CheckRectangle` is a compile-time assertion that `Rectangle` implements the
//! generated trait for the concrete `Types`. The `#[cgp_fn]` snapshot is kept for
//! the rewrite; `#[cgp_type]` is plain.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasScalarType.Scalar in Types)]
    pub fn rectangle_area<Types>(
        &self,
        #[implicit] width: Scalar,
        #[implicit] height: Scalar,
    ) -> Scalar
    where
        Scalar: Mul<Output = Scalar> + Copy,
    {
        let res: Scalar = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea<Types>
        where
            Types: HasScalarType,
        {
            fn rectangle_area(&self) -> <Types as HasScalarType>::Scalar;
        }
        impl<__Context__, Types> RectangleArea<Types> for __Context__
        where
            <Types as HasScalarType>::Scalar: Mul<Output = <Types as HasScalarType>::Scalar>
                + Copy,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = <Types as HasScalarType>::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = <Types as HasScalarType>::Scalar,
                >,
            Types: HasScalarType,
        {
            fn rectangle_area(&self) -> <Types as HasScalarType>::Scalar {
                let width: <Types as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: <Types as HasScalarType>::Scalar = self
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
                let res: <Types as HasScalarType>::Scalar = width * height;
                res
            }
        }
        ")
    }
}

pub struct Types;

impl HasScalarType for Types {
    type Scalar = f64;
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub trait CheckRectangle: RectangleArea<Types> {}
impl CheckRectangle for Rectangle {}
