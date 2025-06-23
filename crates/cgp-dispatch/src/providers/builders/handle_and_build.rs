use cgp_core::field::CanBuildFrom;
use cgp_core::prelude::*;
use cgp_handler::{Computer, TryComputer};

use crate::{BuilderComputer, TryBuilderComputer};

pub struct HandleAndBuild<Provider = UseContext>(pub PhantomData<Provider>);

impl<Context, Code, Input, Builder, Provider, Output> BuilderComputer<Context, Code, Input, Builder>
    for HandleAndBuild<Provider>
where
    Provider: Computer<Context, Code, Input>,
    Builder: CanBuildFrom<Provider::Output, Output = Output>,
{
    type Output = Output;

    fn build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Self::Output {
        let output = Provider::compute(context, code, input);
        builder.build_from(output)
    }
}

impl<Context, Code, Input, Builder, Provider, Output>
    TryBuilderComputer<Context, Code, Input, Builder> for HandleAndBuild<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input>,
    Builder: CanBuildFrom<Provider::Output, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let output = Provider::try_compute(context, code, input)?;
        Ok(builder.build_from(output))
    }
}
