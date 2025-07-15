use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{ComputerComponent, HandlerComponent, TryComputerComponent};
use cgp_monad::{monads::ok::OkMonadic, providers::PipeMonadic};

pub struct DispatchMatchers<Handlers>(pub PhantomData<Handlers>);

delegate_components! {
    <Providers>
    DispatchMatchers<Providers> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            PipeMonadic<OkMonadic, Providers>,
    }
}
