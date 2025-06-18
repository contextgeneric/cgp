use cgp_core::prelude::*;

use crate::{Computer, ComputerComponent};

pub struct DispatchFields<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Fields, Extractor, Provider> Computer<Context, Code, Input>
    for DispatchFields<Provider>
where
    Input: HasFields<Fields = Fields> + HasExtractor<Extractor = Extractor>,
    DispatchHandlers<Fields, Provider>: Computer<Context, Code, Extractor, Output = Output>,
{
    type Output = Output;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Input) -> Output {
        DispatchHandlers::compute(_context, _tag, input.extractor())
    }
}

pub struct DispatchHandlers<Fields, Provider = UseContext>(pub PhantomData<(Fields, Provider)>);

#[cgp_provider]
impl<Context, Code, Tag, Value, Input, Provider, Remainder, RestFields, Output>
    Computer<Context, Code, Input>
    for DispatchHandlers<Either<Field<Tag, Value>, RestFields>, Provider>
where
    Provider: Computer<Context, Code, Field<Tag, Value>, Output = Output>,
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    DispatchHandlers<RestFields, Provider>: Computer<Context, Code, Remainder, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        match input.extract_field(PhantomData) {
            Either::Left(value) => Provider::compute(context, tag, value.into()),
            Either::Right(remainder) => {
                <DispatchHandlers<RestFields, Provider>>::compute(context, tag, remainder)
            }
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> Computer<Context, Code, Input>
    for DispatchHandlers<Void, Provider>
where
    Provider: Computer<Context, Code, Void, Output = Output>,
    Input: FinalizeExtract,
{
    type Output = Output;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Input) -> Output {
        input.finalize_extract()
    }
}
