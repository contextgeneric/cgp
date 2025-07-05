use cgp_core::prelude::*;
use cgp_handler::{
    ComputerComponent, ComputerRefComponent, HandleFieldValue, HandlerComponent,
    HandlerRefComponent, PromoteRef, TryComputerComponent, TryComputerRefComponent,
};

use crate::providers::matchers::to_field_handlers::{
    ToInputFieldHandlers, ToInputFieldHandlersRef,
};
use crate::{MatchWithHandlers, MatchWithHandlersRef};

pub struct MatchWithFieldHandlers<Input, Provider = UseContext>(pub PhantomData<(Input, Provider)>);

pub type MatchWithValueHandlers<Input, Provider = UseContext> =
    MatchWithFieldHandlers<Input, HandleFieldValue<Provider>>;

pub type MatchWithFieldHandlersRef<Input, Provider = UseContext> =
    MatchWithFieldHandlers<Input, PromoteRef<Provider>>;

pub type MatchWithValueHandlersRef<Input, Provider = UseContext> =
    MatchWithFieldHandlers<Input, HandleFieldValue<PromoteRef<Provider>>>;

delegate_components! {
    <Input: ToInputFieldHandlers<Provider>, Provider>
    MatchWithFieldHandlers<Input, Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            MatchWithHandlers<Input::Handlers>,
    }
}

delegate_components! {
    <
        Input: for<'a> ToInputFieldHandlersRef<'a, Provider, Handlers = Handlers>,
        Provider,
        Handlers,
    >
    MatchWithFieldHandlers<Input, Provider> {
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<MatchWithHandlersRef<Handlers>>
    }
}
