use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::ComposeHandlers;

pub struct PipeHandlers<Providers>(pub PhantomData<Providers>);

delegate_components! {
    <Component, Provider, Providers: ComposeProviders<Provider = Provider>>
    PipeHandlers<Providers> {
        Component: Provider,
    }
}

trait ComposeProviders {
    type Provider;
}

impl<ProviderA, ProviderB, RestProviders, OutProviders> ComposeProviders
    for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    Cons<ProviderB, RestProviders>: ComposeProviders<Provider = OutProviders>,
{
    type Provider = ComposeHandlers<ProviderA, OutProviders>;
}

impl<Provider> ComposeProviders for Cons<Provider, Nil> {
    type Provider = Provider;
}
