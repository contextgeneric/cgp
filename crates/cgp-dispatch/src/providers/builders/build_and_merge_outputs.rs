use cgp_core::field::traits::MapFields;
use cgp_core::prelude::*;
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

pub struct ToBuildAndMergeHandler;

impl MapType for ToBuildAndMergeHandler {
    type Map<Handler> = BuildAndMerge<Handler>;
}
