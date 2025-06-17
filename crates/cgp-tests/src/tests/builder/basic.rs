use cgp::core::macros::Builder;
use cgp::prelude::*;

#[derive(Builder)]
pub struct Context {
    pub foo: u64,
    pub bar: String,
}
