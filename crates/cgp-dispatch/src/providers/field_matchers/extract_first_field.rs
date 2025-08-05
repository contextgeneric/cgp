use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct ExtractFirstFieldAndHandle<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Args, Tag, Value, Provider, Output, Remainder>
    Computer<Context, Code, (Input, Args)> for ExtractFirstFieldAndHandle<Tag, Provider>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: Computer<Context, Code, (Field<Tag, Value>, Args), Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Output, Remainder> {
        let value = input.extract_field(PhantomData::<Tag>)?;
        let output = Provider::compute(context, tag, (value.into(), args));
        Ok(output)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Tag, Value, Provider, Output, Remainder>
    TryComputer<Context, Code, (Input, Args)> for ExtractFirstFieldAndHandle<Tag, Provider>
where
    Context: HasErrorType,
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: TryComputer<Context, Code, (Field<Tag, Value>, Args), Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn try_compute(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Result<Output, Remainder>, Context::Error> {
        let value = input.extract_field(PhantomData::<Tag>);

        match value {
            Ok(value) => {
                let output = Provider::try_compute(context, tag, (value.into(), args))?;
                Ok(Ok(output))
            }
            Err(remainder) => Ok(Err(remainder)),
        }
    }
}

#[cgp_provider]
impl<
        Context,
        Code: Send,
        Input: Send,
        Args: Send,
        Tag: Send,
        Value: Send,
        Provider,
        Output: Send,
        Remainder: Send,
    > Handler<Context, Code, (Input, Args)> for ExtractFirstFieldAndHandle<Tag, Provider>
where
    Context: HasAsyncErrorType,
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: Handler<Context, Code, (Field<Tag, Value>, Args), Output = Output>,
{
    type Output = Result<Output, Remainder>;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Result<Output, Remainder>, Context::Error> {
        let value = input.extract_field(PhantomData::<Tag>);

        match value {
            Ok(value) => {
                let output = Provider::handle(context, tag, (value.into(), args)).await?;
                Ok(Ok(output))
            }
            Err(remainder) => Ok(Err(remainder)),
        }
    }
}
