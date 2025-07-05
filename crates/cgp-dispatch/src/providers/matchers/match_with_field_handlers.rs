use cgp_core::prelude::*;
use cgp_handler::{ComputerComponent, HandleFieldValue, HandlerComponent, TryComputerComponent};

use crate::providers::matchers::to_field_handlers::ToInputFieldHandlers;
use crate::MatchWithHandlers;

pub struct MatchWithFieldHandlers<Input, Provider = UseContext>(pub PhantomData<(Input, Provider)>);

pub type MatchWithValueHandlers<Input, Provider = UseContext> =
    MatchWithFieldHandlers<Input, HandleFieldValue<Provider>>;

delegate_components! {
    <Input: ToInputFieldHandlers<Provider>, Provider>
    MatchWithFieldHandlers<Input, Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            MatchWithHandlers<Input::Handlers>
    }
}
