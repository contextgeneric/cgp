use cgp_core::field::{MapFields, MapType};

use crate::{BuildAndMerge, BuildWithHandlers};

pub type BuildAndMergeOutputs<Output, Handlers> =
    BuildWithHandlers<Output, <Handlers as MapFields<ToBuildAndMergeHandler>>::Mapped>;

pub struct ToBuildAndMergeHandler;

impl MapType for ToBuildAndMergeHandler {
    type Mapped<Handler> = BuildAndMerge<Handler>;
}
