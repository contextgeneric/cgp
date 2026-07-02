//! `#[use_type]` importing an abstract type from a *path-qualified* trait:
//! `#[use_type(scalar::HasScalarType.Scalar)]`.
//!
//! The `.` separator between the trait and its associated type lets the trait be a
//! full path (`scalar::HasScalarType`) whose own `::` segments stay unambiguous, so
//! the trait need not be brought into scope by its bare name. The bare alias
//! `Scalar` still rewrites to `<Self as scalar::HasScalarType>::Scalar` and the
//! path-qualified trait becomes the supertrait/bound. `RectangleArea` provides the
//! area, and `Rectangle` fixes the scalar with `UseType<f64>`.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use std::ops::Mul;

use cgp::prelude::*;

mod scalar {
    use cgp::prelude::*;

    #[cgp_type]
    pub trait HasScalarType {
        type Scalar;
    }
}

// `scalar::HasScalarType` is referenced only by path; its bare name is never
// imported into this scope.
#[cgp_component(AreaCalculator)]
#[use_type(scalar::HasScalarType.Scalar)]
pub trait CanCalculateArea {
    fn area(&self) -> Scalar;
}

#[cgp_impl(new RectangleArea)]
#[use_type(scalar::HasScalarType.Scalar)]
impl AreaCalculator
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

delegate_and_check_components! {
    Rectangle {
        scalar::ScalarTypeProviderComponent:
            UseType<f64>,
        AreaCalculatorComponent:
            RectangleArea,
    }
}

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    assert_eq!(rectangle.area(), 12.0);
}
