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
impl<Context, Code, TagA, ValueA, TagB, ValueB, Input, Provider, Remainder, RestFields, Output>
    Computer<Context, Code, Input>
    for DispatchHandlers<
        Either<Field<TagA, ValueA>, Either<Field<TagB, ValueB>, RestFields>>,
        Provider,
    >
where
    Provider: Computer<Context, Code, Field<TagA, ValueA>, Output = Output>,
    Input: ExtractField<TagA, Value = ValueA, Remainder = Remainder>,
    DispatchHandlers<Either<Field<TagB, ValueB>, RestFields>, Provider>:
        Computer<Context, Code, Remainder, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        match input.extract_field(PhantomData) {
            Ok(value) => Provider::compute(context, tag, value.into()),
            Err(remainder) => DispatchHandlers::compute(context, tag, remainder),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Tag, Value, Remainder, Output> Computer<Context, Code, Input>
    for DispatchHandlers<Either<Field<Tag, Value>, Void>, Provider>
where
    Provider: Computer<Context, Code, Field<Tag, Value>, Output = Output>,
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        match input.extract_field(PhantomData) {
            Ok(value) => Provider::compute(context, tag, value.into()),
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}
