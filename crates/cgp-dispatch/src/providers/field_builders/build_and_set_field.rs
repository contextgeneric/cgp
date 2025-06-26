use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

pub struct BuildAndSetField<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

#[cgp_provider]
impl<Context, Code, Tag, Value, Provider, Output, Builder> Computer<Context, Code, Builder>
    for BuildAndSetField<Tag, Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let value = Provider::compute(context, code, &builder);
        builder.build_field(PhantomData::<Tag>, value)
    }
}

#[cgp_provider]
impl<Context, Code, Tag, Value, Provider, Output, Builder> TryComputer<Context, Code, Builder>
    for BuildAndSetField<Tag, Provider>
where
    Context: HasErrorType,
    Provider: for<'a> TryComputer<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let value = Provider::try_compute(context, code, &builder)?;
        Ok(builder.build_field(PhantomData::<Tag>, value))
    }
}

#[cgp_provider]
impl<Context, Code: Send, Builder: Send + Sync, Tag, Value, Provider, Output: Send>
    Handler<Context, Code, Builder> for BuildAndSetField<Tag, Provider>
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
