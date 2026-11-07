use cgp_monad::monadic::ok::OkMonadic;
use cgp_monad::providers::PipeMonadic;

/// Dispatches a value through a list of matchers.
pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
