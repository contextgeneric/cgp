use core::f64::consts::PI;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;
use insta::assert_snapshot;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }

    expand_rectangle_area(output) {
        assert_snapshot!(output, @"
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
    pub fn circle_area(&self, #[implicit] radius: f64) -> f64 {
        PI * radius * radius
    }

    expand_circle_area(output) {
        assert_snapshot!(output, @"
        pub trait CircleArea {
            fn circle_area(&self) -> f64;
        }
        impl<__Context__> CircleArea for __Context__
        where
            Self: HasField<
                Symbol<
                    6,
                    Chars<'r', Chars<'a', Chars<'d', Chars<'i', Chars<'u', Chars<'s', Nil>>>>>>,
                >,
                Value = f64,
            >,
        {
            fn circle_area(&self) -> f64 {
                let radius: f64 = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'r',
                                    Chars<
                                        'a',
                                        Chars<'d', Chars<'i', Chars<'u', Chars<'s', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone();
                PI * radius * radius
            }
        }
        ")
    }
}

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

#[cgp_impl(new CircleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] radius: f64) -> f64 {
        PI * radius * radius
    }
}

#[cgp_impl(new ScaledRectangleAreaCalculator)]
#[use_provider(RectangleAreaCalculator: AreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        RectangleAreaCalculator::area(self) * scale_factor * scale_factor
    }
}

#[cgp_impl(new ScaledCircleAreaCalculator)]
#[use_provider(CircleAreaCalculator: AreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        CircleAreaCalculator::area(self) * scale_factor * scale_factor
    }
}

#[cgp_impl(new ScaledAreaCalculator<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        let base_area = InnerCalculator::area(self);

        base_area * scale_factor * scale_factor
    }
}

#[derive(HasField)]
pub struct IsThisRectangleOrCircle {
    pub width: f64,
    pub height: f64,
    pub radius: f64,
}

impl CanCalculateArea for IsThisRectangleOrCircle {
    fn area(&self) -> f64 {
        CircleAreaCalculator::area(self)
    }
}

#[test]
fn test_rectangle_or_circle() {
    let rectangle_or_circle = IsThisRectangleOrCircle {
        width: 2.0,
        height: 3.0,
        radius: 4.0,
    };

    let area = rectangle_or_circle.area();
    assert_eq!(area, 16.0 * PI);

    let rectangle_area = RectangleAreaCalculator::area(&rectangle_or_circle);
    assert_eq!(rectangle_area, 6.0);

    let circle_area = CircleAreaCalculator::area(&rectangle_or_circle);
    assert_eq!(circle_area, 16.0 * PI);

    let rectangle_area = rectangle_or_circle.rectangle_area();
    assert_eq!(rectangle_area, 6.0);

    let circle_area = rectangle_or_circle.circle_area();
    assert_eq!(circle_area, 16.0 * PI);
}

#[derive(HasField)]
pub struct PlainRectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    PlainRectangle {
        AreaCalculatorComponent:
            RectangleAreaCalculator,
    }
}

#[derive(HasField)]
pub struct ScaledRectangle {
    pub scale_factor: f64,
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}

#[derive(HasField)]
pub struct PlainCircle {
    pub radius: f64,
}

delegate_and_check_components! {
    PlainCircle {
        AreaCalculatorComponent:
            CircleAreaCalculator,
    }
}

#[derive(HasField)]
pub struct ScaledCircle {
    pub scale_factor: f64,
    pub radius: f64,
}

delegate_and_check_components! {
    ScaledCircle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<CircleAreaCalculator>,
    }
}

check_components! {
    #[check_trait(CheckScaledRectangleProviders)]
    #[check_providers(
        RectangleAreaCalculator,
        ScaledAreaCalculator<RectangleAreaCalculator>,
    )]
    ScaledRectangle {
        AreaCalculatorComponent,
    }
}

#[test]
fn test_scaled_area() {
    let rectangle = PlainRectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.area(), 12.0);

    let scaled_rectangle = ScaledRectangle {
        scale_factor: 2.0,
        width: 3.0,
        height: 4.0,
    };

    let circle = PlainCircle { radius: 3.0 };

    assert_eq!(circle.area(), 9.0 * PI);

    assert_eq!(scaled_rectangle.area(), 48.0);

    let scaled_circle = ScaledCircle {
        scale_factor: 2.0,
        radius: 3.0,
    };

    assert_eq!(scaled_circle.area(), 36.0 * PI);
}
