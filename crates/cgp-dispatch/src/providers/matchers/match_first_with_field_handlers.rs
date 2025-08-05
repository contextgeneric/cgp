use cgp_core::prelude::*;
use cgp_handler::UseInputDelegate;

use crate::providers::matchers::to_field_handlers::{
    HasFieldHandlers, MapExtractFirstFieldAndHandle,
};
use crate::{
    HandleFirstFieldValue, MatchFirstWithHandlers, MatchFirstWithHandlersMut,
    MatchFirstWithHandlersRef,
};

pub type MatchFirstWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<Provider>>;

pub type MatchFirstWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<HandleFirstFieldValue<Provider>>>;

pub type MatchFirstWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<Provider>>;

pub type MatchFirstWithValueHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<HandleFirstFieldValue<Provider>>>;

pub type MatchFirstWithFieldHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<Provider>>;

pub type MatchFirstWithValueHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<HandleFirstFieldValue<Provider>>>;

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    new MatchFirstWithFieldHandlersInputs<Provider> {
        (Input, Args): MatchFirstWithHandlers<Input::Handlers>
    }
}

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    new MatchFirstWithFieldHandlersInputsRef<Provider> {
        <'a> (&'a Input, Args):
            MatchFirstWithHandlersRef<Input::Handlers>
    }
}

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    new MatchFirstWithFieldHandlersInputsMut<Provider> {
        <'a> (&'a mut Input, Args):
            MatchFirstWithHandlersMut<Input::Handlers>
    }
}
