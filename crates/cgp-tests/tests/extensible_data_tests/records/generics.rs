use cgp::prelude::*;

#[derive(CgpData)]
pub struct Context<Foo, Bar, Baz>
where
    Foo: Clone,
{
    pub foo: Foo,
    pub bar: Bar,
    pub baz: Baz,
}
