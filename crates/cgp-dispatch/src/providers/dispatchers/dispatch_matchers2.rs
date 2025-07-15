use cgp_core::prelude::*;
use cgp_handler::{ComputerComponent, HandlerComponent, TryComputerComponent};
use cgp_monad::monads::{IdentMonadic, OkMonadic, OkMonadicTrans};
use cgp_monad::providers::PipeMonadic;

delegate_components! {
    <Providers>
    new DispatchMatchers2<Providers> {
        [
            TryComputerComponent,
            HandlerComponent,
        ]:
            PipeMonadic<OkMonadic, Providers>,
        ComputerComponent:
            PipeMonadic<OkMonadicTrans<IdentMonadic>, Providers>,
    }
}
