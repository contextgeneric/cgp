use cgp_core::prelude::*;
use cgp_handler::{PromoteRef, UseInputDelegate};

use crate::providers::matchers::to_field_handlers::{HasFieldHandlers, MapExtractFieldAndHandle};
use crate::{HandleFieldValue, MatchWithHandlers, MatchWithHandlersMut, MatchWithHandlersRef};

pub type MatchWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<Provider>>;

pub type MatchWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<HandleFieldValue<Provider>>>;

pub type MatchWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputsRef<PromoteRef<Provider>>>;

pub type MatchWithValueHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputsRef<HandleFieldValue<Provider>>>;

pub type MatchWithValueHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputsMut<HandleFieldValue<Provider>>>;

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    new MatchWithFieldHandlersInputs<Provider> {
        Input: MatchWithHandlers<Input::Handlers>
    }
}

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    new MatchWithFieldHandlersInputsRef<Provider> {
        <'a> &'a Input:
            MatchWithHandlersRef<Input::Handlers>,
    }
}

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    new MatchWithFieldHandlersInputsMut<Provider> {
        <'a> &'a mut Input:
            MatchWithHandlersMut<Input::Handlers>
    }
}
