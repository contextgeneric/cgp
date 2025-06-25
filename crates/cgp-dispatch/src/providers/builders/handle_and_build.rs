use cgp_core::field::CanBuildFrom;
use cgp_core::prelude::*;
use cgp_handler::{Computer, Handler, TryComputer};

use crate::{BuilderComputer, BuilderHandler, TryBuilderComputer};

pub struct HandleAndBuild<Provider = UseContext>(pub PhantomData<Provider>);

impl<Context, Code, Builder, Provider, Output, Res> BuilderComputer<Context, Code, Builder>
    for HandleAndBuild<Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Output>,
{
    type Output = Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let output = Provider::compute(context, code, &builder);
        builder.build_from(output)
    }
}

impl<Context, Code, Builder, Provider, Output, Res> TryBuilderComputer<Context, Code, Builder>
    for HandleAndBuild<Provider>
where
    Context: HasErrorType,
    Provider: for<'a> TryComputer<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let output = Provider::try_compute(context, code, &builder)?;
        Ok(builder.build_from(output))
    }
}

impl<Context, Code: Send, Builder: Send + Sync, Provider, Output, Res>
    BuilderHandler<Context, Code, Builder> for HandleAndBuild<Provider>
where
    Context: HasAsyncErrorType,
    Provider: for<'a> Handler<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let output = Provider::handle(context, code, &builder).await?;
        Ok(builder.build_from(output))
    }
}
