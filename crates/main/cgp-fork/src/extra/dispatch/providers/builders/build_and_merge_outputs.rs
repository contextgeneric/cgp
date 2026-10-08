use crate::core::field::traits::MapFields;
use crate::core::prelude::*;
use crate::extra::dispatch::{BuildAndMerge, BuildWithHandlers};
use crate::extra::handler::{
    ComputerComponent, ComputerRefComponent, HandlerComponent, HandlerRefComponent,
    TryComputerComponent, TryComputerRefComponent,
};

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
