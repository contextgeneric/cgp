use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasScalarType::{Scalar as S})]
    pub fn rectangle_area(&self, #[implicit] width: S, #[implicit] height: S) -> S
    where
        S: Mul<Output = S> + Copy,
    {
        let res: S = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasScalarType {
            fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            <Self as HasScalarType>::Scalar: Mul<Output = <Self as HasScalarType>::Scalar>
                + Copy,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = <Self as HasScalarType>::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = <Self as HasScalarType>::Scalar,
                >,
            Self: HasScalarType,
        {
            fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar {
                let width: <Self as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: <Self as HasScalarType>::Scalar = self
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
                let res: <Self as HasScalarType>::Scalar = width * height;
                res
            }
        }
        ")
    }
}
