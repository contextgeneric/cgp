use cgp_core::prelude::*;

use crate::{
    Computer, ComputerComponent, HandlerComponent, Promote, TryComputer, TryComputerComponent,
};

pub struct TryPromote<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    TryPromote<Provider> {
        HandlerComponent: Promote<TryPromote<Provider>>,
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> TryComputer<Context, Code, Input>
    for TryPromote<Provider>
where
    Context: HasErrorType,
    Provider: Computer<Context, Code, Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> Computer<Context, Code, Input> for TryPromote<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Result<Output, Context::Error>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::try_compute(context, tag, input)
    }
}
