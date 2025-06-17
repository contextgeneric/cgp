use cgp::core::macros::Builder;
use cgp::prelude::*;

#[derive(Builder)]
pub struct Context<Foo, Bar, Baz>
where
    Foo: Clone,
{
    pub foo: Foo,
    pub bar: Bar,
    pub baz: Baz,
}
