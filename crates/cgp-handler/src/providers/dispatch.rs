use cgp_core::prelude::*;

use crate::{Computer, ComputerComponent};

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

// pub struct DispatchHandlers<Handlers>(pub PhantomData<Handlers>);

// pub struct ExtractAndHandle<Input, Handler>(pub PhantomData<(Input, Handler)>);

// impl<Context, Code, Input, Inner, Output, CurrentHandler, RestHandlers>
//     DispatchComputer<Context, Code, Input> for Cons<ExtractAndHandle<Inner, CurrentHandler>, RestHandlers>
// where
//     Input: CanExtractInto<Inner>,
//     CurrentHandler: Computer<Context, Code, Inner, Output = Output>,
//     RestHandlers: DispatchComputer<Context, Code, Input::Remainder, Output = Output>,
// {
//     type Output = Output;

//     fn compute(context: &Context, tag: PhantomData<Code>, input: Input) -> Self::Output {
//         let res = input.extract_into(PhantomData::<Inner>);

//         match res {
//             Ok(inner) => CurrentHandler::compute(context, tag, inner),
//             Err(remainder) => RestHandlers::compute(context, tag, remainder),
//         }
//     }
// }

// impl<Context, Code, Input> DispatchComputer<Context, Code, Input> for Void {
//     type Output = Void;
// }
