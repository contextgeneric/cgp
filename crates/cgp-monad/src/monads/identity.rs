use crate::traits::{CanWrap, ContainsValue, Functorial};

pub struct IdentMonadic;

impl<T> ContainsValue<T> for IdentMonadic {
    type Value = T;
}

impl<T> CanWrap<T> for IdentMonadic {
    fn wrap(value: T) -> T {
        value
    }
}

impl<T1, T2> Functorial<T1, T2> for IdentMonadic {
    type Output = T2;

    fn map(value: T1, f: impl FnOnce(T1) -> T2) -> T2 {
        f(value)
    }
}
