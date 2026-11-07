use cgp::field::traits::MapFields;
use cgp::prelude::*;
use cgp_handler::{
    ComputerComponent, ComputerRefComponent, HandlerComponent, HandlerRefComponent,
    TryComputerComponent, TryComputerRefComponent,
};

use crate::{BuildAndMerge, BuildWithHandlers};

delegate_components! {
    <Output, Handlers: MapFields<ToBuildAndMergeHandler>>
    new BuildAndMergeOutputs<Output, Handlers> {
        [
            ComputerComponent,
            ComputerRefComponent,
            TryComputerComponent,
            TryComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ]:
            BuildWithHandlers<Output, Handlers::Mapped>
    }
}

/// Maps each handler to a [`BuildAndMerge`] wrapper.
pub struct ToBuildAndMergeHandler;

impl MapType for ToBuildAndMergeHandler {
    type Map<Handler> = BuildAndMerge<Handler>;
}
