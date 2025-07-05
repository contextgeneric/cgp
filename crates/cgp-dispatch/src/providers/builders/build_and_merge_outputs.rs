use cgp_core::field::MapFields;
use cgp_core::prelude::*;
use cgp_handler::{
    ComputerComponent, ComputerRefComponent, HandlerComponent, HandlerRefComponent,
    TryComputerComponent, TryComputerRefComponent,
};

use crate::{BuildAndMerge, BuildWithHandlers};

pub struct BuildAndMergeOutputs<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

delegate_components! {
    <Output, Handlers: MapFields<ToBuildAndMergeHandler>>
    BuildAndMergeOutputs<Output, Handlers> {
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
    type Mapped<Handler> = BuildAndMerge<Handler>;
}
