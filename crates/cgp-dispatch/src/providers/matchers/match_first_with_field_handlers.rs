use cgp_core::prelude::*;
use cgp_handler::{PromoteRef, UseInputDelegate};

use crate::providers::matchers::to_first_field_handlers::HasFirstFieldHandlers;
use crate::{HandleFirstFieldValue, MatchFirstWithHandlers, MatchFirstWithHandlersRef};

pub type MatchFirstWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<Provider>>;

pub type MatchFirstWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<HandleFirstFieldValue<Provider>>>;

pub type MatchFirstWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<PromoteRef<Provider>>>;

pub type MatchFirstWithValueHandlersRef<Provider = UseContext> = UseInputDelegate<
    MatchFirstWithFieldHandlersInputsRef<HandleFirstFieldValue<PromoteRef<Provider>>>,
>;

delegate_components! {
    <Input: HasFirstFieldHandlers<Provider>, Args, Provider>
    new MatchFirstWithFieldHandlersInputs<Provider> {
        (Input, Args): MatchFirstWithHandlers<Input::Handlers>
    }
}

delegate_components! {
    <Input: HasFirstFieldHandlers<Provider>, Provider>
    new MatchFirstWithFieldHandlersInputsRef<Provider> {
        Input:
            PromoteRef<MatchFirstWithHandlersRef<Input::Handlers>>
    }
}
