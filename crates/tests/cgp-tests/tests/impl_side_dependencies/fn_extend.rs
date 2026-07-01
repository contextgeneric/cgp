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
//! See docs/concepts/impl-side-dependencies.md,
//! docs/reference/attributes/extend.md, and docs/reference/macros/cgp_fn.md.

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
        pub trait RectangleArea: HasScalarType {
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
                let width: Self::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: Self::Scalar = self
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
    pub width: f64,
    pub height: f64,
}

impl HasScalarType for Rectangle {
    type Scalar = f64;
}

pub trait CheckRectangle: RectangleArea {}
impl CheckRectangle for Rectangle {}
