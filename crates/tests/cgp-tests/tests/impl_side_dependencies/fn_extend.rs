//! `#[extend(...)]` on a `#[cgp_fn]` adds a *supertrait* bound to the generated
//! trait — the only way to add a supertrait in `#[cgp_fn]`, whose `where` clauses
//! are impl-side dependencies. Here `rectangle_area` declares
//! `#[extend(HasScalarType)]` so the trait reads `RectangleArea: HasScalarType`
//! and its signatures name the abstract type as `Self::Scalar`. The snapshot pins
//! how `#[extend]` lands on the generated trait definition and impl.
//!
//! The `#[cgp_type]` scaffolding is written plainly here — its expansion is owned
//! by the `abstract_types` concept.
//!
//! See cgp-knowledge-base/cgp/concepts/impl-side-dependencies.md,
//! cgp-knowledge-base/cgp/reference/attributes/extend.md, and
//! cgp-knowledge-base/cgp/reference/macros/cgp_fn.md.

use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[extend(HasScalarType)]
    pub fn rectangle_area(
        &self,
        #[implicit] width: Self::Scalar,
        #[implicit] height: Self::Scalar,
    ) -> Self::Scalar
    where
        Self::Scalar: Mul<Output = Self::Scalar> + Copy,
    {
        width * height
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        ///The trait for the `rectangle_area` operation.
        pub trait RectangleArea: HasScalarType {
            ///The `rectangle_area` operation.
            fn rectangle_area(&self) -> Self::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            Self::Scalar: Mul<Output = Self::Scalar> + Copy,
            Self: HasScalarType,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = Self::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = Self::Scalar,
                >,
        {
            fn rectangle_area(&self) -> Self::Scalar {
                let width: Self::Scalar = ::core::clone::Clone::clone(
                    HasField::get_field(
                        self,
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    ),
                );
                let height: Self::Scalar = ::core::clone::Clone::clone(
                    HasField::get_field(
                        self,
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
                    ),
                );
                width * height
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasScalarType for Rectangle {
    type Scalar = f64;
}

pub trait CheckRectangle: RectangleArea {}
impl CheckRectangle for Rectangle {}
