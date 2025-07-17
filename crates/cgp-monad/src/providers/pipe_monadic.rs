use cgp_core::prelude::*;

use crate::providers::ComposeHandlers;
use crate::traits::MonadicBind;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

delegate_components! {
    <Component, Provider, M, Providers: BindProviders<M, Provider = Provider>>
    PipeMonadic<M, Providers> {
        Component: Provider,
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
