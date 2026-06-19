use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea {
            fn rectangle_area(&self) -> f64;
        }
        impl<__Context__> RectangleArea for __Context__
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
