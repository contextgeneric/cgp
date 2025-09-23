use cgp_core::prelude::*;
use cgp_handler::{
    AsyncComputerComponent, AsyncComputerRefComponent, ComputerComponent, ComputerRefComponent,
    HandlerComponent, HandlerRefComponent, PromoteRef, TryComputerComponent,
    TryComputerRefComponent, UseInputDelegate,
};

use crate::providers::matchers::to_field_handlers::{HasFieldHandlers, MapExtractFieldAndHandle};
use crate::{HandleFieldValue, MatchWithHandlers, MatchWithHandlersMut, MatchWithHandlersRef};

pub type MatchWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<Provider>>;

pub type MatchWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<HandleFieldValue<Provider>>>;

pub struct MatchWithFieldHandlersRef<Provider = UseContext>(pub PhantomData<Provider>);

pub struct MatchWithValueHandlersRef<Provider = UseContext>(pub PhantomData<Provider>);

pub struct MatchWithValueHandlersMut<Provider = UseContext>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    MatchWithFieldHandlersRef<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            AsyncComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsRef<Provider>>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<UseInputDelegate<MatchWithFieldHandlersInputsRef<PromoteRef<Provider>>>>,
    }
}

delegate_components! {
    <Provider>
    MatchWithValueHandlersRef<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            AsyncComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsRef<HandleFieldValue<Provider>>>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<UseInputDelegate<MatchWithFieldHandlersInputsRef<HandleFieldValue<PromoteRef<Provider>>>>>,
    }
}

delegate_components! {
    <Provider>
    MatchWithValueHandlersMut<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            AsyncComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsMut<HandleFieldValue<Provider>>>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<UseInputDelegate<MatchWithFieldHandlersInputsMut<HandleFieldValue<PromoteRef<Provider>>>>>,
    }
}

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
