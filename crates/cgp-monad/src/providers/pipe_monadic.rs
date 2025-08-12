use cgp_core::field::MapFields;
use cgp_core::prelude::*;
use cgp_handler::{
    AsyncComputerComponent, ComposeHandlers, ComputerComponent, HandlerComponent,
    TryComputerComponent, TryPromote,
};

use crate::monadic::err::ErrMonadic;
use crate::traits::{MonadicBind, MonadicTrans};

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

delegate_components! {
    <Provider, M, Providers: BindProviders<M, Provider = Provider>>
    PipeMonadic<M, Providers> {
        [
            ComputerComponent,
            AsyncComputerComponent,
        ]: Provider,
    }
}

// Support monadic piping of TryComputer by first demoting them to Computer,
// together with a monad transformer application of ErrMonadic to the base monad,
// compose them, and then TryPromote them again back into TryComputer.

delegate_components! {
    <
        Provider,
        M1: MonadicTrans<ErrMonadic, M = M2>,
        M2,
        ProvidersA: MapFields<TryPromoteProviders, Mapped = ProvidersB>,
        ProvidersB: BindProviders<M2, Provider = Provider>,
    >
    PipeMonadic<M1, ProvidersA> {
        TryComputerComponent: TryPromote<Provider>,
        HandlerComponent: TryPromote<Provider>,
    }
}

pub struct TryPromoteProviders;

impl MapType for TryPromoteProviders {
    type Map<Provider> = TryPromote<Provider>;
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
