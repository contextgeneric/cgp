use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_component, snapshot_cgp_fn, snapshot_cgp_impl};

snapshot_cgp_component! {
    #[cgp_component(AreaCalculator)]
    pub trait CanCalculateArea {
        fn area(&self) -> f64;
    }

    expand_can_calculate_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCalculateArea {
            fn area(&self) -> f64;
        }
        impl<__Context__> CanCalculateArea for __Context__
        where
            __Context__: AreaCalculator<__Context__>,
        {
            fn area(&self) -> f64 {
                __Context__::area(self)
            }
        }
        pub trait AreaCalculator<
            __Context__,
        >: IsProviderFor<AreaCalculatorComponent, __Context__, ()> {
            fn area(__context__: &__Context__) -> f64;
        }
        impl<__Provider__, __Context__> AreaCalculator<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<AreaCalculatorComponent>
                + IsProviderFor<AreaCalculatorComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                AreaCalculatorComponent,
            >>::Delegate: AreaCalculator<__Context__>,
        {
            fn area(__context__: &__Context__) -> f64 {
                <__Provider__ as DelegateComponent<
                    AreaCalculatorComponent,
                >>::Delegate::area(__context__)
            }
        }
        pub struct AreaCalculatorComponent;
        impl<__Context__> AreaCalculator<__Context__> for UseContext
        where
            __Context__: CanCalculateArea,
        {
            fn area(__context__: &__Context__) -> f64 {
                __Context__::area(__context__)
            }
        }
        impl<__Context__> IsProviderFor<AreaCalculatorComponent, __Context__, ()> for UseContext
        where
            __Context__: CanCalculateArea,
        {}
        impl<__Context__, __Components__, __Path__> AreaCalculator<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: AreaCalculator<__Context__>,
        {
            fn area(__context__: &__Context__) -> f64 {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::area(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<AreaCalculatorComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<AreaCalculatorComponent, __Context__, ()>
                + AreaCalculator<__Context__>,
        {}
        ")
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new RectangleAreaCalculator)]
    impl AreaCalculator {
        fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
            width * height
        }
    }

    expand_rectangle_area_calculator(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> AreaCalculator<__Context__> for RectangleAreaCalculator
        where
            __Context__: HasField<
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
            fn area(__context__: &__Context__) -> f64 {
                let width: f64 = __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: f64 = __context__
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
        impl<__Context__> IsProviderFor<AreaCalculatorComponent, __Context__, ()>
        for RectangleAreaCalculator
        where
            __Context__: HasField<
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
        {}
        pub struct RectangleAreaCalculator;
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_provider(RectangleAreaCalculator: AreaCalculator)]
    fn rectangle_area(&self) -> f64 {
        RectangleAreaCalculator::area(self)
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        trait RectangleArea {
            fn rectangle_area(&self) -> f64;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            RectangleAreaCalculator: AreaCalculator<Self>,
        {
            fn rectangle_area(&self) -> f64 {
                RectangleAreaCalculator::area(self)
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

#[test]
fn test_use_provider() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.rectangle_area(), 12.0);
}
