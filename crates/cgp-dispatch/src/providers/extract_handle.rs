use cgp_core::field::CanExtractInto;
use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

pub struct ExtractAndHandle<Input, Handler>(pub PhantomData<(Input, Handler)>);

#[cgp_provider]
impl<Context, Code, Input, Handler, Inner, Output, Remainder> Computer<Context, Code, Input>
    for ExtractAndHandle<Inner, Handler>
where
    Input: CanExtractInto<Inner, Remainder = Remainder>,
    Handler: Computer<Context, Code, Inner, Output = Output>,
{
    type Output = Result<Output, Remainder>;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        let inner = input.extract_into(PhantomData::<Inner>)?;
        let output = Handler::compute(context, tag, inner);
        Ok(output)
    }
}
