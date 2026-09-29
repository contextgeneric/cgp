use cgp_fork_monad::monadic::ok::OkMonadic;
use cgp_fork_monad::providers::PipeMonadic;

pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
