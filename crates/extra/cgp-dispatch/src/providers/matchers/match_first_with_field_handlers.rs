use cgp::prelude::*;
use cgp_handler::UseInputDelegate;

use crate::providers::matchers::to_field_handlers::{
    HasFieldHandlers, MapExtractFirstFieldAndHandle,
};
use crate::{
    HandleFirstFieldValue, MatchFirstWithHandlers, MatchFirstWithHandlersMut,
    MatchFirstWithHandlersRef,
};

/// The matcher for an (Input, Args) input: it matches the enum in the first element and passes the
/// extra arguments to every per-variant handler.
///
/// See
/// [`MatchFirstWithFieldHandlers`](https://contextgeneric.dev/docs/reference/providers/dispatch/match_first_with_handlers)
/// .
pub type MatchFirstWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<Provider>>;

/// Matches the first field of a value with value handlers.
pub type MatchFirstWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<HandleFirstFieldValue<Provider>>>;

/// Matches the first field of a borrowed value with field handlers.
pub type MatchFirstWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<Provider>>;

/// Matches the first field of a borrowed value with value handlers.
pub type MatchFirstWithValueHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<HandleFirstFieldValue<Provider>>>;

/// Matches the first field of a mutably borrowed value with field handlers.
pub type MatchFirstWithFieldHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<Provider>>;

/// Matches the first field of a mutably borrowed value with value handlers.
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
