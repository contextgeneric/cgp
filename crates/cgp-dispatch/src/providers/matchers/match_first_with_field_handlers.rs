use cgp_core::prelude::*;
use cgp_handler::UseInputDelegate;

use crate::providers::matchers::to_first_field_handlers::HasFirstFieldHandlers;
use crate::{HandleFirstFieldValue, MatchFirstWithHandlers};

pub type MatchFirstWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<Provider>>;

pub type MatchFirstWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<HandleFirstFieldValue<Provider>>>;

delegate_components! {
    <Input: HasFirstFieldHandlers<Provider>, Args, Provider>
    new MatchFirstWithFieldHandlersInputs<Provider> {
        (Input, Args): MatchFirstWithHandlers<Input::Handlers>
    }
}
