use cgp_monad::{monads::ok::OkMonadic, providers::PipeMonadic};

pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
