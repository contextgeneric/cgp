use cgp_core::field::CanDowncastFields;
use cgp_core::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

pub struct DowncastAndHandle<Input, Provider = UseContext>(pub PhantomData<(Input, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Provider, Inner, Output, Remainder> Computer<Context, Code, Input>
    for DowncastAndHandle<Inner, Provider>
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
impl<Context, Code, Input, Provider, Inner, Output, Remainder> AsyncComputer<Context, Code, Input>
    for DowncastAndHandle<Inner, Provider>
where
    Input: CanDowncastFields<Inner, Remainder = Remainder>,
    Provider: AsyncComputer<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    async fn compute_async(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        let inner = input.downcast_fields(PhantomData::<Inner>);

        match inner {
            Ok(inner) => {
                let output = Provider::compute_async(context, tag, inner).await;
                Ok(output)
            }
            Err(remainder) => Err(remainder),
        }
    }
}
