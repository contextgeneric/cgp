use cgp::core::macros::Builder;
use cgp::prelude::*;

#[derive(Builder)]
pub struct Context(pub u64, pub String, pub bool);
