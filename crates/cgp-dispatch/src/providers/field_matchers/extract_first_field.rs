use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

pub struct ExtractFirstFieldAndHandle<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Args, Tag, Value, Provider, Output, Remainder>
    Computer<Context, Code, (Input, Args)> for ExtractFirstFieldAndHandle<Tag, Provider>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: Computer<Context, Code, (Field<Tag, Value>, Args), Output = Output>,
{
    type Output = Result<Output, (Remainder, Args)>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Output, (Remainder, Args)> {
        let res = input.extract_field(PhantomData::<Tag>);
        match res {
            Ok(value) => {
                let output = Provider::compute(context, tag, (value.into(), args));
                Ok(output)
            }
            Err(remainder) => Err((remainder, args)),
        }
    }
}

#[cgp_provider]
impl<
        Context: Async,
        Code: Send,
        Input: Send,
        Args: Send,
        Tag: Send,
        Value: Send,
        Provider,
        Output: Send,
        Remainder: Send,
    > AsyncComputer<Context, Code, (Input, Args)> for ExtractFirstFieldAndHandle<Tag, Provider>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: AsyncComputer<Context, Code, (Field<Tag, Value>, Args), Output = Output>,
{
    type Output = Result<Output, (Remainder, Args)>;

    async fn compute_async(
        context: &Context,
        tag: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Result<Output, (Remainder, Args)> {
        let value = input.extract_field(PhantomData::<Tag>);

        match value {
            Ok(value) => {
                let output = Provider::compute_async(context, tag, (value.into(), args)).await;
                Ok(output)
            }
            Err(remainder) => Err((remainder, args)),
        }
    }
}
