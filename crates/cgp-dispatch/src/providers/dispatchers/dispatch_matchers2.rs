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

pub trait HasOutput {
    type Output;
}

pub struct ToPipeError;

impl MapType for ToPipeError {
    type Map<Handler> = PipeError<Handler>;
}

pub trait HasResultError {
    type Error;
}

pub trait IntoResult<T>: HasResultError {
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
