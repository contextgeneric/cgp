use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_fn, snapshot_delegate_and_check_components};

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleAreaCalculator)]
#[uses(RectangleArea)]
impl AreaCalculator {
    fn area(&self) -> f64 {
        self.rectangle_area()
    }
}

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

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        Rectangle {
            AreaCalculatorComponent:
                RectangleAreaCalculator,
        }
    }

    expand_rectangle(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<AreaCalculatorComponent> for Rectangle {
            type Delegate = RectangleAreaCalculator;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<AreaCalculatorComponent, __Context__, __Params__> for Rectangle
        where
            RectangleAreaCalculator: IsProviderFor<
                AreaCalculatorComponent,
                __Context__,
                __Params__,
            >,
        {}
        trait __CanUseRectangle<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseRectangle<AreaCalculatorComponent, ()> for Rectangle {}
        ")
    }
}
