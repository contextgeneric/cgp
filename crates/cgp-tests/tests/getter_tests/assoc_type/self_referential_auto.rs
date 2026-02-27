use core::ops::Mul;

use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasScalarType {
    type Scalar: Mul<Output = Self::Scalar> + Clone;

    fn scalar(&self) -> &Self::Scalar;
}

#[derive(HasField)]
pub struct App {
    pub scalar: f64,
}

#[test]
fn test_auto_getter_scalar() {
    let app = App { scalar: 2.0 };

    assert_eq!(*app.scalar(), 2.0);
}
