use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::traits::{ContainsValue, MonadicLift};

pub struct IdentMonadic;

impl<ProviderA, ProviderB> Compose<ProviderA, ProviderB> for IdentMonadic {
    type Provider = ComposeIdent<ProviderA, ProviderB>;
}

pub struct ComposeIdent<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);

impl<T> ContainsValue<T> for IdentMonadic {
    type Value = T;
}

impl<T> MonadicLift<T, T> for IdentMonadic {
    type Output = T;

    fn lift_value(value: T) -> T {
        value
    }

    fn lift_output(value: T) -> T {
        value
    }
}
