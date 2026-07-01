//! `#[cgp_fn(CustomName)]` overrides the generated trait name, with two
//! `#[implicit]` `f64` arguments read from the context (each `.clone()`d).
//!
//! See docs/reference/macros/cgp_fn.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn(CanCalculateRectangleArea)]
    pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCalculateRectangleArea {
            fn rectangle_area(&self) -> f64;
        }
        impl<__Context__> CanCalculateRectangleArea for __Context__
        where
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = f64,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = f64,
                >,
        {
            fn rectangle_area(&self) -> f64 {
                let width: f64 = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: f64 = self
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

pub trait CheckRectangle: CanCalculateRectangleArea {}
impl CheckRectangle for Rectangle {}
