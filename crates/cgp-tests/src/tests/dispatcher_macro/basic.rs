use cgp::prelude::*;

#[cgp_dispatch]
pub trait CanCall {
    fn call(&self) -> u64;
}
