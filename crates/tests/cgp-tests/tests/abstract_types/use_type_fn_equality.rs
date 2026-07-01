//! `#[use_type]` with a *type-equality* bound in a `#[cgp_fn]`:
//! `#[use_type(HasScalarType::{Scalar = f64})]`.
//!
//! The `{Scalar = f64}` form both imports `Scalar` (rewriting the bare alias to
//! `<Self as HasScalarType>::Scalar`) and pins it to a concrete type by adding
//! `Self: HasScalarType<Scalar = f64>` to the impl's `where` clause, so the body
//! may treat the value as an `f64` while the signature still speaks the abstract
//! alias. The `#[cgp_fn]` snapshot is kept for the rewrite; `#[cgp_type]` is plain.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasScalarType::{Scalar = f64})]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        let res: f64 = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasScalarType {
            fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
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
            Self: HasScalarType<Scalar = f64>,
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
                let res: f64 = width * height;
                res
            }
        }
        ")
    }
}
