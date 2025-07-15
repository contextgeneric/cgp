use cgp_core::prelude::*;
use cgp_handler::{
    Computer, ComputerComponent, Handler, HandlerComponent, TryComputer, TryComputerComponent,
};

use crate::monads::IntoOk;
use crate::traits::{CanBindHandler, CanWrap, ContainsValue, Functorial};

pub struct OkBinder<M>(pub PhantomData<M>);

impl<M> CanBindHandler for OkBinder<M> {
    type Bind<Handler> = BindOk<M, Handler>;
}

pub struct BindOk<M, Cont>(pub PhantomData<(M, Cont)>);

#[cgp_provider]
impl<Context, Code, T, E1, E2, M, Cont, Output> Computer<Context, Code, Result<T, E1>>
    for BindOk<M, Cont>
where
    Cont: Computer<Context, Code, E1>,
    M: ContainsValue<Cont::Output, Value: IntoOk<T, E = E2>>
        + Functorial<Cont::Output, Result<T, E2>, Output = Output>
        + CanWrap<M::Output, Value = Result<T, E2>>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Result<T, E1>) -> Self::Output {
        match input {
            Ok(err) => M::wrap(Ok(err)),
            Err(value) => {
                let res = Cont::compute(context, code, value);
                M::map(res, |value| value.into_ok())
            }
        }
    }
}

#[cgp_provider]
impl<Context, Code, T, E1, E2, M, Cont, Output> TryComputer<Context, Code, Result<T, E1>>
    for BindOk<M, Cont>
where
    Context: HasErrorType,
    Cont: TryComputer<Context, Code, E1>,
    M: ContainsValue<Cont::Output, Value: IntoOk<T, E = E2>>
        + Functorial<Cont::Output, Result<T, E2>, Output = Output>
        + CanWrap<M::Output, Value = Result<T, E2>>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<T, E1>,
    ) -> Result<Self::Output, Context::Error> {
        match input {
            Ok(err) => Ok(M::wrap(Ok(err))),
            Err(value) => {
                let res = Cont::try_compute(context, code, value)?;
                Ok(M::map(res, |value| value.into_ok()))
            }
        }
    }
}

#[cgp_provider]
impl<Context: Send, Code: Send, T: Send, E1: Send, E2: Send, M: Send, Cont: Send, Output: Send>
    Handler<Context, Code, Result<T, E1>> for BindOk<M, Cont>
where
    Context: HasAsyncErrorType,
    Cont: Handler<Context, Code, E1>,
    M: ContainsValue<Cont::Output, Value: IntoOk<T, E = E2>>
        + Functorial<Cont::Output, Result<T, E2>, Output = Output>
        + CanWrap<M::Output, Value = Result<T, E2>>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<T, E1>,
    ) -> Result<Self::Output, Context::Error> {
        match input {
            Ok(err) => Ok(M::wrap(Ok(err))),
            Err(value) => {
                let res = Cont::handle(context, code, value).await?;
                Ok(M::map(res, |value| value.into_ok()))
            }
        }
    }
}
