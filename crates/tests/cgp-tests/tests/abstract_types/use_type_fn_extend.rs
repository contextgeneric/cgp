//! Abstract types in a `#[cgp_fn]` via `#[extend(HasScalarType)]` + `Self::Scalar`.
//!
//! `#[cgp_fn]`'s own `where` clauses are impl-side dependencies, so a supertrait
//! is added with `#[extend]`; the abstract type is then named the long way as
//! `Self::Scalar`. This is the `#[extend]` counterpart to `use_type_fn_alias`
//! (which uses `#[use_type]` and the bare alias). The `#[cgp_fn]` snapshot is kept
//! because the point is how the abstract type flows into the generated trait/impl;
//! the `#[cgp_fn]` expansion mechanics themselves are owned by `implicit_arguments`.
//! Two runtime contexts (`f32`/`f64`) confirm the generic result.
//!
//! See docs/reference/attributes/use_type.md, docs/reference/macros/cgp_fn.md,
//! and docs/concepts/abstract-types.md.

use core::f64;
use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Mul<Output = Self::Scalar> + Copy;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[extend(HasScalarType)]
    pub fn rectangle_area(
        &self,
        #[implicit] width: Self::Scalar,
        #[implicit] height: Self::Scalar,
    ) -> Self::Scalar {
        width * height
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasScalarType {
            fn rectangle_area(&self) -> Self::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
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
pub struct F32Rectangle {
    pub width: f32,
    pub height: f32,
}

impl HasScalarType for F32Rectangle {
    type Scalar = f32;
}

#[derive(HasField)]
pub struct F64Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasScalarType for F64Rectangle {
    type Scalar = f64;
}

#[test]
fn test_rectangle_area() {
    let f32_rectangle = F32Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(f32_rectangle.rectangle_area(), 12.0);

    let f64_rectangle = F64Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(f64_rectangle.rectangle_area(), 12.0);
}
