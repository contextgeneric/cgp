use cgp_core::prelude::*;
use cgp_handler::{Computer, ComputerComponent, Handler, HandlerComponent};

use crate::{
    monads::IntoErr,
    traits::{CanWrap, ContainsValue, Functorial},
};

pub struct BindErr<M, Cont>(pub PhantomData<(M, Cont)>);

#[cgp_provider]
impl<Context, Code, T1, T2, E, M, Cont, Output> Computer<Context, Code, Result<T1, E>>
    for BindErr<M, Cont>
where
    Cont: Computer<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value: IntoErr<E, T = T2>>
        + Functorial<Cont::Output, Result<T2, E>, Output = Output>
        + CanWrap<M::Output, Value = Result<T2, E>>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Result<T1, E>) -> Self::Output {
        match input {
            Err(err) => M::wrap(Err(err)),
            Ok(value) => {
                let res = Cont::compute(context, code, value);
                M::map(res, |value| value.into_err())
            }
        }
    }
}

#[cgp_provider]
impl<Context: Send, Code: Send, T1: Send, T2: Send, E: Send, M: Send, Cont: Send, Output: Send>
    Handler<Context, Code, Result<T1, E>> for BindErr<M, Cont>
where
    Context: HasAsyncErrorType,
    Cont: Handler<Context, Code, T1>,
    M: ContainsValue<Cont::Output, Value: IntoErr<E, T = T2>>
        + Functorial<Cont::Output, Result<T2, E>, Output = Output>
        + CanWrap<M::Output, Value = Result<T2, E>>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Result<T1, E>,
    ) -> Result<Self::Output, Context::Error> {
        match input {
            Err(err) => Ok(M::wrap(Err(err))),
            Ok(value) => {
                let res = Cont::handle(context, code, value).await?;
                Ok(M::map(res, |value| value.into_err()))
            }
        }
    }
}
