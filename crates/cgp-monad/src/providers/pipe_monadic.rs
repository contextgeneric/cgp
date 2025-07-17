use cgp_core::prelude::*;
use cgp_handler::{ComposeHandlers, HandlerComponent, ComputerComponent, TryComputerComponent};

use crate::traits::MonadicBind;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

delegate_components! {
    <Provider, M, Providers: BindProviders<M, Provider = Provider>>
    PipeMonadic<M, Providers> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]: Provider,
    }
}

trait BindProviders<M> {
    type Provider;
}

impl<M, ProviderA, ProviderB, RestProviders, OutProviders> BindProviders<M>
    for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    Cons<ProviderB, RestProviders>: BindProviders<M, Provider = OutProviders>,
    M: MonadicBind<OutProviders>,
{
    type Provider = ComposeHandlers<ProviderA, M::Provider>;
}

impl<M, Provider> BindProviders<M> for Cons<Provider, Nil> {
    type Provider = Provider;
}
