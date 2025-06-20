use cgp_core::field::CanExtractInto;
use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, Handler, HandlerComponent};

pub struct ExtractAndHandle<Input, Provider = UseContext>(pub PhantomData<(Input, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Provider, Inner, Output, Remainder> Computer<Context, Code, Input>
    for ExtractAndHandle<Inner, Provider>
where
    Input: CanExtractInto<Inner, Remainder = Remainder>,
    Provider: Computer<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        let inner = input.extract_into(PhantomData::<Inner>)?;
        let output = Provider::compute(context, tag, inner);
        Ok(output)
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Provider, Inner: Send, Output: Send, Remainder: Send>
    Handler<Context, Code, Input> for ExtractAndHandle<Inner, Provider>
where
    Context: HasAsyncErrorType,
    Input: CanExtractInto<Inner, Remainder = Remainder>,
    Provider: Handler<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Result<Output, Remainder>, Context::Error> {
        let inner = input.extract_into(PhantomData::<Inner>);

        match inner {
            Ok(inner) => {
                let output = Provider::handle(context, tag, inner).await?;
                Ok(Ok(output))
            }
            Err(remainder) => Ok(Err(remainder)),
        }
    }
}
