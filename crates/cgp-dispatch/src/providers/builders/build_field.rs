use cgp_core::prelude::*;
use cgp_handler::Computer;

use crate::BuilderComputer;

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
