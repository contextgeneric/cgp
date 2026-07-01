//! `#[use_type]` reaching a nested foreign type in `#[cgp_fn]` combined with
//! `#[extend_where]`: `#[use_type(HasTypes::Types, @Types::HasScalarType::Scalar)]`
//! plus `#[extend_where(Types: HasScalarType)]`.
//!
//! `HasTypes::Types` imports the abstract `Types`, then `@Types::HasScalarType::Scalar`
//! resolves `Scalar` against it, so the bare alias rewrites to the two-hop
//! `<<Self as HasTypes>::Types as HasScalarType>::Scalar`. Unlike the equality
//! variant, `Types` is *not* pinned to a concrete type, so `#[extend_where(...)]`
//! adds the `<Self as HasTypes>::Types: HasScalarType` bound to the generated trait
//! definition (the aliased `Types` in the attribute is likewise rewritten). The
//! `#[cgp_fn]` snapshot is kept for the rewrite; both `#[cgp_type]` traits are plain.
//! `CheckRectangle` asserts the concrete `Rectangle` implements the generated trait.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_type]
pub trait HasTypes {
    type Types;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasTypes::Types, @Types::HasScalarType::Scalar)]
    #[extend_where(Types: HasScalarType)]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar
    where
        Scalar: Mul<Output = Scalar> + Copy,
    {
        let res: Scalar = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasTypes
        where
            <Self as HasTypes>::Types: HasScalarType,
        {
            fn rectangle_area(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            <<Self as HasTypes>::Types as HasScalarType>::Scalar: Mul<
                    Output = <<Self as HasTypes>::Types as HasScalarType>::Scalar,
                > + Copy,
            <Self as HasTypes>::Types: HasScalarType,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = <<Self as HasTypes>::Types as HasScalarType>::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = <<Self as HasTypes>::Types as HasScalarType>::Scalar,
                >,
            Self: HasTypes,
            <Self as HasTypes>::Types: HasScalarType,
        {
            fn rectangle_area(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar {
                let width: <<Self as HasTypes>::Types as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: <<Self as HasTypes>::Types as HasScalarType>::Scalar = self
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
                let res: <<Self as HasTypes>::Types as HasScalarType>::Scalar = width * height;
                res
            }
        }
        ")
    }
}

pub struct MyTypes;

impl HasScalarType for MyTypes {
    type Scalar = f64;
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasTypes for Rectangle {
    type Types = MyTypes;
}

pub trait CheckRectangle: RectangleArea
where
    Self::Types: HasScalarType,
{
}

impl CheckRectangle for Rectangle {}
