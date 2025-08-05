use cgp_core::prelude::*;
use cgp_handler::{HandleFieldValue, PromoteRef, UseInputDelegate};

use crate::providers::matchers::to_field_handlers::{HasFieldHandlers, MapExtractFieldAndHandle};
use crate::{MatchWithHandlers, MatchWithHandlersRef};

pub type MatchWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<Provider>>;

pub type MatchWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<HandleFieldValue<Provider>>>;

pub type MatchWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputsRef<PromoteRef<Provider>>>;

pub type MatchWithValueHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputsRef<HandleFieldValue<PromoteRef<Provider>>>>;

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    new MatchWithFieldHandlersInputs<Provider> {
        Input: MatchWithHandlers<Input::Handlers>
    }
}

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    new MatchWithFieldHandlersInputsRef<Provider> {
        Input:
            PromoteRef<MatchWithHandlersRef<Input::Handlers>>
    }
}
