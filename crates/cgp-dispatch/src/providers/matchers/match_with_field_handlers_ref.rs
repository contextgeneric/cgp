use cgp_core::prelude::*;
use cgp_handler::{
    ComputerRefComponent, HandleFieldValue, HandlerRefComponent, PromoteRef,
    TryComputerRefComponent,
};

use crate::providers::matchers::to_field_handlers::ToInputFieldHandlersRef;
use crate::MatchWithHandlersRef;

pub type MatchWithFieldHandlersRef<Input, Provider = UseContext> =
    MatchWithFieldHandlersRefImpl<Input, PromoteRef<Provider>>;

pub type MatchWithValueHandlersRef<Input, Provider = UseContext> =
    MatchWithFieldHandlersRefImpl<Input, HandleFieldValue<PromoteRef<Provider>>>;

pub struct MatchWithFieldHandlersRefImpl<Input, Provider = UseContext>(
    pub PhantomData<(Input, Provider)>,
);

delegate_components! {
    <
        Input: for<'a> ToInputFieldHandlersRef<'a, Provider, Handlers = Handlers>,
        Provider,
        Handlers,
    >
    MatchWithFieldHandlersRefImpl<Input, Provider> {
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<MatchWithHandlersRef<Handlers>>
    }
}
