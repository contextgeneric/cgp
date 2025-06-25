use cgp_core::prelude::*;
use cgp_handler::{Computer, Handler, TryComputer};

use crate::{BuilderComputer, BuilderHandler, TryBuilderComputer};

pub struct HandleAndBuildField<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

impl<Context, Code, Tag, Value, Provider, Output, Builder> BuilderComputer<Context, Code, Builder>
    for HandleAndBuildField<Tag, Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    fn build(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let value = Provider::compute(context, code, &builder);
        builder.build_field(PhantomData::<Tag>, value)
    }
}

impl<Context, Code, Tag, Value, Provider, Output, Builder>
    TryBuilderComputer<Context, Code, Builder> for HandleAndBuildField<Tag, Provider>
where
    Context: HasErrorType,
    Provider: for<'a> TryComputer<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let value = Provider::try_compute(context, code, &builder)?;
        Ok(builder.build_field(PhantomData::<Tag>, value))
    }
}

impl<Context, Code: Send, Builder: Send + Sync, Tag, Value, Provider, Output>
    BuilderHandler<Context, Code, Builder> for HandleAndBuildField<Tag, Provider>
where
    Context: HasAsyncErrorType,
    Provider: for<'a> Handler<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let value = Provider::handle(context, code, &builder).await?;
        Ok(builder.build_field(PhantomData::<Tag>, value))
    }
}
