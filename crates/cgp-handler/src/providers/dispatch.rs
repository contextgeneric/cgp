use cgp_core::prelude::*;

use crate::{CanCompute, Computer, ComputerComponent};

pub struct DispatchHandlers<Providers>(pub PhantomData<Providers>);

#[cgp_provider]
impl<Context, Code, Tag, Input, Value, Remainder, RestFields, Output> Computer<Context, Code, Input>
    for DispatchHandlers<Either<Field<Tag, Value>, RestFields>>
where
    Context: CanCompute<Code, Field<Tag, Value>, Output = Output>,
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    DispatchHandlers<RestFields>: Computer<Context, Code, Remainder, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        match input.extract_field(PhantomData) {
            Either::Left(value) => context.compute(PhantomData, value.into()),
            Either::Right(remainder) => {
                <DispatchHandlers<RestFields>>::compute(context, tag, remainder)
            }
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output> Computer<Context, Code, Input> for DispatchHandlers<Void>
where
    Context: CanCompute<Code, Void, Output = Output>,
    Input: FinalizeExtract,
{
    type Output = Output;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Input) -> Output {
        input.finalize_extract()
    }
}
