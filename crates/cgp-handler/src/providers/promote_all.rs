use cgp_core::prelude::*;

use crate::{
    AsyncComputerComponent, AsyncComputerRefComponent, ComputerComponent, ComputerRefComponent,
    HandlerComponent, HandlerRefComponent, Promote, PromoteAsync, PromoteRef, TryComputerComponent,
    TryComputerRefComponent, TryPromote,
};

delegate_components! {
    <Provider>
    new PromoteComputer<Provider> {
        ComputerRefComponent: PromoteRef<Provider>,
        TryComputerComponent: Promote<Provider>,
        TryComputerRefComponent: PromoteRef<Provider>,
        AsyncComputerComponent: PromoteAsync<Provider>,
        AsyncComputerRefComponent: PromoteRef<Provider>,
        HandlerComponent: PromoteAsync<Provider>,
        HandlerRefComponent: PromoteRef<Provider>,
    }
}

delegate_components! {
    <Provider>
    new PromoteTryComputer<Provider> {
        TryComputerComponent: TryPromote<Provider>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerComponent,
            AsyncComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ] ->
            PromoteComputer<Provider>,
    }
}

delegate_components! {
    <Provider>
    new PromoteProducer<Provider> {
        ComputerComponent: Promote<Provider>,
        [
            ComputerRefComponent,
            TryComputerComponent,
            TryComputerRefComponent,
            AsyncComputerComponent,
            AsyncComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ] ->
            PromoteComputer<Provider>,
    }
}

delegate_components! {
    <Provider>
    new PromoteAsyncComputer<Provider> {
        AsyncComputerRefComponent: PromoteRef<Provider>,
        HandlerComponent: Promote<Provider>,
        HandlerRefComponent: PromoteRef<Provider>,
    }
}

delegate_components! {
    <Provider>
    new PromoteHandler<Provider> {
        HandlerComponent: TryPromote<Provider>,
        [
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ] ->
            PromoteAsyncComputer<Provider>,
    }
}
