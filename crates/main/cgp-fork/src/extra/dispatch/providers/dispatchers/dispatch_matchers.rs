use crate::extra::monad::monadic::ok::OkMonadic;
use crate::extra::monad::providers::PipeMonadic;

pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
