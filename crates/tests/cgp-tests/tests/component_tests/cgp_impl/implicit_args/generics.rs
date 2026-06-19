use core::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_and_check_components;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea<Scalar> {
    fn area(&self) -> Scalar;
}

#[cgp_impl(new RectangleArea)]
impl<Scalar> AreaCalculator<Scalar>
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    fn area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
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
            #[check_params(f64)]
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
        impl __CanUseRectangle<AreaCalculatorComponent, f64> for Rectangle {}
        ")
    }
}
