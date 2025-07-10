use cgp_core::prelude::*;
use cgp_handler::{
    ComputerComponent, HandlerComponent, Monad, Monadic, PipeMonadic, TryComputerComponent,
};

delegate_components! {
    <Providers>
    new DispatchMatchers2<Providers> {
        [
            ComputerComponent,
            TryComputerComponent,
            HandlerComponent,
        ]:
            PipeMonadic<OkMonadic, Providers>,
    }
}

pub struct OkMonadic;

pub struct OkMonad<T>(pub PhantomData<T>);

impl<T, E> Monadic<Result<T, E>> for OkMonadic {
    type Value = E;

    type Monad = OkMonad<T>;
}

impl<T> Monad for OkMonad<T> {
    type M<E> = Result<T, E>;

    fn bind<E1, E2>(value: Result<T, E1>, cont: impl Fn(E1) -> Result<T, E2>) -> Result<T, E2> {
        match value {
            Ok(value) => Ok(value),
            Err(err) => cont(err),
        }
    }
}
