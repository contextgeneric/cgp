use cgp_monad::monadic::ok::OkMonadic;
use cgp_monad::providers::PipeMonadic;

pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
