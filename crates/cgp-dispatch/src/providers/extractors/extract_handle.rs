use cgp_core::field::CanExtractInto;
use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

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
