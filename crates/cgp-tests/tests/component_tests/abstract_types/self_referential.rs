use core::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Mul<Output = Self::Scalar> + Clone;
}

pub struct App;

delegate_components! {
    App {
        ScalarTypeProviderComponent:
            UseType<f64>,
    }
}

check_components! {
    CanUseApp for App {
        ScalarTypeProviderComponent,
    }
}
