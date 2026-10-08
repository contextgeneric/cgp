//! `#[derive_promote(Name)]` on `#[cgp_component]` emits a provider struct and an
//! impl that forwards the component's single method to `Computer::compute`.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_component.md.

use cgp_fork_macro::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(AreaCalculator)]
    #[derive_promote(PromoteAreaCalculator)]
    pub trait HasArea {
        fn area(&self) -> f64;
    }

    expand_promote_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasArea {
            fn area(&self) -> f64;
        }
        impl<__Context__> HasArea for __Context__
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
        pub struct PromoteAreaCalculator<__Provider__>(
            pub ::core::marker::PhantomData<__Provider__>,
        );
        impl<__Context__> AreaCalculator<__Context__> for UseContext
        where
            __Context__: HasArea,
        {
            fn area(__context__: &__Context__) -> f64 {
                __Context__::area(__context__)
            }
        }
        impl<__Context__> IsProviderFor<AreaCalculatorComponent, __Context__, ()> for UseContext
        where
            __Context__: HasArea,
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
        impl<__Context__, __Provider__> AreaCalculator<__Context__>
        for PromoteAreaCalculator<__Provider__>
        where
            __Provider__: Computer<__Context__, (), (), Output = f64>,
        {
            fn area(__context__: &__Context__) -> f64 {
                <__Provider__ as Computer<
                    __Context__,
                    (),
                    (),
                >>::compute(__context__, ::core::marker::PhantomData::<()>, ())
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<AreaCalculatorComponent, __Context__, ()>
        for PromoteAreaCalculator<__Provider__>
        where
            __Provider__: Computer<__Context__, (), (), Output = f64>,
        {}
        ")
    }
}
