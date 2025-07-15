use cgp_core::prelude::*;

use crate::traits::MonadicBind;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

impl<M, ProviderA, ProviderB, RestProviders, Component> DelegateComponent<Component>
    for PipeMonadic<M, Cons<ProviderA, Cons<ProviderB, RestProviders>>>
where
    M: MonadicBind<ProviderA, PipeMonadic<M, Cons<ProviderB, RestProviders>>>,
{
    type Delegate = M::Provider;
}

impl<M, Provider, Component> DelegateComponent<Component> for PipeMonadic<M, Cons<Provider, Nil>> {
    type Delegate = Provider;
}
