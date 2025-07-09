use core::marker::PhantomData;

use cgp_core::{field::MapFields, prelude::*};
use cgp_handler::{Computer, ComputerComponent, PipeHandlers};

pub struct DispatchMatchers2<Handlers>(pub PhantomData<Handlers>);

delegate_components! {
    <Handler: MapFields<ToPipeError>>
    DispatchMatchers2<Handler> {
        ComputerComponent:
            PipeHandlers<Handler::Mapped>,
    }
}

pub struct ToPipeError;

impl MapType for ToPipeError {
    type Map<Handler> = PipeError<Handler>;
}

pub struct PipeError<Handler>(pub PhantomData<Handler>);

#[cgp_provider]
impl<Context, Code, Output, RemainderA, RemainderB, Handler>
    Computer<Context, Code, Result<Output, RemainderA>> for PipeError<Handler>
where
    Handler: Computer<Context, Code, RemainderA, Output = Result<Output, RemainderB>>,
{
    type Output = Result<Output, RemainderB>;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<Output, RemainderA>,
    ) -> Self::Output {
        match input {
            Ok(output) => Ok(output),
            Err(remainder) => Handler::compute(context, code, remainder),
        }
    }
}
