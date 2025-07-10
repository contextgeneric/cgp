use core::marker::PhantomData;

use cgp_core::field::MapFields;
use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, PipeHandlers, TryComputer,
    TryComputerComponent,
};

delegate_components! {
    <Providers: MapFields<ToPipeError>>
    new DispatchMatchers<Providers> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            PipeHandlers<Providers::Mapped>,
    }
}

pub struct ToPipeError;

impl MapType for ToPipeError {
    type Map<Handler> = PipeError<Handler>;
}

trait HasResultError {
    type Error;
}

/**
    Helper trait to resolve ambiguity of the Ok type in the generic implementation.
    This allows the `OnlyError<E>` type to be casted to any `Result<T, E>` type, without
    knowing T upfront. This is important to resolve the ambiguity that the input type
    of `MatchWithHandlers` depends on the output type due to the `Ok` type in `Result`.
*/
trait IntoResult<T>: HasResultError {
    fn into_result(self) -> Result<T, Self::Error>;
}

impl<T, E> HasResultError for Result<T, E> {
    type Error = E;
}

impl<T, E> IntoResult<T> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}

pub struct OnlyError<E>(pub E);

impl<E> HasResultError for OnlyError<E> {
    type Error = E;
}

impl<T, E> IntoResult<T> for OnlyError<E> {
    fn into_result(self) -> Result<T, E> {
        Err(self.0)
    }
}

pub struct PipeError<Handler>(pub PhantomData<Handler>);

#[cgp_provider]
impl<Context, Code, Input, Output, RemainderA, RemainderB, Handler> Computer<Context, Code, Input>
    for PipeError<Handler>
where
    Handler: Computer<Context, Code, RemainderA, Output = Result<Output, RemainderB>>,
    Input: IntoResult<Output, Error = RemainderA>,
{
    type Output = Result<Output, RemainderB>;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        match input.into_result() {
            Ok(output) => Ok(output),
            Err(remainder) => Handler::compute(context, code, remainder),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, RemainderA, RemainderB, Handler>
    TryComputer<Context, Code, Input> for PipeError<Handler>
where
    Context: HasErrorType,
    Handler: TryComputer<Context, Code, RemainderA, Output = Result<Output, RemainderB>>,
    Input: IntoResult<Output, Error = RemainderA>,
{
    type Output = Result<Output, RemainderB>;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        match input.into_result() {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Handler::try_compute(context, code, remainder),
        }
    }
}

#[cgp_provider]
impl<Context, Code: Send, Input: Send, Output: Send, RemainderA: Send, RemainderB, Provider>
    Handler<Context, Code, Input> for PipeError<Provider>
where
    Context: HasAsyncErrorType,
    Provider: Handler<Context, Code, RemainderA, Output = Result<Output, RemainderB>>,
    Input: IntoResult<Output, Error = RemainderA>,
{
    type Output = Result<Output, RemainderB>;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        match input.into_result() {
            Ok(output) => Ok(Ok(output)),
            Err(remainder) => Provider::handle(context, code, remainder).await,
        }
    }
}
