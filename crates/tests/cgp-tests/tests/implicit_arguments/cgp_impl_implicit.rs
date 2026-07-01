//! `#[implicit]` arguments inside a `#[cgp_impl]` provider.
//!
//! A provider method can take `#[implicit]` args just like `#[cgp_fn]`; the
//! provider then depends on the context's fields. `implicit_arguments` owns the
//! `#[cgp_impl]`-with-`#[implicit]` snapshot: the expansion drops the implicit
//! parameters from the signature, reads them from the context via
//! `HasField`/`get_field`, and adds the field bounds to the provider's
//! `IsProviderFor` impl. (The basic, no-implicit `#[cgp_impl]` snapshot lives in
//! `basic_delegation`.)
//!
//! See docs/reference/macros/cgp_impl.md and docs/reference/attributes/implicit.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

snapshot_cgp_impl! {
    #[cgp_impl(new RectangleArea)]
    impl AreaCalculator {
        fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
            width * height
        }
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> AreaCalculator<__Context__> for RectangleArea
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
        for RectangleArea
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
        pub struct RectangleArea;
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_and_check_components! {
    Rectangle {
        AreaCalculatorComponent:
            RectangleArea,
    }
}
