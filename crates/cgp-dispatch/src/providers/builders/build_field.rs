use cgp_core::prelude::*;
use cgp_handler::{Computer, TryComputer};

use crate::{BuilderComputer, TryBuilderComputer};

pub struct HandleAndBuildField<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

impl<Context, Code, Input, Tag, Value, Provider, Output, Builder>
    BuilderComputer<Context, Code, Input, Builder> for HandleAndBuildField<Tag, Provider>
where
    Provider: Computer<Context, Code, Input, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    fn build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Self::Output {
        let value = Provider::compute(context, code, input);
        builder.build_field(PhantomData::<Tag>, value)
    }
}

impl<Context, Code, Input, Tag, Value, Provider, Output, Builder>
    TryBuilderComputer<Context, Code, Input, Builder> for HandleAndBuildField<Tag, Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;

    fn try_build(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
        builder: Builder,
    ) -> Result<Self::Output, Context::Error> {
        let value = Provider::try_compute(context, code, input)?;
        Ok(builder.build_field(PhantomData::<Tag>, value))
    }
}
