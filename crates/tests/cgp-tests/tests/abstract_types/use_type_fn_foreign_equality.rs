//! `#[use_type]` chaining an imported associated type into a *nested* foreign type
//! source, with type-equality, in a `#[cgp_fn]`:
//! `#[use_type(HasTypes::Types, @Types::HasScalarType::{Scalar = f64})]`.
//!
//! `HasTypes::Types` imports the abstract `Types` from `Self`, then
//! `@Types::HasScalarType::{Scalar = f64}` resolves `Scalar` against *that*
//! imported type and pins it to `f64`, so the bare `Scalar` alias rewrites to the
//! two-hop `<<Self as HasTypes>::Types as HasScalarType>::Scalar` and the impl
//! gains `<Self as HasTypes>::Types: HasScalarType<Scalar = f64>`. The `#[cgp_fn]`
//! snapshot is kept for the rewrite; both `#[cgp_type]` traits are written plainly.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_type]
pub trait HasTypes {
    type Types: HasScalarType;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(
        HasTypes::Types,
        @Types::HasScalarType::{Scalar = f64},
    )]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        let res: f64 = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasTypes {
            fn rectangle_area(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
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
            <Self as HasTypes>::Types: HasScalarType<Scalar = f64>,
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
                let res: f64 = width * height;
                res
            }
        }
        ")
    }
}
