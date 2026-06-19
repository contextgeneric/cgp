use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_and_check_components;

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

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        Rectangle {
            AreaCalculatorComponent:
                RectangleArea,
        }
    }

    expand_rectangle(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<AreaCalculatorComponent> for Rectangle {
            type Delegate = RectangleArea;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<AreaCalculatorComponent, __Context__, __Params__> for Rectangle
        where
            RectangleArea: IsProviderFor<AreaCalculatorComponent, __Context__, __Params__>,
        {}
        trait __CanUseRectangle<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseRectangle<AreaCalculatorComponent, ()> for Rectangle {}
        ")
    }
}
