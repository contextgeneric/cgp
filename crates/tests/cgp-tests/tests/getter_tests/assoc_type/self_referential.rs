use core::ops::Mul;

use cgp::prelude::*;

#[cgp_getter]
pub trait HasScalar {
    type Scalar: Mul<Output = Self::Scalar> + Clone;

    fn scalar(&self) -> &Self::Scalar;
}

#[derive(HasField)]
pub struct App {
    pub scalar: f64,
}

delegate_components! {
    App {
        ScalarGetterComponent:
            UseField<Symbol!("scalar")>,
    }
}

#[test]
fn test_auto_getter_scalar() {
    let app = App { scalar: 2.0 };

    assert_eq!(*app.scalar(), 2.0);
}
