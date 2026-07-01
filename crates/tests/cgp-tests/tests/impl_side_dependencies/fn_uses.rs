//! `#[uses(...)]` on a `#[cgp_fn]` imports a `Self` trait bound, read like a
//! `use` statement, so the function body can call another capability. Here
//! `scaled_rectangle_area` declares `#[uses(RectangleArea)]` and calls
//! `self.rectangle_area()`; the import lands as `Self: RectangleArea` in the
//! generated impl's `where` clause — an impl-side dependency the consumer trait
//! does not expose.
//!
//! The `rectangle_area` dependency below is a plain `#[cgp_fn]` (its full
//! expansion is owned by the `implicit_arguments` concept), so it is written
//! without a snapshot; the snapshot here pins how `#[uses]` lands.
//!
//! See docs/concepts/impl-side-dependencies.md,
//! docs/reference/attributes/uses.md, and docs/reference/macros/cgp_fn.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[uses(RectangleArea)]
    pub fn scaled_rectangle_area(&self, #[implicit] scale_factor: f64) -> f64 {
        self.rectangle_area() * scale_factor * scale_factor
    }

    expand_scaled_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait ScaledRectangleArea {
            fn scaled_rectangle_area(&self) -> f64;
        }
        impl<__Context__> ScaledRectangleArea for __Context__
        where
            Self: RectangleArea,
            Self: HasField<
                Symbol<
                    12,
                    Chars<
                        's',
                        Chars<
                            'c',
                            Chars<
                                'a',
                                Chars<
                                    'l',
                                    Chars<
                                        'e',
                                        Chars<
                                            '_',
                                            Chars<
                                                'f',
                                                Chars<
                                                    'a',
                                                    Chars<'c', Chars<'t', Chars<'o', Chars<'r', Nil>>>>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                Value = f64,
            >,
        {
            fn scaled_rectangle_area(&self) -> f64 {
                let scale_factor: f64 = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                12,
                                Chars<
                                    's',
                                    Chars<
                                        'c',
                                        Chars<
                                            'a',
                                            Chars<
                                                'l',
                                                Chars<
                                                    'e',
                                                    Chars<
                                                        '_',
                                                        Chars<
                                                            'f',
                                                            Chars<
                                                                'a',
                                                                Chars<'c', Chars<'t', Chars<'o', Chars<'r', Nil>>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone();
                self.rectangle_area() * scale_factor * scale_factor
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
}

pub trait CheckRectangle: ScaledRectangleArea {}
impl CheckRectangle for Rectangle {}
