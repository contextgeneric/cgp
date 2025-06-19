use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

pub struct DispatchFields<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Fields, Extractor, Provider> Computer<Context, Code, Input>
    for DispatchFields<Provider>
where
    Input: HasFields<Fields = Fields> + HasExtractor<Extractor = Extractor>,
    Fields: DispatchComputer<Context, Code, Extractor, Provider, Output = Output>,
{
    type Output = Output;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Input) -> Output {
        Fields::compute(_context, _tag, input.extractor())
    }
}

trait DispatchComputer<Context, Code, Input, Provider> {
    type Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Self::Output;
}

impl<Context, Code, TagA, ValueA, TagB, ValueB, Input, Provider, Remainder, RestFields, Output>
    DispatchComputer<Context, Code, Input, Provider>
    for Either<Field<TagA, ValueA>, Either<Field<TagB, ValueB>, RestFields>>
where
    Provider: Computer<Context, Code, Field<TagA, ValueA>, Output = Output>,
    Input: ExtractField<TagA, Value = ValueA, Remainder = Remainder>,
    Either<Field<TagB, ValueB>, RestFields>:
        DispatchComputer<Context, Code, Remainder, Provider, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        match input.extract_field(PhantomData) {
            Ok(value) => Provider::compute(context, tag, value.into()),
            Err(remainder) => Either::compute(context, tag, remainder),
        }
    }
}

impl<Context, Code, Input, Provider, Tag, Value, Remainder, Output>
    DispatchComputer<Context, Code, Input, Provider> for Either<Field<Tag, Value>, Void>
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
