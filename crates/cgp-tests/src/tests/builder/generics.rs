use cgp::core::macros::Builder;
use cgp::prelude::*;

#[derive(Builder)]
pub struct Context<Foo, Bar>
where
    Foo: Clone,
{
    pub foo: Foo,
    pub bar: Bar,
}
