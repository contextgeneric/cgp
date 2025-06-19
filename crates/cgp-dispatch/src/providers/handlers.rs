use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent};

pub struct DispatchHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Output, Handlers> Computer<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Input: HasExtractor,
    Handlers: DispatchComputer<Context, Code, Input::Extractor, Output = Output>,
    Handlers::Remainder: FinalizeExtract,
{
    type Output = Output;

    fn compute(_context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        let res = Handlers::compute(_context, code, input.extractor());

        match res {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

trait DispatchComputer<Context, Code, Input> {
    type Output;

    type Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder>;
}

impl<
        Context,
        Code,
        Input,
        CurrentHandler,
        NextHandler,
        RestHandlers,
        Output,
        RemainderA,
        RemainderB,
    > DispatchComputer<Context, Code, Input>
    for Cons<CurrentHandler, Cons<NextHandler, RestHandlers>>
where
    CurrentHandler: Computer<Context, Code, Input, Output = Result<Output, RemainderA>>,
    Cons<NextHandler, RestHandlers>:
        DispatchComputer<Context, Code, RemainderA, Output = Output, Remainder = RemainderB>,
{
    type Output = Output;

    type Remainder = RemainderB;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        let res = CurrentHandler::compute(context, tag, input);

        match res {
            Ok(output) => Ok(output),
            Err(remainder) => Cons::compute(context, tag, remainder),
        }
    }
}

impl<Context, Code, Input, Handler, Remainder, Output> DispatchComputer<Context, Code, Input>
    for Cons<Handler, Nil>
where
    Handler: Computer<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;

    type Remainder = Remainder;

    fn compute(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder> {
        Handler::compute(context, tag, input)
    }
}
