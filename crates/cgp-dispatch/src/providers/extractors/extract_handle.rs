use cgp_core::field::CanDowncastFields;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct ExtractAndHandle<Input, Provider = UseContext>(pub PhantomData<(Input, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Provider, Inner, Output, Remainder> Computer<Context, Code, Input>
    for ExtractAndHandle<Inner, Provider>
where
    Input: CanDowncastFields<Inner, Remainder = Remainder>,
    Provider: Computer<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        let inner = input.downcast_fields(PhantomData::<Inner>)?;
        let output = Provider::compute(context, tag, inner);
        Ok(output)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Inner, Output, Remainder> TryComputer<Context, Code, Input>
    for ExtractAndHandle<Inner, Provider>
where
    Context: HasErrorType,
    Input: CanDowncastFields<Inner, Remainder = Remainder>,
    Provider: TryComputer<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Output, Remainder>, Context::Error> {
        let inner = input.downcast_fields(PhantomData::<Inner>);

        match inner {
            Ok(inner) => {
                let output = Provider::try_compute(context, tag, inner)?;
                Ok(Ok(output))
            }
            Err(remainder) => Ok(Err(remainder)),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Provider, Inner: Send, Output: Send, Remainder: Send>
    Handler<Context, Code, Input> for ExtractAndHandle<Inner, Provider>
where
    Context: HasAsyncErrorType,
    Input: CanDowncastFields<Inner, Remainder = Remainder>,
    Provider: Handler<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Output, Remainder>, Context::Error> {
        let inner = input.downcast_fields(PhantomData::<Inner>);

        match inner {
            Ok(inner) => {
                let output = Provider::handle(context, tag, inner).await?;
                Ok(Ok(output))
            }
            Err(remainder) => Ok(Err(remainder)),
        }
    }
}
