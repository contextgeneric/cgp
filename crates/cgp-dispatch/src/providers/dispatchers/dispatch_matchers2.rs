use cgp_core::prelude::*;
use cgp_handler::{
    ComputerComponent, HandlerComponent, OkMonadic, PipeMonadic, TryComputerComponent,
};

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
