use cgp::prelude::*;
use cgp_handler::{AsyncComputer, AsyncComputerComponent, Computer, ComputerComponent};

/// The matcher adapter that tries one variant, hands its payload to an inner provider on a match,
/// and passes the remainder on when the variant does not match.
///
/// See
/// [`ExtractFieldAndHandle`](https://contextgeneric.dev/docs/reference/providers/dispatch/extract_field_and_handle)
/// .
pub struct ExtractFieldAndHandle<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Tag, Value, Provider, Output, Remainder> Computer<Context, Code, Input>
    for ExtractFieldAndHandle<Tag, Provider>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: Computer<Context, Code, Field<Tag, Value>, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        let value = input.extract_field(PhantomData::<Tag>)?;
        let output = Provider::compute(context, tag, value.into());
        Ok(output)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Value, Provider, Output, Remainder>
    AsyncComputer<Context, Code, Input> for ExtractFieldAndHandle<Tag, Provider>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: AsyncComputer<Context, Code, Field<Tag, Value>, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    async fn compute_async(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        let value = input.extract_field(PhantomData::<Tag>);

        match value {
            Ok(value) => {
                let output = Provider::compute_async(context, tag, value.into()).await;
                Ok(output)
            }
            Err(remainder) => Err(remainder),
        }
    }
}
