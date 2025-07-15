use cgp_core::prelude::*;

use crate::traits::MonadicBind;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

delegate_components! {
    <
        ProviderA, ProviderB, RestProviders, Component,
        M: MonadicBind<ProviderA, PipeMonadic<M, Cons<ProviderB, RestProviders>>>,
    >
    PipeMonadic<M, Cons<ProviderA, Cons<ProviderB, RestProviders>>> {
        Component: M::Provider,
    }
}

delegate_components! {
    <M, Provider, Component>
    PipeMonadic<M, Cons<Provider, Nil>> {
        Component: Provider,
    }
}
