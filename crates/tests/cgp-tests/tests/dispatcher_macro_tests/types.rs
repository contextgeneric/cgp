use cgp::prelude::*;

pub struct Foo;
pub struct Bar;

#[derive(CgpVariant)]
pub enum FooBar {
    Foo(Foo),
    Bar(Bar),
}
