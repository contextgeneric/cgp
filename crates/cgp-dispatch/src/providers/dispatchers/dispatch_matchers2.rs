use cgp_core::prelude::*;
use cgp_handler::{ComputerComponent, HandlerComponent, TryComputerComponent};
use cgp_monad::monads::OkMonadic;
use cgp_monad::providers::PipeMonadic;

delegate_components! {
    <Providers>
    new DispatchMatchers2<Providers> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            PipeMonadic<OkMonadic, Providers>,
    }
}
