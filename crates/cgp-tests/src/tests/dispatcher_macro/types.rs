use cgp::prelude::*;

pub struct Foo;
pub struct Bar;

#[derive(HasFields, ExtractField, FromVariant)]
pub enum FooBar {
    Foo(Foo),
    Bar(Bar),
}
