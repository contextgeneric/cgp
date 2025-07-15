use cgp_monad::monads::ok::OkMonadic;
use cgp_monad::providers::PipeMonadic;

pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
