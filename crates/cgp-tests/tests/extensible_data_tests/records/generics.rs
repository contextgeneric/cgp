use cgp::prelude::*;

#[derive(BuildField)]
pub struct Context<Foo, Bar, Baz>
where
    Foo: Clone,
{
    pub foo: Foo,
    pub bar: Bar,
    pub baz: Baz,
}
