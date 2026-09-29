use crate::traits::{ContainsValue, LiftValue, MonadicBind, MonadicTrans};

pub struct IdentMonadic;

impl<M> MonadicTrans<M> for IdentMonadic {
    type M = M;
}

impl<Provider> MonadicBind<Provider> for IdentMonadic {
    type Provider = Provider;
}

impl<T> ContainsValue<T> for IdentMonadic {
    type Value = T;
}

impl<T> LiftValue<T, T> for IdentMonadic {
    type Output = T;

    fn lift_value(value: T) -> T {
        value
    }

    fn lift_output(value: T) -> T {
        value
    }
}
