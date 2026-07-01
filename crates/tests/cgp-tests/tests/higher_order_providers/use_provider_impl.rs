//! `#[use_provider]` on a `#[cgp_impl]`: a higher-order provider parameterized by
//! an inner provider.
//!
//! `ScaledArea<Inner>` takes an inner `AreaCalculator` and scales its result. The
//! inner provider is chosen at wiring time (`ScaledArea<RectangleArea>`), so the
//! same outer provider composes with any base calculator.
//!
//! `higher_order_providers` owns the `#[cgp_impl]`-with-`#[use_provider]`
//! snapshot: the expansion moves the inner provider into the `where` clause as
//! `Inner: AreaCalculator<__Context__>` and threads it into the `IsProviderFor`
//! impl. The plain `RectangleArea` below is incidental scaffolding — its
//! `#[implicit]` expansion is snapshotted in `implicit_arguments`.
//!
//! See docs/reference/attributes/use_provider.md and
//! docs/concepts/higher-order-providers.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new ScaledArea<Inner>)]
    #[use_provider(Inner: AreaCalculator)]
    impl<Inner> AreaCalculator {
        fn area(&self, #[implicit] scale_factor: f64) -> f64 {
            Inner::area(self) * scale_factor * scale_factor
        }
    }

    expand_scaled_area(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__, Inner> AreaCalculator<__Context__> for ScaledArea<Inner>
        where
            __Context__: HasField<
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
            Inner: AreaCalculator<__Context__>,
        {
            fn area(__context__: &__Context__) -> f64 {
                let scale_factor: f64 = __context__
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
                Inner::area(__context__) * scale_factor * scale_factor
            }
        }
        impl<__Context__, Inner> IsProviderFor<AreaCalculatorComponent, __Context__, ()>
        for ScaledArea<Inner>
        where
            __Context__: HasField<
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
            Inner: IsProviderFor<AreaCalculatorComponent, __Context__, ()>
                + AreaCalculator<__Context__>,
        {}
        pub struct ScaledArea<Inner>(pub ::core::marker::PhantomData<(Inner)>);
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl CanCalculateArea for Rectangle {
    fn area(&self) -> f64 {
        RectangleArea::area(self)
    }
}

#[derive(HasField)]
pub struct ScaledRectangle {
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
}

delegate_components! {
    ScaledRectangle {
        AreaCalculatorComponent: ScaledArea<RectangleArea>,
    }
}

#[test]
fn test_scaled_area() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(rectangle.area(), 12.0);

    // The inner `RectangleArea` computes 12.0, then `ScaledArea` scales by 2^2.
    let scaled = ScaledRectangle {
        width: 3.0,
        height: 4.0,
        scale_factor: 2.0,
    };
    assert_eq!(scaled.area(), 48.0);
}
