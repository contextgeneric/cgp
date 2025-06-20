use cgp_core::field::CanBuildFrom;
use cgp_core::prelude::*;
use cgp_handler::Computer;

use crate::BuilderComputer;

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
