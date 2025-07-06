use cgp_core::prelude::*;
use cgp_handler::{
    ComputerComponent, ComputerRefComponent, HandleFieldValue, HandlerComponent,
    HandlerRefComponent, PromoteRef, TryComputerComponent, TryComputerRefComponent,
    UseInputDelegate,
};

use crate::providers::matchers::to_field_handlers::{
    ToInputFieldHandlers, ToInputFieldHandlersRef,
};
use crate::{MatchWithHandlers, MatchWithHandlersRef};

pub struct MatchWithFieldHandlers<Provider = UseContext>(pub PhantomData<Provider>);

pub type MatchWithValueHandlers<Provider = UseContext> =
    MatchWithFieldHandlers<HandleFieldValue<Provider>>;

pub type MatchWithFieldHandlersRef<Provider = UseContext> =
    MatchWithFieldHandlers<PromoteRef<Provider>>;

pub type MatchWithValueHandlersRef<Provider = UseContext> =
    MatchWithFieldHandlers<HandleFieldValue<PromoteRef<Provider>>>;

delegate_components! {
    <Provider>
    MatchWithFieldHandlers<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputs<Provider>>,
    }
}

delegate_components! {
    <Input: ToInputFieldHandlers<Provider>, Provider>
    new MatchWithFieldHandlersInputs<Provider> {
        Input: MatchWithHandlers<Input::Handlers>
    }
}

delegate_components! {
    <Provider>
    MatchWithFieldHandlers<Provider> {
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            HandlerRefComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsRef<Provider>>
    }
}

delegate_components! {
    <
        Input: for<'a> ToInputFieldHandlersRef<'a, Provider, Handlers = Handlers>,
        Provider,
        Handlers,
    >
    new MatchWithFieldHandlersInputsRef<Provider> {
        Input:
            PromoteRef<MatchWithHandlersRef<Handlers>>
    }
}
