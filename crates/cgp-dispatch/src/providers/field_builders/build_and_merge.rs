use cgp_core::field::impls::CanBuildFrom;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct BuildAndMerge<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Builder, Provider, Output, Res> Computer<Context, Code, Builder>
    for BuildAndMerge<Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let output = Provider::compute(context, code, &builder);
        builder.build_from(output)
    }
}

#[cgp_provider]
impl<Context, Code, Builder, Provider, Output, Res> TryComputer<Context, Code, Builder>
    for BuildAndMerge<Provider>
where
    Context: HasErrorType,
    Provider: for<'a> TryComputer<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let output = Provider::try_compute(context, code, &builder)?;
        Ok(builder.build_from(output))
    }
}

#[cgp_provider]
impl<Context, Code, Builder, Provider, Output, Res> Handler<Context, Code, Builder>
    for BuildAndMerge<Provider>
where
    Context: HasErrorType,
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
